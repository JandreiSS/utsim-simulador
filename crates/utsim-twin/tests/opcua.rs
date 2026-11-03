//! OPC UA end to end on 127.0.0.1: the synthetic factory is `utsim-emu` (the OPC UA
//! server of `utsim emular`) running the line with a longer cycle time than the model;
//! the twin's OPC UA client subscribes to its nodes, uses `TempoSimulado` as the clock
//! and must raise a deviation alert on the machine. The session is recorded and the
//! replay gives the same report.

mod common;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use utsim_emu::{Emulator, OpcUaFrontend, RunOptions, SystemClock, run_paced};
use utsim_twin::drift::Level;
use utsim_twin::live::{self, LiveOptions, Shared};
use utsim_twin::record::{Recorder, Recording, SessionHeader};
use utsim_twin::{Twin, TwinConfig, replay, simdist};

#[test]
fn deviation_from_the_synthetic_factory_over_opcua() {
    // The model's distribution first (it does not depend on the connections).
    let window = 200.0;
    let model = common::model(0.6, 20_000.0);
    let draft = TwinConfig::parse(&common::config_text("opc.tcp://127.0.0.1:1/", window, 20));
    let sim = simdist::simulate(&model, &draft.unwrap()).unwrap();

    // The plant: cycle 0.85 instead of the model's 0.6.
    let stop_plant = Arc::new(AtomicBool::new(false));
    let (tx, rx) = channel::<String>();
    let stop2 = stop_plant.clone();
    let factory = std::thread::spawn(move || {
        let plant = common::model(0.85, 1e9);
        let mut emu = Emulator::new(plant.build(1000)).unwrap();
        let names = emu.block_names().to_vec();
        let mut fe = OpcUaFrontend::start("linha", &names, 0, 250.0).unwrap();
        tx.send(fe.endpoint().to_string()).unwrap();
        let opts = RunOptions {
            speed: 250.0,
            tick: Duration::from_millis(20),
            cancel: Some(stop2),
            ..Default::default()
        };
        run_paced(&mut emu, &SystemClock::new(), &mut fe, &opts).unwrap();
    });
    let endpoint = rx.recv_timeout(Duration::from_secs(20)).expect("endpoint");
    assert!(endpoint.starts_with("opc.tcp://127.0.0.1:"));

    let cfg = TwinConfig::parse(&common::config_text(&endpoint, window, 20)).unwrap();
    cfg.validate(&model, None).unwrap();

    let dir = std::env::temp_dir().join(format!("utsim-twin-opcua-{}", std::process::id()));
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
    let cancel = Arc::new(AtomicBool::new(false));
    let started = live::start(utsim_twin::connect::build(&cfg).unwrap(), &cancel).unwrap();
    let shared = Shared {
        health: started.health.clone(),
        twin: Mutex::new(
            Twin::new(model.clone(), cfg.clone(), sim.clone()).with_recorder(recorder),
        ),
    };
    let until = 400.0 + 10.0 * window;
    let end = live::run(
        &shared,
        started,
        &LiveOptions {
            tick: Duration::from_millis(10),
            until: Some(until),
            max_wall: Some(Duration::from_secs(60)),
            cancel,
        },
    );
    let report = shared.json(false);
    stop_plant.store(true, Ordering::Relaxed);
    factory.join().unwrap();
    assert!(
        end >= until,
        "o relógio da fábrica não chegou a {until}: {end}; {report:#}"
    );
    assert_eq!(report["conexoes"]["clp"]["descartadas"], 0);

    let twin = shared.twin.into_inner().unwrap();
    let live_report = twin.report(false);
    drop(twin);
    assert!(live_report.janelas.len() >= 10, "{report:#}");
    let dev = live_report
        .alertas
        .iter()
        .find(|a| a.nivel == Level::Deviation && a.objeto == "maquina")
        .unwrap_or_else(|| {
            panic!(
                "sem desvio na máquina: {:#?} {:#?}",
                live_report.alertas, live_report.janelas
            )
        });
    assert!(dev.janela <= 5, "{dev:#?}");

    // Replay of the recording, without the factory.
    let rec = Recording::open(&dir).unwrap();
    let mut again = Twin::new(model, cfg, sim);
    replay(&mut again, &rec);
    assert_eq!(
        serde_json::to_string(&again.report(false)).unwrap(),
        serde_json::to_string(&live_report).unwrap()
    );
    let _ = std::fs::remove_dir_all(&dir);
}
