//! Pure logic of the "Resultados" area (no egui): run settings, the summary table
//! (sorting, filter by kind), chart data, scenarios built from the rows of the
//! "Cenários" tab and optimisation problems built from the "Otimização" form.
//!
//! The statistics themselves come from `utsim_model::analysis` (one place for the
//! numbers of the CLI and the editor); this module only reshapes them for the screen.

use std::collections::BTreeMap;

use utsim_core::State;
use utsim_core::stats::{Histogram, Summary};
use utsim_edit::Command;
use utsim_model::analysis::{ObjectStats, bottleneck, delta_cell, is_significant, mean_hw};
use utsim_model::experiment::{ExperimentResult, Scenario, apply_patches};
use utsim_model::{Model, RunSettings};
use utsim_opt::{
    Algorithm, Constraint, Domain, Objective, OptProblem, Point, Refine, Sense, Variable,
};

use crate::properties::spec;
use crate::style;

/// Settings of an analysis run, edited in the panel (they start as the model's own and
/// change only the analysis, not the document).
#[derive(Clone, Debug, PartialEq)]
pub struct RunForm {
    pub replications: u32,
    pub stop_time: f64,
    pub warmup: f64,
    pub seed: u64,
    /// Record the content time series (`run.sample_interval`).
    pub series: bool,
    pub sample_interval: f64,
    /// Build histograms (`run.histogram_bins`).
    pub histograms: bool,
    pub bins: usize,
}

/// Default sampling interval: about 200 points over the statistics window.
pub fn default_interval(stop_time: f64, warmup: f64) -> f64 {
    let span = (stop_time - warmup).max(1e-9);
    let raw = span / 200.0;
    // A round number: 1, 2 or 5 times a power of ten.
    let p = 10f64.powf(raw.log10().floor());
    [1.0, 2.0, 5.0, 10.0]
        .into_iter()
        .map(|m| m * p)
        .find(|v| *v >= raw)
        .unwrap_or(raw)
}

impl RunForm {
    /// The settings of `run`; series and histograms are switched on (with a default
    /// interval and 20 bins) so the charts have data.
    pub fn of(run: &RunSettings) -> Self {
        RunForm {
            replications: run.replications.max(1),
            stop_time: run.stop_time,
            warmup: run.warmup,
            seed: run.seed,
            series: true,
            sample_interval: run
                .sample_interval
                .unwrap_or_else(|| default_interval(run.stop_time, run.warmup)),
            histograms: true,
            bins: run.histogram_bins.unwrap_or(20),
        }
    }

    /// A copy of `base` with these settings, validated (Portuguese messages).
    pub fn apply(&self, base: &Model) -> Result<Model, String> {
        let mut m = base.clone();
        m.run.replications = self.replications;
        m.run.stop_time = self.stop_time;
        m.run.warmup = self.warmup;
        m.run.seed = self.seed;
        m.run.sample_interval = self.series.then_some(self.sample_interval);
        m.run.histogram_bins = self.histograms.then_some(self.bins);
        m.validate().map_err(|e| e.to_string())?;
        Ok(m)
    }

    /// Commands that write these settings into the model, as one undo step (series and
    /// histograms are left as the model has them).
    pub fn commands(&self) -> Command {
        Command::Batch(vec![
            set("run.replications", self.replications.to_string()),
            set(
                "run.stop_time",
                crate::properties::ron_text::float(self.stop_time),
            ),
            set(
                "run.warmup",
                crate::properties::ron_text::float(self.warmup),
            ),
            set("run.seed", self.seed.to_string()),
        ])
    }

    /// One-line description, for the report.
    pub fn describe(&self) -> String {
        format!(
            "{} replicação(ões), de t = 0 a t = {} com aquecimento até t = {}, semente {}",
            self.replications, self.stop_time, self.warmup, self.seed
        )
    }
}

fn set(key: &str, value: String) -> Command {
    Command::Set {
        key: key.into(),
        value,
    }
}

/// Portuguese name of a block kind as the report names it.
pub fn kind_label(kind: &str) -> &'static str {
    match kind {
        "source" => "Chegada",
        "queue" => "Fila",
        "processor" => "Processo",
        "combiner" => "Montagem",
        "separator" => "Desmontagem",
        "sink" => "Saída",
        "transporter" => "Transportador",
        "dispatcher" => "Despachante",
        "conveyor" => "Esteira",
        "rack" => "Porta-paletes",
        "flow" => "Fluxo",
        "fluid" => "Fluidos",
        _ => "Outro",
    }
}

