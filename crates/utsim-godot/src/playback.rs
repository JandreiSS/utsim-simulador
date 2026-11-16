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
use utsim_scene::compose::{ITEM_SIZE, turned};
use utsim_scene::style::{self, Shape};
use utsim_scene::{EntityId, EntityKind, LinkKind, MobileRole, Motion, Scene, SceneOptions};

use crate::coords::{layout_to_godot, size_to_godot, to_godot, yaw_to_godot};
use utsim_look::driver::{self, DriverClip, DriverMount};

use crate::looks::{self, Carry, Clip, Context, Look, Source};
use crate::modelos3d::{self, Animado, Catalogo, Pedido};

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
    /// How far the seated driver leans forward, radians (`sentado_inclinar` of the
    /// catalogue; its root and eyes are already turned about the hip).
    pub driver_lean: f32,
    /// Model coordinates of the base, and the footprint `(x, y, z)` size.
    pub position: [f32; 3],
    pub size: [f32; 3],
    /// A queue with `discipline: Lifo` (a stack): labelled and drawn as one.
    pub stack: bool,
    /// Dimensions given in the model (`size`): the asset is fitted to the box `size`
    /// instead of keeping its own proportions.
    pub sized: bool,
    /// A source's arrival mode, for the badge of its label (`horário`, `tabela`,
    /// `sequência`, `taxa`, with `lote` for batches); empty for the plain interval.
    pub badge: String,
    /// The item types a source creates, with their colours and shares (C7: the ring over
    /// it); empty when it does not name types.
    pub type_mix: Vec<([f32; 3], f64)>,
    /// The Blender element it is drawn with (`assets/modelos3d/<stem>.glb`), when there is
    /// one ([`modelos3d`]): it replaces the library file of the look, whose scale, turn and
    /// offset still apply.
    pub modelo3d: Option<String>,
    /// The clips of that element, sorted (the indices of the frame's `anim_clip`).
    pub clips: Vec<String>,
    /// A person drawn with a Blender body: file, protective equipment, colours, height.
    pub pessoa: Option<modelos3d::Pessoa>,
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
    /// Blocks drawn with a Blender element: the clip playing (index into
    /// [`BlockView::clips`], -1 for none) and its phase, 0 to 1 ([`Animado::anim`]).
    pub anim_clip: Vec<i32>,
    pub anim_fase: Vec<f32>,
    /// Rise of the forks past the fork clip's travel (a rack cell above it), asset units,
    /// for the engine to add to the `carro` node; 0 for the others.
    pub anim_extra: Vec<f32>,
    /// The hook of each crane, in its own frame (`u`, `v`, height; [`Playback::ponte_pose`]);
    /// NaN for the other blocks.
    pub ponte: Vec<[f32; 3]>,
    /// Per crane: 1 while an item hangs from its hook (the square under the load is
    /// shown), else 0; NaN for the other blocks.
    pub ponte_carga: Vec<f32>,
    /// Per drawn item: the block whose element holds it at one of its points (the part a
    /// station works on, the load on the forks or the deck), -1 for none, and the point
    /// (index into that block's static `pontos`). The engine puts the item there after
    /// the clip (it rides the gripper, the forks, the deck); `item_position` already has
    /// it for a point of the root.
    pub item_ancora_bloco: Vec<i32>,
    pub item_ancora_ponto: Vec<i32>,
}

/// How far outside a station's footprint an operator working there may stand (its access
/// point) and still be put at the station's `ponto_operador`, metres.
pub const POST_REACH: f32 = 2.0;

/// A real waiting area in the 3D view ([`utsim_model::WaitingArea`]): the centre of the
/// base of each place in the object's own frame (Godot axes: `x` towards the exit, `y`
/// up, `z` towards the back), and where the object stands.
#[derive(Clone, Debug, PartialEq)]
pub struct Espera3d {
    pub places: Vec<[f32; 3]>,
    /// Where the row of items waiting beside the area starts (past its exit end).
    pub beside_x: f32,
    pub root: [f32; 3],
    pub turn: f32,
}

/// An overhead crane (`Crane`) in the 3D view, drawn with the `ponte_rolante` element: its
/// rectangle (centre, turn and `size`), the height the hook travels at and what the scene
/// needs to stretch the element to the size (`json`: the catalogue's `dados`, the size and
/// the rail height, read by `cena.gd`).
#[derive(Clone, Debug, PartialEq)]
pub struct Ponte3d {
    /// Model coordinates of the centre of the rectangle.
    pub centre: [f64; 3],
    pub yaw: f64,
    pub hook_height: f64,
    pub json: String,
    /// The hoisting of the hook from the trace ([`utsim_core::TraceEvent::Hoist`]), in time
    /// order: start, duration, world height from and to. Empty in older traces.
    pub hoists: Vec<(f64, f64, f64, f64)>,
    /// When an item hangs from the hook ([`utsim_model::exposure::LoadTrack::hanging`]).
    pub hanging: Vec<(f64, f64)>,
}

/// Height of the hook above the floor while hooking or unhooking a load in traces without
/// hoisting (older ones), m.
pub const HOOK_WORK: f64 = 1.2;

impl Ponte3d {
    /// The crane `o` drawn with element `e`; `None` without a `size` (the model requires it).
    pub fn new(o: &utsim_model::ObjectDef, e: &modelos3d::Elemento) -> Option<Ponte3d> {
        let utsim_model::ObjectKind::Crane { hook_height, .. } = &o.kind else {
            return None;
        };
        let size = o.size?;
        let rail_base = e.dados["altura_trilho"].as_f64().unwrap_or(6.0);
        // The square under the load (`load_zone`, half side), the same the safety reads.
        let zona = match &o.kind {
            utsim_model::ObjectKind::Crane { load_zone, .. } => *load_zone,
            _ => 0.0,
        };
        let tall_base = f64::from(e.tamanho[2]).max(rail_base);
        let rail = size[2] * rail_base / tall_base;
        let json = serde_json::json!({
            "dados": e.dados,
            "tamanho": size,
            "trilho": rail,
            "zona": zona,
        })
        .to_string();
        Some(Ponte3d {
            centre: o.position,
            yaw: o
                .visual
                .as_ref()
                .and_then(|v| v.yaw)
                .map_or(0.0, f64::to_radians),
            hook_height: *hook_height,
            json,
            hoists: Vec::new(),
            hanging: Vec::new(),
        })
    }

    /// The world height of the hook at `t` by the hoisting of the trace; `None` before the
    /// first hoist or without them.
    pub fn hoist_at(&self, t: f64) -> Option<f64> {
        let k = self.hoists.partition_point(|h| h.0 <= t);
        let &(t0, dur, from, to) = self.hoists.get(k.checked_sub(1)?)?;
        Some(if dur > 0.0 && t < t0 + dur {
            from + (to - from) * (t - t0) / dur
        } else {
            to
        })
    }

    /// Whether an item hangs from the hook at `t` (the square under the load is shown).
    pub fn loaded(&self, t: f64) -> bool {
        self.hanging.iter().any(|&(a, b)| a <= t && t < b)
    }

    /// The hook in the crane's own frame: along the runway (`u`), across (`v`), and its
    /// height above the floor, for a hook over `at` (model coordinates) and the crane's
    /// state.
    pub fn hook(&self, at: [f64; 3], state: Option<utsim_core::State>) -> [f64; 3] {
        let (s, c) = self.yaw.sin_cos();
        let (dx, dy) = (at[0] - self.centre[0], at[1] - self.centre[1]);
        let z = match state {
            Some(utsim_core::State::Loading | utsim_core::State::Unloading) => {
                HOOK_WORK.min(self.hook_height)
            }
            _ => self.hook_height,
        };
        [c * dx + s * dy, -s * dx + c * dy, z]
    }

    /// The model point of the hook `h` ([`Ponte3d::hook`]).
    pub fn global(&self, h: [f64; 3]) -> [f64; 3] {
        let (s, c) = self.yaw.sin_cos();
        [
            self.centre[0] + c * h[0] - s * h[1],
            self.centre[1] + s * h[0] + c * h[1],
            self.centre[2] + h[2],
        ]
    }
}

/// Items drawn in the row beside a full waiting area; the others are only counted.
pub const BESIDE_MAX: usize = 6;

