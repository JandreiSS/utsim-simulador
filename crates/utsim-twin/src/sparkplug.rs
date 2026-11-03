//! Sparkplug B: topics `spBv1.0/<group>/<type>/<edge node>[/<device>]` and protobuf
//! payloads, decoded with `prost` from the message types of the official definition
//! (`proto/sparkplug_b.proto`, Eclipse Tahu, EPL-2.0; see `proto/ORIGEM.txt`).
//!
//! Only reading: births (`NBIRTH`, `DBIRTH`) give the metric names and their aliases,
//! data (`NDATA`, `DDATA`) the values (by name or alias), deaths (`NDEATH`, `DDEATH`)
//! mark every metric of the node or device as bad quality. Gaps in the `seq` counter
//! (0-255 per edge node) are counted. Commands (`NCMD`, `DCMD`, the rebirth request) are
//! never sent.
//!
//! The tag of a metric is `<group>/<edge node>/<metric>` for node metrics and
//! `<group>/<edge node>/<device>/<metric>` for device metrics.

use std::collections::{BTreeMap, BTreeSet};

use crate::obs::{Quality, Value};

/// `Payload` of `sparkplug_b.proto` (fields used for reading).
#[derive(Clone, PartialEq, prost::Message)]
pub struct Payload {
    #[prost(uint64, optional, tag = "1")]
    pub timestamp: Option<u64>,
    #[prost(message, repeated, tag = "2")]
    pub metrics: Vec<Metric>,
    #[prost(uint64, optional, tag = "3")]
    pub seq: Option<u64>,
    #[prost(string, optional, tag = "4")]
    pub uuid: Option<String>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub body: Option<Vec<u8>>,
}

/// `Payload.Metric` of `sparkplug_b.proto`. `metadata` (8), `properties` (9) and the
/// values `dataset_value` (17), `template_value` (18) and `extension_value` (19) are not
/// read (skipped as unknown fields).
#[derive(Clone, PartialEq, prost::Message)]
pub struct Metric {
    #[prost(string, optional, tag = "1")]
    pub name: Option<String>,
    #[prost(uint64, optional, tag = "2")]
    pub alias: Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub timestamp: Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub datatype: Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub is_historical: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub is_transient: Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub is_null: Option<bool>,
    #[prost(oneof = "MetricValue", tags = "10, 11, 12, 13, 14, 15, 16")]
    pub value: Option<MetricValue>,
}

/// `oneof value` of `Payload.Metric` (the scalar members).
#[derive(Clone, PartialEq, prost::Oneof)]
pub enum MetricValue {
    #[prost(uint32, tag = "10")]
    IntValue(u32),
    #[prost(uint64, tag = "11")]
    LongValue(u64),
    #[prost(float, tag = "12")]
    FloatValue(f32),
    #[prost(double, tag = "13")]
    DoubleValue(f64),
    #[prost(bool, tag = "14")]
    BooleanValue(bool),
    #[prost(string, tag = "15")]
    StringValue(String),
    #[prost(bytes = "vec", tag = "16")]
    BytesValue(Vec<u8>),
}

/// Field numbers used above, as `(message, field, number)`: checked against the `.proto`
/// file by a test.
pub const FIELDS: &[(&str, &str, u32)] = &[
    ("Payload", "timestamp", 1),
    ("Payload", "metrics", 2),
    ("Payload", "seq", 3),
    ("Payload", "uuid", 4),
    ("Payload", "body", 5),
    ("Metric", "name", 1),
    ("Metric", "alias", 2),
    ("Metric", "timestamp", 3),
    ("Metric", "datatype", 4),
    ("Metric", "is_historical", 5),
    ("Metric", "is_transient", 6),
    ("Metric", "is_null", 7),
    ("Metric", "int_value", 10),
    ("Metric", "long_value", 11),
    ("Metric", "float_value", 12),
    ("Metric", "double_value", 13),
    ("Metric", "boolean_value", 14),
    ("Metric", "string_value", 15),
    ("Metric", "bytes_value", 16),
];

