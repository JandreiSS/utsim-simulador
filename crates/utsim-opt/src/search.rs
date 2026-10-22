//! Search algorithms: grid, simulated annealing, genetic algorithm, and the final
//! refinement step.
//!
//! All of them go through an [`Evaluator`], which caches evaluations by point (a
//! point is never simulated twice in a run), counts the **distinct** simulated
//! points against `budget`, and records the convergence history.
//!
//! # Handling of infeasible points
//!
//! The metaheuristics work with a *score* to be minimised (`sign · objective mean`,
//! where `sign = -1` for maximisation). An infeasible point gets
//! `score = obj + M · (1 + v)`, where `v` is the total relative violation of the
//! constraints (`distance past the bound / max(|bound|, 1)`) and
//! `M = 10 · max|obj| + 1` over every point evaluated so far. So an infeasible
//! point is always worse than any feasible point already seen, and among infeasible
//! points the one closer to feasibility is better, which lets the search walk back
//! into the feasible region. Infeasible points are never reported as best
//! ([`crate::compare`] only considers feasible ones).
//!
//! # Simulated annealing
//!
//! Starts at the middle value of every domain. A neighbour changes one variable
//! (chosen uniformly): integers and continuous variables move to the adjacent value
//! (±step, ±one grid step of `steps`), choices jump to another option uniformly.
//! Acceptance is Metropolis: always if the score does not worsen, otherwise with
//! probability `exp(-Δ/T)`. The temperature is geometric, `T_k = T0 · α^k`, ending
//! at `T0/1000` after `iterations` steps. `T0` is the mean absolute score
//! difference between the start and up to 5 initial neighbours (so an average-sized
//! worsening is accepted with probability `e⁻¹` at the beginning); if those are all
//! equal, `T0 = max(1, 5% of |score|)`. The probes are ordinary evaluations (cached,
//! counted in the budget).
//!
//! # Genetic algorithm
//!
//! Genes are indices into each variable's value list. The first individual is the
//! middle point, the others are random. Each generation keeps the best individual
//! (elitism of 1) and fills the rest with children: two parents by binary
//! tournament (size 2), uniform crossover, and per-gene mutation (probability
//! `1/n_vars`, resample uniformly). `generations` is the number of breeding rounds
//! after the initial population.

use std::collections::BTreeMap;

use rand::Rng;
use utsim_core::rng::{SimRng, stream};
use utsim_model::{Model, ModelError};

use crate::{
    Algorithm, Evaluation, OptProblem, OptResult, Point, Refine, Sense, compare, evaluate,
    evaluate_with, values,
};

/// Evaluations in order, and the convergence history.
pub(crate) type SearchOutput = (Vec<Evaluation>, Vec<(usize, f64)>);

/// Caching, budget-limited evaluation of points.
pub(crate) struct Evaluator<'a, F> {
    problem: &'a OptProblem,
    sim: F,
    cache: BTreeMap<Point, usize>,
    evals: Vec<Evaluation>,
    history: Vec<(usize, f64)>,
    /// Best feasible objective mean so far (in the problem's own sense).
    best: Option<f64>,
}

impl<'a, F> Evaluator<'a, F>
where
    F: FnMut(&Point) -> Result<Evaluation, ModelError>,
{
    pub(crate) fn new(problem: &'a OptProblem, sim: F) -> Self {
        Evaluator {
            problem,
            sim,
            cache: BTreeMap::new(),
            evals: Vec::new(),
            history: Vec::new(),
            best: None,
        }
    }

    pub(crate) fn exhausted(&self) -> bool {
        self.evals.len() >= self.problem.budget
    }

    fn sign(&self) -> f64 {
        match self.problem.objective.sense {
            Sense::Minimize => 1.0,
            Sense::Maximize => -1.0,
        }
    }

    /// Index of the evaluation of `point`, simulating it if new. `None` when the
    /// point is new and the budget is spent.
    pub(crate) fn eval(&mut self, point: &Point) -> Result<Option<usize>, ModelError> {
        if let Some(&i) = self.cache.get(point) {
            return Ok(Some(i));
        }
        if self.exhausted() {
            return Ok(None);
        }
        let e = (self.sim)(point)?;
        if e.feasible {
            let m = e.objective.mean;
            let better = match self.best {
                None => true,
                Some(b) => self.sign() * m < self.sign() * b,
            };
            if better {
                self.best = Some(m);
            }
        }
        self.evals.push(e);
        let i = self.evals.len() - 1;
        self.cache.insert(point.clone(), i);
        if let Some(b) = self.best {
            self.history.push((self.evals.len(), b));
        }
        Ok(Some(i))
    }

    /// Penalised score to minimise (see the module docs).
    pub(crate) fn score(&self, index: usize) -> f64 {
        let e = &self.evals[index];
        let obj = self.sign() * e.objective.mean;
        if e.feasible {
            return obj;
        }
        let max_abs = self
            .evals
            .iter()
            .map(|x| (self.sign() * x.objective.mean).abs())
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max);
        let big_m = 10.0 * max_abs + 1.0;
        let mut violation = 0.0;
        for (c, (_, s, ok)) in self.problem.constraints.iter().zip(&e.constraints) {
            if *ok {
                continue;
            }
            if let Some(min) = c.min.filter(|m| s.mean < *m) {
                violation += (min - s.mean) / min.abs().max(1.0);
            }
            if let Some(max) = c.max.filter(|m| s.mean > *m) {
                violation += (s.mean - max) / max.abs().max(1.0);
            }
        }
        obj + big_m * (1.0 + violation)
    }

    pub(crate) fn finish(self) -> (Vec<Evaluation>, Vec<(usize, f64)>) {
        (self.evals, self.history)
    }
}

