//! Forecast ("previsão"): from a state of the plant, `R` replications run ahead to a
//! horizon (the end of the shift or `H` hours) and say what is likely to happen.
//!
//! ```text
//!  observed state (twin) or a state file ──> InitialState ──> R replications to the horizon
//!                                                              |  (threads, independent seeds)
//!                 per KPI: mean, CI of the mean, P5–P95 range, P(goal) with Wilson, light
//!                 per instant: the band of production, WIP and the watched contents
//!                 per limit: P(the content crosses it), when (P5, P50, P95)
//!                 "what if": the same state and seeds with the changes, paired differences
//! ```
//!
//! **Seeds.** Replication `r` of a forecast uses the seed `seed_for(seed, instant)` and the
//! replication index `r`: every forecast is reproducible, two forecasts at different
//! instants are independent, and the scenarios of one forecast use the very same streams
//! as the base ("números aleatórios comuns"), so their paired differences are sharp.
//! `utsim prever` (no twin) uses the model's seed as is, so a forecast from a state file
//! repeats `utsim run --estado` with `--stop` at the horizon.
//!
//! **Two intervals.** The confidence interval of the mean ("precisão da média") says how
//! far the computed mean may be from the true one and shrinks with more replications. The
//! P5–P95 range ("faixa do que pode acontecer") says where the actual result of this
//! shift will fall in 9 out of 10 cases and does not shrink with more replications.
//!
//! The probability of meeting a goal is the fraction `k/n` of replications that meet it,
//! with a Wilson interval; the traffic light is green from 90%, yellow from 50%, red below
//! (configurable), as in the dashboard. Crossing times of limits are found on the sampling
//! grid (`points` instants over the horizon).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use utsim_core::{RunReport, Simulation, Step};
use utsim_dash::config::{Dashboard, Target, Thresholds, TimeUnit};
use utsim_dash::kpi::{self, Kpi, Probabilidade, Situation, Spec};
use utsim_dash::num::{decimals, pt, pt_mean_hw, quantile, wilson};
use utsim_model::experiment::{Scenario, apply_patches};
use utsim_model::{InitialState, Model};

use crate::shadow::{self, ObjectState};

/// How far a forecast looks.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Horizon {
    /// This many model time units.
    #[serde(alias = "Tempo")]
    Time(f64),
    /// This many hours (converted with the model's time unit).
    #[serde(alias = "Horas")]
    Hours(f64),
    /// Up to the next end of shift: `ends` are calendar instants within a cycle of
    /// `period` (e.g. `[480.0, 960.0, 1440.0]` in a day of 1440 minutes); the calendar
    /// instant of the start is the state's `clock`.
    #[serde(alias = "FimDoTurno")]
    EndOfShift {
        #[serde(alias = "fins")]
        ends: Vec<f64>,
        #[serde(alias = "periodo")]
        period: f64,
    },
}

impl Default for Horizon {
    fn default() -> Self {
        Horizon::Hours(4.0)
    }
}

impl Horizon {
    /// Reads `4h`, `30min`, `90s`, `2.5h` or a plain number of model time units.
    pub fn parse(text: &str) -> Result<Horizon, String> {
        let t = text.trim().to_lowercase().replace(',', ".");
        let bad = || {
            format!(
                "horizonte '{text}' inválido: use 4h, 30min, 90s ou um número (unidades do modelo)"
            )
        };
        let (num, scale) = if let Some(n) = t.strip_suffix("min") {
            (n, Some(1.0 / 60.0))
        } else if let Some(n) = t.strip_suffix('h') {
            (n, Some(1.0))
        } else if let Some(n) = t.strip_suffix('s') {
            (n, Some(1.0 / 3600.0))
        } else {
            (t.as_str(), None)
        };
        let v: f64 = num.trim().parse().map_err(|_| bad())?;
        if !(v.is_finite() && v > 0.0) {
            return Err(bad());
        }
        Ok(match scale {
            Some(s) => Horizon::Hours(v * s),
            None => Horizon::Time(v),
        })
    }

    /// The horizon in model time units, from a state whose `t = 0` is the calendar
    /// instant `clock`.
    pub fn resolve(&self, clock: f64, unit: TimeUnit) -> Result<f64, String> {
        let h = match self {
            Horizon::Time(t) => *t,
            Horizon::Hours(h) => {
                if unit == TimeUnit::Unit {
                    return Err(
                        "o modelo usa unidade de tempo abstrata (time_unit: Unit): dê o horizonte em unidades do modelo (Time)"
                            .into(),
                    );
                }
                h * unit.per_hour()
            }
            Horizon::EndOfShift { ends, period } => {
                if !(period.is_finite() && *period > 0.0) || ends.is_empty() {
                    return Err("EndOfShift precisa de period positivo e ao menos um fim".into());
                }
                let c = clock.rem_euclid(*period);
                let mut sorted: Vec<f64> = ends.iter().map(|e| e.rem_euclid(*period)).collect();
                sorted.sort_by(f64::total_cmp);
                let eps = 1e-9 * period.max(1.0);
                match sorted.iter().find(|e| **e > c + eps) {
                    Some(e) => e - c,
                    None => sorted[0] + period - c,
                }
            }
        };
        if h.is_finite() && h > 0.0 {
            Ok(h)
        } else {
            Err(format!("o horizonte deve ser positivo (está {h})"))
        }
    }
}

