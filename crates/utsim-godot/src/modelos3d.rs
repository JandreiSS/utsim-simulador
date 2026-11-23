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

/// The line a door holds open between items (`Animado::door_runs`): not a state of the
/// simulation nor of the catalogue.
pub const DOOR_OPEN: &str = "porta_aberta";

/// The radio control of the crane (`controle_remoto`), held by its operator.
pub const RADIO: &str = "controle_remoto";

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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// Lowest corner of the element's box (model coordinates, Z up); with
    /// [`Elemento::tamanho`], the box the engine stretches to a size given in the model.
    #[serde(default)]
    pub minimo: [f32; 3],
    /// How far each part point of the root was moved to seat the item on the surface
    /// under it (Godot frame of the element: x, y up, -model y), by [`Elemento::assentar`].
    #[serde(skip)]
    pub ajustes: BTreeMap<String, [f32; 3]>,
    /// Whether [`Catalogo::assentar`] already ran for this element.
    #[serde(skip)]
    pub assentado: bool,
}

/// Every triangle of a binary glTF (`.glb`) in the file's own frame (Y up), each node's
/// transform applied, with the name of the node that holds it; a node a clip moves also
/// at every key of the clip (what it sweeps), its node name then ending in [`SWEPT`]. A small reader of what the
/// Blender elements use (float positions, 8/16/32-bit indices, triangle lists, the
/// default scene); `None` when the bytes are not a readable `.glb`.
pub fn glb_triangles(bytes: &[u8]) -> Option<Vec<(String, [[f32; 3]; 3])>> {
    let u32_at = |o: usize| -> Option<u32> {
        Some(u32::from_le_bytes(bytes.get(o..o + 4)?.try_into().ok()?))
    };
    if bytes.get(0..4)? != b"glTF" {
        return None;
    }
    let json_len = u32_at(12)? as usize;
    let json: serde_json::Value = serde_json::from_slice(bytes.get(20..20 + json_len)?).ok()?;
    let bin_at = 20 + json_len;
    let bin_len = u32_at(bin_at)? as usize;
    let bin = bytes.get(bin_at + 8..bin_at + 8 + bin_len)?;
    let num = |v: &serde_json::Value| v.as_u64().map(|n| n as usize);
    // An accessor's values as f32 tuples (`comps` each) or as indices.
    let read = |i: usize| -> Option<(Vec<f32>, usize)> {
        let a = &json["accessors"][i];
        let view = &json["bufferViews"][num(&a["bufferView"])?];
        let comps = match a["type"].as_str()? {
            "SCALAR" => 1,
            "VEC2" => 2,
            "VEC3" => 3,
            "VEC4" => 4,
            _ => return None,
        };
        let (size, kind) = match a["componentType"].as_u64()? {
            5126 => (4, 'f'),
            5125 => (4, 'I'),
            5123 => (2, 'H'),
            5121 => (1, 'B'),
            _ => return None,
        };
        let start = num(&view["byteOffset"]).unwrap_or(0) + num(&a["byteOffset"]).unwrap_or(0);
        let stride = num(&view["byteStride"]).unwrap_or(size * comps);
        let count = num(&a["count"])?;
        let mut out = Vec::with_capacity(count * comps);
        for k in 0..count {
            for c in 0..comps {
                let o = start + k * stride + c * size;
                let b = bin.get(o..o + size)?;
                out.push(match kind {
                    'f' => f32::from_le_bytes(b.try_into().ok()?),
                    'I' => u32::from_le_bytes(b.try_into().ok()?) as f32,
                    'H' => u16::from_le_bytes(b.try_into().ok()?) as f32,
                    _ => b[0] as f32,
                });
            }
        }
        Some((out, comps))
    };
    type M4 = [[f32; 4]; 4];
    // A node's own matrix; `pose` replaces its translation, rotation or scale (a key of a
    // clip).
    let local = |n: &serde_json::Value, pose: [Option<&[f32]>; 3]| -> M4 {
        if let Some(m) = n["matrix"].as_array().filter(|m| m.len() == 16) {
            let f = |k: usize| m[k].as_f64().unwrap_or(0.0) as f32;
            return [
                [f(0), f(1), f(2), f(3)],
                [f(4), f(5), f(6), f(7)],
                [f(8), f(9), f(10), f(11)],
                [f(12), f(13), f(14), f(15)],
            ];
        }
        let v = |key: &str, d: &[f32]| -> Vec<f32> {
            n[key].as_array().map_or_else(
                || d.to_vec(),
                |a| a.iter().map(|x| x.as_f64().unwrap_or(0.0) as f32).collect(),
            )
        };
        let t = pose[0].map_or_else(|| v("translation", &[0.0; 3]), <[f32]>::to_vec);
        let r = pose[1].map_or_else(|| v("rotation", &[0.0, 0.0, 0.0, 1.0]), <[f32]>::to_vec);
        let s = pose[2].map_or_else(|| v("scale", &[1.0; 3]), <[f32]>::to_vec);
        let (x, y, z, w) = (r[0], r[1], r[2], r[3]);
        // Column-major: m[column][row].
        [
            [
                (1.0 - 2.0 * (y * y + z * z)) * s[0],
                (2.0 * (x * y + z * w)) * s[0],
                (2.0 * (x * z - y * w)) * s[0],
                0.0,
            ],
            [
                (2.0 * (x * y - z * w)) * s[1],
                (1.0 - 2.0 * (x * x + z * z)) * s[1],
                (2.0 * (y * z + x * w)) * s[1],
                0.0,
            ],
            [
                (2.0 * (x * z + y * w)) * s[2],
                (2.0 * (y * z - x * w)) * s[2],
                (1.0 - 2.0 * (x * x + y * y)) * s[2],
                0.0,
            ],
            [t[0], t[1], t[2], 1.0],
        ]
    };
    let mul = |a: M4, b: M4| -> M4 {
        let mut r = [[0.0; 4]; 4];
        for (c, col) in r.iter_mut().enumerate() {
            for (row, v) in col.iter_mut().enumerate() {
                *v = (0..4).map(|k| a[k][row] * b[c][k]).sum();
            }
        }
        r
    };
    let apply = |m: M4, p: [f32; 3]| -> [f32; 3] {
        let mut r = [0.0; 3];
        for (row, v) in r.iter_mut().enumerate() {
            *v = m[0][row] * p[0] + m[1][row] * p[1] + m[2][row] * p[2] + m[3][row];
        }
        r
    };
    let identity: M4 = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let scene = num(&json["scene"]).unwrap_or(0);
    let nodes = json["nodes"].as_array()?;
    let mut out = Vec::new();
    // The triangles of node `k`'s mesh under the world matrix `m`.
    let emit = |k: usize, m: M4, out: &mut Vec<(String, [[f32; 3]; 3])>| -> Option<()> {
        let node = &nodes[k];
        let name = node["name"].as_str().unwrap_or("").to_string();
        let Some(mesh) = num(&node["mesh"]) else {
            return Some(());
        };
        for p in json["meshes"][mesh]["primitives"].as_array()? {
            if num(&p["mode"]).is_some_and(|mode| mode != 4) {
                continue;
            }
            let Some((pos, 3)) = num(&p["attributes"]["POSITION"]).and_then(read) else {
                continue;
            };
            let pos: Vec<[f32; 3]> = (0..pos.len() / 3)
                .map(|k| apply(m, [pos[3 * k], pos[3 * k + 1], pos[3 * k + 2]]))
                .collect();
            let idx: Vec<usize> = match num(&p["indices"]).and_then(read) {
                Some((i, 1)) => i.into_iter().map(|v| v as usize).collect(),
                _ => (0..pos.len()).collect(),
            };
            for t in (0..idx.len() / 3).map(|k| &idx[3 * k..3 * k + 3]) {
                if let (Some(a), Some(b), Some(c)) = (pos.get(t[0]), pos.get(t[1]), pos.get(t[2])) {
                    out.push((name.clone(), [*a, *b, *c]));
                }
            }
        }
        Some(())
    };
    // The subtree of node `k` whose own world matrix is `m` (its children at rest).
    let subtree =
        |k: usize, m: M4, parents: &mut Vec<M4>, out: &mut Vec<(String, [[f32; 3]; 3])>| {
            let mut stack = vec![(k, m)];
            while let Some((k, m)) = stack.pop() {
                let _ = emit(k, m, out);
                if let Some(children) = nodes[k]["children"].as_array() {
                    for c in children.iter().filter_map(num) {
                        if let Some(slot) = parents.get_mut(c) {
                            *slot = m;
                        }
                        stack.push((c, mul(m, local(&nodes[c], [None; 3]))));
                    }
                }
            }
        };
    // At rest, keeping each node's parent matrix for the poses below.
    let mut parents = vec![identity; nodes.len()];
    for k in json["scenes"][scene]["nodes"]
        .as_array()?
        .iter()
        .filter_map(num)
    {
        subtree(k, local(&nodes[k], [None; 3]), &mut parents, &mut out);
    }
    // Every key of every clip: a moving node (a clamp, a door) at each of its poses, so
    // what it sweeps counts too.
    let rest_parents = parents.clone();
    for anim in json["animations"].as_array().into_iter().flatten() {
        // Per node, the keys of its translation, rotation and scale (values, width).
        type Keys = [Option<(Vec<f32>, usize)>; 3];
        let mut keyed: BTreeMap<usize, Keys> = BTreeMap::new();
        for ch in anim["channels"].as_array().into_iter().flatten() {
            let (Some(k), Some(path)) = (num(&ch["target"]["node"]), ch["target"]["path"].as_str())
            else {
                continue;
            };
            let slot = match path {
                "translation" => 0,
                "rotation" => 1,
                "scale" => 2,
                _ => continue,
            };
            let sampler = &anim["samplers"][num(&ch["sampler"]).unwrap_or(usize::MAX)];
            if let Some(values) = num(&sampler["output"]).and_then(read) {
                keyed.entry(k).or_default()[slot] = Some(values);
            }
        }
        for (k, paths) in keyed {
            if k >= nodes.len() {
                continue;
            }
            let keys = paths
                .iter()
                .flatten()
                .map(|(v, c)| v.len() / (*c).max(1))
                .max()
                .unwrap_or(0);
            for j in 0..keys {
                let at = |slot: usize| -> Option<&[f32]> {
                    let (v, c) = paths[slot].as_ref()?;
                    v.get(j * c..j * c + c)
                };
                let m = mul(rest_parents[k], local(&nodes[k], [at(0), at(1), at(2)]));
                let from = out.len();
                subtree(k, m, &mut parents, &mut out);
                for (name, _) in &mut out[from..] {
                    name.push(SWEPT);
                }
            }
        }
    }
    Some(out)
}

