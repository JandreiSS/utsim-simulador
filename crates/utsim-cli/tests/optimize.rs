//! End-to-end: `utsim otimizar` on the example problem with a small budget.

use std::path::{Path, PathBuf};
use std::process::Command;

fn example() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/otimizacao_servidores.ron")
}

#[test]
fn optimize_example_prints_report_and_writes_files() {
    let dir = std::env::temp_dir().join(format!("utsim-otimizar-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let html = dir.join("relatorio.html");
    let json = dir.join("saida.json");
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
        .arg("otimizar")
        .arg(example())
        .args(["--reps", "3", "--orcamento", "4"])
        .arg("--html")
        .arg(&html)
        .arg("--json")
        .arg(&json)
        .output()
        .expect("runs utsim");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    for part in [
        "Melhor ponto",
        "Pontos refinados",
        "set: {",
        "Explicação",
        "4 ponto(s) avaliado(s)",
    ] {
        assert!(stdout.contains(part), "faltou '{part}' em:\n{stdout}");
    }
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("[4/4]"), "{stderr}");

    let html_text = std::fs::read_to_string(&html).unwrap();
    assert!(html_text.contains("<svg") && html_text.contains("Pontos refinados"));
    let value: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).unwrap()).unwrap();
    assert_eq!(value["evaluations"].as_array().unwrap().len(), 4);
    assert!(!value["refined"].as_array().unwrap().is_empty());
    assert!(stdout.contains("Pontos refinados") && stdout.contains("Pontos da busca"));
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn optimize_reports_invalid_problem_in_portuguese() {
    let dir = std::env::temp_dir().join(format!("utsim-otimizar-err-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let text = std::fs::read_to_string(example()).unwrap().replace(
        "\"fila.capacity\",    domain",
        "\"fila.velocidade\", domain",
    );
    let file = dir.join("ruim.ron");
    std::fs::write(&file, text).unwrap();
    std::fs::copy(
        example().with_file_name("servidores.ron"),
        dir.join("servidores.ron"),
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
        .arg("otimizar")
        .arg(&file)
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("velocidade"));
    std::fs::remove_dir_all(dir).ok();
}
