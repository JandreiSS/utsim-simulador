//! GDExtension bridge between UTSim and the Godot interface (`godot/`).
//!
//! Three classes, all `RefCounted`, so GDScript owns them like any resource:
//! - `UtsimDocument` (`document_class.rs`): the model being edited, over
//!   `utsim_edit::Document` ([`document::Editor`]): commands with undo and redo, the
//!   problems and warnings, the object list and the properties of one object as a schema
//!   ([`schema`]) the panel draws generically;
//! - [`UtsimModel`]: loads and validates a `.ron`, lists errors and warnings, runs the
//!   replications and summarises them, and records the replay of one replication;
//! - [`UtsimPlayback`]: the scene of that replay ([`playback::Playback`]): the static
//!   layout once, as dictionaries, and [`UtsimPlayback::frame`] at any `t`, as packed
//!   arrays (one entry per block and per drawn item), which GDScript reads without
//!   allocating a dictionary per entity.
//!
//! Every position is converted to Godot coordinates here ([`coords`]): `(x, y, z)` of
//! the model becomes `(x, z, -y)` and a yaw is the rotation about `+Y`. The pure modules
//! ([`coords`], [`looks`], [`catalogue`], [`playback`], [`document`], [`schema`],
//! [`ron_text`]) have no Godot types and are unit tested without the engine.

// `#[derive(GodotClass)]` with `init` writes `base: base` in the generated constructor.
#![allow(clippy::redundant_field_names)]

pub mod catalogue;
pub mod coords;
pub mod document;
mod document_class;
pub mod looks;
pub mod playback;
pub mod ron_text;
pub mod schema;

use std::path::PathBuf;

use godot::prelude::*;
use utsim_model::{Model, ModelError};

use crate::looks::Clip;
use crate::playback::{AssetInfo, Playback, role_name, simple_shape};

struct UtsimExtension;

#[gdextension]
unsafe impl ExtensionLibrary for UtsimExtension {}

fn v3(p: [f32; 3]) -> Vector3 {
    Vector3::new(p[0], p[1], p[2])
}

fn rgb(c: [f32; 3]) -> Color {
    Color::from_rgb(c[0], c[1], c[2])
}

pub(crate) fn put(d: &mut VarDictionary, key: &str, value: impl ToGodot) {
    d.set(&key.to_variant(), &value.to_variant());
}

pub(crate) fn strings<S: AsRef<str>>(list: &[S]) -> PackedStringArray {
    let mut out = PackedStringArray::new();
    for s in list {
        out.push(&GString::from(s.as_ref()));
    }
    out
}

/// Error messages of a model error, one per line item.
fn messages(e: &ModelError) -> Vec<String> {
    match e {
        ModelError::Invalid(list) => list.clone(),
        other => vec![other.to_string()],
    }
}

/// A model file: load, validate, run, replay.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimModel {
    pub(crate) model: Option<Model>,
    pub(crate) path: Option<PathBuf>,
    pub(crate) last_error: String,
    pub(crate) base: Base<RefCounted>,
}

#[godot_api]
impl UtsimModel {
    /// Reads and validates `path`. Returns `{ok, errors, warnings, name, description,
    /// objects, replications, stop_time}`; the model is kept only when it is valid.
    #[func]
    fn load(&mut self, path: GString) -> VarDictionary {
        let path = PathBuf::from(path.to_string());
        let mut d = VarDictionary::new();
        self.model = None;
        self.path = None;
        let result = Model::load(&path).and_then(|m| m.validate().map(|()| m));
        match result {
            Ok(m) => {
                put(&mut d, "ok", true);
                put(&mut d, "errors", strings::<String>(&[]));
                put(&mut d, "warnings", strings(&m.warnings()));
                put(&mut d, "name", GString::from(m.name.as_str()));
                put(&mut d, "description", GString::from(m.description.as_str()));
                put(&mut d, "objects", m.objects.len() as i64);
                put(&mut d, "replications", m.run.replications as i64);
                put(&mut d, "stop_time", m.run.stop_time);
                self.model = Some(m);
                self.path = Some(path);
            }
            Err(e) => {
                put(&mut d, "ok", false);
                put(&mut d, "errors", strings(&messages(&e)));
                put(&mut d, "warnings", strings::<String>(&[]));
            }
        }
        d
    }

    #[func]
    fn is_loaded(&self) -> bool {
        self.model.is_some()
    }

    /// Last error of [`Self::run`] or [`Self::playback`].
    #[func]
    fn last_error(&self) -> GString {
        GString::from(self.last_error.as_str())
    }

