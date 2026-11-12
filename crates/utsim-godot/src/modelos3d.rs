//! The 3D elements made in Blender (`assets/modelos3d/`, `docs/elementos_3d_blender.md`):
//! which block is drawn with which `.glb`, the clip each block plays and its phase, where
//! the operator of a station stands (its `ponto_operador` and the walk of the state table),
//! and how each person is dressed (body, protective equipment, colours, height). No Godot
//! types: the bridge (`lib.rs`) copies these numbers into the static scene and the frame
//! arrays.
//!
//! # Which file
//! The interface looks in `assets/modelos3d/` first and keeps the library of
//! `assets/modelos/` as the fallback:
//! - a library asset with fixed size maps to the Blender element made to replace it
//!   ([`stem_for_spec`]: `maquina` → `cnc`, `bancada` → `posto`, and the vehicles and
//!   loads of the same name). Both follow the same conventions (metres, origin on the floor
//!   at the base centre, `+x` front, the same named nodes and the same fork, deck and seat
//!   heights), so the look's scale, turn and offset stay as they are;
//! - the parametric assets (rack, conveyor, floor area, tank) keep the library: their size
//!   comes from the model (bays, levels, length), and the Blender file has one fixed size;
//! - `visual.asset` ([`pedido`]) naming an element of its own (`cnc`, `prensa`, `robo`,
//!   `celula`, `estante`, `cnc_medio`...) draws that file; `<library>_medio` keeps the
//!   library asset (its carry, seat and size) with the `_medio` file when it replaces a
//!   fixed-size asset (`agv_medio`), and the plain library asset when it is parametric
//!   (`esteira_medio`, `porta_paletes_medio`);
//! - people with the skeletal mannequin get `pessoa_homem` or `pessoa_mulher` ([`Pessoa`]),
//!   which have the same skeleton, so the same clips drive them.
//!
//! # Clips
//! Each element has the table `estados` ([`Estado`]): the clip a state plays, its phase
//! from `de` to `ate` over the state's interval in the trace, and the operator's walk.
//! Everything is resolved once per block ([`Animado`]) and the runs of each block's states
//! once per playback ([`runs`]); a frame only looks them up.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use utsim_assets::Spec;
use utsim_core::State;
use utsim_scene::compose::turned;

use crate::coords::to_godot;

/// Length of every clip of the catalogue, seconds of simulated time (24 frames).
pub const CLIP_SECONDS: f64 = 1.0;

/// A place in the layout (Godot, Y up) and a heading about `+Y` (radians).
pub type Pose = ([f32; 3], f32);

/// The runs of a block's states: the start of each, its line of the state table and its
/// line of the part table ([`Animado::pecas`]).
pub type Runs = Vec<(f64, Option<usize>, Option<usize>)>;

/// The run containing an instant ([`run_at`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Run {
    /// Line of the state table ([`Animado::estados`]).
    pub line: Option<usize>,
    /// Line of the part table ([`Animado::pecas`]).
    pub peca: Option<usize>,
    pub start: f64,
    pub end: f64,
}

/// A segment of a part line: `(phase from, phase to, index into Animado::pontos)`.
pub type Trecho = (f32, f32, usize);

/// The line of the state table that stands for `processing` at a station nobody operates.
pub const PROCESSING_AUTO: &str = "processing_auto";

/// A point of an element (an empty node): its place and where its `+x` looks.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Ponto {
    /// Model coordinates of the element (metres, Z up).
    pub pos: [f32; 3],
    /// Turn about Z, degrees.
    #[serde(default)]
    pub giro: f32,
    /// Node it hangs from (the root, or a moving part).
    #[serde(default)]
    pub pai: String,
}

/// A clip of an element.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Clipe {
    #[serde(default)]
    pub laco: bool,
    #[serde(default)]
    pub nos: Vec<String>,
}

/// One line of an element's state table: the clip of a station state, its phase from `de`
/// to `ate` over the state, whether it loops (one loop per state), and the operator's walk
/// `[from point, to point]`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Estado {
    pub clipe: String,
    #[serde(default)]
    pub de: f32,
    #[serde(default = "one")]
    pub ate: f32,
    #[serde(default)]
    pub laco: bool,
    #[serde(default)]
    pub operador: Option<[String; 2]>,
}

fn one() -> f32 {
    1.0
}

/// One file of the catalogue (`catalogo.json`; the fields the bridge uses).
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Elemento {
    pub arquivo: String,
    #[serde(default)]
    pub clipes: BTreeMap<String, Clipe>,
    #[serde(default)]
    pub estados: BTreeMap<String, Estado>,
    /// Where the part (or the load) is in each state: `[phase from, phase to, point]`.
    #[serde(default)]
    pub peca: BTreeMap<String, Vec<(f32, f32, String)>>,
    #[serde(default)]
    pub dados: serde_json::Value,
    #[serde(default)]
    pub nos: Vec<String>,
    #[serde(default)]
    pub pontos: BTreeMap<String, Ponto>,
    #[serde(default)]
    pub tamanho: [f32; 3],
}

impl Elemento {
    /// Clip names, sorted (the indices of the frame's `anim_clip`).
    pub fn clip_names(&self) -> Vec<String> {
        self.clipes.keys().cloned().collect()
    }

    /// A point that hangs from the element's root (it does not move with a clip).
    pub fn fixed_point(&self, stem: &str, name: &str) -> Option<&Ponto> {
        let root = stem.trim_end_matches("_medio");
        self.pontos
            .get(name)
            .filter(|p| p.pai.is_empty() || p.pai == root)
    }
}

/// The catalogue of the Blender elements (only those whose file exists) and their folder.
#[derive(Clone, Debug, PartialEq)]
pub struct Catalogo {
    pub dir: PathBuf,
    pub elementos: BTreeMap<String, Elemento>,
}

impl Catalogo {
    /// Reads `catalogo.json` of `dir` and keeps the elements whose file is there (checked
    /// once, here); `None` when it is missing or unreadable.
    pub fn load(dir: &Path) -> Option<Catalogo> {
        let text = std::fs::read_to_string(dir.join("catalogo.json")).ok()?;
        let mut elementos: BTreeMap<String, Elemento> = serde_json::from_str(&text).ok()?;
        elementos.retain(|_, e| dir.join(&e.arquivo).is_file());
        Some(Catalogo {
            dir: dir.to_path_buf(),
            elementos,
        })
    }

    /// The catalogue found by [`dir_from`].
    pub fn find(start: Option<&Path>) -> Option<Catalogo> {
        Catalogo::load(&dir_from(start)?)
    }

    /// The element `stem` (exact, lower case).
    pub fn get(&self, stem: &str) -> Option<&Elemento> {
        self.elementos.get(stem)
    }

    /// The file of element `stem`.
    pub fn path(&self, stem: &str) -> PathBuf {
        let e = self.elementos.get(stem);
        self.dir
            .join(e.map_or_else(|| format!("{stem}.glb"), |e| e.arquivo.clone()))
    }
}

