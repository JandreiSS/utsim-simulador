//! Search algorithms against problems whose optimum Grid establishes.

use utsim_model::Model;
use utsim_opt::{OptProblem, OptResult, Point, optimize};

/// Servers × buffer with arrivals lost when the buffer is full.
const LOSS_MODEL: &str = r#"Model(
    name: "perda",
    run: (stop_time: 4000.0, warmup: 400.0, seed: 11),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0), on_blocked: Drop)),
        (name: "fila", kind: Queue(capacity: 4)),
        (name: "maquina", kind: Processor(capacity: 1, process: Exp(mean: 1.8))),
        (name: "saida", kind: Sink),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#;

/// Staffing: queue length vs. server cost (as in the unit test of the crate).
const STAFF_MODEL: &str = r#"Model(
    name: "mmc",
    run: (stop_time: 6000.0, warmup: 500.0, seed: 3),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0))),
        (name: "fila", kind: Queue()),
        (name: "maquina", kind: Processor(capacity: 1, process: Exp(mean: 1.8))),
        (name: "saida", kind: Sink),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#;

fn loss_problem(algorithm: &str, budget: usize, extra: &str) -> OptProblem {
    OptProblem::parse(&format!(
        r#"OptProblem(
            name: "perda",
            model: "perda.ron",
            replications: 4,
            variables: [
                (key: "maquina.capacity", domain: Integer(min: 1, max: 6)),
                (key: "fila.capacity", domain: Integer(min: 1, max: 9)),
            ],
            objective: (
                terms: [("saida.input", -0.05)],
                costs: [("maquina.capacity", 40.0), ("fila.capacity", 3.0)],
            ),
            algorithm: {algorithm},
            budget: {budget},
            seed: 7,
            {extra}
        )"#
    ))
    .unwrap()
}

fn best_point(r: &OptResult) -> Point {
    r.best_evaluation().expect("a best point").point.clone()
}

fn assert_distinct(r: &OptResult) {
    let mut seen = std::collections::BTreeSet::new();
    for e in &r.evaluations {
        assert!(seen.insert(e.point.clone()), "duplicate {:?}", e.point);
    }
}

#[test]
fn metaheuristics_match_grid_on_two_variables() {
    let base = Model::parse(LOSS_MODEL).unwrap();
    let grid = optimize(&loss_problem("Grid", 200, ""), &base).unwrap();
    assert_eq!(grid.evaluations.len(), 54);
    let want = best_point(&grid);

    let sa = optimize(&loss_problem("Annealing(iterations: 60)", 24, ""), &base).unwrap();
    assert_eq!(best_point(&sa), want, "annealing");
    assert!(sa.evaluations.len() <= 24);
    assert_distinct(&sa);

    let ga = optimize(
        &loss_problem("Genetic(population: 6, generations: 8)", 30, ""),
        &base,
    )
    .unwrap();
    assert_eq!(best_point(&ga), want, "genetic");
    assert!(ga.evaluations.len() <= 30);
    assert_distinct(&ga);
}

#[test]
fn searches_are_deterministic() {
    let base = Model::parse(LOSS_MODEL).unwrap();
    for alg in [
        "Annealing(iterations: 40)",
        "Genetic(population: 5, generations: 6)",
    ] {
        let a = optimize(&loss_problem(alg, 20, ""), &base).unwrap();
        let b = optimize(&loss_problem(alg, 20, ""), &base).unwrap();
        let pts = |r: &OptResult| {
            r.evaluations
                .iter()
                .map(|e| e.point.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(pts(&a), pts(&b), "{alg}");
        assert_eq!(a.history, b.history, "{alg}");
        assert_eq!(a.best, b.best);
    }
}

#[test]
fn budget_is_respected_and_history_is_monotone() {
    let base = Model::parse(LOSS_MODEL).unwrap();
    for alg in [
        "Grid",
        "Annealing(iterations: 100)",
        "Genetic(population: 8, generations: 20)",
    ] {
        let r = optimize(&loss_problem(alg, 7, ""), &base).unwrap();
        assert!(r.evaluations.len() <= 7, "{alg}: {}", r.evaluations.len());
        assert!(!r.history.is_empty());
        assert!(
            r.history
                .windows(2)
                .all(|w| w[0].0 < w[1].0 && w[1].1 <= w[0].1)
        );
        assert!(r.history.last().unwrap().0 <= 7);
    }
}

#[test]
fn staffing_problem_with_infeasible_points() {
    // The constraint (mean queue length ≤ 3) makes 2 servers infeasible; the penalty
    // must steer both heuristics to a feasible best, same as Grid.
    let base = Model::parse(STAFF_MODEL).unwrap();
    let make = |alg: &str| {
        OptProblem::parse(&format!(
            r#"OptProblem(
                name: "servidores",
                model: "mmc.ron",
                replications: 4,
                variables: [(key: "maquina.capacity", domain: Integer(min: 2, max: 9))],
                objective: (terms: [("fila.content.mean", 10.0)], costs: [("maquina.capacity", 2.0)]),
                constraints: [(metric: "fila.content.mean", max: 3.0)],
                algorithm: {alg},
                budget: 5,
                seed: 3,
            )"#
        ))
        .unwrap()
    };
    let grid = optimize(&make("Grid"), &base).unwrap();
    let want = best_point(&grid);
    for alg in [
        "Annealing(iterations: 30)",
        "Genetic(population: 3, generations: 4)",
    ] {
        let r = optimize(&make(alg), &base).unwrap();
        assert!(r.best_evaluation().unwrap().feasible, "{alg}");
        for (i, e) in r.evaluations.iter().enumerate() {
            if !e.feasible {
                assert_ne!(r.best, Some(i), "{alg}: infeasible point reported as best");
            }
        }
        assert!(r.evaluations.len() <= 5);
        // The grid with the same budget covers 2..=6 servers; the optimum is among them.
        assert_eq!(best_point(&r), want, "{alg}");
    }
}

#[test]
fn refinement_re_simulates_the_best_points() {
    let base = Model::parse(LOSS_MODEL).unwrap();
    let r = optimize(
        &loss_problem(
            "Annealing(iterations: 40)",
            20,
            "refine: Some(Refine(top: 3, replications: 8)),",
        ),
        &base,
    )
    .unwrap();
    assert_eq!(r.refined.len(), 3);
    assert!(r.refined.iter().all(|e| e.per_replication.len() == 8));
    assert!(r.evaluations.iter().all(|e| e.per_replication.len() == 4));
    let best = r.best_evaluation().unwrap();
    assert_eq!(best.per_replication.len(), 8);
    // Same seeds: the first four refined replications equal the search ones.
    let search = r
        .evaluations
        .iter()
        .find(|e| e.point == best.point)
        .unwrap();
    assert_eq!(&best.per_replication[..4], &search.per_replication[..]);
    assert!(r.explanation[0].contains("Refinamento"));
}
