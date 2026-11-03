//! The data behind the page: one JSON per API route, built from the replications.
//!
//! The same map of routes feeds the HTTP server and the static export (the page reads
//! `/api/...` from the embedded copy when there is no server), so the live page and the
//! frozen one show exactly the same numbers.
//!
//! | route | content |
//! |---|---|
//! | `/api/resumo` | title, progress, summary sentence, bottleneck, Little, warnings, card order |
//! | `/api/kpis` | every [`Kpi`](crate::kpi::Kpi) |
//! | `/api/objetos` | one row per object (table of the engineer view) |
//! | `/api/objeto/{nome}` | the detail of one object: KPIs, states, histograms, series, Gantt |
//! | `/api/series` | work in progress over time, mean with confidence band and P5-P95 |
//! | `/api/distribuicoes` | pooled histograms of the time in the system |
//! | `/api/pareto` | time lost per object and cause |
//! | `/api/gantt` | state timeline of the first replication |
//! | `/api/estados` | colour and Portuguese label of each state |
//! | `/api/config` | the dashboard configuration in use |

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::{Value, json};
use utsim_core::stats::{Histogram, Summary};
use utsim_core::{RunReport, State};
use utsim_model::Model;
use utsim_model::analysis::{self, ObjectStats};
use utsim_scene::style::{state_color, state_label};
use utsim_stats::summary;

use crate::config::Dashboard;
use crate::kpi::{self, Kpi, KpiSet, Spec};
use crate::num::{pt_mean_hw, quantile};

/// Progress of the study.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Progress {
    pub feitas: u32,
    pub total: u32,
    pub concluido: bool,
    pub cancelado: bool,
}

/// Every route of the API, ready to serve or to embed.
#[derive(Clone, Debug)]
pub struct Package {
    pub routes: BTreeMap<String, Value>,
    pub kpis: KpiSet,
    pub stats: Vec<ObjectStats>,
    pub title: String,
    /// WIP series, replication by replication, for the spreadsheet.
    pub wip_series: Vec<[f64; 6]>,
}

const ALL_STATES: [State; 18] = [
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
];

/// States that are time lost (for the Pareto chart).
const LOSS_STATES: [&str; 5] = [
    "blocked",
    "waiting_operator",
    "waiting_transport",
    "down",
    "stopped",
];

fn hex(c: [f32; 3]) -> String {
    let b = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02x}{:02x}{:02x}", b(c[0]), b(c[1]), b(c[2]))
}

/// Colour and Portuguese label of every state, by state name (the 3D scene's palette).
fn states_json() -> Value {
    let mut m = serde_json::Map::new();
    for s in ALL_STATES {
        m.insert(
            s.name().to_string(),
            json!({"cor": hex(state_color(Some(s))), "rotulo": state_label(Some(s))}),
        );
    }
    Value::Object(m)
}

fn summary_json(s: &Summary) -> Value {
    json!({
        "valor": s.mean,
        "meia": s.half_width,
        "n": s.n,
        "texto": pt_mean_hw(s.mean, s.half_width),
    })
}

fn opt_summary_json(s: &Option<Summary>) -> Value {
    s.as_ref().map_or(Value::Null, summary_json)
}

fn histogram_json(h: &Histogram) -> Value {
    json!({"min": h.min, "max": h.max, "largura": h.bin_width, "contagens": h.counts})
}

/// Mean, confidence band and P5-P95 of curves sampled at the same instants.
/// Returns `[t, mean, lo, hi, p5, p95]` rows over the shortest curve.
fn band(curves: &[Vec<[f64; 2]>], level: f64) -> Vec<[f64; 6]> {
    let len = curves.iter().map(Vec::len).min().unwrap_or(0);
    (0..len)
        .map(|k| {
            let xs: Vec<f64> = curves.iter().map(|c| c[k][1]).collect();
            let s = summary(&xs, level);
            let (lo, hi) = if s.half_width.is_finite() {
                (s.mean - s.half_width, s.mean + s.half_width)
            } else {
                (s.mean, s.mean)
            };
            [
                curves[0][k][0],
                s.mean,
                lo,
                hi,
                quantile(&xs, 0.05),
                quantile(&xs, 0.95),
            ]
        })
        .collect()
}

/// State timeline of one block in one replication: `[start, end, state]` segments built
/// from the samples (so they are as fine as `sample_interval`).
fn segments(report: &RunReport, name: &str) -> Vec<Value> {
    let Some(b) = report.block(name) else {
        return Vec::new();
    };
    let mut out: Vec<(f64, f64, &str)> = Vec::new();
    for p in &b.series {
        match out.last_mut() {
            Some(last) if last.2 == p.state => last.1 = p.t,
            Some(last) => {
                last.1 = p.t;
                out.push((p.t, p.t, &p.state));
            }
            None => out.push((p.t, p.t, &p.state)),
        }
    }
    if let Some(last) = out.last_mut() {
        last.1 = last.1.max(report.end_time);
    }
    out.into_iter()
        .filter(|s| s.1 > s.0)
        .map(|(a, b, s)| json!([a, b, s]))
        .collect()
}

