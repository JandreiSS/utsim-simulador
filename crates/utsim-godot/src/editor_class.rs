//! `UtsimEditor`: the interaction of Editar in the 3D view for the `editor.gd`, a thin
//! conversion over [`crate::editor3d`] (step 6 of `docs/plano_interface_rust.md`). The
//! script hands it the camera, the scene and the mouse; it answers with effects (as
//! dictionaries, `op` = `ligar`, `desligar`, `equipe`, `selecao`, `status`, `mover_inicio`,
//! `mover`, `mover_fim`, `remover`) and with what to draw ([`UtsimEditor::overlay`]).

use godot::prelude::*;

use crate::editor3d::{
    Block, Camera, Conn, Editor3d, Effect, Mark, PreviewKind, Side, TeamLine, V2, V3, ZoneView,
};

fn v2(p: Vector2) -> V2 {
    [p.x as f64, p.y as f64]
}
fn v3(p: Vector3) -> V3 {
    [p.x as f64, p.y as f64, p.z as f64]
}
fn g3(p: V3) -> Vector3 {
    Vector3::new(p[0] as f32, p[1] as f32, p[2] as f32)
}
fn g2(p: V2) -> Vector2 {
    Vector2::new(p[0] as f32, p[1] as f32)
}
fn side_of(s: &str) -> Option<Side> {
    match s {
        "out" => Some(Side::Out),
        "in" => Some(Side::In),
        "service" => Some(Side::Service),
        _ => None,
    }
}
fn s(d: &VarDictionary, k: &str) -> String {
    d.get(k).map(|v| v.to_string()).unwrap_or_default()
}
fn opt3(p: Option<V3>) -> Variant {
    p.map_or(Variant::nil(), |p| g3(p).to_variant())
}
fn opt2(p: Option<V2>) -> Variant {
    p.map_or(Variant::nil(), |p| g2(p).to_variant())
}
fn team_dict(t: &TeamLine) -> VarDictionary {
    let mut d = VarDictionary::new();
    d.set("kind", &GString::from(t.kind.as_str()));
    d.set("from", &GString::from(t.from.as_str()));
    d.set("to", &GString::from(t.to.as_str()));
    d
}
fn names(v: &[String]) -> PackedStringArray {
    v.iter().map(|n| GString::from(n.as_str())).collect()
}

fn pts(v: &[V3]) -> PackedVector3Array {
    v.iter().map(|p| g3(*p)).collect()
}

