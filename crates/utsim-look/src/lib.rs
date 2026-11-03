//! Appearance of a UTSim scene without an engine: which asset each entity is drawn with
//! ([`visual`]), the third-party catalogue ([`catalogue`]) and the pure rules that move
//! the articulated parts (forks, wheels, light, clips of the people, carried loads).
//!
//! Both interfaces use it: the Bevy viewer (`utsim-viewer`) and the Godot bridge
//! (`utsim-godot`). They only apply the numbers to their own nodes, so the two draw the
//! same model the same way.

pub mod catalogue;
pub mod driver;
pub mod visual;

pub use visual::{Carry, Clip, Context, Look, Source, look_of};
