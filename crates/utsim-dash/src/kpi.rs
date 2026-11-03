//! KPIs of a set of replications, with their intervals and traffic lights.
//!
//! Every KPI is computed once per replication (`T` is the observed time after the
//! warm-up, `end_time - stats_from`) and then summarised across replications:
//!
//! | id | formula (per replication) | unit |
//! |---|---|---|
//! | `vazao` | `Σ chegadas nas Sinks / T`, times the number of time units per hour | itens/h |
//! | `lead_time` | mean of `time_in_system` over the sinks, weighted by the items | time |
//! | `lead_time_p90` | 90th percentile of `time_in_system` (from the histograms) | time |
//! | `wip` | `Σ` time-weighted content of queues, processors, conveyors, racks, combiners and separators | itens |
//! | `utilizacao_gargalo` | work fraction of the busiest resource | % |
//! | `utilizacao_gargalo_turno` | the same over the on-shift time | % |
//! | `utilizacao_media` | mean work fraction of all resources | % |
//! | `oee:<obj>` | availability × performance × quality (needs `oee` in the file) | % |
//! | `otd` | `no prazo / total`, from the sinks' `prazo.no_prazo` and `prazo.total` | % |
//! | `perdas` | `perdidos / (perdidos + entrados)` of the sources that can lose items | % |
//! | `nivel_servico` | items with lead time `<= lead_time_limit` over all items | % |
//!
//! Across replications each KPI has the mean, the confidence interval of the mean
//! (Student-t, at the configured level), the interval of one replication
//! (`mean ± t·s·√(1+1/n)`, "faixa esperada numa rodada") and the empirical percentiles
//! P5, P25, P50, P75, P95. A KPI that depends on data the model does not define simply
//! does not appear: nothing is invented.
//!
//! **Traffic light.** With a goal, the probability of meeting it is the fraction of
//! replications that meet it (`k/n`) and carries a Wilson interval. Green when the
//! probability is at least `thresholds.green` (90%), yellow at least `thresholds.yellow`
//! (50%), red below. When the confidence interval of the mean contains the goal the KPI
//! is flagged `indefinido` and the number of extra replications that would settle it is
//! given.

use serde::Serialize;
use utsim_core::stats::Summary;
use utsim_core::{BlockReport, RunReport};
use utsim_model::Model;
use utsim_model::analysis::{self, ObjectStats};
use utsim_stats::little::{self, DEFAULT_TOLERANCE, LittleCheck};
use utsim_stats::precision::{self, Target as PrecisionTarget};
use utsim_stats::{summary, t_quantile};

use crate::config::{Dashboard, Target};
use crate::num::{decimals, histogram_cdf, histogram_quantile, pt, pt_mean_hw, quantile, wilson};

/// Traffic light of a KPI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Situation {
    Verde,
    Amarelo,
    Vermelho,
    /// The KPI has no goal.
    SemMeta,
    /// Fewer than two replications: no probability can be told.
    Indefinido,
}

/// The goal of a KPI as the page shows it.
#[derive(Clone, Debug, Serialize)]
pub struct Meta {
    /// `"min"` (at least) or `"max"` (at most).
    pub sentido: &'static str,
    pub valor: f64,
}

/// Probability of meeting the goal (fraction of replications), with its Wilson interval.
#[derive(Clone, Debug, Serialize)]
pub struct Probabilidade {
    pub p: f64,
    pub lo: f64,
    pub hi: f64,
    /// Replications that met the goal.
    pub k: usize,
    pub n: usize,
}

/// A factor of a composite KPI (availability, performance and quality of the OEE).
#[derive(Clone, Debug, Serialize)]
pub struct Fator {
    pub nome: String,
    pub valor: f64,
    pub texto: String,
}

