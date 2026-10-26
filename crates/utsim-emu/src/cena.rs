//! The live 3D scene as a frontend of the emulation.
//!
//! [`SceneFrontend`] serves the `utsim-scene/1` stream ([`utsim_scene::server`]) while
//! the emulation runs, on the same simulated clock as the OPC UA server: both are driven
//! by [`crate::run_paced`], so a VR client and a PLC see the same instant. It wraps
//! another frontend (the OPC UA server, or [`crate::NullFrontend`]) and adds the scene's
//! trace feed and the commands that clients send over the socket.

use std::io;
use std::net::ToSocketAddrs;

use utsim_core::TraceRecord;
use utsim_model::Model;
use utsim_scene::server::{LiveServer, ReceivedCommand};
use utsim_scene::{CommandValue, ServerMessage};

use crate::commands::{Input, VAR_SPEED, VAR_STOP_SUFFIX, Value};
use crate::runner::{Frontend, Snapshot};

/// Serves the live scene next to an inner frontend. Create the emulator with
/// [`crate::Emulator::new_traced`] so the scene sees the run from its first event.
pub struct SceneFrontend<F: Frontend> {
    inner: F,
    live: LiveServer,
    blocks: Vec<String>,
    time: f64,
}

impl<F: Frontend> SceneFrontend<F> {
    /// Starts listening on `addr`. `blocks` are the emulator's block names (to validate
    /// client commands); `fps` is the wall-clock frame rate sent to clients.
    pub fn start(
        inner: F,
        addr: impl ToSocketAddrs,
        model: &Model,
        blocks: &[String],
        fps: f64,
    ) -> io::Result<Self> {
        Ok(SceneFrontend {
            inner,
            live: LiveServer::start(addr, model, 0, fps)?,
            blocks: blocks.to_vec(),
            time: 0.0,
        })
    }

    pub fn live(&self) -> &LiveServer {
        &self.live
    }

    /// Converts a client command to an emulator input, or says why it is refused.
    fn input_of(&self, c: &ReceivedCommand) -> Result<Input, String> {
        let value = match c.value {
            CommandValue::Bool(b) => Value::Bool(b),
            CommandValue::Num(n) => Value::Num(n),
        };
        if c.variable == VAR_SPEED {
            return match value {
                Value::Num(v) if v.is_finite() && v > 0.0 => Ok(Input {
                    variable: c.variable.clone(),
                    value,
                }),
                _ => Err(format!("{VAR_SPEED}: esperado um número positivo")),
            };
        }
        let Some(block) = c.variable.strip_suffix(VAR_STOP_SUFFIX) else {
            return Err(format!(
                "variável desconhecida: {} (use <bloco>{VAR_STOP_SUFFIX} ou {VAR_SPEED})",
                c.variable
            ));
        };
        if !self.blocks.iter().any(|b| b == block) {
            return Err(format!("bloco desconhecido: {block}"));
        }
        if !matches!(value, Value::Bool(_)) {
            return Err(format!("{}: esperado um valor booleano", c.variable));
        }
        Ok(Input {
            variable: c.variable.clone(),
            value,
        })
    }
}

impl<F: Frontend> Frontend for SceneFrontend<F> {
    fn poll(&mut self) -> Vec<Input> {
        let mut inputs = self.inner.poll();
        for c in self.live.server().take_commands() {
            match self.input_of(&c) {
                Ok(input) => inputs.push(input),
                Err(mensagem) => self
                    .live
                    .server()
                    .send_to(c.client, &ServerMessage::Aviso { mensagem }),
            }
        }
        inputs
    }

    fn publish(&mut self, snapshot: &Snapshot) {
        self.time = snapshot.time;
        self.inner.publish(snapshot);
    }

    fn wants_trace(&self) -> bool {
        true
    }

    fn trace(&mut self, time: f64, records: Vec<TraceRecord>) {
        self.live.push(time, records);
    }
}

impl<F: Frontend> Drop for SceneFrontend<F> {
    fn drop(&mut self) {
        self.live.finish(self.time);
    }
}
