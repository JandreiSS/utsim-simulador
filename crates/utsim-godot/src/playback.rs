//! The playback core, with no Godot types: a [`Scene`] built from a replay plus the look
//! of each block, packed into flat arrays per frame (one entry per drawn block, one per
//! drawn item). The Godot classes in `lib.rs` only copy these vectors into packed arrays,
//! so everything here is unit-tested without the engine.
//!
//! Positions are Godot coordinates (`crate::coords`): static entities come from the
//! scene contract (`Scene::static_scene`, model coordinates) through
//! [`to_godot`]; per-frame poses are computed in the scene's layout space, which is the
//! same space, with the realistic adjustments of the Bevy viewer (vehicles kept out of
//! machines, an operator at the front of the station, loads on forks, hands and decks).

use std::collections::HashMap;
use std::path::Path;

use utsim_assets::Spec;
use utsim_model::Model;
use utsim_model::replay::Replay;
use utsim_scene::compose::ITEM_SIZE;
use utsim_scene::style::{self, Shape};
use utsim_scene::{EntityId, EntityKind, LinkKind, MobileRole, Motion, Scene, SceneOptions};

use crate::coords::{layout_to_godot, size_to_godot, to_godot, yaw_to_godot};
use utsim_look::driver::{self, DriverClip, DriverMount};

use crate::looks::{self, Carry, Clip, Context, Look, Source};

/// Clips of the bridge's list: the person clips, then the driver's.
pub const CLIPS: usize = Clip::ALL.len() + DriverClip::ALL.len();

/// Motion codes of the frame arrays (`motion`).
pub fn motion_code(m: Motion) -> (i32, f32) {
    match m {
        Motion::Idle => (0, 0.0),
        Motion::Moving { speed } => (1, speed),
        Motion::Loading { progress } => (2, progress),
        Motion::Unloading { progress } => (3, progress),
        Motion::Working => (4, 0.0),
        Motion::Blocked => (5, 0.0),
        Motion::Stopped => (6, 0.0),
    }
}

pub fn role_name(role: MobileRole) -> &'static str {
    match role {
        MobileRole::Person => "person",
        MobileRole::Forklift => "forklift",
        MobileRole::Agv => "agv",
        MobileRole::Vehicle => "vehicle",
    }
}

/// One drawn block (a model object; token flows and fluid networks are left out).
#[derive(Clone, Debug)]
pub struct BlockView {
    /// Replay block index.
    pub b: usize,
    pub name: String,
    pub kind: String,
    pub role: Option<MobileRole>,
    pub look: Option<Look>,
    /// The seated driver of a forklift (presentation only), when the interface has the
    /// mannequin files.
    pub driver: Option<DriverMount>,
    /// Model coordinates of the base, and the footprint `(x, y, z)` size.
    pub position: [f32; 3],
    pub size: [f32; 3],
    /// A queue with `discipline: Lifo` (a stack): labelled and drawn as one.
    pub stack: bool,
    /// A source's arrival mode, for the badge of its label (`horário`, `tabela`,
    /// `sequência`, `taxa`, with `lote` for batches); empty for the plain interval.
    pub badge: String,
    /// The item types a source creates, with their colours and shares (C7: the ring over
    /// it); empty when it does not name types.
    pub type_mix: Vec<([f32; 3], f64)>,
}

/// The item types `name` creates (`item_type`, or the weights of `item_type_mix`) with the
/// colour of each type (`item_types[..].color`, else the palette by type order).
pub fn source_types(model: Option<&Model>, name: &str) -> Vec<([f32; 3], f64)> {
    let Some(m) = model else {
        return Vec::new();
    };
    let Some(obj) = m.objects.iter().find(|o| o.name == name) else {
        return Vec::new();
    };
    let utsim_model::ObjectKind::Source {
        item_type,
        item_type_mix,
        ..
    } = &obj.kind
    else {
        return Vec::new();
    };
    let color = |t: &str| {
        let k = m.item_types.iter().position(|d| d.name == t);
        k.and_then(|k| m.item_types[k].color.as_deref())
            .and_then(looks::parse_color)
            .unwrap_or_else(|| style::origin_color(k.unwrap_or(0) + 1))
    };
    let total: f64 = item_type_mix.iter().map(|(_, w)| w.max(0.0)).sum();
    if !item_type_mix.is_empty() && total > 0.0 {
        item_type_mix
            .iter()
            .filter(|(_, w)| *w > 0.0)
            .map(|(t, w)| (color(t), w / total))
            .collect()
    } else if let Some(t) = item_type {
        vec![(color(t), 1.0)]
    } else {
        Vec::new()
    }
}

/// The badge of a source's label: how it creates items (`docs/plano_chegadas_e_equipes_3d.md`,
/// C7).
fn source_badge(model: Option<&Model>, name: &str) -> String {
    let Some(obj) = model.and_then(|m| m.objects.iter().find(|o| o.name == name)) else {
        return String::new();
    };
    let utsim_model::ObjectKind::Source {
        schedule,
        table,
        batch,
        ..
    } = &obj.kind
    else {
        return String::new();
    };
    let mut parts = Vec::new();
    if let Some(t) = table {
        parts.push(match t.mode {
            utsim_model::TableMode::Sequence => "sequência",
            _ => "tabela",
        });
    } else if schedule.is_some() {
        parts.push("horário");
    }
    if batch.is_some() {
        parts.push("lote");
    }
    parts.join(", ")
}

/// Whether `name` is a queue with `discipline: Lifo` in `model`.
fn is_stack(model: Option<&Model>, name: &str) -> bool {
    model
        .and_then(|m| m.objects.iter().find(|o| o.name == name))
        .and_then(|o| crate::schema::read_field(o, "discipline"))
        .as_deref()
        == Some("Lifo")
}

