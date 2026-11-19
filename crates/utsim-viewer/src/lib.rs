//! Read-only 3D viewer for UTSim replays.
//!
//! The library holds the pure logic (no window, no GPU) so it can be unit-tested. The
//! scene itself (replay index [`timeline`], block placement [`layout`], colors/shapes
//! [`style`], and the composition of every pose at time `t`) lives in the
//! engine-independent `utsim-scene` crate and is re-exported here; loading ([`source`]),
//! editing and the panels stay in this crate. The Bevy application lives in the `utsim-view`
//! binary and only ever *reads* a [`utsim_model::replay::Replay`]: the simulation is
//! never driven by the viewer.

pub mod cameras;
pub mod editing;
pub mod flows;
pub mod flows_ui;
pub mod lod_rules;
pub mod picking;
pub mod properties;
pub mod results;
pub mod roteiro;
pub mod source;
pub mod texto;
pub mod view;

// Pure scene modules, moved to the engine-independent `utsim-scene` crate and
// re-exported so the viewer keeps its paths.
pub use utsim_scene::{fluids, geometry, handling, layout, style, timeline};
// Appearance rules and the third-party catalogue, shared with the Godot bridge.
pub use utsim_look::{catalogue as terceiros, visual};
