//! The twin's routes on the dashboard server (127.0.0.1 only).

mod common;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use utsim_dash::{Dashboard, Server, Study};
use utsim_twin::live::Shared;
use utsim_twin::obs::Quality;
use utsim_twin::panel::TwinPanel;
use utsim_twin::{Twin, TwinConfig, simdist};

fn get(port: u16, path: &str, host: &str) -> (String, String) {
    let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
    write!(
        s,
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .unwrap();
    let mut out = String::new();
    s.read_to_string(&mut out).unwrap();
    let (head, body) = out.split_once("\r\n\r\n").unwrap();
    (head.to_string(), body.to_string())
}

#[test]
fn twin_page_and_api_on_the_dashboard() {
    let model = common::model(0.6, 4_000.0);
    let cfg = TwinConfig::parse(&common::config_text("opc.tcp://127.0.0.1:1/", 200.0, 3)).unwrap();
    let sim = simdist::simulate(&model, &cfg).unwrap();
    let mut twin = Twin::new(model.clone(), cfg, sim);
    let plant = common::model(0.6, 1e9);
    for (t, tag, v) in common::plant(&plant, 7, 1_000.0, 1.0) {
        let o = twin.stamp("clp", &tag, v, Quality::Good, 0, t, 0);
        twin.ingest(o);
    }
    let shared = Arc::new(Shared {
        twin: Mutex::new(twin),
        health: Vec::new(),
    });
    let study = Study::start(model, Dashboard::default(), Some(2), "teste").unwrap();
    let extra: Arc<dyn utsim_dash::server::Extra> = Arc::new(TwinPanel { shared });
    let server = Server::start_with(study, 0, Some(extra)).unwrap();
    let port = server.addr().port();
    let host = format!("127.0.0.1:{port}");

    let (head, body) = get(port, "/gemeo", &host);
    assert!(head.starts_with("HTTP/1.1 200"), "{head}");
    assert!(body.contains("/gemeo.js") && body.contains("modo sombra"));
    let (head, _) = get(port, "/gemeo.js", &host);
    assert!(head.contains("text/javascript"));

    let (_, body) = get(port, "/api/gemeo", &host);
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["modo"], "sombra");
    assert_eq!(v["janelas"].as_array().unwrap().len(), 3);
    assert!(v["objetos"]["maquina"]["saidas"].as_f64().unwrap() > 0.0);
    let (_, body) = get(port, "/api/gemeo/estado", &host);
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(v["estado_inicial"]["clock"].as_f64().unwrap() >= 1_000.0);
    let (_, body) = get(port, "/api/gemeo/estado.ron", &host);
    assert!(body.starts_with("InitialState("), "{body}");

    // The dashboard's own routes still work, and only the local host is served.
    let (head, _) = get(port, "/", &host);
    assert!(head.starts_with("HTTP/1.1 200"));
    let (head, _) = get(port, "/api/gemeo", "evil.example.com");
    assert!(head.starts_with("HTTP/1.1 403"), "{head}");
    server.stop();
}