/// The asset a block is drawn with, as the Godot side needs it.
#[derive(Clone, Debug, PartialEq)]
pub struct AssetInfo {
    /// `"library"`, `"catalogue"` or `"file"`.
    pub source: &'static str,
    /// Library or catalogue name, or the file stem.
    pub name: String,
    /// Library key (`Spec::key`): equal to `name` for the default parameters, whose
    /// `.glb` is the imported `assets/modelos/<name>.glb`; any other key is generated by
    /// [`Playback::glb`].
    pub key: String,
    /// File on disk (catalogue entries and user files).
    pub path: String,
}

impl AssetInfo {
    pub fn of(source: &Source) -> AssetInfo {
        match source {
            Source::Library(spec) => AssetInfo {
                source: "library",
                name: spec.name().to_string(),
                key: spec.key(),
                path: String::new(),
            },
            Source::Catalogue { name, file } => AssetInfo {
                source: "catalogue",
                name: name.clone(),
                key: name.clone(),
                path: file.to_string_lossy().replace('\\', "/"),
            },
            Source::File(file) => AssetInfo {
                source: "file",
                name: file
                    .file_stem()
                    .map_or_else(String::new, |s| s.to_string_lossy().into_owned()),
                key: file.to_string_lossy().replace('\\', "/"),
                path: file.to_string_lossy().replace('\\', "/"),
            },
        }
    }
}

/// Per-frame arrays (see the module docs). Block arrays have one entry per
/// [`Playback::blocks`]; item arrays one per drawn item.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrameData {
    pub time: f64,
    pub position: Vec<[f32; 3]>,
    pub yaw: Vec<f32>,
    pub lift: Vec<f32>,
    pub motion: Vec<i32>,
    /// Speed (moving) or progress (loading, unloading).
    pub param: Vec<f32>,
    /// State colour, or the AGV status light by motion, sRGB.
    pub color: Vec<[f32; 3]>,
    /// Distance driven divided by the asset scale (wheels: `-(rolled / radius)`).
    pub rolled: Vec<f32>,
    /// Skeletal people: clip index ([`Clip::index`], -1 for others) and time in it. The
    /// seated driver of a forklift: [`DriverClip::global_index`].
    pub clip: Vec<i32>,
    pub clip_time: Vec<f32>,
    pub label: Vec<String>,
    pub item_id: Vec<i64>,
    /// Centre of the item's [`ITEM_SIZE`] cube times `item_scale`.
    pub item_position: Vec<[f32; 3]>,
    pub item_yaw: Vec<f32>,
    pub item_scale: Vec<f32>,
    pub item_color: Vec<[f32; 3]>,
    /// Block (index into [`Playback::blocks`]) of a skeletal person holding the item, so
    /// the engine can put it between the hands; -1 otherwise.
    pub item_carrier: Vec<i32>,
    /// Position of the item in its carrier's stack (0 = lowest).
    pub item_stack: Vec<i32>,
    /// Where each mobile resource is going for its current task (Godot coordinates), or
    /// NaN when it has none (the line of the selected member, E8b).
    pub task_target: Vec<[f32; 3]>,
    /// A source that just lost an item (`on_blocked: Drop`): 1 at the loss, falling to 0
    /// over [`FLASH_SECONDS`] of simulated time; 0 for everything else (C7).
    pub flash: Vec<f32>,
}

/// How long (simulated seconds) a source blinks after losing an item.
pub const FLASH_SECONDS: f64 = 1.0;
/// Gap between the items a source holds at its output, side by side (a batch waiting).
const PORT_GAP: f32 = 0.06;

/// Where item `k` of `n` flying together (in the same place: a batch released at once)
/// goes, as an offset across the floor (x, z): a small square grid centred on the place.
pub fn flight_slot(k: usize, n: usize) -> [f32; 2] {
    if n <= 1 {
        return [0.0, 0.0];
    }
    let cols = (n as f32).sqrt().ceil() as usize;
    let rows = n.div_ceil(cols);
    let step = ITEM_SIZE + PORT_GAP;
    let (r, c) = (k / cols, k % cols);
    [
        (c as f32 - (cols - 1) as f32 / 2.0) * step,
        (r as f32 - (rows - 1) as f32 / 2.0) * step,
    ]
}

/// Where item `k` of `n` held by a source waits: at the mouth of its output port (the `+x`
/// side of the footprint), side by side across it, on the floor. `half` is the source's
/// half size (x, height, z), Godot axes.
pub fn port_slot(half: [f32; 3], k: usize, n: usize) -> [f32; 3] {
    let step = ITEM_SIZE + PORT_GAP;
    let across = (k as f32 - (n.max(1) - 1) as f32 / 2.0) * step;
    [
        half[0] + ITEM_SIZE / 2.0 + 0.12,
        ITEM_SIZE / 2.0 + 0.01,
        across,
    ]
}

/// A team or request line (`utsim_scene::team_links`) between drawn blocks: kind name
/// (`equipe`, `transporte`, `operadores`, `manutencao`, `casa`, `pausa`), the block it
/// starts from and the one it ends at (or `None` for a place), with the end point.
#[derive(Clone, Debug, PartialEq)]
pub struct TeamLink {
    pub kind: &'static str,
    pub from: usize,
    pub to: Option<usize>,
    pub to_point: [f32; 3],
}

/// The name of a link kind in the interface.
pub fn link_kind_name(kind: LinkKind) -> &'static str {
    match kind {
        LinkKind::Team => "equipe",
        LinkKind::Transport => "transporte",
        LinkKind::Operators => "operadores",
        LinkKind::Repair => "manutencao",
        LinkKind::Home => "casa",
        LinkKind::Break => "pausa",
    }
}

/// The team and request lines of `model` with block indices mapped by `index_of`
/// (model object order to the drawn blocks); lines to blocks not drawn are skipped.
pub fn team_lines(model: &Model, index_of: impl Fn(usize) -> Option<usize>) -> Vec<TeamLink> {
    utsim_scene::team_links(model)
        .into_iter()
        .filter_map(|l| {
            let to = match l.to {
                Some(b) => Some(index_of(b as usize)?),
                None => None,
            };
            Some(TeamLink {
                kind: link_kind_name(l.kind),
                from: index_of(l.from as usize)?,
                to,
                to_point: to_godot(l.to_point),
            })
        })
        .collect()
}

