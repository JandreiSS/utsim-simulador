//! Sparkplug B: example messages encoded byte by byte (protobuf wire format, written here
//! independently of `prost`), decoded by the twin; aliases, sequence gaps, deaths; the
//! field numbers of the decoder against the official `.proto` file.

mod common;

use common::pb::{V, bytes, metric, payload};
use utsim_twin::obs::{Quality, Value};
use utsim_twin::sparkplug::{Decoder, FIELDS, MessageType, parse_topic};

/// The official definition (read by the test only: it is not built into the program).
const PROTO: &str = include_str!("../proto/sparkplug_b.proto");

#[test]
fn topics() {
    let t = parse_topic("spBv1.0/G1/DDATA/borda1/M1").unwrap();
    assert_eq!(t.kind, MessageType::DData);
    assert_eq!(t.device.as_deref(), Some("M1"));
    assert_eq!(
        parse_topic("spBv1.0/G1/NBIRTH/borda1").unwrap().device,
        None
    );
    assert!(parse_topic("fabrica/linha1/x").is_none());
    assert_eq!(
        parse_topic("spBv1.0/G1/NCMD/borda1").unwrap().kind,
        MessageType::Other
    );
}

#[test]
fn birth_data_and_death_of_a_node_and_device() {
    let mut d = Decoder::new("G1");
    // NBIRTH: bdSeq and one node metric.
    let nbirth = payload(
        0,
        &[
            metric(Some("bdSeq"), None, 8, V::Long(3)),
            metric(Some("Node Control/Rebirth"), None, 11, V::Bool(false)),
            metric(Some("Temperatura"), Some(1), 10, V::Double(21.5)),
        ],
    );
    let r = d.handle("spBv1.0/G1/NBIRTH/borda1", &nbirth).unwrap();
    assert_eq!(r.len(), 1, "bdSeq e Node Control não viram leituras");
    assert_eq!(r[0].tag, "G1/borda1/Temperatura");
    assert_eq!(r[0].value, Value::Num(21.5));
    assert_eq!(r[0].timestamp, Some(1_700_000_000_123));

    // DBIRTH of device M1, with aliases; Int16 −2 stored as two's complement.
    let dbirth = payload(
        1,
        &[
            metric(Some("Saidas"), Some(10), 7, V::Int(40)),
            metric(Some("Estado"), Some(11), 12, V::Str("processing")),
            metric(Some("Offset"), Some(12), 2, V::Int(0xfffe)),
        ],
    );
    let r = d.handle("spBv1.0/G1/DBIRTH/borda1/M1", &dbirth).unwrap();
    assert_eq!(r.len(), 3);
    assert_eq!(r[0].tag, "G1/borda1/M1/Saidas");
    assert_eq!(r[1].value, Value::Text("processing".into()));
    assert_eq!(r[2].value, Value::Num(-2.0));

    // DDATA by alias only.
    let ddata = payload(
        2,
        &[
            metric(None, Some(10), 7, V::Int(41)),
            metric(None, Some(11), 12, V::Str("down")),
        ],
    );
    let r = d.handle("spBv1.0/G1/DDATA/borda1/M1", &ddata).unwrap();
    assert_eq!(
        r.iter()
            .map(|x| (x.tag.as_str(), x.value.clone()))
            .collect::<Vec<_>>(),
        vec![
            ("G1/borda1/M1/Saidas", Value::Num(41.0)),
            ("G1/borda1/M1/Estado", Value::Text("down".into())),
        ]
    );
    assert_eq!(d.seq_gaps, 0);

    // A gap: seq 5 after 2. Unknown alias counted.
    let gap = payload(5, &[metric(None, Some(99), 7, V::Int(1))]);
    assert!(
        d.handle("spBv1.0/G1/DDATA/borda1/M1", &gap)
            .unwrap()
            .is_empty()
    );
    assert_eq!(d.seq_gaps, 1);
    assert_eq!(d.unknown_aliases, 1);

    // Another group is ignored.
    assert!(
        d.handle("spBv1.0/G2/DDATA/borda1/M1", &ddata)
            .unwrap()
            .is_empty()
    );

    // NDEATH: every metric of the node and its devices turns bad.
    let mut death = Vec::new();
    bytes(2, &metric(Some("bdSeq"), None, 8, V::Long(3)), &mut death);
    let r = d.handle("spBv1.0/G1/NDEATH/borda1", &death).unwrap();
    let mut tags: Vec<&str> = r.iter().map(|x| x.tag.as_str()).collect();
    tags.sort();
    assert_eq!(
        tags,
        [
            "G1/borda1/M1/Estado",
            "G1/borda1/M1/Offset",
            "G1/borda1/M1/Saidas",
            "G1/borda1/Temperatura"
        ]
    );
    assert!(r.iter().all(|x| x.quality == Quality::Bad));

    // Garbage is an error, counted.
    assert!(
        d.handle("spBv1.0/G1/NDATA/borda1", &[0xff, 0xff, 0xff])
            .is_err()
    );
    assert_eq!(d.decode_errors, 1);
}

#[test]
fn ddeath_only_kills_its_device() {
    let mut d = Decoder::new("G1");
    d.handle(
        "spBv1.0/G1/NBIRTH/b",
        &payload(0, &[metric(Some("X"), None, 10, V::Double(1.0))]),
    )
    .unwrap();
    d.handle(
        "spBv1.0/G1/DBIRTH/b/D1",
        &payload(1, &[metric(Some("Y"), None, 10, V::Double(2.0))]),
    )
    .unwrap();
    let r = d
        .handle("spBv1.0/G1/DDEATH/b/D1", &payload(2, &[]))
        .unwrap();
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].tag, "G1/b/D1/Y");
}

/// The field numbers of the decoder are those of the official definition.
#[test]
fn fields_match_the_proto_file() {
    assert!(PROTO.contains("SPDX-License-Identifier: EPL-2.0"));
    // Body of a message, from `message <name> {` to its closing brace (nesting counted).
    fn body<'a>(text: &'a str, name: &str) -> &'a str {
        let start = text.find(&format!("message {name} {{")).expect(name);
        let mut depth = 0;
        for (i, c) in text[start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return &text[start..start + i];
                    }
                }
                _ => {}
            }
        }
        panic!("{name} sem fim");
    }
    let metric_body = body(PROTO, "Metric");
    let mut payload_body = body(PROTO, "Payload").to_string();
    // The payload's own fields are the ones outside its nested messages.
    for nested in [
        "Template",
        "DataSet",
        "PropertyValue",
        "PropertySet",
        "PropertySetList",
        "MetaData",
        "Metric",
    ] {
        let inner = body(&payload_body, nested).to_string();
        payload_body = payload_body.replacen(&inner, "", 1);
    }
    for (msg, field, number) in FIELDS {
        let text = if *msg == "Metric" {
            metric_body
        } else {
            payload_body.as_str()
        };
        let found = text.lines().any(|l| {
            let l = l.split("//").next().unwrap_or("");
            let words: Vec<&str> = l.split_whitespace().collect();
            words.windows(3).any(|w| {
                w[0] == *field && w[1] == "=" && w[2].trim_end_matches(';') == number.to_string()
            })
        });
        assert!(found, "{msg}.{field} = {number} não está no .proto");
    }
}
