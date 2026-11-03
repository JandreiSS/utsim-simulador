//! Shadow mode with the synthetic factory in the same process (no network): alerts when
//! the plant's cycle time changes, none when it behaves like the model, recording with
//! anonymisation and retention, and a replay identical to the live session.

mod common;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::time::Duration;

use utsim_twin::config::TwinConfig;
use utsim_twin::connect::{ChannelConnector, Connector};
use utsim_twin::drift::Level;
use utsim_twin::live::{self, LiveOptions, Shared};
use utsim_twin::obs::{Quality, Value};
use utsim_twin::record::{Recorder, Recording, SessionHeader, apply_retention, day_stamp};
use utsim_twin::simdist::{self, SimDistribution};
use utsim_twin::{Twin, replay};

const WINDOW: f64 = 200.0;

fn setup() -> (utsim_model::Model, TwinConfig, SimDistribution) {
    let model = common::model(0.6, 20_000.0);
    let cfg = TwinConfig::parse(&common::config_text(
        "opc.tcp://127.0.0.1:4840/",
        WINDOW,
        20,
    ))
    .unwrap();
    cfg.validate(&model, None).unwrap();
    let sim = simdist::simulate(&model, &cfg).unwrap();
    (model, cfg, sim)
}

/// Feeds the plant straight into a twin, as the live loop would.
fn shadow(plant_cycle: f64, until: f64) -> Twin {
    let (model, cfg, sim) = setup();
    let mut twin = Twin::new(model, cfg, sim);
    let plant = common::model(plant_cycle, 1e9);
    for (t, tag, v) in common::plant(&plant, 1000, until, 1.0) {
        let obs = twin.stamp("clp", &tag, v, Quality::Good, 0, t, 0);
        twin.ingest(obs);
    }
    twin.finish(until);
    twin
}

#[test]
fn simulated_distribution_has_the_mapped_kpis() {
    let (_, _, sim) = setup();
    let keys: Vec<&str> = sim.kpis.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        [
            "fila.wip",
            "maquina.quebra",
            "maquina.utilizacao",
            "maquina.vazao"
        ]
    );
    // M/M/1 with ρ = 0.6: utilisation and throughput per window of 200 near theory.
    assert!((sim.kpis["maquina.utilizacao"].mean - 0.6).abs() < 0.03);
    assert!((sim.kpis["maquina.vazao"].mean - 200.0).abs() < 8.0);
    assert_eq!(sim.kpis["maquina.vazao"].values.len(), 20 * 98);
}

#[test]
fn no_deviation_when_the_plant_behaves_like_the_model() {
    let twin = shadow(0.6, 400.0 + 40.0 * WINDOW);
    let r = twin.report(false);
    assert_eq!(r.janelas.len(), 40);
    assert!(r.janelas[5].values.contains_key("maquina.vazao"));
    let deviations: Vec<_> = r
        .alertas
        .iter()
        .filter(|a| a.nivel == Level::Deviation)
        .collect();
    assert!(deviations.is_empty(), "{deviations:#?}");
    assert_eq!(r.testes, 40 * 4);
    // Few window warnings (BH at 10 % over 4 KPIs).
    assert!(r.alertas.len() <= 6, "{:#?}", r.alertas);
}

#[test]
fn deviation_when_the_plant_cycle_time_grows() {
    let twin = shadow(0.85, 400.0 + 15.0 * WINDOW);
    let r = twin.report(true);
    let first = r
        .alertas
        .iter()
        .find(|a| a.nivel == Level::Deviation && a.objeto == "maquina")
        .unwrap_or_else(|| panic!("sem desvio: {:#?}", r.alertas));
    assert!(first.janela <= 3, "{first:#?}");
    let util = r
        .alertas
        .iter()
        .find(|a| a.kpi == "maquina.utilizacao")
        .expect("alerta de utilização");
    assert_eq!(util.sentido, "acima");
    assert!(
        util.mensagem
            .contains("utilização de maquina acima do modelo")
    );
    assert_eq!(r.kpis["maquina.utilizacao"].nivel, Level::Deviation);
    // The observed state, as an initial state for the forecast.
    let state = r.estado_inicial.expect("estado");
    assert!(state.objects.contains_key("fila") || r.objetos["fila"].conteudo == Some(0.0));
    assert!((state.clock - (400.0 + 15.0 * WINDOW)).abs() < 1e-9);
}