/// Distinct kinds of the rows, in first-seen order (the filter's options).
pub fn kinds(stats: &[ObjectStats]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for s in stats {
        if !out.contains(&s.kind) {
            out.push(s.kind.clone());
        }
    }
    out
}

/// Sortable columns of the summary table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Column {
    Name,
    Kind,
    Input,
    Output,
    Content,
    Staytime,
    Utilization,
}

impl Column {
    pub const ALL: [Column; 7] = [
        Column::Name,
        Column::Kind,
        Column::Input,
        Column::Output,
        Column::Content,
        Column::Staytime,
        Column::Utilization,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Column::Name => "Objeto",
            Column::Kind => "Tipo",
            Column::Input => "Entradas",
            Column::Output => "Saídas",
            Column::Content => "Conteúdo médio",
            Column::Staytime => "Permanência média",
            Column::Utilization => "Ocupação",
        }
    }

    fn value(self, s: &ObjectStats) -> Option<f64> {
        match self {
            Column::Name | Column::Kind => None,
            Column::Input => Some(s.input.mean),
            Column::Output => Some(s.output.mean),
            Column::Content => Some(s.content.mean),
            Column::Staytime => Some(s.staytime.mean),
            Column::Utilization => s.utilization.map(|u| u.mean),
        }
    }
}

/// Indexes of the rows to show: only `kind` when given, sorted by `column`
/// (ascending or descending; rows without a value always go last; ties keep the
/// report order).
pub fn table_order(
    stats: &[ObjectStats],
    kind: Option<&str>,
    column: Column,
    descending: bool,
) -> Vec<usize> {
    let mut rows: Vec<usize> = (0..stats.len())
        .filter(|&i| kind.is_none_or(|k| stats[i].kind == k))
        .collect();
    rows.sort_by(|&a, &b| {
        let (x, y) = (&stats[a], &stats[b]);
        let o = match column {
            Column::Name => x.name.cmp(&y.name),
            Column::Kind => kind_label(&x.kind).cmp(kind_label(&y.kind)),
            _ => match (column.value(x), column.value(y)) {
                (Some(p), Some(q)) => {
                    let o = p.total_cmp(&q);
                    return if descending { o.reverse() } else { o };
                }
                (Some(_), None) => return std::cmp::Ordering::Less,
                (None, Some(_)) => return std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            },
        };
        if descending { o.reverse() } else { o }
    });
    rows
}

/// Text of a cell: `mean ± hw`.
pub fn cell(s: &Summary) -> String {
    mean_hw(s)
}

/// Percentage cell: `87.5% ± 1.2`.
pub fn pct(s: &Summary) -> String {
    if s.half_width.is_nan() {
        format!("{:.1}%", 100.0 * s.mean)
    } else {
        format!("{:.1}% ± {:.1}", 100.0 * s.mean, 100.0 * s.half_width)
    }
}

/// A bar with its 95% interval (`lo..hi`, equal to `mean` without an interval).
#[derive(Clone, Debug, PartialEq)]
pub struct WhiskerBar {
    pub label: String,
    pub mean: f64,
    pub lo: f64,
    pub hi: f64,
    /// Drawn highlighted (the bottleneck, the best scenario).
    pub hot: bool,
}

fn whisker(label: String, s: &Summary, hot: bool) -> WhiskerBar {
    let hw = if s.half_width.is_nan() {
        0.0
    } else {
        s.half_width
    };
    WhiskerBar {
        label,
        mean: s.mean,
        lo: s.mean - hw,
        hi: s.mean + hw,
        hot,
    }
}

/// Utilisation bars of every resource, in report order; the bottleneck is hot.
pub fn utilization_bars(stats: &[ObjectStats]) -> Vec<WhiskerBar> {
    let hot = bottleneck(stats);
    stats
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            s.utilization
                .map(|u| whisker(s.name.clone(), &u, hot == Some(i)))
        })
        .collect()
}