/// One KPI across replications.
#[derive(Clone, Debug, Serialize)]
pub struct Kpi {
    pub id: String,
    pub nome: String,
    pub unidade: String,
    pub formula: String,
    /// Replications with a value.
    pub n: usize,
    pub valor: f64,
    /// Confidence interval of the mean (`null` with fewer than two values).
    pub ic: Option<[f64; 2]>,
    /// Interval expected in one replication.
    pub ip: Option<[f64; 2]>,
    pub p5: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p95: f64,
    /// Ready-to-show text: `118 ± 3`.
    pub texto: String,
    /// Ready-to-show range of one replication: `104 a 131`.
    pub faixa: String,
    pub meta: Option<Meta>,
    pub prob: Option<Probabilidade>,
    pub situacao: Situation,
    /// The interval of the mean contains the goal: the light is not settled.
    pub indefinido: bool,
    /// Extra replications that would settle an `indefinido` KPI.
    pub replicacoes_faltam: Option<usize>,
    /// Replications for a half-width of 2% of the mean.
    pub replicacoes_2pct: Option<usize>,
    /// Value in each replication, in replication order.
    pub valores: Vec<f64>,
    pub fatores: Vec<Fator>,
    pub nota: Option<String>,
}

/// All KPIs of a study plus the system-level diagnostics.
#[derive(Clone, Debug, Serialize)]
pub struct KpiSet {
    pub kpis: Vec<Kpi>,
    /// Name of the bottleneck (highest mean work fraction), if any resource worked.
    pub gargalo: Option<String>,
    pub gargalo_turno: Option<String>,
    pub little: Option<LittleOut>,
    pub avisos: Vec<String>,
}

/// Little's law at the system level: `WIP = vazão × lead time`.
#[derive(Clone, Debug, Serialize)]
pub struct LittleOut {
    pub l: f64,
    pub lambda_w: f64,
    pub relativo: f64,
    pub vale: bool,
}

impl KpiSet {
    pub fn get(&self, id: &str) -> Option<&Kpi> {
        self.kpis.iter().find(|k| k.id == id)
    }
}

/// Everything about a KPI except the data: what [`Kpi::build`] needs to name and judge it.
pub struct Spec<'a> {
    pub id: String,
    pub nome: String,
    pub unidade: String,
    pub formula: String,
    pub target: Option<Target>,
    pub cfg: &'a Dashboard,
}

impl Kpi {
    /// Summarises one value per replication (NaN values are dropped).
    pub fn build(spec: Spec, values: Vec<f64>) -> Kpi {
        let cfg = spec.cfg;
        let xs: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();
        let n = xs.len();
        let s: Summary = summary(&xs, cfg.level);
        let ic = (n >= 2 && s.half_width.is_finite())
            .then_some([s.mean - s.half_width, s.mean + s.half_width]);
        let ip = (n >= 2).then(|| {
            let half = t_quantile(cfg.level, n - 1) * s.std_dev * (1.0 + 1.0 / n as f64).sqrt();
            [s.mean - half, s.mean + half]
        });
        let (p5, p25, p50, p75, p95) = (
            quantile(&xs, 0.05),
            quantile(&xs, 0.25),
            quantile(&xs, 0.5),
            quantile(&xs, 0.75),
            quantile(&xs, 0.95),
        );
        let range_d = decimals(s.mean, (p95 - p5) / 4.0);
        let faixa = if n >= 2 {
            format!("{} a {}", pt(p5, range_d), pt(p95, range_d))
        } else {
            String::new()
        };

        let mut situacao = Situation::SemMeta;
        let mut prob = None;
        let mut indefinido = false;
        let mut faltam = None;
        let mut meta = None;
        if let Some(t) = spec.target {
            meta = Some(Meta {
                sentido: match t {
                    Target::AtLeast(_) => "min",
                    Target::AtMost(_) => "max",
                },
                valor: t.value(),
            });
            if n < 2 {
                situacao = Situation::Indefinido;
            } else {
                let k = xs.iter().filter(|&&x| t.met(x)).count();
                let (lo, hi) = wilson(k, n, cfg.level);
                let p = k as f64 / n as f64;
                prob = Some(Probabilidade { p, lo, hi, k, n });
                situacao = if p >= cfg.thresholds.green {
                    Situation::Verde
                } else if p >= cfg.thresholds.yellow {
                    Situation::Amarelo
                } else {
                    Situation::Vermelho
                };
                if let Some([a, b]) = ic {
                    indefinido = a <= t.value() && t.value() <= b;
                    if indefinido {
                        let gap = (s.mean - t.value()).abs();
                        faltam = if gap > 0.0 {
                            precision::replications_needed(s.std_dev, gap, cfg.level)
                                .map(|need| need.saturating_sub(n).max(1))
                        } else {
                            None
                        };
                    }
                }
            }
        }
        let replicacoes_2pct = if n >= 2 && s.mean != 0.0 {
            precision::precision(&xs, PrecisionTarget::Relative(0.02), cfg.level).needed
        } else {
            None
        };
        Kpi {
            id: spec.id,
            nome: spec.nome,
            unidade: spec.unidade,
            formula: spec.formula,
            n,
            valor: s.mean,
            ic,
            ip,
            p5,
            p25,
            p50,
            p75,
            p95,
            texto: if n == 0 {
                "-".to_string()
            } else {
                pt_mean_hw(s.mean, s.half_width)
            },
            faixa,
            meta,
            prob,
            situacao,
            indefinido,
            replicacoes_faltam: faltam,
            replicacoes_2pct,
            valores: values,
            fatores: Vec::new(),
            nota: None,
        }
    }
}

