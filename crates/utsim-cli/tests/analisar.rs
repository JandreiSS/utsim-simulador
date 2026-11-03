//! End-to-end: `utsim analisar` on the M/M/1 example, every analysis and the JSON.

use std::path::{Path, PathBuf};
use std::process::Command;

fn example() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/mm1.ron")
}

fn utsim(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
        .arg("analisar")
        .arg(example())
        .args(["-n", "4", "--stop", "20000"])
        .args(args)
        .output()
        .expect("runs utsim");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn every_analysis_prints_its_section_and_writes_json() {
    let dir = std::env::temp_dir().join(format!("utsim-analisar-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let json = dir.join("analise.json");
    let (ok, stdout, stderr) = utsim(&[
        "--aquecimento",
        "--precisao",
        "2%",
        "--lotes",
        "--little",
        "--gargalo-movel",
        "--confianca",
        "90%",
        "--json",
        json.to_str().unwrap(),
    ]);
    assert!(ok, "{stderr}");
    for part in [
        "confiança 90%",
        "== Aquecimento",
        "MSER-5: cortar",
        "== Precisão",
        "saida.time_in_system.mean",
        "== Médias em lotes (saida.time_in_system",
        "== Lei de Little",
        "(sistema)",
        "== Gargalo móvel",
        "gargalo principal: maquina",
    ] {
        assert!(stdout.contains(part), "faltou '{part}' em:\n{stdout}");
    }
    let value: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).unwrap()).unwrap();
    for key in [
        "aquecimento",
        "precisao",
        "lotes",
        "little",
        "gargalo_movel",
    ] {
        assert!(value.get(key).is_some(), "faltou {key} no JSON");
    }
    assert_eq!(value["level"], 0.9);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn bad_arguments_are_reported_in_portuguese() {
    let (ok, _, stderr) = utsim(&["--confianca", "150%"]);
    assert!(!ok);
    assert!(stderr.contains("nível de confiança inválido"), "{stderr}");
    let (ok, _, stderr) = utsim(&["--precisao", "x"]);
    assert!(!ok);
    assert!(stderr.contains("alvo de precisão inválido"), "{stderr}");
    let (ok, _, stderr) = utsim(&["--aquecimento", "--medida", "nada.x"]);
    assert!(!ok);
    assert!(stderr.contains("nada"), "{stderr}");
}
