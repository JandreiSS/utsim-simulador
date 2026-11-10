//! Realistic look ("Visual: realista"): which library asset (`utsim-assets`) each entity
//! is drawn with, and the pure rules that drive the articulated nodes from the scene
//! pose (forks, wheels, status light, walk cycle) and attach carried items. No engine
//! types: the Bevy viewer (`utsim-viewer/src/models.rs`) and the Godot bridge
//! (`utsim-godot/src/playback.rs`) only apply these numbers.
//!
//! # Default asset by role and kind
//! - `visual.asset` wins: a library name ([`Spec::by_name`]), a name of the third-party
//!   catalogue ([`crate::catalogue`], placed at real size facing `+x` with the
//!   catalogue's `escala_real` and `giro_y_graus`) or a `.glb` file, relative to the
//!   model file. Parametric library assets (rack, conveyor, floor area) still take their
//!   size from the object's geometry.
//! - Transporters by [`MobileRole`]: `Forklift` → `empilhadeira`, `Agv` → `agv`,
//!   `Person` → the skeletal mannequin of the catalogue ([`PERSON`], animated by
//!   [`person_clip`]) when its files are available ([`Context::skeletal_people`]), else
//!   `operador_simples`; `Vehicle` → `paleteira` in a scene with pallet loads (it has
//!   racks or forklifts), else `agv`.
//! - Objects by kind: `Rack` → `porta_paletes` sized by its `RackParams`, `Conveyor` →
//!   `esteira` of its length, `Processor` → `maquina`, `Combiner` → `bancada`, `Queue` →
//!   a painted floor `area`. Other kinds keep their simple shape.
//! - Items: a pallet with eight boxes in a scene with pallet loads, else one box.

use std::f32::consts::PI;
use std::path::{Path, PathBuf};

use utsim_assets::{AreaSpec, ConveyorSpec, RackSpec, Spec, dims};
use utsim_model::VisualDef;
use utsim_scene::compose::turned;
use utsim_scene::geometry::Geom;
use utsim_scene::handling::{BEAM, BELT_THICKNESS, BELT_WIDTH, RACK_DEPTH};
use utsim_scene::style::{self, Shape};
use utsim_scene::{MobileRole, Motion};

use crate::catalogue;

/// Where an asset comes from.
#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    Library(Spec),
    /// An entry of the third-party catalogue: its name and its file on disk.
    Catalogue {
        name: String,
        file: PathBuf,
    },
    /// A `.glb` file (absolute, or relative to the working directory).
    File(PathBuf),
}

/// How items ride on a mobile resource drawn with this asset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Carry {
    /// On forks centred `forward` metres ahead of the origin; the load bottom is at the
    /// pose's `lift` (forklift) or at a fixed height (pallet jack).
    Forks { forward: f32, height: Option<f32> },
    /// Held in front of the body at the hands (`lift` is the hand height).
    Hands,
    /// On a flat deck at this height.
    Deck(f32),
    /// Unchanged (the simple shapes' stacking).
    Stack,
}

impl Carry {
    /// Short name for interfaces that receive plain data (`forks`, `hands`, `deck`,
    /// `stack`).
    pub fn name(self) -> &'static str {
        match self {
            Carry::Forks { .. } => "forks",
            Carry::Hands => "hands",
            Carry::Deck(_) => "deck",
            Carry::Stack => "stack",
        }
    }
}

/// How one block is drawn in the realistic mode.
#[derive(Clone, Debug, PartialEq)]
pub struct Look {
    pub source: Source,
    /// Uniform scale (`visual.scale`).
    pub scale: f32,
    /// Extra rotation about the vertical, radians (`visual.yaw`; a belt without it: its
    /// heading).
    pub yaw: f32,
    /// Colour of the `pintura` material (`visual.color`), sRGB.
    pub color: Option<[f32; 3]>,
    pub carry: Carry,
    /// Translation of the glTF scene under the block's root (layout space): zero for
    /// the library and user files, the catalogue centring for a third-party asset.
    pub offset: [f32; 3],
    /// The skeletal mannequin of the catalogue ([`is_skeletal`]): driven by animation
    /// clips instead of named nodes.
    pub skeletal: bool,
}

/// Scene-wide choices.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Context {
    /// The scene moves pallet loads (it has racks or forklifts).
    pub pallets: bool,
    /// People get the skeletal mannequin by default (its `.glb` files can be read: the
    /// desktop build, with `assets/terceiros` present and loading).
    pub skeletal_people: bool,
}

/// Catalogue entry of the default skeletal person.
pub const PERSON: &str = "pessoa_operador";
/// Catalogue entry whose file has the extra clips (carrying) for the same skeleton.
pub const PERSON_CLIPS: &str = "pessoa_operador_carga";

/// Whether the catalogue entry `name` is the skeletal mannequin (either file of the
/// Universal Animation Library: same skeleton, same bone names).
pub fn is_skeletal(name: &str) -> bool {
    name.eq_ignore_ascii_case(PERSON) || name.eq_ignore_ascii_case(PERSON_CLIPS)
}

/// `"#rrggbb"` (or `"rrggbb"`) as sRGB components.
pub fn parse_color(text: &str) -> Option<[f32; 3]> {
    let hex = text.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let v = u32::from_str_radix(hex, 16).ok()?;
    Some(utsim_assets::builder::hex(v))
}

/// Library asset of a block by its kind, role and geometry (no `visual`).
fn default_spec(kind: &str, role: Option<MobileRole>, geom: &Geom, ctx: Context) -> Option<Spec> {
    if let Some(role) = role {
        return Some(match role {
            MobileRole::Forklift => Spec::Empilhadeira,
            MobileRole::Agv => Spec::Agv,
            MobileRole::Person => Spec::OperadorSimples,
            MobileRole::Vehicle if ctx.pallets => Spec::Paleteira,
            MobileRole::Vehicle => Spec::Agv,
        });
    }
    match kind {
        "Rack" | "Conveyor" | "Queue" => sized(kind_spec(kind)?, geom),
        "Processor" => Some(Spec::Maquina),
        "Combiner" => Some(Spec::Bancada),
        _ => None,
    }
}