/// Runs the configured algorithm against the real simulator.
pub(crate) fn run(problem: &OptProblem, base: &Model) -> Result<SearchOutput, ModelError> {
    let sim = |p: &Point| evaluate(base, problem, p);
    run_with(problem, sim)
}

pub(crate) fn run_with<F>(problem: &OptProblem, sim: F) -> Result<SearchOutput, ModelError>
where
    F: FnMut(&Point) -> Result<Evaluation, ModelError>,
{
    let mut ev = Evaluator::new(problem, sim);
    match &problem.algorithm {
        Algorithm::Grid => grid_search(problem, &mut ev)?,
        Algorithm::Annealing { iterations } => annealing(problem, &mut ev, *iterations)?,
        Algorithm::Genetic {
            population,
            generations,
        } => genetic(problem, &mut ev, *population, *generations)?,
    }
    Ok(ev.finish())
}

/// Exhaustive search, up to `budget` points (lexicographic order of the variables).
#[allow(dead_code)]
pub(crate) fn grid(problem: &OptProblem, base: &Model) -> Result<SearchOutput, ModelError> {
    let mut p = problem.clone();
    p.algorithm = Algorithm::Grid;
    run(&p, base)
}

fn domains(problem: &OptProblem) -> Vec<Vec<String>> {
    problem
        .variables
        .iter()
        .map(|v| values(&v.domain))
        .collect()
}

fn point_of(problem: &OptProblem, vals: &[Vec<String>], genes: &[usize]) -> Point {
    problem
        .variables
        .iter()
        .zip(vals.iter().zip(genes))
        .map(|(v, (vs, &g))| (v.key.clone(), vs[g].clone()))
        .collect()
}