impl Espera3d {
    pub fn new(
        w: &utsim_model::WaitingArea,
        size: Option<[f64; 3]>,
        root: [f32; 3],
        turn: f32,
    ) -> Espera3d {
        let places = w
            .places(size)
            .iter()
            .map(|p| to_godot(p.center.map(|v| v as f32)))
            .collect();
        let half = size.unwrap_or_else(|| w.default_size())[0] as f32 / 2.0;
        Espera3d {
            places,
            beside_x: half + ITEM_SIZE,
            root,
            turn,
        }
    }

    /// The `k`-th item beside the area (on the floor, in a row past its exit end), or
    /// `None` past [`BESIDE_MAX`].
    pub fn beside(&self, k: usize) -> Option<[f32; 3]> {
        (k < BESIDE_MAX).then_some([self.beside_x + k as f32 * ITEM_SIZE * 1.1, 0.0, 0.0])
    }

    /// The centre of an item whose base is at `local` (the object's frame), in the layout.
    pub fn to_layout(&self, local: [f32; 3]) -> [f32; 3] {
        let w = turned(local, self.turn);
        [
            self.root[0] + w[0],
            self.root[1] + w[1] + ITEM_SIZE / 2.0,
            self.root[2] + w[2],
        ]
    }

    /// Items not drawn (past the places and the row beside the area).
    pub fn hidden(&self, content: usize) -> usize {
        content.saturating_sub(self.places.len() + BESIDE_MAX)
    }
}

/// Distance between operators who work at the same station together (operators per
/// piece), side by side across its post, metres.
pub const POST_SPACING: f32 = 0.7;

