//! Forecast from a state: equal to a plain run from an empty state, an M/M/1 that starts
//! with a long queue drains to its steady state, the probability of a goal in a
//! deterministic line, a scenario with more capacity improves with a significant paired
//! difference, and the replay of a recording repeats the session's forecasts. No network.

mod common;

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use utsim_dash::config::{Target, TimeUnit};
use utsim_dash::kpi::Situation;
use utsim_model::experiment::Scenario;
use utsim_model::{InitialState, Model};
use utsim_twin::forecast::{self, Forecast, ForecastSettings, Horizon, Limit, Request};
use utsim_twin::live::Shared;
use utsim_twin::obs::Quality;
use utsim_twin::panel::TwinPanel;
use utsim_twin::record::{Line, Recorder, Recording, SessionHeader};
use utsim_twin::{Twin, TwinConfig, replay, simdist};

fn mm1(service: f64, stop: f64, reps: u32) -> Model {
    Model::parse(&format!(
        r#"Model(
    name: "mm1",
    run: (stop_time: {stop:.1}, warmup: 0.0, seed: 11, replications: {reps}),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0))),
        (name: "fila", kind: Queue()),
        (name: "maquina", kind: Processor(process: Exp(mean: {service}))),
        (name: "saida", kind: Sink),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#
    ))
    .unwrap()
}

fn queue_of(n: usize) -> InitialState {
    InitialState::parse(&format!(
        "InitialState(objects: {{\"fila\": (count: {n})}})"
    ))
    .unwrap()
}

fn run(model: &Model, settings: &ForecastSettings, state: &InitialState) -> Forecast {
    forecast::forecast(
        &Request {
            model,
            settings,
            unit: TimeUnit::Unit,
            instant: 0.0,
            seed: model.run.seed,
            on_request: true,
        },
        state,
    )
    .unwrap()
}

#[test]
fn from_an_empty_state_it_is_a_plain_run() {
    let model = mm1(0.6, 500.0, 4);
    let normal = model.run_replications().unwrap();
    let times: Vec<f64> = (0..=25).map(|k| 20.0 * f64::from(k)).collect();
    let reps = forecast::replicate_all(&model, &InitialState::default(), &times, &[], &[]).unwrap();
    assert_eq!(reps.len(), 4);
    for (a, b) in normal.iter().zip(&reps) {
        assert_eq!(
            serde_json::to_string(a).unwrap(),
            serde_json::to_string(&b.report).unwrap(),
            "a amostragem no meio da rodada não muda nada"
        );
    }
    let settings = ForecastSettings {
        horizon: Horizon::Time(500.0),
        replications: 4,
        points: 25,
        ..ForecastSettings::default()
    };
    let f = run(&model, &settings, &InitialState::default());
    let produced: Vec<f64> = normal
        .iter()
        .map(|r| r.block("saida").unwrap().input as f64)
        .collect();
    assert_eq!(f.kpi("producao").unwrap().valores, produced);
    // The band ends at the produced totals.
    let last = f.series[0].pontos.last().unwrap();
    assert_eq!(last.t, 500.0);
    let mean = produced.iter().sum::<f64>() / 4.0;
    assert!((last.media - mean).abs() < 1e-9);
}

#[test]
fn mm1_starting_with_a_queue_drains_to_steady_state() {
    // ρ = 0.5: L = ρ / (1 − ρ) = 1 in the system; 50 waiting drain at about 1 per unit.
    let model = mm1(0.5, 1.0, 1);
    let settings = ForecastSettings {
        horizon: Horizon::Time(400.0),
        replications: 40,
        points: 40,
        watch: vec!["fila".into()],
        limits: vec![Limit {
            object: "fila".into(),
            above: None,
            below: Some(3.0),
        }],
        ..ForecastSettings::default()
    };
    let f = run(&model, &settings, &queue_of(50));
    let wip = f.series.iter().find(|s| s.id == "wip").unwrap();
    assert_eq!(wip.pontos.len(), 41);
    assert_eq!(wip.pontos[0].p50, 50.0, "{:?}", wip.pontos[0]);
    let tail: Vec<f64> = wip.pontos[30..].iter().map(|p| p.media).collect();
    let tail_mean = tail.iter().sum::<f64>() / tail.len() as f64;
    assert!((0.5..1.7).contains(&tail_mean), "WIP final {tail_mean}");
    // The band is wide at the start of the drain and the median falls monotonically early on.
    assert!(wip.pontos[2].p50 < wip.pontos[0].p50);
    let e = &f.eventos[0];
    assert_eq!(e.sentido, "abaixo");
    assert!(e.prob.p >= 0.95, "{e:?}");
    let t50 = e.instante_p50.unwrap();
    assert!((30.0..90.0).contains(&t50), "esvazia por volta de {t50}");
    assert!(f.kpi("conteudo_final:fila").unwrap().valor < 3.0);
    assert!(f.resumo.contains("Produção no horizonte"), "{}", f.resumo);
}

