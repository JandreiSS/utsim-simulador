//! The analyses of the dashboard that need more than the replications: the exact
//! state Gantt and the heat map (from the trace of replication 0), the comparison of
//! the scenarios of an experiment and the PERT/CPM Monte Carlo of a part of the model.
//! [`Study`](crate::Study) runs them once the replications end and publishes them as
//! routes next to the others:
//!
//! | route | content |
//! |---|---|
//! | `/api/gantt` | (replaced) exact state timeline of replication 0, from the trace |
//! | `/api/mapa_calor` | time and passages per floor cell, by resource and by class |
//! | `/api/cenarios` | ANOVA, Tukey and Bonferroni of the experiment's scenarios |
//! | `/api/pert` | Monte Carlo of the PERT network: total time, criticality, tornado |
//!
//! The correlation between metrics needs only the replications and is built with the
//! other routes ([`correlation_json`], route `/api/correlacao`).

use std::collections::BTreeMap;

use serde_json::{Value, json};
use utsim_core::RunReport;
use utsim_model::Model;
use utsim_model::analysis;
use utsim_model::experiment::{ExperimentResult, Scenario, run_scenarios};
use utsim_model::replay::Replay;
use utsim_scene::heatmap::{DEFAULT_CELL, HeatGroup, HeatLayer, Heatmap};
use utsim_stats::compare::PairComparison;

use crate::config::{Dashboard, PertSpec};
use crate::num::pt_mean_hw;

/// Most metrics in the correlation matrix (the ones that vary most come first).
pub const MAX_CORRELATION_METRICS: usize = 12;
/// Most rows of the Gantt chart.
pub const MAX_GANTT_ROWS: usize = 40;

/// The experiment whose scenarios the dashboard compares.
#[derive(Clone, Debug, Default)]
pub struct ScenarioSet {
    pub name: String,
    pub scenarios: Vec<Scenario>,
    pub measures: Vec<String>,
}

/// A metric name as people read it: `fila.content.mean` → `fila: conteúdo médio`.
pub fn metric_label(metric: &str) -> String {
    let (obj, rest) = metric.split_once('.').unwrap_or((metric, ""));
    let what = match rest {
        "input" => "entradas".to_string(),
        "output" => "saídas".to_string(),
        "content.mean" => "conteúdo médio".to_string(),
        "content.max" => "conteúdo máximo".to_string(),
        "staytime.mean" => "permanência média".to_string(),
        "utilization" => "ocupação".to_string(),
        "time_in_system.mean" => "tempo no sistema".to_string(),
        r => match r.strip_prefix("state.") {
            Some(st) => format!("fração em {st}"),
            None => r.to_string(),
        },
    };
    if what.is_empty() {
        obj.to_string()
    } else {
        format!("{obj}: {what}")
    }
}

/// `/api/correlacao`: Pearson and Spearman matrix of the default metrics.
pub fn correlation_json(reports: &[RunReport]) -> Value {
    let mut metrics = analysis::default_correlation_metrics(reports);
    metrics.truncate(MAX_CORRELATION_METRICS);
    let m = analysis::correlation(reports, &metrics);
    let pairs: Vec<Value> = m
        .strongest()
        .into_iter()
        .take(10)
        .map(|p| {
            json!({
                "a": metric_label(&p.a), "b": metric_label(&p.b),
                "pearson": p.pearson, "spearman": p.spearman,
                "p_pearson": p.p_pearson, "p_spearman": p.p_spearman,
            })
        })
        .collect();
    json!({
        "n": m.n,
        "metricas": m.names,
        "rotulos": m.names.iter().map(|n| metric_label(n)).collect::<Vec<_>>(),
        "pearson": m.pearson,
        "spearman": m.spearman,
        "p_pearson": m.p_pearson,
        "p_spearman": m.p_spearman,
        "fortes": pairs,
    })
}

/// `/api/gantt` from the trace: exact state segments of every object.
pub fn gantt_json(replay: &Replay) -> Value {
    let end = replay.report.end_time;
    let rows: Vec<Value> = analysis::state_gantt(replay, 0.0, end)
        .into_iter()
        .filter(|r| !matches!(r.kind.as_str(), "Source" | "Sink" | "Dispatcher"))
        .take(MAX_GANTT_ROWS)
        .map(|r| {
            let segs: Vec<Value> = r
                .segments
                .iter()
                .map(|(a, b, s)| json!([a, b, s.name()]))
                .collect();
            json!({"rotulo": r.name, "tipo": r.kind, "segmentos": segs})
        })
        .collect();
    json!({
        "linhas": rows,
        "replicacao": replay.replication,
        "fim": end,
        "aquecimento": replay.report.stats_from,
        "fonte": "rastro",
    })
}

fn layer_json(map: &Heatmap, l: &HeatLayer) -> Value {
    let cells: Vec<Value> = map
        .cells(l)
        .into_iter()
        .map(|c| json!([c.col, c.row, c.time, c.passes]))
        .collect();
    json!({"nome": l.name, "distancia": l.distance, "movimento": l.moving, "celulas": cells})
}