fn kind_spec(kind: &str) -> Option<Spec> {
    match kind {
        "Rack" => Some(Spec::PortaPaletes(RackSpec::default())),
        "Conveyor" => Some(Spec::Esteira(ConveyorSpec::default())),
        "Queue" => Some(Spec::Area(AreaSpec::default())),
        _ => None,
    }
}

/// A parametric library asset sized by the block's geometry (the scene's own numbers,
/// so items land exactly on the beams and rollers).
fn sized(spec: Spec, geom: &Geom) -> Option<Spec> {
    Some(match spec {
        Spec::PortaPaletes(_) => {
            let p = geom.rack_params()?;
            Spec::PortaPaletes(RackSpec {
                bays: p.bays,
                levels: p.levels,
                bay_width: p.bay_width as f32,
                level_height: p.level_height as f32,
                depth: RACK_DEPTH,
                beam: BEAM,
            })
        }
        Spec::Esteira(_) => {
            let p = geom.belt_params()?;
            Spec::Esteira(ConveyorSpec {
                length: p.length as f32,
                width: BELT_WIDTH,
                height: BELT_THICKNESS,
                pitch: 0.125,
            })
        }
        Spec::Area(_) => {
            let (half, _) = style::extents(Shape::LongBox);
            Spec::Area(AreaSpec {
                length: 2.0 * half[0],
                width: 2.0 * half[2],
            })
        }
        other => other,
    })
}

/// How items ride on an asset of this family.
fn carry_of(spec: Option<&Spec>, role: Option<MobileRole>) -> Carry {
    match (spec, role) {
        (Some(Spec::Empilhadeira), _) | (None, Some(MobileRole::Forklift)) => Carry::Forks {
            forward: dims::FORK_CENTER,
            height: None,
        },
        (Some(Spec::Paleteira), _) => Carry::Forks {
            forward: dims::JACK_FORK_CENTER,
            height: Some(dims::JACK_FORK_TOP),
        },
        (Some(Spec::OperadorSimples), _) | (None, Some(MobileRole::Person)) => Carry::Hands,
        (Some(Spec::Agv), _) => Carry::Deck(dims::AGV_DECK),
        _ => Carry::Stack,
    }
}

/// The realistic look of a block, or `None` to keep its simple shape. `model_dir` is
/// the folder of the model file (for `visual.asset` paths).
pub fn look_of(
    kind: &str,
    role: Option<MobileRole>,
    visual: Option<&VisualDef>,
    geom: &Geom,
    ctx: Context,
    model_dir: Option<&Path>,
) -> Option<Look> {
    let asset = visual.and_then(|v| v.asset.as_deref()).map(str::trim);
    let user_scale = visual
        .and_then(|v| v.scale)
        .filter(|s| s.is_finite() && *s > 0.0)
        .map_or(1.0, |s| s as f32);
    let user_yaw = visual
        .and_then(|v| v.yaw)
        .filter(|y| y.is_finite())
        .map_or(0.0, |y| (y as f32).to_radians());
    let color = visual
        .and_then(|v| v.color.as_deref())
        .and_then(parse_color);
    // A third-party asset of the catalogue: real size, facing +x, centred. A person
    // whose `visual.asset` names no model (none, or only the role: "pessoa", "operador",
    // "person") gets the skeletal mannequin when it is available; `operador_simples`
    // keeps the procedural one.
    let library_or_file = asset.is_some_and(|a| {
        let n = a.trim().to_lowercase();
        !matches!(n.as_str(), "operador" | "pessoa" | "person")
            && (Spec::by_name(&n).is_some() || n.ends_with(".glb"))
    });
    let catalogue = asset
        .filter(|a| Spec::by_name(a).is_none())
        .and_then(catalogue::by_name);
    let default_person =
        (role == Some(MobileRole::Person) && ctx.skeletal_people && !library_or_file)
            .then(|| catalogue::by_name(PERSON))
            .flatten();
    if let Some(entry) = catalogue.or(default_person) {
        let p = entry.placement(user_scale, user_yaw);
        return Some(Look {
            source: Source::Catalogue {
                name: entry.nome.clone(),
                file: catalogue::dir_from(model_dir).join(&entry.arquivo),
            },
            scale: p.scale,
            yaw: p.yaw,
            color,
            carry: carry_of(None, role),
            offset: p.offset,
            skeletal: is_skeletal(&entry.nome),
        });
    }
    let source = match asset {
        Some(a) if a.to_lowercase().ends_with(".glb") => {
            let path = Path::new(a);
            Source::File(match model_dir {
                Some(dir) if path.is_relative() => dir.join(path),
                _ => path.to_path_buf(),
            })
        }
        Some(a) => match Spec::by_name(a) {
            Some(spec) => Source::Library(sized(spec, geom).unwrap_or(spec)),
            None => Source::Library(default_spec(kind, role, geom, ctx)?),
        },
        None => Source::Library(default_spec(kind, role, geom, ctx)?),
    };
    let spec = match &source {
        Source::Library(s) => Some(*s),
        _ => None,
    };
    // A belt points along its connection, or where `visual.yaw` turns it (then its
    // heading is that, as the navigation and the scene see it, not the sum).
    let has_yaw = visual.and_then(|v| v.yaw).is_some_and(f64::is_finite);
    let belt = geom.shape == Shape::Belt;
    let yaw = match (belt, has_yaw) {
        (true, false) => geom.yaw,
        _ => user_yaw,
    };
    Some(Look {
        carry: carry_of(spec.as_ref(), role),
        source,
        scale: user_scale,
        yaw,
        color,
        offset: [0.0; 3],
        skeletal: false,
    })
}