/// Observed time of a replication (after the warm-up).
fn observed(r: &RunReport) -> f64 {
    (r.end_time - r.stats_from).max(1e-12)
}

fn sinks(r: &RunReport) -> impl Iterator<Item = &BlockReport> {
    r.blocks.iter().filter(|b| b.kind == "sink")
}

/// Kinds of block that hold work in progress.
fn holds_wip(kind: &str) -> bool {
    matches!(
        kind,
        "queue" | "processor" | "conveyor" | "rack" | "combiner" | "separator"
    )
}

fn fraction_of(block: &BlockReport, state: &str) -> f64 {
    block.states.get(state).copied().unwrap_or(0.0)
}

/// Throughput of a replication, in items per model time unit.
fn throughput(r: &RunReport) -> Option<f64> {
    let mut any = false;
    let total: u64 = sinks(r).inspect(|_| any = true).map(|b| b.input).sum();
    any.then(|| total as f64 / observed(r))
}

/// Mean `time_in_system` over the sinks, weighted by the items counted.
fn lead_time(r: &RunReport) -> Option<f64> {
    let (mut sum, mut count) = (0.0, 0.0);
    for b in sinks(r) {
        if let Some(t) = b.extra.get("time_in_system") {
            sum += t.mean * t.count as f64;
            count += t.count as f64;
        }
    }
    (count > 0.0).then(|| sum / count)
}

/// Quantile `q` of `time_in_system` in one replication, from the sinks' histograms.
fn lead_time_quantile(r: &RunReport, q: f64) -> Option<f64> {
    let hists: Vec<_> = sinks(r)
        .filter_map(|b| b.extra.get("time_in_system")?.histogram.as_ref())
        .collect();
    histogram_quantile(&hists, q)
}

fn wip(r: &RunReport) -> f64 {
    r.blocks
        .iter()
        .filter(|b| holds_wip(&b.kind))
        .map(|b| b.content_mean)
        .sum()
}

fn spec<'a>(cfg: &'a Dashboard, id: &str, nome: &str, unidade: &str, formula: &str) -> Spec<'a> {
    Spec {
        id: id.to_string(),
        nome: nome.to_string(),
        unidade: unidade.to_string(),
        formula: formula.to_string(),
        target: cfg.targets.get(id).copied(),
        cfg,
    }
}

