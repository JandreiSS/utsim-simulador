//! The interaction of Editar in the 3D view (step 6 of `docs/plano_interface_rust.md`):
//! picking by the camera ray, the input, output and service nodes, flow and team links,
//! selection, moving and the selection box. Pure: the bridge (`UtsimEditor`) feeds it the
//! camera, the scene and the mouse, and applies the [`Effect`]s to the document; the
//! `editor.gd` draws what [`Editor3d::overlay`] describes.
//!
//! Coordinates are Godot's world (metres, `y` up) and the viewport's pixels.

use std::collections::HashMap;

pub type V2 = [f64; 2];
pub type V3 = [f64; 3];

/// Pixels around a node or a line that still pick it.
pub const PORT_PIXELS: f64 = 14.0;
pub const PICK_PIXELS: f64 = 8.0;
/// Pixels around a block's centre that pick it when its box is small on screen.
pub const PICK_CENTRE: f64 = 18.0;
/// Pixels the mouse moves before a press becomes a drag.
pub const DRAG_PIXELS: f64 = 4.0;

/// Kinds without an output node, without an input node, and the ones with a service node.
pub const NO_OUTPUT: [&str; 3] = ["Sink", "Transporter", "Dispatcher"];
pub const NO_INPUT: [&str; 3] = ["Source", "Transporter", "Dispatcher"];
pub const SERVICE: [&str; 2] = ["Transporter", "Dispatcher"];

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scale(a: V3, k: f64) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}
fn len(a: V3) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
fn norm(a: V3) -> V3 {
    let l = len(a);
    if l > 1e-12 { scale(a, 1.0 / l) } else { a }
}
fn lerp(a: V3, b: V3, t: f64) -> V3 {
    add(a, scale(sub(b, a), t))
}
fn dist2(a: V2, b: V2) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

/// Distance from `p` to the segment `a`–`b` (screen).
pub fn segment_distance(p: V2, a: V2, b: V2) -> f64 {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let l2 = ab[0] * ab[0] + ab[1] * ab[1];
    if l2 < 1e-6 {
        return dist2(p, a);
    }
    let t = (((p[0] - a[0]) * ab[0] + (p[1] - a[1]) * ab[1]) / l2).clamp(0.0, 1.0);
    dist2(p, [a[0] + ab[0] * t, a[1] + ab[1] * t])
}

/// A 4×4 matrix, rows first.
pub type M4 = [[f64; 4]; 4];

fn mul(a: &M4, b: &M4) -> M4 {
    let mut r = [[0.0; 4]; 4];
    for (i, row) in r.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = (0..4).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    r
}

fn apply(m: &M4, v: [f64; 4]) -> [f64; 4] {
    let mut r = [0.0; 4];
    for (i, x) in r.iter_mut().enumerate() {
        *x = (0..4).map(|k| m[i][k] * v[k]).sum();
    }
    r
}

/// The inverse of `m` (Gauss-Jordan), `None` when it is singular.
pub fn inverse(m: &M4) -> Option<M4> {
    let mut a = *m;
    let mut inv = [[0.0; 4]; 4];
    for (i, row) in inv.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    for c in 0..4 {
        let p = (c..4).max_by(|&x, &y| a[x][c].abs().total_cmp(&a[y][c].abs()))?;
        if a[p][c].abs() < 1e-12 {
            return None;
        }
        a.swap(c, p);
        inv.swap(c, p);
        let d = a[c][c];
        for k in 0..4 {
            a[c][k] /= d;
            inv[c][k] /= d;
        }
        for r in 0..4 {
            if r != c {
                let f = a[r][c];
                for k in 0..4 {
                    a[r][k] -= f * a[c][k];
                    inv[r][k] -= f * inv[c][k];
                }
            }
        }
    }
    Some(inv)
}

/// The camera as Godot's `Camera3D` draws: the world-to-camera transform, the projection
/// and the viewport size. Godot cameras look down their `-z`.
#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    /// World → camera.
    pub view: M4,
    pub proj: M4,
    pub size: V2,
}

impl Default for Camera {
    fn default() -> Self {
        Camera::look_at([0.0, 10.0, 10.0], [0.0, 0.0, 0.0], 75.0, [1600.0, 900.0])
    }
}

impl Camera {
    /// From Godot's column-major matrices: the camera transform's inverse (`view`) and
    /// `get_camera_projection()` (`proj`), 16 numbers each.
    pub fn from_columns(view: &[f64], proj: &[f64], size: V2) -> Camera {
        let m = |c: &[f64]| {
            let mut r = [[0.0; 4]; 4];
            if c.len() >= 16 {
                for col in 0..4 {
                    for row in 0..4 {
                        r[row][col] = c[col * 4 + row];
                    }
                }
            }
            r
        };
        Camera {
            view: m(view),
            proj: m(proj),
            size,
        }
    }

    /// A perspective camera at `eye` looking at `target` (vertical field of view in
    /// degrees), as Godot builds it: the tests' and a fallback.
    pub fn look_at(eye: V3, target: V3, fov_deg: f64, size: V2) -> Camera {
        let f = norm(sub(target, eye)); // forward
        let up = [0.0, 1.0, 0.0];
        let mut r = [
            f[1] * up[2] - f[2] * up[1],
            f[2] * up[0] - f[0] * up[2],
            f[0] * up[1] - f[1] * up[0],
        ];
        if len(r) < 1e-9 {
            r = [1.0, 0.0, 0.0];
        }
        let r = norm(r);
        let u = [
            r[1] * f[2] - r[2] * f[1],
            r[2] * f[0] - r[0] * f[2],
            r[0] * f[1] - r[1] * f[0],
        ];
        // Camera axes: x = r, y = u, z = -f.
        let z = scale(f, -1.0);
        let dot = |a: V3, b: V3| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let view = [
            [r[0], r[1], r[2], -dot(r, eye)],
            [u[0], u[1], u[2], -dot(u, eye)],
            [z[0], z[1], z[2], -dot(z, eye)],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let (near, far) = (0.05, 4000.0);
        let t = 1.0 / (fov_deg.to_radians() / 2.0).tan();
        let aspect = size[0] / size[1];
        let proj = [
            [t / aspect, 0.0, 0.0, 0.0],
            [0.0, t, 0.0, 0.0],
            [
                0.0,
                0.0,
                (far + near) / (near - far),
                2.0 * far * near / (near - far),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ];
        Camera { view, proj, size }
    }

    /// Where `w` is drawn on screen, `None` behind the camera.
    pub fn project(&self, w: V3) -> Option<V2> {
        let c = apply(&self.view, [w[0], w[1], w[2], 1.0]);
        if c[2] >= -1e-6 {
            return None;
        }
        let clip = apply(&self.proj, c);
        if clip[3].abs() < 1e-12 {
            return None;
        }
        let (x, y) = (clip[0] / clip[3], clip[1] / clip[3]);
        Some([
            (x + 1.0) * 0.5 * self.size[0],
            (1.0 - y) * 0.5 * self.size[1],
        ])
    }

    /// The ray under screen point `p`: origin (the camera) and unit direction.
    pub fn ray(&self, p: V2) -> (V3, V3) {
        let inv_view = inverse(&self.view).unwrap_or(IDENTITY);
        let origin = [inv_view[0][3], inv_view[1][3], inv_view[2][3]];
        let Some(inv) = inverse(&mul(&self.proj, &self.view)) else {
            return (origin, [0.0, -1.0, 0.0]);
        };
        let x = p[0] / self.size[0] * 2.0 - 1.0;
        let y = 1.0 - p[1] / self.size[1] * 2.0;
        let w = apply(&inv, [x, y, 0.5, 1.0]);
        let at = [w[0] / w[3], w[1] / w[3], w[2] / w[3]];
        (origin, norm(sub(at, origin)))
    }

    /// The floor point (`y = 0`) under `p`, `None` when the ray misses it.
    pub fn floor_at(&self, p: V2) -> Option<V3> {
        let (o, d) = self.ray(p);
        if d[1].abs() < 1e-6 {
            return None;
        }
        let t = -o[1] / d[1];
        (t >= 0.0).then(|| add(o, scale(d, t)))
    }

    /// `w` is within `pixels` of `p` on screen.
    pub fn near(&self, w: V3, p: V2, pixels: f64) -> bool {
        self.project(w).is_some_and(|s| dist2(s, p) <= pixels)
    }
}

const IDENTITY: M4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

/// A block of the scene, as the editor sees it.
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub name: String,
    pub kind: String,
    /// Where it is drawn (its root, on the floor).
    pub pos: V3,
    /// Its size (x, height, z).
    pub size: V3,
    /// Drawn as a person (the pick box is a person's height).
    pub skeletal: bool,
    /// Has problems (red corners).
    pub problems: bool,
}

impl Block {
    /// The box it is picked by: its footprint (at least 0.8 m) and its height (at least
    /// 0.5 m; a person's 1.8 m).
    pub fn pick_box(&self) -> (V3, V3) {
        let hx = (self.size[0] * 0.5).max(0.4) + 0.05;
        let hz = (self.size[2] * 0.5).max(0.4) + 0.05;
        let h = if self.skeletal {
            1.8
        } else {
            self.size[1].max(0.5)
        };
        let p = self.pos;
        ([p[0] - hx, 0.0, p[2] - hz], [p[0] + hx, h, p[2] + hz])
    }

    fn centre(&self) -> V3 {
        let (lo, hi) = self.pick_box();
        lerp(lo, hi, 0.5)
    }

    pub fn port_out(&self) -> Option<V3> {
        if NO_OUTPUT.contains(&self.kind.as_str()) {
            return None;
        }
        let (lo, hi) = self.pick_box();
        Some([hi[0] + 0.25, 0.06, (lo[2] + hi[2]) / 2.0])
    }

    pub fn port_in(&self) -> Option<V3> {
        if NO_INPUT.contains(&self.kind.as_str()) {
            return None;
        }
        let (lo, hi) = self.pick_box();
        Some([lo[0] - 0.25, 0.06, (lo[2] + hi[2]) / 2.0])
    }

    pub fn port_service(&self) -> Option<V3> {
        if !SERVICE.contains(&self.kind.as_str()) {
            return None;
        }
        let (lo, hi) = self.pick_box();
        Some([hi[0] + 0.3, 0.06, (lo[2] + hi[2]) / 2.0])
    }

    /// Distance from the centre to the edge of the footprint along `dir` (floor).
    fn radius(&self, dir: V3) -> f64 {
        let hx = (self.size[0] * 0.5).max(0.3);
        let hz = (self.size[2] * 0.5).max(0.3);
        let kx = hx / dir[0].abs().max(1e-6);
        let kz = hz / dir[2].abs().max(1e-6);
        kx.min(kz) + 0.15
    }
}

/// The parameter of the ray `o + t d` where it enters the box, `None` when it misses.
pub fn ray_box(o: V3, d: V3, lo: V3, hi: V3) -> Option<f64> {
    let (mut tmin, mut tmax) = (f64::NEG_INFINITY, f64::INFINITY);
    for k in 0..3 {
        if d[k].abs() < 1e-9 {
            if o[k] < lo[k] || o[k] > hi[k] {
                return None;
            }
        } else {
            let t1 = (lo[k] - o[k]) / d[k];
            let t2 = (hi[k] - o[k]) / d[k];
            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }
    }
    (tmax >= tmin.max(0.0)).then_some(tmin.max(0.0))
}

/// A flow connection.
#[derive(Clone, Debug, PartialEq)]
pub struct Conn {
    pub from: String,
    pub to: String,
    pub port: i64,
    pub logical: bool,
}

/// A team or request line (`equipe`, `transporte`, `operadores`, `manutencao`...).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TeamLine {
    pub kind: String,
    pub from: String,
    pub to: String,
}

