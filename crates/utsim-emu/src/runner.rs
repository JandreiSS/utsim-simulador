//! The emulation loop: paced advance of the simulation, external commands, command log.
//!
//! The simulation stays single-threaded. Everything that comes from outside arrives as
//! [`Input`] values, applied between two `run_until` calls at the current simulated
//! time. Logging `(time, variable, value)` is therefore enough to reproduce a run.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use utsim_core::{BlockId, RunReport, Simulation, Step, TraceRecord};

use crate::commands::{CommandLog, Input, VAR_SPEED, VAR_STOP_SUFFIX, Value};
use crate::pacing::{Clock, Pacer};

#[derive(Debug, thiserror::Error)]
pub enum EmuError {
    #[error("{0}")]
    Msg(String),
    #[error(transparent)]
    Log(#[from] crate::commands::LogError),
}

impl From<utsim_core::SimError> for EmuError {
    fn from(e: utsim_core::SimError) -> Self {
        EmuError::Msg(e.to_string())
    }
}

/// Live state of one block, as published to the outside world.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockSnapshot {
    pub name: String,
    pub content: u32,
    pub state: String,
    pub inputs: u64,
    pub outputs: u64,
    /// True while an external stop command is in force.
    pub stopped: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub time: f64,
    pub speed: f64,
    pub blocks: Vec<BlockSnapshot>,
}

/// The outside world: where commands come from and where snapshots go.
pub trait Frontend {
    /// Writes received since the last call, in arrival order.
    fn poll(&mut self) -> Vec<Input>;
    fn publish(&mut self, snapshot: &Snapshot);

    /// Whether this frontend wants the trace records of the run (the live 3D scene).
    /// The emulator must have been created with [`Emulator::new_traced`].
    fn wants_trace(&self) -> bool {
        false
    }

    /// The trace records produced since the last call, after the simulation advanced to
    /// `time`. Called before [`Frontend::publish`]; only when [`Frontend::wants_trace`].
    fn trace(&mut self, _time: f64, _records: Vec<TraceRecord>) {}
}

/// A frontend that does nothing (replay, tests).
pub struct NullFrontend;

impl Frontend for NullFrontend {
    fn poll(&mut self) -> Vec<Input> {
        Vec::new()
    }

    fn publish(&mut self, _snapshot: &Snapshot) {}
}

/// A simulation plus the bookkeeping needed to command it by variable name.
pub struct Emulator {
    sim: Simulation,
    ids: Vec<BlockId>,
    names: Vec<String>,
    stopped: Vec<bool>,
}

impl Emulator {
    pub fn new(sim: Simulation) -> Result<Self, EmuError> {
        Self::start(sim, false)
    }

    /// Like [`Emulator::new`], recording the trace from the very first event so that
    /// [`Emulator::drain_trace`] returns all of it (what the live 3D scene consumes).
    pub fn new_traced(sim: Simulation) -> Result<Self, EmuError> {
        Self::start(sim, true)
    }

    fn start(mut sim: Simulation, traced: bool) -> Result<Self, EmuError> {
        if traced {
            sim.enable_trace();
        }
        sim.begin()?;
        let names: Vec<String> = sim.block_status().into_iter().map(|b| b.0).collect();
        let ids = names
            .iter()
            .map(|n| sim.block_id(n).expect("block listed by block_status"))
            .collect();
        let stopped = vec![false; names.len()];
        Ok(Emulator {
            sim,
            ids,
            names,
            stopped,
        })
    }

    pub fn sim(&self) -> &Simulation {
        &self.sim
    }

    pub fn block_names(&self) -> &[String] {
        &self.names
    }

    pub fn time(&self) -> f64 {
        self.sim.time()
    }

    /// Trace records produced since the last call (empty unless the emulator was created
    /// with [`Emulator::new_traced`]).
    pub fn drain_trace(&mut self) -> Vec<TraceRecord> {
        let records = self.sim.take_trace();
        self.sim.enable_trace();
        records
    }

    /// Applies a block command (`<block>.Parar`). Returns `Ok(false)` for variables that
    /// are not block commands (the speed), so the caller can handle them.
    pub fn apply(&mut self, input: &Input) -> Result<bool, EmuError> {
        let Some(block) = input.variable.strip_suffix(VAR_STOP_SUFFIX) else {
            return Ok(false);
        };
        let Some(i) = self.names.iter().position(|n| n == block) else {
            return Err(EmuError::Msg(format!("bloco desconhecido: {block}")));
        };
        let Value::Bool(stop) = input.value else {
            return Err(EmuError::Msg(format!(
                "{}: esperado um valor booleano",
                input.variable
            )));
        };
        if stop {
            self.sim.stop_block(self.ids[i]);
        } else {
            self.sim.resume_block(self.ids[i]);
        }
        self.stopped[i] = stop;
        Ok(true)
    }