/// A limit on the content of an object: the forecast says how likely it is to be crossed
/// before the horizon, and when.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limit {
    #[serde(alias = "objeto")]
    pub object: String,
    /// The content goes above this.
    #[serde(default, alias = "acima_de", skip_serializing_if = "Option::is_none")]
    pub above: Option<f64>,
    /// The content goes below this.
    #[serde(default, alias = "abaixo_de", skip_serializing_if = "Option::is_none")]
    pub below: Option<f64>,
}

impl Limit {
    fn crossed(&self, x: f64) -> bool {
        match (self.above, self.below) {
            (Some(a), _) => x > a,
            (None, Some(b)) => x < b,
            (None, None) => false,
        }
    }

    fn value(&self) -> f64 {
        self.above.or(self.below).unwrap_or(f64::NAN)
    }
}

/// The `forecast` section of the `.gemeo.ron` file, also a file of its own
/// (`Forecast(...)`) for `utsim prever --config`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Forecast", deny_unknown_fields)]
pub struct ForecastSettings {
    /// A forecast every this many twin time units (absent: a quarter of an hour; 0: only
    /// on request).
    #[serde(default, alias = "a_cada", skip_serializing_if = "Option::is_none")]
    pub every: Option<f64>,
    #[serde(default, alias = "horizonte")]
    pub horizon: Horizon,
    #[serde(default = "default_reps", alias = "replicacoes")]
    pub replications: u32,
    /// Base seed (absent: the model's).
    #[serde(default, alias = "semente", skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// What the model's time unit is (absent: the dashboard file's, else seconds).
    #[serde(
        default,
        alias = "unidade_tempo",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_unit: Option<TimeUnit>,
    /// Calendar instant at twin time 0 (the twin's state `clock` is twin time plus this).
    #[serde(default, alias = "relogio_inicio")]
    pub start_clock: f64,
    /// Confidence level of the intervals.
    #[serde(default = "default_level", alias = "nivel")]
    pub level: f64,
    #[serde(default, alias = "limiares")]
    pub thresholds: Thresholds,
    /// Goals by KPI id (`producao`, `vazao`, `wip`, `lead_time`, `conteudo_final:<obj>`...).
    #[serde(default, alias = "metas")]
    pub targets: BTreeMap<String, Target>,
    /// Limits on contents whose crossing is forecast.
    #[serde(default, alias = "limites")]
    pub limits: Vec<Limit>,
    /// Objects whose content is shown in time (band) and at the horizon.
    #[serde(default, alias = "acompanhar")]
    pub watch: Vec<String>,
    /// KPIs compared in the scenarios (absent: the ones with goals, production, throughput,
    /// WIP and lead time).
    #[serde(default, alias = "comparar")]
    pub compare: Vec<String>,
    /// "What if" variations, as in an experiment: `(name: ..., set: {"obj.campo": "valor"})`.
    #[serde(default, alias = "cenarios")]
    pub scenarios: Vec<Scenario>,
    /// Sampling instants over the horizon (band and crossing times).
    #[serde(default = "default_points", alias = "pontos")]
    pub points: u32,
}

fn default_reps() -> u32 {
    40
}
fn default_level() -> f64 {
    0.95
}
fn default_points() -> u32 {
    48
}

impl Default for ForecastSettings {
    fn default() -> Self {
        ForecastSettings {
            every: None,
            horizon: Horizon::default(),
            replications: default_reps(),
            seed: None,
            time_unit: None,
            start_clock: 0.0,
            level: default_level(),
            thresholds: Thresholds::default(),
            targets: BTreeMap::new(),
            limits: Vec::new(),
            watch: Vec::new(),
            compare: Vec::new(),
            scenarios: Vec::new(),
            points: default_points(),
        }
    }
}

impl ForecastSettings {
    pub fn parse(text: &str) -> Result<ForecastSettings, String> {
        ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(text)
            .map_err(|e| format!("erro de sintaxe na previsão: {e}"))
    }

    /// Interval between scheduled forecasts in model time units (`None`: only on request).
    pub fn interval(&self, unit: TimeUnit) -> Option<f64> {
        match self.every {
            Some(e) if e > 0.0 => Some(e),
            Some(_) => None,
            None => Some(0.25 * unit.per_hour()),
        }
    }