impl TeamLine {
    /// What it means, in words ("operador opera processo").
    pub fn said(&self) -> String {
        match self.kind.as_str() {
            "operadores" => format!("{} opera {}", self.to, self.from),
            "transporte" => format!("{} leva os itens de {}", self.to, self.from),
            "equipe" => format!("{} está na equipe de {}", self.to, self.from),
            k => format!("Ligação de {}: {} → {}", team_name(k), self.from, self.to),
        }
    }
}

pub fn team_name(kind: &str) -> &str {
    match kind {
        "equipe" => "equipe",
        "transporte" => "transporte",
        "operadores" => "operação",
        "manutencao" => "reparo",
        "casa" => "casa",
        "pausa" => "pausa",
        k => k,
    }
}

/// A connection line as drawn (floor points).
#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub from: String,
    pub to: String,
    pub port: i64,
    pub logical: bool,
    pub a: V3,
    pub b: V3,
}

/// A node: which block and which side.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Side {
    Out,
    In,
    Service,
}

impl Side {
    pub fn key(self) -> &'static str {
        match self {
            Side::Out => "out",
            Side::In => "in",
            Side::Service => "service",
        }
    }
}

/// What the bridge does with the document after a gesture.
#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    /// `connect_objects`; `ok` is the status on success.
    Connect {
        from: String,
        to: String,
        ok: String,
    },
    /// `disconnect_objects`.
    Disconnect {
        from: String,
        to: String,
        ok: String,
    },
    /// `team_link(from, to, operators)`; `ok` is the status on success ("" = the
    /// document's own message, `removed` prefixes it with "Removida: ").
    Team {
        from: String,
        to: String,
        operators: bool,
        ok: String,
        removed: bool,
    },
    /// The selection changed (the panels follow it).
    Selection,
    Status(String),
    /// A move begins (`begin_drag` of these names), goes on by a floor delta, ends.
    MoveBegin(Vec<String>),
    Move(V3),
    MoveEnd,
    /// Remove the selected objects.
    Remove(Vec<String>),
    /// A new risk zone along these floor points (the Zona tool).
    ZoneAdd(Vec<V3>),
    /// Zone `name` with a new polygon.
    ZoneSet {
        name: String,
        points: Vec<V3>,
    },
    ZoneRemove(String),
    /// The Rede tool: a node at this floor point (joined to the armed one by the script,
    /// which then calls [`Editor3d::net_placed`]).
    NetAddNode(V3),
    NetMoveNode {
        name: String,
        at: V3,
    },
    NetAddEdge {
        from: String,
        to: String,
    },
    NetRemoveEdge {
        from: String,
        to: String,
    },
    NetRemoveNode(String),
    /// The Colocar tool: an object of library entry `id` at this floor point (`keep`: the
    /// tool stays, Shift).
    Place {
        id: String,
        at: V3,
        keep: bool,
    },
    /// The Corredor tool: a new aisle along these points (already snapped by the script).
    AisleAdd(Vec<V3>),
    AisleMoveVertex {
        name: String,
        index: usize,
        at: V3,
    },
    AisleInsert {
        name: String,
        segment: usize,
        at: V3,
    },
    AisleRemove(String),
}

/// What a press of the Corredor tool began.
#[derive(Clone, Debug, PartialEq)]
enum AislePress {
    Vertex {
        name: String,
        index: usize,
        pos: V2,
    },
    Line(String),
    Insert {
        name: String,
        segment: usize,
        at: V3,
    },
    Floor(V3),
}

/// What a press of the Equipe tool began: the object it starts at, Shift (operators) and
/// whether it was dragged.
#[derive(Clone, Debug, PartialEq)]
struct TeamPress {
    from: String,
    shift: bool,
    pos: V2,
    moved: bool,
}

/// What a press of the Rede tool began: a node (to drag or to click), or the floor.
#[derive(Clone, Debug, PartialEq)]
struct NetPress {
    node: Option<String>,
    floor: Option<V3>,
    pos: V2,
}

/// A risk zone as the editor sees it: its floor polygon and its level (the colour).
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneView {
    pub name: String,
    pub points: Vec<V3>,
    pub level: Option<u8>,
}

/// A zone being reshaped: a vertex, or the whole polygon (`vertex` `None`).
#[derive(Clone, Debug, PartialEq)]
struct ZoneDrag {
    name: String,
    vertex: Option<usize>,
    start: V3,
    original: Vec<V3>,
    moved: bool,
    press: V2,
}

/// What a press began.
#[derive(Clone, Debug, PartialEq)]
enum PressKind {
    Floor,
    Block(String),
    /// A link from an output node (or a block with the Ligar tool: `tool`).
    Link {
        from: String,
        tool: bool,
    },
    LinkIn(String),
    Serve(String),
    /// A line was picked: the release does nothing.
    Pick,
}

#[derive(Clone, Debug, PartialEq)]
struct Press {
    pos: V2,
    shift: bool,
    kind: PressKind,
    moved: bool,
}

/// The editor: the scene seen by the camera, the selection and the gesture under way.
#[derive(Clone, Debug, Default)]
pub struct Editor3d {
    pub camera: Camera,
    pub blocks: Vec<Block>,
    index: HashMap<String, usize>,
    pub conns: Vec<Conn>,
    pub teams: Vec<TeamLine>,
    /// Team kinds hidden by the user (not picked).
    pub hidden: Vec<String>,
    /// `selecionar` or `ligar` (the other tools are the script's).
    pub tool: String,
    pub selected: Vec<String>,
    pub selected_link: Option<(String, String)>,
    pub selected_team: Option<TeamLine>,
    pub connect_from: String,
    pub connect_to: String,
    pub serve_from: String,
    pub hover: String,
    pub hover_node: Option<(String, Side)>,
    pub mouse: V2,
    press: Option<Press>,
    dragging: bool,
    drag_floor: V3,
    /// The box being drawn (screen corners).
    pub boxing: Option<(V2, V2)>,
    last_click: V2,
    last_hits: Vec<String>,
    last_index: usize,
    /// The risk zones (`Model.zones`), the one selected and the one being drawn.
    pub zones: Vec<ZoneView>,
    pub selected_zone: String,
    pub zone_points: Vec<V3>,
    zone_drag: Option<ZoneDrag>,
    /// The path network (`Model.network`): nodes and edges, the node the next click joins
    /// from, the node or edge selected, and the node being dragged.
    pub net_nodes: Vec<(String, V3)>,
    pub net_edges: Vec<(String, String)>,
    pub armed: String,
    pub net_node: String,
    pub net_edge: Option<(String, String)>,
    net_press: Option<NetPress>,
    pub net_drag_at: Option<V3>,
    /// The Colocar tool's library entry.
    pub place_id: String,
    /// The Equipe tool: the start of a link made by two clicks, and the drag under way.
    /// `alt` turns the Ligar tool into Equipe while held.
    pub team_from: String,
    team_press: Option<TeamPress>,
    pub alt: bool,
    /// The aisles (name, centre line), the one selected (and its vertex) and the one being
    /// drawn (snapped points), and the vertex dragged (where to, snapped by the script).
    pub aisles: Vec<(String, Vec<V3>)>,
    pub aisle: String,
    pub aisle_vertex: Option<usize>,
    pub aisle_points: Vec<V3>,
    aisle_press: Option<AislePress>,
    pub aisle_drag_at: Option<V3>,
    /// The grid step points snap to (0: none) and whether the press is a double click.
    pub snap: f64,
    pub double: bool,
}

impl Editor3d {
    pub fn new() -> Editor3d {
        Editor3d {
            tool: "selecionar".into(),
            last_click: [-1000.0, -1000.0],
            ..Default::default()
        }
    }

    /// The scene: blocks, connections and team lines. A connection or line selected that is
    /// gone is dropped.
    pub fn set_scene(&mut self, blocks: Vec<Block>, conns: Vec<Conn>, teams: Vec<TeamLine>) {
        self.index = blocks
            .iter()
            .enumerate()
            .map(|(i, b)| (b.name.clone(), i))
            .collect();
        self.blocks = blocks;
        self.conns = conns;
        self.teams = teams;
        // The selection is the document's to keep (the script filters it by the objects): a
        // scene fed before the 3D view is rebuilt must not drop a block just added.
        let idx = &self.index;
        if let Some((a, b)) = &self.selected_link
            && !self.conns.iter().any(|c| &c.from == a && &c.to == b)
        {
            self.selected_link = None;
        }
        if let Some(t) = &self.selected_team
            && !self.teams.contains(t)
        {
            self.selected_team = None;
        }
        for n in [
            &mut self.connect_from,
            &mut self.connect_to,
            &mut self.serve_from,
        ] {
            if !n.is_empty() && !idx.contains_key(n.as_str()) {
                n.clear();
            }
        }
    }

    /// Moves block `name` where it is drawn now (during a move).
    pub fn set_position(&mut self, name: &str, pos: V3) {
        if let Some(&i) = self.index.get(name) {
            self.blocks[i].pos = pos;
        }
    }

    pub fn block(&self, name: &str) -> Option<&Block> {
        self.index.get(name).map(|&i| &self.blocks[i])
    }

    pub fn kind_of(&self, name: &str) -> &str {
        self.block(name).map_or("", |b| b.kind.as_str())
    }

    // ----- picking ----------------------------------------------------------------------

    /// Blocks under screen point `p`, nearest first; a block small on screen is picked within
    /// [`PICK_CENTRE`] pixels of its centre when the ray hits nothing.
    pub fn hits_at(&self, p: V2) -> Vec<String> {
        let (o, d) = self.camera.ray(p);
        let mut found: Vec<(f64, &str)> = self
            .blocks
            .iter()
            .filter_map(|b| {
                let (lo, hi) = b.pick_box();
                ray_box(o, d, lo, hi).map(|t| (t, b.name.as_str()))
            })
            .collect();
        found.sort_by(|a, b| a.0.total_cmp(&b.0));
        if found.is_empty() {
            let mut best = PICK_CENTRE;
            let mut name = None;
            for b in &self.blocks {
                if let Some(s) = self.camera.project(b.centre()) {
                    let d = dist2(s, p);
                    if d < best {
                        best = d;
                        name = Some(b.name.clone());
                    }
                }
            }
            return name.into_iter().collect();
        }
        found.into_iter().map(|(_, n)| n.to_owned()).collect()
    }

    /// The block's centre on screen.
    pub fn block_screen(&self, name: &str) -> Option<V2> {
        self.camera.project(self.block(name)?.centre())
    }

    pub fn node_pos(&self, name: &str, side: Side) -> Option<V3> {
        let b = self.block(name)?;
        match side {
            Side::Out => b.port_out(),
            Side::In => b.port_in(),
            Side::Service => b.port_service(),
        }
    }

    /// The node under `p`: service first, then output, then input.
    pub fn node_at(&self, p: V2) -> Option<(String, Side)> {
        for side in [Side::Service, Side::Out, Side::In] {
            for b in &self.blocks {
                if let Some(w) = self.node_pos(&b.name, side)
                    && self.camera.near(w, p, PORT_PIXELS)
                {
                    return Some((b.name.clone(), side));
                }
            }
        }
        None
    }