/// `/api/mapa_calor`: the heat map of the statistics window of replication 0, by
/// resource and by class, with the plan (objects and connections).
pub fn heatmap_json(replay: &Replay, model: &Model, cfg: &Dashboard) -> Value {
    let cell = cfg.heatmap_cell.unwrap_or(DEFAULT_CELL);
    let (from, to) = (replay.report.stats_from, replay.report.end_time);
    let by = |group| Heatmap::build(replay, Some(model), cell, group, from, to);
    let (res, class) = (by(HeatGroup::Resource), by(HeatGroup::Class));
    let objects: Vec<Value> = replay
        .blocks
        .iter()
        .map(|b| json!({"nome": b.name, "tipo": b.kind, "x": b.position[0], "y": b.position[1]}))
        .collect();
    let links: Vec<Value> = replay
        .connections
        .iter()
        .filter_map(|&(a, b)| {
            let (pa, pb) = (replay.blocks.get(a)?, replay.blocks.get(b)?);
            Some(json!([
                pa.position[0],
                pa.position[1],
                pb.position[0],
                pb.position[1]
            ]))
        })
        .collect();
    json!({
        "celula": res.cell,
        "x0": res.x0, "y0": res.y0, "colunas": res.cols, "linhas": res.rows,
        "de": from, "ate": to,
        "replicacao": replay.replication,
        "total": layer_json(&res, &res.total),
        "recurso": res.layers.iter().map(|l| layer_json(&res, l)).collect::<Vec<_>>(),
        "classe": class.layers.iter().map(|l| layer_json(&class, l)).collect::<Vec<_>>(),
        "objetos": objects,
        "ligacoes": links,
    })
}

fn pairs_json(pairs: &[PairComparison], names: &[String]) -> Vec<Value> {
    pairs
        .iter()
        .map(|p| {
            json!({
                "a": names[p.i], "b": names[p.j],
                "diferenca": p.difference, "lo": p.low(), "hi": p.high(),
                "p": p.p_value, "difere": p.significant,
            })
        })
        .collect()
}

/// Runs the scenarios over `model` (same seeds) and compares every metric.
pub fn run_scenarios_json(
    model: &Model,
    set: &ScenarioSet,
    level: f64,
    keep_going: &(dyn Fn() -> bool + Sync),
) -> Result<Value, String> {
    let results = run_scenarios(model, &set.scenarios, &set.measures, &|_, _| keep_going())
        .map_err(|e| e.to_string())?;
    let result = ExperimentResult {
        name: set.name.clone(),
        measures: set.measures.clone(),
        scenarios: results,
    };
    Ok(scenarios_json(&result, level))
}

/// `/api/cenarios` of a finished experiment.
pub fn scenarios_json(result: &ExperimentResult, level: f64) -> Value {
    let metrics = analysis::default_comparison_metrics(result);
    let rows: Vec<Value> = metrics
        .iter()
        .map(|m| {
            let c = analysis::compare_scenarios(result, m, level);
            let resumos: Vec<Value> = c
                .summaries
                .iter()
                .map(|s| {
                    json!({"valor": s.mean, "meia": s.half_width, "n": s.n,
                           "texto": pt_mean_hw(s.mean, s.half_width)})
                })
                .collect();
            json!({
                "metrica": m,
                "rotulo": metric_label(m),
                "resumos": resumos,
                "anova": c.anova.as_ref().map(|a| json!({
                    "f": a.f, "gl1": a.df_between, "gl2": a.df_within, "p": a.p_value,
                    "sq_entre": a.ss_between, "sq_dentro": a.ss_within,
                })),
                "tukey": c.tukey.as_ref().map(|t| json!({
                    "q": t.q_critical, "pares": pairs_json(&t.pairs, &c.scenarios),
                })),
                "bonferroni": c.bonferroni.as_ref().map(|b| json!({
                    "comparacoes": b.comparisons, "nivel_cada": b.level_each, "n": b.n,
                    "pares": pairs_json(&b.pairs, &c.scenarios),
                })),
            })
        })
        .collect();
    json!({
        "nome": result.name,
        "cenarios": result.scenarios.iter().map(|s| s.name.clone()).collect::<Vec<_>>(),
        "nivel": level,
        "metricas": rows,
    })
}

