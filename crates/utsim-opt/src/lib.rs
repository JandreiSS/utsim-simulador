//! Simulation-based optimization (phase 7, contract).
//!
//! An [`OptProblem`] names a model, the decision variables (same keys as scenario
//! files: `objeto.campo`, `run.campo`), an objective built from report metrics and
//! variable costs, and constraints on metrics. Every candidate point is simulated
//! with the **same seeds** (common random numbers), so differences between points
//! come from the decisions, not from luck; [`compare`] then uses a paired test to
//! say whether the best point is really better than each other one.
//!
//! The search algorithms front implements [`Algorithm::Annealing`] and
//! [`Algorithm::Genetic`]; [`Algorithm::Grid`] is the reference.

mod search;
#[cfg(test)]
mod search_tests;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use utsim_core::stats::Summary;
use utsim_model::experiment::apply_patches;
use utsim_model::{Model, ModelError};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptProblem {
    pub name: String,
    /// Model file, relative to the problem file.
    pub model: String,
    /// Replications per evaluated point (same seeds for every point).
    #[serde(default = "five")]
    pub replications: u32,
    pub variables: Vec<Variable>,
    pub objective: Objective,
    #[serde(default)]
    pub constraints: Vec<Constraint>,
    #[serde(default)]
    pub algorithm: Algorithm,
    /// Maximum number of distinct points simulated.
    #[serde(default = "default_budget")]
    pub budget: usize,
    /// Seed of the search itself (not of the simulations).
    #[serde(default = "one_u64")]
    pub seed: u64,
    /// Optional refinement: after the search, re-simulate the best points with more
    /// replications (same seeds for all of them) and compare those.
    #[serde(default)]
    pub refine: Option<Refine>,
}

/// Final refinement step, available for every algorithm.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Refine {
    /// How many of the best distinct feasible points to re-simulate.
    pub top: usize,
    /// Replications for each refined point (replaces `OptProblem::replications`;
    /// the first ones reuse the seeds of the search, so they are a superset).
    pub replications: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    /// Scenario key, e.g. `"maquina.capacity"`.
    pub key: String,
    pub domain: Domain,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Domain {
    /// Integers `min, min+step, ..., max`, written as the value itself.
    Integer {
        min: i64,
        max: i64,
        #[serde(default = "one_i64")]
        step: i64,
    },
    /// Reals in `[min, max]`; `template` is RON text with `{x}` replaced by the
    /// value (e.g. `"Exp(mean: {x})"`, or `"{x}"` for a plain number). Grid search
    /// uses `steps` evenly spaced values.
    Continuous {
        min: f64,
        max: f64,
        #[serde(default = "default_template")]
        template: String,
        #[serde(default = "five_usize")]
        steps: usize,
    },
    /// One of the given RON values (e.g. `["Fifo", "Lifo"]`).
    Choice(Vec<String>),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sense {
    #[default]
    Minimize,
    Maximize,
}

/// Objective = Σ weight·metric + Σ cost·variable value (numeric variables only).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Objective {
    #[serde(default)]
    pub sense: Sense,
    /// `(metric key, weight)`, e.g. `("saida.time_in_system.mean", 1.0)`.
    pub terms: Vec<(String, f64)>,
    /// `(variable key, cost per unit)`, e.g. `("maquina.capacity", 50.0)`.
    #[serde(default)]
    pub costs: Vec<(String, f64)>,
}

/// A bound on the **mean** of a metric across replications.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    pub metric: String,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Algorithm {
    /// Every combination (continuous domains use `steps` values).
    #[default]
    Grid,
    /// Simulated annealing over neighbouring points.
    Annealing { iterations: usize },
    /// Genetic algorithm with integer/choice genes.
    Genetic {
        population: usize,
        generations: usize,
    },
}

fn five() -> u32 {
    5
}

fn five_usize() -> usize {
    5
}

