//! Read-only 3D viewer for UTSim replays.
//!
//! The library holds the pure logic (no window, no GPU) so it can be unit-tested:
//! the replay index ([`timeline`]), block placement ([`layout`]), loading ([`source`])
//! and colors/shapes ([`style`]). The Bevy application lives in the `utsim-view`
//! binary and only ever *reads* a [`utsim_model::replay::Replay`]: the simulation is
//! never driven by the viewer.

pub mod editing;
pub mod flows;
pub mod flows_ui;
pub mod fluids;
pub mod geometry;
pub mod handling;
pub mod layout;
pub mod properties;
pub mod roteiro;
pub mod source;
pub mod style;
pub mod timeline;
pub mod view;