fn percent_kpi(cfg: &Dashboard, id: &str, nome: &str, values: Vec<f64>) -> Kpi {
    Kpi::build(plain(cfg, id, nome, "%"), values)
}

fn plain<'a>(cfg: &'a Dashboard, id: &str, nome: &str, unidade: &str) -> Spec<'a> {
    Spec {
        id: id.to_string(),
        nome: nome.to_string(),
        unidade: unidade.to_string(),
        formula: String::new(),
        target: None,
        cfg,
    }
}

impl Package {
    /// Builds every route from the replications finished so far.
    pub fn build(
        model: &Model,
        cfg: &Dashboard,
        reports: &[RunReport],
        progress: Progress,
        config_origin: &str,
    ) -> Package {
        let kpis = kpi::compute(model, reports, cfg);
        let stats = analysis::object_stats_at(model, reports, cfg.level);
        let title = cfg.title.clone().unwrap_or_else(|| model.name.clone());
        let mut routes = BTreeMap::new();

        // Series of the whole system: WIP summed over the blocks that hold work.
        let wip_curves = wip_curves(reports);
        let wip_band = band(&wip_curves, cfg.level);

        let cards = {
            let mut ids: Vec<String> = cfg
                .card_ids()
                .into_iter()
                .filter(|id| kpis.get(id).is_some())
                .collect();
            if ids.is_empty() {
                ids = kpis.kpis.iter().take(6).map(|k| k.id.clone()).collect();
            }
            ids
        };
        let sentence = if reports.is_empty() {
            "Aguardando a primeira replicação.".to_string()
        } else {
            kpi::summary_sentence(&kpis, reports.len(), cfg.level)
        };
        let end = reports.first().map_or(model.run.stop_time, |r| r.end_time);
        routes.insert(
            "/api/resumo".to_string(),
            json!({
                "titulo": title,
                "modelo": model.name,
                "descricao": model.description,
                "progresso": progress,
                "frase": sentence,
                "gargalo": kpis.gargalo,
                "gargalo_turno": kpis.gargalo_turno,
                "little": kpis.little,
                "avisos": kpis.avisos,
                "cartoes": cards,
                "unidade_tempo": cfg.time_unit.label(),
                "por_hora": cfg.time_unit.per_hour(),
                "nivel": cfg.level,
                "limiares": cfg.thresholds,
                "config_origem": config_origin,
                "parada": end,
                "aquecimento": model.run.warmup,
                "semente": model.run.seed,
            }),
        );
        routes.insert("/api/kpis".to_string(), json!({"kpis": kpis.kpis}));
        routes.insert("/api/estados".to_string(), states_json());
        routes.insert(
            "/api/config".to_string(),
            serde_json::to_value(cfg).unwrap_or(Value::Null),
        );
        routes.insert(
            "/api/series".to_string(),
            json!({
                "wip": {"nome": "Estoque em processo (WIP)", "unidade": "itens", "pontos": wip_band},
                "nivel": cfg.level,
            }),
        );

        // Objects.
        let rows: Vec<Value> = stats
            .iter()
            .map(|s| {
                json!({
                    "nome": s.name,
                    "tipo": s.kind,
                    "entradas": summary_json(&s.input),
                    "saidas": summary_json(&s.output),
                    "conteudo": summary_json(&s.content),
                    "permanencia": summary_json(&s.staytime),
                    "ocupacao": opt_summary_json(&s.utilization.map(pct)),
                    "ocupacao_turno": opt_summary_json(&s.utilization_in_shift.map(pct)),
                    "recurso": s.is_resource(),
                    "estados": state_entries(s),
                })
            })
            .collect();
        routes.insert("/api/objetos".to_string(), json!({"objetos": rows}));
        for s in &stats {
            routes.insert(
                format!("/api/objeto/{}", s.name),
                object_detail(s, reports, cfg, model),
            );
        }

        // Distributions.
        let mut hists = Vec::new();
        for s in stats.iter().filter(|s| s.kind == "sink") {
            if let Some(h) = analysis::pooled_histogram(reports, &s.name, "time_in_system") {
                hists.push(json!({
                    "objeto": s.name,
                    "medida": "time_in_system",
                    "rotulo": "Tempo no sistema",
                    "histograma": histogram_json(&h),
                }));
            }
        }
        routes.insert(
            "/api/distribuicoes".to_string(),
            json!({"histogramas": hists}),
        );

        routes.insert("/api/pareto".to_string(), pareto(&stats, reports, cfg));

        let gantt: Vec<Value> = reports
            .first()
            .map(|r| {
                stats
                    .iter()
                    .filter(|s| !matches!(s.kind.as_str(), "source" | "sink" | "dispatcher" | "span"))
                    .take(40)
                    .map(|s| json!({"rotulo": s.name, "tipo": s.kind, "segmentos": segments(r, &s.name)}))
                    .filter(|row| row["segmentos"].as_array().is_some_and(|a| !a.is_empty()))
                    .collect()
            })
            .unwrap_or_default();
        routes.insert(
            "/api/gantt".to_string(),
            json!({"linhas": gantt, "replicacao": 0, "fim": end}),
        );

        Package {
            routes,
            kpis,
            stats,
            title,
            wip_series: wip_band,
        }
    }
}