fn one_u64() -> u64 {
    1
}

fn one_i64() -> i64 {
    1
}

fn default_budget() -> usize {
    200
}

fn default_template() -> String {
    "{x}".to_string()
}

/// A candidate: one value per variable, as RON text ready for the scenario patcher.
pub type Point = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize)]
pub struct Evaluation {
    pub point: Point,
    /// Objective value of each replication (paired across points by seed).
    pub per_replication: Vec<f64>,
    pub objective: Summary,
    /// Constraint metric means and whether each bound holds.
    pub constraints: Vec<(String, Summary, bool)>,
    pub feasible: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct OptResult {
    pub problem: String,
    /// Every simulated point, in the order it was evaluated.
    pub evaluations: Vec<Evaluation>,
    /// Index of the best feasible evaluation. When `refined` is not empty it indexes
    /// `refined`; otherwise it indexes `evaluations` (see [`OptResult::final_evaluations`]).
    pub best: Option<usize>,
    /// Points not significantly different from the best (paired test, 95%). Indexes
    /// the same list as `best`.
    pub equivalent: Vec<usize>,
    /// Portuguese explanation of the outcome.
    pub explanation: Vec<String>,
    /// Re-simulation of the best search points with more replications (empty when
    /// the problem has no `refine`). The final comparison (`best`, `equivalent`)
    /// is made on these.
    #[serde(default)]
    pub refined: Vec<Evaluation>,
    /// Convergence: `(evaluations so far, best feasible objective so far)`, one
    /// entry per simulated point once a feasible point exists.
    #[serde(default)]
    pub history: Vec<(usize, f64)>,
}

impl OptResult {
    /// The list `best` and `equivalent` index into: `refined` if present, else
    /// `evaluations`.
    pub fn final_evaluations(&self) -> &[Evaluation] {
        if self.refined.is_empty() {
            &self.evaluations
        } else {
            &self.refined
        }
    }