    /// Checks the settings against the model: errors (into `errors`) and warnings.
    pub fn validate(&self, model: &Model, errors: &mut Vec<String>) -> Vec<String> {
        let mut warnings = Vec::new();
        let at = "forecast";
        if let Some(e) = self.every {
            if !(e.is_finite() && e >= 0.0) {
                errors.push(format!("{at}.every deve ser ≥ 0 (0: só sob pedido)"));
            }
        }
        match &self.horizon {
            Horizon::Time(h) | Horizon::Hours(h) => {
                if !(h.is_finite() && *h > 0.0) {
                    errors.push(format!("{at}.horizon deve ser positivo"));
                }
            }
            Horizon::EndOfShift { ends, period } => {
                if !(period.is_finite() && *period > 0.0) {
                    errors.push(format!("{at}.horizon: period deve ser positivo"));
                } else if ends.is_empty()
                    || ends
                        .iter()
                        .any(|e| !(e.is_finite() && *e >= 0.0 && e <= period))
                {
                    errors.push(format!(
                        "{at}.horizon: ends deve ter ao menos um instante entre 0 e period ({period})"
                    ));
                }
            }
        }
        if self.time_unit == Some(TimeUnit::Unit) && matches!(self.horizon, Horizon::Hours(_)) {
            errors.push(format!(
                "{at}: horizonte em horas com unidade abstrata (Unit); use Time(...)"
            ));
        }
        if !(2..=10_000).contains(&self.replications) {
            errors.push(format!("{at}.replications deve ficar entre 2 e 10000"));
        }
        if !(self.level > 0.0 && self.level < 1.0) {
            errors.push(format!("{at}.level deve ficar entre 0 e 1"));
        }
        let t = self.thresholds;
        if !(0.0..=1.0).contains(&t.yellow) || !(0.0..=1.0).contains(&t.green) || t.yellow > t.green
        {
            errors.push(format!("{at}.thresholds: use 0 <= yellow <= green <= 1"));
        }
        if !self.start_clock.is_finite() {
            errors.push(format!("{at}.start_clock deve ser finito"));
        }
        if !(1..=2000).contains(&self.points) {
            errors.push(format!("{at}.points deve ficar entre 1 e 2000"));
        }
        for (k, v) in &self.targets {
            if !v.value().is_finite() {
                errors.push(format!("{at}.targets['{k}']: valor deve ser finito"));
            }
        }
        let exists = |name: &str| model.objects.iter().any(|o| o.name == name);
        for (i, l) in self.limits.iter().enumerate() {
            if !exists(&l.object) {
                errors.push(format!(
                    "{at}.limits[{i}]: o objeto '{}' não existe no modelo",
                    l.object
                ));
            }
            match (l.above, l.below) {
                (Some(v), None) | (None, Some(v)) if v.is_finite() => {}
                (Some(_), Some(_)) => {
                    errors.push(format!("{at}.limits[{i}]: use above ou below, não os dois"))
                }
                _ => errors.push(format!(
                    "{at}.limits[{i}]: dê above ou below (um número finito)"
                )),
            }
        }
        for w in &self.watch {
            if !exists(w) {
                errors.push(format!("{at}.watch: o objeto '{w}' não existe no modelo"));
            }
        }
        let mut names = BTreeSet::new();
        for sc in &self.scenarios {
            if sc.name.trim().is_empty() {
                errors.push(format!("{at}.scenarios: há um cenário sem nome"));
            } else if !names.insert(sc.name.as_str()) {
                errors.push(format!(
                    "{at}.scenarios: o nome '{}' aparece duas vezes",
                    sc.name
                ));
            }
            if sc.set.is_empty() {
                warnings.push(format!(
                    "{at}.scenarios: '{}' não muda nada (igual ao como está)",
                    sc.name
                ));
            }
            if let Err(e) = apply_patches(model, &sc.set) {
                errors.push(format!("{at}.scenarios: cenário '{}': {e}", sc.name));
            }
            if sc.set.keys().any(|k| k.starts_with("run.")) {
                warnings.push(format!(
                    "{at}.scenarios: '{}' muda campos de run; o horizonte, a semente e as replicações são os da previsão",
                    sc.name
                ));
            }
        }
        warnings
    }
}

/// Mixes a base seed with the instant of a forecast (SplitMix64): every instant gets its
/// own independent streams, the same instant always the same.
pub fn seed_for(base: u64, instant: f64) -> u64 {
    fn mix(mut z: u64) -> u64 {
        z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    mix(base ^ mix(instant.to_bits()))
}

/// One point of a band in time: quantiles across replications and the mean with its CI.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BandPoint {
    /// Absolute time (the forecast's instant plus the time ahead).
    pub t: f64,
    pub media: f64,
    /// Confidence interval of the mean.
    pub ic: Option<[f64; 2]>,
    pub p5: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p95: f64,
}

/// A quantity in time ("leque").
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Band {
    pub id: String,
    pub nome: String,
    pub unidade: String,
    /// Goal at the horizon, drawn as a line (production).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<f64>,
    pub pontos: Vec<BandPoint>,
}

/// The forecast crossing of a limit.
#[derive(Clone, Debug, Serialize)]
pub struct Crossing {
    pub objeto: String,
    /// `"acima"` or `"abaixo"`.
    pub sentido: &'static str,
    pub limite: f64,
    /// Probability that it happens before the horizon (fraction of replications).
    pub prob: Probabilidade,
    /// Quantiles of the instant among the replications in which it happens (absolute time).
    pub instante_p5: Option<f64>,
    pub instante_p50: Option<f64>,
    pub instante_p95: Option<f64>,
    /// Median instant over all replications (when at least half cross).
    pub instante_mediano: Option<f64>,
    pub texto: String,
}

/// One KPI of a scenario against the base.
#[derive(Clone, Debug, Serialize)]
pub struct Comparison {
    pub id: String,
    pub nome: String,
    pub unidade: String,
    /// Mean in the scenario and its text.
    pub valor: f64,
    pub texto: String,
    /// Mean of the base ("como está").
    pub base: f64,
    /// Mean of the paired differences scenario − base, and its CI.
    pub diferenca: f64,
    pub ic_diferenca: Option<[f64; 2]>,
    pub texto_diferenca: String,
    /// The CI of the difference excludes zero.
    pub significativa: bool,
    /// Better than the base (by the goal, else by the kind of KPI), when significant.
    pub melhora: Option<bool>,
    pub prob: Option<Probabilidade>,
    pub situacao: Situation,
}