/// Relative URIs a `.glb` names for its images and buffers (`Textures/colormap.png`),
/// so they can be served next to it. Embedded data (`data:`), absolute URLs and
/// malformed files give none.
pub fn relative_uris(glb: &[u8]) -> Vec<String> {
    let word = |at: usize| -> Option<usize> {
        let b = glb.get(at..at + 4)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
    };
    if glb.get(0..4) != Some(b"glTF".as_slice()) || word(16) != Some(0x4E4F_534A) {
        return Vec::new();
    }
    let Some(json) = word(12).and_then(|n| glb.get(20..20 + n)) else {
        return Vec::new();
    };
    let Ok(doc) = serde_json::from_slice::<serde_json::Value>(json) else {
        return Vec::new();
    };
    let mut uris = Vec::new();
    for list in ["images", "buffers"] {
        for item in doc[list].as_array().into_iter().flatten() {
            if let Some(uri) = item["uri"].as_str()
                && !uri.starts_with("data:")
                && !uri.contains("://")
                && !uri.starts_with('/')
                && !uri.split(['/', '\\']).any(|part| part == "..")
                && !uris.iter().any(|u| u == uri)
            {
                uris.push(uri.to_string());
            }
        }
    }
    uris
}

/// Length of one step of a walking person, metres (a full cycle is two steps).
pub const STEP: f32 = 0.7;

/// Rotations of the operator's limbs about the lateral axis, radians (positive swings
/// forward).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Limbs {
    pub leg_left: f32,
    pub leg_right: f32,
    pub arm_left: f32,
    pub arm_right: f32,
    /// Forward lean of the torso.
    pub torso: f32,
}

/// Leg angle of a walk cycle with no foot sliding: during the stance half of the cycle
/// the foot stays where it was put down while the hip advances, so `sin(angle)` falls
/// linearly from `+STEP/2` to `-STEP/2` over the leg length; the swing half mirrors it.
fn leg_angle(walked: f32, offset: f32) -> f32 {
    let u = (walked / STEP + offset).rem_euclid(2.0);
    let s = if u < 1.0 {
        1.0 - 2.0 * u
    } else {
        2.0 * u - 3.0
    };
    (s * STEP / 2.0 / dims::LEG).clamp(-1.0, 1.0).asin()
}

/// Arm angle that puts the hands at `height` in front of the body.
fn reach(height: f32) -> f32 {
    let drop = (dims::SHOULDER - height).clamp(0.0, dims::ARM);
    (drop / dims::ARM).acos()
}

/// The operator's pose: legs and arms swing by the distance walked (one step per
/// [`STEP`] metres), arms reach forward to the hands' height while carrying (`lift` >
/// 0), everything still when idle, a small working motion at a station. `clock` is a
/// free-running time in seconds (for the working motion).
pub fn person_pose(motion: Motion, walked: f32, lift: f32, clock: f32) -> Limbs {
    let carrying = lift > 0.0;
    let hold = if carrying { reach(lift) } else { 0.0 };
    match motion {
        Motion::Moving { .. } => {
            let (l, r) = (leg_angle(walked, 0.0), leg_angle(walked, 1.0));
            let (al, ar) = if carrying {
                (hold, hold)
            } else {
                (-0.7 * l, -0.7 * r)
            };
            Limbs {
                leg_left: l,
                leg_right: r,
                arm_left: al,
                arm_right: ar,
                torso: 0.04,
            }
        }
        Motion::Working => {
            let w = (clock * 2.0 * PI * 0.9).sin();
            Limbs {
                arm_left: 0.75 + 0.18 * w,
                arm_right: 0.75 - 0.18 * w,
                torso: 0.12,
                ..Limbs::default()
            }
        }
        Motion::Loading { progress } | Motion::Unloading { progress } => {
            // Bend and reach while the item changes hands.
            let k = (progress * PI).sin();
            Limbs {
                arm_left: hold.max(0.9 * k),
                arm_right: hold.max(0.9 * k),
                torso: 0.25 * k,
                ..Limbs::default()
            }
        }
        _ => Limbs {
            arm_left: hold,
            arm_right: hold,
            ..Limbs::default()
        },
    }
}

/// Animation clips of the skeletal person, from the two files of the Universal
/// Animation Library ([`PERSON`] and [`PERSON_CLIPS`]). `IdleCarry` has no clip of its
/// own: the legs of `Idle_Loop` under the arms of `Walk_Carry_Loop` (two masked nodes).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Clip {
    Idle,
    Walk,
    Jog,
    Carry,
    Interact,
    PickUp,
    IdleCarry,
}

/// Speed above which a person jogs instead of walking, m/s.
pub const JOG_SPEED: f32 = 2.0;
/// Length of the crossfade between two clips, seconds of real time.
pub const FADE: f32 = 0.25;

impl Clip {
    pub const ALL: [Clip; 7] = [
        Clip::Idle,
        Clip::Walk,
        Clip::Jog,
        Clip::Carry,
        Clip::Interact,
        Clip::PickUp,
        Clip::IdleCarry,
    ];

    pub fn index(self) -> usize {
        Clip::ALL.iter().position(|c| *c == self).unwrap_or(0)
    }

    /// Name of the glTF animation, and whether it is in the [`PERSON_CLIPS`] file. For
    /// `IdleCarry`, the upper-body layer (the legs come from [`Clip::Idle`]).
    pub fn source(self) -> (&'static str, bool) {
        match self {
            Clip::Idle => ("Idle_Loop", false),
            Clip::Walk => ("Walk_Loop", false),
            Clip::Jog => ("Jog_Fwd_Loop", false),
            Clip::Carry | Clip::IdleCarry => ("Walk_Carry_Loop", true),
            Clip::Interact => ("Interact", false),
            Clip::PickUp => ("PickUp_Table", false),
        }
    }

    /// Ground covered by one loop of a walking clip, in file units (multiply by the
    /// asset scale), so that the planted foot does not slide. Measured from the clips:
    /// the speed of the stance foot (`ball_l`, lowest 2 cm) relative to the root, times
    /// the loop length. `Walk_Loop` 0.98 m/s over 1.333 s, `Walk_Carry_Loop` 0.65 m/s
    /// over 2 s, `Jog_Fwd_Loop` 5.8 m/s over 0.933 s (a short, fast stance).
    pub fn stride(self) -> Option<f32> {
        match self {
            Clip::Walk => Some(1.306),
            Clip::Carry => Some(1.304),
            Clip::Jog => Some(5.44),
            _ => None,
        }
    }
}