#[test]
fn live_session_replays_identically_with_people_anonymised() {
    let (model, mut cfg, sim) = setup();
    // A person's position, by tag pattern and by rule.
    cfg.record.people = vec!["rtls".into()];
    cfg.tags.push(
        utsim_twin::config::TwinConfig::parse(
            r#"Twin(model: "x", tags: [(source: "clp", tag: "fabrica/rtls/João", object: "chegada",
                 measure: Position, unit: "mm", person: true)])"#,
        )
        .unwrap()
        .tags
        .remove(0),
    );
    let dir = std::env::temp_dir().join(format!("utsim-twin-replay-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let header = SessionHeader {
        formato: 1,
        inicio_ms: utsim_twin::obs::now_ms(),
        config: String::new(),
        modelo: "linha.ron".into(),
        hash_modelo: String::new(),
        anonimizado: true,
        retencao_dias: 30,
    };
    let recorder = Recorder::start(&dir, &header).unwrap();
    let twin = Twin::new(model.clone(), cfg.clone(), sim.clone()).with_recorder(recorder);
    let shared = Shared {
        twin: std::sync::Mutex::new(twin),
        health: Vec::new(),
    };

    // The plant through a channel transport (mock of a connector).
    let until = 400.0 + 12.0 * WINDOW;
    let (tx, rx) = channel();
    let plant = common::model(0.85, 1e9);
    let mut msgs = common::plant(&plant, 1000, until, 1.0);
    for (i, (t, _, _)) in msgs.clone().iter().enumerate().step_by(97) {
        let _ = i;
        msgs.push((
            *t,
            "fabrica/rtls/João".into(),
            Value::Text(format!(
                r#"{{"x": {}, "y": 250, "nome": "João"}}"#,
                t * 10.0
            )),
        ));
    }
    msgs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let producer = std::thread::spawn(move || {
        for (_, tag, v) in msgs {
            tx.send((tag, v, Quality::Good)).unwrap();
        }
    });
    let cancel = Arc::new(AtomicBool::new(false));
    let connectors: Vec<Box<dyn Connector>> = vec![Box::new(ChannelConnector::new("clp", rx))];
    let started = live::start(connectors, &cancel).unwrap();
    let end = live::run(
        &shared,
        started,
        &LiveOptions {
            tick: Duration::from_millis(5),
            until: Some(until),
            max_wall: Some(Duration::from_secs(60)),
            cancel,
        },
    );
    producer.join().unwrap();
    assert!((end - until).abs() < 1e-9, "fim em {end}");
    let twin = shared.twin.into_inner().unwrap();
    let live_report = twin.report(false);
    let session = twin.recorder().unwrap().session().to_path_buf();
    drop(twin);
    assert!(live_report.janelas.len() >= 12);
    assert!(
        live_report
            .alertas
            .iter()
            .any(|a| a.nivel == Level::Deviation)
    );
    assert!(live_report.objetos["chegada"].posicao.is_some());

    // The recording has no name of a person.
    let text: String = std::fs::read_dir(&session)
        .unwrap()
        .map(|e| std::fs::read_to_string(e.unwrap().path()).unwrap())
        .collect();
    assert!(!text.contains("João"), "nome na gravação");
    assert!(text.contains("pessoa-"));

    // Replay without network nor clock: the same report.
    let rec = Recording::open(&dir).unwrap();
    assert_eq!(rec.session, session);
    let mut again = Twin::new(model, cfg, sim);
    replay(&mut again, &rec);
    let replayed = again.report(false);
    assert_eq!(
        serde_json::to_string(&replayed).unwrap(),
        serde_json::to_string(&live_report).unwrap()
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn retention_deletes_old_days() {
    let dir = std::env::temp_dir().join(format!("utsim-twin-retencao-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let now = utsim_twin::obs::now_ms();
    let day = 86_400_000u64;
    let old = dir.join("sessao-velha");
    std::fs::create_dir_all(&old).unwrap();
    std::fs::write(old.join("sessao.json"), "{}").unwrap();
    std::fs::write(
        old.join(format!("observacoes-{}.jsonl", day_stamp(now - 31 * day))),
        "",
    )
    .unwrap();
    let mixed = dir.join("sessao-mista");
    std::fs::create_dir_all(&mixed).unwrap();
    std::fs::write(mixed.join("sessao.json"), "{}").unwrap();
    let keep = mixed.join(format!("observacoes-{}.jsonl", day_stamp(now - 29 * day)));
    let gone = mixed.join(format!("observacoes-{}.jsonl", day_stamp(now - 40 * day)));
    std::fs::write(&keep, "").unwrap();
    std::fs::write(&gone, "").unwrap();
    let removed = apply_retention(&dir, 30, now).unwrap();
    assert_eq!(removed.len(), 2);
    assert!(!old.exists(), "sessão só com dias velhos é apagada");
    assert!(keep.exists() && !gone.exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn mapping_is_validated_against_the_model() {
    let model = common::model(0.6, 20_000.0);
    let bad = r#"Twin(
        model: "linha.ron",
        sources: [(name: "clp", kind: OpcUa(endpoint: "opc.tcp://127.0.0.1:4840/"))],
        tags: [
            (source: "clp", tag: "a", object: "nada", measure: Content),
            (source: "clp", tag: "b", object: "saida", measure: Outputs),
            (source: "clp", tag: "c", object: "chegada", measure: Inputs),
            (source: "xx", tag: "d", object: "fila", measure: Content, unit: "mm"),
            (source: "clp", tag: "e", object: "maquina", measure: Outputs),
            (source: "clp", tag: "f", object: "maquina", measure: Outputs),
            (source: "clp", tag: "e", object: "maquina", measure: State, rollover: 10.0),
        ],
        shadow: (window: 50000.0),
    )"#;
    let cfg = TwinConfig::parse(bad).unwrap();
    let err = cfg.validate(&model, None).unwrap_err().to_string();
    for want in [
        "o objeto 'nada' não existe",
        "Sink) e não tem saídas",
        "Source) e não tem entradas",
        "conexão 'xx' não existe",
        "unidade 'mm' não se aplica",
        "contagem dupla",
        "o mesmo tag aparece em duas regras",
        "rollover só vale para contadores",
        "não cabe numa janela",
    ] {
        assert!(err.contains(want), "falta «{want}» em:\n{err}");
    }
    // A good file only warns (no security, objects without tags).
    let cfg =
        TwinConfig::parse(&common::config_text("opc.tcp://127.0.0.1:4840/", 200.0, 5)).unwrap();
    let w = cfg
        .validate(&model, Some(&common::model_text(0.6, 20_000.0)))
        .unwrap();
    assert!(w.iter().any(|w| w.contains("sem assinatura")));
    assert!(w.iter().any(|w| w.contains("'saida' sem nenhum tag")));
    // The model hash warns when the model changed.
    let mut cfg = cfg;
    cfg.model_hash = Some(utsim_twin::model_hash("outro texto"));
    let w = cfg.validate(&model, Some("texto atual")).unwrap();
    assert!(w.iter().any(|w| w.contains("o modelo mudou")));
    // Portuguese aliases.
    let pt = r#"Twin(
        modelo: "linha.ron",
        relogio: Local(unidade_segundos: 1.0),
        conexoes: [(nome: "uns", tipo: Mqtt(host: "127.0.0.1", topicos: ["f/#"]))],
        mapeamento: [(conexao: "uns", tag: "f/q", objeto: "fila", medida: Conteudo, banda_morta: 0.5)],
        sombra: (janela: 300.0, replicacoes: 5),
        gravacao: (retencao_dias: 7),
    )"#;
    let cfg = TwinConfig::parse(pt).unwrap();
    assert!(
        cfg.record.anonymize_people,
        "anonimização ligada por padrão"
    );
    assert_eq!(cfg.record.keep_days, 7);
    cfg.validate(&model, None).unwrap();
    assert_eq!(
        TwinConfig::parse("Twin(model: \"m\")")
            .unwrap()
            .record
            .keep_days,
        30
    );
}