/// Computes every KPI the model supports.
pub fn compute(model: &Model, reports: &[RunReport], cfg: &Dashboard) -> KpiSet {
    let mut kpis: Vec<Kpi> = Vec::new();
    let mut avisos = Vec::new();
    let tu = cfg.time_unit;
    let stats: Vec<ObjectStats> = analysis::object_stats_at(model, reports, cfg.level);

    // Throughput.
    let per_rep: Vec<Option<f64>> = reports.iter().map(throughput).collect();
    let have_sink = per_rep.iter().any(Option::is_some);
    if have_sink {
        let v: Vec<f64> = per_rep
            .iter()
            .map(|x| x.map_or(f64::NAN, |t| t * tu.per_hour()))
            .collect();
        kpis.push(Kpi::build(
            spec(
                cfg,
                "vazao",
                "Vazão",
                tu.throughput_label(),
                "itens que chegam às Sinks no período observado / tempo observado",
            ),
            v,
        ));
    }

    // Lead time.
    let lt: Vec<f64> = reports
        .iter()
        .map(|r| lead_time(r).unwrap_or(f64::NAN))
        .collect();
    if lt.iter().any(|x| x.is_finite()) {
        kpis.push(Kpi::build(
            spec(
                cfg,
                "lead_time",
                "Lead time médio",
                tu.label(),
                "média do tempo no sistema (time_in_system) das Sinks, ponderada pelos itens",
            ),
            lt.clone(),
        ));
    }
    let p90: Vec<f64> = reports
        .iter()
        .map(|r| lead_time_quantile(r, 0.9).unwrap_or(f64::NAN))
        .collect();
    if p90.iter().any(|x| x.is_finite()) {
        kpis.push(Kpi::build(
            spec(
                cfg,
                "lead_time_p90",
                "Lead time P90",
                tu.label(),
                "percentil 90 do tempo no sistema, a partir dos histogramas das Sinks",
            ),
            p90,
        ));
    }
    if let Some(limit) = cfg.lead_time_limit {
        let v: Vec<f64> = reports
            .iter()
            .map(|r| {
                let hists: Vec<_> = sinks(r)
                    .filter_map(|b| b.extra.get("time_in_system")?.histogram.as_ref())
                    .collect();
                histogram_cdf(&hists, limit).map_or(f64::NAN, |f| 100.0 * f)
            })
            .collect();
        if v.iter().any(|x| x.is_finite()) {
            kpis.push(Kpi::build(
                spec(
                    cfg,
                    "nivel_servico",
                    "Nível de serviço",
                    "%",
                    "fração dos itens com tempo no sistema dentro do limite configurado (lead_time_limit)",
                ),
                v,
            ));
        }
    }

    // Work in progress.
    let wips: Vec<f64> = reports.iter().map(wip).collect();
    if reports
        .first()
        .is_some_and(|r| r.blocks.iter().any(|b| holds_wip(&b.kind)))
    {
        kpis.push(Kpi::build(
            spec(
                cfg,
                "wip",
                "Estoque em processo (WIP)",
                "itens",
                "soma do conteúdo médio, ponderado no tempo, de filas, estações, esteiras, estantes, montagens e separadores",
            ),
            wips.clone(),
        ));
    }

    // Utilisation.
    let hot = analysis::bottleneck(&stats).map(|i| stats[i].name.clone());
    let hot_shift = analysis::bottleneck_in_shift(&stats).map(|i| stats[i].name.clone());
    let util_of = |r: &RunReport, name: &str, shift: bool| -> f64 {
        r.block(name)
            .and_then(BlockReport::utilization)
            .map_or(f64::NAN, |u| {
                100.0 * if shift { u.in_shift } else { u.work }
            })
    };
    if let Some(name) = &hot {
        let v = reports.iter().map(|r| util_of(r, name, false)).collect();
        let mut k = Kpi::build(
            spec(
                cfg,
                "utilizacao_gargalo",
                "Ocupação do gargalo",
                "%",
                "fração do tempo em que o recurso mais ocupado trabalha (processo e preparação)",
            ),
            v,
        );
        k.nota = Some(format!("Gargalo: {name}"));
        kpis.push(k);
    }
    if let Some(name) = &hot_shift {
        let v = reports.iter().map(|r| util_of(r, name, true)).collect();
        let mut k = Kpi::build(
            spec(
                cfg,
                "utilizacao_gargalo_turno",
                "Ocupação do gargalo no turno",
                "%",
                "trabalho dividido pelo tempo em turno (o tempo fora do turno sai da conta)",
            ),
            v,
        );
        k.nota = Some(format!("Gargalo no turno: {name}"));
        kpis.push(k);
    }
    let resources: Vec<&ObjectStats> = stats.iter().filter(|s| s.is_resource()).collect();
    if !resources.is_empty() {
        let v: Vec<f64> = reports
            .iter()
            .map(|r| {
                let us: Vec<f64> = resources
                    .iter()
                    .map(|s| util_of(r, &s.name, false))
                    .filter(|x| x.is_finite())
                    .collect();
                us.iter().sum::<f64>() / us.len().max(1) as f64
            })
            .collect();
        kpis.push(Kpi::build(
            spec(
                cfg,
                "utilizacao_media",
                "Ocupação média dos recursos",
                "%",
                "média, entre os recursos, da fração do tempo trabalhando",
            ),
            v,
        ));
    }

    // OEE of the stations listed in the file.
    for o in &cfg.oee {
        match oee(reports, o) {
            Some((k_oee, factors)) => {
                let mut k = Kpi::build(
                    spec(
                        cfg,
                        &format!("oee:{}", o.object),
                        &format!("OEE de {}", o.object),
                        "%",
                        "disponibilidade × desempenho × qualidade; disponibilidade = 1 - falhas / tempo planejado; desempenho = ciclo ideal × saídas / tempo operando; qualidade = boas / total nas Sinks",
                    ),
                    k_oee,
                );
                for (nome, vals) in factors {
                    let s = summary(&vals, cfg.level);
                    k.fatores.push(Fator {
                        nome: nome.to_string(),
                        valor: s.mean,
                        texto: pt_mean_hw(s.mean, s.half_width),
                    });
                }
                if o.scrap_sinks.is_empty() {
                    k.nota =
                        Some("Sem Sinks de refugo configuradas: qualidade assumida em 100%".into());
                }
                kpis.push(k);
            }
            None => avisos.push(format!(
                "OEE de {}: objeto não encontrado ou sem tempo de operação",
                o.object
            )),
        }
    }
    // OEE measured by the kernel for objects with maintenance (`downtime` with crew,
    // wear, preventive...), unless the file already configures that object.
    let measured: Vec<String> = reports
        .first()
        .map(|r| {
            r.blocks
                .iter()
                .filter(|b| b.custom.contains_key("oee"))
                .filter(|b| !cfg.oee.iter().any(|o| o.object == b.name))
                .map(|b| b.name.clone())
                .collect()
        })
        .unwrap_or_default();
    for name in measured {
        let pct = |key: &str| -> Vec<f64> {
            reports
                .iter()
                .map(|r| {
                    r.block(&name)
                        .and_then(|b| b.custom.get(key))
                        .map_or(f64::NAN, |v| 100.0 * v)
                })
                .collect()
        };
        let mut k = Kpi::build(
            spec(
                cfg,
                &format!("oee:{name}"),
                &format!("OEE de {name}"),
                "%",
                "medido na simulação (manutenção): disponibilidade = (planejado - corretivas) / planejado, com o planejado sem turnos nem preventivas; desempenho = ciclo ideal × peças / tempo funcionando; qualidade = boas / peças",
            ),
            pct("oee"),
        );
        for (nome, key) in [
            ("Disponibilidade", "oee.availability"),
            ("Desempenho", "oee.performance"),
            ("Qualidade", "oee.quality"),
        ] {
            let s = summary(&pct(key), cfg.level);
            k.fatores.push(Fator {
                nome: nome.to_string(),
                valor: s.mean,
                texto: pt_mean_hw(s.mean, s.half_width),
            });
        }
        kpis.push(k);
    }

    // Deadline attainment, by convention of the `prazo` label counted at the sinks.
    let otd: Vec<f64> = reports
        .iter()
        .map(|r| {
            let (mut ok, mut total) = (0.0, 0.0);
            for b in sinks(r) {
                ok += b.custom.get("prazo.no_prazo").copied().unwrap_or(0.0);
                total += b.custom.get("prazo.total").copied().unwrap_or(0.0);
            }
            if total > 0.0 {
                100.0 * ok / total
            } else {
                f64::NAN
            }
        })
        .collect();
    if otd.iter().any(|x| x.is_finite()) {
        kpis.push(Kpi::build(
            spec(
                cfg,
                "otd",
                "Atendimento de prazo",
                "%",
                "itens que saem até o instante do rótulo `prazo` / itens com prazo",
            ),
            otd,
        ));
    }

    // Losses.
    let loss: Vec<f64> = reports
        .iter()
        .map(|r| {
            r.metrics()
                .get(utsim_core::report::MODEL_LOSS_FRACTION)
                .map_or(f64::NAN, |f| 100.0 * f)
        })
        .collect();
    if loss.iter().any(|x| x.is_finite()) {
        kpis.push(Kpi::build(
            spec(
                cfg,
                "perdas",
                "Perdas",
                "%",
                "itens perdidos / (perdidos + entrados) nas fontes com on_blocked: Drop",
            ),
            loss,
        ));
    }

    // Little's law for the system: WIP against throughput × lead time.
    let little = little_out(reports, &wips, cfg.level);

    // Targets that match no KPI are a typo worth reporting.
    for id in cfg.targets.keys() {
        if !kpis.iter().any(|k| &k.id == id) {
            avisos.push(format!("meta para \"{id}\": o modelo não tem esse KPI"));
        }
    }
    for id in &cfg.cards {
        if !kpis.iter().any(|k| &k.id == id) {
            avisos.push(format!("cartão \"{id}\": o modelo não tem esse KPI"));
        }
    }
    KpiSet {
        kpis,
        gargalo: hot,
        gargalo_turno: hot_shift,
        little,
        avisos,
    }
}