const STATES: [State; 18] = [
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

/// State with this report name (`"processing"`...).
pub fn state_named(name: &str) -> Option<State> {
    STATES.into_iter().find(|s| s.name() == name)
}

/// Colour of a state, the same as the 3D view's (`style::state_color`).
pub fn state_color(name: &str) -> [f32; 3] {
    style::state_color(state_named(name))
}

/// Portuguese label of a state (the report name when it is not a known state).
pub fn state_label(name: &str) -> String {
    match state_named(name) {
        Some(s) => style::state_label(Some(s)).to_string(),
        None => name.to_string(),
    }
}

/// One piece of a stacked state bar.
#[derive(Clone, Debug, PartialEq)]
pub struct StateSegment {
    pub state: String,
    /// Where the piece starts and how wide it is, as fractions of the bar (0..1).
    pub start: f64,
    pub width: f64,
    pub color: [f32; 3],
}

/// One stacked bar per object with states: fractions normalised to sum to 1.
pub fn state_bars(stats: &[ObjectStats], rows: &[usize]) -> Vec<(String, Vec<StateSegment>)> {
    rows.iter()
        .map(|&i| &stats[i])
        .filter(|s| !s.states.is_empty())
        .map(|s| {
            let total: f64 = s.states.iter().map(|(_, v)| v.mean).sum();
            let mut start = 0.0;
            let segs = s
                .states
                .iter()
                .map(|(state, v)| {
                    let width = if total > 0.0 { v.mean / total } else { 0.0 };
                    let seg = StateSegment {
                        state: state.clone(),
                        start,
                        width,
                        color: state_color(state),
                    };
                    start += width;
                    seg
                })
                .collect();
            (s.name.clone(), segs)
        })
        .collect()
}

/// Bars of a histogram as `(center, width, count)`.
pub fn histogram_bars(h: &Histogram) -> Vec<(f64, f64, f64)> {
    let width = if h.bin_width > 0.0 { h.bin_width } else { 1.0 };
    (0..h.counts.len())
        .map(|k| {
            let (a, _) = h.edges(k);
            (a + width / 2.0, width, h.counts[k] as f64)
        })
        .collect()
}

/// A parameter that a scenario or a decision variable can change: the patch key, a
/// Portuguese description and the current value as RON text.
#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub key: String,
    pub label: String,
    pub current: String,
}

/// Every parameter the experiment patcher accepts on this model (each object's own
/// fields and the run fields), with the current value.
pub fn parameters(model: &Model) -> Vec<Parameter> {
    let mut out = Vec::new();
    for obj in &model.objects {
        for f in spec::all_fields(&obj.kind) {
            if let Some(current) = spec::read_field(obj, f.key) {
                out.push(Parameter {
                    key: format!("{}.{}", obj.name, f.key),
                    label: format!("{}: {}", obj.name, f.label),
                    current,
                });
            }
        }
        if let Some(t) = &obj.transfer {
            out.push(Parameter {
                key: format!("{}.transfer.speed", obj.name),
                label: format!("{}: velocidade da transferência", obj.name),
                current: crate::properties::ron_text::float(t.speed),
            });
        }
    }
    for f in spec::run_fields() {
        if let Some(current) = spec::read_run(&model.run, f.key) {
            out.push(Parameter {
                key: format!("run.{}", f.key),
                label: format!("Simulação: {}", f.label),
                current,
            });
        }
    }
    if let Some(t) = &model.run.transfer {
        out.push(Parameter {
            key: "run.transfer.speed".into(),
            label: "Simulação: velocidade padrão das transferências".into(),
            current: crate::properties::ron_text::float(t.speed),
        });
    }
    out
}

/// Current value of a parameter key, when it is one of [`parameters`].
pub fn current_value(model: &Model, key: &str) -> Option<String> {
    parameters(model)
        .into_iter()
        .find(|p| p.key == key)
        .map(|p| p.current)
}

/// The "Cenários" tab: parameter columns and one row per scenario.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScenarioForm {
    pub name: String,
    /// Patch keys, one per column.
    pub keys: Vec<String>,
    pub rows: Vec<ScenarioRow>,
    /// Metrics to compare (empty: every metric).
    pub measures: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScenarioRow {
    pub name: String,
    /// RON text per key; an empty cell keeps the base model's value.
    pub values: Vec<String>,
}

impl ScenarioForm {
    /// A form with the base scenario only.
    pub fn new(name: &str) -> Self {
        ScenarioForm {
            name: name.to_string(),
            keys: Vec::new(),
            rows: vec![ScenarioRow {
                name: "base".into(),
                values: Vec::new(),
            }],
            measures: Vec::new(),
        }
    }