/// A "what if" scenario evaluated from the same state with the same seeds.
#[derive(Clone, Debug, Serialize)]
pub struct ScenarioForecast {
    pub nome: String,
    pub mudancas: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erro: Option<String>,
    pub kpis: Vec<Comparison>,
}

/// The result of a forecast.
#[derive(Clone, Debug, Serialize)]
pub struct Forecast {
    /// Twin time of the state (0 for a state file).
    pub instante: f64,
    /// Calendar instant of the state.
    pub relogio: f64,
    /// Horizon, model time units, and as text.
    pub horizonte: f64,
    pub horizonte_texto: String,
    pub unidade: &'static str,
    pub replicacoes: u32,
    pub semente: u64,
    pub nivel: f64,
    /// Asked for (on demand) instead of scheduled.
    pub pedido: bool,
    pub kpis: Vec<Kpi>,
    pub series: Vec<Band>,
    pub eventos: Vec<Crossing>,
    pub cenarios: Vec<ScenarioForecast>,
    pub gargalo: Option<String>,
    pub resumo: String,
    pub avisos: Vec<String>,
    pub notas_estado: Vec<String>,
}

impl Forecast {
    pub fn kpi(&self, id: &str) -> Option<&Kpi> {
        self.kpis.iter().find(|k| k.id == id)
    }
}

/// What one replication gives.
pub struct Replication {
    pub report: RunReport,
    /// Value of each series ([`series_ids`]) at each sampling instant.
    pub series: Vec<Vec<f64>>,
    /// First sampling instant (time ahead) at which each limit is crossed.
    pub crossings: Vec<Option<f64>>,
}

fn holds_wip(kind: &str) -> bool {
    matches!(
        kind,
        "queue" | "processor" | "conveyor" | "rack" | "combiner" | "separator"
    )
}

fn step_to(model: &Model, sim: &mut Simulation, t: f64) -> Result<(), String> {
    loop {
        match sim.run_until(t).map_err(|e| e.to_string())? {
            Step::Decision(d) => match model.run.default_decision {
                Some(c) => sim
                    .decide(c.min(d.options.saturating_sub(1)))
                    .map_err(|e| e.to_string())?,
                None => {
                    return Err(
                        "o modelo pede decisões de um agente externo; defina run.default_decision para prever"
                            .into(),
                    );
                }
            },
            _ => return Ok(()),
        }
    }
}

/// The model a forecast runs: the horizon as stop time, `reps` replications, `seed`.
fn horizon_model(model: &Model, horizon: f64, reps: u32, seed: u64) -> Model {
    let mut m = model.clone();
    m.run.stop_time = horizon;
    m.run.replications = reps;
    m.run.seed = seed;
    // The state's warm-up rules (0 unless the state says otherwise).
    m.run.warmup = 0.0;
    m
}

/// Objects whose content is a series: the watched ones, then the limited ones.
fn tracked(settings: &ForecastSettings) -> Vec<String> {
    let mut v: Vec<String> = settings.watch.clone();
    for l in &settings.limits {
        if !v.contains(&l.object) {
            v.push(l.object.clone());
        }
    }
    v
}

/// Runs replication `rep` of `model` (already the horizon model) from `state`, sampling
/// production, WIP and the content of `objects` at `times`.
pub fn replicate(
    model: &Model,
    state: &InitialState,
    rep: u32,
    times: &[f64],
    objects: &[String],
    limits: &[Limit],
) -> Result<Replication, String> {
    let mut sim = model.build_from(rep, Some(state));
    sim.begin().map_err(|e| e.to_string())?;
    let first = sim.report();
    let kinds: Vec<String> = first.blocks.iter().map(|b| b.kind.clone()).collect();
    let names: Vec<String> = sim.block_status().into_iter().map(|(n, _, _)| n).collect();
    let idx = |o: &str| names.iter().position(|n| n == o);
    let object_ids: Vec<Option<usize>> = objects.iter().map(|o| idx(o)).collect();
    let limit_ids: Vec<Option<usize>> = limits.iter().map(|l| idx(&l.object)).collect();
    let mut series = vec![Vec::with_capacity(times.len()); 2 + objects.len()];
    let mut crossings: Vec<Option<f64>> = vec![None; limits.len()];
    for &t in times {
        if t > 0.0 {
            step_to(model, &mut sim, t)?;
        }
        let report = sim.report();
        let produced: u64 = report
            .blocks
            .iter()
            .filter(|b| b.kind == "sink")
            .map(|b| b.input)
            .sum();
        let status = sim.block_status();
        let wip: usize = status
            .iter()
            .enumerate()
            .filter(|(i, _)| kinds.get(*i).is_some_and(|k| holds_wip(k)))
            .map(|(_, (_, _, c))| *c)
            .sum();
        series[0].push(produced as f64);
        series[1].push(wip as f64);
        for (k, id) in object_ids.iter().enumerate() {
            series[2 + k].push(id.map_or(f64::NAN, |i| status[i].2 as f64));
        }
        for (k, l) in limits.iter().enumerate() {
            if crossings[k].is_none() {
                if let Some(i) = limit_ids[k] {
                    if l.crossed(status[i].2 as f64) {
                        crossings[k] = Some(t);
                    }
                }
            }
        }
    }
    step_to(model, &mut sim, f64::INFINITY)?;
    Ok(Replication {
        report: sim.finish(),
        series,
        crossings,
    })
}

