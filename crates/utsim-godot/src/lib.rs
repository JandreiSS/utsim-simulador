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
//! - `UtsimUi` (`ui_class.rs`): a `Node2D` that draws the interface around the 3D view
//!   with egui ([`egui_host`], [`ui_bar`]): it takes Godot's input, keeps the 3D view from
//!   getting what belongs to a panel, and paints with `RenderingServer` canvas items
//!   ([`egui_mesh`]); widgets keep stable names ([`egui_widgets`]) for the scripted runs.
//!
//! Every position is converted to Godot coordinates here ([`coords`]): `(x, y, z)` of
//! the model becomes `(x, z, -y)` and a yaw is the rotation about `+Y`. The pure modules
//! ([`coords`], [`playback`], [`document`], [`schema`]) have no Godot types and are unit
//! tested without the engine. The appearance rules ([`looks`]) and the third-party
//! catalogue ([`catalogue`]) come from `utsim-look`, shared with the Bevy viewer, and the
//! RON writer ([`ron_text`]) from `utsim-edit`.

// `#[derive(GodotClass)]` with `init` writes `base: base` in the generated constructor.
#![allow(clippy::redundant_field_names)]

pub mod aisle_edit;
pub mod aisle_godot;
pub mod coords;
pub mod dados;
pub mod document;
mod document_class;
pub mod editor3d;
mod editor_class;
pub mod egui_canvas;
pub mod egui_host;
pub mod egui_input;
pub mod egui_mesh;
pub mod egui_widgets;
mod ensino_class;
pub mod extras;
pub mod grid;
pub mod jobs;
mod jobs_class;
pub mod linha;
pub mod network_edit;
pub mod overlay_mesh;
pub mod pert_view;
pub mod playback;
pub mod results;
pub mod schema;
pub mod seguranca;
mod seguranca_class;
pub mod shortcuts;
pub mod tool_panels;
pub mod ui_arrivals;
pub mod ui_bar;
pub mod ui_bottom;
mod ui_class;
pub mod ui_fitwin;
pub mod ui_flows;
pub mod ui_form;
pub mod ui_left;
pub mod ui_pert;
pub mod ui_props;
pub mod ui_results;
pub mod zone_edit;

pub use utsim_edit::ron_text;
pub use utsim_look::{catalogue, visual as looks};

use std::path::PathBuf;

use godot::prelude::*;
use utsim_model::replay::Replay;
use utsim_model::{Model, ModelError};
use utsim_pert::Mode;