    fn node_of(&self, p: V2, side: Side, except: &str) -> Option<String> {
        self.blocks
            .iter()
            .filter(|b| b.name != except)
            .find(|b| {
                self.node_pos(&b.name, side)
                    .is_some_and(|w| self.camera.near(w, p, PORT_PIXELS))
            })
            .map(|b| b.name.clone())
    }

    /// The connection lines as drawn: from node to node when the destination is ahead of the
    /// origin, else edge to edge.
    pub fn segments(&self) -> Vec<Segment> {
        let mut out = Vec::new();
        for c in &self.conns {
            let (Some(fa), Some(fb)) = (self.block(&c.from), self.block(&c.to)) else {
                continue;
            };
            let a = [fa.pos[0], 0.06, fa.pos[2]];
            let b = [fb.pos[0], 0.06, fb.pos[2]];
            let length = len(sub(b, a));
            if length < 1e-3 {
                continue;
            }
            let dir = scale(sub(b, a), 1.0 / length);
            let (mut ra, mut rb) = (fa.radius(dir), fb.radius(scale(dir, -1.0)));
            if ra + rb >= length - 0.2 {
                ra = length * 0.3;
                rb = length * 0.3;
            }
            let (mut sa, mut sb) = (add(a, scale(dir, ra)), sub(b, scale(dir, rb)));
            if let (Some(po), Some(pi)) = (fa.port_out(), fb.port_in())
                && pi[0] - po[0] > 0.4
            {
                let d2 = norm(sub(pi, po));
                sa = add(po, scale(d2, 0.13));
                sb = sub(pi, scale(d2, 0.13));
            }
            out.push(Segment {
                from: c.from.clone(),
                to: c.to.clone(),
                port: c.port,
                logical: c.logical,
                a: sa,
                b: sb,
            });
        }
        out
    }

    /// The connection whose line passes within [`PICK_PIXELS`] of `p`.
    pub fn link_at(&self, p: V2) -> Option<(String, String)> {
        let mut best = PICK_PIXELS;
        let mut hit = None;
        for s in self.segments() {
            let (Some(a), Some(b)) = (self.camera.project(s.a), self.camera.project(s.b)) else {
                continue;
            };
            let d = segment_distance(p, a, b);
            if d < best {
                best = d;
                hit = Some((s.from, s.to));
            }
        }
        hit
    }

    /// The two ends of a team line, `None` when one block is gone.
    pub fn team_ends(&self, l: &TeamLine) -> Option<(V3, V3)> {
        let a = self.block(&l.from)?.pos;
        let b = self.block(&l.to)?.pos;
        Some((add(a, [0.0, 0.09, 0.0]), add(b, [0.0, 0.09, 0.0])))
    }

    pub fn team_line_at(&self, p: V2) -> Option<TeamLine> {
        let mut best = PICK_PIXELS;
        let mut hit = None;
        for l in self.teams.iter().filter(|l| !self.hidden.contains(&l.kind)) {
            let Some((a, b)) = self.team_ends(l) else {
                continue;
            };
            let (Some(a), Some(b)) = (self.camera.project(a), self.camera.project(b)) else {
                continue;
            };
            let d = segment_distance(p, a, b);
            if d < best {
                best = d;
                hit = Some(l.clone());
            }
        }
        hit
    }

    pub fn has_team_line(&self, kind: &str, from: &str, to: &str) -> bool {
        self.teams
            .iter()
            .any(|l| l.kind == kind && l.from == from && l.to == to)
    }

    /// Where the × of the selected connection or team line is (the middle of the line; to
    /// the side of a short connection).
    pub fn remove_point(&self) -> Option<V3> {
        if let Some(t) = &self.selected_team {
            let (a, b) = self.team_ends(t)?;
            return Some(add(lerp(a, b, 0.5), [0.0, 0.03, 0.0]));
        }
        let (from, to) = self.selected_link.as_ref()?;
        let s = self
            .segments()
            .into_iter()
            .find(|s| &s.from == from && &s.to == to)?;
        let mut m = lerp(s.a, s.b, 0.5);
        if len(sub(s.b, s.a)) < 1.4 {
            let d = norm(sub(s.b, s.a));
            m = add(m, scale([-d[2], 0.0, d[0]], 0.5));
        }
        Some(m)
    }

    /// The connections of a node, in the model's order.
    pub fn node_links(&self, name: &str, side: Side) -> Vec<(String, String)> {
        self.conns
            .iter()
            .filter(|c| match side {
                Side::Out => c.from == name,
                Side::In => c.to == name,
                Side::Service => false,
            })
            .map(|c| (c.from.clone(), c.to.clone()))
            .collect()
    }

    // ----- selection --------------------------------------------------------------------

    fn hint(&self, name: &str, out: &mut Vec<Effect>) {
        if SERVICE.contains(&self.kind_of(name)) {
            out.push(Effect::Status(format!(
                "{name}: arraste do nó laranja dele até uma máquina para que ele a opere (ou até um objeto, para levar os itens dele)."
            )));
        }
    }

    pub fn select_only(&mut self, name: &str, out: &mut Vec<Effect>) {
        self.selected = vec![name.to_owned()];
        self.selected_link = None;
        self.selected_team = None;
        out.push(Effect::Selection);
        self.hint(name, out);
    }

    pub fn select_names(&mut self, names: &[String], add: bool) {
        if !add {
            self.selected.clear();
        }
        for n in names {
            if !self.selected.contains(n) {
                self.selected.push(n.clone());
            }
        }
        self.selected_link = None;
        self.selected_team = None;
    }

    pub fn toggle(&mut self, name: &str) {
        if let Some(k) = self.selected.iter().position(|n| n == name) {
            self.selected.remove(k);
        } else {
            self.selected.insert(0, name.to_owned());
        }
        self.selected_link = None;
    }

    pub fn clear_selection(&mut self) {
        self.selected.clear();
        self.selected_link = None;
        self.selected_team = None;
    }

    fn select_node_links(&mut self, name: &str, side: Side, out: &mut Vec<Effect>) -> bool {
        let links = self.node_links(name, side);
        if links.is_empty() {
            return false;
        }
        let k = self
            .selected_link
            .as_ref()
            .and_then(|s| links.iter().position(|l| l == s));
        let next = k.map_or(0, |k| (k + 1) % links.len());
        self.selected.clear();
        self.selected_team = None;
        let (a, b) = links[next].clone();
        self.selected_link = Some((a.clone(), b.clone()));
        out.push(Effect::Selection);
        let extra = if links.len() > 1 {
            format!(
                " ({} de {}: clique no nó de novo para a próxima)",
                next + 1,
                links.len()
            )
        } else {
            String::new()
        };
        out.push(Effect::Status(format!(
            "Ligação {a} → {b} selecionada{extra}. Clique no × ou Del remove."
        )));
        true
    }

    /// The effect that removes the selected connection or team line.
    fn remove_selected(&mut self, out: &mut Vec<Effect>) {
        if let Some(t) = self.selected_team.clone() {
            if !["equipe", "transporte", "operadores"].contains(&t.kind.as_str()) {
                out.push(Effect::Status(format!(
                    "Esta ligação ({}) se muda no painel de propriedades de {}.",
                    team_name(&t.kind),
                    t.from
                )));
                return;
            }
            self.selected_team = None;
            out.push(Effect::Team {
                from: t.from,
                to: t.to,
                operators: t.kind == "operadores",
                ok: String::new(),
                removed: true,
            });
            return;
        }
        if let Some((a, b)) = self.selected_link.take() {
            out.push(Effect::Disconnect {
                ok: format!("Ligação {a} → {b} removida (Ctrl+Z desfaz)."),
                from: a,
                to: b,
            });
        }
    }

    /// Del: the selected connection or line, else the selected objects.
    pub fn delete(&mut self) -> Vec<Effect> {
        let mut out = Vec::new();
        if self.tool == "zona" {
            if self.selected_zone.is_empty() {
                out.push(Effect::Status(
                    "Nenhuma zona selecionada para excluir.".into(),
                ));
            } else {
                out.push(Effect::ZoneRemove(std::mem::take(&mut self.selected_zone)));
                out.push(Effect::Selection);
            }
            return out;
        }
        if self.selected_link.is_some() || self.selected_team.is_some() {
            self.remove_selected(&mut out);
        } else if self.selected.is_empty() {
            out.push(Effect::Status("Nada selecionado para excluir.".into()));
        } else {
            out.push(Effect::Remove(std::mem::take(&mut self.selected)));
        }
        out
    }

    /// Esc: a connection being made, the Ligar tool (`true`: back to Selecionar), the
    /// selection.
    pub fn cancel(&mut self) -> (bool, Vec<Effect>) {
        let mut out = Vec::new();
        if self.tool == "zona" {
            if !self.zone_points.is_empty() {
                self.zone_points.clear();
                out.push(Effect::Status("Zona cancelada.".into()));
                return (false, out);
            }
            if !self.selected_zone.is_empty() {
                self.selected_zone.clear();
                out.push(Effect::Selection);
                return (false, out);
            }
            return (true, out);
        }
        if !self.connect_from.is_empty()
            || !self.connect_to.is_empty()
            || !self.serve_from.is_empty()
        {
            self.connect_from.clear();
            self.connect_to.clear();
            self.serve_from.clear();
            self.press = None;
            out.push(Effect::Status("Ligação cancelada.".into()));
            return (false, out);
        }
        if self.tool == "ligar" {
            return (true, out);
        }
        self.clear_selection();
        out.push(Effect::Selection);
        (false, out)
    }

    pub fn set_tool(&mut self, tool: &str) {
        self.tool = tool.to_owned();
        self.team_from.clear();
        self.team_press = None;
        self.aisle_points.clear();
        self.aisle_press = None;
        self.aisle_drag_at = None;
        self.net_press = None;
        self.net_drag_at = None;
        self.zone_points.clear();
        self.zone_drag = None;
        self.connect_from.clear();
        self.connect_to.clear();
        self.serve_from.clear();
        self.press = None;
        self.dragging = false;
        self.boxing = None;
    }

    // ----- gestures ---------------------------------------------------------------------

    /// The mouse moved (no button, or dragging).
    pub fn hover(&mut self, p: V2) {
        self.mouse = p;
        let hits = self.hits_at(p);
        self.hover_node = self.node_at(p);
        self.hover = hits
            .first()
            .cloned()
            .or_else(|| self.hover_node.as_ref().map(|n| n.0.clone()))
            .unwrap_or_default();
    }