#[test]
fn probability_of_the_goal_in_a_deterministic_line() {
    let model = Model::parse(
        r#"Model(
    name: "fixa",
    run: (stop_time: 1.0, seed: 3),
    objects: [
        (name: "chegada", kind: Source(interarrival: Const(1.0))),
        (name: "maquina", kind: Processor(process: Const(0.5))),
        (name: "saida", kind: Sink),
    ],
    connections: [("chegada", "maquina"), ("maquina", "saida")],
)"#,
    )
    .unwrap();
    let mut settings = ForecastSettings {
        horizon: Horizon::Time(100.0),
        replications: 10,
        points: 10,
        ..ForecastSettings::default()
    };
    let f = run(&model, &settings, &InitialState::default());
    let k = f.kpi("producao").unwrap();
    let n = k.valores[0];
    assert!(
        k.valores.iter().all(|v| *v == n) && n > 90.0,
        "{:?}",
        k.valores
    );
    let z2 = 1.959_963_984_540_054f64.powi(2);

    settings.targets = BTreeMap::from([("producao".to_string(), Target::AtLeast(n))]);
    let f = run(&model, &settings, &InitialState::default());
    let k = f.kpi("producao").unwrap();
    let p = k.prob.as_ref().unwrap();
    assert_eq!((p.k, p.n, p.p), (10, 10, 1.0));
    assert!((p.lo - 10.0 / (10.0 + z2)).abs() < 1e-9, "Wilson: {}", p.lo);
    assert_eq!(p.hi, 1.0);
    assert_eq!(k.situacao, Situation::Verde);
    assert!(f.resumo.contains("100 %"), "{}", f.resumo);

    settings.targets = BTreeMap::from([("producao".to_string(), Target::AtLeast(n + 1.0))]);
    let f = run(&model, &settings, &InitialState::default());
    let k = f.kpi("producao").unwrap();
    let p = k.prob.as_ref().unwrap();
    assert_eq!((p.k, p.p), (0, 0.0));
    assert!((p.hi - z2 / (10.0 + z2)).abs() < 1e-9, "Wilson: {}", p.hi);
    assert_eq!(k.situacao, Situation::Vermelho);
}

#[test]
fn more_capacity_improves_with_a_significant_paired_difference() {
    let model = mm1(0.9, 1.0, 1);
    let settings = ForecastSettings {
        horizon: Horizon::Time(60.0),
        replications: 30,
        points: 12,
        targets: BTreeMap::from([("producao".to_string(), Target::AtLeast(70.0))]),
        scenarios: vec![
            Scenario {
                name: "duas máquinas".into(),
                set: BTreeMap::from([("maquina.capacity".to_string(), "2".to_string())]),
            },
            Scenario {
                name: "fila inexistente".into(),
                set: BTreeMap::from([("nada.capacity".to_string(), "2".to_string())]),
            },
        ],
        ..ForecastSettings::default()
    };
    let f = run(&model, &settings, &queue_of(30));
    let base = f.kpi("producao").unwrap();
    let sc = &f.cenarios[0];
    assert!(sc.erro.is_none(), "{:?}", sc.erro);
    let c = sc.kpis.iter().find(|c| c.id == "producao").unwrap();
    assert!(c.significativa && c.melhora == Some(true), "{c:?}");
    assert!(c.diferenca > 10.0, "{c:?}");
    assert!((c.base - base.valor).abs() < 1e-12);
    assert!(c.prob.as_ref().unwrap().p >= base.prob.as_ref().unwrap().p);
    // Common random numbers: the paired interval is narrower than the unpaired one would be.
    let [lo, hi] = c.ic_diferenca.unwrap();
    let base_hw = base.ic.map(|[a, b]| (b - a) / 2.0).unwrap();
    assert!((hi - lo) / 2.0 < 2.0 * base_hw * 1.5);
    // The WIP gets lower with two machines.
    let w = sc.kpis.iter().find(|c| c.id == "wip").unwrap();
    assert!(w.diferenca < 0.0 && w.melhora == Some(true), "{w:?}");
    // A scenario that does not fit the model is reported, not fatal.
    assert!(f.cenarios[1].erro.as_ref().unwrap().contains("nada"));
}