fn effects(list: Vec<Effect>) -> VarArray {
    let mut out = VarArray::new();
    for e in list {
        let mut d = VarDictionary::new();
        match e {
            Effect::Connect { from, to, ok } | Effect::Disconnect { from, to, ok } => {
                let op = if ok.starts_with("Ligado") {
                    "ligar"
                } else {
                    "desligar"
                };
                d.set("op", &GString::from(op));
                d.set("from", &GString::from(from.as_str()));
                d.set("to", &GString::from(to.as_str()));
                d.set("ok", &GString::from(ok.as_str()));
            }
            Effect::Team {
                from,
                to,
                operators,
                ok,
                removed,
            } => {
                d.set("op", &GString::from("equipe"));
                d.set("from", &GString::from(from.as_str()));
                d.set("to", &GString::from(to.as_str()));
                d.set("operators", operators);
                d.set("ok", &GString::from(ok.as_str()));
                d.set("removed", removed);
            }
            Effect::Selection => d.set("op", &GString::from("selecao")),
            Effect::Status(t) => {
                d.set("op", &GString::from("status"));
                d.set("text", &GString::from(t.as_str()));
            }
            Effect::MoveBegin(n) => {
                d.set("op", &GString::from("mover_inicio"));
                d.set("names", &names(&n));
            }
            Effect::Move(delta) => {
                d.set("op", &GString::from("mover"));
                d.set("delta", g3(delta));
            }
            Effect::MoveEnd => d.set("op", &GString::from("mover_fim")),
            Effect::Remove(n) => {
                d.set("op", &GString::from("remover"));
                d.set("names", &names(&n));
            }
            Effect::ZoneAdd(points) => {
                d.set("op", &GString::from("zona_nova"));
                d.set("points", &pts(&points));
            }
            Effect::ZoneSet { name, points } => {
                d.set("op", &GString::from("zona_forma"));
                d.set("name", &GString::from(name.as_str()));
                d.set("points", &pts(&points));
            }
            Effect::NetAddNode(at) => {
                d.set("op", &GString::from("rede_no"));
                d.set("at", g3(at));
            }
            Effect::NetMoveNode { name, at } => {
                d.set("op", &GString::from("rede_mover"));
                d.set("name", &GString::from(name.as_str()));
                d.set("at", g3(at));
            }
            Effect::NetAddEdge { from, to } => {
                d.set("op", &GString::from("rede_trecho"));
                d.set("from", &GString::from(from.as_str()));
                d.set("to", &GString::from(to.as_str()));
            }
            Effect::NetRemoveEdge { from, to } => {
                d.set("op", &GString::from("rede_trecho_excluir"));
                d.set("from", &GString::from(from.as_str()));
                d.set("to", &GString::from(to.as_str()));
            }
            Effect::NetRemoveNode(name) => {
                d.set("op", &GString::from("rede_no_excluir"));
                d.set("name", &GString::from(name.as_str()));
            }
            Effect::ZoneRemove(name) => {
                d.set("op", &GString::from("zona_excluir"));
                d.set("name", &GString::from(name.as_str()));
            }
        }
        out.push(&d.to_variant());
    }
    out
}

/// The editor of the 3D view (see the module).
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimEditor {
    ed: Editor3d,
    base: Base<RefCounted>,
}

#[godot_api]
impl UtsimEditor {
    /// The camera: `view` is the camera's global transform inverted, `proj` its
    /// `get_camera_projection()`, `size` the viewport's.
    #[func]
    fn set_camera(&mut self, view: Transform3D, proj: Projection, size: Vector2) {
        let mut v = [[0.0; 4]; 4];
        for (r, row) in v.iter_mut().take(3).enumerate() {
            let b = view.basis.rows[r];
            *row = [b.x as f64, b.y as f64, b.z as f64, 0.0];
        }
        v[0][3] = view.origin.x as f64;
        v[1][3] = view.origin.y as f64;
        v[2][3] = view.origin.z as f64;
        v[3] = [0.0, 0.0, 0.0, 1.0];
        let mut p = [[0.0; 4]; 4];
        for (c, col) in proj.cols.iter().enumerate() {
            p[0][c] = col.x as f64;
            p[1][c] = col.y as f64;
            p[2][c] = col.z as f64;
            p[3][c] = col.w as f64;
        }
        self.ed.camera = Camera {
            view: v,
            proj: p,
            size: v2(size),
        };
    }

