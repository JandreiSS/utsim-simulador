//! The observed state of the plant and the KPIs of each time window.
//!
//! Events (from the real plant through the mapping, or from the simulated runs) keep,
//! per object, the counters, the content and the state as step functions of time. The
//! windows `[origin + kW, origin + (k+1)W)` close in order; each closed window gives:
//!
//! | KPI | meaning |
//! |---|---|
//! | `<obj>.vazao` | items that left in the window |
//! | `<obj>.chegadas` | items that entered in the window |
//! | `<obj>.wip` | time-average content |
//! | `<obj>.utilizacao` | fraction of the window running |
//! | `<obj>.quebra` | fraction of the window broken down |
//!
//! A KPI only appears in a window when its measure was known during the whole window
//! (a total counter needs its first reading before the window starts). The same code
//! computes the real and the simulated KPIs, so both use the same estimator.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use utsim_core::BlockState;
use utsim_model::{InitialState, Model};

use crate::config::{Axis, Measure, StateClass};
use crate::map::{Event, EventKind};

/// KPIs of one closed window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowKpis {
    pub index: usize,
    pub start: f64,
    pub end: f64,
    pub values: BTreeMap<String, f64>,
}

/// The observed state of one object.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ObjectState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conteudo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado: Option<StateClass>,
    /// Twin time the current state started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado_desde: Option<f64>,
    pub entradas: f64,
    pub saidas: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posicao: Option<[f64; 3]>,
    /// Twin time of the last event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atualizado: Option<f64>,
}

#[derive(Clone, Debug, Default)]
struct Track {
    obs: ObjectState,
    /// Measures mapped to this object.
    measures: BTreeSet<Measure>,
    /// Time from which the counters count (first event of the object, or the origin for
    /// pulse counters known from the start).
    counting_since: [Option<f64>; 2],
    pending: Option<(StateClass, f64)>,
    /// Time up to which the step functions are integrated.
    last: f64,
    // Window accumulators.
    win_in: f64,
    win_out: f64,
    content_area: f64,
    content_known: f64,
    state_time: [f64; 3],
    state_known: f64,
}

/// Window KPIs over a stream of events.
#[derive(Clone, Debug)]
pub struct Accumulator {
    window: f64,
    origin: f64,
    win_start: f64,
    index: usize,
    objects: BTreeMap<String, Track>,
    closed: Vec<WindowKpis>,
    now: f64,
}

impl Accumulator {
    /// `measures`: which measures are mapped to which object; `counting_from_start`:
    /// whether the counters count from `t = origin` (simulated runs with a baseline at
    /// the origin, pulse counters) instead of from their first reading.
    pub fn new(
        window: f64,
        origin: f64,
        measures: &[(String, Measure)],
        counting_from: Option<f64>,
    ) -> Accumulator {
        let mut objects: BTreeMap<String, Track> = BTreeMap::new();
        for (o, m) in measures {
            let t = objects.entry(o.clone()).or_default();
            t.measures.insert(*m);
            t.last = f64::NEG_INFINITY;
            match m {
                Measure::Inputs => t.counting_since[0] = counting_from,
                Measure::Outputs => t.counting_since[1] = counting_from,
                _ => {}
            }
        }
        Accumulator {
            window,
            origin,
            win_start: origin,
            index: 0,
            objects,
            closed: Vec::new(),
            now: f64::NEG_INFINITY,
        }
    }

    pub fn closed(&self) -> &[WindowKpis] {
        &self.closed
    }

