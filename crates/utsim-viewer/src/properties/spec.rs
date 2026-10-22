//! Pure description of the editable fields (no egui): which fields each object kind
//! has, how to read their current value as RON text, and distribution families.

use std::f64::consts::PI;

use utsim_core::Dist;
use utsim_model::{ObjectDef, ObjectKind, RunSettings};

use super::ron_text as ron;

/// How a field is edited.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Widget {
    Float,
    UInt,
    Bool,
    /// Integer that can be switched off (`None`).
    OptUInt,
    /// One of a fixed set of variants.
    Choice,
    Dist,
    OptDist,
    Components,
    Labels,
    Output,
    Downtime,
    Shifts,
    Batch,
    /// Number that can be switched off (`None`).
    OptFloat,
    /// Names of the transporters in a dispatcher's team.
    Team,
    /// Processor operators section (`None` or a team, count and phases).
    Operators,
    /// Transporter/dispatcher that carries items out of the object (`None` = instant).
    TransportRef,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldSpec {
    /// Field name as used in `Command::Set` keys (`<object>.<key>`).
    pub key: &'static str,
    pub label: &'static str,
    pub widget: Widget,
}

const fn spec(key: &'static str, label: &'static str, widget: Widget) -> FieldSpec {
    FieldSpec { key, label, widget }
}

pub fn kind_name(kind: &ObjectKind) -> &'static str {
    match kind {
        ObjectKind::Source { .. } => "Chegada",
        ObjectKind::Queue { .. } => "Fila",
        ObjectKind::Processor { .. } => "Processo",
        ObjectKind::Combiner { .. } => "Montagem",
        ObjectKind::Separator { .. } => "Desmontagem",
        ObjectKind::Sink => "Saída",
        ObjectKind::Transporter { .. } => "Transportador",
        ObjectKind::Dispatcher { .. } => "Despachante",
    }
}

/// Fields specific to the kind of object.
pub fn kind_fields(kind: &ObjectKind) -> Vec<FieldSpec> {
    use Widget::*;
    match kind {
        ObjectKind::Source { .. } => vec![
            spec("interarrival", "Tempo entre chegadas", Dist),
            spec("first_at_zero", "Primeira chegada em t = 0", Bool),
            spec("max_arrivals", "Máximo de chegadas", OptUInt),
            spec("on_blocked", "Se a saída estiver bloqueada", Choice),
            spec("labels", "Rótulos", Labels),
        ],
        ObjectKind::Queue { .. } => vec![
            spec("capacity", "Capacidade", OptUInt),
            spec("discipline", "Disciplina", Choice),
            spec("batch", "Lote", Batch),
        ],
        ObjectKind::Processor { .. } => vec![
            spec("capacity", "Capacidade", UInt),
            spec("setup", "Preparação (setup)", OptDist),
            spec("process", "Tempo de processamento", Dist),
            spec("operators", "Operadores", Operators),
        ],
        ObjectKind::Combiner { .. } => vec![
            spec("components", "Componentes por entrada", Components),
            spec("mode", "Modo", Choice),
            spec("setup", "Preparação (setup)", OptDist),
            spec("process", "Tempo de processamento", Dist),
        ],
        ObjectKind::Separator { .. } => vec![
            spec("mode", "Modo", Choice),
            spec("process", "Tempo de processamento", Dist),
        ],
        ObjectKind::Sink => Vec::new(),
        ObjectKind::Transporter { .. } => vec![
            spec("speed", "Velocidade máxima", Float),
            spec("acceleration", "Aceleração", OptFloat),
            spec("deceleration", "Desaceleração", OptFloat),
            spec("capacity", "Capacidade (itens)", UInt),
            spec("load_time", "Tempo de carga", Dist),
            spec("unload_time", "Tempo de descarga", Dist),
            spec("home", "Voltar à base quando ocioso", Bool),
        ],
        ObjectKind::Dispatcher { .. } => vec![
            spec("team", "Equipe", Team),
            spec("rule", "Regra de escolha", Choice),
        ],
    }
}