use crate::jobs::Task;
use crate::jobs_class::{UtsimJob, UtsimResults, job_object, results_object};
use crate::looks::Clip;
use crate::pert_view::PertRequest;
use crate::playback::{AssetInfo, Playback, role_name, simple_shape};
use crate::results::{OptForm, RunOutput};

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
    /// A recorded replay (`.json`) opened instead of a model: played, not run.
    pub(crate) replay: Option<Replay>,
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
        self.replay = None;
        self.path = None;
        if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("json"))
        {
            return self.load_replay(path);
        }
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

    /// Whether a recorded replay (`.json`) is open: it plays, but does not run.
    #[func]
    fn is_replay(&self) -> bool {
        self.replay.is_some()
    }

    /// The statistics of the recorded replay, as a run of one replication.
    #[func]
    fn replay_results(&self) -> Option<Gd<UtsimResults>> {
        let r = self.replay.as_ref()?;
        let model = Model::parse(&format!(
            "Model(name: {:?}, run: (stop_time: {:?}, replications: 1), objects: [], connections: [])",
            r.model,
            r.stop_time.max(1e-9)
        ))
        .ok()?;
        Some(results_object(RunOutput::new(
            model,
            vec![r.report.clone()],
        )))
    }

    /// Starts `replications` replications (`0`: the model's own number) on a thread of
    /// the bridge; `null` when no model is loaded.
    #[func]
    fn start_run(&mut self, replications: i64) -> Option<Gd<UtsimJob>> {
        let mut m = self.model.clone()?;
        if replications > 0 {
            m.run.replications = replications.min(10_000) as u32;
        }
        Some(job_object(Task::Run(m)))
    }

    /// Compares the scenarios written in `text` (`[nome]` and `chave = valor` lines, the
    /// model as it is first) with common random numbers, keeping `measures` (empty: all).
    /// `null` on a bad text (see `last_error`).
    #[func]
    fn start_scenarios(
        &mut self,
        text: GString,
        measures: PackedStringArray,
        replications: i64,
    ) -> Option<Gd<UtsimJob>> {
        let mut model = self.model.clone()?;
        if replications > 0 {
            model.run.replications = replications.min(10_000) as u32;
        }
        match results::parse_scenarios(&text.to_string()) {
            Ok(scenarios) => Some(job_object(Task::Scenarios {
                model,
                scenarios,
                measures: measures
                    .as_slice()
                    .iter()
                    .map(|s| s.to_string())
                    .filter(|s| !s.trim().is_empty())
                    .collect(),
            })),
            Err(e) => {
                self.last_error = e;
                None
            }
        }
    }

    /// Optimises `form` = `{metric, maximize, variables, method, replications, budget}`
    /// (see `results::OptForm`). `null` on a bad form (see `last_error`).
    #[func]
    fn start_optimize(&mut self, form: VarDictionary) -> Option<Gd<UtsimJob>> {
        let model = self.model.clone()?;
        let text = |k: &str| {
            form.get(k)
                .map_or_else(String::new, |v| v.stringify().to_string())
        };
        let int = |k: &str, default: i64| {
            form.get(k)
                .and_then(|v| v.try_to::<i64>().ok())
                .unwrap_or(default)
        };
        let f = OptForm {
            metric: text("metric"),
            maximize: form
                .get("maximize")
                .and_then(|v| v.try_to::<bool>().ok())
                .unwrap_or(false),
            variables: text("variables"),
            method: text("method"),
            replications: int("replications", 5).clamp(2, 1000) as u32,
            budget: int("budget", 50).clamp(1, 10_000) as usize,
        };
        match f.build() {
            Ok(problem) => Some(job_object(Task::Optimize {
                model,
                problem: Box::new(problem),
            })),
            Err(e) => {
                self.last_error = e;
                None
            }
        }
    }

    /// PERT/CPM of `names` (or, with `between`, from the first to the second) in `mode`
    /// (`simulado`, `classico` or `montecarlo`), with a `deadline` (0 or less: none) and
    /// `samples` draws of the Monte Carlo (0: 10 000).
    #[func]
    fn start_pert(
        &mut self,
        names: PackedStringArray,
        between: bool,
        mode: GString,
        deadline: f64,
        samples: i64,
    ) -> Option<Gd<UtsimJob>> {
        let model = self.model.clone()?;
        let Some(mode) = Mode::parse(&mode.to_string()) else {
            self.last_error = format!("modo \"{mode}\" desconhecido");
            return None;
        };
        let list: Vec<String> = names.as_slice().iter().map(|s| s.to_string()).collect();
        match PertRequest::of(&list, between, mode) {
            Ok(mut request) => {
                request.deadline = (deadline > 0.0 && deadline.is_finite()).then_some(deadline);
                if samples > 0 {
                    request.samples = samples.min(1_000_000) as usize;
                }
                Some(job_object(Task::Pert { model, request }))
            }
            Err(e) => {
                self.last_error = e;
                None
            }
        }
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

    /// Records replication `replication` and writes it to `path` as JSON, the format of
    /// `utsim run --replay` that `load` reads back. False on error (see `last_error`).
    #[func]
    fn save_replay(&mut self, replication: i64, path: GString) -> bool {
        let Some(model) = &self.model else {
            self.last_error = "nenhum modelo carregado".into();
            return false;
        };
        let r = model
            .replay(replication.max(0) as u32)
            .map_err(|e| e.to_string())
            .and_then(|rp| serde_json::to_string(&rp).map_err(|e| e.to_string()))
            .and_then(|text| std::fs::write(path.to_string(), text).map_err(|e| e.to_string()));
        match r {
            Ok(()) => true,
            Err(e) => {
                self.last_error = e;
                false
            }
        }
    }

    /// Records replication `replication` and builds its scene; `null` on error (see
    /// [`Self::last_error`]). `skeletal_people`: the interface has the mannequin files.
    #[func]
    fn playback(&mut self, replication: i64, skeletal_people: bool) -> Option<Gd<UtsimPlayback>> {
        if let Some(replay) = &self.replay {
            let dir = self.path.as_ref().and_then(|p| p.parent());
            let core = Playback::new(replay, None, dir, skeletal_people);
            return Some(Gd::from_init_fn(|base| UtsimPlayback {
                core: Some(core),
                base,
            }));
        }
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

impl UtsimModel {
    /// Reads a replay recorded by `utsim run --replay` (`.json`): `{ok, errors, warnings,
    /// replay: true, name, objects, replications: 1, stop_time}`.
    fn load_replay(&mut self, path: PathBuf) -> VarDictionary {
        let mut d = VarDictionary::new();
        let read = std::fs::read_to_string(&path)
            .map_err(|e| format!("não foi possível ler {}: {e}", path.display()))
            .and_then(|text| {
                serde_json::from_str::<Replay>(&text)
                    .map_err(|e| format!("replay inválido em {}: {e}", path.display()))
            });
        match read {
            Ok(r) => {
                put(&mut d, "ok", true);
                put(&mut d, "errors", strings::<String>(&[]));
                put(&mut d, "warnings", strings::<String>(&[]));
                put(&mut d, "replay", true);
                put(&mut d, "name", GString::from(r.model.as_str()));
                put(&mut d, "description", GString::new());
                put(&mut d, "objects", r.blocks.len() as i64);
                put(&mut d, "replications", 1_i64);
                put(&mut d, "stop_time", r.stop_time);
                self.replay = Some(r);
                self.path = Some(path);
            }
            Err(e) => {
                put(&mut d, "ok", false);
                put(&mut d, "errors", strings(&[e]));
                put(&mut d, "warnings", strings::<String>(&[]));
            }
        }
        d
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
    /// the second file (`pessoa_operador_carga`). Index 6 (`IdleCarry`) is the arms layer; the
    /// last two are the driver's clips (`Driving_Loop`, `Sitting_Idle_Loop`).
    #[func]
    fn person_clips() -> PackedStringArray {
        let names: Vec<String> = Clip::ALL
            .iter()
            .map(|c| match c.source() {
                (n, true) => format!("ual2/{n}"),
                (n, false) => n.to_string(),
            })
            .chain(
                utsim_look::driver::DriverClip::ALL
                    .iter()
                    .map(|c| c.source().to_string()),
            )
            .collect();
        strings(&names)
    }

    #[func]
    fn end(&self) -> f64 {
        self.core.as_ref().map_or(0.0, Playback::end)
    }

    /// The polygons of the floor marking of the circulation aisles: `{source, pattern
    /// ("continuous" | "zebra" | "arrow"), color, polygon}` (empty without a model).
    #[func]
    fn floor_marks(&self) -> VarArray {
        aisle_godot::marks(self.core.as_ref().and_then(|c| c.model.as_ref()))
    }

    /// Pedestrian x vehicle crossings: `{name, position, radius}`.
    #[func]
    fn crossings(&self) -> VarArray {
        aisle_godot::crossings(self.core.as_ref().and_then(|c| c.model.as_ref()))
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
    /// skeletal`, and for a forklift with a seat `driver {position, yaw, scale, eye, hip}` (the
    /// mannequin's root, the camera eye and the hip on the seat, in the block root's frame).
    #[func]
    fn static_scene(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = &self.core else {
            return d;
        };
        let (lo, hi) = extras::scene_bounds(c);
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
            put(&mut e, "stack", v.stack);
            if v.kind == "Source" {
                // C7: the item a source creates, in the colour its items get.
                let c = utsim_scene::style::origin_color(v.b);
                put(&mut e, "preview_color", Color::from_rgb(c[0], c[1], c[2]));
                // The ring of the item types: `[colour, share]` per type.
                let mut mix = VarArray::new();
                for (c, w) in &v.type_mix {
                    let mut pair = VarArray::new();
                    pair.push(&Color::from_rgb(c[0], c[1], c[2]).to_variant());
                    pair.push(&w.to_variant());
                    mix.push(&pair.to_variant());
                }
                put(&mut e, "type_mix", mix);
            }
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
            if let Some(m) = &v.driver {
                let mut driver = VarDictionary::new();
                put(&mut driver, "position", v3(m.position));
                put(&mut driver, "yaw", m.yaw);
                put(&mut driver, "scale", m.scale);
                put(&mut driver, "eye", v3(m.eye));
                put(&mut driver, "hip", v3(m.hip));
                put(&mut e, "driver", driver);
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
        for n in extras::fluid_nodes(c) {
            let mut e = VarDictionary::new();
            put(&mut e, "name", GString::from(n.name.as_str()));
            put(&mut e, "kind", n.kind_label);
            put(&mut e, "shape", extras::shape_name(n.shape));
            put(&mut e, "position", v3(n.position));
            put(&mut e, "radius", n.radius);
            put(&mut e, "height", n.height);
            put(&mut e, "capacity", n.capacity);
            put(&mut e, "port", n.port);
            fluids.push(&e.to_variant());
        }
        put(&mut d, "fluids", fluids);
        let mut links = PackedInt32Array::new();
        for (a, b) in extras::fluid_links(c) {
            links.push(a as i32);
            links.push(b as i32);
        }
        put(&mut d, "fluid_links", links);
        let mut conv = VarArray::new();
        for (node, block, into) in extras::fluid_converters(c) {
            let mut e = VarDictionary::new();
            put(&mut e, "node", node as i64);
            put(&mut e, "block", block as i64);
            put(&mut e, "into_fluid", into);
            conv.push(&e.to_variant());
        }
        put(&mut d, "fluid_converters", conv);
        put(
            &mut d,
            "flows",
            c.model.as_ref().map_or(0, |m| m.flows.len()) as i64,
        );
        d
    }

    /// The fluid network at `t`: per node `fill` (tanks, 0..1), `level`, `rate`,
    /// `color`, `text`; per link `link_active`.
    #[func]
    fn fluid_frame(&self, t: f64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = &self.core else {
            return d;
        };
        let f = extras::fluid_frame(c, t);
        put(&mut d, "fill", PackedFloat32Array::from(f.fill.as_slice()));
        put(
            &mut d,
            "level",
            PackedFloat64Array::from(f.level.as_slice()),
        );
        put(&mut d, "rate", PackedFloat64Array::from(f.rate.as_slice()));
        put(&mut d, "color", colors(&f.color));
        put(&mut d, "text", strings(&f.text));
        let active: Vec<i32> = f.link_active.iter().map(|&a| i32::from(a)).collect();
        put(
            &mut d,
            "link_active",
            PackedInt32Array::from(active.as_slice()),
        );
        d
    }

    /// The token flows of the model as 2D graphs: `[{name, nodes: [{name, label, x,
    /// y}], edges: PackedInt32Array (pairs)}]`.
    #[func]
    fn flows(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = &self.core else {
            return out;
        };
        for g in extras::flow_graphs(c) {
            let mut d = VarDictionary::new();
            put(&mut d, "name", GString::from(g.name.as_str()));
            let mut nodes = VarArray::new();
            for n in &g.nodes {
                let mut e = VarDictionary::new();
                put(&mut e, "name", GString::from(n.name.as_str()));
                put(&mut e, "label", n.label);
                put(&mut e, "x", n.x);
                put(&mut e, "y", n.y);
                put(&mut e, "layer", n.layer as i64);
                put(&mut e, "lane", n.lane);
                nodes.push(&e.to_variant());
            }
            put(&mut d, "nodes", nodes);
            let mut edges = PackedInt32Array::new();
            for (a, b) in &g.edges {
                edges.push(*a as i32);
                edges.push(*b as i32);
            }
            put(&mut d, "edges", edges);
            out.push(&d.to_variant());
        }
        out
    }

    /// Tokens in each activity of flow `flow` (index into [`Self::flows`]) at `t`.
    #[func]
    fn flow_counts(&self, flow: i64, t: f64) -> PackedInt32Array {
        let Some(c) = &self.core else {
            return PackedInt32Array::new();
        };
        let graphs = extras::flow_graphs(c);
        let Some(g) = graphs.get(flow.max(0) as usize) else {
            return PackedInt32Array::new();
        };
        let v: Vec<i32> = extras::flow_counts(c, g, t)
            .into_iter()
            .map(|x| x as i32)
            .collect();
        PackedInt32Array::from(v.as_slice())
    }

    /// Tokens of every activity of flow `flow` at `samples` instants over the run:
    /// `{times: PackedFloat64Array, counts: [PackedInt32Array per activity]}`.
    #[func]
    fn flow_series(&self, flow: i64, samples: i64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = &self.core else {
            return d;
        };
        let graphs = extras::flow_graphs(c);
        let Some(g) = graphs.get(flow.max(0) as usize) else {
            return d;
        };
        let (times, counts) = extras::flow_series(c, g, samples.clamp(2, 2000) as usize);
        put(&mut d, "times", PackedFloat64Array::from(times.as_slice()));
        let mut list = VarArray::new();
        for row in counts {
            let v: Vec<i32> = row.into_iter().map(|x| x as i32).collect();
            list.push(&PackedInt32Array::from(v.as_slice()).to_variant());
        }
        put(&mut d, "counts", list);
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
        put(&mut d, "task_target", vectors(&f.task_target));
        put(
            &mut d,
            "flash",
            PackedFloat32Array::from(f.flash.as_slice()),
        );
        d
    }

    /// The team and request lines (E1b): `[{kind, from, to, point}]` with block indices
    /// (`to` is -1 for a home or break place, then `point` is where it is).
    #[func]
    fn team_links(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = &self.core else {
            return out;
        };
        for l in &c.links {
            let mut d = VarDictionary::new();
            put(&mut d, "kind", l.kind);
            put(&mut d, "from", l.from as i64);
            put(&mut d, "to", l.to.map_or(-1, |t| t as i64));
            put(
                &mut d,
                "point",
                Vector3::new(l.to_point[0], l.to_point[1], l.to_point[2]),
            );
            out.push(&d.to_variant());
        }
        out
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