/// `/api/pert`: Monte Carlo over the activity network of `spec`, with the
/// sensitivity of the total time to each activity (the tornado).
pub fn pert_json(model: &Model, spec: &PertSpec) -> Result<Value, String> {
    use utsim_pert::{DeriveOptions, McOptions, Selection, derive, monte_carlo};
    let selection = if spec.objects.is_empty() {
        Selection::Between {
            from: spec.from.clone().unwrap_or_default(),
            to: spec.to.clone().unwrap_or_default(),
        }
    } else {
        Selection::Objects(spec.objects.clone())
    };
    let net = derive(model, &selection, &DeriveOptions::default()).map_err(|e| e.to_string())?;
    let mc = monte_carlo::run(
        &net,
        &McOptions {
            samples: spec.samples,
            seed: model.run.seed,
            deadline: spec.deadline,
            ..McOptions::default()
        },
    );
    let name = |i: usize| net.activities[i].name.clone();
    let tornado: Vec<Value> = mc
        .sensitivity
        .iter()
        .map(|s| {
            json!({
                "atividade": name(s.activity),
                "correlacao": s.rank_correlation,
                "crucialidade": s.cruciality,
                "criticidade": mc.criticality[s.activity],
            })
        })
        .collect();
    let critical: Vec<Value> = (0..net.len())
        .filter(|&i| !net.activities[i].is_milestone())
        .map(|i| json!({"atividade": name(i), "indice": mc.criticality[i], "meia": mc.criticality_half_width[i]}))
        .collect();
    Ok(json!({
        "selecao": selection.describe(),
        "amostras": mc.samples,
        "total": {"media": mc.total.mean, "meia": mc.total.half_width, "texto": pt_mean_hw(mc.total.mean, mc.total.half_width),
                  "min": mc.min, "max": mc.max},
        "percentis": mc.percentiles,
        "prazo": mc.deadline.map(|d| json!({"prazo": d.deadline, "p": d.probability, "lo": d.low, "hi": d.high})),
        "criticidade": critical,
        "tornado": tornado,
        "avisos": net.warnings,
    }))
}

/// Every extra route, computed after the replications. A part that fails becomes a
/// `{"erro": ...}` route, so the page can say why.
pub fn extras(
    model: &Model,
    cfg: &Dashboard,
    scenarios: Option<&ScenarioSet>,
    keep_going: &(dyn Fn() -> bool + Sync),
    stage: &dyn Fn(&'static str),
) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    let error = |e: String| json!({ "erro": e });
    stage("lendo o rastro da replicação 0 (Gantt e mapa de calor)");
    match model.replay(0) {
        Ok(replay) => {
            out.insert("/api/gantt".to_string(), gantt_json(&replay));
            out.insert(
                "/api/mapa_calor".to_string(),
                heatmap_json(&replay, model, cfg),
            );
        }
        Err(e) => {
            out.insert("/api/mapa_calor".to_string(), error(e.to_string()));
        }
    }
    if let Some(set) = scenarios {
        stage("rodando os cenários do experimento");
        let v = run_scenarios_json(model, set, cfg.level, keep_going).unwrap_or_else(error);
        out.insert("/api/cenarios".to_string(), v);
    }
    if let Some(spec) = &cfg.pert {
        stage("calculando o PERT/CPM (Monte Carlo)");
        out.insert(
            "/api/pert".to_string(),
            pert_json(model, spec).unwrap_or_else(error),
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_labels_read_well() {
        assert_eq!(metric_label("fila.content.mean"), "fila: conteúdo médio");
        assert_eq!(metric_label("m1.utilization"), "m1: ocupação");
        assert_eq!(metric_label("m1.state.blocked"), "m1: fração em blocked");
        assert_eq!(metric_label("x.custom"), "x: custom");
        assert_eq!(metric_label("total"), "total");
    }

    #[test]
    fn pert_of_a_line_has_a_tornado() {
        let model = Model::parse(
            r#"Model(
                name: "linha",
                run: (stop_time: 1000.0, seed: 5),
                objects: [
                    (name: "chegada", kind: Source(interarrival: Const(10.0))),
                    (name: "a", kind: Processor(process: Triangular(min: 1.0, mode: 2.0, max: 6.0))),
                    (name: "b", kind: Processor(process: Uniform(min: 1.0, max: 2.0))),
                    (name: "saida", kind: Sink),
                ],
                connections: [("chegada", "a"), ("a", "b"), ("b", "saida")],
            )"#,
        )
        .unwrap();
        let spec = PertSpec {
            from: Some("a".into()),
            to: Some("b".into()),
            samples: 2000,
            deadline: Some(6.0),
            ..PertSpec::default()
        };
        let v = pert_json(&model, &spec).unwrap();
        let t = v["tornado"].as_array().unwrap();
        assert_eq!(t.len(), 2);
        // The wider activity drives the total time.
        assert_eq!(t[0]["atividade"], "a");
        assert!(t[0]["correlacao"].as_f64().unwrap() > t[1]["correlacao"].as_f64().unwrap());
        assert!((v["total"]["media"].as_f64().unwrap() - (3.0 + 1.5)).abs() < 0.1);
        assert!(v["prazo"]["p"].as_f64().unwrap() > 0.5);
        let bad = PertSpec {
            objects: vec!["nada".into()],
            ..PertSpec::default()
        };
        assert!(pert_json(&model, &bad).is_err());
    }
}
