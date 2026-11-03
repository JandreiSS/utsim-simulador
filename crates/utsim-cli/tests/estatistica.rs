//! End-to-end: `utsim estatistica` on the example files of `examples/estatistica/`.

use std::path::{Path, PathBuf};
use std::process::Command;

fn example(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/estatistica")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

fn utsim(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
        .arg("estatistica")
        .args(args)
        .output()
        .expect("runs utsim");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn temp_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("utsim-estatistica-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn descriptive_summary_and_json() {
    let json = temp_dir().join("descritiva.json");
    let (ok, out, err) = utsim(&[
        "descritiva",
        &example("setup.csv"),
        "--coluna",
        "antes",
        "--json",
        json.to_str().unwrap(),
    ]);
    assert!(ok, "{err}");
    assert!(
        out.contains("n = 12") && out.contains("média 50.400000"),
        "{out}"
    );
    assert!(out.contains("IC 95.0% da média"), "{out}");
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).unwrap()).unwrap();
    assert_eq!(v["descritiva"]["n"], 12);
    assert!(v["histograma"]["counts"].is_array());
}

#[test]
fn t_tests_paired_and_one_sample() {
    let data = example("setup.csv");
    let (ok, out, err) = utsim(&[
        "teste-t",
        &data,
        "--coluna",
        "antes",
        "--coluna2",
        "depois",
        "--pareado",
    ]);
    assert!(ok, "{err}");
    assert!(
        out.contains("pareado") && out.contains("gl = 11.000000"),
        "{out}"
    );
    let (ok, out, _) = utsim(&[
        "teste-t",
        &data,
        "--coluna",
        "depois",
        "--mu",
        "40",
        "--alternativa",
        "menor",
    ]);
    assert!(ok);
    assert!(out.contains("H₁: μ < 40"), "{out}");
    let (ok, _, err) = utsim(&["teste-t", &data, "--alternativa", "talvez"]);
    assert!(!ok && err.contains("alternativa"), "{err}");
}

#[test]
fn chi_square_proportions_and_rank_tests() {
    let (ok, out, err) = utsim(&["qui-quadrado", &example("turnos.csv")]);
    assert!(ok, "{err}");
    assert!(out.contains("3 × 2") && out.contains("gl = 2"), "{out}");
    let (ok, out, _) = utsim(&[
        "proporcao",
        "--x",
        "27",
        "--n",
        "100",
        "--x2",
        "19",
        "--n2",
        "100",
        "--fisher",
    ]);
    assert!(ok);
    assert!(out.contains("z = 1.344") && out.contains("Fisher"), "{out}");
    let data = example("setup.csv");
    let (ok, out, _) = utsim(&["mann-whitney", &data]);
    assert!(ok);
    assert!(out.contains("U₁ = 144") && out.contains("exato"), "{out}");
    let (ok, out, _) = utsim(&["wilcoxon", &data, "--coluna2", "depois"]);
    assert!(ok);
    assert!(out.contains("W⁺ = 78"), "{out}");
    let (ok, out, _) = utsim(&["anova", &data]);
    assert!(ok);
    assert!(out.contains("Tukey"), "{out}");
}
