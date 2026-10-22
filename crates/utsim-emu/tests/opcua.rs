//! End-to-end: OPC UA server on a free port, real OPC UA client.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use opcua::client::{ClientBuilder, IdentityToken};
use opcua::crypto::SecurityPolicy;
use opcua::types::{
    AttributeId, DataValue, MessageSecurityMode, NodeId, NumericRange, ReadValueId,
    TimestampsToReturn, Variant, WriteValue,
};
use utsim_emu::{Emulator, OpcUaFrontend, RunOptions, SystemClock, run_paced};
use utsim_model::Model;

const MODEL: &str = r#"Model(
    name: "linha",
    run: (stop_time: 100000.0, seed: 4),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0))),
        (name: "fila", kind: Queue()),
        (name: "maquina", kind: Processor(process: Exp(mean: 0.8))),
        (name: "saida", kind: Sink),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#;

fn node(ns: u16, path: &str) -> NodeId {
    NodeId::new(ns, format!("UTSim/linha/{path}"))
}

async fn read(session: &opcua::client::Session, id: &NodeId) -> Variant {
    let rv = ReadValueId {
        node_id: id.clone(),
        attribute_id: AttributeId::Value as u32,
        ..Default::default()
    };
    let v = session
        .read(&[rv], TimestampsToReturn::Neither, 0.0)
        .await
        .expect("read");
    v[0].value
        .clone()
        .unwrap_or_else(|| panic!("sem valor para {id}: {:?}", v[0].status))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn read_content_and_stop_a_machine_over_opcua() {
    // The simulation loop (and the server's own runtime) live in a separate thread.
    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = channel::<String>();
    let cancel2 = cancel.clone();
    let worker = std::thread::spawn(move || {
        let model = Model::parse(MODEL).unwrap();
        let mut emu = Emulator::new(model.build(0)).unwrap();
        let names = emu.block_names().to_vec();
        let mut fe = OpcUaFrontend::start("linha", &names, 0, 100.0).unwrap();
        tx.send(fe.endpoint().to_string()).unwrap();
        let opts = RunOptions {
            speed: 100.0,
            cancel: Some(cancel2),
            ..Default::default()
        };
        run_paced(&mut emu, &SystemClock::new(), &mut fe, &opts).unwrap()
    });
    let endpoint = rx.recv_timeout(Duration::from_secs(10)).expect("endpoint");

    let mut client = ClientBuilder::new()
        .application_name("utsim-test-client")
        .application_uri("urn:utsim:test-client")
        .pki_dir(std::env::temp_dir().join("utsim-emu-test-client-pki"))
        .create_sample_keypair(true)
        .trust_server_certs(true)
        .session_retry_limit(1)
        .client()
        .unwrap();
    let (session, event_loop) = client
        .connect_to_matching_endpoint(
            (
                endpoint.as_str(),
                SecurityPolicy::None.to_str(),
                MessageSecurityMode::None,
            ),
            IdentityToken::Anonymous,
        )
        .await
        .expect("connect");
    let _handle = event_loop.spawn();
    assert!(
        tokio::time::timeout(Duration::from_secs(5), session.wait_for_connection())
            .await
            .expect("timeout connecting")
    );
    let ns = {
        // Discover the namespace index instead of assuming it.
        let arr = session.read_namespace_array().await.expect("namespaces");
        arr.known_namespaces()
            .get("urn:utsim:emulacao")
            .copied()
            .expect("namespace do UTSim")
    };

    // The queue fills up (arrival rate > service rate once the machine stops).
    tokio::time::sleep(Duration::from_millis(300)).await;
    let content = read(&session, &node(ns, "fila/Conteudo")).await;
    assert!(matches!(content, Variant::UInt32(_)), "{content:?}");
    let t0 = match read(&session, &node(ns, "TempoSimulado")).await {
        Variant::Double(t) => t,
        v => panic!("{v:?}"),
    };

    // Stop the machine.
    let write = WriteValue {
        node_id: node(ns, "maquina/Parar"),
        attribute_id: AttributeId::Value as u32,
        index_range: NumericRange::None,
        value: DataValue::value_only(true),
    };
    let status = session.write(&[write]).await.expect("write");
    assert!(status[0].is_good(), "{:?}", status[0]);

    // The next ticks apply it: the state turns "stopped".
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut state = String::new();
    while Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(100)).await;
        if let Variant::String(s) = read(&session, &node(ns, "maquina/Estado")).await {
            state = s.value().clone().unwrap_or_default();
            if state == "stopped" {
                break;
            }
        }
    }
    assert_eq!(state, "stopped");
    let t1 = match read(&session, &node(ns, "TempoSimulado")).await {
        Variant::Double(t) => t,
        v => panic!("{v:?}"),
    };
    assert!(t1 > t0, "o tempo simulado avança: {t0} -> {t1}");

    cancel.store(true, Ordering::Relaxed);
    let _ = session.disconnect().await;
    let outcome = worker.join().expect("worker");
    assert_eq!(outcome.log.commands.len(), 1);
    assert_eq!(outcome.log.commands[0].variable, "maquina.Parar");
}