/// The mark at the end of the node name of a triangle a clip moves there ([`glb_triangles`]):
/// it counts as an obstacle, never as a surface to stand on.
pub const SWEPT: char = '~';

/// Triangles of an element with the node that holds each ([`glb_triangles`]).
type Tris = [(String, [[f32; 3]; 3])];

/// Height of the surface under `(x, z)` (Godot frame) among `tris`: the highest triangle
/// over that point whose height there is at most `top` (not a pose of a clip, [`SWEPT`]).
fn surface(tris: &Tris, x: f32, z: f32, top: f32) -> Option<f32> {
    let mut best: Option<f32> = None;
    for (_, [a, b, c]) in tris.iter().filter(|(n, _)| !n.ends_with(SWEPT)) {
        // Barycentric coordinates of (x, z) in the triangle's floor projection.
        let d = (b[2] - c[2]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[2] - c[2]);
        if d.abs() < 1e-9 {
            continue;
        }
        let u = ((b[2] - c[2]) * (x - c[0]) + (c[0] - b[0]) * (z - c[2])) / d;
        let v = ((c[2] - a[2]) * (x - c[0]) + (a[0] - c[0]) * (z - c[2])) / d;
        let w = 1.0 - u - v;
        if u < -1e-5 || v < -1e-5 || w < -1e-5 {
            continue;
        }
        let y = u * a[1] + v * b[1] + w * c[1];
        if y <= top && best.is_none_or(|h| y > h) {
            best = Some(y);
        }
    }
    best
}