/// A replay ready to be played in Godot.
pub struct Playback {
    pub scene: Scene,
    pub blocks: Vec<BlockView>,
    /// Replay block index to [`Playback::blocks`] index.
    index_of: Vec<Option<usize>>,
    /// Replay indices of the solid stations (`Processor`, `Combiner`).
    stations: Vec<usize>,
    /// Edge of a load on forks (the largest rack load, 0.8 m without racks).
    pub fork_size: f32,
    /// Items are pallets with boxes (the scene has racks, forklifts or pallet jacks).
    pub pallets: bool,
    durations: [f32; CLIPS],
    specs: HashMap<String, Spec>,
    pub connections: Vec<(usize, usize)>,
    /// The model that produced the replay (token flows, network), when there is one.
    pub model: Option<Model>,
    /// The team and request lines between the drawn blocks.
    pub links: Vec<TeamLink>,
    /// Each dispatcher (index into [`Playback::blocks`]) and the members of its team.
    teams: Vec<(usize, Vec<usize>)>,
    /// The instants each source created its items, by replay block (empty for others).
    created: Vec<Vec<f64>>,
    /// The instants each source lost an item (created and destroyed without leaving).
    dropped: Vec<Vec<f64>>,
}

impl Playback {
    /// Builds the scene of `replay` (with the `model` that produced it, for sizes,
    /// networks and `visual`). `skeletal_people`: the interface has the mannequin files.
    pub fn new(
        replay: &Replay,
        model: Option<&Model>,
        model_dir: Option<&Path>,
        skeletal_people: bool,
    ) -> Playback {
        let scene = Scene::build(
            replay,
            &SceneOptions {
                model,
                ..SceneOptions::default()
            },
        );
        let static_scene = scene.static_scene();
        let n = replay.blocks.len();
        let ctx = Context {
            pallets: (0..n).any(|b| {
                replay.blocks[b].kind == "Rack" || scene.role(b) == Some(MobileRole::Forklift)
            }),
            skeletal_people,
        };
        let mut blocks = Vec::new();
        let mut index_of = vec![None; n];
        for e in &static_scene.entities {
            let EntityId::Block(b) = e.id else { continue };
            let b = b as usize;
            let (name, kind, role) = match &e.kind {
                EntityKind::Block { name, kind } => (name.clone(), kind.clone(), None),
                EntityKind::Mobile { name, kind, role } => {
                    (name.clone(), kind.clone(), Some(*role))
                }
                _ => continue,
            };
            let look = looks::look_of(
                &kind,
                role,
                scene.visual(b),
                &scene.geoms[b],
                ctx,
                model_dir,
            );
            index_of[b] = Some(blocks.len());
            let driver = look
                .as_ref()
                .filter(|_| skeletal_people)
                .and_then(|l| driver::driver_mount(role, l));
            let stack = kind == "Queue" && is_stack(model, &name);
            let badge = if kind == "Source" {
                source_badge(model, &name)
            } else {
                String::new()
            };
            let type_mix = if kind == "Source" {
                source_types(model, &name)
            } else {
                Vec::new()
            };
            blocks.push(BlockView {
                b,
                stack,
                badge,
                type_mix,
                name,
                kind,
                role,
                look,
                driver,
                position: e.position,
                size: e.size,
            });
        }
        let pallets = blocks.iter().filter_map(|v| v.look.as_ref()).any(|l| {
            matches!(
                l.source,
                Source::Library(Spec::PortaPaletes(_) | Spec::Empilhadeira | Spec::Paleteira)
            )
        });
        let fork_size = scene
            .geoms
            .iter()
            .filter_map(|g| g.rack_params())
            .map(|p| p.item_size())
            .fold(None, |a: Option<f32>, s| Some(a.map_or(s, |a| a.max(s))))
            .unwrap_or(0.8);
        let mut specs = HashMap::new();
        for v in &blocks {
            if let Some(Look {
                source: Source::Library(spec),
                ..
            }) = &v.look
            {
                specs.insert(spec.key(), *spec);
            }
        }
        let stations = (0..n)
            .filter(|&b| matches!(replay.blocks[b].kind.as_str(), "Processor" | "Combiner"))
            .collect();
        let mut created = vec![Vec::new(); n];
        let mut dropped = vec![Vec::new(); n];
        // Items still at the block that created them: destroyed there, they were lost.
        let mut at_birth: HashMap<u64, usize> = HashMap::new();
        for r in &replay.records {
            match r.event {
                utsim_core::TraceEvent::Created { item, block } => {
                    if let Some(v) = created.get_mut(block) {
                        v.push(r.t);
                    }
                    at_birth.insert(item, block);
                }
                utsim_core::TraceEvent::Moved { item, .. }
                | utsim_core::TraceEvent::Packed { item, .. } => {
                    at_birth.remove(&item);
                }
                utsim_core::TraceEvent::Destroyed { item } => {
                    if let Some(b) = at_birth.remove(&item)
                        && replay.blocks.get(b).is_some_and(|x| x.kind == "Source")
                    {
                        dropped[b].push(r.t);
                    }
                }
                _ => {}
            }
        }
        let links = model.map_or_else(Vec::new, |m| {
            team_lines(m, |b| index_of.get(b).copied().flatten())
        });
        let mut teams: Vec<(usize, Vec<usize>)> = Vec::new();
        for l in links.iter().filter(|l| l.kind == "equipe") {
            let Some(to) = l.to else { continue };
            match teams.iter_mut().find(|t| t.0 == l.from) {
                Some(t) => t.1.push(to),
                None => teams.push((l.from, vec![to])),
            }
        }
        Playback {
            scene,
            blocks,
            index_of,
            stations,
            fork_size,
            pallets,
            durations: [1.0; CLIPS],
            specs,
            connections: replay.connections.clone(),
            model: model.cloned(),
            links,
            teams,
            created,
            dropped,
        }
    }