/// The `assets/modelos3d` folder: `UTSIM_MODELOS3D` when set (a folder without
/// `catalogo.json` turns the Blender elements off), else the first `assets/modelos3d`
/// with `catalogo.json` above `start` (the model's folder), the executable or the working
/// directory, else the one of the source tree, when it exists.
pub fn dir_from(start: Option<&Path>) -> Option<PathBuf> {
    let has = |d: &Path| d.join("catalogo.json").is_file();
    if let Some(d) = std::env::var_os("UTSIM_MODELOS3D") {
        let d = PathBuf::from(d);
        return has(&d).then_some(d);
    }
    let starts = [
        start.map(Path::to_path_buf),
        std::env::current_exe().ok(),
        std::env::current_dir().ok(),
    ];
    for s in starts.into_iter().flatten() {
        for a in s.ancestors() {
            let d = a.join("assets").join("modelos3d");
            if has(&d) {
                return Some(d);
            }
        }
    }
    let d = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/modelos3d");
    has(&d).then_some(d)
}

/// The Blender element made to replace a library asset of fixed size; `None` for the
/// parametric ones (their size comes from the model) and those without an element.
pub fn stem_for_spec(spec: &Spec) -> Option<&'static str> {
    match spec {
        Spec::Maquina => Some("cnc"),
        Spec::Bancada => Some("posto"),
        Spec::Empilhadeira => Some("empilhadeira"),
        Spec::Agv => Some("agv"),
        Spec::Caixa => Some("caixa"),
        Spec::Palete => Some("palete"),
        _ => None,
    }
}

/// The two bodies of the people.
pub const PESSOAS: [&str; 2] = ["pessoa_homem", "pessoa_mulher"];

/// What a `visual.asset` asks of the catalogue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pedido {
    /// An element of its own (`cnc`, `prensa`, `robo`, `estante`, `cnc_medio`...): draw
    /// that file.
    Elemento(String),
    /// `<library>_medio`: the library asset `base` (its carry, seat, size), drawn with the
    /// element `stem` when it replaces a fixed-size asset.
    Biblioteca { base: String, stem: Option<String> },
}

/// What `asset` asks of the catalogue; `None` for everything else (library names, people,
/// third-party names, files, unknown names), which keep the usual look.
pub fn pedido(cat: &Catalogo, asset: Option<&str>) -> Option<Pedido> {
    let n = asset?.trim().to_lowercase();
    if let Some(base) = n.strip_suffix("_medio")
        && let Some(spec) = Spec::by_name(base)
    {
        let stem = (stem_for_spec(&spec) == Some(base) && cat.get(&n).is_some()).then(|| n.clone());
        return Some(Pedido::Biblioteca {
            base: base.to_string(),
            stem,
        });
    }
    if Spec::by_name(&n).is_some()
        || PESSOAS.contains(&n.as_str())
        || utsim_look::catalogue::by_name(&n).is_some()
    {
        return None;
    }
    cat.get(&n).map(|_| Pedido::Elemento(n))
}

/// Where an asset is placed under its block's root, and the root itself (Godot, Y up).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placement {
    /// Uniform scale of the asset.
    pub scale: f32,
    /// Turn of the asset about `+Y` under the root (radians).
    pub asset_yaw: f32,
    /// Translation of the asset under the root.
    pub offset: [f32; 3],
    /// The root: its position and turn.
    pub root: [f32; 3],
    pub turn: f32,
}

/// A point of an asset (model coordinates of the element, Z up) in the scene's layout
/// space (Godot, Y up), with the heading of its `+x` (radians about `+Y`): the asset's
/// scale, turn and offset under the root, then the root's turn and position.
pub fn point_to_layout(p: &Ponto, at: &Placement) -> Pose {
    let g = to_godot(p.pos).map(|v| v * at.scale);
    let a = turned(g, at.asset_yaw);
    let local = [
        a[0] + at.offset[0],
        a[1] + at.offset[1],
        a[2] + at.offset[2],
    ];
    let w = turned(local, at.turn);
    (
        [at.root[0] + w[0], at.root[1] + w[1], at.root[2] + w[2]],
        at.turn + at.asset_yaw + p.giro.to_radians(),
    )
}

/// One line of the state table, resolved for a placed block.
#[derive(Clone, Debug, PartialEq)]
pub struct Passo {
    /// Index into the block's clips.
    pub clip: i32,
    pub de: f32,
    pub ate: f32,
    pub laco: bool,
    /// The operator's walk (from, to) in the layout, when the element has both points.
    pub caminho: Option<(Pose, Pose)>,
}

/// The clips, the state table and the operator post of one block, resolved once (the
/// points in the layout: stations do not move).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Animado {
    /// Clip names, sorted (the indices of the frame's `anim_clip`).
    pub clips: Vec<String>,
    /// State name (`State::name`) and its line; only states whose clip the element has.
    pub estados: Vec<(String, Passo)>,
    /// Where the operator works (`ponto_operador`), when the element has it.
    pub post: Option<Pose>,
    /// The points the part tables name that the element has (the indices of the frame's
    /// `item_ancora_ponto`), and, for those hanging from the root, their place in the
    /// layout (`None` for the moving ones: the garra, the forks, the deck).
    pub pontos: Vec<(String, Option<Pose>)>,
    /// State name and its part line: `(phase from, phase to, index into pontos)`.
    pub pecas: Vec<(String, Vec<Trecho>)>,
    /// The forks follow the scene's lift (`garfo_subir` over `dados.garfo_curso`), not the
    /// state table: `(clip, rest height, travel)`.
    pub garfo: Option<(i32, f32, f32)>,
    /// The deck rises `dados.mesa_curso` metres at the end of its clip (`mesa_subir`).
    pub mesa: Option<(i32, f32)>,
}

impl Animado {
    /// Resolves element `e` (file `stem`) placed at `at`. `auto`: a station nobody
    /// operates, whose `processing` plays the `processing_auto` lines when it has them.
    pub fn resolve(e: &Elemento, stem: &str, at: &Placement, auto: bool) -> Animado {
        let clips = e.clip_names();
        let point = |name: &str| e.fixed_point(stem, name).map(|p| point_to_layout(p, at));
        // `processing_auto` is not a state of the simulation: it replaces `processing`.
        // The automatic line counts only when its clip is in the file; else `processing`.
        let auto = auto
            && e.estados
                .get(PROCESSING_AUTO)
                .is_some_and(|l| clips.contains(&l.clipe));
        let pick = |state: &String| -> Option<String> {
            if state == PROCESSING_AUTO {
                return None;
            }
            let auto_key = (auto && state == "processing").then_some(PROCESSING_AUTO);
            Some(auto_key.unwrap_or(state).to_string())
        };
        let estados = e
            .estados
            .keys()
            .filter_map(|state| {
                let key = pick(state)?;
                let line = e.estados.get(&key).or_else(|| e.estados.get(state))?;
                Some((state, line))
            })
            .filter_map(|(state, line)| {
                let clip = clips.iter().position(|c| *c == line.clipe)? as i32;
                let caminho = line
                    .operador
                    .as_ref()
                    .and_then(|[a, b]| Some((point(a)?, point(b)?)));
                Some((
                    state.clone(),
                    Passo {
                        clip,
                        de: line.de,
                        ate: line.ate,
                        laco: line.laco,
                        caminho,
                    },
                ))
            })
            .collect();
        let mut pontos: Vec<(String, Option<Pose>)> = Vec::new();
        let mut pecas = Vec::new();
        for state in e.peca.keys() {
            let Some(key) = pick(state) else { continue };
            let Some(line) = e.peca.get(&key).or_else(|| e.peca.get(state)) else {
                continue;
            };
            let mut segs = Vec::new();
            for (f0, f1, name) in line {
                if !e.pontos.contains_key(name) {
                    continue;
                }
                let k = match pontos.iter().position(|(n, _)| n == name) {
                    Some(k) => k,
                    None => {
                        pontos.push((name.clone(), point(name)));
                        pontos.len() - 1
                    }
                };
                segs.push((*f0, *f1, k));
            }
            if !segs.is_empty() {
                pecas.push((state.clone(), segs));
            }
        }
        let garfo = clips
            .iter()
            .position(|c| c == "garfo_subir")
            .zip(e.dados["garfo_curso"].as_f64())
            .map(|(k, travel)| {
                let rest = e.dados["garfo_repouso"].as_f64().unwrap_or(0.0);
                (k as i32, rest as f32, travel.max(1e-3) as f32)
            });
        let mesa = clips
            .iter()
            .position(|c| c == "mesa_subir")
            .zip(e.dados["mesa_curso"].as_f64())
            .map(|(k, curso)| (k as i32, curso as f32));
        Animado {
            clips,
            estados,
            post: point("ponto_operador"),
            pontos,
            pecas,
            garfo,
            mesa,
        }
    }