    /// Runs `replications` replications (`0`: the model's own number), in parallel.
    /// Returns `{ok, error, replications, end_time, metrics}`, where `metrics` maps
    /// `"block.metric"` to `{mean, half_width, std_dev, n}` (the numbers of `utsim run`).
    #[func]
    fn run(&mut self, replications: i64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(model) = &self.model else {
            put(&mut d, "ok", false);
            put(&mut d, "error", "nenhum modelo carregado");
            return d;
        };
        let mut m = model.clone();
        if replications > 0 {
            m.run.replications = replications.min(10_000) as u32;
        }
        match m.run_replications() {
            Ok(reports) => {
                let summary = utsim_core::summarize(&reports);
                let mut metrics = VarDictionary::new();
                for (k, s) in &summary {
                    let mut row = VarDictionary::new();
                    put(&mut row, "mean", s.mean);
                    put(&mut row, "half_width", s.half_width);
                    put(&mut row, "std_dev", s.std_dev);
                    put(&mut row, "n", s.n as i64);
                    put(&mut metrics, k, row);
                }
                put(&mut d, "ok", true);
                put(&mut d, "replications", reports.len() as i64);
                put(
                    &mut d,
                    "end_time",
                    reports.first().map_or(0.0, |r| r.end_time),
                );
                put(&mut d, "metrics", metrics);
            }
            Err(e) => {
                self.last_error = e.to_string();
                put(&mut d, "ok", false);
                put(&mut d, "error", GString::from(self.last_error.as_str()));
            }
        }
        d
    }

    /// Records replication `replication` and builds its scene; `null` on error (see
    /// [`Self::last_error`]). `skeletal_people`: the interface has the mannequin files.
    #[func]
    fn playback(&mut self, replication: i64, skeletal_people: bool) -> Option<Gd<UtsimPlayback>> {
        let model = self.model.as_ref()?;
        match model.replay(replication.max(0) as u32) {
            Ok(replay) => {
                let dir = self.path.as_ref().and_then(|p| p.parent());
                let core = Playback::new(&replay, Some(model), dir, skeletal_people);
                Some(Gd::from_init_fn(|base| UtsimPlayback {
                    core: Some(core),
                    base,
                }))
            }
            Err(e) => {
                self.last_error = e.to_string();
                None
            }
        }
    }
}

/// The scene of one replay, frame by frame.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimPlayback {
    pub(crate) core: Option<Playback>,
    pub(crate) base: Base<RefCounted>,
}

#[godot_api]
impl UtsimPlayback {
    /// Names of the person clips in clip-index order; a `ual2/` prefix marks the clips of
    /// the second file (`pessoa_operador_carga`). Index 6 (`IdleCarry`) is the arms layer.
    #[func]
    fn person_clips() -> PackedStringArray {
        let names: Vec<String> = Clip::ALL
            .iter()
            .map(|c| match c.source() {
                (n, true) => format!("ual2/{n}"),
                (n, false) => n.to_string(),
            })
            .collect();
        strings(&names)
    }

    #[func]
    fn end(&self) -> f64 {
        self.core.as_ref().map_or(0.0, Playback::end)
    }

    /// Next event strictly after `t`, or -1.
    #[func]
    fn next_event(&self, t: f64) -> f64 {
        self.core
            .as_ref()
            .and_then(|c| c.next_event(t))
            .unwrap_or(-1.0)
    }

    /// Last event strictly before `t`, or -1.
    #[func]
    fn prev_event(&self, t: f64) -> f64 {
        self.core
            .as_ref()
            .and_then(|c| c.prev_event(t))
            .unwrap_or(-1.0)
    }

    #[func]
    fn set_clip_durations(&mut self, durations: PackedFloat32Array) {
        if let Some(c) = self.core.as_mut() {
            c.set_clip_durations(durations.as_slice());
        }
    }

    /// `.glb` of a library asset with non-default parameters (a rack of its own size).
    #[func]
    fn glb(&self, key: GString) -> PackedByteArray {
        let bytes = self
            .core
            .as_ref()
            .and_then(|c| c.glb(&key.to_string()))
            .unwrap_or_default();
        PackedByteArray::from(bytes.as_slice())
    }

