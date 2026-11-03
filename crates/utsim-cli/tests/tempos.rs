//! End-to-end: `utsim tempos` on `examples/homem_maquina.ron` and its improved method.

use std::path::{Path, PathBuf};
use std::process::Command;

fn example(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

fn utsim(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
        .arg("tempos")
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
    let dir = std::env::temp_dir().join(format!("utsim-tempos-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn study_with_every_output_and_comparison() {
    let dir = temp_dir();
    let (html, svg, csv, json) = (
        dir.join("hm.html"),
        dir.join("hm.svg"),
        dir.join("hm.csv"),
        dir.join("hm.json"),
    );
    let s = |p: &PathBuf| p.to_string_lossy().into_owned();
    let (ok, out, err) = utsim(&[
        &example("homem_maquina.ron"),
        "--ritmo",
        "110",
        "--tolerancia",
        "15",
        "--custo-operador",
        "30",
        "--custo-maquina",
        "50",
        "--comparar",
        &example("homem_maquina_melhorado.ron"),
        "--html",
        &s(&html),
        "--svg",
        &s(&svg),
        "--csv",
        &s(&csv),
        "--json",
        &s(&json),
    ]);
    assert!(ok, "{err}");
    assert!(out.contains("== ana com maq1, maq2, maq3 =="), "{out}");
    assert!(out.contains("n' = (a + t)/(a + b)"));
    assert!(out.contains("ótimo pelo custo: n = 3"));
    assert!(out.contains("tolerância 15%"));
    assert!(out.contains("Comparação"));
    let page = std::fs::read_to_string(&html).unwrap();
    assert!(page.contains("Comparação de métodos") && page.contains("<svg"));
    assert!(std::fs::read_to_string(&svg).unwrap().starts_with("<?xml"));
    let table = std::fs::read_to_string(&csv).unwrap();
    assert!(table.contains("ana,carga/descarga maq1,carga_descarga"));
    let cmp = std::fs::read_to_string(dir.join("hm-comparacao.csv")).unwrap();
    assert!(cmp.starts_with("ator,elemento,antes,depois,diferenca,variacao"));
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).unwrap()).unwrap();
    assert_eq!(v["estudo"]["grupos"][0]["operador"], "ana");
    assert!(v["comparacao"]["elementos"].as_array().unwrap().len() > 5);
    assert_eq!(v["estudo"]["grupos"][0]["padrao"]["tolerancia"], 0.15);
}

#[test]
fn bad_arguments_are_explained() {
    let (ok, _, err) = utsim(&[&example("homem_maquina.ron"), "--ator", "ninguem"]);
    assert!(!ok && err.contains("ninguem"), "{err}");
    let (ok, _, err) = utsim(&[&example("homem_maquina.ron"), "--unidade", "semana"]);
    assert!(!ok && err.contains("unidade"), "{err}");
    let (ok, _, err) = utsim(&[&example("homem_maquina.ron"), "--custo-operador", "30"]);
    assert!(!ok && err.contains("custo-maquina"), "{err}");
}