    /// The part line of `state`, as an index into [`Animado::pecas`].
    pub fn peca_line(&self, state: State) -> Option<usize> {
        self.pecas.iter().position(|(s, _)| s == state.name())
    }

    /// The point (index into [`Animado::pontos`]) where the part of run `run` is at `t`:
    /// the segment of the part line that holds the state's phase (the same phase as the
    /// clip's, before `de` and `ate`).
    pub fn anchor(&self, run: Option<Run>, t: f64) -> Option<usize> {
        let run = run?;
        let segs = &self.pecas[run.peca?].1;
        let looping = run.line.is_some_and(|k| self.estados[k].1.laco);
        let s = if looping && !(run.end - run.start).is_finite() {
            ((t - run.start).max(0.0) / CLIP_SECONDS).fract() as f32
        } else {
            state_phase(run.start, run.end, t)
        };
        segs.iter()
            .find(|(f0, f1, _)| s >= *f0 && s < *f1)
            .or(segs.last().filter(|(_, f1, _)| s >= *f1 - 1e-6))
            .or(segs.first())
            .map(|seg| seg.2)
    }

    /// The fork clip, its phase and the rise past the clip's travel, for a fork `lift`
    /// (top of the forks above the floor, metres, as the scene gives it, world size) on an
    /// asset at `scale`, when the element has forks. The phase is the carriage's rise over
    /// the travel; a rack cell higher than the travel keeps the clip at 1 and the rest of
    /// the rise is returned (asset units), for the engine to add to the `carro` node.
    pub fn fork(&self, lift: f32, scale: f32) -> Option<(i32, f32, f32)> {
        let (clip, rest, travel) = self.garfo?;
        let scale = scale.max(1e-3);
        // Both in the asset's own units: the world lift over the scale, the file's rest.
        let rise = (lift / scale - rest).max(0.0);
        Some((
            clip,
            (rise / travel).clamp(0.0, 1.0),
            (rise - travel).max(0.0),
        ))
    }

    /// The line of `state`, as an index into [`Animado::estados`].
    pub fn line(&self, state: State) -> Option<usize> {
        self.estados.iter().position(|(s, _)| s == state.name())
    }

    /// The runs of the block's states (`(t, state)` in time order) as its tables see
    /// them: the start of each run, its state line and its part line. A state set again
    /// does not restart a clip that does not loop, nor do consecutive states with the same
    /// lines (`waiting_repair` then `maintenance`); a looping clip starts a new loop at
    /// every record (one loop per piece). States without lines are one run.
    pub fn runs(&self, states: impl IntoIterator<Item = (f64, State)>) -> Runs {
        let mut out: Runs = Vec::new();
        for (t, s) in states {
            let (line, peca) = (self.line(s), self.peca_line(s));
            let same_peca = |last: Option<usize>| match (last, peca) {
                (None, None) => true,
                (Some(a), Some(b)) => self.pecas[a].1 == self.pecas[b].1,
                _ => false,
            };
            let same = out.last().is_some_and(|&(_, last, last_peca)| {
                same_peca(last_peca)
                    && match (last, line) {
                        (None, None) => true,
                        (Some(a), Some(b)) => {
                            let (pa, pb) = (&self.estados[a].1, &self.estados[b].1);
                            // Every `processing` record starts a run: one cycle per piece,
                            // also for the automatic cycle of an unmanned station.
                            !pb.laco && pa == pb && s != State::Processing
                        }
                        _ => false,
                    }
            });
            if !same {
                out.push((t, line, peca));
            }
        }
        out
    }

    /// The clip at `t` (index into [`Animado::clips`], -1 for none) and its phase, in the
    /// run `run` ([`run_at`]).
    pub fn anim(&self, run: Option<Run>, t: f64) -> (i32, f32) {
        match run {
            Some(Run {
                line: Some(k),
                start,
                end,
                ..
            }) => {
                let p = &self.estados[k].1;
                (p.clip, clip_phase(p, start, end, t))
            }
            _ => (-1, 0.0),
        }
    }

    /// Where the operator stands at `t`: along the line's walk by the state's phase,
    /// else at the post; `None` without a post or a walk.
    pub fn operator(&self, run: Option<Run>, t: f64) -> Option<Pose> {
        if let Some(Run {
            line: Some(k),
            start,
            end,
            ..
        }) = run
            && let Some((a, b)) = self.estados[k].1.caminho
        {
            return Some(walk(a, b, state_phase(start, end, t)));
        }
        self.post
    }
}

/// The run containing `t`.
pub fn run_at(runs: &[(f64, Option<usize>, Option<usize>)], t: f64) -> Option<Run> {
    let k = runs.partition_point(|r| r.0 <= t).checked_sub(1)?;
    let end = runs.get(k + 1).map_or(f64::INFINITY, |r| r.0);
    Some(Run {
        line: runs[k].1,
        peca: runs[k].2,
        start: runs[k].0,
        end,
    })
}

/// Phase of a state at `t` in its interval `start..end`, 0 to 1 (`end` infinite: the run
/// goes past the end of the trace, and the phase holds at 0.5).
pub fn state_phase(start: f64, end: f64, t: f64) -> f32 {
    let span = end - start;
    if !span.is_finite() {
        // The run goes past the end of the trace: its length is unknown, so the phase
        // holds at the middle (a door half open, the part at the work point).
        return 0.5;
    }
    let p = if span > 1e-9 {
        (t - start).max(0.0) / span
    } else {
        1.0
    };
    p.clamp(0.0, 1.0) as f32
}

/// Phase of the clip of line `p` at `t`: `de + (ate - de) * phase of the state`. A looping
/// clip makes one loop over the state (one per clip length when the state has no end).
pub fn clip_phase(p: &Passo, start: f64, end: f64, t: f64) -> f32 {
    let open = !(end - start).is_finite();
    let s = if p.laco && open {
        ((t - start).max(0.0) / CLIP_SECONDS).fract() as f32
    } else {
        state_phase(start, end, t)
    };
    (p.de + (p.ate - p.de) * s).clamp(0.0, 1.0)
}