/// Fields every object has. Mobile resources (transporters, dispatchers) are not
/// wired by ports, so they have no output rule and no `transport`.
pub fn common_fields(kind: &ObjectKind) -> Vec<FieldSpec> {
    let mut fields = Vec::new();
    if !utsim_model::is_mobile(kind) {
        fields.push(spec("output", "Regra de saída", Widget::Output));
    }
    fields.push(spec("downtime", "Quebras", Widget::Downtime));
    fields.push(spec("shifts", "Turnos", Widget::Shifts));
    if !utsim_model::is_mobile(kind) {
        fields.push(spec("transport", "Transporte", Widget::TransportRef));
    }
    fields
}

/// Every editable field of an object, kind-specific first.
pub fn all_fields(kind: &ObjectKind) -> Vec<FieldSpec> {
    let mut fields = kind_fields(kind);
    fields.extend(common_fields(kind));
    fields
}

/// Names of the objects an object can point to with `transport`/`operators.team`:
/// every Transporter or Dispatcher of the model, in model order.
pub fn mobile_names(objects: &[ObjectDef]) -> Vec<&str> {
    objects
        .iter()
        .filter(|o| utsim_model::is_mobile(&o.kind))
        .map(|o| o.name.as_str())
        .collect()
}

/// Names of the Transporters of the model (candidates for a dispatcher's team).
pub fn transporter_names(objects: &[ObjectDef]) -> Vec<&str> {
    objects
        .iter()
        .filter(|o| matches!(o.kind, ObjectKind::Transporter { .. }))
        .map(|o| o.name.as_str())
        .collect()
}

/// Dispatch rules as `(ron text, Portuguese label)`.
pub const DISPATCH_RULES: [(&str, &str); 3] = [
    ("FirstAvailable", "Primeiro livre"),
    ("RoundRobin", "Rodízio"),
    ("Nearest", "Mais próximo"),
];

/// Fields of the `run` section that scenario patches (and thus `Set`) understand.
pub fn run_fields() -> Vec<FieldSpec> {
    vec![
        spec("stop_time", "Tempo de simulação", Widget::Float),
        spec("warmup", "Aquecimento", Widget::Float),
        spec("seed", "Semente", Widget::UInt),
        spec("replications", "Replicações", Widget::UInt),
    ]
}

pub fn read_run(run: &RunSettings, key: &str) -> Option<String> {
    Some(match key {
        "stop_time" => ron::float(run.stop_time),
        "warmup" => ron::float(run.warmup),
        "seed" => run.seed.to_string(),
        "replications" => run.replications.to_string(),
        _ => return None,
    })
}