/// Whether a cube of `edge` standing at `bottom` centred on `(x, z)` (Godot frame) cuts a
/// triangle of `tris` (by the triangles' boxes, a hair above the bottom).
fn cuts(tris: &Tris, x: f32, z: f32, bottom: f32, edge: f32) -> bool {
    let h = edge / 2.0;
    let lo = [x - h, bottom + 0.003, z - h];
    let hi = [x + h, bottom + edge, z + h];
    tris.iter().any(|(_, t)| {
        (0..3).all(|k| {
            let a = t[0][k].min(t[1][k]).min(t[2][k]);
            let b = t[0][k].max(t[1][k]).max(t[2][k]);
            a < hi[k] && b > lo[k]
        })
    })
}

/// How an element is fitted to a size given in the model: its stretch along its own axes
/// (Godot) and the offset that stands the stretched box on the floor at the root's centre.
/// The engine applies both (`cena.gd`), and the points of the element follow them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Encaixe {
    pub stretch: [f32; 3],
    pub offset: [f32; 3],
}

/// Box `(min, max)` of a box `(min, max)` in the asset's own frame once scaled by `k`
/// (per axis), turned by `yaw` about `+Y` (quarter turns: the box stays a box) and moved
/// by `offset`.
fn placed_box(
    lo: [f32; 3],
    hi: [f32; 3],
    k: [f32; 3],
    yaw: f32,
    offset: [f32; 3],
) -> ([f32; 3], [f32; 3]) {
    let mut a = [f32::INFINITY; 3];
    let mut b = [f32::NEG_INFINITY; 3];
    for c in 0..8 {
        let p = [
            if c & 1 == 0 { lo[0] } else { hi[0] } * k[0],
            if c & 2 == 0 { lo[1] } else { hi[1] } * k[1],
            if c & 4 == 0 { lo[2] } else { hi[2] } * k[2],
        ];
        let t = turned(p, yaw);
        for j in 0..3 {
            a[j] = a[j].min(t[j] + offset[j]);
            b[j] = b[j].max(t[j] + offset[j]);
        }
    }
    (a, b)
}