    /// Lengths of the person clips, seconds, in [`Clip::ALL`] order (read by the engine
    /// from the imported animations).
    pub fn set_clip_durations(&mut self, d: &[f32]) {
        for (k, v) in d.iter().take(CLIPS).enumerate() {
            if v.is_finite() && *v > 0.0 {
                self.durations[k] = *v;
            }
        }
    }

    /// `.glb` bytes of the library asset with this key (a parametric size), or `None`.
    pub fn glb(&self, key: &str) -> Option<Vec<u8>> {
        self.specs.get(key).map(Spec::glb)
    }

    pub fn end(&self) -> f64 {
        self.scene.end
    }

    pub fn next_event(&self, t: f64) -> Option<f64> {
        self.scene.timeline.next_event_time(t)
    }

    pub fn prev_event(&self, t: f64) -> Option<f64> {
        self.scene.timeline.prev_event_time(t)
    }

    /// Godot position of the base of block `i`, and its size as Godot extents.
    pub fn static_pose(&self, i: usize) -> ([f32; 3], [f32; 3]) {
        let v = &self.blocks[i];
        (to_godot(v.position), size_to_godot(v.size))
    }

    /// Segments of the path network, Godot coordinates.
    pub fn network(&self) -> Vec<([f32; 3], [f32; 3], bool)> {
        self.scene
            .static_scene()
            .network
            .iter()
            .map(|s| (to_godot(s.from), to_godot(s.to), s.capacity.is_some()))
            .collect()
    }

    /// Fluid nodes: name, kind, base and size (Godot coordinates).
    pub fn fluid_nodes(&self) -> Vec<(String, String, [f32; 3], [f32; 3])> {
        self.scene
            .static_scene()
            .entities
            .into_iter()
            .filter_map(|e| match e.kind {
                EntityKind::FluidNode { name, kind } => {
                    Some((name, kind, to_godot(e.position), size_to_godot(e.size)))
                }
                _ => None,
            })
            .collect()
    }

    /// Index into [`Playback::blocks`] of replay block `b`, when it is drawn.
    pub fn drawn_index(&self, b: usize) -> Option<usize> {
        self.index_of.get(b).copied().flatten()
    }

    /// Connections as pairs of drawn-block indices.
    pub fn connection_pairs(&self) -> Vec<(usize, usize)> {
        self.connections
            .iter()
            .filter_map(|&(a, b)| {
                Some((
                    self.index_of.get(a).copied()??,
                    self.index_of.get(b).copied()??,
                ))
            })
            .collect()
    }

    /// Mobile pose as drawn (`models::drawn_pose` of the viewer), layout space.
    fn drawn_pose(&self, b: usize, t: f64, look: &Look) -> Option<utsim_scene::MobilePose> {
        let s = &self.scene;
        let mut p = s.mobile_at(b, t)?;
        let original = p.position;
        let reach = match look.carry {
            Carry::Forks { forward, .. } => forward + 0.6,
            Carry::Hands => 0.35,
            _ => 0.6,
        };
        for &st in &self.stations {
            p.position =
                looks::keep_outside(p.position, p.yaw, s.positions[st], s.geoms[st].half, reach);
        }
        if look.carry == Carry::Hands && p.motion == Motion::Working {
            let station = self.stations.iter().copied().find(|&st| {
                [0, 2]
                    .iter()
                    .all(|&k| (original[k] - s.positions[st][k]).abs() <= s.geoms[st].half[k])
            });
            if let Some(st) = station {
                (p.position, p.yaw) = looks::station_front(s.positions[st], s.geoms[st].half);
            }
        }
        Some(p)
    }

