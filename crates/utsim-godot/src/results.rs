//! The "Resultados" panel of the Godot interface (G3), with no Godot types: the numbers
//! of a set of replications reshaped for the screen as JSON values, which the bridge turns
//! into dictionaries and arrays (`lib.rs`, `variant_of_json`).
//!
//! Every number comes from `utsim_model::analysis` (the same as `utsim run`, the HTML
//! report and the Bevy results window): the table per object with the 95% intervals, the
//! bottleneck overall and within shift, utilisation and state bars, the content series
//! and the pooled histograms. The output analyses (warm-up, replications needed, Little's
//! law, shifting bottleneck), the scenario comparison and the optimisation are shaped
//! here too; the runs they need go through [`crate::jobs`].
//!
//! Numbers that are not finite (the half-width of one replication) become `null`.

use std::collections::BTreeMap;

use serde_json::{Map, Value, json};
use utsim_core::RunReport;
use utsim_core::State;
use utsim_core::stats::{Histogram, Summary};
use utsim_model::Model;
use utsim_model::analysis::{
    self, BottleneckShare, LittleReport, ObjectStats, WarmupAnalysis, bottleneck,
    bottleneck_in_shift,
};
use utsim_model::experiment::{ExperimentResult, Scenario};
use utsim_opt::{Algorithm, Domain, Objective, OptProblem, OptResult, Sense, Variable};
use utsim_scene::style;
use utsim_stats::DEFAULT_LEVEL;
use utsim_stats::little::LittleCheck;

/// Content curves of single replications drawn under the mean (more would be noise).
pub const MAX_CURVES: usize = 10;

/// A finite number, or `null`.
pub fn num(x: f64) -> Value {
    if x.is_finite() { json!(x) } else { Value::Null }
}

/// `{mean, half_width, std_dev, n}`.
pub fn summary(s: &Summary) -> Value {
    json!({"mean": num(s.mean), "half_width": num(s.half_width), "std_dev": num(s.std_dev), "n": s.n})
}

fn opt_summary(s: &Option<Summary>) -> Value {
    s.as_ref().map_or(Value::Null, summary)
}

const STATES: [State; 21] = [
    State::Idle,
    State::Empty,
    State::Generating,
    State::Releasing,
    State::Collecting,
    State::Setup,
    State::Processing,
    State::Blocked,
    State::WaitingOperator,
    State::WaitingTransport,
    State::Down,
    State::OffShift,
    State::TravelEmpty,
    State::TravelLoaded,
    State::Loading,
    State::Unloading,
    State::Utilize,
    State::Stopped,
    State::WaitingRepair,
    State::Maintenance,
    State::Resting,
];

/// The state called `name` in the reports (`"processing"`...).
pub fn state_named(name: &str) -> Option<State> {
    STATES.iter().copied().find(|s| s.name() == name)
}

/// Portuguese label of a report state name.
pub fn state_label(name: &str) -> String {
    match state_named(name) {
        Some(s) => style::state_label(Some(s)).to_string(),
        None => name.to_string(),
    }
}

/// Portuguese label of a report kind (`"processor"`...).
pub fn kind_label(kind: &str) -> &str {
    match kind {
        "source" => "chegada",
        "queue" => "fila",
        "processor" => "processo",
        "combiner" => "montagem",
        "separator" => "desmontagem",
        "sink" => "saída",
        "transporter" => "transportador",
        "dispatcher" => "despachante",
        "conveyor" => "esteira",
        "rack" => "porta-paletes",
        "span" => "medida de tempo",
        "flow" => "fluxo",
        "fluid" => "fluidos",
        other => other,
    }
}

/// The replications of one run, with their analysis.
#[derive(Clone, Debug)]
pub struct RunOutput {
    pub model: Model,
    pub reports: Vec<RunReport>,
    pub stats: Vec<ObjectStats>,
}

impl RunOutput {
    pub fn new(model: Model, reports: Vec<RunReport>) -> RunOutput {
        let stats = analysis::object_stats_at(&model, &reports, DEFAULT_LEVEL);
        RunOutput {
            model,
            reports,
            stats,
        }
    }