    pub fn add_key(&mut self, key: &str) {
        if key.trim().is_empty() || self.keys.iter().any(|k| k == key) {
            return;
        }
        self.keys.push(key.trim().to_string());
        for r in &mut self.rows {
            r.values.resize(self.keys.len(), String::new());
        }
    }

    pub fn remove_key(&mut self, index: usize) {
        if index < self.keys.len() {
            self.keys.remove(index);
            for r in &mut self.rows {
                if index < r.values.len() {
                    r.values.remove(index);
                }
            }
        }
    }

    /// Adds a scenario with a unique name; its cells start empty (base values).
    pub fn add_row(&mut self) {
        let mut n = self.rows.len() + 1;
        let mut name = format!("cenário {n}");
        while self.rows.iter().any(|r| r.name == name) {
            n += 1;
            name = format!("cenário {n}");
        }
        self.rows.push(ScenarioRow {
            name,
            values: vec![String::new(); self.keys.len()],
        });
    }

    /// The scenarios, checked against `base`: unique non-empty names, and each one
    /// patches and validates (errors in Portuguese, one per problem).
    pub fn build(&self, base: &Model) -> Result<Vec<Scenario>, Vec<String>> {
        let mut errors = Vec::new();
        let mut out = Vec::new();
        if self.rows.is_empty() {
            errors.push("adicione pelo menos um cenário".to_string());
        }
        for (i, row) in self.rows.iter().enumerate() {
            let name = row.name.trim();
            if name.is_empty() {
                errors.push(format!("cenário {}: dê um nome", i + 1));
            } else if self.rows[..i].iter().any(|r| r.name.trim() == name) {
                errors.push(format!("cenário '{name}': nome repetido"));
            }
            let set: BTreeMap<String, String> = self
                .keys
                .iter()
                .zip(&row.values)
                .filter(|(_, v)| !v.trim().is_empty())
                .map(|(k, v)| (k.clone(), v.trim().to_string()))
                .collect();
            if let Err(e) = apply_patches(base, &set) {
                errors.push(format!("cenário '{name}': {e}"));
            }
            out.push(Scenario {
                name: name.to_string(),
                set,
            });
        }
        if errors.is_empty() {
            Ok(out)
        } else {
            Err(errors)
        }
    }

    /// The form of an experiment file's scenarios (columns: every key any scenario sets).
    pub fn from_scenarios(name: &str, scenarios: &[Scenario], measures: &[String]) -> Self {
        let mut keys: Vec<String> = scenarios
            .iter()
            .flat_map(|s| s.set.keys().cloned())
            .collect();
        keys.sort();
        keys.dedup();
        let rows = scenarios
            .iter()
            .map(|s| ScenarioRow {
                name: s.name.clone(),
                values: keys
                    .iter()
                    .map(|k| s.set.get(k).cloned().unwrap_or_default())
                    .collect(),
            })
            .collect();
        ScenarioForm {
            name: name.to_string(),
            keys,
            rows,
            measures: measures.to_vec(),
        }
    }
}

/// One line of the scenario comparison table.
#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonRow {
    pub metric: String,
    /// `mean ± hw` per scenario.
    pub cells: Vec<String>,
    /// Paired difference to the first scenario per scenario (`None` for the first),
    /// as text with `≈` when not significant, and whether it is significant.
    pub deltas: Vec<Option<(String, bool)>>,
}

pub fn comparison_rows(result: &ExperimentResult) -> Vec<ComparisonRow> {
    result
        .metric_names()
        .into_iter()
        .map(|m| ComparisonRow {
            cells: result
                .scenarios
                .iter()
                .map(|s| s.summary.get(&m).map_or("-".to_string(), cell))
                .collect(),
            deltas: (0..result.scenarios.len())
                .map(|i| {
                    result
                        .difference_to_first(i, &m)
                        .map(|d| (delta_cell(&d), is_significant(&d)))
                })
                .collect(),
            metric: m,
        })
        .collect()
}

/// Bars of one metric per scenario, with the 95% interval.
pub fn scenario_bars(result: &ExperimentResult, metric: &str) -> Vec<WhiskerBar> {
    result
        .scenarios
        .iter()
        .filter_map(|s| {
            s.summary
                .get(metric)
                .map(|v| whisker(s.name.clone(), v, false))
        })
        .collect()
}

/// Domain kind of a decision variable in the form.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DomainKind {
    #[default]
    Integer,
    Continuous,
    Choice,
}