/// Fraction of time in each state of an object, with the scene's colour and label.
fn state_entries(s: &ObjectStats) -> Vec<Value> {
    s.states
        .iter()
        .filter(|(_, sum)| sum.mean > 0.0)
        .map(|(st, sum)| {
            let known = ALL_STATES.iter().find(|x| x.name() == st);
            json!({
                "estado": st,
                "rotulo": known.map_or(st.as_str(), |k| state_label(Some(*k))),
                "cor": hex(state_color(known.copied())),
                "fracao": sum.mean,
                "meia": sum.half_width,
                "texto": format!("{} %", pt_mean_hw(100.0 * sum.mean, 100.0 * sum.half_width)),
            })
        })
        .collect()
}

fn pct(s: Summary) -> Summary {
    Summary {
        n: s.n,
        mean: 100.0 * s.mean,
        std_dev: 100.0 * s.std_dev,
        half_width: 100.0 * s.half_width,
    }
}

/// WIP of the whole system over time, one curve per replication.
fn wip_curves(reports: &[RunReport]) -> Vec<Vec<[f64; 2]>> {
    reports
        .iter()
        .map(|r| {
            let blocks: Vec<_> = r
                .blocks
                .iter()
                .filter(|b| {
                    !b.series.is_empty()
                        && matches!(
                            b.kind.as_str(),
                            "queue" | "processor" | "conveyor" | "rack" | "combiner" | "separator"
                        )
                })
                .collect();
            let len = blocks.iter().map(|b| b.series.len()).min().unwrap_or(0);
            (0..len)
                .map(|k| {
                    let t = blocks[0].series[k].t;
                    [t, blocks.iter().map(|b| b.series[k].content as f64).sum()]
                })
                .collect()
        })
        .filter(|c: &Vec<[f64; 2]>| !c.is_empty())
        .collect()
}

/// Time lost per object and cause, longest first.
fn pareto(stats: &[ObjectStats], reports: &[RunReport], cfg: &Dashboard) -> Value {
    let t = reports.first().map_or(0.0, |r| r.end_time - r.stats_from);
    let hours = !matches!(cfg.time_unit, crate::config::TimeUnit::Unit);
    let scale = if hours {
        1.0 / cfg.time_unit.per_hour()
    } else {
        1.0
    };
    let mut items: Vec<(f64, Value)> = Vec::new();
    for s in stats
        .iter()
        .filter(|s| !matches!(s.kind.as_str(), "source" | "sink"))
    {
        for (state, sum) in &s.states {
            if LOSS_STATES.contains(&state.as_str()) && sum.mean > 0.0 {
                let amount = sum.mean * t * scale;
                let label = ALL_STATES
                    .iter()
                    .find(|st| st.name() == state)
                    .map_or(state.as_str(), |st| state_label(Some(*st)));
                items.push((
                    amount,
                    json!({
                        "objeto": s.name, "estado": state, "rotulo": label,
                        "tempo": amount, "fracao": sum.mean,
                    }),
                ));
            }
        }
    }
    items.sort_by(|a, b| b.0.total_cmp(&a.0));
    items.truncate(12);
    json!({
        "unidade": if hours { "h" } else { cfg.time_unit.label() },
        "itens": items.into_iter().map(|i| i.1).collect::<Vec<_>>(),
    })
}