    /// The same run with `more` replications appended (the ones after the last).
    pub fn with_more(mut self, more: Vec<RunReport>) -> RunOutput {
        self.model.run.replications += more.len() as u32;
        self.reports.extend(more);
        RunOutput::new(self.model, self.reports)
    }

    pub fn bottleneck(&self) -> Option<&str> {
        bottleneck(&self.stats).map(|i| self.stats[i].name.as_str())
    }

    pub fn bottleneck_in_shift(&self) -> Option<&str> {
        bottleneck_in_shift(&self.stats).map(|i| self.stats[i].name.as_str())
    }

    /// Run settings and the two bottlenecks.
    pub fn summary(&self) -> Value {
        let run = &self.model.run;
        json!({
            "model": self.model.name,
            "replications": self.reports.len(),
            "seed": run.seed,
            "stop_time": run.stop_time,
            "warmup": run.warmup,
            "end_time": self.reports.first().map_or(0.0, |r| r.end_time),
            "sampled": self.reports.iter().any(|r| r.blocks.iter().any(|b| !b.series.is_empty())),
            "bottleneck": self.bottleneck(),
            "bottleneck_in_shift": self.bottleneck_in_shift(),
        })
    }

    /// One row per object: entries, exits, mean content, stay time and (resources)
    /// utilisation overall and within shift, each `{mean, half_width}`.
    pub fn table(&self) -> Value {
        let (b, bs) = (self.bottleneck(), self.bottleneck_in_shift());
        Value::Array(
            self.stats
                .iter()
                .map(|s| {
                    json!({
                        "name": s.name,
                        "kind": s.kind,
                        "kind_label": kind_label(&s.kind),
                        "resource": s.is_resource(),
                        "input": summary(&s.input),
                        "output": summary(&s.output),
                        "content": summary(&s.content),
                        "staytime": summary(&s.staytime),
                        "utilization": opt_summary(&s.utilization),
                        "utilization_in_shift": opt_summary(&s.utilization_in_shift),
                        "utilization_incl_blocked": opt_summary(&s.utilization_incl_blocked),
                        "bottleneck": b == Some(s.name.as_str()),
                        "bottleneck_in_shift": bs == Some(s.name.as_str()),
                    })
                })
                .collect(),
        )
    }

    /// Every metric of `utsim run`, sorted by key.
    pub fn metrics(&self) -> Value {
        Value::Array(
            utsim_core::summarize(&self.reports)
                .iter()
                .map(|(k, s)| {
                    let mut v = summary(s);
                    v["key"] = json!(k);
                    v
                })
                .collect(),
        )
    }

    /// Utilisation bars of the resources: `which` is `trabalho` (work), `turno` (within
    /// shift) or `bloqueio` (work plus blocked).
    pub fn utilization(&self, which: &str) -> Value {
        let b = self.bottleneck();
        Value::Array(
            self.stats
                .iter()
                .filter_map(|s| {
                    let u = match which {
                        "turno" => s.utilization_in_shift.or(s.utilization),
                        "bloqueio" => s.utilization_incl_blocked,
                        _ => s.utilization,
                    }?;
                    Some(json!({
                        "name": s.name,
                        "mean": num(u.mean),
                        "half_width": num(u.half_width),
                        "bottleneck": b == Some(s.name.as_str()),
                    }))
                })
                .collect(),
        )
    }

    /// One stacked bar per object with states: `{name, segments: [{state, label,
    /// fraction, color}]}`, the fractions summing to 1, coloured as in the 3D view.
    pub fn states(&self) -> Value {
        Value::Array(
            self.stats
                .iter()
                .filter(|s| !s.states.is_empty())
                .map(|s| {
                    let total: f64 = s.states.iter().map(|(_, v)| v.mean).sum();
                    let segments: Vec<Value> = s
                        .states
                        .iter()
                        .map(|(state, v)| {
                            let f = if total > 0.0 { v.mean / total } else { 0.0 };
                            json!({
                                "state": state,
                                "label": state_label(state),
                                "fraction": num(f),
                                "color": style::state_color(state_named(state)),
                            })
                        })
                        .collect();
                    json!({"name": s.name, "segments": segments})
                })
                .collect(),
        )
    }