/// `DataType` codes of the specification used in the conversion.
mod dt {
    pub const INT8: u32 = 1;
    pub const INT16: u32 = 2;
    pub const INT32: u32 = 3;
    pub const INT64: u32 = 4;
    pub const DATETIME: u32 = 13;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageType {
    NBirth,
    NDeath,
    DBirth,
    DDeath,
    NData,
    DData,
    /// Commands and the host state: ignored.
    Other,
}

/// A parsed topic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Topic {
    pub group: String,
    pub kind: MessageType,
    pub edge: String,
    pub device: Option<String>,
}

pub fn parse_topic(topic: &str) -> Option<Topic> {
    let mut p = topic.split('/');
    if p.next()? != "spBv1.0" {
        return None;
    }
    let group = p.next()?.to_string();
    let kind = match p.next()? {
        "NBIRTH" => MessageType::NBirth,
        "NDEATH" => MessageType::NDeath,
        "DBIRTH" => MessageType::DBirth,
        "DDEATH" => MessageType::DDeath,
        "NDATA" => MessageType::NData,
        "DDATA" => MessageType::DData,
        _ => MessageType::Other,
    };
    let edge = p.next()?.to_string();
    let device = p.next().map(str::to_string);
    if p.next().is_some() {
        return None;
    }
    Some(Topic {
        group,
        kind,
        edge,
        device,
    })
}

/// A reading decoded from a Sparkplug message.
#[derive(Clone, Debug, PartialEq)]
pub struct Reading {
    pub tag: String,
    pub value: Value,
    pub quality: Quality,
    /// Source timestamp, ms since 1970 (metric's, else the payload's).
    pub timestamp: Option<u64>,
}

fn convert(m: &Metric) -> Option<Value> {
    if m.is_null == Some(true) {
        return None;
    }
    let datatype = m.datatype.unwrap_or(0);
    Some(match m.value.as_ref()? {
        MetricValue::IntValue(v) => Value::Num(match datatype {
            dt::INT8 => f64::from(*v as u8 as i8),
            dt::INT16 => f64::from(*v as u16 as i16),
            dt::INT32 => f64::from(*v as i32),
            _ => f64::from(*v),
        }),
        MetricValue::LongValue(v) => Value::Num(match datatype {
            dt::INT64 => *v as i64 as f64,
            dt::DATETIME => *v as f64,
            _ => *v as f64,
        }),
        MetricValue::FloatValue(v) => Value::Num(f64::from(*v)),
        MetricValue::DoubleValue(v) => Value::Num(*v),
        MetricValue::BooleanValue(b) => Value::Bool(*b),
        MetricValue::StringValue(s) => Value::Text(s.clone()),
        MetricValue::BytesValue(_) => return None,
    })
}

/// Decoding state of one group: aliases and metric names learned from the births,
/// sequence numbers.
#[derive(Debug, Default)]
pub struct Decoder {
    group: String,
    /// `(edge, device)` → alias → metric name.
    aliases: BTreeMap<(String, Option<String>), BTreeMap<u64, String>>,
    /// `(edge, device)` → metric names (for the deaths).
    names: BTreeMap<(String, Option<String>), BTreeSet<String>>,
    last_seq: BTreeMap<String, u64>,
    /// Messages whose `seq` did not follow the previous one.
    pub seq_gaps: u64,
    /// Data with an alias never seen in a birth.
    pub unknown_aliases: u64,
    pub decode_errors: u64,
}

impl Decoder {
    pub fn new(group: &str) -> Decoder {
        Decoder {
            group: group.to_string(),
            ..Decoder::default()
        }
    }

    fn tag(&self, edge: &str, device: &Option<String>, metric: &str) -> String {
        match device {
            Some(d) => format!("{}/{edge}/{d}/{metric}", self.group),
            None => format!("{}/{edge}/{metric}", self.group),
        }
    }