fn grid_search<F>(problem: &OptProblem, ev: &mut Evaluator<'_, F>) -> Result<(), ModelError>
where
    F: FnMut(&Point) -> Result<Evaluation, ModelError>,
{
    let vals = domains(problem);
    let total: usize = vals.iter().map(Vec::len).product();
    for index in 0..total.min(problem.budget) {
        let mut rest = index;
        let mut genes = vec![0; vals.len()];
        for (g, vs) in genes.iter_mut().zip(&vals).rev() {
            *g = rest % vs.len();
            rest /= vs.len();
        }
        ev.eval(&point_of(problem, &vals, &genes))?;
    }
    Ok(())
}

fn middle(vals: &[Vec<String>]) -> Vec<usize> {
    vals.iter().map(|vs| (vs.len() - 1) / 2).collect()
}

fn is_choice(problem: &OptProblem, var: usize) -> bool {
    matches!(problem.variables[var].domain, crate::Domain::Choice(_))
}

fn neighbour(
    problem: &OptProblem,
    vals: &[Vec<String>],
    genes: &[usize],
    rng: &mut SimRng,
) -> Vec<usize> {
    let movable: Vec<usize> = (0..vals.len()).filter(|&i| vals[i].len() > 1).collect();
    let mut next = genes.to_vec();
    if movable.is_empty() {
        return next;
    }
    let var = movable[rng.random_range(0..movable.len())];
    let n = vals[var].len();
    if is_choice(problem, var) {
        let mut j = rng.random_range(0..n - 1);
        if j >= genes[var] {
            j += 1;
        }
        next[var] = j;
    } else {
        let up = rng.random_bool(0.5);
        let g = genes[var];
        next[var] = if (up && g + 1 < n) || g == 0 {
            g + 1
        } else {
            g - 1
        };
    }
    next
}

fn annealing<F>(
    problem: &OptProblem,
    ev: &mut Evaluator<'_, F>,
    iterations: usize,
) -> Result<(), ModelError>
where
    F: FnMut(&Point) -> Result<Evaluation, ModelError>,
{
    let vals = domains(problem);
    let mut rng = stream(problem.seed, 0, 0x4f50_5401);
    let mut current = middle(&vals);
    let Some(start) = ev.eval(&point_of(problem, &vals, &current))? else {
        return Ok(());
    };

    // Initial temperature from the spread of a few neighbours of the start.
    let mut diffs = Vec::new();
    for _ in 0..iterations.min(5) {
        let nb = neighbour(problem, &vals, &current, &mut rng);
        let Some(i) = ev.eval(&point_of(problem, &vals, &nb))? else {
            break;
        };
        let d = (ev.score(i) - ev.score(start)).abs();
        if d.is_finite() {
            diffs.push(d);
        }
    }
    let mean_diff = if diffs.is_empty() {
        0.0
    } else {
        diffs.iter().sum::<f64>() / diffs.len() as f64
    };
    let mut temp = if mean_diff > 0.0 {
        mean_diff
    } else {
        (0.05 * ev.score(start).abs()).max(1.0)
    };
    let cooling = (1e-3_f64).powf(1.0 / iterations.max(1) as f64);

    let mut current_eval = start;
    for _ in 0..iterations {
        let nb = neighbour(problem, &vals, &current, &mut rng);
        let Some(i) = ev.eval(&point_of(problem, &vals, &nb))? else {
            break;
        };
        let delta = ev.score(i) - ev.score(current_eval);
        let u: f64 = rng.random();
        if delta <= 0.0 || u < (-delta / temp).exp() {
            current = nb;
            current_eval = i;
        }
        temp *= cooling;
    }
    Ok(())
}

fn genetic<F>(
    problem: &OptProblem,
    ev: &mut Evaluator<'_, F>,
    population: usize,
    generations: usize,
) -> Result<(), ModelError>
where
    F: FnMut(&Point) -> Result<Evaluation, ModelError>,
{
    let vals = domains(problem);
    let n_vars = vals.len();
    let size = population.max(2);
    let mut rng = stream(problem.seed, 0, 0x4741_0001);
    let p_mut = 1.0 / n_vars as f64;

    let mut pop: Vec<Vec<usize>> = vec![middle(&vals)];
    while pop.len() < size {
        pop.push(
            vals.iter()
                .map(|vs| rng.random_range(0..vs.len()))
                .collect(),
        );
    }

    for generation in 0..=generations {
        // Evaluate (cache makes repeated individuals free).
        let mut scored: Vec<(f64, Vec<usize>)> = Vec::with_capacity(size);
        for genes in &pop {
            let Some(i) = ev.eval(&point_of(problem, &vals, genes))? else {
                return Ok(());
            };
            scored.push((ev.score(i), genes.clone()));
        }
        if generation == generations {
            break;
        }
        let elite = scored
            .iter()
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|s| s.1.clone())
            .unwrap_or_default();
        let tournament = |rng: &mut SimRng| -> usize {
            let a = rng.random_range(0..scored.len());
            let b = rng.random_range(0..scored.len());
            if scored[b].0 < scored[a].0 { b } else { a }
        };
        let mut next = vec![elite];
        while next.len() < size {
            let pa = tournament(&mut rng);
            let pb = tournament(&mut rng);
            let child: Vec<usize> = (0..n_vars)
                .map(|g| {
                    let gene = if rng.random_bool(0.5) {
                        scored[pa].1[g]
                    } else {
                        scored[pb].1[g]
                    };
                    if rng.random_bool(p_mut) {
                        rng.random_range(0..vals[g].len())
                    } else {
                        gene
                    }
                })
                .collect();
            next.push(child);
        }
        pop = next;
    }
    Ok(())
}

/// Re-simulates the `top` best distinct feasible points with `refine.replications`
/// replications (same seeds for all) and makes the refined comparison the final one.
pub(crate) fn apply_refinement(
    problem: &OptProblem,
    base: &Model,
    refine: &Refine,
    result: &mut OptResult,
) -> Result<(), ModelError> {
    let sign = match problem.objective.sense {
        Sense::Minimize => 1.0,
        Sense::Maximize => -1.0,
    };
    let mut order: Vec<usize> = (0..result.evaluations.len())
        .filter(|&i| result.evaluations[i].feasible)
        .collect();
    order.sort_by(|&a, &b| {
        let (x, y) = (&result.evaluations[a], &result.evaluations[b]);
        (sign * x.objective.mean).total_cmp(&(sign * y.objective.mean))
    });
    order.truncate(refine.top);
    if order.is_empty() {
        return Ok(());
    }
    let refined = order
        .iter()
        .map(|&i| {
            evaluate_with(
                base,
                problem,
                &result.evaluations[i].point,
                refine.replications,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let n = refined.len();
    let compared = compare(problem, refined);
    result.refined = compared.evaluations;
    result.best = compared.best;
    result.equivalent = compared.equivalent;
    let mut explanation = vec![format!(
        "Refinamento: os {n} melhores pontos da busca foram simulados de novo com {} \
         replicações (mesmas sementes para todos).",
        refine.replications
    )];
    explanation.extend(compared.explanation);
    result.explanation = explanation;
    Ok(())
}