    /// Content of `object` over time: the mean across replications and up to
    /// [`MAX_CURVES`] single replications, `[[t, value], ...]`.
    pub fn series(&self, object: &str) -> Value {
        let s = analysis::content_series(&self.reports, object);
        json!({
            "object": object,
            "mean": s.mean,
            "replications": s.replications.iter().take(MAX_CURVES).collect::<Vec<_>>(),
        })
    }

    /// Tallies of `object` with a histogram (`staytime` first).
    pub fn tallies(&self, object: &str) -> Vec<String> {
        analysis::histogram_tallies(&self.reports, object)
    }

    /// The pooled histogram of `tally` of `object`, or `null`.
    pub fn histogram(&self, object: &str, tally: &str) -> Value {
        analysis::pooled_histogram(&self.reports, object, tally)
            .map_or(Value::Null, |h| histogram(&h))
    }

    /// Replications needed for `target` (`"2%"` relative to the mean, or an absolute
    /// half-width) on `metrics` (empty: the default ones, utilisations, contents and
    /// stay times that are not zero). `more` is the largest number still to run.
    pub fn precision(&self, target: &str, metrics: &[String]) -> Result<Value, String> {
        let t = analysis::parse_target(target).ok_or_else(|| {
            format!("precisão \"{target}\" inválida: use, por exemplo, 2% ou 0.5")
        })?;
        let metrics = if metrics.is_empty() {
            analysis::default_precision_metrics(&self.reports)
        } else {
            metrics.to_vec()
        };
        let rows = analysis::precision(&self.reports, &metrics, t, DEFAULT_LEVEL);
        let more = rows.iter().filter_map(|(_, p)| p.more).max().unwrap_or(0);
        let rows: Vec<Value> = rows
            .iter()
            .map(|(k, p)| {
                json!({
                    "metric": k,
                    "current": summary(&p.current),
                    "target": num(p.target),
                    "needed": p.needed,
                    "more": p.more,
                })
            })
            .collect();
        Ok(json!({"target": target, "rows": rows, "more": more}))
    }
}

pub fn histogram(h: &Histogram) -> Value {
    json!({"min": num(h.min), "max": num(h.max), "bin_width": num(h.bin_width), "counts": h.counts})
}

// ---------------------------------------------------------------------------------------
// Output analyses
// ---------------------------------------------------------------------------------------

/// Welch's curve and the MSER-5 cut, with the suggested `run.warmup`.
pub fn warmup(a: &WarmupAnalysis, label: &str) -> Value {
    let welch: Vec<[f64; 2]> = a
        .welch
        .iter()
        .enumerate()
        .filter_map(|(i, w)| a.times.get(i).map(|t| [*t, *w]))
        .collect();
    let mean: Vec<[f64; 2]> = a.times.iter().zip(&a.mean).map(|(t, m)| [*t, *m]).collect();
    json!({
        "series": label,
        "window": a.window,
        "mean": mean,
        "welch": welch,
        "suggested": a.suggested,
        "cut": a.mser.as_ref().map(|m| m.cut),
        "too_short": a.mser.as_ref().is_some_and(|m| m.too_short),
        "points": a.mean.len(),
    })
}

/// Traffic light of Little's law: `verde` (the interval of `L - λW` holds zero),
/// `amarelo` (it does not, but the relative gap is within the tolerance: many
/// replications make tiny end effects "significant"), `vermelho` (it does not close).
pub fn little_light(c: &LittleCheck) -> &'static str {
    let d = &c.difference;
    let holds_zero =
        !d.half_width.is_finite() || (d.mean - d.half_width <= 0.0 && 0.0 <= d.mean + d.half_width);
    if holds_zero {
        "verde"
    } else if c.holds {
        "amarelo"
    } else {
        "vermelho"
    }
}

fn little_check(c: &LittleCheck) -> Value {
    json!({
        "l": summary(&c.l),
        "lambda": summary(&c.lambda),
        "w": summary(&c.w),
        "lambda_w": summary(&c.lambda_w),
        "difference": summary(&c.difference),
        "relative": num(c.relative),
        "holds": c.holds,
        "light": little_light(c),
    })
}