    /// The scene: `blocks` ({name, kind, pos, size, skeletal, problems}), `conns` ({from,
    /// to, port, logical}), `teams` ({kind, from, to}) and the team kinds hidden.
    #[func]
    fn set_scene(
        &mut self,
        blocks: VarArray,
        conns: VarArray,
        teams: VarArray,
        hidden: PackedStringArray,
    ) {
        if self.ed.tool.is_empty() {
            self.ed = Editor3d::new();
        }
        let dicts = |a: &VarArray| -> Vec<VarDictionary> {
            a.iter_shared()
                .filter_map(|v| v.try_to::<VarDictionary>().ok())
                .collect()
        };
        let blocks = dicts(&blocks)
            .iter()
            .map(|d| Block {
                name: s(d, "name"),
                kind: s(d, "kind"),
                pos: d
                    .get("pos")
                    .and_then(|v| v.try_to::<Vector3>().ok())
                    .map_or([0.0; 3], v3),
                size: d
                    .get("size")
                    .and_then(|v| v.try_to::<Vector3>().ok())
                    .map_or([1.0; 3], v3),
                skeletal: d
                    .get("skeletal")
                    .and_then(|v| v.try_to::<bool>().ok())
                    .unwrap_or(false),
                problems: d
                    .get("problems")
                    .and_then(|v| v.try_to::<bool>().ok())
                    .unwrap_or(false),
            })
            .collect();
        let conns = dicts(&conns)
            .iter()
            .map(|d| Conn {
                from: s(d, "from"),
                to: s(d, "to"),
                port: d
                    .get("port")
                    .and_then(|v| v.try_to::<i64>().ok())
                    .unwrap_or(0),
                logical: d
                    .get("logical")
                    .and_then(|v| v.try_to::<bool>().ok())
                    .unwrap_or(false),
            })
            .collect();
        let teams = dicts(&teams)
            .iter()
            .map(|d| TeamLine {
                kind: s(d, "kind"),
                from: s(d, "from"),
                to: s(d, "to"),
            })
            .collect();
        self.ed.hidden = hidden.as_slice().iter().map(|g| g.to_string()).collect();
        self.ed.set_scene(blocks, conns, teams);
    }

    /// The risk zones: {name, points (floor, Godot), level (0 = none)}.
    #[func]
    fn set_zones(&mut self, zones: VarArray) {
        let list = zones
            .iter_shared()
            .filter_map(|v| v.try_to::<VarDictionary>().ok())
            .map(|d| ZoneView {
                name: s(&d, "name"),
                points: d
                    .get("points")
                    .and_then(|v| v.try_to::<PackedVector3Array>().ok())
                    .map(|a| a.as_slice().iter().map(|p| v3(*p)).collect())
                    .unwrap_or_default(),
                level: d
                    .get("level")
                    .and_then(|v| v.try_to::<i64>().ok())
                    .filter(|l| *l > 0)
                    .map(|l| l as u8),
            })
            .collect();
        self.ed.set_zones(list);
    }

    /// The path network: `nodes` ({name, position}) and `edges` ({from, to}).
    #[func]
    fn set_network(&mut self, nodes: VarArray, edges: VarArray) {
        let dicts = |a: &VarArray| -> Vec<VarDictionary> {
            a.iter_shared()
                .filter_map(|v| v.try_to::<VarDictionary>().ok())
                .collect()
        };
        let nodes = dicts(&nodes)
            .iter()
            .map(|d| {
                let at = d
                    .get("position")
                    .and_then(|v| v.try_to::<Vector3>().ok())
                    .map_or([0.0; 3], v3);
                (s(d, "name"), at)
            })
            .collect();
        let edges = dicts(&edges)
            .iter()
            .map(|d| (s(d, "from"), s(d, "to")))
            .collect();
        self.ed.set_network(nodes, edges);
    }

    /// The network state for the script: `{armed, node, edge ([from, to] or []), drag (the
    /// node dragged and where, or {})}`.
    #[func]
    fn network_state(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        d.set("armed", &GString::from(self.ed.armed.as_str()));
        d.set("node", &GString::from(self.ed.net_node.as_str()));
        let edge = self
            .ed
            .net_edge
            .as_ref()
            .map_or(PackedStringArray::new(), |(a, b)| {
                names(&[a.clone(), b.clone()])
            });
        d.set("edge", &edge);
        d
    }

    /// Where node `name` is drawn now, or null.
    #[func]
    fn net_position(&self, name: GString) -> Variant {
        opt3(self.ed.net_position(&name.to_string()))
    }

    #[func]
    fn net_node_screen(&self, name: GString) -> Variant {
        opt2(
            self.ed
                .net_position(&name.to_string())
                .and_then(|w| self.ed.camera.project(w)),
        )
    }

    #[func]
    fn net_placed(&mut self, name: GString) {
        self.ed.net_placed(&name.to_string());
    }