/// Maps `f` over `0..n` in up to 8 threads; the result does not depend on the number of
/// threads.
fn par_map<T: Send>(n: u32, f: impl Fn(u32) -> T + Sync) -> Vec<T> {
    let reps: Vec<u32> = (0..n).collect();
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .clamp(1, 8);
    let chunk = reps.len().div_ceil(threads).max(1);
    std::thread::scope(|sc| {
        let handles: Vec<_> = reps
            .chunks(chunk)
            .map(|part| {
                let f = &f;
                sc.spawn(move || part.iter().map(|&r| f(r)).collect::<Vec<_>>())
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("forecast thread"))
            .collect()
    })
}

/// Runs `reps` replications of `model` (the horizon model) from `state`.
pub fn replicate_all(
    model: &Model,
    state: &InitialState,
    times: &[f64],
    objects: &[String],
    limits: &[Limit],
) -> Result<Vec<Replication>, String> {
    par_map(model.run.replications, |r| {
        replicate(model, state, r, times, objects, limits)
    })
    .into_iter()
    .collect()
}

/// KPIs of a set of replications: production, the dashboard's KPIs, the final content of
/// the watched objects.
fn kpis_of(
    model: &Model,
    reps: &[Replication],
    dash: &Dashboard,
    watch: &[String],
) -> (Vec<Kpi>, Option<String>) {
    let reports: Vec<RunReport> = reps.iter().map(|r| r.report.clone()).collect();
    let spec = |id: &str, nome: String, unidade: &str, formula: &str| Spec {
        id: id.to_string(),
        nome,
        unidade: unidade.to_string(),
        formula: formula.to_string(),
        target: dash.targets.get(id).copied(),
        cfg: dash,
    };
    let produced: Vec<f64> = reports
        .iter()
        .map(|r| {
            r.blocks
                .iter()
                .filter(|b| b.kind == "sink")
                .map(|b| b.input as f64)
                .sum()
        })
        .collect();
    let mut kpis = Vec::new();
    if reports
        .first()
        .is_some_and(|r| r.blocks.iter().any(|b| b.kind == "sink"))
    {
        kpis.push(Kpi::build(
            spec(
                "producao",
                "Produção no horizonte".into(),
                "itens",
                "itens que chegam às Sinks entre o instante da previsão e o horizonte",
            ),
            produced,
        ));
    }
    let set = kpi::compute(model, &reports, dash);
    kpis.extend(set.kpis);
    for (k, w) in watch.iter().enumerate() {
        let v: Vec<f64> = reps
            .iter()
            .map(|r| r.series[2 + k].last().copied().unwrap_or(f64::NAN))
            .collect();
        kpis.push(Kpi::build(
            spec(
                &format!("conteudo_final:{w}"),
                format!("Conteúdo de {w} no horizonte"),
                "itens",
                "itens no objeto no instante do horizonte",
            ),
            v,
        ));
    }
    (kpis, set.gargalo)
}

/// Whether a higher value of KPI `id` is better (when it has no goal).
fn higher_is_better(id: &str) -> Option<bool> {
    if id == "producao"
        || id == "vazao"
        || id == "otd"
        || id == "nivel_servico"
        || id.starts_with("oee:")
    {
        Some(true)
    } else if id.starts_with("lead_time")
        || id == "wip"
        || id == "perdas"
        || id.starts_with("conteudo_final:")
    {
        Some(false)
    } else {
        None
    }
}

fn band_point(t: f64, xs: &[f64], level: f64) -> BandPoint {
    let v: Vec<f64> = xs.iter().copied().filter(|x| x.is_finite()).collect();
    let s = utsim_stats::summary(&v, level);
    BandPoint {
        t,
        media: s.mean,
        ic: (v.len() >= 2 && s.half_width.is_finite())
            .then_some([s.mean - s.half_width, s.mean + s.half_width]),
        p5: quantile(&v, 0.05),
        p25: quantile(&v, 0.25),
        p50: quantile(&v, 0.5),
        p75: quantile(&v, 0.75),
        p95: quantile(&v, 0.95),
    }
}

fn pct(p: f64) -> String {
    format!("{} %", pt(100.0 * p, 0))
}

fn light(s: Situation) -> &'static str {
    match s {
        Situation::Verde => "verde",
        Situation::Amarelo => "amarelo",
        Situation::Vermelho => "vermelho",
        Situation::SemMeta => "sem meta",
        Situation::Indefinido => "indefinido",
    }
}

fn horizon_text(h: f64, unit: TimeUnit) -> String {
    match unit {
        TimeUnit::Unit => format!("{} u.t.", pt(h, decimals(h, f64::NAN).min(2))),
        _ => {
            let hours = h / unit.per_hour();
            if hours >= 1.0 {
                format!("{} h", pt(hours, decimals(hours, f64::NAN).min(2)))
            } else {
                let min = hours * 60.0;
                format!("{} min", pt(min, decimals(min, f64::NAN).min(1)))
            }
        }
    }
}

