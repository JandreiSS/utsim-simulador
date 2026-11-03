//! Mapping: an observation of a tag becomes an event on a model object (an increment of
//! a counter, a state, a content, a position), after scale, unit, offset and filters.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::{Axis, Counter, Measure, StateClass, TagMap};
use crate::obs::{Observation, Quality, Value};

/// What happened to an object, at twin time `t`.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub t: f64,
    pub object: String,
    pub kind: EventKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventKind {
    /// `delta` more items entered or left.
    Count {
        measure: Measure,
        delta: f64,
    },
    /// The object entered this state (counted only after `debounce`, when positive).
    State {
        class: StateClass,
        debounce: f64,
    },
    Content(f64),
    /// A full position, or one axis.
    Position {
        axis: Option<Axis>,
        value: [f64; 3],
    },
}

/// Why an observation produced no event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skip {
    /// Bad quality at the source (or sensor lost).
    BadQuality,
    /// Not a number where one is expected (or a missing JSON field).
    NotANumber,
    /// Outside `[min, max]`.
    OutOfRange,
    /// A state code with no class.
    UnknownState,
    /// Filtered: no change, or inside the dead band.
    NoChange,
    /// The first reading of a total counter: the base the increments count from.
    Baseline,
}

/// Counts of one rule, for the panel.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RuleStats {
    pub recebidas: u64,
    pub eventos: u64,
    pub qualidade_ruim: u64,
    pub nao_numericas: u64,
    pub fora_da_faixa: u64,
    pub estado_desconhecido: u64,
    /// Last value received (after anonymisation) and its twin time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimo_valor: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimo_t: Option<f64>,
    pub ultima_qualidade: Quality,
}

#[derive(Clone, Debug, Default)]
struct Filter {
    /// Last total of a counter (after scale).
    last_total: Option<f64>,
    /// Last value emitted (content, position axis), for the dead band.
    last_emitted: Option<f64>,
    last_position: Option<[f64; 3]>,
    last_state: Option<StateClass>,
}

pub struct Mapper {
    rules: Vec<TagMap>,
    index: HashMap<(String, String), usize>,
    filters: Vec<Filter>,
    pub stats: Vec<RuleStats>,
}

impl Mapper {
    pub fn new(rules: &[TagMap]) -> Mapper {
        let index = rules
            .iter()
            .enumerate()
            .map(|(i, r)| ((r.source.clone(), r.tag.clone()), i))
            .collect();
        Mapper {
            rules: rules.to_vec(),
            index,
            filters: vec![Filter::default(); rules.len()],
            stats: vec![RuleStats::default(); rules.len()],
        }
    }

    pub fn rules(&self) -> &[TagMap] {
        &self.rules
    }

    /// The rule of a tag of a source.
    pub fn rule_for(&self, source: &str, tag: &str) -> Option<usize> {
        self.index
            .get(&(source.to_string(), tag.to_string()))
            .copied()
    }

    /// Applies rule `rule` to an observation.
    pub fn apply(&mut self, rule: usize, obs: &Observation) -> Result<Event, Skip> {
        let r = &self.rules[rule];
        let st = &mut self.stats[rule];
        st.recebidas += 1;
        st.ultimo_valor = Some(obs.value.clone());
        st.ultimo_t = Some(obs.t);
        st.ultima_qualidade = obs.quality;
        let out = convert(r, &mut self.filters[rule], obs);
        match out {
            Ok(_) => st.eventos += 1,
            Err(Skip::BadQuality) => st.qualidade_ruim += 1,
            Err(Skip::NotANumber) => st.nao_numericas += 1,
            Err(Skip::OutOfRange) => st.fora_da_faixa += 1,
            Err(Skip::UnknownState) => st.estado_desconhecido += 1,
            Err(Skip::NoChange | Skip::Baseline) => {}
        }
        out.map(|kind| Event {
            t: obs.t,
            object: r.object.clone(),
            kind,
        })
    }
}

/// The value at the JSON pointer of the rule; without one, a JSON object gives its
/// `valor` or `value` member.
fn extract(r: &TagMap, v: &Value) -> Option<Value> {
    let Value::Text(text) = v else {
        return Some(v.clone());
    };
    let t = text.trim_start();
    if !(t.starts_with('{') || t.starts_with('[')) {
        return Some(v.clone());
    }
    let json: serde_json::Value = serde_json::from_str(text).ok()?;
    let node = match &r.field {
        Some(p) => json.pointer(p)?.clone(),
        None if r.measure == Measure::Position => json,
        None => json
            .get("valor")
            .or_else(|| json.get("value"))
            .cloned()
            .unwrap_or(json),
    };
    Some(match node {
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => Value::Num(n.as_f64()?),
        serde_json::Value::String(s) => Value::Text(s),
        other => Value::Text(other.to_string()),
    })
}

fn in_range(r: &TagMap, x: f64) -> bool {
    r.min.is_none_or(|m| x >= m) && r.max.is_none_or(|m| x <= m)
}