    fn check_seq(&mut self, edge: &str, seq: Option<u64>, birth: bool) {
        let Some(seq) = seq else { return };
        if let Some(prev) = self.last_seq.get(edge) {
            if !birth && seq != (prev + 1) % 256 {
                self.seq_gaps += 1;
            }
        }
        self.last_seq.insert(edge.to_string(), seq);
    }

    /// Decodes one MQTT message. Messages of another group or of other types give
    /// nothing.
    pub fn handle(&mut self, topic: &str, payload: &[u8]) -> Result<Vec<Reading>, String> {
        let Some(t) = parse_topic(topic) else {
            return Ok(Vec::new());
        };
        if t.group != self.group || t.kind == MessageType::Other {
            return Ok(Vec::new());
        }
        let p = <Payload as prost::Message>::decode(payload).map_err(|e| {
            self.decode_errors += 1;
            format!("{topic}: carga Sparkplug inválida: {e}")
        })?;
        let key = (t.edge.clone(), t.device.clone());
        let mut out = Vec::new();
        match t.kind {
            MessageType::NBirth | MessageType::DBirth => {
                if t.kind == MessageType::NBirth {
                    // A new session of the node: its devices are born again too.
                    self.aliases.retain(|(e, _), _| *e != t.edge);
                    self.names.retain(|(e, _), _| *e != t.edge);
                }
                self.check_seq(&t.edge, p.seq, t.kind == MessageType::NBirth);
                let mut aliases = BTreeMap::new();
                let mut names = BTreeSet::new();
                for m in &p.metrics {
                    let Some(name) = &m.name else { continue };
                    if let Some(a) = m.alias {
                        aliases.insert(a, name.clone());
                    }
                    names.insert(name.clone());
                    if name == "bdSeq" || name.starts_with("Node Control/") {
                        continue;
                    }
                    if let Some(v) = convert(m) {
                        out.push(Reading {
                            tag: self.tag(&t.edge, &t.device, name),
                            value: v,
                            quality: Quality::Good,
                            timestamp: m.timestamp.or(p.timestamp),
                        });
                    }
                }
                self.aliases.insert(key.clone(), aliases);
                self.names.insert(key, names);
            }
            MessageType::NData | MessageType::DData => {
                self.check_seq(&t.edge, p.seq, false);
                for m in &p.metrics {
                    let name = match (&m.name, m.alias) {
                        (Some(n), _) => n.clone(),
                        (None, Some(a)) => match self.aliases.get(&key).and_then(|x| x.get(&a)) {
                            Some(n) => n.clone(),
                            None => {
                                self.unknown_aliases += 1;
                                continue;
                            }
                        },
                        (None, None) => continue,
                    };
                    if m.is_historical == Some(true) {
                        continue;
                    }
                    if let Some(v) = convert(m) {
                        out.push(Reading {
                            tag: self.tag(&t.edge, &t.device, &name),
                            value: v,
                            quality: Quality::Good,
                            timestamp: m.timestamp.or(p.timestamp),
                        });
                    }
                }
            }
            MessageType::NDeath | MessageType::DDeath => {
                let lost: Vec<(String, Option<String>)> = self
                    .names
                    .keys()
                    .filter(|(e, d)| {
                        *e == t.edge && (t.kind == MessageType::NDeath || *d == t.device)
                    })
                    .cloned()
                    .collect();
                for k in lost {
                    for name in &self.names[&k] {
                        if name == "bdSeq" || name.starts_with("Node Control/") {
                            continue;
                        }
                        out.push(Reading {
                            tag: self.tag(&k.0, &k.1, name),
                            value: Value::Bool(false),
                            quality: Quality::Bad,
                            timestamp: p.timestamp,
                        });
                    }
                }
                if t.kind == MessageType::NDeath {
                    self.last_seq.remove(&t.edge);
                }
            }
            MessageType::Other => {}
        }
        Ok(out)
    }
}