    /// Selects a node or an edge from the script (the panel renames, reverses).
    #[func]
    fn net_select(&mut self, node: GString, edge: PackedStringArray) {
        self.ed.net_node = node.to_string();
        let e: Vec<String> = edge.as_slice().iter().map(|g| g.to_string()).collect();
        self.ed.net_edge = (e.len() == 2).then(|| (e[0].clone(), e[1].clone()));
    }

    #[func]
    fn set_armed(&mut self, name: GString) {
        self.ed.armed = name.to_string();
    }

    #[func]
    fn net_cancel(&mut self) -> bool {
        self.ed.net_cancel()
    }

    #[func]
    fn net_delete(&mut self) -> VarArray {
        effects(self.ed.net_delete())
    }

    /// The grid step new points snap to (0: none).
    #[func]
    fn set_snap(&mut self, step: f64) {
        self.ed.snap = step;
    }

    /// The next press is a double click.
    #[func]
    fn set_double(&mut self, on: bool) {
        self.ed.double = on;
    }

    #[func]
    fn zone_finish(&mut self) -> VarArray {
        effects(self.ed.zone_finish())
    }

    #[func]
    fn zone_back(&mut self) {
        self.ed.zone_back();
    }

    #[func]
    fn selected_zone(&self) -> GString {
        self.ed.selected_zone.as_str().into()
    }

    #[func]
    fn select_zone(&mut self, name: GString) {
        self.ed.selected_zone = name.to_string();
    }

    #[func]
    fn zone_drawing_count(&self) -> i64 {
        self.ed.zone_points.len() as i64
    }

    #[func]
    fn zone_vertex_screen(&self, name: GString, k: i64) -> Variant {
        opt2(
            self.ed
                .zone_vertex_screen(&name.to_string(), k.max(0) as usize),
        )
    }

    #[func]
    fn zone_screen(&self, name: GString) -> Variant {
        opt2(self.ed.zone_screen(&name.to_string()))
    }

    /// Where block `name` is drawn now (during a move).
    #[func]
    fn set_position(&mut self, name: GString, pos: Vector3) {
        self.ed.set_position(&name.to_string(), v3(pos));
    }

    #[func]
    fn set_tool(&mut self, tool: GString) {
        if self.ed.tool.is_empty() {
            self.ed = Editor3d::new();
        }
        self.ed.set_tool(&tool.to_string());
    }

    #[func]
    fn hover(&mut self, p: Vector2) {
        self.ed.hover(v2(p));
    }
    #[func]
    fn press(&mut self, p: Vector2, shift: bool) -> VarArray {
        effects(self.ed.press(v2(p), shift))
    }
    #[func]
    fn drag(&mut self, p: Vector2) -> VarArray {
        effects(self.ed.drag(v2(p)))
    }
    #[func]
    fn release(&mut self, p: Vector2) -> VarArray {
        effects(self.ed.release(v2(p)))
    }
    #[func]
    fn delete(&mut self) -> VarArray {
        effects(self.ed.delete())
    }
    /// Esc: `{sair_da_ferramenta, efeitos}`.
    #[func]
    fn cancel(&mut self) -> VarDictionary {
        let (leave, list) = self.ed.cancel();
        let mut d = VarDictionary::new();
        d.set("sair_da_ferramenta", leave);
        d.set("efeitos", &effects(list));
        d
    }

    // ----- selection ----------------------------------------------------------------------

