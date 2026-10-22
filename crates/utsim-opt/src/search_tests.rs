//! Unit tests of the search algorithms on a synthetic landscape (no simulation).

use std::cell::Cell;

use utsim_core::stats::Summary;
use utsim_model::ModelError;

use crate::search::run_with;
use crate::{Algorithm, Constraint, Evaluation, OptProblem, Point, compare};

fn problem(algorithm: &str, budget: usize) -> OptProblem {
    let mut p = OptProblem::parse(&format!(
        r#"OptProblem(name: "s", model: "m.ron", variables: [
            (key: "x", domain: Integer(min: 0, max: 12)),
            (key: "y", domain: Integer(min: 0, max: 8)),
        ], objective: (terms: [("m", 1.0)]), algorithm: {algorithm}, budget: {budget}, seed: 5)"#
    ))
    .unwrap();
    p.constraints = vec![Constraint {
        metric: "c".into(),
        min: None,
        max: Some(11.0),
    }];
    p
}

/// Objective (x-6)² + (y-3)², with a deceptive infeasible region (x + y > 11) where
/// the raw objective is much lower. The feasible optimum is (6, 3) with value 0.
fn fake(point: &Point, calls: &Cell<usize>) -> Result<Evaluation, ModelError> {
    calls.set(calls.get() + 1);
    let x: f64 = point["x"].parse().unwrap();
    let y: f64 = point["y"].parse().unwrap();
    let obj = (x - 6.0).powi(2) + (y - 3.0).powi(2) - 5.0 * (x + y - 11.0).max(0.0);
    let feasible = x + y <= 11.0;
    Ok(Evaluation {
        point: point.clone(),
        per_replication: vec![obj, obj],
        objective: Summary::from_samples(&[obj, obj]),
        constraints: vec![("c".into(), Summary::from_samples(&[x + y, x + y]), feasible)],
        feasible,
    })
}

const ALGORITHMS: [&str; 2] = [
    "Annealing(iterations: 300)",
    "Genetic(population: 10, generations: 30)",
];

#[test]
fn cache_means_one_simulation_per_distinct_point() {
    for alg in ALGORITHMS {
        let p = problem(alg, 40);
        let calls = Cell::new(0);
        let (evals, history) = run_with(&p, |pt| fake(pt, &calls)).unwrap();
        assert_eq!(calls.get(), evals.len(), "{alg}");
        let distinct: std::collections::BTreeSet<_> =
            evals.iter().map(|e| e.point.clone()).collect();
        assert_eq!(distinct.len(), evals.len(), "{alg}");
        assert!(evals.len() <= 40, "{alg}");
        assert!(history.len() <= evals.len());
        let r = compare(&p, evals);
        assert!(r.evaluations[r.best.unwrap()].feasible, "{alg}");
    }
}

#[test]
fn heuristics_find_the_feasible_optimum_below_the_grid_size() {
    // 13 × 9 = 117 points in the grid; the budget is half of it.
    for alg in ALGORITHMS {
        let p = problem(alg, 58);
        let calls = Cell::new(0);
        let (evals, _) = run_with(&p, |pt| fake(pt, &calls)).unwrap();
        let r = compare(&p, evals);
        let best = &r.evaluations[r.best.unwrap()];
        assert_eq!(best.objective.mean, 0.0, "{alg}: {:?}", best.point);
    }
}

#[test]
fn zero_budget_and_degenerate_domains_do_not_panic() {
    for alg in ALGORITHMS {
        let (evals, history) = run_with(&problem(alg, 0), |pt| fake(pt, &Cell::new(0))).unwrap();
        assert!(evals.is_empty() && history.is_empty());
    }
    let mut p = problem("Annealing(iterations: 10)", 5);
    p.variables.truncate(1);
    p.variables[0].domain = crate::Domain::Integer {
        min: 3,
        max: 3,
        step: 1,
    };
    let (evals, _) = run_with(&p, |pt| {
        let mut pt = pt.clone();
        pt.insert("y".into(), "0".into());
        fake(&pt, &Cell::new(0))
    })
    .unwrap();
    assert_eq!(evals.len(), 1);
    let _ = Algorithm::Grid;
}