/// A position from a JSON array `[x, y, z]` or object `{"x":…, "y":…, "z":…}`.
fn position_of(v: &Value) -> Option<[f64; 3]> {
    let Value::Text(t) = v else { return None };
    let json: serde_json::Value = serde_json::from_str(t).ok()?;
    let mut p = [0.0; 3];
    match json {
        serde_json::Value::Array(a) if (2..=3).contains(&a.len()) => {
            for (i, x) in a.iter().enumerate() {
                p[i] = x.as_f64()?;
            }
        }
        serde_json::Value::Object(o) => {
            for (i, k) in ["x", "y", "z"].iter().enumerate() {
                match o.get(*k) {
                    Some(x) => p[i] = x.as_f64()?,
                    None if i == 2 => {}
                    None => return None,
                }
            }
        }
        _ => return None,
    }
    p.iter().all(|x| x.is_finite()).then_some(p)
}

fn convert(r: &TagMap, f: &mut Filter, obs: &Observation) -> Result<EventKind, Skip> {
    if obs.quality == Quality::Bad {
        return Err(Skip::BadQuality);
    }
    let v = extract(r, &obs.value).ok_or(Skip::NotANumber)?;
    let scaled = |x: f64| x * r.scale * r.unit_factor() + r.offset;
    match r.measure {
        Measure::Inputs | Measure::Outputs => {
            let x = scaled(v.as_f64().ok_or(Skip::NotANumber)?);
            if !in_range(r, x) {
                return Err(Skip::OutOfRange);
            }
            let delta = match r.counter {
                Counter::Pulse => x,
                Counter::Total => {
                    let last = f.last_total.replace(x);
                    match last {
                        None => return Err(Skip::Baseline),
                        Some(l) if x >= l => x - l,
                        Some(l) => match r.rollover {
                            Some(m) => x + m * r.scale - l,
                            // A decrease without wrap-around is a reset of the counter.
                            None => x,
                        },
                    }
                }
            };
            if delta <= 0.0 {
                return Err(Skip::NoChange);
            }
            Ok(EventKind::Count {
                measure: r.measure,
                delta,
            })
        }
        Measure::Content => {
            let x = scaled(v.as_f64().ok_or(Skip::NotANumber)?);
            if !in_range(r, x) {
                return Err(Skip::OutOfRange);
            }
            if let (Some(d), Some(l)) = (r.deadband, f.last_emitted) {
                if (x - l).abs() < d {
                    return Err(Skip::NoChange);
                }
            }
            if f.last_emitted == Some(x) {
                return Err(Skip::NoChange);
            }
            f.last_emitted = Some(x);
            Ok(EventKind::Content(x))
        }
        Measure::State => {
            let key = v.as_key();
            let class = r
                .states
                .get(&key)
                .copied()
                .or_else(|| StateClass::of_name(&key))
                .ok_or(Skip::UnknownState)?;
            if f.last_state == Some(class) {
                return Err(Skip::NoChange);
            }
            f.last_state = Some(class);
            Ok(EventKind::State {
                class,
                debounce: r.debounce.unwrap_or(0.0),
            })
        }
        Measure::Position => {
            let p = match r.axis {
                Some(axis) => {
                    let x = scaled(v.as_f64().ok_or(Skip::NotANumber)?);
                    let mut p = f.last_position.unwrap_or([0.0; 3]);
                    p[axis as usize] = x;
                    p
                }
                None => position_of(&v).ok_or(Skip::NotANumber)?.map(scaled),
            };
            if p.iter().any(|x| !in_range(r, *x)) {
                return Err(Skip::OutOfRange);
            }
            if let (Some(d), Some(l)) = (r.deadband, f.last_position) {
                let dist =
                    ((p[0] - l[0]).powi(2) + (p[1] - l[1]).powi(2) + (p[2] - l[2]).powi(2)).sqrt();
                if dist < d {
                    return Err(Skip::NoChange);
                }
            }
            f.last_position = Some(p);
            Ok(EventKind::Position {
                axis: r.axis,
                value: p,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(measure: Measure) -> TagMap {
        TagMap {
            source: "s".into(),
            tag: "t".into(),
            object: "o".into(),
            measure,
            scale: 1.0,
            offset: 0.0,
            unit: None,
            counter: Counter::Total,
            rollover: None,
            deadband: None,
            min: None,
            max: None,
            debounce: None,
            states: Default::default(),
            field: None,
            axis: None,
            person: false,
        }
    }

    fn obs(t: f64, v: Value) -> Observation {
        Observation {
            t,
            wall_ms: 0,
            source: "s".into(),
            seq: 0,
            tag: "t".into(),
            value: v,
            quality: Quality::Good,
            rule: Some(0),
        }
    }

    fn count(m: &mut Mapper, t: f64, v: f64) -> Option<f64> {
        match m.apply(0, &obs(t, Value::Num(v))) {
            Ok(Event {
                kind: EventKind::Count { delta, .. },
                ..
            }) => Some(delta),
            _ => None,
        }
    }

    #[test]
    fn total_counter_with_rollover_and_reset() {
        let mut r = rule(Measure::Outputs);
        r.rollover = Some(100.0);
        let mut m = Mapper::new(&[r]);
        assert_eq!(
            count(&mut m, 0.0, 95.0),
            None,
            "a primeira leitura é a base"
        );
        assert_eq!(count(&mut m, 1.0, 98.0), Some(3.0));
        assert_eq!(count(&mut m, 2.0, 2.0), Some(4.0), "volta em 100");
        assert_eq!(count(&mut m, 3.0, 2.0), None);
        // Without rollover a decrease is a reset.
        let mut m = Mapper::new(&[rule(Measure::Outputs)]);
        count(&mut m, 0.0, 50.0);
        assert_eq!(count(&mut m, 1.0, 3.0), Some(3.0));
    }

    #[test]
    fn pulse_counter_and_scale() {
        let mut r = rule(Measure::Inputs);
        r.counter = Counter::Pulse;
        let mut m = Mapper::new(&[r]);
        assert_eq!(count(&mut m, 0.0, 1.0), Some(1.0));
        assert!(m.apply(0, &obs(1.0, Value::Bool(false))).is_err());
        assert!(matches!(
            m.apply(0, &obs(1.0, Value::Bool(true))).unwrap().kind,
            EventKind::Count { delta, .. } if delta == 1.0
        ));
    }

    #[test]
    fn content_dead_band_range_and_quality() {
        let mut r = rule(Measure::Content);
        r.deadband = Some(1.0);
        r.max = Some(100.0);
        r.scale = 2.0;
        let mut m = Mapper::new(&[r]);
        assert_eq!(
            m.apply(0, &obs(0.0, Value::Num(5.0))).unwrap().kind,
            EventKind::Content(10.0)
        );
        assert_eq!(m.apply(0, &obs(1.0, Value::Num(5.4))), Err(Skip::NoChange));
        assert_eq!(
            m.apply(0, &obs(2.0, Value::Num(6.0))).unwrap().kind,
            EventKind::Content(12.0)
        );
        assert_eq!(
            m.apply(0, &obs(3.0, Value::Num(60.0))),
            Err(Skip::OutOfRange)
        );
        let mut bad = obs(4.0, Value::Num(1.0));
        bad.quality = Quality::Bad;
        assert_eq!(m.apply(0, &bad), Err(Skip::BadQuality));
        assert_eq!(
            m.apply(0, &obs(5.0, Value::Text("abc".into()))),
            Err(Skip::NotANumber)
        );
        assert_eq!(m.stats[0].qualidade_ruim, 1);
        assert_eq!(m.stats[0].fora_da_faixa, 1);
        assert_eq!(m.stats[0].eventos, 2);
    }

    #[test]
    fn states_with_custom_codes() {
        let mut r = rule(Measure::State);
        r.states.insert("3".into(), StateClass::Down);
        r.debounce = Some(2.0);
        let mut m = Mapper::new(&[r]);
        let e = m.apply(0, &obs(0.0, Value::Num(3.0))).unwrap();
        assert_eq!(
            e.kind,
            EventKind::State {
                class: StateClass::Down,
                debounce: 2.0
            }
        );
        assert!(matches!(
            m.apply(0, &obs(1.0, Value::Text("processing".into())))
                .unwrap()
                .kind,
            EventKind::State {
                class: StateClass::Running,
                ..
            }
        ));
        assert_eq!(
            m.apply(0, &obs(2.0, Value::Text("rodando".into()))),
            Err(Skip::NoChange)
        );
        assert_eq!(
            m.apply(0, &obs(3.0, Value::Num(9.0))),
            Err(Skip::UnknownState)
        );
    }

    #[test]
    fn json_fields_and_positions_in_mm() {
        let mut r = rule(Measure::Content);
        r.field = Some("/fila/qtd".into());
        let mut m = Mapper::new(&[r]);
        let v = Value::Text(r#"{"fila": {"qtd": 7}}"#.into());
        assert_eq!(
            m.apply(0, &obs(0.0, v)).unwrap().kind,
            EventKind::Content(7.0)
        );
        let missing = Value::Text(r#"{"fila": {}}"#.into());
        assert_eq!(m.apply(0, &obs(1.0, missing)), Err(Skip::NotANumber));

        // Without a field, a JSON object gives its "valor".
        let mut m = Mapper::new(&[rule(Measure::Content)]);
        let v = Value::Text(r#"{"valor": 4, "t": 9}"#.into());
        assert_eq!(
            m.apply(0, &obs(0.0, v)).unwrap().kind,
            EventKind::Content(4.0)
        );

        let mut r = rule(Measure::Position);
        r.unit = Some("mm".into());
        let mut m = Mapper::new(&[r]);
        let v = Value::Text(r#"{"x": 1500, "y": 250}"#.into());
        assert_eq!(
            m.apply(0, &obs(0.0, v)).unwrap().kind,
            EventKind::Position {
                axis: None,
                value: [1.5, 0.25, 0.0]
            }
        );
        let mut r = rule(Measure::Position);
        r.axis = Some(Axis::Y);
        let mut m = Mapper::new(&[r]);
        assert_eq!(
            m.apply(0, &obs(0.0, Value::Num(3.0))).unwrap().kind,
            EventKind::Position {
                axis: Some(Axis::Y),
                value: [0.0, 3.0, 0.0]
            }
        );
    }
}
