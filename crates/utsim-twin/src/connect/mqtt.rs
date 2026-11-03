//! MQTT client (3.1.1, QoS 0, subscribe only), for simple payloads and for Sparkplug B.
//!
//! Simple payloads: a number, `true`/`false`, a text, or a JSON document (the mapping
//! picks the field with `field`). The tag is the topic.

use std::thread::JoinHandle;
use std::time::Duration;

use rumqttc::{Client, Event, MqttOptions, Packet, QoS};

use super::{Connector, Sink};
use crate::obs::{Quality, Value};
use crate::sparkplug::Decoder;

/// Longest payload accepted (bigger ones are dropped and counted as invalid).
pub const MAX_PAYLOAD: usize = 256 * 1024;

pub struct MqttConnector {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub topics: Vec<String>,
    pub client_id: Option<String>,
    pub credentials_env: Option<String>,
    /// Decode as Sparkplug B for this group.
    pub sparkplug_group: Option<String>,
}

/// The value of a simple MQTT payload.
pub fn value_of_payload(payload: &[u8]) -> Option<Value> {
    let text = std::str::from_utf8(payload).ok()?.trim();
    Some(match serde_json::from_str::<serde_json::Value>(text) {
        Ok(serde_json::Value::Bool(b)) => Value::Bool(b),
        Ok(serde_json::Value::Number(n)) => Value::Num(n.as_f64()?),
        Ok(serde_json::Value::String(s)) => Value::Text(s),
        Ok(serde_json::Value::Null) => return None,
        Ok(other) => Value::Text(other.to_string()),
        Err(_) => Value::Text(text.to_string()),
    })
}

impl Connector for MqttConnector {
    fn name(&self) -> &str {
        &self.name
    }

    fn start(self: Box<Self>, sink: Sink) -> Result<JoinHandle<()>, String> {
        let id = self
            .client_id
            .clone()
            .unwrap_or_else(|| format!("utsim-gemeo-{}", std::process::id()));
        let mut opts = MqttOptions::new(id, self.host.clone(), self.port);
        opts.set_keep_alive(Duration::from_secs(30));
        opts.set_clean_session(true);
        opts.set_max_packet_size(MAX_PAYLOAD + 1024, 1024);
        if let Some(var) = &self.credentials_env {
            let cred = std::env::var(var)
                .map_err(|_| format!("conexão '{}': variável {var} não definida", self.name))?;
            let (u, p) = cred.split_once(':').unwrap_or((cred.as_str(), ""));
            opts.set_credentials(u, p);
        }
        let (client, mut connection) = Client::new(opts, 64);
        Ok(std::thread::spawn(move || {
            let mut decoder = self.sparkplug_group.as_deref().map(Decoder::new);
            let mut wait = Duration::from_millis(200);
            while !sink.cancelled() {
                let event = match connection.recv_timeout(Duration::from_millis(100)) {
                    Err(rumqttc::RecvTimeoutError::Timeout) => continue,
                    Err(rumqttc::RecvTimeoutError::Disconnected) => break,
                    Ok(e) => e,
                };
                match event {
                    Ok(Event::Incoming(Packet::ConnAck(_))) => {
                        wait = Duration::from_millis(200);
                        // (Re)subscribe on every connection: the session is clean.
                        for t in &self.topics {
                            if let Err(e) = client.try_subscribe(t.clone(), QoS::AtMostOnce) {
                                sink.error(format!("assinatura de {t}: {e}"));
                            }
                        }
                        sink.set_connected(true);
                    }
                    Ok(Event::Incoming(Packet::Publish(p))) => {
                        if p.payload.len() > MAX_PAYLOAD {
                            sink.update(|h| h.invalidas += 1);
                            continue;
                        }
                        match &mut decoder {
                            Some(d) => match d.handle(&p.topic, &p.payload) {
                                Ok(readings) => {
                                    for r in readings {
                                        sink.send(
                                            r.tag,
                                            r.value,
                                            r.quality,
                                            r.timestamp.map(|ms| ms * 1000),
                                        );
                                    }
                                    let gaps = d.seq_gaps;
                                    sink.update(|h| h.lacunas_seq = gaps);
                                }
                                Err(e) => {
                                    sink.update(|h| h.invalidas += 1);
                                    sink.error(e);
                                }
                            },
                            None => match value_of_payload(&p.payload) {
                                Some(v) => sink.send(p.topic.clone(), v, Quality::Good, None),
                                None => sink.update(|h| h.invalidas += 1),
                            },
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        sink.set_connected(false);
                        sink.error(format!("MQTT: {e}"));
                        let until = std::time::Instant::now() + wait;
                        while std::time::Instant::now() < until && !sink.cancelled() {
                            std::thread::sleep(Duration::from_millis(50));
                        }
                        wait = (wait * 2).min(Duration::from_secs(30));
                    }
                }
            }
            let _ = client.try_disconnect();
            sink.set_connected(false);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_payloads() {
        assert_eq!(value_of_payload(b" 12.5 "), Some(Value::Num(12.5)));
        assert_eq!(value_of_payload(b"true"), Some(Value::Bool(true)));
        assert_eq!(
            value_of_payload(b"\"down\""),
            Some(Value::Text("down".into()))
        );
        assert_eq!(value_of_payload(b"down"), Some(Value::Text("down".into())));
        assert_eq!(
            value_of_payload(br#"{"valor": 3}"#),
            Some(Value::Text(r#"{"valor":3}"#.into()))
        );
        assert_eq!(value_of_payload(b"null"), None);
        assert_eq!(value_of_payload(&[0xff, 0xfe]), None);
    }
}