    /// Everything that changes at `t`.
    pub fn frame(&self, t: f64) -> FrameData {
        let s = &self.scene;
        let n = self.blocks.len();
        let mut f = FrameData {
            time: t,
            ..FrameData::default()
        };
        f.position.reserve(n);
        let mut poses: Vec<Option<utsim_scene::MobilePose>> = Vec::with_capacity(n);
        for v in &self.blocks {
            let b = v.b;
            let state = s.timeline.block_state(b, t);
            let scale = v.look.as_ref().map_or(1.0, |l| l.scale);
            let pose = match (&v.look, v.role) {
                (Some(look), Some(_)) => self.drawn_pose(b, t, look),
                (None, Some(_)) => s.mobile_at(b, t),
                _ => None,
            };
            let lost = self.dropped[b].partition_point(|&d| d <= t);
            f.flash
                .push(match lost.checked_sub(1).map(|k| t - self.dropped[b][k]) {
                    Some(dt) if dt < FLASH_SECONDS => (1.0 - dt / FLASH_SECONDS) as f32,
                    _ => 0.0,
                });
            f.task_target.push(
                pose.and_then(|_| s.timeline.travel_target(b, t))
                    .map_or([f32::NAN; 3], |p| to_godot(p.map(|v| v as f32))),
            );
            poses.push(pose);
            let (position, yaw, motion, lift) = match pose {
                Some(p) => (
                    layout_to_godot(p.position),
                    yaw_to_godot(p.yaw),
                    p.motion,
                    p.lift,
                ),
                None => (
                    layout_to_godot(s.base_at(b, t)),
                    0.0,
                    match state {
                        Some(utsim_core::State::Processing | utsim_core::State::Setup) => {
                            Motion::Working
                        }
                        Some(utsim_core::State::Blocked) => Motion::Blocked,
                        Some(
                            utsim_core::State::Down
                            | utsim_core::State::OffShift
                            | utsim_core::State::Stopped,
                        ) => Motion::Stopped,
                        _ => Motion::Idle,
                    },
                    0.0,
                ),
            };
            let (code, param) = motion_code(motion);
            let agv = v.role == Some(MobileRole::Agv);
            let distance = if v.role.is_some() {
                s.distance_at(b, t) as f32
            } else {
                0.0
            };
            let content = s.timeline.block_content(b, t);
            let (clip, clip_time) = match &v.look {
                Some(l) if l.skeletal => {
                    let carrying = lift > 0.0 || content > 0;
                    let c = looks::person_clip(motion, carrying);
                    let progress = match motion {
                        Motion::Loading { progress } | Motion::Unloading { progress } => progress,
                        _ => 0.0,
                    };
                    // IdleCarry: the time of the idle legs (the arms hold at 0).
                    let timed = if c == Clip::IdleCarry { Clip::Idle } else { c };
                    let d = self.durations[timed.index()];
                    (
                        c.index() as i32,
                        looks::clip_time(timed, d, distance, scale, t, progress),
                    )
                }
                _ if v.driver.is_some() => {
                    let c = driver::driver_clip(motion);
                    let d = self.durations[c.global_index()];
                    let time = if d > 0.0 {
                        t.rem_euclid(d as f64) as f32
                    } else {
                        0.0
                    };
                    (c.global_index() as i32, time)
                }
                _ => (-1, 0.0),
            };
            let received = (v.kind == "Sink").then(|| s.timeline.block_received(b, t));
            f.position.push(position);
            f.yaw.push(yaw);
            f.lift.push(lift);
            f.motion.push(code);
            f.param.push(param);
            f.color.push(looks::light_color(agv, motion, state));
            f.rolled.push(distance / scale);
            f.clip.push(clip);
            f.clip_time.push(clip_time);
            let mut shown = if v.stack {
                format!("{} · LIFO", v.name)
            } else {
                v.name.clone()
            };
            if v.kind == "Source" {
                // C7: the mode badge, and in a run the items created so far.
                if !v.badge.is_empty() {
                    shown.push_str(&format!(" · {}", v.badge));
                }
                let made = self.created[b].partition_point(|&c| c <= t);
                if !self.created[b].is_empty() {
                    shown.push_str(&format!(" · {made} criados"));
                }
            }
            // A queue drawn as a conveyor counts what does not fit on its belt.
            let hidden = v
                .look
                .as_ref()
                .and_then(|l| looks::queue_belt(&v.kind, l))
                .map_or(0, |belt| belt.hidden(content, ITEM_SIZE));
            f.label
                .push(style::label_text(&shown, content, hidden, received));
        }
        // A dispatcher says how many of its members are free (idle) now.
        for (d, members) in &self.teams {
            let free = members
                .iter()
                .filter(|&&m| f.motion.get(m).copied() == Some(motion_code(Motion::Idle).0))
                .count();
            if let Some(l) = f.label.get_mut(*d) {
                l.push_str(&format!(" · {free}/{} livres", members.len()));
            }
        }
        // Items: where the scene puts them, moved onto forks, hands and decks.
        let mut stacked: HashMap<usize, usize> = HashMap::new();
        let mut resting: HashMap<usize, usize> = HashMap::new();
        // Queues drawn as conveyors: an item flying in or out does not sink into the belt.
        let belts: Vec<([f32; 3], looks::QueueBelt)> = (0..self.blocks.len())
            .filter_map(|i| {
                let v = &self.blocks[i];
                let belt = looks::queue_belt(&v.kind, v.look.as_ref()?)?;
                Some((self.static_pose(i).0, belt))
            })
            .collect();
        let items = s.items_at(t);
        // Items held by a source (it is blocked, or a batch waits to leave): how many each.
        let mut held: HashMap<usize, usize> = HashMap::new();
        for p in items.iter().filter(|p| p.drawn) {
            if let Some(EntityId::Block(b)) = p.parent
                && let Some(i) = self.index_of.get(b as usize).copied().flatten()
                && self.blocks[i].kind == "Source"
            {
                *held.entry(i).or_default() += 1;
            }
        }
        let mut at_port: HashMap<usize, usize> = HashMap::new();
        // Items flying in the same place (a batch that left together): how many there.
        let place = |c: [f32; 3]| c.map(|v| (v * 1000.0).round() as i64);
        let mut flying: HashMap<[i64; 3], usize> = HashMap::new();
        for p in items.iter().filter(|p| p.drawn && p.parent.is_none()) {
            *flying.entry(place(p.center)).or_default() += 1;
        }
        let mut in_flight: HashMap<[i64; 3], usize> = HashMap::new();
        for p in items.into_iter().filter(|p| p.drawn) {
            let mut center = p.center;
            if p.parent.is_none()
                && let Some(&n) = flying.get(&place(center))
                && n > 1
            {
                // Side by side instead of one inside the other.
                let k = in_flight.entry(place(center)).or_default();
                let o = flight_slot(*k, n);
                *k += 1;
                center[0] += o[0];
                center[2] += o[1];
            }
            if let Some(EntityId::Block(b)) = p.parent
                && let Some(i) = self.index_of.get(b as usize).copied().flatten()
                && let Some(&n) = held.get(&i)
            {
                // C7: at the mouth of the output port, side by side.
                let k = at_port.entry(i).or_default();
                let (base, size) = self.static_pose(i);
                let o = port_slot([size[0] / 2.0, size[1], size[2] / 2.0], *k, n);
                *k += 1;
                center = [base[0] + o[0], base[1] + o[1], base[2] + o[2]];
                let origin = s.timeline.item_origin(p.item).unwrap_or(0);
                f.item_id.push(p.id as i64);
                f.item_position.push(layout_to_godot(center));
                f.item_yaw.push(0.0);
                f.item_scale.push(1.0);
                f.item_color.push(style::origin_color(origin));
                f.item_carrier.push(-1);
                f.item_stack.push(0);
                continue;
            }
            let mut scale = p.scale;
            let mut carrier = -1;
            let mut stack = 0;
            if let Some(EntityId::Block(b)) = p.parent
                && let Some(i) = self.index_of.get(b as usize).copied().flatten()
                && let Some(look) = &self.blocks[i].look
            {
                let v = &self.blocks[i];
                if let Some(belt) = looks::queue_belt(&v.kind, look) {
                    // On top of the belt, in a line (the scene's slots are the simple
                    // queue's: their base is the block's).
                    let k = resting.entry(i).or_default();
                    let simple = style::slot_offset(Shape::LongBox, *k);
                    let Some(at) = belt.slot(*k, ITEM_SIZE) else {
                        continue;
                    };
                    *k += 1;
                    center = [
                        center[0] - simple[0] + at[0],
                        center[1] - simple[1] + at[1],
                        center[2] - simple[2] + at[2],
                    ];
                } else if v.kind == "Queue" {
                    center[1] -= looks::queue_drop();
                } else if let Some(pose) = poses[i] {
                    if let Carry::Forks { .. } = look.carry {
                        scale = self.fork_size / ITEM_SIZE;
                    }
                    let k = stacked.entry(i).or_default();
                    if let Some(c) = looks::carried_center(
                        look.carry,
                        pose.position,
                        pose.yaw,
                        pose.lift,
                        *k,
                        scale * ITEM_SIZE,
                    ) {
                        center = c;
                    }
                    if look.skeletal {
                        carrier = i as i32;
                        stack = *k as i32;
                    }
                    *k += 1;
                }
            }
            if p.parent.is_none() {
                for (base, belt) in &belts {
                    let d = [
                        center[0] - base[0],
                        center[1] - base[1],
                        center[2] - base[2],
                    ];
                    if belt.covers(d, scale * ITEM_SIZE) {
                        center[1] = center[1].max(base[1] + belt.rest_height(scale * ITEM_SIZE));
                    }
                }
            }
            let origin = s.timeline.item_origin(p.item).unwrap_or(0);
            f.item_id.push(p.id as i64);
            f.item_position.push(layout_to_godot(center));
            f.item_yaw.push(yaw_to_godot(p.yaw));
            f.item_scale.push(scale);
            f.item_color.push(style::origin_color(origin));
            f.item_carrier.push(carrier);
            f.item_stack.push(stack);
        }
        f
    }
}