    /// The layout: `{end, bounds_min, bounds_max, pallets, fork_size, blocks, network,
    /// network_controlled, connections, fluids}`. Each block is a dictionary with `name,
    /// kind, role, mobile, position, size, shape` and, when it has a realistic look,
    /// `asset {source, name, key, path}, scale, asset_yaw, offset, color, carry,
    /// skeletal`.
    #[func]
    fn static_scene(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = &self.core else {
            return d;
        };
        let header = c.scene.static_scene();
        let (a, b) = (
            coords::to_godot(header.bounds.0),
            coords::to_godot(header.bounds.1),
        );
        let lo = [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])];
        let hi = [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])];
        put(&mut d, "end", c.end());
        put(&mut d, "bounds_min", v3(lo));
        put(&mut d, "bounds_max", v3(hi));
        put(&mut d, "pallets", c.pallets);
        put(&mut d, "fork_size", c.fork_size);
        let mut blocks = VarArray::new();
        for (i, v) in c.blocks.iter().enumerate() {
            let (position, size) = c.static_pose(i);
            let mut e = VarDictionary::new();
            put(&mut e, "name", GString::from(v.name.as_str()));
            put(&mut e, "kind", GString::from(v.kind.as_str()));
            put(&mut e, "role", v.role.map_or("", role_name));
            put(&mut e, "mobile", v.role.is_some());
            put(&mut e, "position", v3(position));
            put(&mut e, "size", v3(size));
            put(&mut e, "shape", simple_shape(&v.kind));
            if let Some(l) = &v.look {
                let a = AssetInfo::of(&l.source);
                let mut asset = VarDictionary::new();
                put(&mut asset, "source", a.source);
                put(&mut asset, "name", GString::from(a.name.as_str()));
                put(&mut asset, "key", GString::from(a.key.as_str()));
                put(&mut asset, "path", GString::from(a.path.as_str()));
                put(&mut e, "asset", asset);
                put(&mut e, "scale", l.scale);
                put(&mut e, "asset_yaw", l.yaw);
                put(&mut e, "offset", v3(l.offset));
                if let Some(col) = l.color {
                    put(&mut e, "color", rgb(col));
                }
                put(&mut e, "carry", l.carry.name());
                put(&mut e, "skeletal", l.skeletal);
            }
            blocks.push(&e.to_variant());
        }
        put(&mut d, "blocks", blocks);
        let mut network = PackedVector3Array::new();
        let mut controlled = PackedInt32Array::new();
        for (a, b, ctl) in c.network() {
            network.push(v3(a));
            network.push(v3(b));
            controlled.push(ctl as i32);
        }
        put(&mut d, "network", network);
        put(&mut d, "network_controlled", controlled);
        let mut connections = PackedInt32Array::new();
        for (a, b) in c.connection_pairs() {
            connections.push(a as i32);
            connections.push(b as i32);
        }
        put(&mut d, "connections", connections);
        let mut fluids = VarArray::new();
        for (name, kind, position, size) in c.fluid_nodes() {
            let mut e = VarDictionary::new();
            put(&mut e, "name", GString::from(name.as_str()));
            put(&mut e, "kind", GString::from(kind.as_str()));
            put(&mut e, "position", v3(position));
            put(&mut e, "size", v3(size));
            fluids.push(&e.to_variant());
        }
        put(&mut d, "fluids", fluids);
        d
    }

    /// Everything that changes at `t`, as packed arrays: per block `position, yaw, lift,
    /// motion, param, color, rolled, clip, clip_time, label`; per drawn item `item_id,
    /// item_position, item_yaw, item_scale, item_color, item_carrier, item_stack`.
    #[func]
    fn frame(&self, t: f64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = &self.core else {
            return d;
        };
        let f = c.frame(t);
        put(&mut d, "time", f.time);
        put(&mut d, "position", vectors(&f.position));
        put(&mut d, "yaw", PackedFloat32Array::from(f.yaw.as_slice()));
        put(&mut d, "lift", PackedFloat32Array::from(f.lift.as_slice()));
        put(
            &mut d,
            "motion",
            PackedInt32Array::from(f.motion.as_slice()),
        );
        put(
            &mut d,
            "param",
            PackedFloat32Array::from(f.param.as_slice()),
        );
        put(&mut d, "color", colors(&f.color));
        put(
            &mut d,
            "rolled",
            PackedFloat32Array::from(f.rolled.as_slice()),
        );
        put(&mut d, "clip", PackedInt32Array::from(f.clip.as_slice()));
        put(
            &mut d,
            "clip_time",
            PackedFloat32Array::from(f.clip_time.as_slice()),
        );
        put(&mut d, "label", strings(&f.label));
        put(
            &mut d,
            "item_id",
            PackedInt64Array::from(f.item_id.as_slice()),
        );
        put(&mut d, "item_position", vectors(&f.item_position));
        put(
            &mut d,
            "item_yaw",
            PackedFloat32Array::from(f.item_yaw.as_slice()),
        );
        put(
            &mut d,
            "item_scale",
            PackedFloat32Array::from(f.item_scale.as_slice()),
        );
        put(&mut d, "item_color", colors(&f.item_color));
        put(
            &mut d,
            "item_carrier",
            PackedInt32Array::from(f.item_carrier.as_slice()),
        );
        put(
            &mut d,
            "item_stack",
            PackedInt32Array::from(f.item_stack.as_slice()),
        );
        d
    }
}

fn vectors(list: &[[f32; 3]]) -> PackedVector3Array {
    let v: Vec<Vector3> = list.iter().map(|p| v3(*p)).collect();
    PackedVector3Array::from(v.as_slice())
}

fn colors(list: &[[f32; 3]]) -> PackedColorArray {
    let v: Vec<Color> = list.iter().map(|c| rgb(*c)).collect();
    PackedColorArray::from(v.as_slice())
}
