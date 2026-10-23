//! OPC UA server exposing the simulation (address space `UTSim/<model>/...`).
//!
//! The server runs on its own tokio runtime. It talks to the simulation loop only
//! through a channel (writes) and `set_values` calls (publications), so the simulation
//! itself stays single-threaded.

use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};

use opcua::server::address_space::VariableBuilder;
use opcua::server::node_manager::memory::{SimpleNodeManager, simple_node_manager};
use opcua::server::{ServerBuilder, ServerHandle};
use opcua::types::{DataTypeId, DataValue, NodeId, ObjectId, StatusCode, UAString, Variant};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

use crate::commands::{Input, VAR_SPEED, VAR_STOP_SUFFIX, Value};
use crate::runner::{EmuError, Frontend, Snapshot};

const NAMESPACE_URI: &str = "urn:utsim:emulacao";

/// Browse names of the per-block variables.
pub const VAR_CONTENT: &str = "Conteudo";
pub const VAR_STATE: &str = "Estado";
pub const VAR_INPUTS: &str = "Entradas";
pub const VAR_OUTPUTS: &str = "Saidas";
pub const VAR_TIME: &str = "TempoSimulado";

struct BlockNodes {
    content: NodeId,
    state: NodeId,
    inputs: NodeId,
    outputs: NodeId,
    stop: NodeId,
}

pub struct OpcUaFrontend {
    runtime: Runtime,
    handle: ServerHandle,
    manager: Arc<SimpleNodeManager>,
    rx: Receiver<Input>,
    blocks: Vec<BlockNodes>,
    time: NodeId,
    speed: NodeId,
    endpoint: String,
    /// Last value written to each node, to publish only changes.
    last: Vec<(NodeId, Variant)>,
}