impl Elemento {
    /// Seats the item on each part point of the root (the points of the `peca` table that
    /// do not move with a clip) on the surface under it, an item of `edge` (element units):
    /// down or up to the highest surface under the point (a table top), then, when the
    /// item would cut the element (a clamp on the table), moved the least along that
    /// surface until it does not. A point with no surface within 6 cm under it is left as it
    /// is. `tris` are the element's triangles.
    pub fn assentar(&mut self, stem: &str, tris: &Tris, edge: f32) {
        let mut names: Vec<String> = self
            .peca
            .values()
            .flatten()
            .map(|(_, _, n)| n.clone())
            .collect();
        names.sort();
        names.dedup();
        use std::f32::consts::FRAC_1_SQRT_2 as D;
        const DIRS: [[f32; 2]; 8] = [
            [1.0, 0.0],
            [-1.0, 0.0],
            [0.0, 1.0],
            [0.0, -1.0],
            [D, D],
            [-D, D],
            [D, -D],
            [-D, -D],
        ];
        for name in names {
            let Some(p) = self.fixed_point(stem, &name).cloned() else {
                continue;
            };
            let g = to_godot(p.pos);
            // Only a surface within a few centimetres: a point over a bin or a chute is
            // where the item is meant to be.
            let Some(bottom) = surface(tris, g[0], g[2], g[1] + 0.06).filter(|y| *y >= g[1] - 0.06)
            else {
                continue;
            };
            let mut at = [g[0], g[2]];
            if cuts(tris, at[0], at[1], bottom, edge) {
                let found = (1..=40).find_map(|r| {
                    let r = r as f32 * 0.01;
                    DIRS.iter().find_map(|d| {
                        let (x, z) = (g[0] + d[0] * r, g[2] + d[1] * r);
                        let same = surface(tris, x, z, g[1] + 0.06)
                            .is_some_and(|s| (s - bottom).abs() < 0.01);
                        (same && !cuts(tris, x, z, bottom, edge)).then_some([x, z])
                    })
                });
                if let Some(f) = found {
                    at = f;
                }
            }
            let adj = [at[0] - g[0], bottom - g[1], at[1] - g[2]];
            if adj.iter().all(|v| v.abs() < 1e-5) {
                continue;
            }
            if let Some(q) = self.pontos.get_mut(&name) {
                // Godot (x, y, z) is model (x, -z, y).
                q.pos = [q.pos[0] + adj[0], q.pos[1] - adj[2], q.pos[2] + adj[1]];
            }
            self.ajustes.insert(name, adj);
        }
    }

    /// The element fitted to `size` (Godot extents in the root's frame): scaled by `scale`,
    /// turned by `yaw` and moved by `offset` under the root, then stretched along each axis
    /// to `size` and stood on the floor at the root's centre. `None` without a box.
    pub fn encaixe(
        &self,
        scale: f32,
        yaw: f32,
        offset: [f32; 3],
        size: [f32; 3],
    ) -> Option<Encaixe> {
        let (m, t) = (self.minimo, self.tamanho);
        if t.iter().any(|v| !(v.is_finite() && *v > 1e-4))
            || size.iter().any(|v| !(v.is_finite() && *v > 1e-4))
        {
            return None;
        }
        // The box in the asset's frame (Godot): model x, z, -y.
        let lo = [m[0], m[2], -(m[1] + t[1])];
        let hi = [m[0] + t[0], m[2] + t[2], -m[1]];
        let (a, b) = placed_box(lo, hi, [scale; 3], yaw, offset);
        let f = [
            size[0] / (b[0] - a[0]),
            size[1] / (b[1] - a[1]),
            size[2] / (b[2] - a[2]),
        ];
        let sideways = yaw.sin().abs() > 0.7;
        let stretch = if sideways { [f[2], f[1], f[0]] } else { f };
        let k = [scale * stretch[0], scale * stretch[1], scale * stretch[2]];
        let (a, b) = placed_box(lo, hi, k, yaw, offset);
        Some(Encaixe {
            stretch,
            offset: [
                offset[0] - (a[0] + b[0]) / 2.0,
                offset[1] - a[1],
                offset[2] - (a[2] + b[2]) / 2.0,
            ],
        })
    }

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