fn object_detail(s: &ObjectStats, reports: &[RunReport], cfg: &Dashboard, model: &Model) -> Value {
    let name = &s.name;
    let blocks: Vec<&utsim_core::BlockReport> =
        reports.iter().filter_map(|r| r.block(name)).collect();
    let tu = cfg.time_unit;
    let t_of = |r: &RunReport| (r.end_time - r.stats_from).max(1e-12);
    let per_rep = |f: &dyn Fn(&utsim_core::BlockReport, f64) -> f64| -> Vec<f64> {
        reports
            .iter()
            .filter_map(|r| r.block(name).map(|b| f(b, t_of(r))))
            .collect()
    };
    let mut kpis: Vec<Kpi> = Vec::new();
    let flow = if s.kind == "sink" {
        "Chegadas"
    } else {
        "Saídas"
    };
    kpis.push(Kpi::build(
        plain(
            cfg,
            "vazao",
            &format!("{flow} por hora"),
            tu.throughput_label(),
        ),
        per_rep(&|b, t| {
            let n = if b.kind == "sink" { b.input } else { b.output };
            n as f64 / t * tu.per_hour()
        }),
    ));
    if blocks.iter().any(|b| b.utilization().is_some()) {
        kpis.push(percent_kpi(
            cfg,
            "ocupacao",
            "Ocupação",
            blocks
                .iter()
                .map(|b| b.utilization().map_or(f64::NAN, |u| 100.0 * u.work))
                .collect(),
        ));
        kpis.push(percent_kpi(
            cfg,
            "ocupacao_turno",
            "Ocupação no turno",
            blocks
                .iter()
                .map(|b| b.utilization().map_or(f64::NAN, |u| 100.0 * u.in_shift))
                .collect(),
        ));
        let state_pct = |st: &str| -> Vec<f64> {
            blocks
                .iter()
                .map(|b| 100.0 * b.states.get(st).copied().unwrap_or(0.0))
                .collect()
        };
        kpis.push(percent_kpi(
            cfg,
            "bloqueado",
            "Bloqueado",
            state_pct("blocked"),
        ));
        kpis.push(percent_kpi(
            cfg,
            "ocioso",
            "Ocioso / aguardando peça",
            state_pct("idle"),
        ));
        if blocks.iter().any(|b| b.states.contains_key("down")) {
            kpis.push(percent_kpi(cfg, "falha", "Em falha", state_pct("down")));
        }
    }
    kpis.push(Kpi::build(
        plain(cfg, "conteudo", "Conteúdo médio", "itens"),
        blocks.iter().map(|b| b.content_mean).collect(),
    ));
    kpis.push(Kpi::build(
        plain(cfg, "permanencia", "Permanência média", tu.label()),
        blocks.iter().map(|b| b.staytime.mean).collect(),
    ));

    let states = state_entries(s);

    let mut hists = Vec::new();
    for tally in analysis::histogram_tallies(reports, name) {
        if let Some(h) = analysis::pooled_histogram(reports, name, &tally) {
            let label = if tally == "staytime" {
                "Permanência".to_string()
            } else {
                tally.clone()
            };
            hists.push(json!({"medida": tally, "rotulo": label, "histograma": histogram_json(&h)}));
        }
    }

    let content = analysis::content_series(reports, name);
    let curves = content.replications;
    let series = band(&curves, cfg.level);

    let extras: Vec<Value> = s
        .extra
        .iter()
        .map(|(k, v)| json!({"nome": format!("{k} (média)"), "resumo": summary_json(v)}))
        .chain(
            s.custom
                .iter()
                .map(|(k, v)| json!({"nome": k, "resumo": summary_json(v)})),
        )
        .collect();

    json!({
        "nome": name,
        "tipo": s.kind,
        "kpis": kpis,
        "estados": states,
        "histogramas": hists,
        "serie": {"nome": format!("Conteúdo de {name}"), "unidade": "itens", "pontos": series},
        "gantt": reports.first().map(|r| segments(r, name)).unwrap_or_default(),
        "extras": extras,
        "fim": reports.first().map_or(model.run.stop_time, |r| r.end_time),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_has_the_mean_between_the_edges() {
        let curves = vec![
            vec![[0.0, 1.0], [1.0, 2.0]],
            vec![[0.0, 3.0], [1.0, 4.0]],
            vec![[0.0, 5.0], [1.0, 6.0]],
        ];
        let b = band(&curves, 0.95);
        assert_eq!(b.len(), 2);
        assert_eq!(b[0][0], 0.0);
        assert!((b[0][1] - 3.0).abs() < 1e-12);
        assert!(b[0][2] < b[0][1] && b[0][1] < b[0][3]);
        assert!(b[0][4] >= 1.0 && b[0][5] <= 5.0);
        assert!(band(&[], 0.95).is_empty());
    }

    #[test]
    fn state_palette_has_every_state() {
        let v = states_json();
        assert_eq!(v.as_object().unwrap().len(), ALL_STATES.len());
        assert_eq!(v["processing"]["cor"], "#33b84d");
        assert_eq!(v["blocked"]["rotulo"], "bloqueado");
    }
}
