//! End-to-end: `utsim linha` (list, presets, report, placing presets) and the `--linha`
//! option of `utsim run`.

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
    let dir = std::env::temp_dir().join(format!("utsim-linha-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn list_and_presets() {
    let (ok, out, err) = utsim(&["linha", "lista"]);
    assert!(ok, "{err}");
    assert!(out.contains("lean_office"), "{out}");
    let (ok, out, _) = utsim(&["linha", "lean_office", "predefinicoes"]);
    assert!(ok);
    for id in [
        "entrada",
        "caixa_entrada",
        "aprovacao",
        "reuniao",
        "conclusao",
    ] {
        assert!(out.contains(id), "{id}");
    }
    let (ok, out, _) = utsim(&["linha", "lean_office", "vocabulario"]);
    assert!(ok && out.contains("caso"));
    let (ok, _, err) = utsim(&["linha", "saude"]);
    assert!(!ok && err.contains("linhas instaladas"), "{err}");
}

#[test]
fn report_of_the_ready_model_with_every_output() {
    let dir = temp_dir();
    let (html, json) = (dir.join("lo.html"), dir.join("lo.json"));
    let (ok, out, err) = utsim(&[
        "linha",
        "lean_office",
        "relatorio",
        &example("aprovacao_de_compras.ron"),
        "-n",
        "2",
        "--html",
        &html.to_string_lossy(),
        "--json",
        &json.to_string_lossy(),
        "--comparar",
        &example("aprovacao_de_compras.ron"),
    ]);
    assert!(ok, "{err}");
    for want in [
        "Lead time (L/T)",
        "Touch time",
        "FPY",
        "% dentro do SLA",
        "%C&A",
    ] {
        assert!(out.contains(want), "{want}: {out}");
    }
    let page = std::fs::read_to_string(&html).unwrap();
    for want in [
        "Mapa do fluxo de valor do processo administrativo",
        "Estado futuro",
        "Fluxograma de raias por departamento",
        "Gerência",
    ] {
        assert!(page.contains(want), "{want}");
    }
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).unwrap()).unwrap();
    assert_eq!(v["replicacoes"].as_array().unwrap().len(), 2);
}

#[test]
fn place_presets_and_run_with_the_line() {
    let dir = temp_dir();
    let model = dir.join("novo.ron");
    let _ = std::fs::remove_file(&model);
    let m = model.to_string_lossy().into_owned();
    let place = |args: &[&str]| {
        let mut all = vec!["linha", "lean_office", "colocar", m.as_str()];
        all.extend_from_slice(args);
        let (ok, out, err) = utsim(&all);
        assert!(ok, "{args:?}: {err}");
        out
    };
    place(&[
        "entrada",
        "--nome",
        "pedidos",
        "--campo",
        "intervalo=Exp(mean: 10.0)",
    ]);
    place(&[
        "caixa_entrada",
        "--posicao",
        "1",
        "0",
        "--departamento",
        "Compras",
    ]);
    place(&["posto", "--posicao", "2", "0", "--campo", "ca=0.9"]);
    place(&["equipe", "--posicao", "2", "2"]);
    let out = place(&["conclusao", "--posicao", "3", "0"]);
    assert!(out.contains("concluido"), "{out}");
    let text = std::fs::read_to_string(&model).unwrap();
    assert!(text.contains("line: \"lean_office\""), "{text}");
    assert!(text.contains("preset: \"lean_office/posto\""), "{text}");
    // Connect by hand and run with the line's indicators.
    let text = text.replace(
        "connections: [],",
        "connections: [(\"pedidos\", \"caixa\"), (\"caixa\", \"posto\"), (\"posto\", \"concluido\")],",
    );
    std::fs::write(&model, text).unwrap();
    let (ok, out, err) = utsim(&["run", &m, "--linha", "lean_office"]);
    assert!(ok, "{err}");
    assert!(out.contains("Lead time (L/T)"), "{out}");
    let (ok, _, err) = utsim(&["run", &m, "--linha", "nada"]);
    assert!(!ok && err.contains("linhas instaladas"), "{err}");
}

/// Splits a shell command line on spaces, keeping text in single quotes together.
fn shell_words(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut quoted = false;
    for c in line.chars() {
        match c {
            '\'' => quoted = !quoted,
            ' ' if !quoted => {
                if !word.is_empty() {
                    words.push(std::mem::take(&mut word));
                }
            }
            _ => word.push(c),
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

#[test]
fn tutorial_commands_build_the_example_model() {
    let doc = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/manual/src/linhas/lean-office/tutorial.md"),
    )
    .unwrap();
    let dir = temp_dir().join("tutorial");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let model = dir.join("reembolso.ron");
    let m = model.to_string_lossy().into_owned();
    let commands: Vec<&str> = doc
        .lines()
        .filter(|l| l.starts_with("utsim linha lean_office colocar reembolso.ron "))
        .collect();
    assert_eq!(commands.len(), 10);
    for line in commands {
        let args: Vec<String> = shell_words(line)
            .into_iter()
            .skip(1)
            .map(|w| if w == "reembolso.ron" { m.clone() } else { w })
            .collect();
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        let (ok, _, err) = utsim(&args);
        assert!(ok, "{line}: {err}");
    }
    // The connections and run settings of the tutorial's step 3, as in the example.
    let want = utsim_model::Model::load(example("reembolso_atual.ron")).unwrap();
    let example_text = std::fs::read_to_string(example("reembolso_atual.ron")).unwrap();
    let run = "run: (stop_time: 43200.0, warmup: 4320.0, seed: 5, replications: 5)";
    assert!(doc.contains(run) && example_text.contains(run));
    for (a, b) in &want.connections {
        let pair = format!("(\"{a}\", \"{b}\")");
        assert!(doc.contains(&pair), "{pair} fora do tutorial");
    }
    let mut made = utsim_model::Model::load(&model).unwrap();
    made.connections = want.connections.clone();
    made.run = want.run.clone();
    made.name = want.name.clone();
    made.description = want.description.clone();
    made.format = want.format;
    assert_eq!(made, want);
}
