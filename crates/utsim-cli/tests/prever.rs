//! End-to-end: `utsim prever` from the shipped state, with scenarios and a goal; the
//! forecast repeats `utsim run --estado` with `--stop` at the horizon.

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
fn forecast_from_the_shipped_state_with_scenarios() {
    let model = examples().join("turnos.ron");
    let state = examples().join("estados/turnos_21h30.ron");
    let scenarios = examples().join("estados/turnos_cenarios.ron");
    let dir = std::env::temp_dir().join(format!("utsim-prever-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let out = dir.join("previsao.json");
    let run_out = dir.join("run.json");
    let (ok, stdout, stderr) = utsim(&[
        "prever",
        model.to_str().unwrap(),
        "--estado",
        state.to_str().unwrap(),
        "--horizonte",
        "2h",
        "--unidade",
        "min",
        "-n",
        "6",
        "--cenarios",
        scenarios.to_str().unwrap(),
        "--meta",
        "producao>=5",
        "--json",
        out.to_str().unwrap(),
    ]);
    assert!(ok, "{stderr}");
    assert!(stdout.contains("Produção no horizonte"), "{stdout}");
    assert!(stdout.contains("E se"), "{stdout}");
    assert!(stdout.contains("montagem mais rápida"), "{stdout}");
    let f: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&out).unwrap()).unwrap();
    assert_eq!(f["horizonte"], 120.0);
    assert_eq!(f["relogio"], 930.0);
    assert_eq!(f["cenarios"].as_array().unwrap().len(), 2);
    let producao = &f["kpis"][0];
    assert_eq!(producao["id"], "producao");
    assert!(producao["prob"]["n"] == 6);

    // The same as `utsim run --estado ... --stop 120`, replication by replication.
    let (ok, _, stderr) = utsim(&[
        "run",
        model.to_str().unwrap(),
        "--estado",
        state.to_str().unwrap(),
        "--stop",
        "120",
        "-n",
        "6",
        "--json",
        run_out.to_str().unwrap(),
    ]);
    assert!(ok, "{stderr}");
    let r: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&run_out).unwrap()).unwrap();
    let from_run: Vec<f64> = r["replications"]
        .as_array()
        .unwrap()
        .iter()
        .map(|rep| {
            rep["blocks"]
                .as_array()
                .unwrap()
                .iter()
                .find(|b| b["name"] == "expedicao")
                .unwrap()["input"]
                .as_f64()
                .unwrap()
        })
        .collect();
    let from_forecast: Vec<f64> = producao["valores"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect();
    assert_eq!(from_forecast, from_run);

    let (ok, _, stderr) = utsim(&[
        "prever",
        model.to_str().unwrap(),
        "--horizonte",
        "2h",
        "-n",
        "2",
    ]);
    // The model is in minutes; without --unidade the dashboard default (seconds) gives a
    // 7200-minute horizon, which is valid; a bad goal is not.
    assert!(ok, "{stderr}");
    let (ok, _, stderr) = utsim(&["prever", model.to_str().unwrap(), "--meta", "producao=5"]);
    assert!(!ok && stderr.contains("meta"), "{stderr}");
    let _ = std::fs::remove_dir_all(&dir);
}