/// The pose `p` of the way (0 to 1) from `a` to `b`, turning the short way round.
pub fn walk(a: Pose, b: Pose, p: f32) -> Pose {
    let p = p.clamp(0.0, 1.0);
    let lerp = |x: f32, y: f32| x + (y - x) * p;
    let tau = std::f32::consts::TAU;
    let d = (b.1 - a.1 + std::f32::consts::PI).rem_euclid(tau) - std::f32::consts::PI;
    (
        [
            lerp(a.0[0], b.0[0]),
            lerp(a.0[1], b.0[1]),
            lerp(a.0[2], b.0[2]),
        ],
        a.1 + d * p,
    )
}

/// How a person is drawn with the Blender bodies: the file, the protective equipment
/// shown (`epi_<name>` nodes), the colours of the recoloured materials (`"#rrggbb"`) and
/// the height, all picked from the person's node and, where the model says nothing, from
/// its name (the same every run).
#[derive(Clone, Debug, PartialEq)]
pub struct Pessoa {
    /// `pessoa_homem` or `pessoa_mulher`.
    pub arquivo: String,
    pub path: PathBuf,
    pub epis: Vec<String>,
    pub cores: Vec<(String, String)>,
    /// Height, metres, and the scale that gives it (height / `altura_base`).
    pub altura: f32,
    pub escala: f32,
    /// Width and depth over the height scale: `sqrt(IMC / imc_base)` within the body's
    /// `largura` range, 1 without a mass.
    pub largura: f32,
    /// Scale and offset of the mannequin the body replaces (the fallback when the body
    /// does not load).
    pub escala_boneco: f32,
    pub offset_boneco: [f32; 3],
}

/// FNV-1a of `name` and `salt`: a stable number per person and per choice.
fn hash(name: &str, salt: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in name.bytes().chain([0u8]).chain(salt.bytes()) {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// A protective equipment name as the catalogue writes it: lower case, no accents,
/// `_` for spaces and hyphens, and the usual other names (`"Óculos"` → `oculos`,
/// `"protetor auricular"` → `protetor_auricular`, `respirador` → `mascara`, `luva` →
/// `luvas`, `bota` → `botina`).
pub fn epi_key(name: &str) -> String {
    let key: String = name
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            ' ' | '-' => '_',
            other => other,
        })
        .collect();
    let alias = match key.as_str() {
        "respirador" | "respiradores" | "mascaras" => "mascara",
        "luva" => "luvas",
        "bota" | "botas" | "botinas" => "botina",
        "capacetes" => "capacete",
        "coletes" => "colete",
        "aventais" => "avental",
        "protetores_auriculares" | "protetor_auriculares" | "abafador" => "protetor_auricular",
        "oculos_de_protecao" | "oculos_de_seguranca" => "oculos",
        _ => return key,
    };
    alias.to_string()
}

fn strings(v: &serde_json::Value) -> Vec<String> {
    v.as_array()
        .into_iter()
        .flatten()
        .filter_map(|s| s.as_str().map(str::to_string))
        .collect()
}

/// `"#rrggbb"` of a colour text, or `None` when it is not one.
fn hex(text: &str) -> Option<String> {
    let h = text.trim().trim_start_matches('#').to_lowercase();
    (h.len() == 6 && u32::from_str_radix(&h, 16).is_ok()).then(|| format!("#{h}"))
}

/// The body of person `name`. `forced`: the body named by `visual.asset`
/// (`pessoa_homem`, `pessoa_mulher`); `ppe`: what the person wears (`Model::worn_ppe`);
/// `color`: its `visual.color` (the shirt); `corpo`: the person's record (sex, height and
/// mass), where given. `None` when the catalogue has neither body. The mannequin's scale
/// and offset are filled by the caller.
pub fn pessoa(
    cat: &Catalogo,
    name: &str,
    forced: Option<&str>,
    ppe: &[String],
    color: Option<&str>,
    corpo: Option<&utsim_core::Person>,
) -> Option<Pessoa> {
    let forced = forced
        .map(|f| f.trim().to_lowercase())
        .filter(|f| PESSOAS.contains(&f.as_str()));
    let pick = match corpo.and_then(|p| p.sex) {
        Some(utsim_core::Sex::Male) => PESSOAS[0],
        Some(utsim_core::Sex::Female) => PESSOAS[1],
        None => PESSOAS[(hash(name, "sexo") % 2) as usize],
    };
    let arquivo = match forced {
        Some(f) => f,
        None if cat.get(pick).is_some() => pick.to_string(),
        // Only one body available: everyone gets it.
        None => PESSOAS.iter().find(|p| cat.get(p).is_some())?.to_string(),
    };
    let e = cat.get(&arquivo)?;
    let d = &e.dados;
    let known = strings(&d["epis"]);
    let mut epis: Vec<String> = ppe
        .iter()
        .map(|p| epi_key(p))
        .filter(|p| known.contains(p))
        .collect();
    epis.sort();
    epis.dedup();
    let recolor = strings(&d["recolorir"]);
    let choose = |list: &str, salt: &str| {
        let options = strings(&d[list]);
        (!options.is_empty())
            .then(|| options[(hash(name, salt) % options.len() as u64) as usize].clone())
            .and_then(|c| hex(&c))
    };
    let mut cores = Vec::new();
    for material in &recolor {
        let c = match material.as_str() {
            "pele" => choose("peles", "pele"),
            "cabelo" => choose("cabelos", "cabelo"),
            "camisa" => color.and_then(hex),
            _ => None,
        };
        if let Some(c) = c {
            cores.push((material.clone(), c));
        }
    }
    let alturas: Vec<f32> = d["alturas"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|v| v.as_f64().map(|x| x as f32))
        .collect();
    let base = d["altura_base"]
        .as_f64()
        .map_or(1.82, |v| v as f32)
        .max(0.1);
    let u = (hash(name, "altura") % 1001) as f32 / 1000.0;
    let drawn = match alturas.as_slice() {
        [lo, hi, ..] => lo + (hi - lo) * u,
        [one] => *one,
        [] => base,
    };
    let given = corpo
        .and_then(|p| p.height_m)
        .filter(|h| h.is_finite() && *h > 0.0);
    let altura = given.map_or(drawn, |h| h as f32);
    let largura = largura(d, given, corpo.and_then(|p| p.mass_kg));
    Some(Pessoa {
        path: cat.dir.join(&e.arquivo),
        arquivo,
        epis,
        cores,
        altura,
        escala: altura / base,
        largura,
        escala_boneco: 1.0,
        offset_boneco: [0.0; 3],
    })
}

/// Degrees the seated person leans forward (`dados.sentado_inclinar` of the person
/// bodies): the clips sit it too far back. 0 when the catalogue does not say.
pub fn sentado_inclinar(cat: &Catalogo) -> f32 {
    PESSOAS
        .iter()
        .filter_map(|p| cat.get(p))
        .find_map(|e| e.dados["sentado_inclinar"].as_f64())
        .filter(|g| g.is_finite())
        .map_or(0.0, |g| g as f32)
}

