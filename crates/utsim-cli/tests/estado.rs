//! End-to-end: `utsim run --estado` and `--salvar-estado`, chaining two runs.

use std::path::{Path, PathBuf};
use std::process::Command;

fn examples() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples")
}

fn utsim(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
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
fn run_from_the_shipped_state_and_chain_a_saved_one() {
    let model = examples().join("turnos.ron");
    let state = examples().join("estados/turnos_21h30.ron");
    let model = model.to_str().unwrap();
    let (ok, stdout, stderr) = utsim(&["run", model, "--estado", state.to_str().unwrap()]);
    assert!(ok, "{stderr}");
    assert!(stdout.contains("relógio 930"), "{stdout}");
    // count sem idade gera um aviso.
    assert!(stderr.contains("aviso: fila:"), "{stderr}");

    let dir = std::env::temp_dir().join(format!("utsim-estado-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let saved = dir.join("t1000.ron");
    let saved = saved.to_str().unwrap();
    let (ok, stdout, stderr) = utsim(&[
        "run",
        model,
        "-n",
        "1",
        "--stop",
        "2000",
        "--salvar-estado",
        "1000",
        saved,
    ]);
    assert!(ok, "{stderr}");
    assert!(stdout.contains("gravado em"), "{stdout}");
    let text = std::fs::read_to_string(saved).unwrap();
    assert!(text.starts_with("InitialState("), "{text}");
    assert!(text.contains("clock: 1000.0"), "{text}");

    // Retoma do estado gravado e grava de novo: o relógio soma.
    let again = dir.join("t1500.ron");
    let (ok, _, stderr) = utsim(&[
        "run",
        model,
        "-n",
        "2",
        "--stop",
        "1000",
        "--estado",
        saved,
        "--salvar-estado",
        "500",
        again.to_str().unwrap(),
    ]);
    assert!(ok, "{stderr}");
    let text = std::fs::read_to_string(&again).unwrap();
    assert!(text.contains("clock: 1500.0"), "{text}");

    // Um estado incompatível é recusado com a mensagem do problema.
    let bad = dir.join("ruim.ron");
    std::fs::write(&bad, "InitialState(objects: {\"fila\": (count: 99)})").unwrap();
    let (ok, _, stderr) = utsim(&["run", model, "--estado", bad.to_str().unwrap()]);
    assert!(!ok);
    assert!(stderr.contains("capacidade é 20"), "{stderr}");
    std::fs::remove_dir_all(&dir).ok();
}
