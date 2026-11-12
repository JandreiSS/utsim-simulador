//! End-to-end: `utsim pessoa ajustar` on `examples/pessoas_tempos_proprios.ron` (ana has a
//! time study, bruno has none) and on small models written here.

use std::path::{Path, PathBuf};
use std::process::Command;

use utsim_model::Model;

fn example(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(name)
}

fn utsim(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
        .args(["pessoa", "ajustar"])
        .args(args)
        .output()
        .expect("runs utsim");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("utsim-pessoa-{tag}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn s(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

#[test]
fn the_fit_is_shown_and_nothing_is_written() {
    let model = example("pessoas_tempos_proprios.ron");
    let before = std::fs::read_to_string(&model).unwrap();
    let dir = temp_dir("mostra");
    let json = dir.join("ana.json");
    let (ok, out, err) = utsim(&[&s(&model), "--pessoa", "ana", "--json", &s(&json)]);
    assert!(ok, "{out}\n{err}");
    assert!(
        out.contains("Tarefa 'torno', método 'A': 60 leituras em 5 horários"),
        "{out}"
    );
    assert!(out.contains("tempo descansado t0 = "), "{out}");
    // Readings on both sides of the pause, and five hours: μ comes from the data.
    assert!(out.contains("(estimada)"), "{out}");
    assert!(out.contains("IC 95 %: t0 entre"), "{out}");
    assert!(out.contains("distribuição do tempo descansado:"), "{out}");
    assert!(out.contains("nada foi gravado"), "{out}");
    assert_eq!(std::fs::read_to_string(&model).unwrap(), before);
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).unwrap()).unwrap();
    assert_eq!(v["pessoa"], "ana");
    let fit = &v["ajustes"][0];
    assert_eq!(fit["curva"]["task"], "torno");
    assert_eq!(fit["curva"]["recovery_estimated"], true);
    assert!(
        fit["fadiga"]["Dynamic"]["work"].as_f64().unwrap() > 0.0,
        "{fit}"
    );
    assert!(!fit["distribuicao"]["familia"].as_str().unwrap().is_empty());
}

#[test]
fn the_fit_is_written_into_the_profile() {
    let dir = temp_dir("grava");
    let out_model = dir.join("ajustado.ron");
    let (ok, out, err) = utsim(&[
        &s(&example("pessoas_tempos_proprios.ron")),
        "--pessoa",
        "ana",
        "--gravar",
        "--saida",
        &s(&out_model),
    ]);
    assert!(ok, "{out}\n{err}");
    assert!(out.contains("fadiga pela tarefa 'torno'"), "{out}");
    let m = Model::load(&out_model).unwrap();
    m.validate().unwrap();
    let person = |name: &str| {
        m.objects
            .iter()
            .find(|o| o.name == name)
            .unwrap()
            .person
            .as_deref()
            .unwrap()
            .clone()
    };
    let ana = person("ana");
    let own = &ana.times["torno"];
    assert_eq!(own.method, "A");
    // The readings were drawn around 40 s rested.
    let mean = own.dist.mean();
    assert!(mean > 34.0 && mean < 44.0, "{mean}");
    match ana.fatigue {
        Some(utsim_core::Fatigue::Dynamic {
            work,
            recovery,
            time_factor,
            ..
        }) => {
            assert!(work > 0.0 && time_factor > 0.0);
            // Estimated, so not the 1e-3 the profile had.
            assert_ne!(recovery, 1.0e-3);
        }
        ref other => panic!("{other:?}"),
    }
    // The study stays, so the fit can be redone; bruno is untouched.
    assert_eq!(ana.study.as_ref().unwrap().readings.len(), 5);
    let original = Model::load(example("pessoas_tempos_proprios.ron")).unwrap();
    let bruno = original.objects.iter().find(|o| o.name == "bruno").unwrap();
    assert_eq!(bruno.person.as_deref(), Some(&person("bruno")));
    // And the run uses it.
    let report = m.build(0).run().unwrap();
    assert!(report.metrics()["ana.person.own_times"] > 0.0);
}

#[test]
fn problems_are_explained() {
    let model = s(&example("pessoas_tempos_proprios.ron"));
    let (ok, _, err) = utsim(&[&model, "--pessoa", "bruno"]);
    assert!(!ok);
    assert!(err.contains("bruno não tem estudo de tempos"), "{err}");
    let (ok, _, err) = utsim(&[&model, "--pessoa", "carla"]);
    assert!(!ok);
    assert!(err.contains("\"carla\" não existe"), "{err}");
    let (ok, _, err) = utsim(&[&model, "--pessoa", "ana", "--tarefa", "fresa"]);
    assert!(!ok);
    assert!(err.contains("nenhuma leitura da tarefa 'fresa'"), "{err}");
    let (ok, _, err) = utsim(&[&model, "--pessoa", "ana", "--unidade", "dia"]);
    assert!(!ok);
    assert!(err.contains("use s, min ou h"), "{err}");

    // Two methods of one task: both are shown, but only one can be written.
    let dir = temp_dir("metodos");
    let path = dir.join("dois_metodos.ron");
    let block = |m: &str, h: f64, t: f64| {
        format!(
            "(tarefa: \"torno\", metodo: \"{m}\", hora: {h:?}, tempos: [{}, {}, {}])",
            t - 1.0,
            t,
            t + 1.0
        )
    };
    let readings: Vec<String> = [("A", 40.0), ("B", 30.0)]
        .iter()
        .flat_map(|(m, t)| {
            [(0.5, 1.0), (4.0, 1.1), (7.5, 1.15)]
                .iter()
                .map(|(h, k)| block(m, *h, t * k))
                .collect::<Vec<_>>()
        })
        .collect();
    std::fs::write(
        &path,
        format!(
            r#"Model(
    name: "dois métodos",
    run: (stop_time: 100.0),
    objects: [
        (name: "fonte", kind: Source(interarrival: Const(60.0))),
        (name: "torno", kind: Processor(process: Const(40.0), operators: Some(OperatorUse(team: "ana")))),
        (name: "saida", kind: Sink),
        (name: "ana", kind: Transporter(speed: 1.0),
            person: Some((estudo: Some((leituras: [{}]))))),
    ],
    connections: [("fonte", "torno"), ("torno", "saida")],
)"#,
            readings.join(", ")
        ),
    )
    .unwrap();
    let (ok, out, err) = utsim(&[&s(&path), "--pessoa", "ana"]);
    assert!(ok, "{out}\n{err}");
    assert!(
        out.contains("método 'A'") && out.contains("método 'B'"),
        "{out}"
    );
    let (ok, _, err) = utsim(&[&s(&path), "--pessoa", "ana", "--gravar"]);
    assert!(!ok);
    assert!(err.contains("mais de um método (A, B)"), "{err}");
    let (ok, out, err) = utsim(&[&s(&path), "--pessoa", "ana", "--metodo", "B", "--gravar"]);
    assert!(ok, "{out}\n{err}");
    let m = Model::load(&path).unwrap();
    let ana = m.objects.iter().find(|o| o.name == "ana").unwrap();
    let own = &ana.person.as_deref().unwrap().times["torno"];
    assert_eq!(own.method, "B");
    assert!((own.dist.mean() - 30.0).abs() < 2.0, "{:?}", own.dist);
}