    pub fn press(&mut self, p: V2, shift: bool) -> Vec<Effect> {
        if self.tool == "zona" {
            return self.zone_press(p);
        }
        if self.tool == "rede" {
            return self.net_press(p);
        }
        if self.tool == "colocar" {
            return self.place_press(p, shift);
        }
        if self.tool == "equipe" || (self.tool == "ligar" && self.alt) {
            return self.team_press_at(p, shift);
        }
        let mut out = Vec::new();
        self.mouse = p;
        let hits = self.hits_at(p);
        if let Some(x) = self.remove_point()
            && self.camera.near(x, p, PORT_PIXELS)
        {
            self.remove_selected(&mut out);
            return out;
        }
        let begin = |kind| Press {
            pos: p,
            shift,
            kind,
            moved: false,
        };
        if self.connect_from.is_empty() {
            if let Some(res) = self.node_of(p, Side::Service, "") {
                out.push(Effect::Status(format!(
                    "Solte sobre a máquina que {res} vai operar (ou sobre um objeto, para levar os itens dele). Esc cancela."
                )));
                self.serve_from = res.clone();
                self.press = Some(begin(PressKind::Serve(res)));
                return out;
            }
            if let Some(from) = self.node_of(p, Side::Out, "") {
                self.connect_from = from.clone();
                self.press = Some(begin(PressKind::Link { from, tool: false }));
                out.push(Effect::Status(
                    "Ligar: solte sobre o destino (ou clique nele). Esc cancela.".into(),
                ));
                return out;
            }
            if let Some(to) = self.node_of(p, Side::In, "") {
                self.connect_to = to.clone();
                self.press = Some(begin(PressKind::LinkIn(to)));
                out.push(Effect::Status(
                    "Ligar: solte sobre a origem (ou o nó de saída dela). Esc cancela.".into(),
                ));
                return out;
            }
        }
        if self.tool == "ligar" {
            if self.connect_from.is_empty() {
                if let Some(from) = hits.first().cloned() {
                    self.connect_from = from.clone();
                    self.press = Some(begin(PressKind::Link { from, tool: true }));
                    out.push(Effect::Status(
                        "Ligar: solte sobre o destino (ou clique nele). Esc cancela.".into(),
                    ));
                } else {
                    out.push(Effect::Status("Ligar: clique em um bloco.".into()));
                }
            } else if let Some(to) = hits.first().cloned() {
                let from = std::mem::take(&mut self.connect_from);
                out.push(Effect::Connect {
                    ok: format!("Ligado: {from} → {to}. Clique em outra origem ou Esc."),
                    from,
                    to,
                });
            } else {
                out.push(Effect::Status("Ligar: clique em um bloco.".into()));
            }
            return out;
        }
        if hits.is_empty() {
            self.press = Some(begin(PressKind::Floor));
            if let Some((a, b)) = self.link_at(p) {
                self.press = Some(begin(PressKind::Pick));
                self.selected.clear();
                self.selected_team = None;
                out.push(Effect::Status(format!(
                    "Ligação {a} → {b} selecionada. Clique no × ou Del remove."
                )));
                self.selected_link = Some((a, b));
                out.push(Effect::Selection);
            } else if let Some(t) = self.team_line_at(p) {
                self.press = Some(begin(PressKind::Pick));
                self.selected.clear();
                self.selected_link = None;
                out.push(Effect::Status(format!(
                    "{}. Clique no × ou Del remove.",
                    t.said()
                )));
                self.selected_team = Some(t);
                out.push(Effect::Selection);
            }
            self.last_hits.clear();
            return out;
        }
        // Clicking again at the same point walks through the overlapping blocks.
        let mut index = 0;
        if dist2(p, self.last_click) < 5.0 && hits == self.last_hits && hits.len() > 1 {
            index = (self.last_index + 1) % hits.len();
        }
        self.last_click = p;
        self.last_hits = hits.clone();
        self.last_index = index;
        let name = hits[index].clone();
        self.press = Some(begin(PressKind::Block(name.clone())));
        if hits.len() > 1 {
            out.push(Effect::Status(format!(
                "{name} ({} de {} blocos neste ponto: clique de novo para o próximo).",
                index + 1,
                hits.len()
            )));
        }
        if !shift && (!self.selected.contains(&name) || index > 0) {
            self.select_only(&name, &mut out);
        } else if hits.len() == 1 {
            self.hint(&name, &mut out);
        }
        out
    }

    pub fn drag(&mut self, p: V2) -> Vec<Effect> {
        if self.tool == "zona" {
            self.zone_drag_to(p);
            return Vec::new();
        }
        if self.tool == "rede" {
            self.net_drag_to(p);
            return Vec::new();
        }
        if let Some(t) = self.team_press.as_mut() {
            self.mouse = p;
            if dist2(p, t.pos) >= DRAG_PIXELS {
                t.moved = true;
            }
            return Vec::new();
        }
        let mut out = Vec::new();
        self.mouse = p;
        let Some(press) = self.press.as_mut() else {
            return out;
        };
        if !self.dragging && self.boxing.is_none() && dist2(p, press.pos) < DRAG_PIXELS {
            return out;
        }
        match press.kind.clone() {
            PressKind::Link { .. } | PressKind::LinkIn(_) | PressKind::Serve(_) => {
                press.moved = true
            }
            PressKind::Block(_) => {
                if press.shift {
                    return out;
                }
                if !self.dragging {
                    let start = press.pos;
                    match self.camera.floor_at(start) {
                        Some(f) if !self.selected.is_empty() => {
                            self.dragging = true;
                            self.drag_floor = f;
                            out.push(Effect::MoveBegin(self.selected.clone()));
                        }
                        _ => return out,
                    }
                }
                if let Some(f) = self.camera.floor_at(p) {
                    let mut d = sub(f, self.drag_floor);
                    d[1] = 0.0;
                    out.push(Effect::Move(d));
                }
            }
            PressKind::Floor => {
                let start = press.pos;
                self.boxing = Some((start, p));
            }
            PressKind::Pick => {}
        }
        out
    }

    pub fn release(&mut self, p: V2) -> Vec<Effect> {
        if self.tool == "zona" {
            return self.zone_release();
        }
        if self.tool == "rede" {
            return self.net_release();
        }
        if self.team_press.is_some() {
            return self.team_release(p);
        }
        let mut out = Vec::new();
        self.mouse = p;
        let Some(press) = self.press.take() else {
            return out;
        };
        match press.kind {
            PressKind::Link { from, tool } => {
                if !press.moved && !tool && self.select_node_links(&from, Side::Out, &mut out) {
                    self.connect_from.clear();
                    return out;
                }
                if !press.moved {
                    if !tool {
                        self.connect_from.clear();
                        out.push(Effect::Status(format!(
                            "{from}: arraste deste nó verde até o destino para ligar."
                        )));
                    } else {
                        self.connect_from = from.clone();
                        out.push(Effect::Status(format!(
                            "Ligar: {from} → clique no destino (Esc cancela)."
                        )));
                    }
                    return out;
                }
                self.connect_from.clear();
                let to = self
                    .node_of(p, Side::In, &from)
                    .or_else(|| self.hits_at(p).into_iter().find(|n| *n != from));
                match to {
                    Some(to) => out.push(Effect::Connect {
                        ok: format!("Ligado: {from} → {to}."),
                        from,
                        to,
                    }),
                    None => out.push(Effect::Status(
                        "Ligação cancelada: solte sobre o bloco de destino.".into(),
                    )),
                }
            }
            PressKind::LinkIn(to) => {
                self.connect_to.clear();
                if !press.moved {
                    if !self.select_node_links(&to, Side::In, &mut out) {
                        out.push(Effect::Status(format!(
                            "{to} não recebe de ninguém: arraste do nó de saída da origem até aqui."
                        )));
                    }
                    return out;
                }
                let from = self
                    .node_of(p, Side::Out, &to)
                    .or_else(|| self.hits_at(p).into_iter().find(|n| *n != to));
                match from {
                    Some(from) => out.push(Effect::Connect {
                        ok: format!("Ligado: {from} → {to}."),
                        from,
                        to,
                    }),
                    None => out.push(Effect::Status(
                        "Ligação cancelada: solte sobre o bloco de origem.".into(),
                    )),
                }
            }
            PressKind::Serve(res) => {
                self.serve_from.clear();
                if !press.moved {
                    self.select_only(&res, &mut out);
                    return out;
                }
                self.finish_serve(p, &res, press.shift, &mut out);
            }
            PressKind::Pick => {}
            PressKind::Block(name) => {
                if self.dragging {
                    self.dragging = false;
                    out.push(Effect::MoveEnd);
                    out.push(Effect::Status(format!(
                        "{} objeto(s) movidos.",
                        self.selected.len()
                    )));
                } else if press.shift {
                    self.toggle(&name);
                    out.push(Effect::Selection);
                }
            }
            PressKind::Floor => {
                if let Some((a, _)) = self.boxing.take() {
                    let (lo, hi) = (
                        [a[0].min(p[0]), a[1].min(p[1])],
                        [a[0].max(p[0]), a[1].max(p[1])],
                    );
                    let inside: Vec<String> = self
                        .blocks
                        .iter()
                        .filter(|b| {
                            self.block_screen(&b.name).is_some_and(|s| {
                                s[0] >= lo[0] && s[0] <= hi[0] && s[1] >= lo[1] && s[1] <= hi[1]
                            })
                        })
                        .map(|b| b.name.clone())
                        .collect();
                    self.select_names(&inside, press.shift);
                    out.push(Effect::Selection);
                    out.push(Effect::Status(format!(
                        "{} objeto(s) na caixa.",
                        inside.len()
                    )));
                } else if !press.shift {
                    self.clear_selection();
                    out.push(Effect::Selection);
                }
            }
        }
        out
    }

    /// The end of a drag from a service node: who the resource serves.
    fn finish_serve(&mut self, p: V2, res: &str, shift: bool, out: &mut Vec<Effect>) {
        let Some(to) = self.hits_at(p).into_iter().find(|n| n != res) else {
            out.push(Effect::Status(format!(
                "Nada mudou: solte sobre a máquina (ou o objeto) que {res} vai atender."
            )));
            return;
        };
        let kind = self.kind_of(&to).to_owned();
        let res_kind = self.kind_of(res).to_owned();
        let (line, operators) = if kind == "Processor" && !shift {
            ("operadores", true)
        } else if kind == "Dispatcher" && res_kind == "Transporter" {
            ("equipe", false)
        } else if SERVICE.contains(&kind.as_str()) {
            out.push(Effect::Status(
                "Solte sobre uma máquina ou um objeto (um transportador só entra na equipe de um despachante).".into(),
            ));
            return;
        } else {
            ("transporte", false)
        };
        if self.has_team_line(line, &to, res) {
            out.push(Effect::Status(format!(
                "Já está ligado: {to} → {res}. Para desfazer, clique na linha e no ×."
            )));
            return;
        }
        let said = TeamLine {
            kind: line.into(),
            from: to.clone(),
            to: res.to_owned(),
        }
        .said();
        out.push(Effect::Team {
            from: to,
            to: res.to_owned(),
            operators,
            ok: format!("Ligado: {said}. Ctrl+Z desfaz."),
            removed: false,
        });
    }

    // ----- drawing ----------------------------------------------------------------------