/// One decision variable of the "Otimização" form.
#[derive(Clone, Debug, PartialEq)]
pub struct VariableRow {
    pub key: String,
    pub kind: DomainKind,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    /// Continuous: RON text with `{x}` for the value.
    pub template: String,
    /// Continuous: number of grid values.
    pub steps: usize,
    /// Choice: options separated by `;`.
    pub choices: String,
    /// Cost per unit of the value in the objective (numeric variables; 0: no cost).
    pub cost: f64,
}

impl VariableRow {
    pub fn new(key: &str) -> Self {
        VariableRow {
            key: key.to_string(),
            kind: DomainKind::Integer,
            min: 1.0,
            max: 4.0,
            step: 1.0,
            template: "{x}".into(),
            steps: 5,
            choices: String::new(),
            cost: 0.0,
        }
    }

    fn domain(&self) -> Domain {
        match self.kind {
            DomainKind::Integer => Domain::Integer {
                min: self.min.round() as i64,
                max: self.max.round() as i64,
                step: (self.step.round() as i64).max(1),
            },
            DomainKind::Continuous => Domain::Continuous {
                min: self.min,
                max: self.max,
                template: self.template.clone(),
                steps: self.steps,
            },
            DomainKind::Choice => Domain::Choice(
                self.choices
                    .split(';')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect(),
            ),
        }
    }
}

/// Search method of the form.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Method {
    #[default]
    Grid,
    Annealing,
    Genetic,
}

/// A bound on a metric's mean; empty texts are no bound.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConstraintRow {
    pub metric: String,
    pub min: String,
    pub max: String,
}

/// The "Otimização" tab.
#[derive(Clone, Debug, PartialEq)]
pub struct OptForm {
    pub name: String,
    pub replications: u32,
    pub variables: Vec<VariableRow>,
    pub objective_metric: String,
    pub weight: f64,
    pub maximize: bool,
    pub constraints: Vec<ConstraintRow>,
    pub method: Method,
    pub iterations: usize,
    pub population: usize,
    pub generations: usize,
    pub budget: usize,
    pub seed: u64,
    pub refine: bool,
    pub refine_top: usize,
    pub refine_replications: u32,
}

impl Default for OptForm {
    fn default() -> Self {
        OptForm {
            name: "otimização".into(),
            replications: 5,
            variables: Vec::new(),
            objective_metric: String::new(),
            weight: 1.0,
            maximize: false,
            constraints: Vec::new(),
            method: Method::Grid,
            iterations: 40,
            population: 8,
            generations: 6,
            budget: 50,
            seed: 1,
            refine: false,
            refine_top: 3,
            refine_replications: 10,
        }
    }
}

fn bound(text: &str, what: &str, metric: &str, errors: &mut Vec<String>) -> Option<f64> {
    let t = text.trim().replace(',', ".");
    if t.is_empty() {
        return None;
    }
    match t.parse::<f64>() {
        Ok(v) if v.is_finite() => Some(v),
        _ => {
            errors.push(format!(
                "restrição de '{metric}': {what} '{text}' não é um número"
            ));
            None
        }
    }
}

impl OptForm {
    /// The optimisation problem, checked against `base` (`OptProblem::validate` plus
    /// the form's own checks). `model` is the model file name recorded in the problem.
    pub fn build(&self, base: &Model, model: &str) -> Result<OptProblem, Vec<String>> {
        let mut errors = Vec::new();
        if self.objective_metric.trim().is_empty() {
            errors.push("escolha a métrica do objetivo".to_string());
        }
        let mut constraints = Vec::new();
        for c in &self.constraints {
            if c.metric.trim().is_empty() {
                continue;
            }
            let min = bound(&c.min, "mínimo", &c.metric, &mut errors);
            let max = bound(&c.max, "máximo", &c.metric, &mut errors);
            if min.is_none() && max.is_none() {
                errors.push(format!(
                    "restrição de '{}': informe mínimo ou máximo",
                    c.metric
                ));
            }
            constraints.push(Constraint {
                metric: c.metric.trim().to_string(),
                min,
                max,
            });
        }
        let algorithm = match self.method {
            Method::Grid => Algorithm::Grid,
            Method::Annealing => Algorithm::Annealing {
                iterations: self.iterations.max(1),
            },
            Method::Genetic => Algorithm::Genetic {
                population: self.population.max(2),
                generations: self.generations,
            },
        };
        let problem = OptProblem {
            name: self.name.clone(),
            model: model.to_string(),
            replications: self.replications,
            variables: self
                .variables
                .iter()
                .map(|v| Variable {
                    key: v.key.trim().to_string(),
                    domain: v.domain(),
                })
                .collect(),
            objective: Objective {
                sense: if self.maximize {
                    Sense::Maximize
                } else {
                    Sense::Minimize
                },
                terms: if self.objective_metric.trim().is_empty() {
                    Vec::new()
                } else {
                    vec![(self.objective_metric.trim().to_string(), self.weight)]
                },
                costs: self
                    .variables
                    .iter()
                    .filter(|v| v.cost != 0.0 && v.kind != DomainKind::Choice)
                    .map(|v| (v.key.trim().to_string(), v.cost))
                    .collect(),
            },
            constraints,
            algorithm,
            budget: self.budget.max(1),
            seed: self.seed,
            refine: self.refine.then_some(Refine {
                top: self.refine_top,
                replications: self.refine_replications,
            }),
        };
        errors.extend(problem.validate(base));
        if errors.is_empty() {
            Ok(problem)
        } else {
            Err(errors)
        }
    }
}

