//! The live scene served by the emulation: a client sees the run and can stop a block.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use utsim_emu::{Emulator, NullFrontend, RunOptions, SceneFrontend, SystemClock, run_paced};
use utsim_model::Model;
use utsim_scene::{EntityId, EntityKind, Frame, Header};

const MODEL: &str = r#"Model(
    name: "mm1",
    run: (stop_time: 2000.0, seed: 4),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0)), position: (0.0, 0.0, 0.0)),
        (name: "fila", kind: Queue(), position: (4.0, 0.0, 0.0)),
        (name: "maquina", kind: Processor(process: Exp(mean: 0.8)), position: (8.0, 0.0, 0.0)),
        (name: "saida", kind: Sink, position: (12.0, 0.0, 0.0)),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#;

fn next_line(r: &mut BufReader<TcpStream>) -> serde_json::Value {
    let mut s = String::new();
    assert!(r.read_line(&mut s).unwrap() > 0, "connection closed");
    serde_json::from_str(&s).unwrap()
}

#[test]
fn a_client_command_stops_a_block_and_the_frames_show_it() {
    let model = Model::parse(MODEL).unwrap();
    model.validate().unwrap();
    let mut emu = Emulator::new_traced(model.build(0)).unwrap();
    let names = emu.block_names().to_vec();
    let mut frontend =
        SceneFrontend::start(NullFrontend, "127.0.0.1:0", &model, &names, 50.0).unwrap();
    let addr = frontend.live().local_addr();
    let cancel = Arc::new(AtomicBool::new(false));
    let opts = RunOptions {
        speed: 50.0,
        cancel: Some(cancel.clone()),
        ..Default::default()
    };
    let run = std::thread::spawn(move || {
        run_paced(&mut emu, &SystemClock::new(), &mut frontend, &opts).unwrap();
    });

    let stream = TcpStream::connect(addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    let mut reader = BufReader::new(stream);
    let header: Header = serde_json::from_value(next_line(&mut reader)).unwrap();
    let id = header
        .scene
        .entities
        .iter()
        .find(|e| matches!(&e.kind, EntityKind::Block { name, .. } if name == "maquina"))
        .map(|e| e.id)
        .expect("maquina in the header");

    let w = reader.get_mut();
    w.write_all(b"{\"tipo\":\"comando\",\"variavel\":\"naoexiste.Parar\",\"valor\":true}\n")
        .unwrap();
    w.write_all(b"{\"tipo\":\"comando\",\"variavel\":\"maquina.Parar\",\"valor\":true}\n")
        .unwrap();

    let deadline = Instant::now() + Duration::from_secs(20);
    let (mut warned, mut stopped, mut saw_items) = (false, false, false);
    while !(warned && stopped) {
        assert!(
            Instant::now() < deadline,
            "warned={warned} stopped={stopped}"
        );
        let v = next_line(&mut reader);
        match v.get("tipo").and_then(|t| t.as_str()) {
            Some("aviso") => {
                assert!(v["mensagem"].as_str().unwrap().contains("naoexiste"));
                warned = true;
            }
            Some(_) => {}
            None => {
                let frame: Frame = serde_json::from_value(v).unwrap();
                saw_items |= frame
                    .poses
                    .iter()
                    .any(|p| matches!(p.id, EntityId::Item(_)));
                stopped |= frame
                    .poses
                    .iter()
                    .any(|p| p.id == id && p.state.as_deref() == Some("stopped"));
            }
        }
    }
    assert!(saw_items, "items of the running simulation were streamed");
    cancel.store(true, Ordering::Relaxed);
    run.join().unwrap();
    // After the run the server says goodbye.
    let mut fim = false;
    for _ in 0..200 {
        let mut s = String::new();
        if reader.read_line(&mut s).unwrap_or(0) == 0 {
            break;
        }
        fim |= s.contains("\"tipo\":\"fim\"");
    }
    assert!(fim, "fim message before the connection closes");
}