/// Wall time of a reading at twin time `t` (a fixed day: the recording splits its files
/// by day, and the forecast lines follow the readings' days).
fn wall(t: f64) -> u64 {
    1_793_000_000_000 + (t * 1000.0) as u64
}

fn twin_with_forecast() -> (Twin, ForecastSettings) {
    let model = common::model(0.6, 4_000.0);
    let cfg = TwinConfig::parse(&common::config_text("opc.tcp://127.0.0.1:1/", 200.0, 3)).unwrap();
    let sim = simdist::simulate(&model, &cfg).unwrap();
    let mut twin = Twin::new(model, cfg, sim);
    let settings = ForecastSettings {
        every: Some(300.0),
        horizon: Horizon::Time(150.0),
        replications: 4,
        points: 10,
        targets: BTreeMap::from([("producao".to_string(), Target::AtLeast(140.0))]),
        limits: vec![Limit {
            object: "fila".into(),
            above: Some(5.0),
            below: None,
        }],
        scenarios: vec![Scenario {
            name: "mais rápida".into(),
            set: BTreeMap::from([("maquina.process".to_string(), "Exp(mean: 0.5)".to_string())]),
        }],
        ..ForecastSettings::default()
    };
    twin.enable_forecast(settings.clone(), TimeUnit::Unit);
    (twin, settings)
}

#[test]
fn replaying_a_recording_repeats_its_forecasts() {
    let (mut live, _) = twin_with_forecast();
    let dir = std::env::temp_dir().join(format!("utsim-twin-previsao-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let header = SessionHeader {
        formato: 1,
        inicio_ms: utsim_twin::obs::now_ms(),
        config: common::config_text("opc.tcp://127.0.0.1:1/", 200.0, 3),
        modelo: "linha.ron".into(),
        hash_modelo: String::new(),
        anonimizado: true,
        retencao_dias: 30,
    };
    live = live.with_recorder(Recorder::start(&dir, &header).unwrap());
    let plant = common::model(0.6, 1e9);
    let obs = common::plant(&plant, 3, 1_000.0, 1.0);
    let half = obs.len() / 2;
    for (i, (t, tag, v)) in obs.into_iter().enumerate() {
        // The live loop also moves the clock between readings.
        live.advance(t);
        let o = live.stamp("clp", &tag, v, Quality::Good, 0, t, wall(t));
        live.ingest(o);
        if i == half {
            assert!(live.request_forecast());
        }
    }
    live.finish(1_000.0);
    live.run_forecasts();
    let session = live.recorder().unwrap().session().to_path_buf();
    let got = live.forecasts();
    // Scheduled at 300, 600, 900, plus the one asked for.
    assert_eq!(got.len(), 4, "{:?}", live.forecast_errors());
    assert_eq!(got.iter().filter(|f| f.pedido).count(), 1);
    let scheduled: Vec<f64> = got
        .iter()
        .filter(|f| !f.pedido)
        .map(|f| f.instante)
        .collect();
    assert_eq!(scheduled, [300.0, 600.0, 900.0]);
    assert!(got[0].kpi("producao").unwrap().prob.is_some());
    assert_eq!(got[0].cenarios.len(), 1);
    assert!(got[1].relogio > 0.0);
    // Independent seeds per instant.
    assert_ne!(got[0].semente, got[1].semente);

    let rec = Recording::open(&session).unwrap();
    let lines = rec
        .lines
        .iter()
        .filter(|l| matches!(l, Line::Forecast { .. }))
        .count();
    assert_eq!(lines, 4);
    let (mut again, _) = twin_with_forecast();
    replay(&mut again, &rec);
    again.run_forecasts();
    assert_eq!(
        serde_json::to_string(live.forecasts()).unwrap(),
        serde_json::to_string(again.forecasts()).unwrap()
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_forecast_section_is_validated() {
    let model = common::model(0.6, 4_000.0);
    let ok = common::config_text("opc.tcp://127.0.0.1:1/", 200.0, 3).replace(
        "\n)",
        r#"
    forecast: (horizon: Time(100.0), limits: [(object: "fila", above: 4.0)],
               scenarios: [(name: "x", set: {"maquina.capacity": "2"})]),
)"#,
    );
    let cfg = TwinConfig::parse(&ok).unwrap();
    cfg.validate(&model, None).unwrap();
    assert_eq!(cfg.forecast.as_ref().unwrap().replications, 40);
    let bad = common::config_text("opc.tcp://127.0.0.1:1/", 200.0, 3).replace(
        "\n)",
        r#"
    previsao: (replicacoes: 1, limites: [(objeto: "nada", acima_de: 4.0), (objeto: "fila")],
               acompanhar: ["outro"], cenarios: [(name: "x", set: {"maquina.nada": "2"})]),
)"#,
    );
    let cfg = TwinConfig::parse(&bad).unwrap();
    let err = cfg.validate(&model, None).unwrap_err().to_string();
    for want in [
        "replications",
        "'nada' não existe",
        "dê above ou below",
        "'outro' não existe",
        "cenário 'x'",
    ] {
        assert!(err.contains(want), "falta '{want}' em: {err}");
    }
}

#[test]
fn the_shipped_example_has_a_valid_forecast() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples/gemeo");
    let text = std::fs::read_to_string(format!("{dir}/linha.gemeo.ron")).unwrap();
    let p = utsim_twin::prepare_text(&text, Some(std::path::Path::new(dir)), None).unwrap();
    let f = p.config.forecast.as_ref().unwrap();
    assert_eq!(f.horizon, Horizon::Time(400.0));
    assert_eq!(f.scenarios.len(), 1);
}