pub fn little(r: &LittleReport) -> Value {
    let objects: Vec<Value> = r
        .objects
        .iter()
        .map(|o| {
            let mut v = little_check(&o.check);
            v["name"] = json!(o.name);
            v["kind"] = json!(o.kind);
            v
        })
        .collect();
    json!({
        "objects": objects,
        "system": r.system.as_ref().map(little_check),
        "transit": r.transit.as_ref().map(summary),
    })
}

/// Fraction of the time each resource was the sole or a shared bottleneck.
pub fn shifting(rows: &[BottleneckShare]) -> Value {
    Value::Array(
        rows.iter()
            .map(|r| {
                json!({
                    "name": r.name,
                    "kind": r.kind,
                    "sole": summary(&r.sole),
                    "shifting": summary(&r.shifting),
                    "total": num(r.total()),
                })
            })
            .collect(),
    )
}

// ---------------------------------------------------------------------------------------
// Scenarios and optimisation
// ---------------------------------------------------------------------------------------

/// Scenarios written as text, one block each:
///
/// ```text
/// [mais um servidor]
/// maquina.capacity = 2
/// run.stop_time = 960.0
/// ```
///
/// Lines starting with `#` and blank lines are ignored. When the first block changes
/// something, a `base` scenario (the model as it is) comes first, so every scenario is
/// compared with the model.
pub fn parse_scenarios(text: &str) -> Result<Vec<Scenario>, String> {
    let mut out: Vec<Scenario> = Vec::new();
    for (k, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            let name = name.trim();
            if name.is_empty() {
                return Err(format!("linha {}: cenário sem nome", k + 1));
            }
            if out.iter().any(|s| s.name == name) {
                return Err(format!("linha {}: cenário \"{name}\" repetido", k + 1));
            }
            out.push(Scenario {
                name: name.to_string(),
                set: BTreeMap::new(),
            });
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!(
                "linha {}: use \"[nome]\" para começar um cenário e \"chave = valor\" para mudar um campo",
                k + 1
            ));
        };
        let (key, value) = (key.trim(), value.trim());
        if key.is_empty() || value.is_empty() {
            return Err(format!("linha {}: chave ou valor vazio", k + 1));
        }
        let Some(current) = out.last_mut() else {
            return Err(format!(
                "linha {}: \"{key}\" antes do primeiro [cenário]",
                k + 1
            ));
        };
        current.set.insert(key.to_string(), value.to_string());
    }
    if out.is_empty() {
        return Err("nenhum cenário: escreva [nome] e as mudanças embaixo".into());
    }
    if !out[0].set.is_empty() && out.iter().all(|s| s.name != "base") {
        out.insert(
            0,
            Scenario {
                name: "base".into(),
                set: BTreeMap::new(),
            },
        );
    }
    Ok(out)
}

/// The comparison table: per metric, each scenario's `{mean, half_width}` and its paired
/// difference to the first one (common random numbers), marked when significant.
pub fn scenarios(r: &ExperimentResult) -> Value {
    let names: Vec<&str> = r.scenarios.iter().map(|s| s.name.as_str()).collect();
    let rows: Vec<Value> = r
        .metric_names()
        .iter()
        .map(|m| {
            let values: Vec<Value> = r
                .scenarios
                .iter()
                .map(|s| s.summary.get(m).map_or(Value::Null, summary))
                .collect();
            let diffs: Vec<Value> = (0..r.scenarios.len())
                .map(|i| {
                    r.difference_to_first(i, m).map_or(Value::Null, |d| {
                        let mut v = summary(&d);
                        v["significant"] = json!(analysis::is_significant(&d));
                        v
                    })
                })
                .collect();
            json!({"metric": m, "values": values, "differences": diffs})
        })
        .collect();
    json!({"scenarios": names, "rows": rows})
}

/// The optimisation form of the panel.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OptForm {
    /// Metric to minimise or maximise (`saida.time_in_system.mean`).
    pub metric: String,
    pub maximize: bool,
    /// One variable per line: `chave mínimo máximo [passo]` (integers), or with a
    /// decimal point (continuous, five values on the grid).
    pub variables: String,
    /// `grade`, `recozimento` or `genetico`.
    pub method: String,
    pub replications: u32,
    pub budget: usize,
}

fn number(text: &str) -> Option<f64> {
    text.replace(',', ".").parse().ok()
}