/// Current value of field `key` as RON text, ready for `Command::Set`.
pub fn read_field(obj: &ObjectDef, key: &str) -> Option<String> {
    match key {
        "output" => return Some(ron::output(&obj.output)),
        "downtime" => return Some(ron::option(&obj.downtime, ron::downtime)),
        "shifts" => return Some(ron::option(&obj.shifts, ron::shifts)),
        "transport" => return Some(ron::option(&obj.transport, |t| ron::string(t))),
        _ => {}
    }
    Some(match (&obj.kind, key) {
        (ObjectKind::Processor { operators, .. }, "operators") => ron::option(operators, |o| {
            ron::operators(&o.team, o.count, o.setup, o.process)
        }),
        (ObjectKind::Transporter { speed, .. }, "speed") => ron::float(*speed),
        (ObjectKind::Transporter { acceleration, .. }, "acceleration") => {
            ron::option(acceleration, |v| ron::float(*v))
        }
        (ObjectKind::Transporter { deceleration, .. }, "deceleration") => {
            ron::option(deceleration, |v| ron::float(*v))
        }
        (ObjectKind::Transporter { capacity, .. }, "capacity") => capacity.to_string(),
        (ObjectKind::Transporter { load_time, .. }, "load_time") => ron::dist(load_time),
        (ObjectKind::Transporter { unload_time, .. }, "unload_time") => ron::dist(unload_time),
        (ObjectKind::Transporter { home, .. }, "home") => home.to_string(),
        (ObjectKind::Dispatcher { team, .. }, "team") => ron::strings(team),
        (ObjectKind::Dispatcher { rule, .. }, "rule") => ron::debug(rule),
        (ObjectKind::Source { interarrival, .. }, "interarrival") => ron::dist(interarrival),
        (ObjectKind::Source { first_at_zero, .. }, "first_at_zero") => first_at_zero.to_string(),
        (ObjectKind::Source { max_arrivals, .. }, "max_arrivals") => {
            ron::option(max_arrivals, |v| v.to_string())
        }
        (ObjectKind::Source { labels, .. }, "labels") => ron::labels(labels),
        (ObjectKind::Source { on_blocked, .. }, "on_blocked") => ron::debug(on_blocked),
        (ObjectKind::Queue { capacity, .. }, "capacity") => {
            ron::option(capacity, |v| v.to_string())
        }
        (ObjectKind::Queue { discipline, .. }, "discipline") => ron::debug(discipline),
        (ObjectKind::Queue { batch, .. }, "batch") => {
            ron::option(batch, |b| ron::batch(b.size, b.max_wait, b.flush))
        }
        (ObjectKind::Processor { capacity, .. }, "capacity") => capacity.to_string(),
        (ObjectKind::Processor { setup, .. }, "setup")
        | (ObjectKind::Combiner { setup, .. }, "setup") => ron::option(setup, ron::dist),
        (ObjectKind::Processor { process, .. }, "process")
        | (ObjectKind::Combiner { process, .. }, "process")
        | (ObjectKind::Separator { process, .. }, "process") => ron::dist(process),
        (ObjectKind::Combiner { components, .. }, "components") => ron::usizes(components),
        (ObjectKind::Combiner { mode, .. }, "mode") => ron::debug(mode),
        (ObjectKind::Separator { mode, .. }, "mode") => ron::debug(mode),
        _ => return None,
    })
}

/// Portuguese names of the distribution families, in menu order.
pub const FAMILIES: [&str; 20] = [
    "Constante",
    "Uniforme",
    "Exponencial",
    "Triangular",
    "Normal",
    "Lognormal",
    "Gama",
    "Erlang",
    "Weibull",
    "Beta",
    "Log-logística",
    "Pearson V",
    "Pearson VI",
    "Discreta",
    "Empírica",
    "Poisson",
    "Geométrica",
    "Binomial",
    "Binomial negativa",
    "Deslocada",
];

pub fn family_index(d: &Dist) -> usize {
    match d {
        Dist::Const(_) => 0,
        Dist::Uniform { .. } => 1,
        Dist::Exp { .. } => 2,
        Dist::Triangular { .. } => 3,
        Dist::Normal { .. } => 4,
        Dist::LogNormal { .. } => 5,
        Dist::Gamma { .. } => 6,
        Dist::Erlang { .. } => 7,
        Dist::Weibull { .. } => 8,
        Dist::Beta { .. } => 9,
        Dist::LogLogistic { .. } => 10,
        Dist::PearsonV { .. } => 11,
        Dist::PearsonVI { .. } => 12,
        Dist::Discrete(_) => 13,
        Dist::Empirical(_) => 14,
        Dist::Poisson { .. } => 15,
        Dist::Geometric { .. } => 16,
        Dist::Binomial { .. } => 17,
        Dist::NegBinomial { .. } => 18,
        Dist::Shifted { .. } => 19,
    }
}