/// Which clip a person plays: walking (jogging above [`JOG_SPEED`]) or walking with the
/// load when moving, the pick-up while loading or unloading, the work at the station
/// panel while working, standing (holding the load) otherwise.
pub fn person_clip(motion: Motion, carrying: bool) -> Clip {
    match motion {
        Motion::Moving { .. } if carrying => Clip::Carry,
        Motion::Moving { speed } if speed > JOG_SPEED => Clip::Jog,
        Motion::Moving { .. } => Clip::Walk,
        Motion::Loading { .. } | Motion::Unloading { .. } => Clip::PickUp,
        Motion::Working => Clip::Interact,
        _ if carrying => Clip::IdleCarry,
        _ => Clip::Idle,
    }
}

/// Time inside `clip` (of length `duration`, seconds) for a person who has walked
/// `distance` metres (at asset `scale`) at simulated time `t`, `progress` into a load
/// window. Walking clips follow the distance (one loop per stride, so the clip rate
/// follows the speed and the feet do not slide), the pick-up follows the window, the
/// others loop on the simulated clock. Nothing depends on how the clock got to `t`.
pub fn clip_time(
    clip: Clip,
    duration: f32,
    distance: f32,
    scale: f32,
    t: f64,
    progress: f32,
) -> f32 {
    if duration <= 0.0 {
        return 0.0;
    }
    let time = match (clip, clip.stride()) {
        (_, Some(stride)) => {
            let loops = distance as f64 / (stride * scale.max(1e-3)) as f64;
            loops.rem_euclid(1.0) as f32 * duration
        }
        (Clip::PickUp, _) => progress.clamp(0.0, 1.0) * duration,
        _ => t.rem_euclid(duration as f64) as f32,
    };
    time.clamp(0.0, duration * 0.9999)
}

/// Crossfade: the weight of `target` rises and every other one falls by `dt / FADE`.
pub fn fade(weights: &mut [f32; 7], target: Clip, dt: f32) {
    let step = (dt / FADE).max(0.0);
    for (k, w) in weights.iter_mut().enumerate() {
        *w = if k == target.index() {
            (*w + step).min(1.0)
        } else {
            (*w - step).max(0.0)
        };
    }
}

/// Status light colour (sRGB): AGVs by their motion (green moving, orange blocked, red
/// stopped, blue loading or unloading, white otherwise); everything else by the block
/// state colour.
pub fn light_color(agv: bool, motion: Motion, state: Option<utsim_core::State>) -> [f32; 3] {
    if !agv {
        return style::state_color(state);
    }
    match motion {
        Motion::Moving { .. } => [0.15, 0.90, 0.30],
        Motion::Blocked => [1.0, 0.55, 0.0],
        Motion::Stopped => [0.95, 0.12, 0.08],
        Motion::Loading { .. } | Motion::Unloading { .. } => [0.15, 0.45, 1.0],
        Motion::Idle | Motion::Working => [0.85, 0.92, 1.0],
    }
}

/// Rotation of a wheel of `radius` after rolling `distance` forward (radians about the
/// lateral axis, negative forward).
pub fn wheel_angle(distance: f32, radius: f32) -> f32 {
    if radius <= 1e-3 {
        0.0
    } else {
        -(distance / radius).rem_euclid(2.0 * PI)
    }
}

/// Centre of the `index`-th item (`size` edge) carried by a mobile at `base` (layout
/// space, Y up) heading `yaw`, or `None` for [`Carry::Stack`].
pub fn carried_center(
    carry: Carry,
    base: [f32; 3],
    yaw: f32,
    lift: f32,
    index: usize,
    size: f32,
) -> Option<[f32; 3]> {
    let ahead = |d: f32| {
        // Layout space: yaw turns +X towards -Z (counter-clockwise seen from above).
        [base[0] + d * yaw.cos(), base[2] - d * yaw.sin()]
    };
    let k = index as f32;
    let (xz, y) = match carry {
        Carry::Forks { forward, height } => {
            let bottom = height.unwrap_or(lift);
            (ahead(forward), base[1] + bottom + size / 2.0 + k * size)
        }
        Carry::Hands => {
            let hands = if lift > 0.0 {
                lift
            } else {
                utsim_scene::compose::HAND_HEIGHT
            };
            (ahead(0.25 + size / 2.0), base[1] + hands + k * size)
        }
        Carry::Deck(h) => (ahead(0.0), base[1] + h + size / 2.0 + k * size),
        Carry::Stack => return None,
    };
    Some([xz[0], y, xz[1]])
}

/// Where an operator works at a station whose footprint has half extents `half` (in its
/// own frame) and that is turned by `turn` about the vertical: at the front (the local
/// `+Z` side, where the machine's door and panel are), facing it.
pub fn station_front(base: [f32; 3], half: [f32; 3], turn: f32) -> ([f32; 3], f32) {
    let f = turned([0.0, 0.0, half[2] + 0.3], turn);
    ([base[0] + f[0], base[1], base[2] + f[2]], PI / 2.0 + turn)
}

/// Keeps a mobile of radius `radius` out of a station (a machine drawn as a solid
/// body): when its layout `position` falls inside the station's footprint (`center`,
/// half extents `half` in its own frame, turned by `turn`, grown by `radius`), it is
/// moved back against its heading `yaw` to the edge. The trace parks vehicles at the
/// station's network node, its centre.
pub fn keep_outside(
    position: [f32; 3],
    yaw: f32,
    center: [f32; 3],
    half: [f32; 3],
    turn: f32,
    radius: f32,
) -> [f32; 3] {
    let (hx, hz) = (half[0] + radius, half[2] + radius);
    // In the station's own frame.
    let d = turned(
        [position[0] - center[0], 0.0, position[2] - center[2]],
        -turn,
    );
    let (dx, dz) = (d[0], d[2]);
    if dx.abs() >= hx || dz.abs() >= hz {
        return position;
    }
    // Backwards along the heading: -(cos, -sin) in layout space.
    let back = [-yaw.cos(), 0.0, yaw.sin()];
    let b = turned(back, -turn);
    let (bx, bz) = (b[0], b[2]);
    let exit = |d: f32, b: f32, h: f32| {
        if b.abs() < 1e-6 {
            f32::INFINITY
        } else {
            ((h * b.signum()) - d) / b
        }
    };
    let t = exit(dx, bx, hx).min(exit(dz, bz, hz));
    [
        position[0] + back[0] * t,
        position[1],
        position[2] + back[2] * t,
    ]
}