/// Everything a forecast needs besides the state.
pub struct Request<'a> {
    pub model: &'a Model,
    pub settings: &'a ForecastSettings,
    pub unit: TimeUnit,
    /// Twin time of the state (shown on the time axis).
    pub instant: f64,
    /// Seed of the replications (see [`seed_for`]).
    pub seed: u64,
    pub on_request: bool,
}

/// Runs a forecast from `state` (and its scenarios).
pub fn forecast(req: &Request, state: &InitialState) -> Result<Forecast, String> {
    let s = req.settings;
    let horizon = s.horizon.resolve(state.clock, req.unit)?;
    let base = horizon_model(req.model, horizon, s.replications, req.seed);
    let mut avisos: Vec<String> = base.check_state(state).map_err(|e| e.to_string())?;
    let points = s.points.max(1);
    let times: Vec<f64> = (0..=points)
        .map(|k| horizon * f64::from(k) / f64::from(points))
        .collect();
    let objects = tracked(s);
    let reps = replicate_all(&base, state, &times, &objects, &s.limits)?;
    let dash = Dashboard {
        time_unit: req.unit,
        level: s.level,
        thresholds: s.thresholds,
        targets: s.targets.clone(),
        ..Dashboard::default()
    };
    let (kpis, gargalo) = kpis_of(&base, &reps, &dash, &s.watch);
    for k in s.targets.keys() {
        if !kpis.iter().any(|x| x.id == *k) {
            avisos.push(format!(
                "a meta '{k}' não corresponde a nenhum KPI da previsão"
            ));
        }
    }

    // Bands in time.
    let n = reps.len();
    let mut series = Vec::new();
    let names: Vec<(String, String, &str)> = [
        (
            "producao".to_string(),
            "Produção acumulada".to_string(),
            "itens",
        ),
        (
            "wip".to_string(),
            "Estoque em processo (WIP)".to_string(),
            "itens",
        ),
    ]
    .into_iter()
    .chain(
        objects
            .iter()
            .map(|o| (format!("conteudo:{o}"), format!("Conteúdo de {o}"), "itens")),
    )
    .collect();
    for (k, (id, nome, unidade)) in names.into_iter().enumerate() {
        let pontos = times
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                let xs: Vec<f64> = reps.iter().map(|r| r.series[k][i]).collect();
                band_point(req.instant + t, &xs, s.level)
            })
            .collect();
        let meta = (id == "producao")
            .then(|| s.targets.get("producao").map(|t| t.value()))
            .flatten();
        series.push(Band {
            id,
            nome,
            unidade: unidade.to_string(),
            meta,
            pontos,
        });
    }

    // Crossings of limits.
    let eventos = s
        .limits
        .iter()
        .enumerate()
        .map(|(k, l)| {
            let mut at: Vec<f64> = reps
                .iter()
                .filter_map(|r| r.crossings[k])
                .map(|t| req.instant + t)
                .collect();
            at.sort_by(f64::total_cmp);
            let hits = at.len();
            let (lo, hi) = wilson(hits, n, s.level);
            let p = if n > 0 { hits as f64 / n as f64 } else { 0.0 };
            let q = |x: f64| (!at.is_empty()).then(|| quantile(&at, x));
            // Lower median over all replications, the ones that never cross counting
            // as "after the horizon".
            let mediano = if n > 0 {
                at.get((n - 1) / 2).copied()
            } else {
                None
            };
            let sentido = if l.above.is_some() { "acima" } else { "abaixo" };
            let d = decimals(horizon, f64::NAN).min(1);
            let mut texto = format!(
                "{} {sentido} de {}: em {} das replicações (Wilson {}: {} a {})",
                l.object,
                pt(l.value(), decimals(l.value(), f64::NAN).min(2)),
                pct(p),
                pct(s.level),
                pct(lo),
                pct(hi)
            );
            if let (Some(a), Some(m), Some(b)) = (q(0.05), q(0.5), q(0.95)) {
                texto += &format!(
                    "; quando acontece, por volta de t = {} (P5 a P95: {} a {})",
                    pt(m, d),
                    pt(a, d),
                    pt(b, d)
                );
            }
            Crossing {
                objeto: l.object.clone(),
                sentido,
                limite: l.value(),
                prob: Probabilidade {
                    p,
                    lo,
                    hi,
                    k: hits,
                    n,
                },
                instante_p5: q(0.05),
                instante_p50: q(0.5),
                instante_p95: q(0.95),
                instante_mediano: mediano,
                texto,
            }
        })
        .collect();

    // Scenarios: same state, same seeds (common random numbers).
    let compare: Vec<String> = if s.compare.is_empty() {
        let mut v: Vec<String> = s.targets.keys().cloned().collect();
        for id in ["producao", "vazao", "wip", "lead_time"] {
            if !v.iter().any(|x| x == id) {
                v.push(id.to_string());
            }
        }
        v.extend(s.watch.iter().map(|w| format!("conteudo_final:{w}")));
        v
    } else {
        s.compare.clone()
    };
    let mut cenarios = Vec::new();
    for sc in &s.scenarios {
        let run = || -> Result<Vec<Comparison>, String> {
            let patched = apply_patches(req.model, &sc.set).map_err(|e| e.to_string())?;
            let m = horizon_model(&patched, horizon, s.replications, req.seed);
            m.check_state(state).map_err(|e| e.to_string())?;
            let sreps = replicate_all(&m, state, &times, &objects, &s.limits)?;
            let (skpis, _) = kpis_of(&m, &sreps, &dash, &s.watch);
            Ok(compare
                .iter()
                .filter_map(|id| {
                    let b = kpis.iter().find(|k| k.id == *id)?;
                    let k = skpis.iter().find(|k| k.id == *id)?;
                    let d: Vec<f64> = k
                        .valores
                        .iter()
                        .zip(&b.valores)
                        .filter(|(x, y)| x.is_finite() && y.is_finite())
                        .map(|(x, y)| x - y)
                        .collect();
                    let ds = utsim_stats::summary(&d, s.level);
                    let ic = (d.len() >= 2 && ds.half_width.is_finite())
                        .then_some([ds.mean - ds.half_width, ds.mean + ds.half_width]);
                    let significativa = ic.is_some_and(|[a, b]| a > 0.0 || b < 0.0);
                    let up = match s.targets.get(id) {
                        Some(Target::AtLeast(_)) => Some(true),
                        Some(Target::AtMost(_)) => Some(false),
                        None => higher_is_better(id),
                    };
                    let melhora = (significativa && ds.mean != 0.0)
                        .then_some(up.map(|u| u == (ds.mean > 0.0)))
                        .flatten();
                    let mut texto_diferenca = if ds.half_width.is_finite() {
                        let dd = decimals(ds.mean, ds.half_width);
                        format!(
                            "{}{} ± {}",
                            if ds.mean >= 0.0 { "+" } else { "" },
                            pt(ds.mean, dd),
                            pt(ds.half_width, dd)
                        )
                    } else {
                        pt(ds.mean, decimals(ds.mean, f64::NAN))
                    };
                    if !significativa {
                        texto_diferenca += " ≈";
                    }
                    Some(Comparison {
                        id: id.clone(),
                        nome: k.nome.clone(),
                        unidade: k.unidade.clone(),
                        valor: k.valor,
                        texto: k.texto.clone(),
                        base: b.valor,
                        diferenca: ds.mean,
                        ic_diferenca: ic,
                        texto_diferenca,
                        significativa,
                        melhora,
                        prob: k.prob.clone(),
                        situacao: k.situacao,
                    })
                })
                .collect())
        };
        let (kpis, erro) = match run() {
            Ok(k) => (k, None),
            Err(e) => (Vec::new(), Some(e)),
        };
        cenarios.push(ScenarioForecast {
            nome: sc.name.clone(),
            mudancas: sc.set.clone(),
            erro,
            kpis,
        });
    }

    let horizonte_texto = horizon_text(horizon, req.unit);
    let resumo = summary(&kpis, n, s.level, req.instant, horizon, &horizonte_texto);
    avisos.sort();
    avisos.dedup();
    Ok(Forecast {
        instante: req.instant,
        relogio: state.clock,
        horizonte: horizon,
        horizonte_texto,
        unidade: req.unit.label(),
        replicacoes: s.replications,
        semente: req.seed,
        nivel: s.level,
        pedido: req.on_request,
        kpis,
        series,
        eventos,
        cenarios,
        gargalo,
        resumo,
        avisos,
        notas_estado: Vec::new(),
    })
}