    /// Seats the part points of element `stem` on its surfaces ([`Elemento::assentar`]),
    /// once, from the triangles of its file.
    pub fn assentar(&mut self, stem: &str) {
        let dir = self.dir.clone();
        let Some(e) = self.elementos.get_mut(stem) else {
            return;
        };
        if e.assentado {
            return;
        }
        e.assentado = true;
        if e.peca.is_empty() {
            return;
        }
        let Some(tris) = std::fs::read(dir.join(&e.arquivo))
            .ok()
            .and_then(|b| glb_triangles(&b))
        else {
            return;
        };
        e.assentar(stem, &tris, utsim_scene::compose::ITEM_SIZE);
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

/// A Blender element drawn as the items of a source (`visual.item`): its file, and the
/// scale and offset that fit it in the item's cube ([`utsim_scene::compose::ITEM_SIZE`]),
/// its longest side the cube's edge, standing on the cube's bottom at its centre (Godot,
/// under the item's node, whose origin is the cube's centre).
#[derive(Clone, Debug, PartialEq)]
pub struct ModeloPeca {
    pub stem: String,
    pub path: String,
    pub scale: f32,
    pub offset: [f32; 3],
}

impl ModeloPeca {
    /// The element `stem` of the catalogue fitted to the item's cube; `None` when the
    /// catalogue has no such element or no box for it.
    pub fn of(cat: &Catalogo, stem: &str) -> Option<ModeloPeca> {
        let e = cat.get(stem)?;
        let (m, t) = (e.minimo, e.tamanho);
        let big = t.iter().copied().fold(0.0f32, f32::max);
        if !(big.is_finite() && big > 1e-4) {
            return None;
        }
        let edge = utsim_scene::compose::ITEM_SIZE;
        let k = edge / big;
        // The box's bottom centre (model x, y, z) to the cube's bottom centre.
        let (cx, cy) = (m[0] + t[0] / 2.0, m[1] + t[1] / 2.0);
        Some(ModeloPeca {
            stem: stem.to_string(),
            path: cat.path(stem).to_string_lossy().replace('\\', "/"),
            scale: k,
            offset: [-cx * k, -m[2] * k - edge / 2.0, cy * k],
        })
    }
}

/// Where an asset is placed under its block's root, and the root itself (Godot, Y up).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placement {
    /// Uniform scale of the asset.
    pub scale: f32,
    /// Stretch of the asset along its own axes (Godot, after `scale`): `[1, 1, 1]` but for
    /// an element given a size in the model ([`Elemento::encaixe`]).
    pub stretch: [f32; 3],
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
    let g = to_godot(p.pos);
    let g = [
        g[0] * at.scale * at.stretch[0],
        g[1] * at.scale * at.stretch[1],
        g[2] * at.scale * at.stretch[2],
    ];
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

    /// The runs of a door that opens for the items passing through it (the `doca` at a
    /// source or a sink: its `entering` and `leaving` lines), from the times the items
    /// pass, in order. The door opens in the `d` before an item, stays open while items
    /// keep coming (gaps under `3d`) and closes after the last; `d` is a quarter of the
    /// usual gap, at most 1. `None` when the element has no such lines.
    pub fn door_runs(&mut self, times: &[f64]) -> Option<Runs> {
        let line = |s: &Self, n: &str| s.estados.iter().position(|(k, _)| k == n);
        let (enter, leave) = (line(self, "entering")?, line(self, "leaving")?);
        let hold = match line(self, DOOR_OPEN) {
            Some(k) => k,
            None => {
                let p = &self.estados[enter].1;
                let open = Passo {
                    de: p.ate,
                    ate: p.ate,
                    laco: false,
                    caminho: None,
                    ..p.clone()
                };
                self.estados.push((DOOR_OPEN.to_string(), open));
                self.estados.len() - 1
            }
        };
        let mut gaps: Vec<f64> = times
            .windows(2)
            .map(|w| w[1] - w[0])
            .filter(|g| *g > 1e-9)
            .collect();
        gaps.sort_by(f64::total_cmp);
        let d = gaps.get(gaps.len() / 2).map_or(1.0, |m| (m / 4.0).min(1.0));
        let mut out: Runs = Vec::new();
        let mut k = 0;
        while k < times.len() {
            let start = times[k];
            let mut end = start;
            k += 1;
            while k < times.len() && times[k] - end < 3.0 * d {
                end = times[k];
                k += 1;
            }
            out.push((start - d, Some(enter), None));
            out.push((start, Some(hold), None));
            out.push((end + d / 2.0, Some(leave), None));
            out.push((end + 1.5 * d, None, None));
        }
        Some(out)
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

/// The top of a waiting table (`espera`: `dados.tampo` and `dados.area`): where a queue
/// drawn with it lays its resting items, in a grid over the free rectangle of the top, the
/// oldest at the exit (`+x`, the `ponto_saida` side), then a second layer on top; what does
/// not fit in [`utsim_scene::style::MAX_LAYERS`] layers is only counted. Element units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tampo {
    /// Free rectangle of the top, along `x` and across (`y`), centred on the origin.
    pub area: [f32; 2],
    /// Height of the top.
    pub altura: f32,
}

/// Gap between two items on a table top, as a fraction of the item size.
const TAMPO_FOLGA: f32 = 0.1;

impl Tampo {
    /// The top of element `e`, when its `dados` say where it is.
    pub fn of(e: &Elemento) -> Option<Tampo> {
        let area = e.dados.get("area")?.as_array()?;
        let x = area.first()?.as_f64()? as f32;
        let y = area.get(1)?.as_f64()? as f32;
        let altura = e.dados.get("tampo")?.as_f64()? as f32;
        (x > 0.0 && y > 0.0 && altura.is_finite()).then_some(Tampo {
            area: [x, y],
            altura,
        })
    }

    /// The top of the element placed by `at` (its scale and stretch), in metres, with the
    /// placement that then only turns and moves it: the grid of items is laid in metres,
    /// so a stretched table holds more items, not wider ones.
    pub fn placed(&self, at: &Placement) -> (Tampo, Placement) {
        let k = |j: usize| at.scale * at.stretch[j];
        (
            Tampo {
                // Model x is Godot x, model y is Godot -z, model z is Godot y.
                area: [self.area[0] * k(0), self.area[1] * k(2)],
                altura: self.altura * k(1),
            },
            Placement {
                scale: 1.0,
                stretch: [1.0; 3],
                ..*at
            },
        )
    }

    /// Columns (along `x`) and rows (across) of items of `size`.
    pub fn grade(&self, size: f32) -> (usize, usize) {
        let pitch = size.max(1e-3) * (1.0 + TAMPO_FOLGA);
        let n = |l: f32| ((l / pitch).floor() as usize).max(1);
        (n(self.area[0]), n(self.area[1]))
    }

    /// Items of `size` in one layer.
    pub fn per_layer(&self, size: f32) -> usize {
        let (c, r) = self.grade(size);
        c * r
    }

    /// The point under resting item `i` (0: the oldest) of `size`: the centre of its cell
    /// on the top of its layer. `None` past the layers drawn.
    pub fn slot(&self, i: usize, size: f32) -> Option<Ponto> {
        let (c, r) = self.grade(size);
        let layer = i / (c * r);
        if layer >= utsim_scene::style::MAX_LAYERS {
            return None;
        }
        let k = i % (c * r);
        let (col, row) = (k / r, k % r);
        let x = self.area[0] / 2.0 - self.area[0] / c as f32 * (col as f32 + 0.5);
        let y = -self.area[1] / 2.0 + self.area[1] / r as f32 * (row as f32 + 0.5);
        Some(Ponto {
            pos: [x, y, self.altura + layer as f32 * size],
            ..Ponto::default()
        })
    }

    /// Resting items not drawn.
    pub fn hidden(&self, resting: usize, size: f32) -> usize {
        resting.saturating_sub(self.per_layer(size) * utsim_scene::style::MAX_LAYERS)
    }
}

/// The corrected clips of the person bodies (`pessoa_clipes`: only the skeleton and the
/// clips to swap, `dados.substitui`): its file and the clip names it replaces in the
/// clips the Blender bodies play (UAL2: `Walk_Carry_Loop`). `None` when the catalogue
/// has no such entry.
pub fn clipes_corrigidos(cat: &Catalogo) -> Option<(PathBuf, Vec<String>)> {
    let e = cat.get("pessoa_clipes")?;
    let names: Vec<String> = e.dados["substitui"]
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    (!names.is_empty()).then(|| (cat.path("pessoa_clipes"), names))
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
        stretch: [1.0; 3],
        asset_yaw: 0.0,
        offset: [0.0; 3],
        root: [0.0; 3],
        turn: 0.0,
    };

    #[test]
    fn the_part_of_the_workstation_sits_on_its_top_clear_of_the_clamp() {
        let Some(mut c) = Catalogo::find(None) else {
            return;
        };
        if c.get("posto").is_none() {
            return;
        }
        let before = c.get("posto").unwrap().pontos["ponto_peca"].pos;
        c.assentar("posto");
        let e = c.get("posto").unwrap();
        let adj = e.ajustes["ponto_peca"];
        let tris = glb_triangles(&std::fs::read(c.path("posto")).unwrap()).unwrap();
        let g = to_godot(e.pontos["ponto_peca"].pos);
        let edge = utsim_scene::compose::ITEM_SIZE;
        // On the jig plate of the bench (0,94 m, the surface under the point), and the box
        // cuts nothing (the clamp beside it).
        assert!((g[1] - 0.94).abs() < 0.005, "{g:?} {adj:?}");
        assert!(!cuts(&tris, g[0], g[2], g[1], edge), "{g:?}");
        let top = surface(&tris, g[0], g[2], g[1] + 0.01).unwrap();
        assert!((top - g[1]).abs() < 1e-4);
        // Moved the least along the top, clear of where the clamp swings (the `ciclo` clip).
        let moved =
            ((g[0] - to_godot(before)[0]).powi(2) + (g[2] - to_godot(before)[2]).powi(2)).sqrt();
        assert!(moved > 0.0 && moved < 0.3, "{moved}");
        let seated = e.pontos["ponto_peca"].clone();
        // Every element: what was moved, and by how much (each by less than 0,4 m).
        for stem in c.elementos.keys().cloned().collect::<Vec<_>>() {
            c.assentar(&stem);
            for (n, a) in &c.get(&stem).unwrap().ajustes {
                eprintln!("assentado: {stem} {n} {a:?}");
                assert!(a.iter().all(|v| v.abs() < 0.41), "{stem} {n} {a:?}");
            }
        }
        // The point as it was cut the clamp.
        let b = to_godot(before);
        assert!(cuts(&tris, b[0], b[2], 0.94, edge));
        // Once only.
        c.assentar("posto");
        assert_eq!(c.get("posto").unwrap().pontos["ponto_peca"], seated);
    }

    #[test]
    fn a_resized_element_stretches_and_its_points_follow() {
        // A 1 m × 0,5 m × 1 m table (model x, y, z), its top point at the middle.
        let e = Elemento {
            minimo: [-0.5, -0.25, 0.0],
            tamanho: [1.0, 0.5, 1.0],
            ..Elemento::default()
        };
        // Given 2 m long, 1 m wide and 1 m tall (Godot x, y, z): twice along x and across.
        let fit = e.encaixe(1.0, 0.0, [0.0; 3], [2.0, 1.0, 1.0]).unwrap();
        assert!(close(fit.stretch, [2.0, 1.0, 2.0]), "{fit:?}");
        assert!(close(fit.offset, [0.0; 3]), "{fit:?}");
        let at = Placement {
            stretch: fit.stretch,
            offset: fit.offset,
            ..AT
        };
        let corner = Ponto {
            pos: [0.5, 0.25, 1.0],
            ..Ponto::default()
        };
        let (pos, _) = point_to_layout(&corner, &at);
        assert!(close(pos, [1.0, 1.0, -0.5]), "{pos:?}");
        // Turned a quarter under the root: the axes swap, the box still fills the size.
        let fit = e
            .encaixe(1.0, FRAC_PI_2, [0.0; 3], [2.0, 1.0, 1.0])
            .unwrap();
        assert!(close(fit.stretch, [1.0, 1.0, 4.0]), "{fit:?}");
        // Off the floor and off centre: stood on the floor at the root's centre.
        let e = Elemento {
            minimo: [0.0, 0.0, 0.2],
            tamanho: [1.0, 1.0, 1.0],
            ..Elemento::default()
        };
        let fit = e.encaixe(1.0, 0.0, [0.0; 3], [1.0, 2.0, 1.0]).unwrap();
        assert!(close(fit.stretch, [1.0, 2.0, 1.0]), "{fit:?}");
        assert!(close(fit.offset, [-0.5, -0.4, 0.5]), "{fit:?}");
        // No box: no fit.
        assert!(
            Elemento::default()
                .encaixe(1.0, 0.0, [0.0; 3], [1.0; 3])
                .is_none()
        );
    }

    #[test]
    fn the_corrected_clips_name_the_file_and_what_they_replace() {
        let (path, names) = clipes_corrigidos(&cat()).expect("pessoa_clipes no catálogo");
        assert!(path.is_file(), "{}", path.display());
        assert!(names.contains(&"Walk_Carry_Loop".to_string()), "{names:?}");
    }

    #[test]
    fn the_waiting_table_lays_its_items_on_the_top() {
        let c = cat();
        let e = c.get("espera").expect("espera no catálogo");
        let t = Tampo::of(e).unwrap();
        assert!(
            (t.altura - 0.76).abs() < 1e-6 && t.area == [1.1, 0.66],
            "{t:?}"
        );
        // Items of 0.3 m: 3 along, 2 across, two layers.
        assert_eq!(t.grade(0.3), (3, 2));
        let first = t.slot(0, 0.3).unwrap();
        let last = t.slot(5, 0.3).unwrap();
        // The oldest at the exit (+x), all inside the free rectangle, on the top.
        assert!(
            first.pos[0] > 0.3 && last.pos[0] < -0.3,
            "{first:?} {last:?}"
        );
        for i in 0..12 {
            let p = t.slot(i, 0.3).unwrap().pos;
            assert!(
                p[0].abs() + 0.15 <= 0.55 + 1e-4 && p[1].abs() + 0.15 <= 0.33 + 1e-4,
                "{i}: {p:?}"
            );
            assert!(
                (p[2] - if i < 6 { 0.76 } else { 1.06 }).abs() < 1e-5,
                "{i}: {p:?}"
            );
        }
        assert!(t.slot(12, 0.3).is_none());
        assert_eq!((t.hidden(12, 0.3), t.hidden(20, 0.3)), (0, 8));
        // The other elements have no top.
        assert!(Tampo::of(c.get("cnc").unwrap()).is_none());
        // `visual.asset: "espera"` asks for it.
        assert_eq!(
            pedido(&c, Some("espera")),
            Some(Pedido::Elemento("espera".into()))
        );
    }

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
    fn the_dock_door_opens_before_each_item_and_stays_open_for_a_burst() {
        let passo = |de: f32, ate: f32| Passo {
            clip: 0,
            de,
            ate,
            laco: false,
            caminho: None,
        };
        let mut a = Animado {
            clips: vec!["porta_abrir".into()],
            estados: vec![
                ("entering".into(), passo(0.0, 1.0)),
                ("leaving".into(), passo(1.0, 0.0)),
            ],
            ..Animado::default()
        };
        // Two items almost together, then one every 10: the door opens one unit before.
        let runs = a.door_runs(&[10.0, 10.05, 20.0, 30.0]).unwrap();
        let at = |t: f64| a.anim(run_at(&runs, t), t);
        assert_eq!(at(5.0), (-1, 0.0), "fechada antes");
        assert!((at(9.5).1 - 0.5).abs() < 1e-4, "{:?}", at(9.5));
        assert_eq!(at(10.03), (0, 1.0), "aberta entre as duas peças");
        assert_eq!(at(10.3), (0, 1.0));
        assert_eq!(at(15.0), (-1, 0.0), "fechada entre as chegadas");
        assert_eq!(at(20.0), (0, 1.0));
        // Without the lines (a machine), no door.
        assert!(Animado::default().door_runs(&[1.0]).is_none());
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
        assert!(close(post.0, [0.85, 0.0, 1.4]));
        let door = ([0.85, 0.0, 2.0], FRAC_PI_2);
        let at = |s: State, t: f64| {
            let runs = a.runs([(0.0, s), (4.0, State::Idle)]);
            a.operator(run_at(&runs, t), t).unwrap()
        };
        assert!(close(at(State::Entering, 0.0).0, door.0));
        assert!(close(at(State::Entering, 2.0).0, [0.85, 0.0, 1.7]));
        assert!(close(at(State::Entering, 3.9999).0, post.0));
        assert!(close(at(State::Leaving, 0.0).0, post.0));
        assert!(close(at(State::Leaving, 2.0).0, [0.85, 0.0, 1.7]));
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