impl OpcUaFrontend {
    /// Starts the server on `127.0.0.1:port` (0 picks a free port).
    pub fn start(
        model: &str,
        block_names: &[String],
        port: u16,
        initial_speed: f64,
    ) -> Result<Self, EmuError> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|e| EmuError::Msg(format!("runtime tokio: {e}")))?;
        let _guard = runtime.enter();
        let listener = runtime
            .block_on(TcpListener::bind(("127.0.0.1", port)))
            .map_err(|e| EmuError::Msg(format!("não foi possível abrir a porta {port}: {e}")))?;
        let port = listener
            .local_addr()
            .map_err(|e| EmuError::Msg(e.to_string()))?
            .port();
        let endpoint = format!("opc.tcp://127.0.0.1:{port}/");

        let pki = std::env::temp_dir().join(format!("utsim-emu-pki-{port}"));
        let (server, handle) = ServerBuilder::new_anonymous("UTSim")
            .application_uri("urn:utsim:servidor")
            .product_uri("urn:utsim")
            .host("127.0.0.1")
            .port(port)
            .pki_dir(pki)
            .discovery_urls(vec![endpoint.clone()])
            .with_node_manager(simple_node_manager(
                opcua::server::diagnostics::NamespaceMetadata {
                    namespace_uri: NAMESPACE_URI.to_owned(),
                    ..Default::default()
                },
                "utsim",
            ))
            .build()
            .map_err(|e| EmuError::Msg(format!("servidor OPC UA: {e}")))?;
        let manager = handle
            .node_managers()
            .get_of_type::<SimpleNodeManager>()
            .ok_or_else(|| EmuError::Msg("gerenciador de nós não encontrado".into()))?;
        let ns = handle
            .get_namespace_index(NAMESPACE_URI)
            .ok_or_else(|| EmuError::Msg("namespace não registrado".into()))?;

        let (tx, rx) = channel::<Input>();
        let tx = Arc::new(Mutex::new(tx));
        let mut blocks = Vec::new();
        let time;
        let speed;
        {
            let mut space = manager.address_space().write();
            let id = |path: &str| NodeId::new(ns, format!("UTSim/{model}/{path}"));
            let root = NodeId::new(ns, format!("UTSim/{model}"));
            let objects: NodeId = ObjectId::ObjectsFolder.into();
            let utsim_folder = NodeId::new(ns, "UTSim");
            space.add_folder(&utsim_folder, "UTSim", "UTSim", &objects);
            space.add_folder(&root, model, model, &utsim_folder);

            time = id(VAR_TIME);
            VariableBuilder::new(&time, VAR_TIME, VAR_TIME)
                .data_type(DataTypeId::Double)
                .value(0.0f64)
                .organized_by(&root)
                .insert(&mut *space);
            speed = id(VAR_SPEED);
            VariableBuilder::new(&speed, VAR_SPEED, VAR_SPEED)
                .data_type(DataTypeId::Double)
                .value(initial_speed)
                .writable()
                .organized_by(&root)
                .insert(&mut *space);

            for name in block_names {
                let folder = id(name);
                space.add_folder(&folder, name.as_str(), name.as_str(), &root);
                let mut var = |label: &str, dt: DataTypeId, v: Variant, writable: bool| {
                    let nid = id(&format!("{name}/{label}"));
                    let mut b = VariableBuilder::new(&nid, label, label)
                        .data_type(dt)
                        .value(v)
                        .organized_by(&folder);
                    if writable {
                        b = b.writable();
                    }
                    b.insert(&mut *space);
                    nid
                };
                blocks.push(BlockNodes {
                    content: var(VAR_CONTENT, DataTypeId::UInt32, 0u32.into(), false),
                    state: var(
                        VAR_STATE,
                        DataTypeId::String,
                        UAString::from("").into(),
                        false,
                    ),
                    inputs: var(VAR_INPUTS, DataTypeId::UInt64, 0u64.into(), false),
                    outputs: var(VAR_OUTPUTS, DataTypeId::UInt64, 0u64.into(), false),
                    stop: var("Parar", DataTypeId::Boolean, false.into(), true),
                });
            }
        }

        // Writes go to the simulation loop through the channel.
        let write_cb = |tx: Arc<Mutex<Sender<Input>>>, variable: String| {
            move |value: DataValue, _range: &opcua::types::NumericRange| -> StatusCode {
                let v = match (value.value, variable.as_str()) {
                    (Some(Variant::Boolean(b)), v) if v.ends_with(VAR_STOP_SUFFIX) => {
                        Value::Bool(b)
                    }
                    (Some(Variant::Double(d)), VAR_SPEED) if d.is_finite() && d > 0.0 => {
                        Value::Num(d)
                    }
                    (Some(Variant::Double(_)), VAR_SPEED) => return StatusCode::BadOutOfRange,
                    _ => return StatusCode::BadTypeMismatch,
                };
                let input = Input {
                    variable: variable.clone(),
                    value: v,
                };
                match tx.lock() {
                    Ok(tx) if tx.send(input).is_ok() => StatusCode::Good,
                    _ => StatusCode::BadInternalError,
                }
            }
        };
        manager
            .inner()
            .add_write_callback(speed.clone(), write_cb(tx.clone(), VAR_SPEED.to_string()));
        for (nodes, name) in blocks.iter().zip(block_names) {
            manager.inner().add_write_callback(
                nodes.stop.clone(),
                write_cb(tx.clone(), format!("{name}{VAR_STOP_SUFFIX}")),
            );
        }

        runtime.spawn(async move {
            if let Err(e) = server.run_with(listener).await {
                eprintln!("servidor OPC UA terminou com erro: {e}");
            }
        });

        Ok(OpcUaFrontend {
            runtime,
            handle,
            manager,
            rx,
            blocks,
            time,
            speed,
            endpoint,
            last: Vec::new(),
        })
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Sets `flag` when the process receives Ctrl-C (so the run ends cleanly).
    pub fn cancel_on_ctrl_c(&self, flag: Arc<std::sync::atomic::AtomicBool>) {
        self.runtime.spawn(async move {
            if tokio::signal::ctrl_c().await.is_ok() {
                flag.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        });
    }
}

impl Frontend for OpcUaFrontend {
    fn poll(&mut self) -> Vec<Input> {
        self.rx.try_iter().collect()
    }

    fn publish(&mut self, s: &Snapshot) {
        let mut updates: Vec<(NodeId, Variant)> = Vec::with_capacity(self.blocks.len() * 5 + 2);
        updates.push((self.time.clone(), s.time.into()));
        updates.push((self.speed.clone(), s.speed.into()));
        for (nodes, b) in self.blocks.iter().zip(&s.blocks) {
            updates.push((nodes.content.clone(), b.content.into()));
            updates.push((nodes.state.clone(), UAString::from(b.state.as_str()).into()));
            updates.push((nodes.inputs.clone(), b.inputs.into()));
            updates.push((nodes.outputs.clone(), b.outputs.into()));
            updates.push((nodes.stop.clone(), b.stopped.into()));
        }
        // Publish only what changed since the last tick.
        let changed: Vec<(NodeId, Variant)> = updates
            .into_iter()
            .filter(
                |(id, v)| match self.last.iter_mut().find(|(lid, _)| lid == id) {
                    Some((_, lv)) if lv == v => false,
                    Some((_, lv)) => {
                        *lv = v.clone();
                        true
                    }
                    None => {
                        self.last.push((id.clone(), v.clone()));
                        true
                    }
                },
            )
            .collect();
        if changed.is_empty() {
            return;
        }
        let _ = self.manager.set_values(
            self.handle.subscriptions(),
            changed
                .iter()
                .map(|(id, v)| (id, None, DataValue::new_now(v.clone()))),
        );
    }
}

impl Drop for OpcUaFrontend {
    fn drop(&mut self) {
        self.handle.cancel();
    }
}