/// The manager's sentence: production (or the first KPI with a goal), its precision, its
/// range and the probability of the goal.
fn summary(kpis: &[Kpi], n: usize, level: f64, instant: f64, horizon: f64, htext: &str) -> String {
    let d = decimals(instant + horizon, f64::NAN).min(1);
    let mut s = format!(
        "Previsão de t = {} a t = {} ({htext}), com {n} replicações.",
        pt(instant, d),
        pt(instant + horizon, d)
    );
    let main = kpis
        .iter()
        .find(|k| k.id == "producao")
        .or_else(|| kpis.iter().find(|k| k.meta.is_some()));
    if let Some(k) = main {
        let half = k.ic.map_or(f64::NAN, |[a, b]| (b - a) / 2.0);
        s += &format!(
            " {} esperada: {} {} (precisão da média, IC {})",
            k.nome,
            pt_mean_hw(k.valor, half),
            k.unidade,
            pct(level)
        );
        if k.n >= 2 {
            s += &format!(
                "; em 9 de cada 10 casos fica entre {} (faixa P5 a P95)",
                k.faixa.replace(" a ", " e ")
            );
        }
        s += ".";
    }
    for k in kpis.iter().filter(|k| k.meta.is_some()) {
        let meta = k.meta.as_ref().expect("filtered");
        if let Some(p) = &k.prob {
            s += &format!(
                " A meta de {} {} para {} é cumprida em {} dos casos (Wilson {}: {} a {}): {}.",
                if meta.sentido == "min" {
                    "no mínimo"
                } else {
                    "no máximo"
                },
                pt(meta.valor, decimals(meta.valor, f64::NAN).min(2)),
                k.nome.to_lowercase(),
                pct(p.p),
                pct(level),
                pct(p.lo),
                pct(p.hi),
                light(k.situacao)
            );
        }
    }
    s
}

