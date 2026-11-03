//! End-to-end: `utsim qualidade` on the example files of `examples/qualidade/`.

use std::path::{Path, PathBuf};
use std::process::Command;

fn example(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/qualidade")
        .join(name)
}

fn utsim(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
        .arg("qualidade")
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
    let dir = std::env::temp_dir().join(format!("utsim-qualidade-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn chart_phase_one_then_phase_two() {
    let dir = temp_dir();
    let data = example("diametros.csv");
    let limits = dir.join("limites.json");
    let html = dir.join("carta.html");
    let (ok, out, err) = utsim(&[
        "carta",
        data.to_str().unwrap(),
        "--tipo",
        "xbarr",
        "--regras",
        "we",
        "--excluir",
        "21,22,23,24,25",
        "--salvar-limites",
        limits.to_str().unwrap(),
        "--html",
        html.to_str().unwrap(),
    ]);
    assert!(ok, "{err}");
    assert!(out.contains("fase I"), "{out}");
    let page = std::fs::read_to_string(&html).unwrap();
    assert!(page.starts_with("<!doctype html>") && page.contains("<svg"));
    // Phase II with the frozen limits: the shifted subgroups 21-25 signal.
    let (ok, out, err) = utsim(&[
        "carta",
        data.to_str().unwrap(),
        "--tipo",
        "xbarr",
        "--limites",
        limits.to_str().unwrap(),
    ]);
    assert!(ok, "{err}");
    assert!(
        out.contains("fase II") && out.contains("subgrupo   2"),
        "{out}"
    );
}

#[test]
fn attribute_and_memory_charts() {
    let (ok, out, err) = utsim(&[
        "carta",
        example("defeituosos.csv").to_str().unwrap(),
        "--tipo",
        "p",
    ]);
    assert!(ok, "{err}");
    assert!(out.contains("limites variáveis"), "{out}");
    for tipo in ["ewma", "cusum"] {
        let (ok, out, err) = utsim(&[
            "carta",
            example("diametros.csv").to_str().unwrap(),
            "--tipo",
            tipo,
        ]);
        assert!(ok, "{err}");
        assert!(out.to_uppercase().contains(&tipo.to_uppercase()), "{out}");
    }
    let (ok, _, err) = utsim(&["carta", "nao_existe.csv"]);
    assert!(!ok && err.contains("não foi possível ler"), "{err}");
}

#[test]
fn capability_with_every_method() {
    let dir = temp_dir();
    let html = dir.join("capacidade.html");
    let json = dir.join("capacidade.json");
    let (ok, out, err) = utsim(&[
        "capacidade",
        example("lead_time.csv").to_str().unwrap(),
        "--lse",
        "18",
        "--metodo",
        "todos",
        "--arredondar",
        "--html",
        html.to_str().unwrap(),
        "--json",
        json.to_str().unwrap(),
    ]);
    assert!(ok, "{err}");
    for s in ["Cpk", "Box-Cox", "Método dos percentis", "Nível sigma"] {
        assert!(out.contains(s), "{s}: {out}");
    }
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).unwrap()).unwrap();
    assert!(v["normal"]["ppk"]["value"].as_f64().unwrap() > 0.0);
    assert!(v["box_cox"]["fit"]["lambda"].is_number());
    let (ok, _, err) = utsim(&["capacidade", example("lead_time.csv").to_str().unwrap()]);
    assert!(!ok && err.contains("limite de especificação"), "{err}");
}

#[test]
fn measurement_system_studies() {
    let (ok, out, err) = utsim(&[
        "msa",
        example("grr.csv").to_str().unwrap(),
        "--tolerancia",
        "6",
    ]);
    assert!(ok, "{err}");
    assert!(
        out.contains("ANOVA") && out.contains("médias e amplitudes"),
        "{out}"
    );
    assert!(out.contains("ndc"), "{out}");
    let (ok, out, err) = utsim(&[
        "msa",
        example("atributos.csv").to_str().unwrap(),
        "--tipo",
        "atributos",
    ]);
    assert!(ok, "{err}");
    assert!(out.contains("kappa de Fleiss"), "{out}");
    let (ok, out, err) = utsim(&[
        "msa",
        example("linearidade.csv").to_str().unwrap(),
        "--tipo",
        "linearidade",
    ]);
    assert!(ok, "{err}");
    assert!(out.contains("%linearidade"), "{out}");
}

#[test]
fn arl_by_simulation() {
    let (ok, out, err) = utsim(&["arl", "--corridas", "2000"]);
    assert!(ok, "{err}");
    assert!(out.contains("fórmula fechada: 370"), "{out}");
}