    /// What to draw over the scene.
    pub fn overlay(&self) -> Overlay {
        let mut o = Overlay::default();
        for b in &self.blocks {
            let (lo, hi) = b.pick_box();
            if let Some(k) = self.selected.iter().position(|n| *n == b.name) {
                o.corners
                    .push((lo, hi, if k == 0 { Mark::Primary } else { Mark::Other }));
            }
            if b.problems {
                o.corners.push((lo, hi, Mark::Problem));
            }
            for side in [Side::Out, Side::In, Side::Service] {
                if let Some(w) = self.node_pos(&b.name, side) {
                    let lit = self.hover_node.as_ref() == Some(&(b.name.clone(), side))
                        || match side {
                            Side::Out => self.connect_from == b.name,
                            Side::In => self.connect_to == b.name,
                            Side::Service => self.serve_from == b.name,
                        };
                    let edge = if side == Side::In { lo[0] } else { hi[0] };
                    o.nodes.push(NodeMark {
                        at: w,
                        side,
                        lit,
                        edge,
                    });
                }
            }
        }
        o.remove = self.remove_point();
        if let Some(press) = &self.press
            && press.moved
        {
            let f = self.camera.floor_at(self.mouse).map(|f| [f[0], 0.06, f[2]]);
            match &press.kind {
                PressKind::Link { from, .. } => {
                    let a = self
                        .node_pos(from, Side::Out)
                        .or_else(|| self.block(from).map(|b| [b.pos[0], 0.06, b.pos[2]]));
                    if let (Some(a), Some(f)) = (a, f) {
                        o.preview = Some((a, f, PreviewKind::Flow));
                    }
                }
                PressKind::LinkIn(to) => {
                    if let (Some(b), Some(f)) = (self.node_pos(to, Side::In), f) {
                        o.preview = Some((f, b, PreviewKind::Flow));
                    }
                }
                PressKind::Serve(res) => {
                    if let (Some(a), Some(f)) = (self.node_pos(res, Side::Service), f) {
                        o.preview = Some((a, f, PreviewKind::Service));
                    }
                }
                _ => {}
            }
        }
        // A click-click connection with the Ligar tool follows the mouse too.
        if o.preview.is_none()
            && !self.connect_from.is_empty()
            && let Some(f) = self.camera.floor_at(self.mouse)
        {
            let a = self.node_pos(&self.connect_from, Side::Out).or_else(|| {
                self.block(&self.connect_from)
                    .map(|b| [b.pos[0], 0.06, b.pos[2]])
            });
            if let Some(a) = a {
                o.preview = Some((a, [f[0], 0.06, f[2]], PreviewKind::Flow));
            }
        }
        o.boxing = self.boxing;
        o.zones = self
            .zones
            .iter()
            .map(|z| (z.clone(), z.name == self.selected_zone))
            .collect();
        if !self.zone_points.is_empty() {
            let mut pts = self.zone_points.clone();
            if let Some(f) = self.camera.floor_at(self.mouse) {
                pts.push(self.snapped(f));
            }
            o.zone_drawing = pts;
        }
        o
    }

    // ----- the Colocar and Equipe tools ------------------------------------------------

    fn place_press(&mut self, p: V2, keep: bool) -> Vec<Effect> {
        self.mouse = p;
        match self.camera.floor_at(p) {
            Some(at) if !self.place_id.is_empty() => vec![Effect::Place {
                id: self.place_id.clone(),
                at,
                keep,
            }],
            Some(_) => Vec::new(),
            None => vec![Effect::Status(
                "Aponte para o chão para colocar o objeto.".into(),
            )],
        }
    }

    fn team_press_at(&mut self, p: V2, shift: bool) -> Vec<Effect> {
        self.mouse = p;
        let hits = self.hits_at(p);
        if !self.team_from.is_empty() {
            let from = std::mem::take(&mut self.team_from);
            return match hits.first() {
                Some(to) => self.team_effect(&from, to, shift),
                None => vec![Effect::Status(
                    "Equipe: clique no transportador ou despachante de destino.".into(),
                )],
            };
        }
        let Some(from) = hits.first().cloned() else {
            return vec![Effect::Status(
                "Equipe: comece num despachante ou num objeto.".into(),
            )];
        };
        self.team_from = from.clone();
        self.team_press = Some(TeamPress {
            from,
            shift,
            pos: p,
            moved: false,
        });
        Vec::new()
    }

    fn team_release(&mut self, p: V2) -> Vec<Effect> {
        let Some(t) = self.team_press.take() else {
            return Vec::new();
        };
        if !t.moved {
            // A click: the next click is the destination.
            return vec![Effect::Status(format!(
                "Equipe: {} → clique no destino (Esc cancela).",
                t.from
            ))];
        }
        self.team_from.clear();
        match self.hits_at(p).first() {
            Some(to) => self.team_effect(&t.from, to, t.shift),
            None => vec![Effect::Status(
                "Equipe: solte sobre o transportador ou despachante.".into(),
            )],
        }
    }

    /// The `team_link` of the Equipe tool (the document says what changed).
    fn team_effect(&self, from: &str, to: &str, operators: bool) -> Vec<Effect> {
        if from == to {
            return vec![Effect::Status("Equipe: solte sobre outro objeto.".into())];
        }
        vec![Effect::Team {
            from: from.to_owned(),
            to: to.to_owned(),
            operators,
            ok: String::new(),
            removed: false,
        }]
    }

    /// Esc with the Equipe tool: the link begun by a click (true when there was one).
    pub fn team_cancel(&mut self) -> bool {
        self.team_press = None;
        !std::mem::take(&mut self.team_from).is_empty()
    }

    // ----- the Corredor tool -----------------------------------------------------------

    /// The aisles (name, centre line). The selection stays if it still exists.
    pub fn set_aisles(&mut self, aisles: Vec<(String, Vec<V3>)>) {
        match aisles.iter().find(|a| a.0 == self.aisle) {
            None => {
                self.aisle.clear();
                self.aisle_vertex = None;
            }
            Some(a) => {
                if self.aisle_vertex.is_some_and(|k| k >= a.1.len()) {
                    self.aisle_vertex = None;
                }
            }
        }
        self.aisles = aisles;
    }

    /// Where vertex `index` of aisle `name` is drawn (where it is dragged to, while dragged).
    pub fn aisle_vertex_pos(&self, name: &str, index: usize) -> Option<V3> {
        if let (
            Some(AislePress::Vertex {
                name: n, index: k, ..
            }),
            Some(at),
        ) = (&self.aisle_press, self.aisle_drag_at)
            && n == name
            && *k == index
        {
            return Some(at);
        }
        self.aisles
            .iter()
            .find(|a| a.0 == name)
            .and_then(|a| a.1.get(index).copied())
    }

    /// The vertex nearest to `p` within 12 px (the selected aisle wins ties).
    pub fn aisle_vertex_at(&self, p: V2) -> Option<(String, usize)> {
        let mut best = 12.0;
        let mut hit = None;
        for (name, pts) in &self.aisles {
            for k in 0..pts.len() {
                let Some(w) = self.aisle_vertex_pos(name, k) else {
                    continue;
                };
                if let Some(sp) = self.camera.project(w) {
                    let d = dist2(sp, p);
                    if d < best || (d == best && *name == self.aisle) {
                        best = d;
                        hit = Some((name.clone(), k));
                    }
                }
            }
        }
        hit
    }

    /// The centre line nearest to `p` within 9 px: the aisle and the segment.
    pub fn aisle_line_at(&self, p: V2) -> Option<(String, usize)> {
        let mut best = 9.0;
        let mut hit = None;
        for (name, pts) in &self.aisles {
            for k in 0..pts.len().saturating_sub(1) {
                let (Some(a), Some(b)) =
                    (self.camera.project(pts[k]), self.camera.project(pts[k + 1]))
                else {
                    continue;
                };
                let d = segment_distance(p, a, b);
                if d < best || (d == best && *name == self.aisle) {
                    best = d;
                    hit = Some((name.clone(), k));
                }
            }
        }
        hit
    }

    /// The vertex being dragged, `(aisle, index)`, for the script to snap the point to the
    /// other aisles.
    pub fn aisle_dragging(&self) -> Option<(String, usize)> {
        match &self.aisle_press {
            Some(AislePress::Vertex { name, index, .. }) => Some((name.clone(), *index)),
            _ => None,
        }
    }

    /// A press of the Corredor tool; `floor` is the floor point under `p` already snapped
    /// (to the grid and to the other aisles) by the script.
    pub fn aisle_press_at(&mut self, p: V2, shift: bool, floor: Option<V3>) -> Vec<Effect> {
        self.mouse = p;
        self.aisle_press = None;
        if !self.aisle_points.is_empty() {
            if let Some(f) = floor {
                self.aisle_press = Some(AislePress::Floor(f));
            }
            return Vec::new();
        }
        if !shift && let Some((name, index)) = self.aisle_vertex_at(p) {
            self.aisle_press = Some(AislePress::Vertex {
                name,
                index,
                pos: p,
            });
            return Vec::new();
        }
        let line = self.aisle_line_at(p);
        match (line, shift, floor) {
            (Some((name, _)), false, _) => self.aisle_press = Some(AislePress::Line(name)),
            (Some((name, segment)), true, Some(at)) if name == self.aisle => {
                // Shift on the selected line: a bend in the middle of it.
                self.aisle_press = Some(AislePress::Insert { name, segment, at });
            }
            (_, _, Some(f)) => self.aisle_press = Some(AislePress::Floor(f)),
            (_, _, None) => {
                return vec![Effect::Status(
                    "Aponte para o chão para pôr um ponto do corredor.".into(),
                )];
            }
        }
        Vec::new()
    }

    /// The vertex dragged goes to `at` (snapped by the script) once the mouse moved enough.
    pub fn aisle_drag_to(&mut self, p: V2, at: Option<V3>) {
        self.mouse = p;
        if let Some(AislePress::Vertex { pos, .. }) = &self.aisle_press
            && (self.aisle_drag_at.is_some() || dist2(p, *pos) >= DRAG_PIXELS)
            && let Some(at) = at
        {
            self.aisle_drag_at = Some(at);
        }
    }

    pub fn aisle_release(&mut self) -> Vec<Effect> {
        let Some(press) = self.aisle_press.take() else {
            return Vec::new();
        };
        let dragged = self.aisle_drag_at.take();
        match press {
            AislePress::Vertex { name, index, .. } => {
                if let Some(at) = dragged {
                    self.aisle = name.clone();
                    self.aisle_vertex = Some(index);
                    return vec![Effect::AisleMoveVertex { name, index, at }];
                }
                self.select_aisle(&name, Some(index))
            }
            AislePress::Line(name) => self.select_aisle(&name, None),
            AislePress::Insert { name, segment, at } => {
                self.aisle_vertex = Some(segment + 1);
                vec![Effect::AisleInsert { name, segment, at }]
            }
            AislePress::Floor(at) => {
                if self
                    .aisle_points
                    .last()
                    .is_some_and(|l| len(sub(*l, at)) < 0.02)
                {
                    return self.aisle_finish();
                }
                self.aisle.clear();
                self.aisle_vertex = None;
                self.aisle_points.push(at);
                let msg = if self.aisle_points.len() == 1 {
                    "Corredor: clique os próximos pontos; Enter ou duplo clique termina, Backspace volta um ponto, Esc cancela.".to_owned()
                } else {
                    format!(
                        "{} ponto(s). Enter ou duplo clique termina.",
                        self.aisle_points.len()
                    )
                };
                vec![Effect::Selection, Effect::Status(msg)]
            }
        }
    }

    fn select_aisle(&mut self, name: &str, vertex: Option<usize>) -> Vec<Effect> {
        self.aisle = name.to_owned();
        self.aisle_vertex = vertex;
        let msg = match vertex {
            Some(k) => format!(
                "Vértice {} de {name} selecionado (arraste para mover).",
                k + 1
            ),
            None => format!(
                "Corredor {name} selecionado (Del exclui; Shift + clique na linha acrescenta um vértice)."
            ),
        };
        vec![Effect::Selection, Effect::Status(msg)]
    }

    /// Enter (or a click on the last point): the aisle being drawn ends. Empty when none.
    pub fn aisle_finish(&mut self) -> Vec<Effect> {
        if self.aisle_points.is_empty() {
            return Vec::new();
        }
        let line = std::mem::take(&mut self.aisle_points);
        if line.len() < 2 {
            return vec![Effect::Status(
                "Um corredor precisa de ao menos 2 pontos.".into(),
            )];
        }
        vec![Effect::AisleAdd(line)]
    }

