//! OPC UA client: one subscription with a monitored item per mapped node. Read-only: the
//! client never calls Write nor Call. Reconnects with a growing wait when the server
//! goes away.

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use opcua::client::{ClientBuilder, DataChangeCallback, IdentityToken, Session};
use opcua::crypto::SecurityPolicy;
use opcua::types::{
    MessageSecurityMode, MonitoredItemCreateRequest, MonitoringMode, MonitoringParameters, NodeId,
    ReadValueId, TimestampsToReturn, Variant,
};

use super::{Connector, Sink};
use crate::config::OpcSecurity;
use crate::obs::{Quality, Value};

pub struct OpcUaConnector {
    pub name: String,
    pub endpoint: String,
    pub namespace: Option<String>,
    pub interval_ms: u64,
    pub security: OpcSecurity,
    pub trust_server: bool,
    pub tags: Vec<String>,
}

/// The value of an OPC UA variant, when it is a scalar the twin understands.
pub fn value_of(v: &Variant) -> Option<Value> {
    Some(match v {
        Variant::Boolean(b) => Value::Bool(*b),
        Variant::SByte(x) => Value::Num(f64::from(*x)),
        Variant::Byte(x) => Value::Num(f64::from(*x)),
        Variant::Int16(x) => Value::Num(f64::from(*x)),
        Variant::UInt16(x) => Value::Num(f64::from(*x)),
        Variant::Int32(x) => Value::Num(f64::from(*x)),
        Variant::UInt32(x) => Value::Num(f64::from(*x)),
        Variant::Int64(x) => Value::Num(*x as f64),
        Variant::UInt64(x) => Value::Num(*x as f64),
        Variant::Float(x) => Value::Num(f64::from(*x)),
        Variant::Double(x) => Value::Num(*x),
        Variant::String(s) => Value::Text(s.value().clone().unwrap_or_default()),
        _ => return None,
    })
}

/// Microseconds since 1970 of an OPC UA timestamp (100 ns ticks since 1601).
pub fn unix_us(t: &opcua::types::DateTime) -> Option<u64> {
    const EPOCH_1970: i64 = 116_444_736_000_000_000;
    if t.is_null() {
        return None;
    }
    u64::try_from((t.ticks() - EPOCH_1970) / 10).ok()
}

/// Node id of a tag: a full id (`ns=2;s=M1.Estado`, `i=2258`) or a string identifier in
/// the namespace `ns`.
pub fn node_of(tag: &str, ns: Option<u16>) -> Result<NodeId, String> {
    let full = ["ns=", "i=", "s=", "g=", "b="]
        .iter()
        .any(|p| tag.starts_with(p));
    if full {
        NodeId::from_str(tag).map_err(|_| format!("node id inválido: {tag}"))
    } else {
        match ns {
            Some(ns) => Ok(NodeId::new(ns, tag.to_string())),
            None => Err(format!(
                "tag '{tag}' sem ns= e a conexão não declara namespace"
            )),
        }
    }
}

impl Connector for OpcUaConnector {
    fn name(&self) -> &str {
        &self.name
    }

    fn start(self: Box<Self>, sink: Sink) -> Result<JoinHandle<()>, String> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .map_err(|e| format!("runtime tokio: {e}"))?;
        Ok(std::thread::spawn(move || {
            rt.block_on(async move {
                let mut wait = Duration::from_millis(500);
                while !sink.cancelled() {
                    match self.session(&sink).await {
                        Ok(()) => break,
                        Err(e) => {
                            sink.set_connected(false);
                            sink.error(e);
                        }
                    }
                    let until = tokio::time::Instant::now() + wait;
                    while tokio::time::Instant::now() < until && !sink.cancelled() {
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                    wait = (wait * 2).min(Duration::from_secs(30));
                }
            });
        }))
    }
}