impl OptForm {
    pub fn build(&self) -> Result<OptProblem, String> {
        let metric = self.metric.trim();
        if metric.is_empty() {
            return Err("escolha a métrica do objetivo".into());
        }
        let mut variables = Vec::new();
        for (k, line) in self.variables.lines().enumerate() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() || parts[0].starts_with('#') {
                continue;
            }
            let bad = || {
                format!(
                    "variável na linha {}: use \"chave mínimo máximo [passo]\"",
                    k + 1
                )
            };
            if !(3..=4).contains(&parts.len()) {
                return Err(bad());
            }
            let ints: Option<Vec<i64>> = parts[1..].iter().map(|p| p.parse().ok()).collect();
            let domain = match ints {
                Some(v) => Domain::Integer {
                    min: v[0],
                    max: v[1],
                    step: v.get(2).copied().unwrap_or(1).max(1),
                },
                None => {
                    let (Some(min), Some(max)) = (number(parts[1]), number(parts[2])) else {
                        return Err(bad());
                    };
                    let steps = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
                    Domain::Continuous {
                        min,
                        max,
                        template: "{x}".into(),
                        steps,
                    }
                }
            };
            variables.push(Variable {
                key: parts[0].to_string(),
                domain,
            });
        }
        if variables.is_empty() {
            return Err("escreva ao menos uma variável: chave mínimo máximo [passo]".into());
        }
        let algorithm = match self.method.as_str() {
            "recozimento" => Algorithm::Annealing { iterations: 60 },
            "genetico" => Algorithm::Genetic {
                population: 12,
                generations: 8,
            },
            _ => Algorithm::Grid,
        };
        Ok(OptProblem {
            name: "otimizacao".into(),
            model: String::new(),
            replications: self.replications.max(2),
            variables,
            objective: Objective {
                sense: if self.maximize {
                    Sense::Maximize
                } else {
                    Sense::Minimize
                },
                terms: vec![(metric.to_string(), 1.0)],
                costs: Vec::new(),
            },
            constraints: Vec::new(),
            algorithm,
            budget: self.budget.max(1),
            seed: 1,
            refine: None,
        })
    }
}

