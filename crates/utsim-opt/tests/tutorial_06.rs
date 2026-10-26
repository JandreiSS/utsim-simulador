//! Tutorial 06 do manual: a otimização de atendentes e sala de espera acha 5 atendentes
//! e 10 lugares (custo 100), como o texto diz.

use std::path::Path;

use utsim_model::Model;
use utsim_opt::{OptProblem, optimize};

#[test]
fn optimum_is_five_servers_and_ten_places() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/otimizacao_tutorial_06.ron");
    let (problem, base_dir) = OptProblem::load(&path).unwrap();
    let base = Model::load(base_dir.join(&problem.model)).unwrap();
    let result = optimize(&problem, &base).unwrap();
    assert_eq!(result.evaluations.len(), 24);
    let best = result.best_evaluation().expect("há um ponto viável");
    assert_eq!(best.point["atendentes.capacity"], "5");
    assert_eq!(best.point["fila.capacity"], "10");
    assert!((best.objective.mean - 100.0).abs() < 1e-9);
}