/// Operators at the same station (the drawn index each pose was put at) stand side by
/// side across its post, in the order of the blocks, the middle of the row on the post:
/// two operators per piece do not stand inside each other.
fn spread_at_posts(poses: &mut [Option<(utsim_scene::MobilePose, Option<usize>)>]) {
    let mut by: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, p) in poses.iter().enumerate() {
        if let Some((_, Some(st))) = p {
            by.entry(*st).or_default().push(k);
        }
    }
    for list in by.values().filter(|l| l.len() > 1) {
        let n = list.len() as f32;
        for (r, &k) in list.iter().enumerate() {
            if let Some((p, _)) = poses[k].as_mut() {
                let off = (r as f32 - (n - 1.0) / 2.0) * POST_SPACING;
                let side = turned([0.0, 0.0, 1.0], p.yaw);
                p.position[0] += side[0] * off;
                p.position[2] += side[2] * off;
            }
        }
    }
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
    /// The catalogue of the Blender elements, when its folder was found.
    pub modelos3d: Option<Catalogo>,
    /// Per drawn block, its Blender element resolved once (clips, state table, operator
    /// post and walks in the layout) and the runs of its states as the table sees them
    /// ([`Animado::runs`]); `None` for the others.
    pub animados: Vec<Option<(Animado, modelos3d::Runs)>>,
    /// Per drawn block, the top of the waiting table a queue is drawn with (`espera`) and
    /// where the table is placed; `None` for the others.
    pub tampos: Vec<Option<(modelos3d::Tampo, modelos3d::Placement)>>,
    /// Per drawn block, the real waiting area of a queue (`waiting_area`): its places and
    /// where it stands; `None` for the others.
    pub esperas: Vec<Option<Espera3d>>,
    /// Per drawn block, the overhead crane it is; `None` for the others.
    pub pontes: Vec<Option<Ponte3d>>,
    /// The live numbers of every block (the panel of Simular, `ui_ao_vivo`), built on
    /// the first use from the trace kept by [`Playback::com_rastro`], which then goes.
    vivo: std::sync::OnceLock<crate::ao_vivo::AoVivo>,
    rastro: std::sync::Mutex<Option<Replay>>,
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
        let catalogo = Catalogo::find(model_dir);
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
            // A queue with a real waiting area is drawn with the element of its kind (the
            // table, the pallet, the shelf, the marked floor).
            let espera_visual = model
                .and_then(|m| m.objects.iter().find(|o| o.name == name))
                .and_then(|o| o.waiting_area.filter(|_| kind == "Queue"))
                .filter(|w| {
                    catalogo
                        .as_ref()
                        .is_some_and(|c| c.get(w.element()).is_some())
                })
                .map(|w| utsim_model::VisualDef {
                    asset: Some(w.element().to_string()),
                    ..scene.visual(b).cloned().unwrap_or_default()
                });
            let espera_visual = espera_visual.or_else(|| {
                (kind == "Crane"
                    && catalogo
                        .as_ref()
                        .is_some_and(|c| c.get("ponte_rolante").is_some()))
                .then(|| utsim_model::VisualDef {
                    asset: Some("ponte_rolante".to_string()),
                    ..scene.visual(b).cloned().unwrap_or_default()
                })
            });
            let visual = espera_visual.as_ref().or(scene.visual(b));
            let mut look = looks::look_of(&kind, role, visual, &scene.geoms[b], ctx, model_dir);
            let (modelo3d, pessoa) = match &catalogo {
                Some(cat) => blender_look(cat, &mut look, &name, visual, model, |v| {
                    looks::look_of(&kind, role, Some(v), &scene.geoms[b], ctx, model_dir)
                }),
                None => (None, None),
            };
            let clips = modelo3d
                .as_deref()
                .zip(catalogo.as_ref())
                .and_then(|(s, c)| c.get(s))
                .map(modelos3d::Elemento::clip_names)
                .unwrap_or_default();
            index_of[b] = Some(blocks.len());
            let mut driver = look
                .as_ref()
                .filter(|_| skeletal_people)
                .and_then(|l| driver::driver_mount(role, l));
            let lean = catalogo.as_ref().map_or(0.0, modelos3d::sentado_inclinar);
            let driver_lean = driver
                .as_mut()
                .map_or(0.0, |m| modelos3d::inclinar_motorista(m, lean));
            let stack = kind == "Queue" && is_stack(model, &name);
            let sized = role.is_none()
                && !matches!(kind.as_str(), "Conveyor" | "Rack" | "Crane")
                && model
                    .and_then(|m| m.objects.iter().find(|o| o.name == name))
                    .is_some_and(|o| o.size.is_some());
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
                modelo3d,
                clips,
                pessoa,
                stack,
                sized,
                badge,
                type_mix,
                name,
                kind,
                role,
                look,
                driver,
                driver_lean,
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
            modelos3d: catalogo,
            animados: Vec::new(),
            tampos: Vec::new(),
            esperas: Vec::new(),
            pontes: Vec::new(),
            vivo: std::sync::OnceLock::new(),
            rastro: std::sync::Mutex::new(None),
        }
        .with_blender(replay)
    }

    /// Resolves the Blender element of every block once (clips, state table, operator
    /// post and walks in the layout) and the runs of its states in the trace.
    fn with_blender(mut self, replay: &Replay) -> Playback {
        let mut animados: Vec<Option<(Animado, modelos3d::Runs)>> = (0..self.blocks.len())
            .map(|i| {
                let v = &self.blocks[i];
                if v.pessoa.is_some() {
                    return None;
                }
                let stem = v.modelo3d.as_deref()?;
                let e = self.modelos3d.as_ref()?.get(stem)?;
                let auto = self.unmanned(i);
                Some((
                    Animado::resolve(e, stem, &self.placement(i)?, auto),
                    Vec::new(),
                ))
            })
            .collect();
        let mut states: HashMap<usize, Vec<(f64, utsim_core::State)>> = HashMap::new();
        for r in &replay.records {
            if let utsim_core::TraceEvent::State { block, state } = r.event
                && let Some(i) = self.drawn_index(block)
                && animados[i].is_some()
            {
                states.entry(i).or_default().push((r.t, state));
            }
        }
        for (i, list) in states {
            if let Some((a, runs)) = animados[i].as_mut() {
                *runs = a.runs(list);
            }
        }
        self.animados = animados;
        let tracks = self
            .model
            .as_ref()
            .map(|m| utsim_model::exposure::load_tracks(m, &replay.records, replay.stop_time))
            .unwrap_or_default();
        self.pontes = (0..self.blocks.len())
            .map(|i| {
                let v = &self.blocks[i];
                if v.kind != "Crane" {
                    return None;
                }
                let o = self
                    .model
                    .as_ref()?
                    .objects
                    .iter()
                    .find(|o| o.name == v.name)?;
                let e = self.modelos3d.as_ref()?.get(v.modelo3d.as_deref()?)?;
                let mut p = Ponte3d::new(o, e)?;
                p.hoists = replay
                    .records
                    .iter()
                    .filter_map(|r| match r.event {
                        utsim_core::TraceEvent::Hoist {
                            carrier,
                            from,
                            to,
                            duration,
                        } if carrier == v.b => Some((r.t, duration, from, to)),
                        _ => None,
                    })
                    .collect();
                if let Some(tr) = tracks.iter().find(|tr| tr.name == v.name) {
                    p.hanging = tr.hanging.clone();
                }
                Some(p)
            })
            .collect();
        self.esperas = (0..self.blocks.len())
            .map(|i| {
                let v = &self.blocks[i];
                let o = self
                    .model
                    .as_ref()?
                    .objects
                    .iter()
                    .find(|o| o.name == v.name)?;
                let w = o.waiting_area.filter(|_| v.kind == "Queue")?;
                Some(Espera3d::new(
                    &w,
                    o.size,
                    self.static_pose(i).0,
                    self.turn(i),
                ))
            })
            .collect();
        self.tampos = (0..self.blocks.len())
            .map(|i| {
                let v = &self.blocks[i];
                if v.kind != "Queue" || self.esperas[i].is_some() {
                    return None;
                }
                let e = self.modelos3d.as_ref()?.get(v.modelo3d.as_deref()?)?;
                Some((modelos3d::Tampo::of(e)?, self.placement(i)?))
            })
            .collect();
        self
    }

    /// Keeps the trace of `replay` (the one the scene was built from) for the live panel:
    /// it is read only if a panel opens ([`Playback::vivo`]).
    pub fn com_rastro(self, replay: Replay) -> Playback {
        if let Ok(mut r) = self.rastro.lock() {
            *r = Some(replay);
        }
        self
    }

    /// The live numbers of every block, read from the kept trace on the first call;
    /// `None` without a trace ([`Playback::com_rastro`]).
    pub fn vivo(&self) -> Option<&crate::ao_vivo::AoVivo> {
        if let Some(v) = self.vivo.get() {
            return Some(v);
        }
        let replay = self.rastro.lock().ok()?.take()?;
        Some(self.vivo.get_or_init(|| {
            crate::ao_vivo::AoVivo::new(&replay, self.model.as_ref(), |b| self.scene.role(b))
        }))
    }

    /// The asset of block `i` as the engine loads it, and the library asset to fall back
    /// on when it is a Blender element (`source: "modelos3d"`).
    pub fn asset_of(&self, i: usize) -> Option<(AssetInfo, Option<AssetInfo>)> {
        let v = &self.blocks[i];
        let base = AssetInfo::of(&v.look.as_ref()?.source);
        match (&v.modelo3d, &self.modelos3d) {
            (Some(stem), Some(cat)) => Some((
                AssetInfo {
                    source: "modelos3d",
                    name: stem.clone(),
                    key: stem.clone(),
                    path: cat.path(stem).to_string_lossy().replace('\\', "/"),
                },
                Some(base),
            )),
            _ => Some((base, None)),
        }
    }

    /// How the asset of block `i` is placed: its scale, turn and offset under the root,
    /// and the root.
    pub fn placement(&self, i: usize) -> Option<modelos3d::Placement> {
        let look = self.blocks[i].look.as_ref()?;
        let (asset_yaw, offset) = self.asset_pose(i)?;
        Some(modelos3d::Placement {
            scale: look.scale,
            asset_yaw,
            offset,
            root: self.static_pose(i).0,
            turn: self.turn(i),
        })
    }

    /// The Blender element of block `i` and the run of its states at `t`.
    fn animado(&self, i: usize, t: f64) -> Option<(&Animado, Option<modelos3d::Run>)> {
        let (a, runs) = self.animados.get(i)?.as_ref()?;
        Some((a, modelos3d::run_at(runs, t)))
    }

    /// Where the operator of station `i` (drawn index) stands at `t`, from its Blender
    /// element: along the walk of the state's line (`entering`: door to post; `leaving`:
    /// back), else at its `ponto_operador`. `None`: no such point.
    pub fn operator_post(&self, i: usize, t: f64) -> Option<([f32; 3], f32)> {
        let (a, run) = self.animado(i, t)?;
        a.operator(run, t)
    }

    /// The hook of crane `i` (drawn index) at `t`, in its own frame (`u`, `v`, height):
    /// over the last point of its travels (its centre before the first one), at the height
    /// of the trace's hoisting (older traces: down while hooking or unhooking).
    pub fn ponte_pose(&self, i: usize, t: f64) -> Option<[f64; 3]> {
        let p = self.pontes.get(i)?.as_ref()?;
        let b = self.blocks[i].b;
        let at = self.scene.timeline.carrier_pos(b, t).unwrap_or(p.centre);
        let mut h = p.hook(at, self.scene.timeline.block_state(b, t));
        if let Some(z) = p.hoist_at(t) {
            h[2] = z - p.centre[2];
        }
        Some(h)
    }

    /// Whether crane `i` (drawn index) carries a hanging item at `t`; `None` for the other
    /// blocks.
    pub fn ponte_carga(&self, i: usize, t: f64) -> Option<bool> {
        Some(self.pontes.get(i)?.as_ref()?.loaded(t))
    }

    /// Whether block `i` is a `Processor` nobody operates (no `operators`, no `also`): its
    /// `processing` plays the element's `processing_auto` lines. Decided by the model, so
    /// it is the same at every instant.
    pub fn unmanned(&self, i: usize) -> bool {
        let name = &self.blocks[i].name;
        self.model
            .as_ref()
            .and_then(|m| m.objects.iter().find(|o| &o.name == name))
            .is_some_and(|o| {
                // Operators only for the setup (`process: false`) leave the process
                // unmanned too.
                matches!(o.kind, utsim_model::ObjectKind::Processor { .. })
                    && o.kind.operator_uses().all(|u| !u.process)
            })
    }

    /// The clip of block `i` at `t` and its phase, or `(-1, 0)`. `lift`: the block's fork
    /// height in this frame; the forks of an element with `garfo_subir` follow it (the
    /// same height the loads on them get), not the state table.
    /// The third number is the rise of the forks past the clip's travel (asset units;
    /// 0 for everything else): the engine adds it to the `carro` node after the clip.
    pub fn anim_at(&self, i: usize, t: f64, lift: f32) -> (i32, f32, f32) {
        let Some((a, run)) = self.animado(i, t) else {
            return (-1, 0.0, 0.0);
        };
        if a.garfo.is_some() {
            let scale = self.blocks[i].look.as_ref().map_or(1.0, |l| l.scale);
            return match a.fork(lift, scale) {
                Some((k, p, extra)) if p > 1e-4 => (k, p, extra),
                _ => (-1, 0.0, 0.0),
            };
        }
        let (k, p) = a.anim(run, t);
        (k, p, 0.0)
    }

    /// Where the part block `i` works on (or carries) is at `t`: an index into its
    /// element's points ([`Animado::pontos`]), from the part table of its state.
    pub fn anchor_at(&self, i: usize, t: f64) -> Option<usize> {
        let (a, run) = self.animado(i, t)?;
        a.anchor(run, t)
    }

    /// The points block `i`'s part table names (the frame's `item_ancora_ponto` indices).
    pub fn anchor_points(&self, i: usize) -> Vec<String> {
        self.animados
            .get(i)
            .and_then(Option::as_ref)
            .map_or_else(Vec::new, |(a, _)| {
                a.pontos.iter().map(|(n, _)| n.clone()).collect()
            })
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

    /// How block `i` is turned about Godot's `+Y` (radians): the turn of a fixed block
    /// ([`Scene::turn`]: its `visual.yaw`, a belt's heading), 0 for the mobiles. The engine
    /// turns the block's root by it, so the asset, the simple shape and the editor's box
    /// and nodes turn together.
    pub fn turn(&self, i: usize) -> f32 {
        yaw_to_godot(self.scene.turn(self.blocks[i].b))
    }

    /// Size of block `i` in its own frame (Godot extents), before [`Self::turn`]: a belt's
    /// length and width, not the box of the turned belt.
    pub fn local_size(&self, i: usize) -> [f32; 3] {
        let h = self.scene.geoms[self.blocks[i].b].local_half();
        [2.0 * h[0], 2.0 * h[1], 2.0 * h[2]]
    }

    /// The asset's rotation and offset under the block's root, which is already turned
    /// by [`Self::turn`]: the look's own (`Look::yaw`, `Look::offset`) without that turn.
    pub fn asset_pose(&self, i: usize) -> Option<(f32, [f32; 3])> {
        let look = self.blocks[i].look.as_ref()?;
        let turn = self.turn(i);
        Some((look.yaw - turn, turned(look.offset, -turn)))
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

    /// Mobile pose as drawn (`models::drawn_pose` of the viewer), layout space, and the
    /// station (drawn index) whose post it was put at.
    fn drawn_pose(
        &self,
        b: usize,
        t: f64,
        look: &Look,
    ) -> Option<(utsim_scene::MobilePose, Option<usize>)> {
        let s = &self.scene;
        let mut p = s.mobile_at(b, t)?;
        let original = p.position;
        let reach = match look.carry {
            Carry::Forks { forward, .. } => forward + 0.6,
            Carry::Hands => 0.35,
            _ => 0.6,
        };
        for &st in &self.stations {
            p.position = looks::keep_outside(
                p.position,
                p.yaw,
                s.positions[st],
                s.geoms[st].half,
                s.turn(st),
                reach,
            );
        }
        let mut at_post = None;
        if look.carry == Carry::Hands && p.motion == Motion::Working {
            let station = self.stations.iter().copied().find(|&st| {
                [0, 2]
                    .iter()
                    .all(|&k| (original[k] - s.positions[st][k]).abs() <= s.geoms[st].half[k])
            });
            // The Blender element's operator point (and the walk from its door), else the
            // front of the footprint. An operator working at the station's access point
            // (outside the footprint) goes to the element's point too.
            let placed = match station {
                Some(st) => {
                    let i = self.drawn_index(st);
                    let post = i.and_then(|i| self.operator_post(i, t)).unwrap_or_else(|| {
                        looks::station_front(s.positions[st], s.geoms[st].half, s.turn(st))
                    });
                    Some((post, i))
                }
                None => self
                    .near_station(original)
                    .and_then(|i| Some((self.operator_post(i, t)?, Some(i)))),
            };
            if let Some((post, i)) = placed {
                (p.position, p.yaw) = post;
                at_post = i;
            }
        }
        Some((p, at_post))
    }

    /// The nearest station (machine or bench, drawn index) whose footprint, grown by
    /// [`POST_REACH`], holds `at` (layout): an operator working at that station's access
    /// point goes to its post, if it has one (never to a neighbour's).
    fn near_station(&self, at: [f32; 3]) -> Option<usize> {
        let s = &self.scene;
        self.stations
            .iter()
            .filter_map(|&st| {
                let i = self.drawn_index(st)?;
                let c = s.positions[st];
                let d = turned([at[0] - c[0], 0.0, at[2] - c[2]], -s.turn(st));
                let h = s.geoms[st].half;
                let inside = d[0].abs() <= h[0] + POST_REACH && d[2].abs() <= h[2] + POST_REACH;
                inside.then(|| (d[0] * d[0] + d[2] * d[2], i))
            })
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, i)| i)
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
        // Where the part each block works on (or carries) goes, from its part table.
        let mut anchors: Vec<Option<usize>> = Vec::with_capacity(n);
        let mut drawn: Vec<_> = self
            .blocks
            .iter()
            .map(|v| match (&v.look, v.role) {
                (Some(look), Some(_)) => self.drawn_pose(v.b, t, look),
                (None, Some(_)) => s.mobile_at(v.b, t).map(|p| (p, None)),
                _ => None,
            })
            .collect();
        spread_at_posts(&mut drawn);
        for (v, pose) in self.blocks.iter().zip(drawn) {
            let b = v.b;
            let state = s.timeline.block_state(b, t);
            let scale = v.look.as_ref().map_or(1.0, |l| l.scale);
            let pose = pose.map(|p| p.0);
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
            let (anim_clip, anim_fase, anim_extra) = self.anim_at(f.anim_clip.len(), t, lift);
            f.anim_extra.push(anim_extra);
            f.ponte.push(
                self.ponte_pose(f.anim_clip.len(), t)
                    .map_or([f32::NAN; 3], |h| h.map(|v| v as f32)),
            );
            f.ponte_carga.push(
                self.ponte_carga(f.anim_clip.len(), t)
                    .map_or(f32::NAN, |c| if c { 1.0 } else { 0.0 }),
            );
            anchors.push(self.anchor_at(f.anim_clip.len(), t));
            f.anim_clip.push(anim_clip);
            f.anim_fase.push(anim_fase);
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
            // A queue drawn as a conveyor (or on a waiting table) counts what does not fit
            // on its belt (its top).
            let i = self.index_of[b].unwrap_or(usize::MAX);
            let hidden = match (self.esperas.get(i), self.tampos.get(i)) {
                (Some(Some(e)), _) => e.hidden(content),
                (_, Some(Some((tampo, at)))) => {
                    tampo.hidden(content, ITEM_SIZE / at.scale.max(1e-6))
                }
                _ => v
                    .look
                    .as_ref()
                    .and_then(|l| looks::queue_belt(&v.kind, l))
                    .map_or(0, |belt| belt.hidden(content, ITEM_SIZE)),
            };
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
        // Items beside a full waiting area, per block.
        let mut beside: HashMap<usize, usize> = HashMap::new();
        // Queues drawn as conveyors: an item flying in or out does not sink into the belt.
        let belts: Vec<([f32; 3], looks::QueueBelt)> = (0..self.blocks.len())
            .filter_map(|i| {
                let v = &self.blocks[i];
                let belt = looks::queue_belt(&v.kind, v.look.as_ref()?)?;
                Some((self.static_pose(i).0, belt))
            })
            .collect();
        let items = s.items_at(t);
        // The part each station with a part point works on: the item that got there last
        // (with capacity above one, the one whose processing started last).
        let mut working: HashMap<usize, (f64, u64)> = HashMap::new();
        for p in items.iter().filter(|p| p.drawn) {
            if let Some(EntityId::Block(b)) = p.parent
                && let Some(i) = self.index_of.get(b as usize).copied().flatten()
                && poses[i].is_none()
                && anchors[i].is_some()
            {
                let since = s.timeline.item_pos_since(p.item, t).1;
                let w = working.entry(i).or_insert((since, p.id));
                if since > w.0 {
                    *w = (since, p.id);
                }
            }
        }
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
                let (base, _) = self.static_pose(i);
                let size = self.local_size(i);
                // At the output side of its own frame, turned with it.
                let o = turned(
                    port_slot([size[0] / 2.0, size[1], size[2] / 2.0], *k, n),
                    self.turn(i),
                );
                *k += 1;
                center = [base[0] + o[0], base[1] + o[1], base[2] + o[2]];
                let origin = s.timeline.item_origin(p.item).unwrap_or(0);
                f.item_id.push(p.id as i64);
                f.item_position.push(layout_to_godot(center));
                f.item_yaw.push(self.turn(i));
                f.item_scale.push(1.0);
                f.item_color.push(style::origin_color(origin));
                f.item_carrier.push(-1);
                f.item_stack.push(0);
                f.item_ancora_bloco.push(-1);
                f.item_ancora_ponto.push(-1);
                continue;
            }
            let mut scale = p.scale;
            let mut carrier = -1;
            let mut stack = 0;
            let mut anchor: Option<(usize, usize)> = None;
            if let Some(EntityId::Block(b)) = p.parent
                && let Some(i) = self.index_of.get(b as usize).copied().flatten()
                && let Some(look) = &self.blocks[i].look
            {
                let v = &self.blocks[i];
                if let (Some(Some(pp)), Some(h)) = (self.pontes.get(i), self.ponte_pose(i, t)) {
                    // Hanging under the hook (the stack, one under the other).
                    let k = stacked.entry(i).or_default();
                    let low = h[2] - f64::from(ITEM_SIZE) * (0.5 + *k as f64) - 0.05;
                    *k += 1;
                    center = to_godot(pp.global([h[0], h[1], low]).map(|v| v as f32));
                } else if let Some(belt) = looks::queue_belt(&v.kind, look) {
                    // On top of the belt, in a line (the scene's slots are the simple
                    // queue's: their base is the block's).
                    let k = resting.entry(i).or_default();
                    let simple = turned(style::slot_offset(Shape::LongBox, *k), self.turn(i));
                    let Some(at) = belt.slot(*k, ITEM_SIZE) else {
                        continue;
                    };
                    *k += 1;
                    center = [
                        center[0] - simple[0] + at[0],
                        center[1] - simple[1] + at[1],
                        center[2] - simple[2] + at[2],
                    ];
                } else if let Some(Some(e)) = self.esperas.get(i) {
                    // On its own place (the one the simulation gave it), or beside the area
                    // while every place is taken.
                    let place = s
                        .timeline
                        .rack_slot(v.b, p.item, t)
                        .and_then(|k| e.places.get(k).copied());
                    let local = match place {
                        Some(c) => c,
                        None => {
                            let k = beside.entry(i).or_default();
                            let Some(c) = e.beside(*k) else {
                                continue;
                            };
                            *k += 1;
                            c
                        }
                    };
                    center = e.to_layout(local);
                } else if let Some(Some((tampo, at))) = self.tampos.get(i) {
                    // On the top of the waiting table, in its grid.
                    let k = resting.entry(i).or_default();
                    let Some(cell) = tampo.slot(*k, ITEM_SIZE / at.scale.max(1e-6)) else {
                        continue;
                    };
                    *k += 1;
                    let (on, _) = modelos3d::point_to_layout(&cell, at);
                    center = [on[0], on[1] + ITEM_SIZE / 2.0, on[2]];
                } else if v.kind == "Queue" {
                    center[1] -= looks::queue_drop();
                } else if poses[i].is_none()
                    && let Some(k) = anchors[i]
                {
                    // A station working on its part: that one at the point of its part
                    // table (the others keep the scene's slots). A point of the root is
                    // placed here; a moving one by the engine, after the clip.
                    if working.get(&i).is_some_and(|w| w.1 == p.id) {
                        anchor = Some((i, k));
                        if let Some(Some((a, _))) = self.animados.get(i)
                            && let Some((at, _)) = a.pontos[k].1
                        {
                            center = [at[0], at[1] + scale * ITEM_SIZE / 2.0, at[2]];
                        }
                    }
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
                    // A vehicle with a part table: on its load point (forks, deck), the
                    // stack on top.
                    if let Some(point) = anchors[i] {
                        anchor = Some((i, point));
                        stack = *k as i32;
                        // The deck of the AGV rises with its clip: the load with it, as
                        // the engine puts it on the deck's point.
                        if let Some(Some((a, _))) = self.animados.get(i)
                            && let Some((clip, curso)) = a.mesa
                            && f.anim_clip[i] == clip
                        {
                            center[1] += f.anim_fase[i] * curso * look.scale;
                        }
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
            f.item_ancora_bloco.push(anchor.map_or(-1, |a| a.0 as i32));
            f.item_ancora_ponto.push(anchor.map_or(-1, |a| a.1 as i32));
        }
        f
    }
}

/// The Blender element a block is drawn with, and the body of a person ([`modelos3d`]):
/// `visual.asset` naming an element draws that file (`relook` builds the look of a
/// `.glb`); a person with the default skeletal mannequin gets a Blender body (its scale is
/// the person's height, its offset follows); a library asset of fixed size gets the
/// element made to replace it. The look is adjusted in place.
fn blender_look(
    cat: &Catalogo,
    look: &mut Option<Look>,
    name: &str,
    visual: Option<&utsim_model::VisualDef>,
    model: Option<&Model>,
    relook: impl Fn(&utsim_model::VisualDef) -> Option<Look>,
) -> (Option<String>, Option<modelos3d::Pessoa>) {
    let asset = visual.and_then(|v| v.asset.as_deref());
    match modelos3d::pedido(cat, asset) {
        Some(Pedido::Elemento(stem)) => {
            let v = utsim_model::VisualDef {
                asset: Some(cat.path(&stem).to_string_lossy().into_owned()),
                ..visual.cloned().unwrap_or_default()
            };
            *look = relook(&v);
            return (look.as_ref().map(|_| stem), None);
        }
        Some(Pedido::Biblioteca { base, stem }) => {
            // `agv_medio`: the library AGV (deck, light, wheels) with the `_medio` file;
            // `esteira_medio`: the library conveyor of the model's length.
            let v = utsim_model::VisualDef {
                asset: Some(base),
                ..visual.cloned().unwrap_or_default()
            };
            *look = relook(&v);
            return (look.as_ref().and(stem), None);
        }
        None => {}
    }
    let Some(l) = look.as_mut() else {
        return (None, None);
    };
    let word = asset.map(|a| a.trim().to_lowercase());
    let default_body = match word.as_deref() {
        None | Some("pessoa" | "operador" | "person") => true,
        Some(w) => modelos3d::PESSOAS.contains(&w),
    };
    if l.skeletal && default_body {
        // What the person wears, as the exposure analysis counts it: its own `ppe` and
        // that of the objects it serves.
        let ppe: Vec<String> = model
            .and_then(|m| Some(m.worn_ppe(m.objects.iter().find(|o| o.name == name)?)))
            .map_or_else(Vec::new, |set| set.into_iter().collect());
        let color = visual.and_then(|v| v.color.as_deref());
        let corpo = model
            .and_then(|m| m.objects.iter().find(|o| o.name == name))
            .and_then(|o| o.person.as_deref());
        let Some(mut p) = modelos3d::pessoa(cat, name, word.as_deref(), &ppe, color, corpo) else {
            return (None, None);
        };
        p.escala_boneco = l.scale;
        p.offset_boneco = l.offset;
        // The mannequin's scale is its catalogue scale times the user's; the body is at
        // real size (its skeleton is `altura_base` tall).
        let mannequin = crate::catalogue::by_name(looks::PERSON).map_or(1.0, |e| e.escala_real);
        let old = l.scale;
        l.scale = old / mannequin.max(1e-6) * p.escala;
        let k = l.scale / old.max(1e-6);
        l.offset = l.offset.map(|v| v * k);
        return (Some(p.arquivo.clone()), Some(p));
    }
    let stem = match &l.source {
        Source::Library(spec) => modelos3d::stem_for_spec(spec).filter(|s| cat.get(s).is_some()),
        _ => None,
    };
    (stem.map(String::from), None)
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
    fn a_turned_object_turns_its_root_and_the_items_resting_on_it() {
        use std::f32::consts::FRAC_PI_2;
        let (mut model, path) = example("tutorial_01_caixa.ron");
        for o in model.objects.iter_mut().filter(|o| o.name == "fila") {
            o.visual = Some(utsim_model::VisualDef {
                yaw: Some(90.0),
                ..Default::default()
            });
        }
        let replay = model.replay(0).expect("replay");
        let pb = Playback::new(&replay, Some(&model), path.parent(), false);
        let i = pb.blocks.iter().position(|v| v.name == "fila").unwrap();
        let k = pb.blocks.iter().position(|v| v.name == "caixa").unwrap();
        assert!((pb.turn(i) - FRAC_PI_2).abs() < 1e-6);
        assert_eq!(pb.turn(k), 0.0);
        // The root turns: the asset under it does not turn again.
        if let Some((yaw, _)) = pb.asset_pose(i) {
            assert!(yaw.abs() < 1e-6, "{yaw}");
        }
        // The items waiting in the queue lie along its own x, turned: in its frame they
        // are in the lanes (|z| <= 0.4), and they face its way.
        let b = pb.blocks[i].b;
        let base = pb.scene.positions[b];
        let mut seen = 0;
        for t in (1..200).map(|s| s as f64 * 5.0) {
            for p in pb.scene.items_at(t) {
                if p.drawn && p.parent == Some(EntityId::Block(b as u32)) {
                    let d = [p.center[0] - base[0], 0.0, p.center[2] - base[2]];
                    let local = turned(d, -pb.turn(i));
                    assert!(local[2].abs() <= 0.41, "{local:?}");
                    assert!((p.yaw - FRAC_PI_2).abs() < 1e-6);
                    seen += 1;
                }
            }
        }
        assert!(seen > 0, "items waited in the queue");
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
                f.anim_clip.len(),
                f.anim_fase.len(),
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

    /// The model of the roteiro `g4_blender3d.json`: a CNC (the default machine) whose
    /// operator gets in for 4 s and out for 4 s around a 10 s process, a press by
    /// `visual.asset` with setup, and a person with protective equipment.
    fn blender_model(skeletal: bool) -> Playback {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../godot/roteiros/g4_blender3d.ron");
        let model = Model::load(&path).expect("model");
        let replay = model.replay(0).expect("replay");
        Playback::new(&replay, Some(&model), path.parent(), skeletal)
    }

    fn index(pb: &Playback, name: &str) -> usize {
        pb.blocks.iter().position(|v| v.name == name).unwrap()
    }

    #[test]
    fn blender_elements_replace_the_fixed_library_assets() {
        let pb = blender_model(true);
        let cnc = index(&pb, "cnc_1");
        let (a, fallback) = pb.asset_of(cnc).unwrap();
        assert_eq!((a.source, a.name.as_str()), ("modelos3d", "cnc"));
        assert!(Path::new(&a.path).is_file(), "{}", a.path);
        assert_eq!(fallback.unwrap().name, "maquina");
        assert_eq!(
            pb.blocks[cnc].clips,
            ["ciclo", "ciclo_auto", "porta_abrir", "quebra", "setup"]
        );
        // The library's look still places it: same scale, turn and offset.
        let look = pb.blocks[cnc].look.as_ref().unwrap();
        assert_eq!((look.scale, look.offset), (1.0, [0.0; 3]));
        // Named by `visual.asset`.
        let press = index(&pb, "prensa_1");
        assert_eq!(pb.asset_of(press).unwrap().0.name, "prensa");
        // A queue keeps its floor area (sized by the model), with no clips.
        let queue = index(&pb, "fila");
        assert_eq!(pb.asset_of(queue).unwrap().0.source, "library");
        assert!(pb.blocks[queue].clips.is_empty());
        let f = pb.frame(10.0);
        assert_eq!((f.anim_clip[queue], f.anim_fase[queue]), (-1, 0.0));
    }

    #[test]
    fn stations_play_the_clip_of_their_state() {
        let pb = blender_model(true);
        let cnc = index(&pb, "cnc_1");
        let clip = |name: &str| pb.blocks[cnc].clips.iter().position(|c| c == name).unwrap() as i32;
        // The trace: entering 2.45..6.45, processing 6.45..16.45, leaving 16.45..20.45.
        let at = |t: f64| {
            let f = pb.frame(t);
            (f.anim_clip[cnc], f.anim_fase[cnc])
        };
        let (k, p) = at(4.45);
        assert_eq!(k, clip("porta_abrir"));
        assert!((p - 0.5).abs() < 1e-3, "the door half open: {p}");
        assert!(at(2.46).1 < 0.01 && at(6.44).1 > 0.99);
        let (k, p) = at(18.45);
        assert_eq!(k, clip("porta_abrir"));
        assert!((p - 0.5).abs() < 1e-3, "the door half closed: {p}");
        assert!(at(16.46).1 > 0.99 && at(20.44).1 < 0.01);
        // One loop of the cycle over the 10 s of processing (one piece).
        let (k, p) = at(11.7);
        assert_eq!(k, clip("ciclo"));
        assert!((p - 0.525).abs() < 1e-3, "{p}");
        assert_eq!(at(30.0), (-1, 0.0));
        // The press sets up (0..5) and then processes (5..25) with no operator: the
        // automatic cycle, one per piece (the door opens, closes and opens again).
        let press = index(&pb, "prensa_1");
        assert!(pb.unmanned(press) && !pb.unmanned(cnc));
        // Operators only for the setup (`process: false`): nobody runs the process.
        let setup_only = playback("tutorial_03_operadores.ron", true);
        let maq = setup_only
            .blocks
            .iter()
            .position(|v| v.name == "maq1")
            .unwrap();
        assert!(setup_only.unmanned(maq));
        let f = pb.frame(2.5);
        assert_eq!(pb.blocks[press].clips[f.anim_clip[press] as usize], "setup");
        let f = pb.frame(10.0);
        assert_eq!(
            pb.blocks[press].clips[f.anim_clip[press] as usize],
            "ciclo_auto"
        );
        assert!((f.anim_fase[press] - 0.25).abs() < 1e-4);
    }

    #[test]
    fn parts_ride_the_points_of_the_element_working_on_them() {
        let pb = blender_model(true);
        let press = index(&pb, "prensa_1");
        let cnc = index(&pb, "cnc_1");
        let names = |i: usize| pb.anchor_points(i);
        let anchored = |t: f64, i: usize| {
            let f = pb.frame(t);
            (0..f.item_id.len())
                .find(|&k| f.item_ancora_bloco[k] == i as i32)
                .map(|k| {
                    (
                        names(i)[f.item_ancora_ponto[k] as usize].clone(),
                        f.item_position[k],
                    )
                })
        };
        // The press without an operator: the work point, then the exit along `ciclo_auto`
        // (the entry segment, the first 2/24, passes while the part still flies in from
        // the source: the hand-over keeps the scene's flight).
        let (a, _) = pb.animados[press].as_ref().unwrap();
        let place = |n: &str| a.pontos.iter().find(|p| p.0 == n).unwrap().1.unwrap().0;
        assert_eq!(
            names(press)[pb.anchor_at(press, 5.4).unwrap()],
            "ponto_entrada"
        );
        for (t, want) in [(15.0, "ponto_peca"), (24.6, "ponto_saida")] {
            let (got, at) = anchored(t, press).unwrap_or_else(|| panic!("t={t}"));
            assert_eq!(got, want, "t={t}");
            // A point of the root: already placed here, the part's base on it.
            let p = place(want);
            assert!(
                (at[0] - p[0]).abs() < 1e-4 && (at[2] - p[2]).abs() < 1e-4,
                "t={t}"
            );
            assert!((at[1] - p[1] - ITEM_SIZE / 2.0).abs() < 1e-4, "t={t}");
        }
        // The CNC: at the entry while the crew gets in (the part is still flying in from
        // the queue then), at the work point while it runs, at the exit while the crew
        // gets out.
        assert_eq!(
            names(cnc)[pb.anchor_at(cnc, 4.45).unwrap()],
            "ponto_entrada"
        );
        assert_eq!(anchored(11.7, cnc).unwrap().0, "ponto_peca");
        assert_eq!(anchored(18.45, cnc).unwrap().0, "ponto_saida");
        // Idle: no part, no anchor anywhere.
        assert!(pb.frame(60.0).item_ancora_bloco.iter().all(|&b| b == -1));
    }

    fn espera_dupla() -> Playback {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../godot/roteiros/g4_espera_dupla.ron");
        let model = Model::load(&path).expect("model");
        let replay = model.replay(0).expect("replay");
        Playback::new(&replay, Some(&model), path.parent(), true)
    }

    #[test]
    fn a_queue_on_the_waiting_table_keeps_its_items_on_the_top() {
        let pb = espera_dupla();
        let q = index(&pb, "fila");
        assert_eq!(pb.blocks[q].modelo3d.as_deref(), Some("espera"));
        let (tampo, at) = pb.tampos[q].expect("a mesa tem tampo");
        let base = layout_to_godot(at.root);
        let top = at.root[1] + tampo.altura * at.scale;
        // At 40 s the 16 items came (none in flight) and one is on the machine: the top
        // holds 12, the label counts the rest.
        let f = pb.frame(40.0);
        let mut on = 0;
        for (k, p) in f.item_position.iter().enumerate() {
            let d = [p[0] - base[0], p[1] - base[1], p[2] - base[2]];
            if d[0].abs() <= 0.7 && d[2].abs() <= 0.5 {
                assert!(d[0].abs() + ITEM_SIZE / 2.0 <= 0.55 + 1e-3, "{d:?}");
                assert!(d[2].abs() + ITEM_SIZE / 2.0 <= 0.33 + 1e-3, "{d:?}");
                let layer = ((d[1] - top - ITEM_SIZE / 2.0) / ITEM_SIZE).round();
                assert!(
                    (d[1] - (top + ITEM_SIZE / 2.0 + layer * ITEM_SIZE)).abs() < 1e-3
                        && (0.0..2.0).contains(&layer),
                    "{k}: {d:?}"
                );
                on += 1;
            }
        }
        let i = pb.blocks[q].b;
        let content = pb.scene.timeline.block_content(i, 40.0);
        assert_eq!(content, 14);
        assert_eq!(on, 12, "{content} na fila");
        assert!(
            f.label[q].contains(&format!("+{}", content - 12)),
            "{}",
            f.label[q]
        );
    }

    #[test]
    fn the_overhead_crane_moves_its_bridge_trolley_and_hook_by_the_trace() {
        let pb = playback("ponte_rolante.ron", false);
        let k = pb.blocks.iter().position(|v| v.name == "ponte").unwrap();
        assert_eq!(pb.blocks[k].modelo3d.as_deref(), Some("ponte_rolante"));
        assert!(
            !pb.blocks[k].sized,
            "stretched node by node, not as a whole"
        );
        let p = pb.pontes[k].as_ref().expect("ponte");
        let json: serde_json::Value = serde_json::from_str(&p.json).unwrap();
        assert_eq!(json["tamanho"][0].as_f64(), Some(40.0));
        assert!(json["dados"]["estica"].is_object());
        // Before any travel the hook is over the centre, up at the travel height.
        let rest = pb.ponte_pose(k, 0.0).unwrap();
        assert!(rest[0].abs() < 1e-9 && rest[1].abs() < 1e-9 && (rest[2] - 6.0).abs() < 1e-9);
        assert!(!p.hoists.is_empty(), "the trace's hoisting");
        assert!(!p.hanging.is_empty(), "the loads hanging");
        assert_eq!(json["zona"].as_f64(), Some(2.0));
        // Somewhere along the run: away from the centre, the hook down by the trace's
        // hoisting (between the floor and the travel height), and a load hanging with the
        // square under it.
        let mut moved = false;
        let mut lowered = false;
        let mut loaded = false;
        for s in 1..20000 {
            let t = s as f64;
            let h = pb.ponte_pose(k, t).unwrap();
            moved |= h[0].abs() > 1.0 || h[1].abs() > 1.0;
            assert!(h[2] > -1e-9 && h[2] < 6.0 + 1e-9, "{t}: {h:?}");
            lowered |= h[2] < 3.0;
            let f = pb.frame(t);
            assert!(f.ponte[k].iter().all(|v| v.is_finite()));
            assert!(
                f.ponte
                    .iter()
                    .enumerate()
                    .all(|(j, v)| j == k || v[0].is_nan())
            );
            assert_eq!(f.ponte_carga[k] == 1.0, p.loaded(t));
            loaded |= f.ponte_carga[k] == 1.0;
            if moved && lowered && loaded {
                break;
            }
        }
        assert!(moved && lowered && loaded);
    }

    #[test]
    fn a_real_waiting_area_draws_each_item_on_its_place() {
        let text = |area: &str| {
            format!(
                r#"Model(
                    name: "espera real",
                    run: (stop_time: 60.0, seed: 1, replications: 1),
                    objects: [
                        (name: "fonte", kind: Source(interarrival: Const(1.0), first_at_zero: true, max_arrivals: Some(5)),
                         position: (0.0, 0.0, 0.0)),
                        (name: "fila", kind: Queue(), position: (4.0, 0.0, 0.0), visual: (yaw: 90.0){area}),
                        (name: "maquina", kind: Processor(process: Const(10.0)), position: (8.0, 0.0, 0.0)),
                        (name: "saida", kind: Sink, position: (12.0, 0.0, 0.0)),
                    ],
                    connections: [("fonte", "fila"), ("fila", "maquina"), ("maquina", "saida")],
                )"#
            )
        };
        // One column, two layers: 2 under, 3 on top, 4 and 5 beside the table.
        let model = Model::parse(&text(
            ", waiting_area: Some((kind: Table, pitch: Some(0.6)))",
        ))
        .unwrap();
        let replay = model.replay(0).unwrap();
        let pb = Playback::new(&replay, Some(&model), None, false);
        let q = pb.blocks.iter().position(|v| v.name == "fila").unwrap();
        let e = pb.esperas[q].as_ref().expect("espera real");
        assert_eq!(e.places.len(), 2);
        assert!(pb.tampos[q].is_none());
        // At 8 s every item has arrived (the last one flies in until a little after 4 s).
        let f = pb.frame(8.0);
        let at = |local: [f32; 3]| e.to_layout(local);
        let near = |a: [f32; 3], b: [f32; 3]| (0..3).all(|k| (a[k] - b[k]).abs() < 1e-4);
        let drawn: Vec<[f32; 3]> = f.item_position.clone();
        // The two places (turned with the queue: 90 degrees) and the row beside it.
        for want in [
            at(e.places[0]),
            at(e.places[1]),
            at(e.beside(0).unwrap()),
            at(e.beside(1).unwrap()),
        ] {
            assert!(
                drawn.iter().any(|p| near(*p, want)),
                "{want:?} not in {drawn:?}"
            );
        }
        // The one on top is right above the one under it.
        assert!((at(e.places[1])[1] - at(e.places[0])[1] - ITEM_SIZE).abs() < 1e-5);
        // Nothing hidden yet: 4 waiting, 2 places + the row.
        assert!(!f.label[q].contains('+'), "{}", f.label[q]);
        // At 15 s the top one left: its place was taken by one from beside the table.
        let f = pb.frame(15.0);
        assert!(f.item_position.iter().any(|p| near(*p, at(e.places[1]))));
    }

    #[test]
    fn two_operators_per_piece_stand_side_by_side_at_the_post() {
        let pb = espera_dupla();
        let cnc = index(&pb, "cnc_1");
        let (carla, davi) = (index(&pb, "carla"), index(&pb, "davi"));
        let (a, _) = pb.animados[cnc].as_ref().unwrap();
        let (post, yaw) = a.post.unwrap();
        let f = pb.frame(15.0);
        for m in [carla, davi] {
            assert_eq!(f.motion[m], motion_code(Motion::Working).0);
            assert!((f.yaw[m] - yaw).abs() < 1e-5);
        }
        let (c, d) = (f.position[carla], f.position[davi]);
        // Across the post, the post in the middle.
        let gap = ((c[0] - d[0]).powi(2) + (c[2] - d[2]).powi(2)).sqrt();
        assert!((gap - POST_SPACING).abs() < 1e-3, "{c:?} {d:?}");
        let mid = [(c[0] + d[0]) / 2.0, (c[2] + d[2]) / 2.0];
        assert!((mid[0] - post[0]).abs() < 1e-3 && (mid[1] - post[2]).abs() < 1e-3);
        // One operator alone stays on the post.
        let mut one = vec![Some((
            pb.scene.mobile_at(pb.blocks[carla].b, 15.0).unwrap(),
            Some(cnc),
        ))];
        let before = one[0].unwrap().0.position;
        spread_at_posts(&mut one);
        assert_eq!(one[0].unwrap().0.position, before);
    }

    #[test]
    fn the_operator_walks_between_the_door_and_the_post_of_the_element() {
        let pb = blender_model(true);
        let cnc = index(&pb, "cnc_1");
        let ana = index(&pb, "ana");
        // The CNC at (2, 0): its post at (0.85, -1.4) and its door at (0.85, -2.0) of
        // its own frame, both looking at the machine (+y of the model).
        let (a, _) = pb.animados[cnc].as_ref().unwrap();
        let (post, yaw) = a.post.unwrap();
        let entering = a.line(utsim_core::State::Entering).unwrap();
        let ((door, _), to) = a.estados[entering].1.caminho.unwrap();
        assert_eq!(to.0, post);
        let near = |a: [f32; 3], b: [f32; 3]| (0..3).all(|k| (a[k] - b[k]).abs() < 1e-3);
        assert!(near(post, to_godot([2.85, -1.4, 0.0])), "{post:?}");
        assert!(near(door, to_godot([2.85, -2.0, 0.0])), "{door:?}");
        assert!((yaw - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
        let mid = [0.5 * (post[0] + door[0]), 0.0, 0.5 * (post[2] + door[2])];
        for (t, want) in [
            (2.46, door),
            (4.45, mid),
            (10.0, post),
            (18.45, mid),
            (20.44, door),
        ] {
            let f = pb.frame(t);
            assert_eq!(f.motion[ana], motion_code(Motion::Working).0, "t={t}");
            let got = f.position[ana];
            assert!(
                (got[0] - want[0]).abs() < 0.01 && (got[2] - want[2]).abs() < 0.01,
                "t={t}: {got:?} vs {want:?}"
            );
            assert!((f.yaw[ana] - yaw).abs() < 1e-5);
        }
        // A station without a Blender post: the operator stays where the scene puts it (the
        // access point, outside the machine), and is not sent to another station's post.
        let mut bare = blender_model(true);
        bare.animados[cnc] = None;
        assert!(bare.operator_post(cnc, 10.0).is_none());
        let f = bare.frame(10.0);
        assert!(
            (f.position[ana][0] - post[0]).hypot(f.position[ana][2] - post[2]) > 0.2,
            "{:?}",
            f.position[ana]
        );
    }

    #[test]
    fn people_get_a_blender_body_with_their_equipment_and_height() {
        let pb = blender_model(true);
        let ana = index(&pb, "ana");
        let v = &pb.blocks[ana];
        let body = v.pessoa.as_ref().expect("a Blender body");
        assert_eq!(pb.asset_of(ana).unwrap().0.name, body.arquivo);
        assert_eq!(pb.asset_of(ana).unwrap().1.unwrap().name, looks::PERSON);
        // What it wears, as the exposure analysis counts it: its own (by the catalogue's
        // names, "Óculos" → oculos) and the station's it operates.
        assert_eq!(
            body.epis,
            ["capacete", "luvas", "oculos", "protetor_auricular"]
        );
        // Falling back on the mannequin, it keeps the mannequin's own scale.
        let mannequin = crate::catalogue::by_name(looks::PERSON)
            .unwrap()
            .escala_real;
        assert!((body.escala_boneco - mannequin).abs() < 1e-5);
        assert!(
            body.cores
                .contains(&("camisa".to_string(), "#2e7d32".to_string()))
        );
        // Real size: the height over the skeleton's, and the same every run.
        let look = v.look.as_ref().unwrap();
        assert!(look.skeletal);
        assert!(
            (look.scale - body.escala).abs() < 1e-5,
            "{} {}",
            look.scale,
            body.escala
        );
        assert_eq!(blender_model(true).blocks[ana].pessoa, v.pessoa);
        // No clips of its own and no state runs: the UAL clips drive it.
        assert!(v.clips.is_empty());
        assert_eq!(pb.frame(10.0).anim_clip[ana], -1);
        // Bruno's record: a man's body, 1,95 m tall, as wide as 120 kg asks.
        let bruno = &pb.blocks[index(&pb, "bruno")];
        let body = bruno.pessoa.as_ref().expect("a Blender body");
        assert_eq!(body.arquivo, "pessoa_homem");
        assert!((body.altura - 1.95).abs() < 1e-6);
        assert!(
            body.largura > 1.15 && body.largura < 1.17,
            "{}",
            body.largura
        );
        assert!((bruno.look.as_ref().unwrap().scale - body.escala).abs() < 1e-5);
        // Without the mannequin files the person keeps the procedural operator.
        let bare = blender_model(false);
        assert!(bare.blocks[ana].pessoa.is_none());
        assert_eq!(bare.asset_of(ana).unwrap().0.source, "library");
    }

    #[test]
    fn the_forks_follow_the_lift_once_and_carry_the_pallet_on_their_point() {
        let pb = playback("centro_de_distribuicao.ron", true);
        let forklifts: Vec<usize> = (0..pb.blocks.len())
            .filter(|&i| pb.blocks[i].role == Some(MobileRole::Forklift))
            .collect();
        assert!(!forklifts.is_empty());
        let (mut high, mut carried) = (0.0f32, 0);
        let mut k = 0.0;
        while k <= pb.end() {
            let f = pb.frame(k);
            for &i in &forklifts {
                // The Blender forklift, with the library's seat and forks.
                assert_eq!(pb.blocks[i].modelo3d.as_deref(), Some("empilhadeira"));
                assert!(pb.blocks[i].driver.is_some());
                let (a, _) = pb.animados[i].as_ref().unwrap();
                let lift = f.lift[i];
                if lift > 0.1 + 1e-3 {
                    // One source for the forks: the clip, at the carriage's rise over
                    // the travel (the engine then leaves the `carro` node alone).
                    assert_eq!(a.clips[f.anim_clip[i] as usize], "garfo_subir");
                    assert!((f.anim_fase[i] - ((lift - 0.1) / 6.0).min(1.0)).abs() < 1e-4);
                    high = high.max(f.anim_fase[i]);
                } else {
                    assert_eq!(f.anim_clip[i], -1, "t={k}");
                }
                // A load on the forks goes to the fork point.
                for j in 0..f.item_id.len() {
                    if f.item_ancora_bloco[j] == i as i32 {
                        assert_eq!(
                            pb.anchor_points(i)[f.item_ancora_ponto[j] as usize],
                            "ponto_carga"
                        );
                        carried += 1;
                    }
                }
            }
            k += pb.end() / 300.0;
        }
        assert!(carried > 0);
        // A high rack bay lifts the forks well past the 0.05 that only clears the floor.
        assert!(high > 0.3, "{high}");
    }

    #[test]
    fn the_robot_and_the_agv_carry_their_parts_on_their_points() {
        // The model of the roteiro `g4_blender3d_pecas.json`.
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../godot/roteiros/g4_blender3d_pecas.ron");
        let model = Model::load(&path).expect("model");
        let replay = model.replay(0).expect("replay");
        let pb = Playback::new(&replay, Some(&model), path.parent(), true);
        let at = |name: &str, t: f64| {
            let i = index(&pb, name);
            let f = pb.frame(t);
            let points = pb.anchor_points(i);
            let on: Vec<String> = (0..f.item_id.len())
                .filter(|&k| f.item_ancora_bloco[k] == i as i32)
                .map(|k| points[f.item_ancora_ponto[k] as usize].clone())
                .collect();
            let clip =
                (f.anim_clip[i] >= 0).then(|| pb.blocks[i].clips[f.anim_clip[i] as usize].clone());
            (clip, f.anim_fase[i], on)
        };
        // The robot (no operator, no automatic line: its cycle) processes 20..26: the part
        // in the gripper in the middle third, at the exit at the end.
        let (clip, p, on) = at("robo_1", 23.0);
        assert_eq!(
            (clip.as_deref(), on.as_slice()),
            (Some("ciclo"), ["ponto_garra".to_string()].as_slice())
        );
        assert!((p - 0.5).abs() < 1e-4);
        assert_eq!(at("robo_1", 25.0).2, ["ponto_saida"]);
        // The AGV: the deck goes up while loading (1.45..4.45), stays up with the load on
        // its point while travelling, and comes down while unloading (16.45..19.45).
        let (clip, p, _) = at("agv_1", 2.95);
        assert_eq!(clip.as_deref(), Some("mesa_subir"));
        assert!((p - 0.5).abs() < 1e-3, "{p}");
        let (_, p, on) = at("agv_1", 10.0);
        assert_eq!(
            (p, on.as_slice()),
            (1.0, ["ponto_carga".to_string()].as_slice())
        );
        assert!((at("agv_1", 17.95).1 - 0.5).abs() < 1e-3);
        // The fourth pallet goes to the top cell (4.5 m, inside the 6 m of the mast): the
        // clip carries the whole rise, nothing is added to the carriage, and the pallet
        // rides on the forks at the cell's height.
        let i = index(&pb, "empilhadeira_1");
        let f = pb.frame(198.0);
        assert!(f.lift[i] >= 4.5, "{}", f.lift[i]);
        assert!((f.anim_fase[i] - (f.lift[i] - 0.1) / 6.0).abs() < 1e-4);
        assert_eq!(f.anim_extra[i], 0.0);
        let k = (0..f.item_id.len())
            .find(|&k| f.item_ancora_bloco[k] == i as i32)
            .expect("the pallet on the forks");
        let half = f.item_scale[k] * ITEM_SIZE / 2.0;
        assert!((f.item_position[k][1] - half - f.lift[i]).abs() < 1e-3);
        // The third one goes in the bay at 3 m: half of the mast's 6 m.
        let (clip, p, on) = at("empilhadeira_1", 138.0);
        assert_eq!(clip.as_deref(), Some("garfo_subir"));
        assert!((p - 0.5).abs() < 0.02, "{p}");
        assert_eq!(on, ["ponto_carga"]);
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