fn point_text(point: &BTreeMap<String, String>) -> String {
    point
        .iter()
        .map(|(k, v)| format!("{k} = {v}"))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn optimized(r: &OptResult) -> Value {
    let evaluations: Vec<Value> = r
        .final_evaluations()
        .iter()
        .enumerate()
        .map(|(i, e)| {
            json!({
                "point": point_text(&e.point),
                "objective": summary(&e.objective),
                "feasible": e.feasible,
                "best": r.best == Some(i),
                "equivalent": r.equivalent.contains(&i),
            })
        })
        .collect();
    json!({
        "evaluations": evaluations,
        "best": r.best_evaluation().map(|e| point_text(&e.point)),
        "explanation": r.explanation,
        "history": r.history,
        "simulated": r.evaluations.len(),
    })
}

/// A JSON object with the given entries (for the bridge's own small results).
pub fn object(entries: &[(&str, Value)]) -> Value {
    let mut m = Map::new();
    for (k, v) in entries {
        m.insert((*k).to_string(), v.clone());
    }
    Value::Object(m)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::path::Path;

    pub(crate) fn example(name: &str) -> Model {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples")
            .join(name);
        Model::load(path).unwrap()
    }

    fn run(name: &str, reps: u32) -> RunOutput {
        let mut m = example(name);
        m.run.replications = reps;
        let reports = m.run_replications().unwrap();
        RunOutput::new(m, reports)
    }

    #[test]
    fn the_table_matches_utsim_run_and_marks_the_bottleneck() {
        let out = run("servidores.ron", 4);
        let table = out.table();
        let rows = table.as_array().unwrap();
        assert_eq!(rows.len(), out.stats.len());
        let summary = utsim_core::summarize(&out.reports);
        for r in rows {
            let name = r["name"].as_str().unwrap();
            let key = format!("{name}.output");
            if let Some(s) = summary.get(&key) {
                assert_eq!(r["output"]["mean"].as_f64().unwrap(), s.mean, "{key}");
            }
        }
        let b = out.bottleneck().unwrap().to_string();
        let marked: Vec<_> = rows.iter().filter(|r| r["bottleneck"] == true).collect();
        assert_eq!(marked.len(), 1);
        assert_eq!(marked[0]["name"], json!(b));
        assert_eq!(out.summary()["bottleneck"], json!(b));
        let bars = out.utilization("trabalho");
        assert!(
            bars.as_array()
                .unwrap()
                .iter()
                .any(|v| v["name"] == json!(b))
        );
    }

    #[test]
    fn states_sum_to_one_and_have_the_colours_of_the_scene() {
        let out = run("servidores.ron", 2);
        for row in out.states().as_array().unwrap() {
            let segs = row["segments"].as_array().unwrap();
            let total: f64 = segs.iter().map(|s| s["fraction"].as_f64().unwrap()).sum();
            assert!((total - 1.0).abs() < 1e-9, "{row}");
            for s in segs {
                let state = s["state"].as_str().unwrap();
                let want = style::state_color(state_named(state));
                let got: Vec<f64> = serde_json::from_value(s["color"].clone()).unwrap();
                assert!((got[0] - f64::from(want[0])).abs() < 1e-6);
            }
        }
        assert_eq!(state_label("processing"), "processando");
        assert_eq!(state_label("outro"), "outro");
    }

    #[test]
    fn more_replications_append_to_the_ones_already_run() {
        let a = run("mm1.ron", 3);
        let mut m = a.model.clone();
        let more: Vec<RunReport> = (3..5)
            .map(|r| {
                let mut sim = m.build(r);
                m.drive(&mut sim).unwrap()
            })
            .collect();
        let merged = a.with_more(more);
        m.run.replications = 5;
        let direct = RunOutput::new(m.clone(), m.run_replications().unwrap());
        assert_eq!(merged.model.run.replications, 5);
        assert_eq!(merged.table(), direct.table());
        let p = merged.precision("50%", &[]).unwrap();
        assert!(!p["rows"].as_array().unwrap().is_empty());
        assert!(merged.precision("muito", &[]).is_err());
    }

    #[test]
    fn scenarios_are_read_from_text_with_the_base_first() {
        let s = parse_scenarios(
            "# comparação\n[dois]\nservidor.capacity = 2\n\n[três]\nservidor.capacity = 3\n",
        )
        .unwrap();
        assert_eq!(
            s.iter().map(|x| x.name.as_str()).collect::<Vec<_>>(),
            ["base", "dois", "três"]
        );
        assert_eq!(s[1].set["servidor.capacity"], "2");
        assert!(parse_scenarios("a = 1").is_err());
        assert!(parse_scenarios("[a]\n[a]").is_err());
        assert!(parse_scenarios("[a]\nsem igual").is_err());
        assert!(parse_scenarios("").is_err());
        let only = parse_scenarios("[atual]").unwrap();
        assert_eq!(only.len(), 1);
    }

    #[test]
    fn the_optimisation_form_builds_the_problem() {
        let f = OptForm {
            metric: "saida.time_in_system.mean".into(),
            maximize: false,
            variables: "servidor.capacity 1 3\nchegada.interarrival 0.5 1.5".into(),
            method: "grade".into(),
            replications: 3,
            budget: 20,
        };
        let p = f.build().unwrap();
        assert_eq!(p.variables.len(), 2);
        assert!(matches!(
            p.variables[0].domain,
            Domain::Integer {
                min: 1,
                max: 3,
                step: 1
            }
        ));
        assert!(matches!(p.variables[1].domain, Domain::Continuous { .. }));
        assert!(OptForm::default().build().is_err());
        let mut g = f.clone();
        g.variables = "x 1".into();
        assert!(g.build().is_err());
    }

    #[test]
    fn little_lights() {
        let ok = utsim_stats::little::check(&[2.0, 2.1], &[1.0, 1.0], &[2.0, 2.1], 0.95, 0.02);
        assert_eq!(little_light(&ok), "verde");
        let bad = utsim_stats::little::check(
            &[2.0, 2.0, 2.0],
            &[1.0, 1.0, 1.0],
            &[1.0, 1.01, 0.99],
            0.95,
            0.02,
        );
        assert_eq!(little_light(&bad), "vermelho");
    }
}