fn http(port: u16, method: &str, path: &str, header: &str) -> (String, String) {
    let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
    write!(
        s,
        "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\n{header}Content-Length: 0\r\nConnection: close\r\n\r\n"
    )
    .unwrap();
    let mut out = String::new();
    s.read_to_string(&mut out).unwrap();
    let (head, body) = out.split_once("\r\n\r\n").unwrap();
    (head.to_string(), body.to_string())
}

#[test]
fn forecast_page_and_routes_on_the_dashboard() {
    let (mut twin, _) = twin_with_forecast();
    let model = twin.model().clone();
    let plant = common::model(0.6, 1e9);
    for (t, tag, v) in common::plant(&plant, 5, 400.0, 1.0) {
        let o = twin.stamp("clp", &tag, v, Quality::Good, 0, t, wall(t));
        twin.ingest(o);
    }
    twin.run_forecasts();
    let shared = Arc::new(Shared {
        twin: Mutex::new(twin),
        health: Vec::new(),
    });
    let study =
        utsim_dash::Study::start(model, utsim_dash::Dashboard::default(), Some(2), "t").unwrap();
    let extra: Arc<dyn utsim_dash::server::Extra> = Arc::new(TwinPanel {
        shared: shared.clone(),
    });
    let server = utsim_dash::Server::start_with(study, 0, Some(extra)).unwrap();
    let port = server.addr().port();

    let (head, body) = http(port, "GET", "/gemeo/previsao", "");
    assert!(head.starts_with("HTTP/1.1 200"), "{head}");
    assert!(body.contains("/previsao.js") && body.contains("Precisão da média"));
    assert!(body.contains("Faixa do que pode acontecer"));
    let (head, _) = http(port, "GET", "/previsao.js", "");
    assert!(head.contains("text/javascript"));
    let (_, body) = http(port, "GET", "/api/gemeo/previsao", "");
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["ativa"], true);
    assert_eq!(v["ultima"]["instante"], 300.0);
    assert!(v["ultima"]["series"][0]["pontos"].as_array().unwrap().len() == 11);
    assert_eq!(v["proxima"], 600.0);

    // Asking for a forecast needs the X-UTSim header.
    let (head, _) = http(port, "POST", "/api/gemeo/previsao", "");
    assert!(!head.starts_with("HTTP/1.1 200"), "{head}");
    let (head, body) = http(port, "POST", "/api/gemeo/previsao", "X-UTSim: 1\r\n");
    assert!(head.starts_with("HTTP/1.1 200"), "{head}");
    assert!(body.contains("\"ok\":true"), "{body}");
    let (_, body) = http(port, "GET", "/api/gemeo/previsao", "");
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["na_fila"], 1);
    shared.twin.lock().unwrap().run_forecasts();
    let (_, body) = http(port, "GET", "/api/gemeo/previsoes", "");
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v.as_array().unwrap().len(), 2);
    assert_eq!(v[1]["pedido"], true);
    server.stop();
}