/// A queue drawn with the conveyor (`visual.asset: "esteira"`): its resting items ride
/// on top of the rollers, in one line along the belt, the oldest at the `+x` end; a full
/// line starts a second layer on top, and what does not fit in [`style::MAX_LAYERS`]
/// layers is only counted (the label shows "+N").
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QueueBelt {
    /// Length and outside width of the conveyor, and the height of the rollers' top.
    pub length: f32,
    pub width: f32,
    pub top: f32,
    /// Heading of the conveyor (the look's yaw).
    pub yaw: f32,
}

/// Gap between two items in the line, as a fraction of the item size.
const QUEUE_GAP: f32 = 0.1;

/// The belt of a queue whose look is the conveyor, or `None` (other kinds or looks).
pub fn queue_belt(kind: &str, look: &Look) -> Option<QueueBelt> {
    match (&look.source, kind) {
        (Source::Library(Spec::Esteira(c)), "Queue") => Some(QueueBelt {
            length: c.length.max(0.2) * look.scale,
            width: c.width.max(0.2) * look.scale,
            top: c.height.max(0.05) * look.scale,
            yaw: look.yaw,
        }),
        _ => None,
    }
}

impl QueueBelt {
    /// Items of `size` in one line along the belt.
    pub fn per_layer(&self, size: f32) -> usize {
        ((self.length / (size * (1.0 + QUEUE_GAP))).floor() as usize).max(1)
    }

    /// Centre of resting item `i` (0: the oldest) relative to the block base, or `None`
    /// when it is past the layers drawn.
    pub fn slot(&self, i: usize, size: f32) -> Option<[f32; 3]> {
        let n = self.per_layer(size);
        let layer = i / n;
        if layer >= style::MAX_LAYERS {
            return None;
        }
        let pitch = self.length / n as f32;
        let along = self.length / 2.0 - pitch * ((i % n) as f32 + 0.5);
        let (s, c) = self.yaw.sin_cos();
        Some([
            along * c,
            self.rest_height(size) + layer as f32 * size,
            -along * s,
        ])
    }

    /// Height of the centre of an item of `size` lying on the rollers.
    pub fn rest_height(&self, size: f32) -> f32 {
        self.top + 0.01 + size / 2.0
    }

    /// Whether an item of `size` centred at `d` (relative to the block base) is over the
    /// belt (it would sink into it below [`Self::rest_height`]).
    pub fn covers(&self, d: [f32; 3], size: f32) -> bool {
        let (s, c) = self.yaw.sin_cos();
        let along = d[0] * c - d[2] * s;
        let across = d[0] * s + d[2] * c;
        along.abs() < self.length / 2.0 + size / 2.0 && across.abs() < self.width / 2.0 + size / 2.0
    }

    /// Resting items not drawn.
    pub fn hidden(&self, resting: usize, size: f32) -> usize {
        resting.saturating_sub(self.per_layer(size) * style::MAX_LAYERS)
    }
}

/// Height the simple shapes lift a queue's items to (the realistic queue is a floor
/// area: its items rest on the floor instead).
pub fn queue_drop() -> f32 {
    style::extents(Shape::LongBox).1 - 0.01
}

#[cfg(test)]
mod tests {
    use super::*;
    use utsim_scene::handling::{Handling, RackParams};

    fn geom(kind: &str) -> Geom {
        Geom::new(kind, None, None)
    }

    #[test]
    fn a_queue_drawn_as_a_conveyor_lines_its_items_on_the_belt() {
        let ctx = Context::default();
        let visual = VisualDef {
            asset: Some("esteira".into()),
            ..VisualDef::default()
        };
        let look = look_of("Queue", None, Some(&visual), &geom("Queue"), ctx, None).unwrap();
        let belt = queue_belt("Queue", &look).expect("a queue with the conveyor look");
        let Source::Library(Spec::Esteira(c)) = look.source else {
            panic!("the conveyor asset");
        };
        // The numbers come from the conveyor drawn.
        assert_eq!(
            (belt.length, belt.width, belt.top),
            (c.length, c.width, c.height)
        );
        let size = utsim_scene::compose::ITEM_SIZE;
        let n = belt.per_layer(size);
        assert!(n >= 5, "{n}");
        for i in 0..n * style::MAX_LAYERS {
            let p = belt.slot(i, size).unwrap();
            // Inside the footprint, along the length and within the side frames.
            assert!(
                p[0].abs() + size / 2.0 <= belt.length / 2.0 + 1e-5,
                "slot {i}: {p:?}"
            );
            assert!(
                p[2].abs() + size / 2.0 <= belt.width / 2.0 + 1e-5,
                "slot {i}: {p:?}"
            );
            // Resting on the rollers (first layer) or on the item below.
            let layer = i / n;
            let bottom = p[1] - size / 2.0;
            assert!(bottom >= belt.top - 1e-5, "slot {i} under the belt top");
            assert!(
                (bottom - belt.top - layer as f32 * size).abs() < 0.02,
                "slot {i}"
            );
        }
        // One line: the first layer is all at the same height and side.
        let a = belt.slot(0, size).unwrap();
        let b = belt.slot(1, size).unwrap();
        assert_eq!((a[1], a[2]), (b[1], b[2]));
        assert!(a[0] > b[0], "the oldest at the +x end");
        // Past the layers drawn: counted, not drawn.
        assert_eq!(belt.slot(n * style::MAX_LAYERS, size), None);
        assert_eq!(belt.hidden(n * style::MAX_LAYERS + 7, size), 7);
        // Other queues keep the floor area.
        let plain = look_of("Queue", None, None, &geom("Queue"), ctx, None).unwrap();
        assert_eq!(queue_belt("Queue", &plain), None);
    }