/// Leans the seated driver `graus` forward about its hip: the root and the eyes turn
/// about the hip, on the driver's own side axis (its `x`, turned by its `yaw`). Returns
/// the lean in radians, for the scene to turn the mannequin by.
pub fn inclinar_motorista(m: &mut utsim_look::driver::DriverMount, graus: f32) -> f32 {
    let a = graus.to_radians();
    if a == 0.0 {
        return 0.0;
    }
    // The side axis of the driver in the vehicle's frame (+x of the mannequin, turned by
    // `yaw` as the engines turn: +z towards +x).
    let (s, c) = m.yaw.sin_cos();
    let axis = [c, 0.0, -s];
    let turn = |v: [f32; 3]| {
        let d = [v[0] - m.hip[0], v[1] - m.hip[1], v[2] - m.hip[2]];
        // Rodrigues: d cos a + (k × d) sin a + k (k·d)(1 − cos a).
        let (sa, ca) = a.sin_cos();
        let k = axis;
        let cross = [
            k[1] * d[2] - k[2] * d[1],
            k[2] * d[0] - k[0] * d[2],
            k[0] * d[1] - k[1] * d[0],
        ];
        let dot = k[0] * d[0] + k[1] * d[1] + k[2] * d[2];
        let r: [f32; 3] =
            std::array::from_fn(|i| d[i] * ca + cross[i] * sa + k[i] * dot * (1.0 - ca));
        [m.hip[0] + r[0], m.hip[1] + r[1], m.hip[2] + r[2]]
    };
    m.position = turn(m.position);
    m.eye = turn(m.eye);
    a
}