    /// Esc: the aisle being drawn, else the selection (true when there was one).
    pub fn aisle_cancel(&mut self) -> bool {
        if !self.aisle_points.is_empty() {
            self.aisle_points.clear();
            return true;
        }
        if !self.aisle.is_empty() {
            self.aisle.clear();
            self.aisle_vertex = None;
            return true;
        }
        false
    }

    /// Del or Backspace: the last point while drawing, else the selected aisle (empty when
    /// neither).
    pub fn aisle_delete(&mut self) -> Vec<Effect> {
        if self.aisle_points.pop().is_some() {
            return vec![
                Effect::Selection,
                Effect::Status(format!("Ponto removido ({}).", self.aisle_points.len())),
            ];
        }
        if !self.aisle.is_empty() {
            self.aisle_vertex = None;
            return vec![Effect::AisleRemove(std::mem::take(&mut self.aisle))];
        }
        Vec::new()
    }

    // ----- the path network (the Rede tool) --------------------------------------------

    /// The network: nodes (name, floor point) and edges. What is selected stays if it still
    /// exists.
    pub fn set_network(&mut self, nodes: Vec<(String, V3)>, edges: Vec<(String, String)>) {
        let has = |n: &str| nodes.iter().any(|x| x.0 == n);
        if !has(&self.armed) {
            self.armed.clear();
        }
        if !has(&self.net_node) {
            self.net_node.clear();
        }
        if let Some(e) = &self.net_edge
            && !edges.contains(e)
        {
            self.net_edge = None;
        }
        self.net_nodes = nodes;
        self.net_edges = edges;
    }

    /// Where node `name` is drawn now (where it is dragged to, while dragged).
    pub fn net_position(&self, name: &str) -> Option<V3> {
        if let (Some(p), Some(at)) = (&self.net_press, self.net_drag_at)
            && p.node.as_deref() == Some(name)
        {
            return Some(at);
        }
        self.net_nodes.iter().find(|n| n.0 == name).map(|n| n.1)
    }

    /// The node within 14 px of `p` (the nearest).
    pub fn net_node_at(&self, p: V2) -> Option<String> {
        let mut best = PORT_PIXELS;
        let mut hit = None;
        for (n, w) in &self.net_nodes {
            if let Some(s) = self.camera.project(*w) {
                let d = dist2(s, p);
                if d <= best {
                    best = d;
                    hit = Some(n.clone());
                }
            }
        }
        hit
    }

    /// The edge within 9 px of `p` (the nearest).
    pub fn net_edge_at(&self, p: V2) -> Option<(String, String)> {
        let mut best = 9.0;
        let mut hit = None;
        for (a, b) in &self.net_edges {
            let (Some(wa), Some(wb)) = (self.net_position(a), self.net_position(b)) else {
                continue;
            };
            let (Some(sa), Some(sb)) = (self.camera.project(wa), self.camera.project(wb)) else {
                continue;
            };
            let d = segment_distance(p, sa, sb);
            if d <= best {
                best = d;
                hit = Some((a.clone(), b.clone()));
            }
        }
        hit
    }

    fn net_press(&mut self, p: V2) -> Vec<Effect> {
        let mut out = Vec::new();
        self.mouse = p;
        if let Some(n) = self.net_node_at(p) {
            self.net_press = Some(NetPress {
                node: Some(n),
                floor: None,
                pos: p,
            });
            return out;
        }
        if let Some((a, b)) = self.net_edge_at(p) {
            self.net_node.clear();
            out.push(Effect::Status(format!(
                "Trecho {a} → {b} selecionado (Del remove)."
            )));
            self.net_edge = Some((a, b));
            out.push(Effect::Selection);
            self.net_press = None;
            return out;
        }
        match self.camera.floor_at(p) {
            Some(f) => {
                self.net_press = Some(NetPress {
                    node: None,
                    floor: Some(self.snapped(f)),
                    pos: p,
                })
            }
            None => {
                self.net_press = None;
                out.push(Effect::Status(
                    "Aponte para o chão para colocar um nó.".into(),
                ));
            }
        }
        out
    }

    fn net_drag_to(&mut self, p: V2) {
        self.mouse = p;
        let Some(press) = &self.net_press else {
            return;
        };
        if press.node.is_none() || (self.net_drag_at.is_none() && dist2(p, press.pos) < DRAG_PIXELS)
        {
            return;
        }
        if let Some(f) = self.camera.floor_at(p) {
            self.net_drag_at = Some(self.snapped(f));
        }
    }

    fn net_release(&mut self) -> Vec<Effect> {
        let mut out = Vec::new();
        let Some(press) = self.net_press.take() else {
            return out;
        };
        let dragged = self.net_drag_at.take();
        if let Some(n) = press.node {
            if let Some(at) = dragged {
                out.push(Effect::NetMoveNode { name: n, at });
                return out;
            }
            if !self.armed.is_empty() && self.armed != n {
                out.push(Effect::NetAddEdge {
                    from: self.armed.clone(),
                    to: n.clone(),
                });
            }
            self.armed = n.clone();
            self.net_node = n;
            self.net_edge = None;
            out.push(Effect::Selection);
            return out;
        }
        if let Some(at) = press.floor {
            out.push(Effect::NetAddNode(at));
        }
        out
    }

    /// The script placed node `name` (from [`Effect::NetAddNode`]): it is armed and selected.
    pub fn net_placed(&mut self, name: &str) {
        self.armed = name.to_owned();
        self.net_node = name.to_owned();
        self.net_edge = None;
    }

    /// Esc with the Rede tool: stops the path (true when there was one).
    pub fn net_cancel(&mut self) -> bool {
        !std::mem::take(&mut self.armed).is_empty()
    }

    /// Del with the Rede tool: the edge or the node selected (empty when nothing is).
    pub fn net_delete(&mut self) -> Vec<Effect> {
        if let Some((from, to)) = self.net_edge.take() {
            return vec![Effect::NetRemoveEdge { from, to }, Effect::Selection];
        }
        if !self.net_node.is_empty() {
            return vec![
                Effect::NetRemoveNode(std::mem::take(&mut self.net_node)),
                Effect::Selection,
            ];
        }
        Vec::new()
    }

    // ----- risk zones (the Zona tool) ---------------------------------------------------

    /// The zones of the model (floor polygons). The one selected stays if it still exists.
    pub fn set_zones(&mut self, zones: Vec<ZoneView>) {
        if !zones.iter().any(|z| z.name == self.selected_zone) {
            self.selected_zone.clear();
        }
        if self.zone_drag.is_none() {
            self.zones = zones;
        }
    }

    fn snapped(&self, f: V3) -> V3 {
        if self.snap <= 0.0 {
            return [f[0], 0.0, f[2]];
        }
        let s = self.snap;
        [(f[0] / s).round() * s, 0.0, (f[2] / s).round() * s]
    }

    /// The zone under the floor point of `p` (the last drawn wins).
    pub fn zone_at(&self, p: V2) -> Option<String> {
        let f = self.camera.floor_at(p)?;
        self.zones
            .iter()
            .rev()
            .find(|z| {
                let poly: Vec<[f64; 2]> = z.points.iter().map(|v| [v[0], v[2]]).collect();
                crate::zone_edit::contains(&poly, [f[0], f[2]])
            })
            .map(|z| z.name.clone())
    }

    /// The vertex of the selected zone near `p`.
    fn zone_vertex_at(&self, p: V2) -> Option<usize> {
        let z = self.zones.iter().find(|z| z.name == self.selected_zone)?;
        z.points
            .iter()
            .position(|v| self.camera.near(*v, p, PORT_PIXELS))
    }

    /// Screen point of vertex `k` of zone `name` (the roteiros' target).
    pub fn zone_vertex_screen(&self, name: &str, k: usize) -> Option<V2> {
        let z = self.zones.iter().find(|z| z.name == name)?;
        self.camera.project(*z.points.get(k)?)
    }

    /// Screen point of the middle of zone `name`.
    pub fn zone_screen(&self, name: &str) -> Option<V2> {
        let z = self.zones.iter().find(|z| z.name == name)?;
        let n = z.points.len().max(1) as f64;
        let c = z.points.iter().fold([0.0; 3], |a, v| add(a, *v));
        self.camera.project(scale(c, 1.0 / n))
    }

    fn zone_press(&mut self, p: V2) -> Vec<Effect> {
        let mut out = Vec::new();
        self.mouse = p;
        let double = std::mem::take(&mut self.double);
        let floor = self.camera.floor_at(p).map(|f| self.snapped(f));
        if !self.zone_points.is_empty() {
            let closes = self.zone_points.len() >= 3
                && (double || self.camera.near(self.zone_points[0], p, PORT_PIXELS));
            if closes {
                return self.zone_finish();
            }
            if let Some(f) = floor
                && self.zone_points.last() != Some(&f)
            {
                self.zone_points.push(f);
            }
            out.push(Effect::Status(format!(
                "Zona: {} ponto(s). Clique no primeiro ponto, Enter ou duplo clique fecha; Backspace volta; Esc cancela.",
                self.zone_points.len()
            )));
            return out;
        }
        if let Some(k) = self.zone_vertex_at(p) {
            self.begin_zone_drag(Some(k), p, floor);
            return out;
        }
        if let Some(name) = self.zone_at(p) {
            if name != self.selected_zone {
                self.selected_zone = name.clone();
                out.push(Effect::Selection);
                out.push(Effect::Status(format!(
                    "Zona {name}: arraste para mover, arraste um vértice para mudar a forma; Del exclui."
                )));
            }
            self.begin_zone_drag(None, p, floor);
            return out;
        }
        if !self.selected_zone.is_empty() {
            self.selected_zone.clear();
            out.push(Effect::Selection);
        }
        if let Some(f) = floor {
            self.zone_points.push(f);
            out.push(Effect::Status(
                "Zona: clique para pôr os outros pontos; Enter, duplo clique ou o primeiro ponto fecha.".into(),
            ));
        }
        out
    }

    fn begin_zone_drag(&mut self, vertex: Option<usize>, p: V2, floor: Option<V3>) {
        let Some(z) = self.zones.iter().find(|z| z.name == self.selected_zone) else {
            return;
        };
        let start = self.camera.floor_at(p).or(floor).unwrap_or([0.0; 3]);
        self.zone_drag = Some(ZoneDrag {
            name: z.name.clone(),
            vertex,
            start,
            original: z.points.clone(),
            moved: false,
            press: p,
        });
    }

    fn zone_drag_to(&mut self, p: V2) {
        self.mouse = p;
        let Some(f) = self.camera.floor_at(p) else {
            return;
        };
        let snapped = self.snapped(f);
        let step = self.snap;
        let Some(d) = self.zone_drag.as_mut() else {
            return;
        };
        if !d.moved && dist2(p, d.press) < DRAG_PIXELS {
            return;
        }
        d.moved = true;
        let pts = match d.vertex {
            Some(k) => {
                let mut v = d.original.clone();
                if let Some(x) = v.get_mut(k) {
                    *x = snapped;
                }
                v
            }
            None => {
                let mut delta = sub(f, d.start);
                if step > 0.0 {
                    delta = [
                        (delta[0] / step).round() * step,
                        0.0,
                        (delta[2] / step).round() * step,
                    ];
                }
                delta[1] = 0.0;
                d.original.iter().map(|v| add(*v, delta)).collect()
            }
        };
        let name = d.name.clone();
        if let Some(z) = self.zones.iter_mut().find(|z| z.name == name) {
            z.points = pts;
        }
    }