    #[test]
    fn roles_and_kinds_pick_their_assets() {
        let ctx = Context::default();
        let pick = |kind: &str, role| look_of(kind, role, None, &geom(kind), ctx, None);
        let spec = |l: Option<Look>| match l.map(|l| l.source) {
            Some(Source::Library(s)) => Some(s.name()),
            _ => None,
        };
        assert_eq!(
            spec(pick("Transporter", Some(MobileRole::Forklift))),
            Some("empilhadeira")
        );
        assert_eq!(
            spec(pick("Transporter", Some(MobileRole::Agv))),
            Some("agv")
        );
        assert_eq!(
            spec(pick("Transporter", Some(MobileRole::Person))),
            Some("operador_simples")
        );
        assert_eq!(
            spec(pick("Transporter", Some(MobileRole::Vehicle))),
            Some("agv")
        );
        let pallets = Context {
            pallets: true,
            ..Context::default()
        };
        let jack = look_of(
            "Transporter",
            Some(MobileRole::Vehicle),
            None,
            &geom("Transporter"),
            pallets,
            None,
        );
        assert_eq!(spec(jack), Some("paleteira"));
        assert_eq!(spec(pick("Processor", None)), Some("maquina"));
        assert_eq!(spec(pick("Queue", None)), Some("area"));
        assert_eq!(spec(pick("Conveyor", None)), Some("esteira"));
        assert_eq!(spec(pick("Source", None)), None);
    }

    #[test]
    fn a_rack_takes_its_params_and_visual_overrides_apply() {
        let p = RackParams {
            bays: 6,
            levels: 4,
            bay_width: 1.2,
            level_height: 1.5,
        };
        let g = Geom::new("Rack", Some(Handling::Rack(p)), None);
        let visual = VisualDef {
            asset: None,
            scale: Some(2.0),
            yaw: Some(90.0),
            color: Some("#ff0000".into()),
        };
        let l = look_of("Rack", None, Some(&visual), &g, Context::default(), None).unwrap();
        let Source::Library(Spec::PortaPaletes(r)) = l.source else {
            panic!("{:?}", l.source)
        };
        assert_eq!(
            (r.bays, r.levels, r.bay_width, r.level_height),
            (6, 4, 1.2, 1.5)
        );
        assert_eq!(l.scale, 2.0);
        assert!((l.yaw - PI / 2.0).abs() < 1e-6);
        assert_eq!(l.color, Some([1.0, 0.0, 0.0]));
        // A file path is relative to the model.
        let visual = VisualDef {
            asset: Some("pecas/torno.glb".into()),
            ..VisualDef::default()
        };
        let l = look_of(
            "Processor",
            None,
            Some(&visual),
            &geom("Processor"),
            Context::default(),
            Some(Path::new("m")),
        )
        .unwrap();
        assert_eq!(
            l.source,
            Source::File(Path::new("m").join("pecas/torno.glb"))
        );
        // A library name on another kind.
        let visual = VisualDef {
            asset: Some("empilhadeira".into()),
            ..VisualDef::default()
        };
        let l = look_of(
            "Transporter",
            Some(MobileRole::Vehicle),
            Some(&visual),
            &geom("Transporter"),
            Context::default(),
            None,
        )
        .unwrap();
        assert_eq!(l.source, Source::Library(Spec::Empilhadeira));
        assert!(matches!(l.carry, Carry::Forks { height: None, .. }));
    }

    #[test]
    fn a_third_party_asset_is_placed_by_the_catalogue() {
        let visual = VisualDef {
            asset: Some("bancada_kenney".into()),
            scale: Some(1.5),
            yaw: Some(90.0),
            ..VisualDef::default()
        };
        let l = look_of(
            "Processor",
            None,
            Some(&visual),
            &geom("Processor"),
            Context::default(),
            None,
        )
        .unwrap();
        let entry = catalogue::by_name("bancada_kenney").unwrap();
        assert_eq!(
            l.source,
            Source::Catalogue {
                name: entry.nome.clone(),
                file: catalogue::file_of(entry),
            }
        );
        // escala_real 4 times the user's 1.5; giro_y_graus 90 plus the user's 90.
        assert!((l.scale - 6.0).abs() < 1e-6);
        assert!((l.yaw - PI).abs() < 1e-5);
        assert_eq!(l.offset, entry.placement(1.5, PI / 2.0).offset);
        // A person of the catalogue carries in the hands like the library operator.
        let visual = VisualDef {
            asset: Some("pessoa_mini_homem_a".into()),
            ..VisualDef::default()
        };
        let role = Some(MobileRole::Person);
        let l = look_of(
            "Transporter",
            role,
            Some(&visual),
            &geom("Transporter"),
            Context::default(),
            None,
        )
        .unwrap();
        assert_eq!(l.carry, Carry::Hands);
        // Library names win over the catalogue, and unknown names keep the default.
        let visual = VisualDef {
            asset: Some("nada_disso".into()),
            ..VisualDef::default()
        };
        let l = look_of(
            "Processor",
            None,
            Some(&visual),
            &geom("Processor"),
            Context::default(),
            None,
        )
        .unwrap();
        assert_eq!(l.source, Source::Library(Spec::Maquina));
        assert_eq!(l.offset, [0.0; 3]);
    }

    #[test]
    fn the_stance_foot_does_not_slide() {
        // Over a stance half-cycle the planted foot (hip + L sin(angle)) stays put.
        for k in 0..10 {
            let d = k as f32 * STEP / 10.0 * 0.999;
            let foot = d + dims::LEG * leg_angle(d, 0.0).sin();
            assert!((foot - STEP / 2.0).abs() < 1e-4, "{d}: {foot}");
        }
        // The legs alternate.
        let p = person_pose(Motion::Moving { speed: 1.2 }, 0.2, 0.0, 0.0);
        assert!(p.leg_left > 0.0 && p.leg_right < 0.0);
        assert!(p.arm_left < 0.0 && p.arm_right > 0.0);
        assert!((p.leg_left + p.leg_right).abs() < 1e-5);
    }

    #[test]
    fn arms_reach_the_hand_height_when_carrying_and_rest_when_idle() {
        let p = person_pose(Motion::Moving { speed: 1.0 }, 0.3, 1.0, 0.0);
        let hand = dims::SHOULDER - dims::ARM * p.arm_left.cos();
        assert!((hand - 1.0).abs() < 1e-4, "{hand}");
        assert_eq!(person_pose(Motion::Idle, 3.0, 0.0, 1.0), Limbs::default());
        let w = person_pose(Motion::Working, 0.0, 0.0, 0.3);
        assert!(w.arm_left > 0.4 && w.leg_left == 0.0);
    }