    #[func]
    fn selected(&self) -> PackedStringArray {
        names(&self.ed.selected)
    }
    #[func]
    fn select_only(&mut self, name: GString) -> VarArray {
        let mut out = Vec::new();
        self.ed.select_only(&name.to_string(), &mut out);
        effects(out)
    }
    #[func]
    fn select_names(&mut self, list: PackedStringArray, add: bool) {
        let v: Vec<String> = list.as_slice().iter().map(|g| g.to_string()).collect();
        self.ed.select_names(&v, add);
    }
    /// The selection as the script has it (a rename, an undo), keeping the connection or
    /// line selected.
    #[func]
    fn set_selected(&mut self, list: PackedStringArray) {
        self.ed.selected = list.as_slice().iter().map(|g| g.to_string()).collect();
    }
    #[func]
    fn toggle(&mut self, name: GString) {
        self.ed.toggle(&name.to_string());
    }
    #[func]
    fn clear_selection(&mut self) {
        self.ed.clear_selection();
    }
    /// The connection selected, `[from, to]` or empty.
    #[func]
    fn selected_link(&self) -> PackedStringArray {
        self.ed
            .selected_link
            .as_ref()
            .map_or(PackedStringArray::new(), |(a, b)| {
                names(&[a.clone(), b.clone()])
            })
    }
    #[func]
    fn selected_team(&self) -> VarDictionary {
        self.ed
            .selected_team
            .as_ref()
            .map_or(VarDictionary::new(), team_dict)
    }
    #[func]
    fn connect_from(&self) -> GString {
        self.ed.connect_from.as_str().into()
    }
    #[func]
    fn connect_to(&self) -> GString {
        self.ed.connect_to.as_str().into()
    }
    #[func]
    fn serve_from(&self) -> GString {
        self.ed.serve_from.as_str().into()
    }
    #[func]
    fn hover_name(&self) -> GString {
        self.ed.hover.as_str().into()
    }
    /// The node under the mouse, `[block, "out"|"in"|"service"]` or empty.
    #[func]
    fn hover_node(&self) -> VarArray {
        let mut a = VarArray::new();
        if let Some((n, side)) = &self.ed.hover_node {
            a.push(&GString::from(n.as_str()).to_variant());
            a.push(&GString::from(side.key()).to_variant());
        }
        a
    }
    #[func]
    fn busy(&self) -> bool {
        self.ed.boxing.is_some()
    }

    // ----- queries ------------------------------------------------------------------------

    #[func]
    fn hits_at(&self, p: Vector2) -> PackedStringArray {
        names(&self.ed.hits_at(v2(p)))
    }
    #[func]
    fn floor_at(&self, p: Vector2) -> Variant {
        opt3(self.ed.camera.floor_at(v2(p)))
    }
    #[func]
    fn project(&self, w: Vector3) -> Variant {
        opt2(self.ed.camera.project(v3(w)))
    }
    #[func]
    fn block_screen(&self, name: GString) -> Variant {
        opt2(self.ed.block_screen(&name.to_string()))
    }
    /// A node of block `name` (`out`, `in` or `service`), or null.
    #[func]
    fn node_pos(&self, name: GString, side: GString) -> Variant {
        let Some(side) = side_of(&side.to_string()) else {
            return Variant::nil();
        };
        opt3(self.ed.node_pos(&name.to_string(), side))
    }
    #[func]
    fn node_at(&self, p: Vector2) -> VarArray {
        let mut a = VarArray::new();
        if let Some((n, side)) = self.ed.node_at(v2(p)) {
            a.push(&GString::from(n.as_str()).to_variant());
            a.push(&GString::from(side.key()).to_variant());
        }
        a
    }
    #[func]
    fn link_at(&self, p: Vector2) -> PackedStringArray {
        self.ed
            .link_at(v2(p))
            .map_or(PackedStringArray::new(), |(a, b)| names(&[a, b]))
    }
    #[func]
    fn team_line_at(&self, p: Vector2) -> VarDictionary {
        self.ed
            .team_line_at(v2(p))
            .as_ref()
            .map_or(VarDictionary::new(), team_dict)
    }
    #[func]
    fn has_team_line(&self, kind: GString, from: GString, to: GString) -> bool {
        self.ed
            .has_team_line(&kind.to_string(), &from.to_string(), &to.to_string())
    }
    /// The middle of a team line on screen, or null.
    #[func]
    fn team_line_screen(&self, kind: GString, from: GString, to: GString) -> Variant {
        let l = TeamLine {
            kind: kind.to_string(),
            from: from.to_string(),
            to: to.to_string(),
        };
        opt2(self.ed.team_ends(&l).and_then(|(a, b)| {
            self.ed.camera.project([
                (a[0] + b[0]) / 2.0,
                (a[1] + b[1]) / 2.0,
                (a[2] + b[2]) / 2.0,
            ])
        }))
    }
    #[func]
    fn remove_point(&self) -> Variant {
        opt3(self.ed.remove_point())
    }
    #[func]
    fn remove_screen(&self) -> Variant {
        opt2(
            self.ed
                .remove_point()
                .and_then(|p| self.ed.camera.project(p)),
        )
    }
    /// The connection lines: {from, to, port, logical, a, b}.
    #[func]
    fn segments(&self) -> VarArray {
        let mut out = VarArray::new();
        for sg in self.ed.segments() {
            let mut d = VarDictionary::new();
            d.set("from", &GString::from(sg.from.as_str()));
            d.set("to", &GString::from(sg.to.as_str()));
            d.set("port", sg.port);
            d.set("logical", sg.logical);
            d.set("a", g3(sg.a));
            d.set("b", g3(sg.b));
            out.push(&d.to_variant());
        }
        out
    }