    pub fn advance(&mut self, t: f64) -> Result<Step, EmuError> {
        match self.sim.run_until(t)? {
            Step::Decision(_) => Err(EmuError::Msg(
                "o modelo pede decisões externas (agente); a emulação não suporta isso".into(),
            )),
            s => Ok(s),
        }
    }

    pub fn snapshot(&self, speed: f64) -> Snapshot {
        let report = self.sim.report();
        let status = self.sim.block_status();
        let blocks = status
            .into_iter()
            .enumerate()
            .map(|(i, (name, state, content))| {
                let (inputs, outputs) = report
                    .block(&name)
                    .map(|b| (b.input, b.output))
                    .unwrap_or((0, 0));
                BlockSnapshot {
                    name,
                    content: content as u32,
                    state: state.map(|s| s.name().to_string()).unwrap_or_default(),
                    inputs,
                    outputs,
                    stopped: self.stopped[i],
                }
            })
            .collect();
        Snapshot {
            time: self.sim.time(),
            speed,
            blocks,
        }
    }

    pub fn finish(&mut self) -> RunReport {
        self.sim.finish()
    }
}

#[derive(Clone, Debug)]
pub struct RunOptions {
    /// 1.0 = real time, 10.0 = ten times faster.
    pub speed: f64,
    /// Length of one pacing tick (wall time).
    pub tick: Duration,
    /// Simulated time at which to stop (default: the model's stop time).
    pub until: Option<f64>,
    /// Set to true from another thread to end the run early.
    pub cancel: Option<Arc<AtomicBool>>,
}

impl Default for RunOptions {
    fn default() -> Self {
        RunOptions {
            speed: 1.0,
            tick: Duration::from_millis(50),
            until: None,
            cancel: None,
        }
    }
}

pub struct RunOutcome {
    pub report: RunReport,
    pub log: CommandLog,
}

/// Runs the paced emulation loop until the end time (or cancellation).
pub fn run_paced(
    emu: &mut Emulator,
    clock: &dyn Clock,
    frontend: &mut dyn Frontend,
    opts: &RunOptions,
) -> Result<RunOutcome, EmuError> {
    let end = opts.until.unwrap_or(f64::INFINITY);
    let mut pacer = Pacer::new(opts.speed, clock.now(), emu.time());
    let mut log = CommandLog::default();
    let mut deadline = clock.now();
    loop {
        if opts
            .cancel
            .as_ref()
            .is_some_and(|c| c.load(Ordering::Relaxed))
        {
            break;
        }
        // 1. External writes, applied at the current simulated time.
        for input in frontend.poll() {
            let now = emu.time();
            if input.variable == VAR_SPEED {
                match input.value {
                    Value::Num(v) if v.is_finite() && v > 0.0 => {
                        pacer.set_speed(v, clock.now(), now);
                    }
                    _ => continue,
                }
            } else if let Err(e) = emu.apply(&input) {
                eprintln!("aviso: comando ignorado: {e}");
                continue;
            }
            log.record(now, &input);
        }
        // 2. Advance to where the wall clock says we should be.
        let target = pacer.target(clock.now()).min(end);
        let step = emu.advance(target)?;
        if frontend.wants_trace() {
            let records = emu.drain_trace();
            frontend.trace(emu.time(), records);
        }
        frontend.publish(&emu.snapshot(pacer.speed()));
        if matches!(step, Step::Finished) || target >= end {
            break;
        }
        pacer.resync_if_late(clock.now(), emu.time());
        // 3. Wait for the next tick.
        deadline += opts.tick;
        let now = clock.now();
        if deadline > now {
            clock.sleep(deadline - now);
        } else {
            deadline = now;
        }
    }
    Ok(RunOutcome {
        report: emu.finish(),
        log,
    })
}

/// Replays a command log without wall-clock pacing: same commands at the same simulated
/// times. Speed commands are ignored (they do not affect the simulation).
pub fn replay(
    emu: &mut Emulator,
    log: &CommandLog,
    until: Option<f64>,
) -> Result<RunReport, EmuError> {
    let end = until.unwrap_or(f64::INFINITY);
    for c in &log.commands {
        if c.time > end {
            break;
        }
        emu.advance(c.time)?;
        let input = Input {
            variable: c.variable.clone(),
            value: c.value.clone(),
        };
        if c.variable != VAR_SPEED {
            emu.apply(&input)?;
        }
    }
    emu.advance(end)?;
    Ok(emu.finish())
}