    #[test]
    fn agv_light_follows_the_motion() {
        assert_eq!(
            light_color(true, Motion::Moving { speed: 1.0 }, None),
            [0.15, 0.90, 0.30]
        );
        assert_eq!(light_color(true, Motion::Blocked, None), [1.0, 0.55, 0.0]);
        assert_eq!(light_color(true, Motion::Stopped, None)[0], 0.95);
        assert_eq!(
            light_color(true, Motion::Loading { progress: 0.5 }, None)[2],
            1.0
        );
        assert_eq!(
            light_color(false, Motion::Idle, None),
            style::state_color(None)
        );
    }

    #[test]
    fn loads_ride_on_the_forks_at_lift_height() {
        let carry = Carry::Forks {
            forward: dims::FORK_CENTER,
            height: None,
        };
        // Facing +x (yaw 0): ahead along +X; facing model +y (yaw pi/2): along Bevy -Z.
        let c = carried_center(carry, [10.0, 0.0, 0.0], 0.0, 2.5, 0, 0.6).unwrap();
        assert!((c[0] - 10.0 - dims::FORK_CENTER).abs() < 1e-5);
        assert!((c[1] - 2.8).abs() < 1e-5);
        let c = carried_center(carry, [0.0, 0.0, 0.0], PI / 2.0, 0.1, 0, 0.6).unwrap();
        assert!(c[0].abs() < 1e-5 && (c[2] + dims::FORK_CENTER).abs() < 1e-5);
        assert!(carried_center(Carry::Stack, [0.0; 3], 0.0, 0.0, 0, 0.3).is_none());
        let h = carried_center(Carry::Hands, [0.0; 3], 0.0, 1.0, 0, 0.3).unwrap();
        assert!((h[1] - 1.0).abs() < 1e-6 && h[0] > 0.3);
    }

    #[test]
    fn a_forklift_turns_to_face_the_rack_cell_mid_window() {
        use utsim_scene::compose::{FORK_REACH, rack_approach};
        // The scene's fork reach is the library model's.
        assert_eq!(FORK_REACH, dims::FORK_CENTER);
        let (rack, pos) = ([60.0, 0.0, 0.0], [60.0, 0.0, 0.0]);
        // At the window ends it is where the trace says.
        let (p, y) = rack_approach(pos, 0.0, 0.0, rack, 1.8, 0.0);
        assert_eq!((p, y), (pos, 0.0));
        // In the middle: in front of the cell, facing the rack (-Z), forks under the cell.
        let (p, y) = rack_approach(pos, 0.0, 0.5, rack, 1.8, 0.0);
        assert!((y - PI / 2.0).abs() < 1e-5);
        let fork = carried_center(
            Carry::Forks {
                forward: dims::FORK_CENTER,
                height: None,
            },
            p,
            y,
            1.5,
            0,
            0.6,
        )
        .unwrap();
        assert!(
            (fork[0] - 61.8).abs() < 1e-4 && fork[2].abs() < 1e-4,
            "{fork:?}"
        );
        // Turns the short way round.
        let (_, y) = rack_approach(pos, PI, 0.5, rack, 0.0, 0.0);
        assert!((y - PI / 2.0).abs() < 1e-5, "{y}");
    }

    #[test]
    fn people_pick_the_clip_of_their_motion() {
        let walk = Motion::Moving { speed: 1.2 };
        assert_eq!(person_clip(walk, false), Clip::Walk);
        assert_eq!(person_clip(Motion::Moving { speed: 2.5 }, false), Clip::Jog);
        assert_eq!(person_clip(walk, true), Clip::Carry);
        assert_eq!(person_clip(Motion::Idle, true), Clip::IdleCarry);
        assert_eq!(person_clip(Motion::Blocked, false), Clip::Idle);
        assert_eq!(person_clip(Motion::Stopped, false), Clip::Idle);
        assert_eq!(person_clip(Motion::Working, false), Clip::Interact);
        assert_eq!(
            person_clip(Motion::Unloading { progress: 0.3 }, true),
            Clip::PickUp
        );
        for c in Clip::ALL {
            assert_eq!(Clip::ALL[c.index()], c);
        }
    }

    #[test]
    fn the_walk_cycle_follows_the_distance_so_the_feet_do_not_slide() {
        let (d, s) = (1.3333, 0.96);
        let stride = Clip::Walk.stride().unwrap() * s;
        // One loop per stride, whatever the time.
        let a = clip_time(Clip::Walk, d, 2.0, s, 10.0, 0.0);
        let b = clip_time(Clip::Walk, d, 2.0 + stride, s, 99.0, 0.0);
        assert!((a - b).abs() < 1e-3, "{a} {b}");
        // At speed v the clip plays at v / (stride / duration) times its own rate.
        let v = 1.5;
        let rate = (clip_time(Clip::Walk, d, 0.1 * v, s, 0.0, 0.0)
            - clip_time(Clip::Walk, d, 0.0, s, 0.0, 0.0))
            / 0.1;
        assert!((rate - v * d / stride).abs() < 1e-3, "{rate}");
        // The pick-up follows the window; the rest loops on the simulated clock.
        assert!((clip_time(Clip::PickUp, 0.8, 0.0, s, 5.0, 0.5) - 0.4).abs() < 1e-6);
        let idle = clip_time(Clip::Idle, 2.5, 7.0, s, 3.5, 0.0);
        assert!((idle - 1.0).abs() < 1e-5);
        assert_eq!(idle, clip_time(Clip::Idle, 2.5, 0.0, s, 3.5, 0.9));
    }

    #[test]
    fn clips_crossfade_over_a_quarter_second() {
        let mut w = [0.0; 7];
        w[Clip::Idle.index()] = 1.0;
        fade(&mut w, Clip::Walk, 0.1);
        assert!((w[Clip::Walk.index()] - 0.4).abs() < 1e-6);
        assert!((w[Clip::Idle.index()] - 0.6).abs() < 1e-6);
        fade(&mut w, Clip::Walk, 0.2);
        assert_eq!(w[Clip::Walk.index()], 1.0);
        assert_eq!(w[Clip::Idle.index()], 0.0);
    }