/// An instant of the twin at which a forecast was asked for, with what was observed.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub t: f64,
    pub on_request: bool,
    pub objects: BTreeMap<String, ObjectState>,
}

/// The forecast of a twin snapshot: the observed state as an initial state (clock =
/// `start_clock` + twin time), the seed mixed with the instant.
pub fn of_snapshot(
    model: &Model,
    settings: &ForecastSettings,
    unit: TimeUnit,
    snap: &Snapshot,
) -> Result<Forecast, String> {
    let (mut state, notes) = shadow::initial_state(model, &snap.objects, snap.t);
    state.clock += settings.start_clock;
    let base = settings.seed.unwrap_or(model.run.seed);
    let mut f = forecast(
        &Request {
            model,
            settings,
            unit,
            instant: snap.t,
            seed: seed_for(base, snap.t),
            on_request: snap.on_request,
        },
        &state,
    )?;
    f.notas_estado = notes;
    Ok(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizons_parse_and_resolve() {
        assert_eq!(Horizon::parse("4h").unwrap(), Horizon::Hours(4.0));
        assert_eq!(Horizon::parse("30min").unwrap(), Horizon::Hours(0.5));
        assert_eq!(Horizon::parse("90s").unwrap(), Horizon::Hours(0.025));
        assert_eq!(Horizon::parse("240").unwrap(), Horizon::Time(240.0));
        assert_eq!(Horizon::parse("2,5h").unwrap(), Horizon::Hours(2.5));
        assert!(Horizon::parse("-1h").is_err() && Horizon::parse("x").is_err());
        assert_eq!(
            Horizon::Hours(4.0).resolve(0.0, TimeUnit::Minutes).unwrap(),
            240.0
        );
        assert_eq!(
            Horizon::Hours(1.0).resolve(0.0, TimeUnit::Seconds).unwrap(),
            3600.0
        );
        assert!(Horizon::Hours(1.0).resolve(0.0, TimeUnit::Unit).is_err());
        let shift = Horizon::EndOfShift {
            ends: vec![480.0, 960.0, 1440.0],
            period: 1440.0,
        };
        // 21h30 in minutes since 06h00 = 930: the shift ends at 960.
        assert_eq!(shift.resolve(930.0, TimeUnit::Minutes).unwrap(), 30.0);
        // Exactly at an end: the next one.
        assert_eq!(shift.resolve(480.0, TimeUnit::Minutes).unwrap(), 480.0);
        // Next day.
        assert_eq!(
            shift.resolve(1440.0 + 100.0, TimeUnit::Minutes).unwrap(),
            380.0
        );
        assert_eq!(shift.resolve(1000.0, TimeUnit::Minutes).unwrap(), 440.0);
    }

    #[test]
    fn seeds_depend_on_the_instant_only() {
        assert_eq!(seed_for(7, 900.0), seed_for(7, 900.0));
        assert_ne!(seed_for(7, 900.0), seed_for(7, 1800.0));
        assert_ne!(seed_for(7, 900.0), seed_for(8, 900.0));
    }

    #[test]
    fn settings_parse_with_portuguese_names() {
        let s = ForecastSettings::parse(
            r#"Forecast(a_cada: 600.0, horizonte: FimDoTurno(fins: [480.0], periodo: 1440.0),
                replicacoes: 20, metas: {"producao": AtLeast(450.0)},
                limites: [(objeto: "fila", acima_de: 20.0)], acompanhar: ["fila"],
                cenarios: [(name: "mais um", set: {"maquina.capacity": "2"})])"#,
        )
        .unwrap();
        assert_eq!(s.every, Some(600.0));
        assert_eq!(s.replications, 20);
        assert_eq!(s.limits[0].above, Some(20.0));
        assert_eq!(s.scenarios[0].name, "mais um");
        assert_eq!(s.interval(TimeUnit::Seconds), Some(600.0));
        let d = ForecastSettings::default();
        assert_eq!(d.interval(TimeUnit::Seconds), Some(900.0));
        assert_eq!(d.interval(TimeUnit::Minutes), Some(15.0));
        let on_demand = ForecastSettings {
            every: Some(0.0),
            ..d
        };
        assert_eq!(on_demand.interval(TimeUnit::Seconds), None);
        assert!(ForecastSettings::parse("Forecast(horizonte: Horas(4.0), x: 1)").is_err());
    }

    /// The example of the manual parses.
    #[test]
    fn manual_example_parses() {
        let page = include_str!("../../../docs/manual/src/gemeo-digital.md");
        let start = page.find("```ron\nForecast(").expect("exemplo") + "```ron\n".len();
        let end = start + page[start..].find("```").unwrap();
        let s = ForecastSettings::parse(&page[start..end]).unwrap();
        assert_eq!(s.time_unit, Some(TimeUnit::Minutes));
        assert_eq!(s.start_clock, 360.0);
        assert_eq!(s.scenarios.len(), 2);
        // Twin time 0 is 06h00 (minute 360 of the day): the shift ends at 480 (08h00).
        assert_eq!(
            s.horizon.resolve(s.start_clock, TimeUnit::Minutes).unwrap(),
            120.0
        );
    }
}