type Factors = Vec<(&'static str, Vec<f64>)>;

/// OEE of one station per replication, plus its three factors.
fn oee(reports: &[RunReport], o: &crate::config::OeeSpec) -> Option<(Vec<f64>, Factors)> {
    reports.first()?.block(&o.object)?;
    let (mut oee, mut a, mut p, mut q) = (vec![], vec![], vec![], vec![]);
    for r in reports {
        let b = r.block(&o.object)?;
        let t = observed(r);
        let off = fraction_of(b, "off_shift");
        let planned = (1.0 - off).max(1e-9);
        let avail = (1.0 - fraction_of(b, "down") / planned).clamp(0.0, 1.0);
        // Busy server-time: the processor's own count when it has several servers.
        let busy = b
            .custom
            .get("servers.busy")
            .copied()
            .unwrap_or_else(|| fraction_of(b, "processing"));
        let perf = if busy > 1e-9 {
            (o.ideal_cycle * b.output as f64 / (busy * t)).min(1.0)
        } else {
            f64::NAN
        };
        let (mut scrap, mut total) = (0.0, 0.0);
        for s in sinks(r) {
            total += s.input as f64;
            if o.scrap_sinks.contains(&s.name) {
                scrap += s.input as f64;
            }
        }
        let qual = if o.scrap_sinks.is_empty() || total <= 0.0 {
            1.0
        } else {
            1.0 - scrap / total
        };
        a.push(100.0 * avail);
        p.push(100.0 * perf);
        q.push(100.0 * qual);
        oee.push(100.0 * avail * perf * qual);
    }
    if oee.iter().all(|x| x.is_nan()) {
        return None;
    }
    Some((
        oee,
        vec![("Disponibilidade", a), ("Desempenho", p), ("Qualidade", q)],
    ))
}

fn little_out(reports: &[RunReport], wips: &[f64], level: f64) -> Option<LittleOut> {
    let lambda: Vec<Option<f64>> = reports.iter().map(throughput).collect();
    let w: Vec<Option<f64>> = reports.iter().map(lead_time).collect();
    if reports.len() < 2 || lambda.iter().any(Option::is_none) || w.iter().any(Option::is_none) {
        return None;
    }
    let lambda: Vec<f64> = lambda.into_iter().flatten().collect();
    let w: Vec<f64> = w.into_iter().flatten().collect();
    let c: LittleCheck = little::check(wips, &lambda, &w, level, DEFAULT_TOLERANCE);
    Some(LittleOut {
        l: c.l.mean,
        lambda_w: c.lambda_w.mean,
        relativo: c.relative,
        vale: c.holds,
    })
}

/// The one-paragraph summary of the manager view, built from the numbers.
pub fn summary_sentence(set: &KpiSet, reps: usize, level: f64) -> String {
    let mut s = format!("Com {reps} replicações");
    if let Some(k) = set.get("vazao") {
        s += &format!(", a vazão esperada é {} {}", k.texto, k.unidade);
        if k.n >= 2 {
            s += &format!(
                "; em 9 de cada 10 rodadas ela fica entre {} e {}",
                pt(k.p5, decimals(k.valor, (k.p95 - k.p5) / 4.0)),
                pt(k.p95, decimals(k.valor, (k.p95 - k.p5) / 4.0))
            );
        }
        s += ".";
    } else {
        s += ".";
    }
    for k in set.kpis.iter().filter(|k| k.meta.is_some()) {
        let meta = k.meta.as_ref().map_or(0.0, |m| m.valor);
        match &k.prob {
            Some(p) => {
                s += &format!(
                    " A meta de {} para {} ({}) é cumprida em {}% das rodadas.",
                    pt(meta, decimals(meta, f64::NAN).min(2)),
                    k.nome.to_lowercase(),
                    if k.meta.as_ref().is_some_and(|m| m.sentido == "min") {
                        "no mínimo"
                    } else {
                        "no máximo"
                    },
                    pt(100.0 * p.p, 0)
                );
            }
            None => s += &format!(" Meta de {}: rode mais replicações.", k.nome.to_lowercase()),
        }
    }
    if let Some(g) = &set.gargalo
        && let Some(u) = set.get("utilizacao_gargalo")
    {
        s += &format!(" Gargalo: {g}, ocupado {}% do tempo.", pt(u.valor, 0));
    }
    let _ = level;
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> Dashboard {
        Dashboard::default()
    }

    fn build(values: Vec<f64>, target: Option<Target>) -> Kpi {
        let c = cfg();
        Kpi::build(
            Spec {
                id: "x".into(),
                nome: "X".into(),
                unidade: "u".into(),
                formula: String::new(),
                target,
                cfg: &c,
            },
            values,
        )
    }

    #[test]
    fn oee_measured_by_the_kernel_becomes_a_kpi() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples/manutencao.ron");
        let mut m = Model::load(path).unwrap();
        m.run.replications = 2;
        m.run.stop_time = 2000.0;
        let reports = m.run_replications().unwrap();
        let set = compute(&m, &reports, &cfg());
        let k = set
            .kpis
            .iter()
            .find(|k| k.id == "oee:torno")
            .expect("KPI do OEE");
        assert_eq!(k.fatores.len(), 3);
        let first = reports[0].block("torno").unwrap().custom["oee"];
        assert!(k.valor > 0.0 && k.valor <= 100.0 && first > 0.0);
    }

    #[test]
    fn statistics_of_known_values() {
        // mean 3, s = sqrt(2.5), n = 5: half-width = 2.776 * 1.5811 / 2.2361.
        let k = build(vec![1.0, 2.0, 3.0, 4.0, 5.0], None);
        assert_eq!(k.n, 5);
        assert!((k.valor - 3.0).abs() < 1e-12);
        let hw = 2.776 * 2.5f64.sqrt() / 5f64.sqrt();
        let ic = k.ic.unwrap();
        assert!((ic[1] - 3.0 - hw).abs() < 1e-9 && (3.0 - ic[0] - hw).abs() < 1e-9);
        // The interval of one replication is wider than the interval of the mean.
        let ip = k.ip.unwrap();
        assert!(ip[1] - ip[0] > ic[1] - ic[0]);
        assert_eq!((k.p5, k.p50, k.p95), (1.2, 3.0, 4.8));
        assert_eq!(k.situacao, Situation::SemMeta);
        assert!(k.meta.is_none() && k.prob.is_none());
    }

    #[test]
    fn traffic_light_follows_the_probability_not_the_mean() {
        // 10 replications, 9 meet "at least 100": probability 90% -> green.
        let mut v = vec![110.0; 9];
        v.push(60.0);
        let k = build(v, Some(Target::AtLeast(100.0)));
        let p = k.prob.as_ref().unwrap();
        assert_eq!((p.k, p.n), (9, 10));
        assert!((p.p - 0.9).abs() < 1e-12);
        assert_eq!(k.situacao, Situation::Verde);
        // Only the number of replications that meet the goal changes below.
        for (ok, light) in [
            (8, Situation::Amarelo),
            (5, Situation::Amarelo),
            (4, Situation::Vermelho),
        ] {
            let mut v = vec![150.0; ok];
            v.extend(vec![99.0; 10 - ok]);
            let k = build(v, Some(Target::AtLeast(100.0)));
            assert_eq!(k.situacao, light, "k = {ok}");
        }
    }

    #[test]
    fn at_most_goals_and_wilson_interval() {
        // 3 of 4 replications are at most 10.
        let k = build(vec![8.0, 9.0, 10.0, 15.0], Some(Target::AtMost(10.0)));
        let p = k.prob.unwrap();
        assert_eq!((p.k, p.n), (3, 4));
        // Wilson 95% for 3/4: (0.3006, 0.9544).
        assert!(
            (p.lo - 0.3006).abs() < 1e-3 && (p.hi - 0.9544).abs() < 1e-3,
            "{p:?}"
        );
        assert_eq!(k.situacao, Situation::Amarelo);
        assert_eq!(k.meta.unwrap().sentido, "max");
    }

    #[test]
    fn interval_crossing_the_goal_is_undefined_and_says_how_many_more() {
        let k = build(
            vec![98.0, 104.0, 101.0, 99.0, 103.0],
            Some(Target::AtLeast(100.5)),
        );
        assert!(k.indefinido);
        assert!(k.replicacoes_faltam.unwrap() >= 1);
        // Far from the goal: settled.
        let k = build(
            vec![98.0, 104.0, 101.0, 99.0, 103.0],
            Some(Target::AtLeast(50.0)),
        );
        assert!(!k.indefinido && k.situacao == Situation::Verde);
    }

    #[test]
    fn one_replication_has_no_probability() {
        let k = build(vec![5.0], Some(Target::AtLeast(1.0)));
        assert_eq!(k.situacao, Situation::Indefinido);
        assert!(k.ic.is_none() && k.prob.is_none());
        assert_eq!(build(vec![], None).texto, "-");
    }

    #[test]
    fn nan_replications_are_dropped() {
        let k = build(vec![1.0, f64::NAN, 3.0], None);
        assert_eq!(k.n, 2);
        assert_eq!(k.valor, 2.0);
        assert_eq!(k.valores.len(), 3);
    }
}