    /// What to draw: `nodes` ([pos, side, lit, edge_x]), `corners` ([min, max, mark]:
    /// `primary`, `other`, `problem`), `remove` (pos or null), `preview` ([a, b, kind] or
    /// empty: `flow`, `service`), `box` (Rect2 or null).
    #[func]
    fn overlay(&self) -> VarDictionary {
        let o = self.ed.overlay();
        let mut d = VarDictionary::new();
        let mut nodes = VarArray::new();
        for n in &o.nodes {
            let mut a = VarArray::new();
            a.push(&g3(n.at).to_variant());
            a.push(&GString::from(n.side.key()).to_variant());
            a.push(&n.lit.to_variant());
            a.push(&(n.edge as f32).to_variant());
            nodes.push(&a.to_variant());
        }
        d.set("nodes", &nodes);
        let mut corners = VarArray::new();
        for (lo, hi, m) in &o.corners {
            let mut a = VarArray::new();
            a.push(&g3(*lo).to_variant());
            a.push(&g3(*hi).to_variant());
            let k = match m {
                Mark::Primary => "primary",
                Mark::Other => "other",
                Mark::Problem => "problem",
            };
            a.push(&GString::from(k).to_variant());
            corners.push(&a.to_variant());
        }
        d.set("corners", &corners);
        d.set("remove", &opt3(o.remove));
        let mut preview = VarArray::new();
        if let Some((a, b, k)) = o.preview {
            preview.push(&g3(a).to_variant());
            preview.push(&g3(b).to_variant());
            let k = match k {
                PreviewKind::Flow => "flow",
                PreviewKind::Service => "service",
            };
            preview.push(&GString::from(k).to_variant());
        }
        d.set("preview", &preview);
        let mut zones = VarArray::new();
        for (z, sel) in &o.zones {
            let mut a = VarDictionary::new();
            a.set("name", &GString::from(z.name.as_str()));
            a.set("points", &pts(&z.points));
            a.set("level", z.level.map_or(0, i64::from));
            a.set("selected", *sel);
            let c = crate::zone_edit::level_color(z.level);
            a.set("color", Color::from_rgb(c[0], c[1], c[2]));
            zones.push(&a.to_variant());
        }
        d.set("zones", &zones);
        d.set("zone_drawing", &pts(&o.zone_drawing));
        d.set(
            "box",
            &o.boxing.map_or(Variant::nil(), |(a, b)| {
                Rect2::new(
                    Vector2::new(a[0].min(b[0]) as f32, a[1].min(b[1]) as f32),
                    Vector2::new((a[0] - b[0]).abs() as f32, (a[1] - b[1]).abs() as f32),
                )
                .to_variant()
            }),
        );
        d
    }
}