    fn zone_release(&mut self) -> Vec<Effect> {
        let Some(d) = self.zone_drag.take() else {
            return Vec::new();
        };
        if !d.moved {
            return Vec::new();
        }
        let points = self
            .zones
            .iter()
            .find(|z| z.name == d.name)
            .map(|z| z.points.clone())
            .unwrap_or_default();
        vec![
            Effect::ZoneSet {
                name: d.name,
                points,
            },
            Effect::Status("Zona alterada (Ctrl+Z desfaz).".into()),
        ]
    }

    /// Enter or a double click: closes the zone being drawn.
    pub fn zone_finish(&mut self) -> Vec<Effect> {
        if self.zone_points.len() < 3 {
            return vec![Effect::Status(
                "Uma zona precisa de pelo menos 3 pontos.".into(),
            )];
        }
        vec![Effect::ZoneAdd(std::mem::take(&mut self.zone_points))]
    }

    /// Backspace: takes the last point back.
    pub fn zone_back(&mut self) {
        self.zone_points.pop();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    Primary,
    Other,
    Problem,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewKind {
    Flow,
    Service,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeMark {
    pub at: V3,
    pub side: Side,
    pub lit: bool,
    /// The x of the block's side the node sticks out of.
    pub edge: f64,
}

/// What the overlay draws.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Overlay {
    pub nodes: Vec<NodeMark>,
    pub corners: Vec<(V3, V3, Mark)>,
    pub remove: Option<V3>,
    pub preview: Option<(V3, V3, PreviewKind)>,
    pub boxing: Option<(V2, V2)>,
    /// The risk zones (and whether each is selected), and the one being drawn (with the
    /// point under the mouse last).
    pub zones: Vec<(ZoneView, bool)>,
    pub zone_drawing: Vec<V3>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(name: &str, kind: &str, x: f64, z: f64) -> Block {
        Block {
            name: name.into(),
            kind: kind.into(),
            pos: [x, 0.0, z],
            size: [1.0, 1.0, 1.0],
            skeletal: false,
            problems: false,
        }
    }

    /// A line seen from above and in front: chegada, espera, processo, saída on `x`, an
    /// operator behind.
    fn editor() -> Editor3d {
        let mut e = Editor3d::new();
        e.camera = Camera::look_at([6.0, 16.0, 10.0], [6.0, 0.0, 0.0], 50.0, [1600.0, 900.0]);
        e.set_scene(
            vec![
                block("chegada", "Source", 0.0, 0.0),
                block("espera", "Queue", 4.0, 0.0),
                block("processo", "Processor", 8.0, 0.0),
                block("saida", "Sink", 12.0, 0.0),
                block("operador", "Transporter", 6.0, -4.0),
            ],
            vec![Conn {
                from: "chegada".into(),
                to: "espera".into(),
                port: 1,
                logical: false,
            }],
            Vec::new(),
        );
        e
    }

    fn screen(e: &Editor3d, w: V3) -> V2 {
        e.camera.project(w).expect("on screen")
    }

    fn click(e: &mut Editor3d, p: V2, shift: bool) -> Vec<Effect> {
        let mut out = e.press(p, shift);
        out.extend(e.release(p));
        out
    }

    fn drag(e: &mut Editor3d, a: V2, b: V2) -> Vec<Effect> {
        let mut out = e.press(a, false);
        for k in 1..=8 {
            let t = k as f64 / 8.0;
            out.extend(e.drag([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]));
        }
        out.extend(e.release(b));
        out
    }

    #[test]
    fn the_camera_projects_and_casts_rays_back_to_the_same_floor_point() {
        let e = editor();
        for w in [
            [0.0, 0.0, 0.0],
            [12.0, 0.0, 0.0],
            [6.0, 0.0, -4.0],
            [3.3, 0.0, 1.7],
        ] {
            let s = screen(&e, w);
            let f = e.camera.floor_at(s).unwrap();
            assert!(len(sub(f, w)) < 1e-6, "{w:?} -> {s:?} -> {f:?}");
        }
        // Behind the camera: nothing.
        assert!(e.camera.project([6.0, 40.0, 30.0]).is_none());
        // Godot's matrices, column by column, give the same camera.
        let col = |m: &M4| (0..16).map(|k| m[k % 4][k / 4]).collect::<Vec<f64>>();
        let c = Camera::from_columns(&col(&e.camera.view), &col(&e.camera.proj), e.camera.size);
        assert_eq!(c, e.camera);
        let m = [
            [2.0, 1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 3.0],
            [1.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let i = inverse(&m).unwrap();
        let id = mul(&m, &i);
        for r in 0..4 {
            for c in 0..4 {
                assert!((id[r][c] - IDENTITY[r][c]).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn blocks_and_nodes_are_picked_where_they_are_drawn() {
        let e = editor();
        let p = e.block_screen("processo").unwrap();
        assert_eq!(e.hits_at(p), vec!["processo".to_owned()]);
        assert!(e.hits_at([5.0, 5.0]).is_empty());
        let out = e.block("processo").unwrap().port_out().unwrap();
        assert_eq!(
            e.node_at(screen(&e, out)),
            Some(("processo".into(), Side::Out))
        );
        let inp = e.block("saida").unwrap().port_in().unwrap();
        assert_eq!(e.node_at(screen(&e, inp)), Some(("saida".into(), Side::In)));
        // A source has no input, a sink no output; a resource has only the service node.
        assert!(e.block("chegada").unwrap().port_in().is_none());
        assert!(e.block("saida").unwrap().port_out().is_none());
        let op = e.block("operador").unwrap();
        assert!(op.port_out().is_none() && op.port_in().is_none());
        assert_eq!(
            e.node_at(screen(&e, op.port_service().unwrap())),
            Some(("operador".into(), Side::Service))
        );
        // The connection is picked on its line.
        let s = &e.segments()[0];
        let m = screen(&e, lerp(s.a, s.b, 0.5));
        assert_eq!(e.link_at(m), Some(("chegada".into(), "espera".into())));
    }

    #[test]
    fn click_selects_shift_adds_floor_clears_and_the_box_selects_what_is_inside() {
        let mut e = editor();
        let p = e.block_screen("espera").unwrap();
        let out = click(&mut e, p, false);
        assert_eq!(e.selected, vec!["espera".to_owned()]);
        assert!(out.contains(&Effect::Selection));
        let q = e.block_screen("processo").unwrap();
        click(&mut e, q, true);
        assert_eq!(e.selected, vec!["processo".to_owned(), "espera".to_owned()]);
        click(&mut e, [5.0, 5.0], false);
        assert!(e.selected.is_empty());
        // A box around espera and processo, not the others.
        let a = e.block_screen("espera").unwrap();
        let b = e.block_screen("processo").unwrap();
        let out = drag(
            &mut e,
            [a[0] - 40.0, a[1] - 60.0],
            [b[0] + 40.0, b[1] + 60.0],
        );
        assert_eq!(e.selected, vec!["espera".to_owned(), "processo".to_owned()]);
        assert!(out.contains(&Effect::Status("2 objeto(s) na caixa.".into())));
        assert!(e.boxing.is_none());
    }

    #[test]
    fn dragging_a_block_moves_the_selection_by_the_floor_delta() {
        let mut e = editor();
        let a = e.block_screen("processo").unwrap();
        let target = screen(&e, [10.0, 0.0, 1.0]);
        let out = drag(&mut e, a, target);
        assert_eq!(out.first(), Some(&Effect::Selection));
        assert!(out.contains(&Effect::MoveBegin(vec!["processo".into()])));
        assert!(out.contains(&Effect::MoveEnd));
        let last = out
            .iter()
            .rev()
            .find_map(|x| {
                if let Effect::Move(d) = x {
                    Some(*d)
                } else {
                    None
                }
            })
            .unwrap();
        // The press was on the box, not the floor point under the centre: the delta is the
        // floor's, about (2, 0, 1) from where it was grabbed.
        let grabbed = e.camera.floor_at(a).unwrap();
        let want = sub([10.0, 0.0, 1.0], grabbed);
        assert!(
            len(sub(last, [want[0], 0.0, want[2]])) < 1e-6,
            "{last:?} {want:?}"
        );
    }

    #[test]
    fn nodes_connect_both_ways_and_a_click_selects_the_link_that_the_x_removes() {
        let mut e = editor();
        let from = screen(&e, e.block("espera").unwrap().port_out().unwrap());
        let to = screen(&e, e.block("processo").unwrap().port_in().unwrap());
        let out = drag(&mut e, from, to);
        assert!(out.contains(&Effect::Connect {
            from: "espera".into(),
            to: "processo".into(),
            ok: "Ligado: espera → processo.".into()
        }));
        assert!(e.connect_from.is_empty());
        // Backwards: from the input node of saida to processo's block.
        let inp = screen(&e, e.block("saida").unwrap().port_in().unwrap());
        let q = e.block_screen("processo").unwrap();
        let out = drag(&mut e, inp, q);
        assert!(out.iter().any(
            |x| matches!(x, Effect::Connect { from, to, .. } if from == "processo" && to == "saida")
        ));
        // A click on chegada's output node selects its link; the × removes it.
        let node = screen(&e, e.block("chegada").unwrap().port_out().unwrap());
        click(&mut e, node, false);
        assert_eq!(e.selected_link, Some(("chegada".into(), "espera".into())));
        let x = screen(&e, e.remove_point().unwrap());
        let out = click(&mut e, x, false);
        assert!(
            out.iter()
                .any(|x| matches!(x, Effect::Disconnect { from, .. } if from == "chegada"))
        );
        assert!(e.selected_link.is_none());
        // A node without links only explains.
        let lonely = screen(&e, e.block("processo").unwrap().port_out().unwrap());
        let out = click(&mut e, lonely, false);
        assert!(
            matches!(&out[..], [Effect::Status(_), Effect::Status(s)] if s.contains("arraste deste nó verde"))
        );
    }

    #[test]
    fn the_service_node_operates_a_machine_carries_from_a_wait_and_never_undoes_by_dragging() {
        let mut e = editor();
        let node = screen(&e, e.block("operador").unwrap().port_service().unwrap());
        let q = e.block_screen("processo").unwrap();
        let out = drag(&mut e, node, q);
        assert!(out.contains(&Effect::Team {
            from: "processo".into(),
            to: "operador".into(),
            operators: true,
            ok: "Ligado: operador opera processo. Ctrl+Z desfaz.".into(),
            removed: false
        }));
        let q = e.block_screen("espera").unwrap();
        let out = drag(&mut e, node, q);
        assert!(
            out.iter().any(
                |x| matches!(x, Effect::Team { from, operators: false, .. } if from == "espera")
            )
        );
        // Once the line exists, dragging again only says so.
        let line = TeamLine {
            kind: "operadores".into(),
            from: "processo".into(),
            to: "operador".into(),
        };
        let blocks = e.blocks.clone();
        let conns = e.conns.clone();
        e.set_scene(blocks, conns, vec![line.clone()]);
        let q = e.block_screen("processo").unwrap();
        let out = drag(&mut e, node, q);
        assert!(matches!(out.last(), Some(Effect::Status(s)) if s.starts_with("Já está ligado")));
        // A click on the line selects it; Del removes it with the same team_link.
        let (a, b) = e.team_ends(&line).unwrap();
        let q = screen(&e, lerp(a, b, 0.5));
        click(&mut e, q, false);
        assert_eq!(e.selected_team, Some(line));
        let out = e.delete();
        assert!(matches!(
            &out[..],
            [Effect::Team {
                removed: true,
                operators: true,
                ..
            }]
        ));
    }

    #[test]
    fn the_zone_tool_draws_selects_reshapes_moves_and_deletes_a_zone() {
        let mut e = editor();
        e.set_tool("zona");
        e.snap = 1.0;
        // Four clicks and a click on the first point: one zone, snapped to the grid.
        let corners = [
            [1.1, 0.0, 2.0],
            [3.0, 0.0, 2.2],
            [3.0, 0.0, 4.0],
            [1.0, 0.0, 4.0],
        ];
        for c in corners {
            let q = screen(&e, c);
            e.press(q, false);
            e.release(q);
        }
        assert_eq!(e.zone_points.len(), 4);
        assert_eq!(e.zone_points[0], [1.0, 0.0, 2.0]);
        let q = screen(&e, corners[0]);
        let out = e.press(q, false);
        let pts = match &out[..] {
            [Effect::ZoneAdd(p)] => p.clone(),
            other => panic!("{other:?}"),
        };
        assert_eq!(pts.len(), 4);
        assert!(e.zone_points.is_empty());
        e.set_zones(vec![ZoneView {
            name: "zona_1".into(),
            points: pts.clone(),
            level: Some(3),
        }]);
        // A click inside selects it; dragging the middle moves it by whole steps.
        let inside = screen(&e, [2.0, 0.0, 3.0]);
        let out = click(&mut e, inside, false);
        assert_eq!(e.selected_zone, "zona_1");
        assert!(out.contains(&Effect::Selection));
        let q = screen(&e, [4.0, 0.0, 3.0]);
        let out = drag(&mut e, inside, q);
        let moved = match &out[..] {
            [Effect::ZoneSet { name, points }, Effect::Status(_)] if name == "zona_1" => {
                points.clone()
            }
            other => panic!("{other:?}"),
        };
        assert_eq!(moved[0], [3.0, 0.0, 2.0]);
        // A vertex is dragged on its own.
        e.set_zones(vec![ZoneView {
            name: "zona_1".into(),
            points: moved.clone(),
            level: Some(3),
        }]);
        let v = screen(&e, moved[2]);
        let q = screen(&e, [6.0, 0.0, 6.0]);
        let out = drag(&mut e, v, q);
        assert!(
            matches!(&out[..], [Effect::ZoneSet { points, .. }, _] if points[2] == [6.0, 0.0, 6.0] && points[0] == moved[0])
        );
        // Fewer than three points do not close; Backspace takes one back; Esc cancels.
        let a = screen(&e, [10.0, 0.0, -3.0]);
        e.press(a, false);
        e.release(a);
        assert!(
            e.selected_zone.is_empty(),
            "a click outside deselects and starts a zone"
        );
        assert!(matches!(&e.zone_finish()[..], [Effect::Status(_)]));
        e.zone_back();
        assert!(e.zone_points.is_empty());
        // Del removes the selected zone; Esc deselects, then leaves the tool.
        let q = inside_of(&e, &moved);
        click(&mut e, q, false);
        assert_eq!(
            e.delete(),
            vec![Effect::ZoneRemove("zona_1".into()), Effect::Selection]
        );
        assert!(e.cancel().0);
    }

    #[test]
    fn the_rede_tool_places_nodes_joins_them_moves_and_deletes() {
        let mut e = editor();
        e.set_tool("rede");
        e.snap = 1.0;
        // A click on the floor: a node there (the script places it and says its name).
        let a = screen(&e, [0.2, 0.0, 6.1]);
        e.press(a, false);
        assert_eq!(e.release(a), vec![Effect::NetAddNode([0.0, 0.0, 6.0])]);
        e.net_placed("n1");
        assert_eq!((e.armed.as_str(), e.net_node.as_str()), ("n1", "n1"));
        e.set_network(
            vec![
                ("n1".into(), [0.0, 0.0, 6.0]),
                ("n2".into(), [6.0, 0.0, 6.0]),
            ],
            vec![],
        );
        // A click on another node joins the armed one to it and arms it.
        let b = screen(&e, [6.0, 0.0, 6.0]);
        e.press(b, false);
        let out = e.release(b);
        assert_eq!(
            out,
            vec![
                Effect::NetAddEdge {
                    from: "n1".into(),
                    to: "n2".into()
                },
                Effect::Selection
            ]
        );
        assert_eq!(e.armed, "n2");
        e.set_network(
            vec![
                ("n1".into(), [0.0, 0.0, 6.0]),
                ("n2".into(), [6.0, 0.0, 6.0]),
            ],
            vec![("n1".into(), "n2".into())],
        );
        // Dragging a node moves it, snapped; the line follows while dragging.
        e.press(b, false);
        let to = screen(&e, [7.2, 0.0, 4.9]);
        for k in 1..=6 {
            let t = k as f64 / 6.0;
            e.drag([b[0] + (to[0] - b[0]) * t, b[1] + (to[1] - b[1]) * t]);
        }
        assert_eq!(e.net_position("n2"), Some([7.0, 0.0, 5.0]));
        assert_eq!(
            e.release(to),
            vec![Effect::NetMoveNode {
                name: "n2".into(),
                at: [7.0, 0.0, 5.0]
            }]
        );
        // A click on the edge selects it; Del removes it; Esc stops the path once.
        let m = screen(&e, [3.0, 0.0, 6.0]);
        let out = click(&mut e, m, false);
        assert!(out.contains(&Effect::Selection));
        assert_eq!(e.net_edge, Some(("n1".into(), "n2".into())));
        assert!(matches!(
            &e.net_delete()[..],
            [Effect::NetRemoveEdge { .. }, Effect::Selection]
        ));
        assert!(e.net_cancel());
        assert!(!e.net_cancel());
    }

    #[test]
    fn colocar_places_where_clicked_and_equipe_links_by_drag_or_two_clicks() {
        let mut e = editor();
        e.set_tool("colocar");
        e.place_id = "Queue".into();
        let q = screen(&e, [2.0, 0.0, 3.0]);
        let out = e.press(q, true);
        assert!(
            matches!(&out[..], [Effect::Place { id, keep: true, at }] if id == "Queue" && len(sub(*at, [2.0, 0.0, 3.0])) < 1e-6)
        );
        // Equipe: a drag from a machine to the operator (Shift: its operators).
        e.set_tool("equipe");
        let a = e.block_screen("processo").unwrap();
        let b = e.block_screen("operador").unwrap();
        let mut out = e.press(a, true);
        for k in 1..=6 {
            let t = k as f64 / 6.0;
            out.extend(e.drag([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]));
        }
        out.extend(e.release(b));
        assert!(
            matches!(&out[..], [Effect::Team { from, to, operators: true, .. }] if from == "processo" && to == "operador")
        );
        // Two clicks; Alt turns Ligar into Equipe.
        e.set_tool("ligar");
        e.alt = true;
        click(&mut e, a, false);
        assert_eq!(e.team_from, "processo");
        let out = click(&mut e, b, false);
        assert!(matches!(
            &out[..],
            [Effect::Team {
                operators: false,
                ..
            }]
        ));
        e.alt = false;
        click(&mut e, a, false);
        assert!(!e.team_cancel(), "without Alt the press went to Ligar");
    }

    #[test]
    fn the_corredor_tool_draws_selects_moves_bends_and_deletes() {
        let mut e = editor();
        e.set_tool("corredor");
        // Three points (the script snaps them), the third twice: the aisle ends.
        for w in [
            [0.0, 0.0, 2.0],
            [4.0, 0.0, 2.0],
            [4.0, 0.0, 5.0],
            [4.0, 0.0, 5.0],
        ] {
            let q = screen(&e, w);
            e.aisle_press_at(q, false, Some(w));
            let out = e.aisle_release();
            if e.aisle_points.is_empty() {
                assert!(matches!(&out[..], [Effect::AisleAdd(p)] if p.len() == 3));
            }
        }
        assert!(e.aisle_points.is_empty());
        e.set_aisles(vec![(
            "corredor_1".into(),
            vec![[0.0, 0.0, 2.0], [4.0, 0.0, 2.0], [4.0, 0.0, 5.0]],
        )]);
        // A click on the line selects; on a vertex too; dragging a vertex moves it.
        let m = screen(&e, [2.0, 0.0, 2.0]);
        e.aisle_press_at(m, false, None);
        e.aisle_release();
        assert_eq!((e.aisle.as_str(), e.aisle_vertex), ("corredor_1", None));
        let v = screen(&e, [4.0, 0.0, 5.0]);
        e.aisle_press_at(v, false, None);
        assert_eq!(e.aisle_dragging(), Some(("corredor_1".into(), 2)));
        let to = screen(&e, [6.0, 0.0, 6.0]);
        e.aisle_drag_to(to, Some([6.0, 0.0, 6.0]));
        assert_eq!(e.aisle_vertex_pos("corredor_1", 2), Some([6.0, 0.0, 6.0]));
        assert_eq!(
            e.aisle_release(),
            vec![Effect::AisleMoveVertex {
                name: "corredor_1".into(),
                index: 2,
                at: [6.0, 0.0, 6.0]
            }]
        );
        // Shift on the selected line adds a bend; Del removes the aisle; Esc deselects.
        e.aisle_press_at(m, true, Some([2.0, 0.0, 2.0]));
        assert!(matches!(
            &e.aisle_release()[..],
            [Effect::AisleInsert { segment: 0, .. }]
        ));
        assert_eq!(
            e.aisle_delete(),
            vec![Effect::AisleRemove("corredor_1".into())]
        );
        assert!(!e.aisle_cancel());
        // Backspace while drawing takes the last point; a single point does not make one.
        e.set_aisles(Vec::new());
        e.aisle_press_at(m, false, Some([9.0, 0.0, 9.0]));
        e.aisle_release();
        assert!(matches!(
            &e.aisle_delete()[..],
            [Effect::Selection, Effect::Status(_)]
        ));
        e.aisle_press_at(m, false, Some([9.0, 0.0, 9.0]));
        e.aisle_release();
        assert!(matches!(&e.aisle_finish()[..], [Effect::Status(_)]));
    }

    fn inside_of(e: &Editor3d, pts: &[V3]) -> V2 {
        let n = pts.len() as f64;
        let c = pts.iter().fold([0.0; 3], |a, v| add(a, *v));
        screen(e, scale(c, 1.0 / n))
    }

    #[test]
    fn the_overlay_lights_the_node_under_the_mouse_and_marks_the_selection() {
        let mut e = editor();
        let node = e.block("espera").unwrap().port_out().unwrap();
        e.hover(screen(&e, node));
        assert_eq!(e.hover_node, Some(("espera".into(), Side::Out)));
        let q = e.block_screen("processo").unwrap();
        click(&mut e, q, false);
        let o = e.overlay();
        assert!(
            o.nodes
                .iter()
                .any(|n| n.lit && n.side == Side::Out && n.at == node)
        );
        assert_eq!(o.corners.iter().filter(|c| c.2 == Mark::Primary).count(), 1);
        // Nodes: chegada out, espera in+out, processo in+out, saida in, operador service.
        assert_eq!(o.nodes.len(), 7);
        // Esc clears the selection; with the Ligar tool it asks to leave it.
        assert!(!e.cancel().0);
        assert!(e.selected.is_empty());
        e.set_tool("ligar");
        assert!(e.cancel().0);
    }
}