/// `chave = valor, ...` of a point.
pub fn point_text(point: &Point) -> String {
    point
        .iter()
        .map(|(k, v)| format!("{k} = {v}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// One line of the table of evaluated points.
#[derive(Clone, Debug, PartialEq)]
pub struct OptRow {
    pub point: String,
    pub objective: String,
    pub feasible: bool,
    pub is_best: bool,
    /// Statistically equivalent to the best (paired test, 95%).
    pub equivalent: bool,
    /// Paired difference to the best (`≈` when not significant); empty for the best.
    pub delta: String,
}

/// Rows of the final comparison (refined points when there are any), feasible first,
/// best objective on top, as the CLI report sorts them.
pub fn opt_rows(problem: &OptProblem, result: &utsim_opt::OptResult) -> Vec<OptRow> {
    let evals = result.final_evaluations();
    let best = result.best.map(|b| &evals[b]);
    let mut order: Vec<usize> = (0..evals.len()).collect();
    let sign = if problem.objective.sense == Sense::Maximize {
        -1.0
    } else {
        1.0
    };
    order.sort_by(|&a, &b| {
        let (x, y) = (&evals[a], &evals[b]);
        y.feasible
            .cmp(&x.feasible)
            .then((sign * x.objective.mean).total_cmp(&(sign * y.objective.mean)))
    });
    order
        .into_iter()
        .map(|i| {
            let e = &evals[i];
            let is_best = result.best == Some(i);
            OptRow {
                point: point_text(&e.point),
                objective: mean_hw(&e.objective),
                feasible: e.feasible,
                is_best,
                equivalent: result.equivalent.contains(&i),
                delta: match best {
                    Some(b) if !is_best => delta_cell(&utsim_opt::paired_difference(e, b)),
                    _ => String::new(),
                },
            }
        })
        .collect()
}

/// Convergence curve: `(evaluations so far, best feasible objective so far)`.
pub fn convergence(result: &utsim_opt::OptResult) -> Vec<[f64; 2]> {
    result.history.iter().map(|&(n, v)| [n as f64, v]).collect()
}

/// "Aplicar ao modelo": sets every value of `point`, as one undo step.
pub fn apply_point(point: &Point) -> Command {
    Command::Batch(point.iter().map(|(k, v)| set(k, v.clone())).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use utsim_model::analysis::object_stats;
    use utsim_model::experiment::run_scenarios;

    const LINE: &str = r#"Model(
        name: "linha",
        run: (stop_time: 3000.0, warmup: 100.0, seed: 7, replications: 3),
        objects: [
            (name: "chegada", kind: Source(interarrival: Exp(mean: 4.0))),
            (name: "fila", kind: Queue()),
            (name: "corte", kind: Processor(process: Exp(mean: 2.0))),
            (name: "fila2", kind: Queue()),
            (name: "furacao", kind: Processor(process: Exp(mean: 3.6))),
            (name: "saida", kind: Sink),
        ],
        connections: [("chegada", "fila"), ("fila", "corte"), ("corte", "fila2"),
                      ("fila2", "furacao"), ("furacao", "saida")],
    )"#;

    fn model() -> Model {
        Model::parse(LINE).unwrap()
    }

    fn stats(form: &RunForm) -> Vec<ObjectStats> {
        let m = form.apply(&model()).unwrap();
        object_stats(&m, &m.run_replications().unwrap())
    }

    #[test]
    fn run_form_turns_series_on_and_validates() {
        let m = model();
        let f = RunForm::of(&m.run);
        assert!(f.series && f.histograms);
        assert_eq!(f.sample_interval, 20.0); // (3000 - 100) / 200 = 14.5 -> 20
        let run = f.apply(&m).unwrap();
        assert_eq!(run.run.sample_interval, Some(20.0));
        assert_eq!(run.run.histogram_bins, Some(20));
        let mut bad = f.clone();
        bad.warmup = 5000.0;
        assert!(bad.apply(&m).unwrap_err().contains("warmup"));
        assert!(matches!(f.commands(), Command::Batch(c) if c.len() == 4));
        assert_eq!(default_interval(1000.0, 0.0), 5.0);
    }

    #[test]
    fn table_sorts_filters_and_marks_the_bottleneck() {
        let s = stats(&RunForm::of(&model().run));
        let order = table_order(&s, None, Column::Utilization, true);
        assert_eq!(s[order[0]].name, "furacao");
        assert_eq!(s[order[1]].name, "corte");
        // Rows without utilisation go last, in either direction.
        let up = table_order(&s, None, Column::Utilization, false);
        assert_eq!(s[up[0]].name, "corte");
        assert!(s[*up.last().unwrap()].utilization.is_none());
        let queues = table_order(&s, Some("queue"), Column::Name, false);
        assert_eq!(
            queues
                .iter()
                .map(|&i| s[i].name.as_str())
                .collect::<Vec<_>>(),
            ["fila", "fila2"]
        );
        assert_eq!(kinds(&s), ["source", "queue", "processor", "sink"]);
        let bars = utilization_bars(&s);
        assert_eq!(bars.len(), 2);
        assert!(bars.iter().find(|b| b.label == "furacao").unwrap().hot);
        assert!(bars.iter().all(|b| b.lo <= b.mean && b.mean <= b.hi));
    }

    #[test]
    fn state_bars_use_the_3d_colours_and_sum_to_one() {
        let s = stats(&RunForm::of(&model().run));
        let all: Vec<usize> = (0..s.len()).collect();
        let bars = state_bars(&s, &all);
        let (_, segs) = bars.iter().find(|(n, _)| n == "furacao").unwrap();
        let total: f64 = segs.iter().map(|g| g.width).sum();
        assert!((total - 1.0).abs() < 1e-9);
        let busy = segs.iter().find(|g| g.state == "processing").unwrap();
        assert_eq!(busy.color, style::state_color(Some(State::Processing)));
        assert_eq!(state_label("processing"), "processando");
        assert_eq!(state_label("nada"), "nada");
    }

    #[test]
    fn histogram_bars_are_centred() {
        let h = Histogram::build(&[0.0, 1.0, 2.0, 3.0], 2).unwrap();
        assert_eq!(histogram_bars(&h), [(0.75, 1.5, 2.0), (2.25, 1.5, 2.0)]);
    }

    #[test]
    fn parameters_list_patchable_keys_with_values() {
        let p = parameters(&model());
        let cap = p.iter().find(|p| p.key == "furacao.capacity").unwrap();
        assert_eq!(cap.current, "1");
        assert!(p.iter().any(|p| p.key == "chegada.interarrival"));
        assert_eq!(current_value(&model(), "run.seed").as_deref(), Some("7"));
        // Every listed key is accepted by the patcher with its own value.
        for param in &p {
            let set = BTreeMap::from([(param.key.clone(), param.current.clone())]);
            assert!(apply_patches(&model(), &set).is_ok(), "{param:?}");
        }
    }

    #[test]
    fn scenarios_from_rows() {
        let base = model();
        let mut f = ScenarioForm::new("servidores");
        f.add_key("furacao.capacity");
        f.add_key("furacao.capacity");
        f.add_row();
        f.rows[1].values[0] = "2".into();
        let sc = f.build(&base).unwrap();
        assert_eq!(sc.len(), 2);
        assert!(sc[0].set.is_empty(), "empty cells keep the base");
        assert_eq!(sc[1].set["furacao.capacity"], "2");
        assert_eq!(sc[1].name, "cenário 2");

        f.rows[1].values[0] = "0".into();
        f.rows.push(ScenarioRow {
            name: "base".into(),
            values: vec!["muitas".into()],
        });
        let errors = f.build(&base).unwrap_err();
        assert!(
            errors.iter().any(|e| e.contains("capacity deve ser >= 1")),
            "{errors:?}"
        );
        assert!(
            errors.iter().any(|e| e.contains("nome repetido")),
            "{errors:?}"
        );
        assert!(errors.iter().any(|e| e.contains("muitas")), "{errors:?}");

        let back = ScenarioForm::from_scenarios("e", &sc, &[]);
        assert_eq!(back.keys, ["furacao.capacity"]);
        assert_eq!(back.rows[0].values, [""]);
        f.remove_key(0);
        assert!(f.keys.is_empty() && f.rows.iter().all(|r| r.values.is_empty()));
    }

    #[test]
    fn comparison_rows_flag_significance() {
        let mut base = model();
        base.run.replications = 4;
        let mut f = ScenarioForm::new("e");
        f.add_key("furacao.capacity");
        f.add_row();
        f.rows[1].values[0] = "2".into();
        let measures = vec!["fila2.content.mean".to_string()];
        let scenarios =
            run_scenarios(&base, &f.build(&base).unwrap(), &measures, &|_, _| true).unwrap();
        let result = ExperimentResult {
            name: "e".into(),
            measures,
            scenarios,
        };
        let rows = comparison_rows(&result);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].deltas[0], None);
        let (text, significant) = rows[0].deltas[1].clone().unwrap();
        assert!(significant && text.starts_with('-'), "{text}");
        assert_eq!(scenario_bars(&result, "fila2.content.mean").len(), 2);
    }

    #[test]
    fn optimisation_form_builds_a_valid_problem() {
        let base = model();
        let mut f = OptForm::default();
        assert!(f.build(&base, "m.ron").is_err());
        let mut v = VariableRow::new("furacao.capacity");
        v.min = 1.0;
        v.max = 3.0;
        v.cost = 10.0;
        f.variables.push(v);
        f.objective_metric = "saida.time_in_system.mean".into();
        f.constraints.push(ConstraintRow {
            metric: "furacao.state.processing".into(),
            min: String::new(),
            max: "0,95".into(),
        });
        let p = f.build(&base, "m.ron").unwrap();
        assert_eq!(p.objective.costs, [("furacao.capacity".to_string(), 10.0)]);
        assert_eq!(p.constraints[0].max, Some(0.95));
        assert_eq!(utsim_opt::values(&p.variables[0].domain), ["1", "2", "3"]);
        f.method = Method::Genetic;
        f.refine = true;
        let p = f.build(&base, "m.ron").unwrap();
        assert!(matches!(p.algorithm, Algorithm::Genetic { .. }) && p.refine.is_some());
        f.constraints[0].max = "x".into();
        assert!(f.build(&base, "m.ron").is_err());
        f.constraints.clear();
        f.method = Method::Grid;
        f.refine = false;
        f.replications = 2;
        let p = f.build(&base, "m.ron").unwrap();
        let r = utsim_opt::optimize(&p, &base).unwrap();
        let rows = opt_rows(&p, &r);
        assert_eq!(rows.len(), 3);
        assert!(rows[0].is_best && rows[0].delta.is_empty());
        assert!(rows[1..].iter().all(|r| !r.delta.is_empty()));
        assert!(!convergence(&r).is_empty());
        let mut c = VariableRow::new("chegada.interarrival");
        c.kind = DomainKind::Choice;
        c.choices = "Exp(mean: 4.0); Exp(mean: 3.0)".into();
        assert_eq!(utsim_opt::values(&c.domain()).len(), 2);
    }

    #[test]
    fn applying_a_point_is_one_undo_step() {
        let point = Point::from([
            ("furacao.capacity".to_string(), "2".to_string()),
            ("run.seed".to_string(), "3".to_string()),
        ]);
        let path = std::env::temp_dir().join("utsim_results_apply_point.ron");
        std::fs::write(&path, LINE).unwrap();
        let mut doc = utsim_edit::Document::open(&path).unwrap();
        doc.apply(apply_point(&point)).unwrap();
        assert!(doc.model().objects.iter().any(|o| matches!(
            o.kind,
            utsim_model::ObjectKind::Processor { capacity: 2, .. }
        )));
        assert_eq!(doc.model().run.seed, 3);
        doc.undo();
        assert_eq!(doc.model().run.seed, 7);
    }
}