/// Width factor of a body (`dados` of its catalogue entry) for a person `height_m` tall
/// and `mass_kg` heavy: `sqrt(IMC / imc_base)`, IMC = mass / height², kept within the
/// body's `largura` range. 1 without a mass or a height (a drawn height has no mass to
/// go with it), or without `imc_base` in the catalogue.
fn largura(d: &serde_json::Value, height_m: Option<f64>, mass_kg: Option<f64>) -> f32 {
    let (Some(h), Some(m), Some(base)) = (height_m, mass_kg, d["imc_base"].as_f64()) else {
        return 1.0;
    };
    if !(h > 0.0 && m > 0.0 && base > 0.0 && m.is_finite()) {
        return 1.0;
    }
    let range: Vec<f64> = d["largura"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(serde_json::Value::as_f64)
        .collect();
    let (lo, hi) = match range.as_slice() {
        [lo, hi, ..] if lo <= hi => (*lo, *hi),
        _ => (0.85, 1.25),
    };
    ((m / (h * h)) / base).sqrt().clamp(lo, hi) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    fn cat() -> Catalogo {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/modelos3d");
        Catalogo::load(&dir).expect("assets/modelos3d/catalogo.json")
    }

    fn close(a: [f32; 3], b: [f32; 3]) -> bool {
        (0..3).all(|k| (a[k] - b[k]).abs() < 1e-4)
    }

    const AT: Placement = Placement {
        scale: 1.0,
        asset_yaw: 0.0,
        offset: [0.0; 3],
        root: [0.0; 3],
        turn: 0.0,
    };

    #[test]
    fn the_seated_driver_leans_forward_about_the_hip() {
        let mount = |yaw: f32| utsim_look::driver::DriverMount {
            position: [0.0, 0.2, -0.1],
            yaw,
            scale: 1.0,
            eye: [0.0, 1.3, 0.0],
            hip: [0.0, 0.7, 0.0],
        };
        // Facing +z: the eyes go forward (+z) and down, the hip stays, distances kept.
        let mut m = mount(0.0);
        let a = inclinar_motorista(&mut m, 15.0);
        assert!((a - 15f32.to_radians()).abs() < 1e-6);
        let s = 15f32.to_radians().sin();
        let c = 15f32.to_radians().cos();
        assert!(close(m.eye, [0.0, 0.7 + 0.6 * c, 0.6 * s]), "{:?}", m.eye);
        assert_eq!(m.hip, [0.0, 0.7, 0.0]);
        // Turned a quarter (facing +x): the lean goes towards +x.
        let mut q = mount(FRAC_PI_2);
        q.eye = [0.0, 1.3, 0.0];
        inclinar_motorista(&mut q, 15.0);
        assert!(q.eye[0] > 0.15 && q.eye[2].abs() < 1e-5, "{:?}", q.eye);
        // No lean: nothing moves.
        let mut z = mount(0.3);
        assert_eq!(inclinar_motorista(&mut z, 0.0), 0.0);
        assert_eq!(z, mount(0.3));
        // The catalogue says how much (0 when it does not).
        let g = sentado_inclinar(&cat());
        assert!((0.0..=30.0).contains(&g), "{g}");
    }

    #[test]
    fn the_record_of_a_person_sets_the_body_its_height_and_its_width() {
        let c = cat();
        let person = |sex, h, m| utsim_core::Person {
            sex,
            height_m: h,
            mass_kg: m,
            ..Default::default()
        };
        let base = |file: &str, key: &str| c.get(file).unwrap().dados[key].as_f64().unwrap();
        // The sex picks the body, whatever the name draws.
        for name in ["ana", "operador_1", "operador_2", "operador_3"] {
            let men = person(Some(utsim_core::Sex::Male), None, None);
            let women = person(Some(utsim_core::Sex::Female), None, None);
            let a = pessoa(&c, name, None, &[], None, Some(&men)).unwrap();
            let b = pessoa(&c, name, None, &[], None, Some(&women)).unwrap();
            assert_eq!(
                (a.arquivo.as_str(), b.arquivo.as_str()),
                ("pessoa_homem", "pessoa_mulher")
            );
            // Without a sex, the name; the body named by the look still wins.
            let none = pessoa(&c, name, None, &[], None, Some(&person(None, None, None))).unwrap();
            assert_eq!(none, pessoa(&c, name, None, &[], None, None).unwrap());
            let forced = pessoa(&c, name, Some("pessoa_mulher"), &[], None, Some(&men)).unwrap();
            assert_eq!(forced.arquivo, "pessoa_mulher");
        }
        // The height given: scale = height / altura_base; no mass, width 1.
        let tall = person(Some(utsim_core::Sex::Male), Some(1.95), None);
        let a = pessoa(&c, "ana", None, &[], None, Some(&tall)).unwrap();
        assert!((a.altura - 1.95).abs() < 1e-6);
        let hb = base("pessoa_homem", "altura_base") as f32;
        assert!((a.escala - 1.95 / hb).abs() < 1e-6);
        assert_eq!(a.largura, 1.0);
        // The mass: width = sqrt(IMC / imc_base), 120 kg at 1,95 m → IMC 31,56.
        let heavy = person(Some(utsim_core::Sex::Male), Some(1.95), Some(120.0));
        let a = pessoa(&c, "ana", None, &[], None, Some(&heavy)).unwrap();
        let imc = 120.0 / (1.95f64 * 1.95);
        let want = (imc / base("pessoa_homem", "imc_base")).sqrt() as f32;
        assert!(
            (a.largura - want).abs() < 1e-5 && a.largura > 1.15,
            "{}",
            a.largura
        );
        // The same IMC as the body's: width 1, for her body too (its own imc_base).
        let ib = base("pessoa_mulher", "imc_base");
        let even = person(
            Some(utsim_core::Sex::Female),
            Some(1.6),
            Some(ib * 1.6 * 1.6),
        );
        let b = pessoa(&c, "ana", None, &[], None, Some(&even)).unwrap();
        assert!((b.largura - 1.0).abs() < 1e-5, "{}", b.largura);
        // Kept within the `largura` range of the body.
        let thin = person(None, Some(1.9), Some(40.0));
        let fat = person(None, Some(1.5), Some(200.0));
        let range = |p: &Pessoa| {
            let r = &c.get(&p.arquivo).unwrap().dados["largura"];
            (r[0].as_f64().unwrap() as f32, r[1].as_f64().unwrap() as f32)
        };
        let t = pessoa(&c, "ana", None, &[], None, Some(&thin)).unwrap();
        let f = pessoa(&c, "ana", None, &[], None, Some(&fat)).unwrap();
        assert_eq!(t.largura, range(&t).0);
        assert_eq!(f.largura, range(&f).1);
        // A mass without a height: the drawn height has no mass to go with it.
        let only_mass = person(None, None, Some(200.0));
        let a = pessoa(&c, "ana", None, &[], None, Some(&only_mass)).unwrap();
        assert_eq!(a.largura, 1.0);
    }

    #[test]
    fn the_catalogue_maps_the_fixed_library_assets_and_keeps_the_parametric_ones() {
        let c = cat();
        for spec in [Spec::Maquina, Spec::Bancada, Spec::Empilhadeira, Spec::Agv] {
            let stem = stem_for_spec(&spec).unwrap();
            assert!(c.get(stem).is_some(), "{stem}");
            assert!(c.path(stem).is_file());
        }
        // Sized by the model: the library keeps drawing them.
        let rack = Spec::by_name("porta_paletes").unwrap();
        let belt = Spec::by_name("esteira").unwrap();
        assert_eq!(stem_for_spec(&rack), None);
        assert_eq!(stem_for_spec(&belt), None);
        // `visual.asset` naming an element of its own; library names and people are not.
        let el = |s: &str| Some(Pedido::Elemento(s.to_string()));
        assert_eq!(pedido(&c, Some(" Prensa ")), el("prensa"));
        assert_eq!(pedido(&c, Some("cnc_medio")), el("cnc_medio"));
        assert_eq!(pedido(&c, Some("estante_medio")), el("estante_medio"));
        assert_eq!(pedido(&c, Some("esteira")), None);
        assert_eq!(pedido(&c, Some("pessoa_homem")), None);
        assert_eq!(pedido(&c, Some("nave")), None);
        assert_eq!(pedido(&c, None), None);
        // `<library>_medio`: the library asset, with the `_medio` file when it replaces a
        // fixed-size asset.
        assert_eq!(
            pedido(&c, Some("agv_medio")),
            Some(Pedido::Biblioteca {
                base: "agv".into(),
                stem: Some("agv_medio".into())
            })
        );
        assert_eq!(
            pedido(&c, Some("esteira_medio")),
            Some(Pedido::Biblioteca {
                base: "esteira".into(),
                stem: None
            })
        );
        // The clips of the CNC, sorted, and its fixed points.
        let cnc = c.get("cnc").unwrap();
        assert_eq!(
            cnc.clip_names(),
            ["ciclo", "ciclo_auto", "porta_abrir", "quebra", "setup"]
        );
        assert!(cnc.fixed_point("cnc", "ponto_operador").is_some());
        assert!(cnc.fixed_point("cnc_medio", "ponto_porta").is_some());
        // The fork's point moves with the forks: not a fixed one.
        let fork = c.get("empilhadeira").unwrap();
        assert!(fork.fixed_point("empilhadeira", "ponto_carga").is_none());
    }

    #[test]
    fn a_point_goes_to_the_layout_with_the_asset_scale_and_the_station_turn() {
        let p = Ponto {
            pos: [-0.15, -1.2, 0.0],
            giro: 90.0,
            pai: "cnc".into(),
        };
        // Unturned, unscaled: model (x, y, z) is Godot (x, z, -y) from the root.
        let at = Placement {
            root: [10.0, 0.0, -5.0],
            ..AT
        };
        let (pos, yaw) = point_to_layout(&p, &at);
        assert!(close(pos, [9.85, 0.0, -3.8]), "{pos:?}");
        // +x of the point turned 90° about Z faces +y of the model: -z in Godot (the
        // machine, from its work side).
        assert!((yaw - FRAC_PI_2).abs() < 1e-6);
        // Scaled twice, then the station turned a quarter: the work side (+z) goes to +x.
        let at = Placement {
            scale: 2.0,
            turn: FRAC_PI_2,
            ..at
        };
        let (pos, yaw) = point_to_layout(&p, &at);
        assert!(close(pos, [10.0 + 2.4, 0.0, -5.0 + 0.3]), "{pos:?}");
        assert!((yaw - PI).abs() < 1e-6);
        // The asset's own turn and offset under the root come before the root's turn.
        let at = Placement {
            asset_yaw: FRAC_PI_2,
            offset: [0.5, 0.0, 0.0],
            turn: FRAC_PI_2,
            ..AT
        };
        let (pos, _) = point_to_layout(&p, &at);
        // (-0.15, 0, 1.2) turned 90° → (1.2, 0, 0.15); + offset (1.7, 0, 0.15); turned
        // 90° again → (0.15, 0, -1.7).
        assert!(close(pos, [0.15, 0.0, -1.7]), "{pos:?}");
    }

    #[test]
    fn the_state_table_gives_the_clip_and_its_phase() {
        let c = cat();
        let a = Animado::resolve(c.get("cnc").unwrap(), "cnc", &AT, false);
        let clip = |n: &str| a.clips.iter().position(|c| c == n).unwrap() as i32;
        let at = |s: State, start: f64, end: f64, t: f64| {
            let runs = a.runs([(start, s), (end, State::Idle)]);
            a.anim(run_at(&runs, t), t)
        };
        // Entering opens the door over the interval, leaving closes it.
        assert_eq!(
            at(State::Entering, 10.0, 14.0, 10.0),
            (clip("porta_abrir"), 0.0)
        );
        assert_eq!(
            at(State::Entering, 10.0, 14.0, 11.0),
            (clip("porta_abrir"), 0.25)
        );
        assert_eq!(
            at(State::Leaving, 10.0, 14.0, 10.0),
            (clip("porta_abrir"), 1.0)
        );
        assert_eq!(
            at(State::Leaving, 10.0, 14.0, 13.0),
            (clip("porta_abrir"), 0.25)
        );
        // One loop of the cycle per processing state.
        assert_eq!(at(State::Processing, 0.0, 20.0, 5.0), (clip("ciclo"), 0.25));
        assert_eq!(at(State::Setup, 0.0, 4.0, 3.0), (clip("setup"), 0.75));
        // Down opens the cover; waiting for the repair and maintenance hold it open.
        assert_eq!(at(State::Down, 0.0, 10.0, 5.0), (clip("quebra"), 0.5));
        assert_eq!(
            at(State::WaitingRepair, 0.0, 10.0, 1.0),
            (clip("quebra"), 1.0)
        );
        assert_eq!(
            at(State::Maintenance, 0.0, 10.0, 1.0),
            (clip("quebra"), 1.0)
        );
        // States outside the table, and the time after the last state change.
        assert_eq!(at(State::Idle, 0.0, 1.0, 0.5), (-1, 0.0));
        assert_eq!(a.anim(None, 0.5), (-1, 0.0));
        // The cell has only the door: processing plays nothing.
        let cell = Animado::resolve(c.get("celula").unwrap(), "celula", &AT, false);
        let runs = cell.runs([(0.0, State::Processing)]);
        assert_eq!(cell.anim(run_at(&runs, 0.5), 0.5), (-1, 0.0));
        // The station posts have setup now.
        let posto = Animado::resolve(c.get("posto").unwrap(), "posto", &AT, false);
        assert!(posto.line(State::Setup).is_some() && posto.line(State::Entering).is_none());
    }

    #[test]
    fn runs_merge_only_the_same_line_of_a_clip_that_does_not_loop() {
        let c = cat();
        let a = Animado::resolve(c.get("cnc").unwrap(), "cnc", &AT, false);
        let r = a.runs([
            (0.0, State::Idle),
            (1.0, State::Blocked),
            (2.0, State::Entering),
            (3.0, State::Entering),
            (5.0, State::Processing),
            (7.0, State::Processing),
            (9.0, State::Down),
            (10.0, State::WaitingRepair),
            (12.0, State::Maintenance),
            (20.0, State::Idle),
        ]);
        let starts: Vec<f64> = r.iter().map(|x| x.0).collect();
        // Blocked has a part line of its own (the part waits at the exit). The door does
        // not restart; each processing record is a loop; waiting and maintenance are the
        // same lines.
        assert_eq!(starts, [0.0, 1.0, 2.0, 5.0, 7.0, 9.0, 10.0, 20.0]);
        assert_eq!(run_at(&r, 6.0).map(|x| (x.start, x.end)), Some((5.0, 7.0)));
        assert_eq!(
            run_at(&r, 15.0).map(|x| (x.start, x.end)),
            Some((10.0, 20.0))
        );
        assert_eq!(run_at(&r, 40.0).unwrap().end, f64::INFINITY);
        assert_eq!(run_at(&r, -1.0), None);
        // A loop with no end: one loop per clip length.
        let open = a.runs([(0.0, State::Processing)]);
        let (_, p) = a.anim(run_at(&open, 2.25), 2.25);
        assert!((p - 0.25).abs() < 1e-5, "{p}");
    }

    #[test]
    fn the_operator_walks_the_table_path_and_stands_at_the_post() {
        let c = cat();
        let a = Animado::resolve(c.get("cnc").unwrap(), "cnc", &AT, false);
        let post = a.post.unwrap();
        assert!(close(post.0, [-0.15, 0.0, 1.2]));
        let door = ([-0.15, 0.0, 1.8], FRAC_PI_2);
        let at = |s: State, t: f64| {
            let runs = a.runs([(0.0, s), (4.0, State::Idle)]);
            a.operator(run_at(&runs, t), t).unwrap()
        };
        assert!(close(at(State::Entering, 0.0).0, door.0));
        assert!(close(at(State::Entering, 2.0).0, [-0.15, 0.0, 1.5]));
        assert!(close(at(State::Entering, 3.9999).0, post.0));
        assert!(close(at(State::Leaving, 0.0).0, post.0));
        assert!(close(at(State::Leaving, 2.0).0, [-0.15, 0.0, 1.5]));
        assert!(close(at(State::Processing, 1.0).0, post.0));
        assert!((at(State::Processing, 1.0).1 - FRAC_PI_2).abs() < 1e-6);
        // No post and no walk (the conveyor): the caller keeps the front of the station.
        let belt = Animado::resolve(c.get("esteira").unwrap(), "esteira", &AT, false);
        assert_eq!(belt.operator(None, 0.0), None);
        // Turning the short way round (from 170° to -170° passes by 180°).
        let (_, yaw) = walk(
            ([0.0; 3], 170f32.to_radians()),
            ([0.0; 3], (-170f32).to_radians()),
            0.5,
        );
        assert!(
            (yaw.rem_euclid(std::f32::consts::TAU) - PI).abs() < 1e-5,
            "{yaw}"
        );
    }

    fn point_name(a: &Animado, k: Option<usize>) -> &str {
        k.map_or("", |k| a.pontos[k].0.as_str())
    }

    #[test]
    fn a_station_nobody_operates_plays_the_automatic_cycle() {
        let c = cat();
        let cnc = c.get("cnc").unwrap();
        let auto = Animado::resolve(cnc, "cnc", &AT, true);
        let manned = Animado::resolve(cnc, "cnc", &AT, false);
        let clip = |a: &Animado, n: &str| a.clips.iter().position(|c| c == n).unwrap() as i32;
        let at = |a: &Animado, t: f64| {
            let runs = a.runs([(0.0, State::Processing), (24.0, State::Idle)]);
            let run = run_at(&runs, t);
            (a.anim(run, t), point_name(a, a.anchor(run, t)).to_string())
        };
        // The door opens for the part to come in, closes over the cycle, opens to let it
        // out: the part goes from the entry to the work point to the exit.
        let ((k, p), at_point) = at(&auto, 1.0);
        assert_eq!(k, clip(&auto, "ciclo_auto"));
        assert!((p - 1.0 / 24.0).abs() < 1e-5);
        assert_eq!(at_point, "ponto_entrada");
        assert_eq!(at(&auto, 12.0).1, "ponto_peca");
        assert_eq!(at(&auto, 23.0).1, "ponto_saida");
        // Operated: the looping cycle, the part at the work point all along.
        let ((k, _), at_point) = at(&manned, 1.0);
        assert_eq!(k, clip(&manned, "ciclo"));
        assert_eq!(at_point, "ponto_peca");
        // An element without the automatic line keeps its cycle.
        let posto = Animado::resolve(c.get("posto").unwrap(), "posto", &AT, true);
        assert_eq!(at(&posto, 12.0).0.0, clip(&posto, "ciclo"));
        // Points of the root are already in the layout.
        let k = auto
            .pontos
            .iter()
            .position(|(n, _)| n == "ponto_peca")
            .unwrap();
        assert!(close(
            auto.pontos[k].1.unwrap().0,
            to_godot([-0.3, -0.45, 1.1])
        ));
    }

    #[test]
    fn the_robot_takes_the_part_from_the_entry_to_the_exit_in_its_gripper() {
        let c = cat();
        let robo = Animado::resolve(c.get("robo").unwrap(), "robo", &AT, true);
        let runs = robo.runs([(0.0, State::Processing), (12.0, State::Blocked)]);
        let at = |t: f64| point_name(&robo, robo.anchor(run_at(&runs, t), t)).to_string();
        assert_eq!(at(2.0), "ponto_entrada");
        assert_eq!(at(6.0), "ponto_garra");
        assert_eq!(at(10.0), "ponto_saida");
        assert_eq!(at(15.0), "ponto_saida");
        // The gripper moves: its place comes from the engine, after the clip.
        let k = robo
            .pontos
            .iter()
            .position(|(n, _)| n == "ponto_garra")
            .unwrap();
        assert_eq!(robo.pontos[k].1, None);
        // The cycle and the part share the phase.
        assert_eq!(robo.anim(run_at(&runs, 6.0), 6.0).1, 0.5);
        // Idle: no part line, no anchor.
        let idle = robo.runs([(0.0, State::Idle)]);
        assert_eq!(robo.anchor(run_at(&idle, 1.0), 1.0), None);
    }

    #[test]
    fn vehicles_lift_their_load_by_the_table_or_by_the_forks_height() {
        let c = cat();
        let agv = Animado::resolve(c.get("agv").unwrap(), "agv", &AT, false);
        let runs = agv.runs([
            (0.0, State::Loading),
            (2.0, State::TravelLoaded),
            (10.0, State::Unloading),
            (12.0, State::TravelEmpty),
        ]);
        let mesa = agv.clips.iter().position(|c| c == "mesa_subir").unwrap() as i32;
        assert_eq!(agv.anim(run_at(&runs, 1.0), 1.0), (mesa, 0.5));
        assert_eq!(agv.anim(run_at(&runs, 5.0), 5.0), (mesa, 1.0));
        assert_eq!(agv.anim(run_at(&runs, 11.5), 11.5), (mesa, 0.25));
        assert_eq!(agv.anim(run_at(&runs, 13.0), 13.0), (-1, 0.0));
        assert_eq!(
            point_name(&agv, agv.anchor(run_at(&runs, 5.0), 5.0)),
            "ponto_carga"
        );
        assert_eq!(agv.anchor(run_at(&runs, 13.0), 13.0), None);
        assert_eq!(agv.fork(1.0, 1.0), None);
        assert_eq!(agv.mesa.map(|m| m.1), Some(0.06));
        // The forks follow the lift: rest 0.10 m, 6 m of telescopic mast; a high bay goes up.
        let fork = Animado::resolve(c.get("empilhadeira").unwrap(), "empilhadeira", &AT, false);
        let garfo = fork.clips.iter().position(|c| c == "garfo_subir").unwrap() as i32;
        assert_eq!(fork.fork(0.10, 1.0), Some((garfo, 0.0, 0.0)));
        let (_, p, _) = fork.fork(0.70, 1.0).unwrap();
        assert!((p - 0.1).abs() < 1e-6, "{p}");
        let (_, p, extra) = fork.fork(4.6, 1.0).unwrap();
        assert!((p - 0.75).abs() < 1e-6, "{p}");
        assert_eq!(extra, 0.0);
        // Only above the 6 m of travel does the rest go to the carriage.
        let (_, p, extra) = fork.fork(7.1, 1.0).unwrap();
        assert_eq!(p, 1.0);
        assert!((extra - 1.0).abs() < 1e-5, "{extra}");
        // At half size: the rest (0.10 in the file) is 0.05 m in the world.
        assert_eq!(fork.fork(0.05, 0.5), Some((garfo, 0.0, 0.0)));
        let (_, p, _) = fork.fork(1.55, 0.5).unwrap();
        assert!((p - 0.5).abs() < 1e-5, "{p}");
    }

    #[test]
    fn each_piece_of_an_unmanned_station_gets_its_own_cycle() {
        let c = cat();
        let cnc = c.get("cnc").unwrap();
        let auto = Animado::resolve(cnc, "cnc", &AT, true);
        // Capacity two: processing is set again at every entry and exit, and each record
        // starts an automatic cycle (one piece), not one cycle for the busy period.
        let r = auto.runs([
            (0.0, State::Processing),
            (4.0, State::Processing),
            (10.0, State::Processing),
            (14.0, State::Idle),
        ]);
        let starts: Vec<f64> = r.iter().map(|x| x.0).collect();
        assert_eq!(starts, [0.0, 4.0, 10.0, 14.0]);
        assert_eq!(auto.anim(run_at(&r, 2.0), 2.0).1, 0.5);
        // The run past the end of the trace has no length: the phase holds at the middle
        // (the door shut, the part at the work point), it does not run to 1 in a second.
        let open = auto.runs([(0.0, State::Processing)]);
        assert_eq!(auto.anim(run_at(&open, 100.0), 100.0).1, 0.5);
        assert_eq!(
            point_name(&auto, auto.anchor(run_at(&open, 100.0), 100.0)),
            "ponto_peca"
        );
        // A file without the automatic clip: the station keeps the cycle of `processing`.
        let mut bare = cnc.clone();
        bare.clipes.remove("ciclo_auto");
        let fallback = Animado::resolve(&bare, "cnc", &AT, true);
        let k = fallback.line(State::Processing).unwrap();
        assert_eq!(fallback.clips[fallback.estados[k].1.clip as usize], "ciclo");
    }

    #[test]
    fn people_are_dressed_the_same_every_run() {
        let c = cat();
        let ppe = vec![
            "Capacete".to_string(),
            "Óculos".to_string(),
            "protetor auricular".to_string(),
            "Respirador".to_string(),
            "luva".to_string(),
            "cinto".to_string(),
        ];
        let a = pessoa(&c, "ana", None, &ppe, Some("#3366CC"), None).unwrap();
        assert_eq!(
            a,
            pessoa(&c, "ana", None, &ppe, Some("#3366CC"), None).unwrap()
        );
        // Only the equipment of the catalogue, by its own names.
        assert_eq!(
            a.epis,
            [
                "capacete",
                "luvas",
                "mascara",
                "oculos",
                "protetor_auricular"
            ]
        );
        // Skin and hair from the suggested tones, the shirt from `visual.color`.
        let e = c.get(&a.arquivo).unwrap();
        let tones = |k: &str| strings(&e.dados[k]);
        let color = |m: &str| a.cores.iter().find(|c| c.0 == m).map(|c| c.1.clone());
        assert!(tones("peles").contains(&color("pele").unwrap()));
        assert!(tones("cabelos").contains(&color("cabelo").unwrap()));
        assert_eq!(color("camisa").as_deref(), Some("#3366cc"));
        assert_eq!(color("calca"), None);
        // Height in the range of the body, scale = height / altura_base.
        let range: Vec<f64> = e.dados["alturas"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        assert!(f64::from(a.altura) >= range[0] - 1e-6 && f64::from(a.altura) <= range[1] + 1e-6);
        assert!(
            (a.escala - a.altura / e.dados["altura_base"].as_f64().unwrap() as f32).abs() < 1e-6
        );
        assert!(a.path.is_file());
        // Without a colour the shirt keeps its own; the body can be forced.
        let b = pessoa(&c, "ana", Some("pessoa_mulher"), &[], None, None).unwrap();
        assert_eq!(b.arquivo, "pessoa_mulher");
        assert!(b.epis.is_empty() && !b.cores.iter().any(|c| c.0 == "camisa"));
        // Different names give both bodies and different heights.
        let names: Vec<String> = (0..20).map(|k| format!("operador_{k}")).collect();
        let bodies: Vec<String> = names
            .iter()
            .map(|n| pessoa(&c, n, None, &[], None, None).unwrap().arquivo)
            .collect();
        assert!(bodies.iter().any(|b| b == "pessoa_homem"));
        assert!(bodies.iter().any(|b| b == "pessoa_mulher"));
        let h0 = pessoa(&c, &names[0], None, &[], None, None).unwrap().altura;
        assert!(
            names
                .iter()
                .any(|n| pessoa(&c, n, None, &[], None, None).unwrap().altura != h0)
        );
    }

    #[test]
    fn epi_names_lose_case_accents_and_spaces_and_take_the_usual_aliases() {
        assert_eq!(epi_key(" Óculos "), "oculos");
        assert_eq!(epi_key("Protetor auricular"), "protetor_auricular");
        assert_eq!(epi_key("máscara"), "mascara");
        assert_eq!(epi_key("protetor-auricular"), "protetor_auricular");
        assert_eq!(epi_key("Respirador"), "mascara");
        assert_eq!(epi_key("luva"), "luvas");
        assert_eq!(epi_key("Botas"), "botina");
        assert_eq!(epi_key("bota"), "botina");
    }
}