/// A distribution of family `index` with sensible parameters and mean `m` (> 0).
pub fn family_default(index: usize, m: f64) -> Dist {
    match index {
        0 => Dist::Const(m),
        1 => Dist::Uniform {
            min: 0.5 * m,
            max: 1.5 * m,
        },
        2 => Dist::Exp { mean: m },
        3 => Dist::Triangular {
            min: 0.5 * m,
            mode: m,
            max: 1.5 * m,
        },
        4 => Dist::Normal {
            mean: m,
            sd: 0.2 * m,
        },
        5 => Dist::LogNormal {
            mean: m,
            sd: 0.5 * m,
        },
        6 => Dist::Gamma {
            shape: 2.0,
            scale: m / 2.0,
        },
        7 => Dist::Erlang { k: 2, mean: m },
        8 => Dist::Weibull {
            shape: 2.0,
            // Mean of Weibull(2, s) is s * gamma(1.5) = s * sqrt(pi) / 2.
            scale: m / (PI.sqrt() / 2.0),
        },
        9 => Dist::Beta {
            alpha: 2.0,
            beta: 2.0,
            min: 0.0,
            max: 2.0 * m,
        },
        10 => {
            // Mean of LogLogistic(3, s) is s * (pi/3) / sin(pi/3).
            let x = PI / 3.0;
            Dist::LogLogistic {
                shape: 3.0,
                scale: m * x.sin() / x,
            }
        }
        11 => Dist::PearsonV {
            shape: 3.0,
            scale: 2.0 * m,
        },
        12 => Dist::PearsonVI {
            shape1: 2.0,
            shape2: 3.0,
            scale: m,
        },
        13 => Dist::Discrete(vec![(0.5 * m, 1.0), (1.5 * m, 1.0)]),
        14 => Dist::Empirical(vec![0.5 * m, m, 1.5 * m]),
        15 => Dist::Poisson { mean: m },
        16 => Dist::Geometric { p: 1.0 / (1.0 + m) },
        17 => {
            let n = (2.0 * m).ceil().clamp(1.0, f64::from(u32::MAX)) as u32;
            Dist::Binomial {
                n,
                p: (m / f64::from(n)).min(1.0),
            }
        }
        18 => Dist::NegBinomial {
            r: 2.0,
            p: 2.0 / (m + 2.0),
        },
        _ => Dist::Shifted {
            offset: 0.0,
            dist: Box::new(Dist::Exp { mean: m }),
        },
    }
}

/// Switches `current` to family `index`, keeping roughly the same mean when possible.
/// Switching to `Deslocada` wraps the current distribution, which keeps it exactly.
pub fn switch_family(current: &Dist, index: usize) -> Dist {
    if index == 19 {
        return Dist::Shifted {
            offset: 0.0,
            dist: Box::new(current.clone()),
        };
    }
    let mean = current.mean();
    let m = if mean.is_finite() && mean > 0.0 {
        mean
    } else {
        1.0
    };
    family_default(index, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_family_has_a_valid_default_with_the_requested_mean() {
        for (i, name) in FAMILIES.iter().enumerate() {
            for m in [0.4, 1.0, 7.5] {
                let d = family_default(i, m);
                assert_eq!(family_index(&d), i, "{name}");
                assert_eq!(d.validate(), Ok(()), "{name} m={m}: {d:?}");
                // Discrete families round some parameters (Binomial's n).
                let tol = if i == 17 { 0.05 * m + 0.5 } else { 1e-9 * m };
                assert!(
                    (d.mean() - m).abs() <= tol.max(1e-9),
                    "{name}: mean {} vs {m}",
                    d.mean()
                );
            }
        }
    }

    #[test]
    fn switching_keeps_the_mean_and_shifted_wraps() {
        let d = Dist::Gamma {
            shape: 3.0,
            scale: 2.0,
        };
        for (i, name) in FAMILIES.iter().enumerate() {
            let s = switch_family(&d, i);
            assert_eq!(s.validate(), Ok(()), "{name}");
            if i != 17 {
                assert!((s.mean() - 6.0).abs() < 1e-6, "{name}: {s:?}");
            }
        }
        // A zero-mean source falls back to a mean of 1.
        assert_eq!(switch_family(&Dist::Const(0.0), 2), Dist::Exp { mean: 1.0 });
    }
}
