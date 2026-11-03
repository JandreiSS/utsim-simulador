//! MQTT end to end on 127.0.0.1: a minimal broker embedded in the test (MQTT 3.1.1,
//! QoS 0: CONNECT, SUBSCRIBE, PUBLISH, PINGREQ), the twin's real MQTT client
//! (`rumqttc`) for JSON payloads and for Sparkplug B, through the live loop.

mod common;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use common::pb::{V, metric, payload};
use utsim_twin::config::StateClass;
use utsim_twin::live::{self, LiveOptions, Shared};
use utsim_twin::{Twin, simdist};

type Subscribers = Arc<Mutex<Vec<(Vec<String>, TcpStream)>>>;

fn read_packet(s: &mut TcpStream) -> std::io::Result<(u8, Vec<u8>)> {
    let mut b = [0u8; 1];
    s.read_exact(&mut b)?;
    let kind = b[0];
    let (mut len, mut mult) = (0usize, 1usize);
    loop {
        s.read_exact(&mut b)?;
        len += usize::from(b[0] & 0x7f) * mult;
        mult *= 128;
        if b[0] & 0x80 == 0 {
            break;
        }
    }
    let mut body = vec![0u8; len];
    s.read_exact(&mut body)?;
    Ok((kind, body))
}

fn remaining(mut n: usize, out: &mut Vec<u8>) {
    loop {
        let mut b = (n % 128) as u8;
        n /= 128;
        if n > 0 {
            b |= 0x80;
        }
        out.push(b);
        if n == 0 {
            break;
        }
    }
}

/// A broker that keeps the subscriptions of every client.
fn broker() -> (u16, Subscribers) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let subs: Subscribers = Arc::new(Mutex::new(Vec::new()));
    let s2 = subs.clone();
    std::thread::spawn(move || {
        for conn in listener.incoming() {
            let Ok(mut stream) = conn else { continue };
            let subs = s2.clone();
            std::thread::spawn(move || {
                while let Ok((kind, body)) = read_packet(&mut stream) {
                    match kind >> 4 {
                        1 => stream.write_all(&[0x20, 0x02, 0x00, 0x00]).unwrap(),
                        8 => {
                            let pid = [body[0], body[1]];
                            let mut i = 2;
                            let mut topics = Vec::new();
                            while i + 2 <= body.len() {
                                let n = usize::from(body[i]) << 8 | usize::from(body[i + 1]);
                                topics.push(
                                    String::from_utf8(body[i + 2..i + 2 + n].to_vec()).unwrap(),
                                );
                                i += 3 + n;
                            }
                            let mut ack = vec![0x90];
                            remaining(2 + topics.len(), &mut ack);
                            ack.extend_from_slice(&pid);
                            ack.extend(std::iter::repeat_n(0u8, topics.len()));
                            stream.write_all(&ack).unwrap();
                            subs.lock()
                                .unwrap()
                                .push((topics, stream.try_clone().unwrap()));
                        }
                        12 => stream.write_all(&[0xd0, 0x00]).unwrap(),
                        14 => break,
                        _ => {}
                    }
                }
            });
        }
    });
    (port, subs)
}

fn matches(filter: &str, topic: &str) -> bool {
    match filter.strip_suffix('#') {
        Some(prefix) => topic.starts_with(prefix),
        None => filter == topic,
    }
}

fn publish(subs: &Subscribers, topic: &str, body: &[u8]) {
    let mut p = vec![0x30];
    remaining(2 + topic.len() + body.len(), &mut p);
    p.extend_from_slice(&(topic.len() as u16).to_be_bytes());
    p.extend_from_slice(topic.as_bytes());
    p.extend_from_slice(body);
    for (filters, s) in subs.lock().unwrap().iter_mut() {
        if filters.iter().any(|f| matches(f, topic)) {
            s.write_all(&p).unwrap();
        }
    }
}

#[test]
fn json_and_sparkplug_over_mqtt() {
    let (port, subs) = broker();
    let text = format!(
        r#"Twin(
    model: "linha.ron",
    clock: Wall(unit_seconds: 0.01),
    sources: [
        (name: "uns", kind: Mqtt(host: "127.0.0.1", port: {port}, topics: ["fabrica/#"])),
        (name: "spb", kind: Sparkplug(host: "127.0.0.1", port: {port}, group: "G1")),
    ],
    tags: [
        (source: "uns", tag: "fabrica/fila/qtd", object: "fila", measure: Content),
        (source: "uns", tag: "fabrica/maquina/estado", object: "maquina", measure: State,
         states: {{"3": Down}}),
        (source: "spb", tag: "G1/borda1/M1/Saidas", object: "maquina", measure: Outputs),
    ],
    shadow: (window: 200.0, replications: 2, warmup: 400.0, samples_per_window: 5),
)"#
    );
    let cfg = utsim_twin::TwinConfig::parse(&text).unwrap();
    let model = common::model(0.6, 2_000.0);
    cfg.validate(&model, None).unwrap();
    let sim = simdist::simulate(&model, &cfg).unwrap();
    let cancel = Arc::new(AtomicBool::new(false));
    let started = live::start(utsim_twin::connect::build(&cfg).unwrap(), &cancel).unwrap();
    let shared = Arc::new(Shared {
        health: started.health.clone(),
        twin: Mutex::new(Twin::new(model, cfg, sim)),
    });
    let (s2, c2) = (shared.clone(), cancel.clone());
    let worker = std::thread::spawn(move || {
        live::run(
            &s2,
            started,
            &LiveOptions {
                tick: Duration::from_millis(10),
                until: None,
                max_wall: Some(Duration::from_secs(30)),
                cancel: c2,
            },
        )
    });

    // Both clients subscribed.
    let deadline = Instant::now() + Duration::from_secs(10);
    while subs.lock().unwrap().len() < 2 {
        assert!(Instant::now() < deadline, "clientes MQTT não assinaram");
        std::thread::sleep(Duration::from_millis(20));
    }
    publish(
        &subs,
        "fabrica/fila/qtd",
        br#"{"valor": 7, "unidade": "pecas"}"#,
    );
    publish(&subs, "fabrica/maquina/estado", b"3");
    publish(&subs, "fabrica/outro", b"1");
    let birth = payload(0, &[metric(Some("Saidas"), Some(5), 7, V::Int(40))]);
    publish(&subs, "spBv1.0/G1/DBIRTH/borda1/M1", &birth);
    let data = payload(1, &[metric(None, Some(5), 7, V::Int(45))]);
    publish(&subs, "spBv1.0/G1/DDATA/borda1/M1", &data);

    let deadline = Instant::now() + Duration::from_secs(10);
    let report = loop {
        let r = shared.json(false);
        let fila = &r["objetos"]["fila"]["conteudo"];
        let saidas = &r["objetos"]["maquina"]["saidas"];
        if fila == 7.0 && saidas == 5.0 && r["objetos"]["maquina"]["estado"] == "Down" {
            break r;
        }
        assert!(Instant::now() < deadline, "leituras não chegaram: {r:#}");
        std::thread::sleep(Duration::from_millis(20));
    };
    cancel.store(true, Ordering::Relaxed);
    worker.join().unwrap();
    assert_eq!(report["conexoes"]["uns"]["recebidas"], 3);
    assert_eq!(report["conexoes"]["spb"]["recebidas"], 2);
    assert_eq!(report["nao_mapeadas"], 1);
    let twin = shared.twin.lock().unwrap();
    assert_eq!(
        twin.report(false).objetos["maquina"].estado,
        Some(StateClass::Down)
    );
}
