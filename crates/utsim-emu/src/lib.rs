//! Emulation (virtual commissioning) of UTSim models over OPC UA.
//!
//! A real-time runner paces the simulation against the wall clock, an OPC UA server
//! exposes block variables and accepts commands, and a command log makes every run
//! reproducible without the network.

pub mod commands;
pub mod pacing;
pub mod runner;
pub mod server;

pub use commands::{CommandLog, Input, LoggedCommand, Value};
pub use pacing::{Clock, FakeClock, Pacer, SystemClock};
pub use runner::{
    BlockSnapshot, EmuError, Emulator, Frontend, NullFrontend, RunOptions, RunOutcome, Snapshot,
    replay, run_paced,
};
pub use server::OpcUaFrontend;