    /// Windows closed since `from` (an index into [`Accumulator::closed`]).
    pub fn closed_since(&self, from: usize) -> &[WindowKpis] {
        &self.closed[from.min(self.closed.len())..]
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn objects(&self) -> BTreeMap<String, ObjectState> {
        self.objects
            .iter()
            .map(|(k, t)| (k.clone(), t.obs.clone()))
            .collect()
    }

    /// Integrates one object's step functions up to `to`, applying a pending state when
    /// its debounce ends before `to`.
    fn integrate(t: &mut Track, win_start: f64, to: f64) {
        if let Some((class, at)) = t.pending {
            if at <= to {
                Self::integrate_plain(t, win_start, at);
                t.obs.estado = Some(class);
                t.obs.estado_desde = Some(at);
                t.pending = None;
            }
        }
        Self::integrate_plain(t, win_start, to);
    }

    fn integrate_plain(t: &mut Track, win_start: f64, to: f64) {
        let from = t.last.max(win_start);
        if to > from {
            let dt = to - from;
            if let Some(c) = t.obs.conteudo {
                t.content_area += c * dt;
                t.content_known += dt;
            }
            if let Some(s) = t.obs.estado {
                t.state_time[s.index()] += dt;
                t.state_known += dt;
            }
        }
        if to > t.last {
            t.last = to;
        }
    }

    /// Closes every window that ends at or before `t`.
    pub fn advance(&mut self, t: f64) {
        if t > self.now {
            self.now = t;
        }
        while self.win_start + self.window <= t {
            let end = self.win_start + self.window;
            let mut values = BTreeMap::new();
            for (name, tr) in &mut self.objects {
                Self::integrate(tr, self.win_start, end);
                let full = |known: f64| known >= self.window * (1.0 - 1e-9);
                if tr.measures.contains(&Measure::Outputs)
                    && tr.counting_since[1].is_some_and(|s| s <= self.win_start)
                {
                    values.insert(format!("{name}.vazao"), tr.win_out);
                }
                if tr.measures.contains(&Measure::Inputs)
                    && tr.counting_since[0].is_some_and(|s| s <= self.win_start)
                {
                    values.insert(format!("{name}.chegadas"), tr.win_in);
                }
                if tr.measures.contains(&Measure::Content) && full(tr.content_known) {
                    values.insert(format!("{name}.wip"), tr.content_area / self.window);
                }
                if tr.measures.contains(&Measure::State) && full(tr.state_known) {
                    values.insert(
                        format!("{name}.utilizacao"),
                        tr.state_time[StateClass::Running.index()] / self.window,
                    );
                    values.insert(
                        format!("{name}.quebra"),
                        tr.state_time[StateClass::Down.index()] / self.window,
                    );
                }
                tr.win_in = 0.0;
                tr.win_out = 0.0;
                tr.content_area = 0.0;
                tr.content_known = 0.0;
                tr.state_time = [0.0; 3];
                tr.state_known = 0.0;
            }
            self.closed.push(WindowKpis {
                index: self.index,
                start: self.win_start,
                end,
                values,
            });
            self.index += 1;
            self.win_start = end;
        }
        // Objects are integrated only at their own events and at window ends, so the
        // result does not depend on when the clock is advanced (live and replay agree to
        // the last bit).
    }

    /// Applies an event (at its time; events must come in time order).
    pub fn apply(&mut self, e: &Event) {
        self.advance(e.t);
        let t = e.t.max(self.now);
        let ws = self.win_start;
        let tr = self.objects.entry(e.object.clone()).or_default();
        Self::integrate(tr, ws, t);
        tr.obs.atualizado = Some(t);
        match &e.kind {
            EventKind::Count { measure, delta } => {
                let i = if *measure == Measure::Inputs { 0 } else { 1 };
                if tr.counting_since[i].is_none() {
                    // The first increment seen: counting starts now.
                    tr.counting_since[i] = Some(t);
                }
                if t >= ws {
                    if i == 0 {
                        tr.win_in += delta;
                    } else {
                        tr.win_out += delta;
                    }
                }
                if i == 0 {
                    tr.obs.entradas += delta;
                } else {
                    tr.obs.saidas += delta;
                }
            }
            EventKind::Content(c) => {
                tr.obs.conteudo = Some(*c);
            }
            EventKind::State { class, debounce } => {
                if *debounce > 0.0 {
                    if tr.obs.estado == Some(*class) {
                        tr.pending = None;
                    } else {
                        tr.pending = Some((*class, t + debounce));
                    }
                } else {
                    tr.pending = None;
                    if tr.obs.estado != Some(*class) {
                        tr.obs.estado = Some(*class);
                        tr.obs.estado_desde = Some(t);
                    }
                }
            }
            EventKind::Position { axis, value } => {
                let mut p = tr.obs.posicao.unwrap_or([0.0; 3]);
                match axis {
                    Some(a) => {
                        let i = match a {
                            Axis::X => 0,
                            Axis::Y => 1,
                            Axis::Z => 2,
                        };
                        p[i] = value[i];
                    }
                    None => p = *value,
                }
                tr.obs.posicao = Some(p);
            }
        }
    }

    /// Mark a counter as known from its first reading (a total counter's baseline).
    pub fn baseline(&mut self, object: &str, measure: Measure, t: f64) {
        if let Some(tr) = self.objects.get_mut(object) {
            let i = match measure {
                Measure::Inputs => 0,
                Measure::Outputs => 1,
                _ => return,
            };
            tr.counting_since[i].get_or_insert(t);
        }
    }

    pub fn origin(&self) -> f64 {
        self.origin
    }
}

/// The observed state as an initial state file, for a run started from the plant as it
/// is now (the future forecast): contents as counts, breakdowns with their elapsed time,
/// transporter positions. Objects whose state the model refuses are left out, with a
/// note.
pub fn initial_state(
    model: &Model,
    objects: &BTreeMap<String, ObjectState>,
    now: f64,
) -> (InitialState, Vec<String>) {
    let mut notes = Vec::new();
    let mut state = InitialState {
        clock: if now.is_finite() { now.max(0.0) } else { 0.0 },
        warmup: None,
        objects: BTreeMap::new(),
    };
    for (name, o) in objects {
        if model.objects.iter().all(|m| m.name != *name) {
            continue;
        }
        let mut bs = BlockState::default();
        if let Some(c) = o.conteudo {
            let n = c.round().max(0.0) as usize;
            if n > 0 {
                bs.count = Some(n);
            }
        }
        if let (Some(StateClass::Down), Some(since)) = (o.estado, o.estado_desde) {
            bs.down = Some((now - since).max(0.0));
        }
        if let Some(p) = o.posicao {
            bs.position = Some(p);
        }
        if bs == BlockState::default() {
            continue;
        }
        let mut single = InitialState {
            objects: BTreeMap::from([(name.clone(), bs.clone())]),
            ..state.clone()
        };
        single.clock = state.clock;
        match model.check_state(&single) {
            Ok(_) => {
                state.objects.insert(name.clone(), bs);
            }
            Err(e) => {
                // Retry without the content (e.g. a Sink or a Source refuses items).
                let mut lean = bs.clone();
                lean.count = None;
                let ok = lean != BlockState::default() && {
                    single.objects.insert(name.clone(), lean.clone());
                    model.check_state(&single).is_ok()
                };
                if ok {
                    state.objects.insert(name.clone(), lean);
                    notes.push(format!(
                        "{name}: conteúdo observado não entra no estado ({e})"
                    ));
                } else {
                    notes.push(format!(
                        "{name}: estado observado recusado pelo modelo ({e})"
                    ));
                }
            }
        }
    }
    if state.objects.len() > 1 {
        if let Err(e) = model.check_state(&state) {
            notes.push(format!("o estado completo não passou na validação: {e}"));
        }
    }
    (state, notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(t: f64, object: &str, kind: EventKind) -> Event {
        Event {
            t,
            object: object.into(),
            kind,
        }
    }

    #[test]
    fn windows_by_hand() {
        let measures = vec![
            ("m".to_string(), Measure::Outputs),
            ("m".to_string(), Measure::State),
            ("f".to_string(), Measure::Content),
        ];
        let mut a = Accumulator::new(10.0, 0.0, &measures, Some(0.0));
        let run = EventKind::State {
            class: StateClass::Running,
            debounce: 0.0,
        };
        let stop = EventKind::State {
            class: StateClass::Stopped,
            debounce: 0.0,
        };
        a.apply(&ev(0.0, "m", run.clone()));
        a.apply(&ev(0.0, "f", EventKind::Content(2.0)));
        a.apply(&ev(4.0, "f", EventKind::Content(4.0)));
        a.apply(&ev(
            5.0,
            "m",
            EventKind::Count {
                measure: Measure::Outputs,
                delta: 1.0,
            },
        ));
        a.apply(&ev(6.0, "m", stop));
        a.apply(&ev(
            12.0,
            "m",
            EventKind::Count {
                measure: Measure::Outputs,
                delta: 2.0,
            },
        ));
        a.advance(20.0);
        let w = a.closed();
        assert_eq!(w.len(), 2);
        assert_eq!(w[0].values["m.vazao"], 1.0);
        assert!((w[0].values["m.utilizacao"] - 0.6).abs() < 1e-12);
        // 2·4 + 4·6 = 32 → 3.2.
        assert!((w[0].values["f.wip"] - 3.2).abs() < 1e-12);
        assert_eq!(w[1].values["m.vazao"], 2.0);
        assert_eq!(w[1].values["m.utilizacao"], 0.0);
        assert_eq!(w[1].values["f.wip"], 4.0);
    }

    #[test]
    fn debounce_delays_the_state_and_drops_glitches() {
        let measures = vec![("m".to_string(), Measure::State)];
        let mut a = Accumulator::new(10.0, 0.0, &measures, None);
        let st = |class| EventKind::State {
            class,
            debounce: 2.0,
        };
        a.apply(&ev(0.0, "m", st(StateClass::Running)));
        // Down for 1 unit only: a glitch.
        a.apply(&ev(3.0, "m", st(StateClass::Down)));
        a.apply(&ev(4.0, "m", st(StateClass::Running)));
        a.advance(10.0);
        let w = &a.closed()[0];
        // Unknown for the first 2 units: the window is not complete.
        assert!(!w.values.contains_key("m.utilizacao"));
        a.advance(20.0);
        let w = &a.closed()[1];
        assert_eq!(w.values["m.utilizacao"], 1.0);
        assert_eq!(w.values["m.quebra"], 0.0);
    }

    #[test]
    fn counters_need_a_baseline_before_the_window() {
        let measures = vec![("m".to_string(), Measure::Outputs)];
        let mut a = Accumulator::new(10.0, 0.0, &measures, None);
        a.baseline("m", Measure::Outputs, 5.0);
        a.advance(20.0);
        assert!(!a.closed()[0].values.contains_key("m.vazao"));
        assert_eq!(a.closed()[1].values["m.vazao"], 0.0);
    }
}
