//! Connectors: each one reads one source in its own thread and sends what it reads to
//! the twin through a bounded channel (when full, readings are dropped and counted). None
//! of them writes to the plant: the OPC UA client only creates a subscription, the MQTT
//! clients only subscribe (Sparkplug commands and rebirth requests are never published).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::config::{SourceKind, TwinConfig};
use crate::obs::{Quality, RawObservation, Value};

#[cfg(feature = "mqtt")]
pub mod mqtt;
#[cfg(feature = "opcua")]
pub mod opcua;

/// Capacity of the channel from the connectors to the twin.
pub const CHANNEL_CAPACITY: usize = 100_000;

/// Connection state, shown on the panel.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Health {
    pub conectado: bool,
    pub recebidas: u64,
    pub descartadas: u64,
    pub ultimo_erro: Option<String>,
    /// Readings that could not be decoded (bad payloads, Sparkplug errors).
    pub invalidas: u64,
    /// Sparkplug: gaps in the sequence numbers.
    pub lacunas_seq: u64,
}

/// What connectors share: the channel, the cancel flag and the health.
#[derive(Clone)]
pub struct Sink {
    pub name: String,
    tx: SyncSender<RawObservation>,
    pub cancel: Arc<AtomicBool>,
    pub health: Arc<Mutex<Health>>,
    seq: Arc<Mutex<u64>>,
}

impl Sink {
    pub fn new(
        name: &str,
        tx: SyncSender<RawObservation>,
        cancel: Arc<AtomicBool>,
    ) -> (Sink, Arc<Mutex<Health>>) {
        let health = Arc::new(Mutex::new(Health::default()));
        (
            Sink {
                name: name.to_string(),
                tx,
                cancel,
                health: health.clone(),
                seq: Arc::new(Mutex::new(0)),
            },
            health,
        )
    }

    pub fn cancelled(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }

    /// Sends one reading (`source_us`: the source's timestamp, microseconds since 1970).
    pub fn send(&self, tag: String, value: Value, quality: Quality, source_us: Option<u64>) {
        let seq = {
            let mut s = self.seq.lock().expect("seq lock");
            *s += 1;
            *s
        };
        let obs = RawObservation {
            source: self.name.clone(),
            tag,
            value,
            quality,
            seq,
            at: Instant::now(),
            source_us,
        };
        let mut h = self.health.lock().expect("health lock");
        match self.tx.try_send(obs) {
            Ok(()) => h.recebidas += 1,
            Err(TrySendError::Full(_)) => h.descartadas += 1,
            Err(TrySendError::Disconnected(_)) => {}
        }
    }

    pub fn set_connected(&self, on: bool) {
        self.health.lock().expect("health lock").conectado = on;
    }

    pub fn error(&self, e: impl Into<String>) {
        let mut h = self.health.lock().expect("health lock");
        h.ultimo_erro = Some(e.into());
    }

    pub fn update(&self, f: impl FnOnce(&mut Health)) {
        f(&mut self.health.lock().expect("health lock"));
    }
}

/// A connector, ready to start.
pub trait Connector: Send {
    fn name(&self) -> &str;
    /// Starts the reading thread.
    fn start(self: Box<Self>, sink: Sink) -> Result<JoinHandle<()>, String>;
}

/// A connector fed by a channel (tests, or another program in the same process).
pub struct ChannelConnector {
    name: String,
    rx: Receiver<(String, Value, Quality)>,
}

impl ChannelConnector {
    pub fn new(name: &str, rx: Receiver<(String, Value, Quality)>) -> ChannelConnector {
        ChannelConnector {
            name: name.to_string(),
            rx,
        }
    }
}

impl Connector for ChannelConnector {
    fn name(&self) -> &str {
        &self.name
    }

    fn start(self: Box<Self>, sink: Sink) -> Result<JoinHandle<()>, String> {
        Ok(std::thread::spawn(move || {
            sink.set_connected(true);
            while !sink.cancelled() {
                match self.rx.recv_timeout(std::time::Duration::from_millis(20)) {
                    Ok((tag, v, q)) => sink.send(tag, v, q, None),
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
            sink.set_connected(false);
        }))
    }
}

/// The tags of each source (what the OPC UA client subscribes to), clock tag included.
pub fn tags_of(cfg: &TwinConfig, source: &str) -> Vec<String> {
    let mut v: Vec<String> = cfg
        .tags
        .iter()
        .filter(|r| r.source == source)
        .map(|r| r.tag.clone())
        .collect();
    if let crate::config::ClockSpec::Tag { source: s, tag } = &cfg.clock {
        if s == source {
            v.push(tag.clone());
        }
    }
    v.sort();
    v.dedup();
    v
}

/// Builds the connectors of the configuration.
pub fn build(cfg: &TwinConfig) -> Result<Vec<Box<dyn Connector>>, String> {
    let mut out: Vec<Box<dyn Connector>> = Vec::new();
    for s in &cfg.sources {
        match &s.kind {
            #[cfg(feature = "opcua")]
            SourceKind::OpcUa {
                endpoint,
                namespace,
                interval_ms,
                security,
                trust_server,
            } => out.push(Box::new(opcua::OpcUaConnector {
                name: s.name.clone(),
                endpoint: endpoint.clone(),
                namespace: namespace.clone(),
                interval_ms: *interval_ms,
                security: *security,
                trust_server: *trust_server,
                tags: tags_of(cfg, &s.name),
            })),
            #[cfg(feature = "mqtt")]
            SourceKind::Mqtt {
                host,
                port,
                topics,
                client_id,
                credentials_env,
            } => out.push(Box::new(mqtt::MqttConnector {
                name: s.name.clone(),
                host: host.clone(),
                port: *port,
                topics: topics.clone(),
                client_id: client_id.clone(),
                credentials_env: credentials_env.clone(),
                sparkplug_group: None,
            })),
            #[cfg(feature = "mqtt")]
            SourceKind::Sparkplug {
                host,
                port,
                group,
                client_id,
                credentials_env,
            } => out.push(Box::new(mqtt::MqttConnector {
                name: s.name.clone(),
                host: host.clone(),
                port: *port,
                topics: vec![format!("spBv1.0/{group}/#")],
                client_id: client_id.clone(),
                credentials_env: credentials_env.clone(),
                sparkplug_group: Some(group.clone()),
            })),
            #[allow(unreachable_patterns)]
            other => {
                return Err(format!(
                    "conexão '{}': este programa foi compilado sem suporte a {:?}",
                    s.name,
                    std::mem::discriminant(other)
                ));
            }
        }
    }
    Ok(out)
}