    #[test]
    fn people_get_the_skeletal_mannequin_when_it_is_available() {
        let person = Some(MobileRole::Person);
        let g = geom("Transporter");
        let ctx = Context {
            skeletal_people: true,
            ..Context::default()
        };
        let l = look_of("Transporter", person, None, &g, ctx, None).unwrap();
        let entry = catalogue::by_name(PERSON).unwrap();
        assert_eq!(
            l.source,
            Source::Catalogue {
                name: entry.nome.clone(),
                file: catalogue::file_of(entry),
            }
        );
        assert!(l.skeletal && l.carry == Carry::Hands);
        // Without the files: the procedural operator.
        let l = look_of("Transporter", person, None, &g, Context::default(), None).unwrap();
        assert_eq!(l.source, Source::Library(Spec::OperadorSimples));
        assert!(!l.skeletal);
        // A name that only marks the role keeps the mannequin; a library name wins.
        let only_role = VisualDef {
            asset: Some("pessoa".into()),
            ..VisualDef::default()
        };
        let l = look_of("Transporter", person, Some(&only_role), &g, ctx, None).unwrap();
        assert!(l.skeletal);
        let simple = VisualDef {
            asset: Some("operador_simples".into()),
            ..VisualDef::default()
        };
        let l = look_of("Transporter", person, Some(&simple), &g, ctx, None).unwrap();
        assert_eq!(l.source, Source::Library(Spec::OperadorSimples));
        // Both files of the library share the skeleton.
        assert!(is_skeletal("pessoa_operador_carga") && !is_skeletal("pessoa_mini_homem_a"));
    }

    #[test]
    fn vehicles_stop_at_the_edge_of_a_station() {
        let (c, h) = ([60.0, 0.0, 0.0], [0.9, 0.5, 0.9]);
        // Arriving along +x at the centre: held 0.9 + 0.6 before it.
        let p = keep_outside([60.0, 0.0, 0.0], 0.0, c, h, 0.0, 0.6);
        assert!((p[0] - 58.5).abs() < 1e-5 && p[2].abs() < 1e-6, "{p:?}");
        // Heading model +y (layout -Z): held on the +Z side.
        let p = keep_outside([60.0, 0.0, 0.2], PI / 2.0, c, h, 0.0, 0.6);
        assert!(
            (p[2] - 1.5).abs() < 1e-5 && (p[0] - 60.0).abs() < 1e-5,
            "{p:?}"
        );
        // Outside: unchanged.
        assert_eq!(
            keep_outside([50.0, 0.0, 0.0], 0.0, c, h, 0.0, 0.6),
            [50.0, 0.0, 0.0]
        );
    }

    #[test]
    fn a_turned_station_has_its_front_and_its_footprint_turned_too() {
        // A long station (2 x 0.5 half extents) turned a quarter: its front (+Z of its own
        // frame) is now layout +X, and the operator faces -X (heading pi).
        let (c, h) = ([10.0, 0.0, 0.0], [2.0, 0.5, 0.5]);
        let (p, y) = station_front(c, h, PI / 2.0);
        assert!((p[0] - 10.8).abs() < 1e-5 && p[2].abs() < 1e-5, "{p:?}");
        assert!((y - PI).abs() < 1e-5);
        assert_eq!(station_front(c, h, 0.0).0, [10.0, 0.0, 0.8]);
        // Unturned, a point 1.5 m along -Z is outside (0.5 + 0.6 < 1.5); turned a
        // quarter, the long side lies along Z and holds it.
        let at = [10.0, 0.0, -1.5];
        assert_eq!(keep_outside(at, PI / 2.0, c, h, 0.0, 0.6), at);
        let p = keep_outside(at, -PI / 2.0, c, h, PI / 2.0, 0.6);
        // Heading -pi/2 is layout +Z: moved back along -Z to the turned edge, 2 + 0.6.
        assert!(
            (p[2] + 2.6).abs() < 1e-4 && (p[0] - 10.0).abs() < 1e-4,
            "{p:?}"
        );
    }

    #[test]
    fn wheels_roll_backwards_about_the_axle() {
        assert!((wheel_angle(0.33, 0.33) + 1.0).abs() < 1e-5);
        assert_eq!(wheel_angle(1.0, 0.0), 0.0);
    }

    #[test]
    fn external_texture_uris_are_found_in_a_glb() {
        let glb = |json: &str| {
            let mut json = json.as_bytes().to_vec();
            while !json.len().is_multiple_of(4) {
                json.push(b' ');
            }
            let mut b = b"glTF".to_vec();
            b.extend(2u32.to_le_bytes());
            b.extend(((20 + json.len()) as u32).to_le_bytes());
            b.extend((json.len() as u32).to_le_bytes());
            b.extend(0x4E4F_534Au32.to_le_bytes());
            b.extend(json);
            b
        };
        let file = glb(
            r#"{"images":[{"uri":"Textures/colormap.png"},{"uri":"data:image/png;base64,AA"},
                {"uri":"../fora.png"},{"bufferView":0}],"buffers":[{"byteLength":4}]}"#,
        );
        assert_eq!(relative_uris(&file), vec!["Textures/colormap.png"]);
        assert!(relative_uris(b"nada").is_empty());
        // The library models embed everything.
        assert!(relative_uris(&Spec::Maquina.glb()).is_empty());
    }

    #[test]
    fn carry_names_are_the_plain_data_of_the_bridge() {
        let forks = Carry::Forks {
            forward: 1.0,
            height: None,
        };
        let names: Vec<_> = [forks, Carry::Hands, Carry::Deck(0.3), Carry::Stack]
            .into_iter()
            .map(Carry::name)
            .collect();
        assert_eq!(names, ["forks", "hands", "deck", "stack"]);
    }

    #[test]
    fn colors_parse() {
        assert_eq!(parse_color("#00ff00"), Some([0.0, 1.0, 0.0]));
        assert_eq!(parse_color("verde"), None);
    }
}