    pub fn best_evaluation(&self) -> Option<&Evaluation> {
        self.best.map(|i| &self.final_evaluations()[i])
    }
}

impl OptProblem {
    pub fn parse(text: &str) -> Result<OptProblem, ModelError> {
        let options = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
        Ok(options.from_str(text)?)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<(OptProblem, PathBuf), ModelError> {
        let path = path.as_ref();
        let problem = OptProblem::parse(&std::fs::read_to_string(path)?)?;
        let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
        Ok((problem, dir))
    }

    /// Structural checks (Portuguese messages), against the base model.
    pub fn validate(&self, base: &Model) -> Vec<String> {
        let mut errors = Vec::new();
        if self.variables.is_empty() {
            errors.push("otimização sem variáveis".into());
        }
        if self.replications < 2 {
            errors.push("use pelo menos 2 replicações por ponto (para o teste pareado)".into());
        }
        if let Some(r) = &self.refine
            && (r.top == 0 || r.replications < 2)
        {
            errors.push("refinamento: use top ≥ 1 e pelo menos 2 replicações".into());
        }
        for v in &self.variables {
            let ok = match &v.domain {
                Domain::Integer { min, max, step } => min <= max && *step > 0,
                Domain::Continuous {
                    min,
                    max,
                    template,
                    steps,
                } => min <= max && template.contains("{x}") && *steps >= 1,
                Domain::Choice(options) => !options.is_empty(),
            };
            if !ok {
                errors.push(format!("variável '{}': domínio inválido", v.key));
            }
            // The key must be patchable with the first value of the domain.
            let probe = first_value(&v.domain);
            let set = BTreeMap::from([(v.key.clone(), probe)]);
            if let Err(e) = apply_patches(base, &set) {
                errors.push(format!("variável '{}': {e}", v.key));
            }
        }
        if self.objective.terms.is_empty() && self.objective.costs.is_empty() {
            errors.push("objetivo sem termos".into());
        }
        for (key, _) in &self.objective.costs {
            let numeric = self
                .variables
                .iter()
                .any(|v| v.key == *key && !matches!(v.domain, Domain::Choice(_)));
            if !numeric {
                errors.push(format!(
                    "custo de '{key}': precisa ser uma variável numérica"
                ));
            }
        }
        errors
    }
}

fn format_number(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{x:.1}")
    } else {
        format!("{x}")
    }
}

fn first_value(domain: &Domain) -> String {
    values(domain).into_iter().next().unwrap_or_default()
}

/// Every value of a domain (continuous: `steps` evenly spaced), as RON text.
pub fn values(domain: &Domain) -> Vec<String> {
    match domain {
        Domain::Integer { min, max, step } => {
            let step = (*step).max(1) as usize;
            (*min..=*max).step_by(step).map(|v| v.to_string()).collect()
        }
        Domain::Continuous {
            min,
            max,
            template,
            steps,
        } => {
            let n = (*steps).max(1);
            (0..n)
                .map(|i| {
                    let x = if n == 1 {
                        *min
                    } else {
                        min + (max - min) * i as f64 / (n - 1) as f64
                    };
                    template.replace("{x}", &format_number(x))
                })
                .collect()
        }
        Domain::Choice(options) => options.clone(),
    }
}

/// Numeric value of a variable's text (for costs); `None` for non-numeric text.
fn numeric(text: &str) -> Option<f64> {
    text.trim().parse::<f64>().ok()
}

/// Simulates one point and scores it.
pub fn evaluate(
    base: &Model,
    problem: &OptProblem,
    point: &Point,
) -> Result<Evaluation, ModelError> {
    evaluate_with(base, problem, point, problem.replications)
}

/// Like [`evaluate`], with an explicit number of replications.
pub fn evaluate_with(
    base: &Model,
    problem: &OptProblem,
    point: &Point,
    replications: u32,
) -> Result<Evaluation, ModelError> {
    let mut model = apply_patches(base, point)?;
    model.run.replications = replications;
    let reports = model.run_replications()?;
    let metric = |r: &utsim_core::RunReport, key: &str| -> Result<f64, ModelError> {
        r.metrics().get(key).copied().ok_or_else(|| {
            ModelError::Invalid(vec![format!("métrica '{key}' não existe no relatório")])
        })
    };
    let costs: f64 = problem
        .objective
        .costs
        .iter()
        .map(|(key, c)| point.get(key).and_then(|v| numeric(v)).unwrap_or(0.0) * c)
        .sum();
    let mut per_replication = Vec::with_capacity(reports.len());
    for r in &reports {
        let mut value = costs;
        for (key, w) in &problem.objective.terms {
            value += w * metric(r, key)?;
        }
        per_replication.push(value);
    }
    let mut constraints = Vec::new();
    for c in &problem.constraints {
        let xs = reports
            .iter()
            .map(|r| metric(r, &c.metric))
            .collect::<Result<Vec<_>, _>>()?;
        let s = Summary::from_samples(&xs);
        let ok = c.min.is_none_or(|m| s.mean >= m) && c.max.is_none_or(|m| s.mean <= m);
        constraints.push((c.metric.clone(), s, ok));
    }
    Ok(Evaluation {
        point: point.clone(),
        objective: Summary::from_samples(&per_replication),
        per_replication,
        feasible: constraints.iter().all(|c| c.2),
        constraints,
    })
}

/// Paired comparison of `a` against `b` (same seeds): mean of (a − b) and its 95%
/// half-width. The difference is significant when the interval excludes zero.
pub fn paired_difference(a: &Evaluation, b: &Evaluation) -> Summary {
    let d: Vec<f64> = a
        .per_replication
        .iter()
        .zip(&b.per_replication)
        .map(|(x, y)| x - y)
        .collect();
    Summary::from_samples(&d)
}

/// Picks the best feasible evaluation and finds the statistically equivalent ones.
pub fn compare(problem: &OptProblem, evaluations: Vec<Evaluation>) -> OptResult {
    let better = |a: f64, b: f64| match problem.objective.sense {
        Sense::Minimize => a < b,
        Sense::Maximize => a > b,
    };
    let best = evaluations
        .iter()
        .enumerate()
        .filter(|(_, e)| e.feasible)
        .fold(None::<usize>, |acc, (i, e)| match acc {
            Some(j) if !better(e.objective.mean, evaluations[j].objective.mean) => Some(j),
            _ => Some(i),
        });
    let mut equivalent = Vec::new();
    let mut explanation = Vec::new();
    match best {
        Some(b) => {
            let e = &evaluations[b];
            explanation.push(format!(
                "Melhor ponto viável: {} (objetivo {:.4} ± {:.4}).",
                describe(&e.point),
                e.objective.mean,
                e.objective.half_width
            ));
            for (i, other) in evaluations.iter().enumerate() {
                if i == b || !other.feasible {
                    continue;
                }
                let d = paired_difference(other, e);
                if d.half_width.is_nan() || d.mean.abs() <= d.half_width {
                    equivalent.push(i);
                }
            }
            if !equivalent.is_empty() {
                explanation.push(format!(
                    "{} ponto(s) não diferem do melhor com 95% de confiança (teste pareado): \
                     mais replicações seriam necessárias para separá-los.",
                    equivalent.len()
                ));
            }
        }
        None => explanation.push("Nenhum ponto avaliado atende às restrições.".into()),
    }
    OptResult {
        problem: problem.name.clone(),
        evaluations,
        best,
        equivalent,
        explanation,
        refined: Vec::new(),
        history: Vec::new(),
    }
}

fn describe(point: &Point) -> String {
    point
        .iter()
        .map(|(k, v)| format!("{k} = {v}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Runs the search and compares the evaluated points.
pub fn optimize(problem: &OptProblem, base: &Model) -> Result<OptResult, ModelError> {
    optimize_with(problem, base, &mut |_, _| true)
}

/// Upper bound of the points the search will simulate (the budget, or fewer when the
/// algorithm stops earlier), plus the refined ones. Used for progress bars.
pub fn planned_evaluations(problem: &OptProblem) -> usize {
    let grid: usize = problem
        .variables
        .iter()
        .map(|v| values(&v.domain).len().max(1))
        .product();
    let search = match &problem.algorithm {
        Algorithm::Grid => grid,
        Algorithm::Annealing { iterations } => 1 + iterations.min(&5) + iterations,
        Algorithm::Genetic {
            population,
            generations,
        } => population.max(&2) * (generations + 1),
    };
    let search = search.min(grid).min(problem.budget).max(1);
    search + problem.refine.as_ref().map_or(0, |r| r.top.min(search))
}

/// Like [`optimize`], calling `progress(done, planned)` after every simulated point
/// (`planned` from [`planned_evaluations`]). Returning `false` stops the search with
/// [`ModelError::Cancelled`].
pub fn optimize_with(
    problem: &OptProblem,
    base: &Model,
    progress: &mut dyn FnMut(usize, usize) -> bool,
) -> Result<OptResult, ModelError> {
    let errors = problem.validate(base);
    if !errors.is_empty() {
        return Err(ModelError::Invalid(errors));
    }
    let planned = planned_evaluations(problem);
    let done = std::cell::Cell::new(0usize);
    let progress = std::cell::RefCell::new(progress);
    let tick = || -> Result<(), ModelError> {
        done.set(done.get() + 1);
        if (progress.borrow_mut())(done.get(), planned.max(done.get())) {
            Ok(())
        } else {
            Err(ModelError::Cancelled)
        }
    };
    let sim = |p: &Point| {
        let e = evaluate(base, problem, p)?;
        tick()?;
        Ok(e)
    };
    let (evaluations, history) = search::run_with(problem, sim)?;
    let mut result = compare(problem, evaluations);
    result.history = history;
    if let Some(refine) = &problem.refine {
        search::apply_refinement(problem, base, refine, &mut result, &tick)?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL: &str = r#"Model(
        name: "mmc",
        run: (stop_time: 20000.0, warmup: 1000.0, seed: 3),
        objects: [
            (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0))),
            (name: "fila", kind: Queue()),
            (name: "maquina", kind: Processor(capacity: 1, process: Exp(mean: 1.8))),
            (name: "saida", kind: Sink),
        ],
        connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
    )"#;

    fn problem() -> OptProblem {
        OptProblem::parse(
            r#"OptProblem(
                name: "servidores",
                model: "mmc.ron",
                replications: 4,
                variables: [(key: "maquina.capacity", domain: Integer(min: 2, max: 5))],
                objective: (
                    terms: [("fila.content.mean", 10.0)],
                    costs: [("maquina.capacity", 1.0)],
                ),
            )"#,
        )
        .unwrap()
    }

    #[test]
    fn values_of_domains() {
        assert_eq!(
            values(&Domain::Integer {
                min: 1,
                max: 7,
                step: 3
            }),
            ["1", "4", "7"]
        );
        let c = Domain::Continuous {
            min: 0.5,
            max: 1.5,
            template: "Exp(mean: {x})".into(),
            steps: 3,
        };
        assert_eq!(
            values(&c),
            ["Exp(mean: 0.5)", "Exp(mean: 1.0)", "Exp(mean: 1.5)"]
        );
        assert_eq!(values(&Domain::Choice(vec!["Fifo".into()])), ["Fifo"]);
    }

    #[test]
    fn grid_finds_the_staffing_trade_off() {
        // λ = 1, service mean 1.8: 2 servers are barely stable (ρ = 0.9, Lq ≈ 7.7),
        // 3 servers cut the queue to Lq ≈ 0.53; cost 1 per server, 10 per waiting item.
        // Objective: c=2 ≈ 79, c=3 ≈ 8.3, c=4 ≈ 5.6, c=5 ≈ 5.1 (Erlang C).
        let base = Model::parse(MODEL).unwrap();
        let r = optimize(&problem(), &base).unwrap();
        assert_eq!(r.evaluations.len(), 4);
        let best = &r.evaluations[r.best.unwrap()];
        assert!(
            ["4", "5"].contains(&best.point["maquina.capacity"].as_str()),
            "{:?}",
            best.point
        );
        assert!(
            r.evaluations[0].objective.mean > 40.0,
            "2 servidores ficam congestionados"
        );
        assert!(!r.explanation.is_empty());
    }

    #[test]
    fn progress_counts_points_and_can_cancel() {
        let base = Model::parse(MODEL).unwrap();
        let mut p = problem();
        p.replications = 2;
        assert_eq!(planned_evaluations(&p), 4);
        let mut seen = Vec::new();
        let r = optimize_with(&p, &base, &mut |done, planned| {
            seen.push((done, planned));
            true
        })
        .unwrap();
        assert_eq!(seen, [(1, 4), (2, 4), (3, 4), (4, 4)]);
        assert_eq!(r.evaluations.len(), 4);
        let stopped = optimize_with(&p, &base, &mut |done, _| done < 2);
        assert!(matches!(stopped, Err(ModelError::Cancelled)));
    }

    #[test]
    fn validation_catches_bad_problems() {
        let base = Model::parse(MODEL).unwrap();
        let mut p = problem();
        p.variables[0].key = "maquina.velocidade".into();
        p.replications = 1;
        let errors = p.validate(&base);
        assert!(
            errors.iter().any(|e| e.contains("velocidade")),
            "{errors:?}"
        );
        assert!(
            errors.iter().any(|e| e.contains("2 replicações")),
            "{errors:?}"
        );
    }
}