impl OpcUaConnector {
    /// One session: connect, subscribe, wait until cancelled or disconnected.
    async fn session(&self, sink: &Sink) -> Result<(), String> {
        let pki = std::env::temp_dir().join(format!("utsim-gemeo-pki-{}", std::process::id()));
        let mut client = ClientBuilder::new()
            .application_name("UTSim gemeo")
            .application_uri("urn:utsim:gemeo")
            .pki_dir(pki)
            .create_sample_keypair(true)
            .trust_server_certs(self.trust_server || self.security == OpcSecurity::None)
            .session_retry_limit(0)
            .client()
            .map_err(|e| format!("cliente OPC UA: {}", e.join("; ")))?;
        let (policy, mode) = match self.security {
            OpcSecurity::None => (SecurityPolicy::None, MessageSecurityMode::None),
            OpcSecurity::SignAndEncrypt => (
                SecurityPolicy::Basic256Sha256,
                MessageSecurityMode::SignAndEncrypt,
            ),
        };
        let (session, event_loop) = client
            .connect_to_matching_endpoint(
                (self.endpoint.as_str(), policy.to_str(), mode),
                IdentityToken::Anonymous,
            )
            .await
            .map_err(|e| format!("{}: {e}", self.endpoint))?;
        let handle = event_loop.spawn();
        let connected =
            tokio::time::timeout(Duration::from_secs(10), session.wait_for_connection()).await;
        if !matches!(connected, Ok(true)) {
            handle.abort();
            return Err(format!("{}: sem conexão", self.endpoint));
        }
        let result = self.subscribe(&session, sink).await;
        if result.is_ok() {
            sink.set_connected(true);
            while !sink.cancelled() && !handle.is_finished() {
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        }
        let _ = session.disconnect().await;
        handle.abort();
        sink.set_connected(false);
        if sink.cancelled() {
            result
        } else {
            result.and(Err(format!("{}: conexão perdida", self.endpoint)))
        }
    }

    async fn subscribe(&self, session: &Arc<Session>, sink: &Sink) -> Result<(), String> {
        let ns = match &self.namespace {
            Some(uri) => {
                let map = session
                    .read_namespace_array()
                    .await
                    .map_err(|e| format!("namespaces: {e}"))?;
                Some(
                    map.known_namespaces()
                        .get(uri)
                        .copied()
                        .ok_or_else(|| format!("namespace '{uri}' não existe no servidor"))?,
                )
            }
            None => None,
        };
        let mut by_node: HashMap<NodeId, String> = HashMap::new();
        let mut items = Vec::new();
        for tag in &self.tags {
            let node = node_of(tag, ns)?;
            by_node.insert(node.clone(), tag.clone());
            items.push(MonitoredItemCreateRequest::new(
                ReadValueId::from(node),
                MonitoringMode::Reporting,
                MonitoringParameters {
                    sampling_interval: self.interval_ms as f64,
                    queue_size: 100,
                    discard_oldest: true,
                    ..Default::default()
                },
            ));
        }
        let cb_sink = sink.clone();
        let sub = session
            .create_subscription(
                Duration::from_millis(self.interval_ms),
                60,
                20,
                0,
                0,
                true,
                DataChangeCallback::new(move |dv, item| {
                    let node = &item.item_to_monitor().node_id;
                    let Some(tag) = by_node.get(node) else { return };
                    let quality = match dv.status {
                        Some(s) if s.is_bad() => Quality::Bad,
                        Some(s) if s.is_uncertain() => Quality::Uncertain,
                        _ => Quality::Good,
                    };
                    let ts = dv.source_timestamp.as_ref().and_then(unix_us);
                    match dv.value.as_ref().and_then(value_of) {
                        Some(v) => cb_sink.send(tag.clone(), v, quality, ts),
                        None => cb_sink.send(tag.clone(), Value::Bool(false), Quality::Bad, ts),
                    }
                }),
            )
            .await
            .map_err(|e| format!("assinatura: {e}"))?;
        let results = session
            .create_monitored_items(sub, TimestampsToReturn::Both, items)
            .await
            .map_err(|e| format!("itens monitorados: {e}"))?;
        let bad: Vec<String> = results
            .iter()
            .zip(&self.tags)
            .filter(|(r, _)| r.result.status_code.is_bad())
            .map(|(r, t)| format!("{t} ({})", r.result.status_code))
            .collect();
        if !bad.is_empty() {
            sink.error(format!("nós recusados pelo servidor: {}", bad.join(", ")));
        }
        Ok(())
    }
}