/// Simple shape of a block without a realistic look, as the Bevy viewer draws it
/// (`block_mesh` in `crates/utsim-viewer/src/scene.rs`).
pub fn simple_shape(kind: &str) -> &'static str {
    match style::shape_of(kind) {
        Shape::Cone => "cone",
        Shape::Cylinder => "cylinder",
        Shape::Funnel => "funnel",
        Shape::Frustum => "frustum",
        Shape::Flag => "flag",
        _ => "box",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn example(name: &str) -> (Model, PathBuf) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples")
            .join(name);
        (Model::load(&path).expect("model"), path)
    }

    fn playback(name: &str, skeletal: bool) -> Playback {
        let (model, path) = example(name);
        let replay = model.replay(0).expect("replay");
        Playback::new(&replay, Some(&model), path.parent(), skeletal)
    }

    #[test]
    fn a_source_shows_its_types_its_losses_and_what_it_holds() {
        // The ring of the types: shares that add up to one, in the types' colours.
        let pb = playback("chegadas_por_tabela.ron", false);
        let avulsos = pb.blocks.iter().find(|v| v.name == "avulsos").unwrap();
        let shares: f64 = avulsos.type_mix.iter().map(|m| m.1).sum();
        assert_eq!(avulsos.type_mix.len(), 2);
        assert!((shares - 1.0).abs() < 1e-9 && (avulsos.type_mix[0].1 - 0.8).abs() < 1e-9);
        assert_eq!(
            avulsos.type_mix[0].0,
            looks::parse_color("#3b82f6").unwrap()
        );
        // A source that drops items blinks right after a loss, and not long after.
        let pb = playback("atendimento_com_perda.ron", false);
        let i = pb.blocks.iter().position(|v| v.name == "chegada").unwrap();
        let b = pb.blocks[i].b;
        let first = *pb.dropped[b].first().expect("a lost item");
        assert!(pb.frame(first).flash[i] > 0.9);
        let quiet = pb.dropped[b]
            .windows(2)
            .find(|w| w[1] - w[0] > 2.0 * FLASH_SECONDS)
            .map(|w| w[0] + 1.5 * FLASH_SECONDS)
            .expect("a pause between losses");
        assert_eq!(pb.frame(quiet).flash[i], 0.0);
        // Items a source holds wait at its output port, side by side, outside its footprint.
        let a = port_slot([0.8, 1.0, 0.8], 0, 3);
        let c = port_slot([0.8, 1.0, 0.8], 2, 3);
        assert!(a[0] - ITEM_SIZE / 2.0 > 0.8);
        assert!((a[2] + c[2]).abs() < 1e-6 && c[2] - a[2] >= 2.0 * ITEM_SIZE);
    }

    #[test]
    fn a_batch_in_flight_travels_side_by_side() {
        // The slots: distinct, centred, at least an item apart.
        for n in 1..10 {
            let slots: Vec<[f32; 2]> = (0..n).map(|k| flight_slot(k, n)).collect();
            let mean = slots
                .iter()
                .fold([0.0, 0.0], |a, s| [a[0] + s[0], a[1] + s[1]]);
            assert!(
                mean[0].abs() / n as f32 <= ITEM_SIZE && mean[1].abs() / n as f32 <= ITEM_SIZE,
                "{n}: centrado"
            );
            for i in 0..n {
                for j in i + 1..n {
                    let d = ((slots[i][0] - slots[j][0]).powi(2)
                        + (slots[i][1] - slots[j][1]).powi(2))
                    .sqrt();
                    assert!(d >= ITEM_SIZE, "{n}: {i} e {j}");
                }
            }
        }
        // In a run with batches, no two items ever share a place in a frame.
        let pb = playback("chegadas_por_tabela.ron", false);
        let end = pb.end().min(3000.0);
        let mut t = 0.0;
        while t <= end {
            let f = pb.frame(t);
            let mut seen = std::collections::HashSet::new();
            for (k, at) in f.item_position.iter().enumerate() {
                if f.item_carrier[k] >= 0 {
                    continue;
                }
                let key = at.map(|v| (v * 1000.0).round() as i64);
                assert!(seen.insert(key), "t = {t}: dois itens em {at:?}");
            }
            t += 7.0;
        }
    }

    #[test]
    fn a_source_label_says_how_it_creates_and_how_many() {
        let pb = playback("chegadas_por_tabela.ron", false);
        let (i, v) = pb
            .blocks
            .iter()
            .enumerate()
            .find(|(_, v)| v.kind == "Source" && !v.badge.is_empty())
            .expect("a source with a table");
        assert!(
            v.badge.contains("tabela") || v.badge.contains("sequência"),
            "{}",
            v.badge
        );
        let first = pb.frame(0.0).label[i].clone();
        let last = pb.frame(pb.end()).label[i].clone();
        assert!(first.contains(&v.badge), "{first}");
        assert!(last.contains(" criados"), "{last}");
        let count = |l: &str| {
            l.split(" · ")
                .find_map(|p| p.strip_suffix(" criados")?.parse::<usize>().ok())
                .unwrap_or(0)
        };
        assert!(count(&last) > count(&first), "{first} / {last}");
        // A plain source has no badge.
        let plain = playback("mm1.ron", false);
        let s = plain.blocks.iter().find(|v| v.kind == "Source").unwrap();
        assert!(s.badge.is_empty());
    }

    #[test]
    fn team_lines_and_the_free_members_come_with_the_frame() {
        let pb = playback("equipe_com_prioridade.ron", false);
        let at = |n: &str| pb.blocks.iter().position(|v| v.name == n).unwrap();
        let (d, op1, op2) = (at("operadores"), at("op1"), at("op2"));
        let team: Vec<_> = pb.links.iter().filter(|l| l.kind == "equipe").collect();
        assert_eq!(team.len(), 2);
        assert!(team.iter().all(|l| l.from == d));
        assert!(team.iter().any(|l| l.to == Some(op1)) && team.iter().any(|l| l.to == Some(op2)));
        // The processors ask the team for operators; the members have home and break places.
        assert!(
            pb.links
                .iter()
                .any(|l| l.kind == "operadores" && l.to == Some(d))
        );
        assert!(
            pb.links
                .iter()
                .any(|l| l.kind == "casa" && l.from == op1 && l.to.is_none())
        );
        for t in [0.0, 60.0, 300.0] {
            let f = pb.frame(t);
            assert_eq!(f.task_target.len(), pb.blocks.len());
            assert!(f.label[d].contains("/2 livres"), "{}", f.label[d]);
            // Only mobile resources have a task target.
            assert!(f.task_target[at("torno")][0].is_nan());
        }
        // Someone travels at some point of the run.
        let moving = (0..200).any(|k| {
            let f = pb.frame(k as f64 * 10.0);
            !f.task_target[op1][0].is_nan() || !f.task_target[op2][0].is_nan()
        });
        assert!(moving);
    }

    #[test]
    fn a_queue_drawn_as_a_conveyor_keeps_its_items_on_the_belt() {
        let model = Model::parse(
            r#"Model(
                name: "fila na esteira",
                run: (stop_time: 300.0, seed: 3, replications: 1),
                objects: [
                    (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0)), position: (0.0, 0.0, 0.0)),
                    (name: "fila", kind: Queue(), position: (5.0, 0.0, 0.0), visual: (asset: "esteira")),
                    (name: "maquina", kind: Processor(capacity: 1, process: Exp(mean: 4.0)), position: (10.0, 0.0, 0.0)),
                    (name: "saida", kind: Sink, position: (14.0, 0.0, 0.0)),
                ],
                connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
            )"#,
        )
        .unwrap();
        let replay = model.replay(0).expect("replay");
        let pb = Playback::new(&replay, Some(&model), None, false);
        let q = pb.blocks.iter().position(|v| v.name == "fila").unwrap();
        let belt = looks::queue_belt("Queue", pb.blocks[q].look.as_ref().unwrap()).unwrap();
        let base = pb.static_pose(q).0;
        for t in [30.0, 150.0, 300.0] {
            let f = pb.frame(t);
            let mut on = 0;
            for p in &f.item_position {
                let d = [p[0] - base[0], p[1] - base[1], p[2] - base[2]];
                // Over the belt's footprint (Godot: y up; the belt runs along x).
                if d[0].abs() <= belt.length / 2.0 + 0.5 && d[2].abs() <= belt.width / 2.0 + 0.5 {
                    assert!(
                        d[1] - ITEM_SIZE / 2.0 >= belt.top - 1e-3,
                        "t {t}: {d:?} under the belt top"
                    );
                    if (d[1] - belt.rest_height(ITEM_SIZE)).abs() < 1e-3 {
                        on += 1;
                    }
                }
            }
            assert!(
                on >= belt.per_layer(ITEM_SIZE),
                "t {t}: {on} items on the belt"
            );
            assert!(f.label[q].contains('+'), "{}", f.label[q]);
        }
    }

    #[test]
    fn arrays_have_one_entry_per_block_and_per_item() {
        let pb = playback("centro_de_distribuicao.ron", true);
        assert!(!pb.blocks.is_empty());
        for t in [0.0, pb.end() * 0.3, pb.end() * 0.7, pb.end()] {
            let f = pb.frame(t);
            let n = pb.blocks.len();
            for len in [
                f.position.len(),
                f.yaw.len(),
                f.lift.len(),
                f.motion.len(),
                f.param.len(),
                f.color.len(),
                f.rolled.len(),
                f.clip.len(),
                f.clip_time.len(),
                f.label.len(),
            ] {
                assert_eq!(len, n);
            }
            let m = f.item_id.len();
            for len in [
                f.item_position.len(),
                f.item_yaw.len(),
                f.item_scale.len(),
                f.item_color.len(),
                f.item_carrier.len(),
                f.item_stack.len(),
            ] {
                assert_eq!(len, m);
            }
        }
    }

    #[test]
    fn fixed_blocks_sit_where_the_scene_contract_puts_them() {
        let pb = playback("centro_de_distribuicao.ron", false);
        let header = pb.scene.header();
        let f = pb.frame(0.0);
        for (i, v) in pb.blocks.iter().enumerate() {
            if v.role.is_some() {
                continue;
            }
            let e = header
                .scene
                .entities
                .iter()
                .find(|e| e.id == EntityId::Block(v.b as u32))
                .unwrap();
            let g = to_godot(e.position);
            for k in 0..3 {
                assert!(
                    (f.position[i][k] - g[k]).abs() < 1e-4,
                    "{}: {:?} vs {g:?}",
                    v.name,
                    f.position[i]
                );
            }
            assert_eq!(pb.static_pose(i).0, g);
        }
    }

    #[test]
    fn mobiles_match_the_scene_frame_in_godot_coordinates() {
        // Without machines in the way the drawn pose is the scene's pose: compare the
        // packed position with the contract frame mapped by (x, z, -y).
        let pb = playback("agvs.ron", false);
        for t in [1.0, pb.end() / 2.0] {
            let f = pb.frame(t);
            let contract = pb.scene.frame(t);
            for (i, v) in pb.blocks.iter().enumerate() {
                if v.role != Some(MobileRole::Agv) {
                    continue;
                }
                let pose = contract
                    .poses
                    .iter()
                    .find(|p| p.id == EntityId::Block(v.b as u32))
                    .unwrap();
                let g = to_godot(pose.position);
                let inside = pb.stations.iter().any(|&st| {
                    let (c, h) = (pb.scene.positions[st], pb.scene.geoms[st].half);
                    (g[0] - c[0]).abs() < h[0] + 1.0 && (g[2] - c[2]).abs() < h[2] + 1.0
                });
                if inside {
                    continue;
                }
                for (a, b) in f.position[i].iter().zip(g) {
                    assert!((a - b).abs() < 1e-4);
                }
                assert!((f.yaw[i] - pose.yaw).abs() < 1e-5);
                assert_eq!(motion_code(pose.motion).0, f.motion[i]);
            }
        }
    }

    #[test]
    fn people_carry_and_get_clips() {
        let mut pb = playback("pessoas_carregando.ron", true);
        pb.set_clip_durations(&[2.0, 1.333, 0.933, 2.0, 3.0, 2.5, 2.0]);
        let people: Vec<usize> = (0..pb.blocks.len())
            .filter(|&i| pb.blocks[i].look.as_ref().is_some_and(|l| l.skeletal))
            .collect();
        assert!(!people.is_empty());
        let mut walked = false;
        let mut held = false;
        let mut k = 0.0;
        while k <= pb.end() {
            let f = pb.frame(k);
            for &i in &people {
                assert!(f.clip[i] >= 0);
                walked |= f.clip[i] == Clip::Walk.index() as i32
                    || f.clip[i] == Clip::Carry.index() as i32;
            }
            held |= f.item_carrier.iter().any(|&c| c >= 0);
            k += pb.end() / 200.0;
        }
        assert!(walked && held);
    }

    #[test]
    fn forklifts_get_a_seated_driver_that_drives_when_moving() {
        let mut pb = playback("centro_de_distribuicao.ron", true);
        pb.set_clip_durations(&[2.0, 1.333, 0.933, 2.0, 3.0, 2.5, 2.0, 1.667, 1.667]);
        let forklifts: Vec<usize> = (0..pb.blocks.len())
            .filter(|&i| pb.blocks[i].role == Some(MobileRole::Forklift))
            .collect();
        assert!(!forklifts.is_empty());
        let (mut drove, mut sat) = (false, false);
        let mut k = 0.0;
        while k <= pb.end() {
            let f = pb.frame(k);
            for &i in &forklifts {
                assert!(pb.blocks[i].driver.is_some());
                let (code, _) = (f.motion[i], 0);
                let want = driver::driver_clip(match code {
                    1 => Motion::Moving { speed: 1.0 },
                    2 => Motion::Loading { progress: 0.5 },
                    3 => Motion::Unloading { progress: 0.5 },
                    _ => Motion::Idle,
                });
                assert_eq!(f.clip[i], want.global_index() as i32, "t={k} motion={code}");
                assert!(f.clip_time[i] >= 0.0 && f.clip_time[i] < 1.7);
                drove |= want == DriverClip::Drive;
                sat |= want == DriverClip::Sit;
            }
            k += pb.end() / 300.0;
        }
        assert!(drove && sat);
        // Without the mannequin files there is no driver, and the clip stays -1.
        let bare = playback("centro_de_distribuicao.ron", false);
        assert!(bare.blocks.iter().all(|v| v.driver.is_none()));
        assert!(bare.frame(10.0).clip.iter().all(|&c| c == -1));
    }

    #[test]
    fn parametric_assets_are_generated_and_defaults_imported() {
        let pb = playback("centro_de_distribuicao.ron", false);
        let mut generated = 0;
        for v in &pb.blocks {
            let Some(look) = &v.look else { continue };
            let a = AssetInfo::of(&look.source);
            if a.source == "library" && a.key != a.name {
                let glb = pb.glb(&a.key).expect("bytes");
                assert_eq!(&glb[0..4], b"glTF");
                generated += 1;
            }
        }
        assert!(generated > 0, "the racks of this model have their own size");
        assert!(pb.pallets);
    }
}
