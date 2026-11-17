//! The `Rede` (path network) and `Fluidos` (fluid network) layers of `Editar`.
//!
//! Like the object layer, these systems only turn mouse and keyboard into
//! [`Command`]s applied through the document (undoable, validated live). The panels
//! come from `utsim_viewer::properties::network`; the pure helpers (picking, connectors,
//! fluid layout) from `utsim_viewer::editing`.

use bevy::prelude::*;
use bevy_egui::egui;
use utsim_edit::fluids::{FLUID_PALETTE, fluid_entry, fluid_template};
use utsim_edit::network::NODE_BASE;
use utsim_edit::{Command, EdgeDef, FluidNode, NodeDef};
use utsim_viewer::editing::{
    LINE_PICK_PIXELS, Layer, NODE_PICK_PIXELS, connector_segments, fluid_positions_command,
    ground_to_model_snapped, nearest_point, nearest_segment,
};
use utsim_viewer::fluids::{self, NodeShape};
use utsim_viewer::layout;
use utsim_viewer::properties::network::{
    converter_objects, edge_panel, edge_text, fluid_node_panel, node_panel,
};
use utsim_viewer::texto::{fx, tx, vx};
use utsim_viewer::view;

use crate::camera::CameraLock;
use crate::editor::{Editor, Tool};
use crate::scene;
use crate::ui::Label;

/// Selection of the `Rede` layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetSel {
    Node(String),
    /// An edge, by the ends it is stored with.
    Edge(String, String),
}

/// A left-button gesture of the `Rede` or `Fluidos` layer.
pub struct LayerPress {
    start: Vec2,
    /// Node under the cursor when the button went down, and the ground offset from the
    /// cursor to it (so it does not jump when dragged).
    grabbed: Option<(String, Vec2)>,
    moved: bool,
    /// The automatic fluid layout was written into the model before the first move.
    prepared: bool,
}

/// What the mouse system knows about the pointer this frame.
pub struct Pointer {
    pub cursor: Option<Vec2>,
    pub ray: Option<Ray3d>,
    /// Bevy ground `(x, z)` under the cursor.
    pub ground: Option<Vec2>,
    pub over_ui: bool,
    pub shift: bool,
    pub just_pressed: bool,
    pub pressed: bool,
    pub just_released: bool,
}

const SELECTED: Color = Color::srgb(1.0, 1.0, 1.0);
const PENDING: Color = Color::srgb(1.0, 0.85, 0.2);
const GHOST: Color = Color::srgb(0.3, 0.9, 0.4);
const CONNECTOR: Color = Color::srgb(0.70, 0.62, 0.40);

/// Drops a selection or a pending start that no longer exists (after undo, delete...).
pub fn prune(ed: &mut Editor) {
    let model = ed.doc.model();
    let node = |n: &str| {
        model
            .network
            .as_ref()
            .is_some_and(|x| x.nodes.iter().any(|m| m.name == n))
    };
    let edge = |a: &str, b: &str| {
        model
            .network
            .as_ref()
            .is_some_and(|x| x.edges.iter().any(|e| e.from == a && e.to == b))
    };
    let fluid = |n: &str| {
        model
            .fluids
            .as_ref()
            .is_some_and(|x| x.nodes.iter().any(|m| m.name == n))
    };
    let keep = match &ed.net_sel {
        Some(NetSel::Node(n)) => node(n),
        Some(NetSel::Edge(a, b)) => edge(a, b),
        None => true,
    };
    if !keep {
        ed.net_sel = None;
    }
    if ed.fluid_sel.as_deref().is_some_and(|n| !fluid(n)) {
        ed.fluid_sel = None;
    }
    match &ed.tool {
        Tool::Edge(Some(n)) if !node(n) => ed.tool = Tool::Edge(None),
        Tool::FluidLink(Some(n)) if !fluid(n) => ed.tool = Tool::FluidLink(None),
        _ => {}
    }
}

/// `Del` in the `Rede` and `Fluidos` layers: removes the selected node or edge (a node
/// takes its edges or links along).
pub fn delete(ed: &mut Editor) {
    match ed.layer {
        Layer::Network => match ed.net_sel.take() {
            Some(NetSel::Node(name)) => {
                ed.exec(Command::RemoveNode { name: name.clone() });
                if ed.status.is_empty() {
                    ed.status = fx("camadas-no_excluido", &[("nome", vx(&name))]);
                }
            }
            Some(NetSel::Edge(from, to)) => {
                ed.exec(Command::RemoveEdge {
                    from: from.clone(),
                    to: to.clone(),
                });
                if ed.status.is_empty() {
                    ed.status = fx(
                        "camadas-aresta_excluida",
                        &[("de", vx(&from)), ("para", vx(&to))],
                    );
                }
            }
            None => {}
        },
        Layer::Fluids => {
            if let Some(name) = ed.fluid_sel.take() {
                ed.exec(Command::RemoveFluidNode { name: name.clone() });
                if ed.status.is_empty() {
                    ed.status = fx("camadas-no_fluido_excluido", &[("nome", vx(&name))]);
                }
            }
        }
        Layer::Objects => {}
    }
}

/// `Esc`: ends the edge (or link) being drawn, else goes back to selecting, else
/// clears the selection.
pub fn escape(ed: &mut Editor) {
    ed.tool = match &ed.tool {
        Tool::Edge(Some(_)) => Tool::Edge(None),
        Tool::FluidLink(Some(_)) => Tool::FluidLink(None),
        Tool::Select => {
            ed.net_sel = None;
            ed.fluid_sel = None;
            Tool::Select
        }
        _ => Tool::Select,
    };
    ed.layer_press = None;
}

/// Selected objects, nodes and links follow a rename made in the panel.
pub fn follow_renames(ed: &mut Editor, commands: &[Command]) {
    for c in commands {
        match c {
            Command::RenameNode { from, to } => {
                if ed.net_sel == Some(NetSel::Node(from.clone())) {
                    ed.net_sel = Some(NetSel::Node(to.clone()));
                }
            }
            Command::RenameFluidNode { from, to } => {
                if ed.fluid_sel.as_deref() == Some(from.as_str()) {
                    ed.fluid_sel = Some(to.clone());
                }
            }
            Command::SetEdge { from, to, edge } => {
                if ed.net_sel == Some(NetSel::Edge(from.clone(), to.clone())) {
                    ed.net_sel = Some(NetSel::Edge(edge.from.clone(), edge.to.clone()));
                }
            }
            Command::Batch(inner) => follow_renames(ed, inner),
            _ => {}
        }
    }
}

/// The name of a layer on screen; [`Layer::label`] stays the Portuguese name that scripts
/// and the recorded state use.
pub fn layer_name(layer: Layer) -> &'static str {
    match layer {
        Layer::Objects => tx("camada-objetos"),
        Layer::Network => tx("camada-rede"),
        Layer::Fluids => tx("camada-fluidos"),
    }
}

/// The three layers as buttons at the top of the left panel.
pub fn layer_switch(ui: &mut egui::Ui, ed: &mut Editor) {
    ui.horizontal(|ui| {
        for layer in Layer::ALL {
            let hint = match layer {
                Layer::Objects => tx("camadas-objetos_dica"),
                Layer::Network => tx("camadas-rede_dica"),
                Layer::Fluids => tx("camadas-fluidos_dica"),
            };
            if ui
                .selectable_label(ed.layer == layer, layer_name(layer))
                .on_hover_text(hint)
                .clicked()
                && ed.layer != layer
            {
                ed.layer = layer;
                ed.tool = Tool::Select;
                ed.layer_press = None;
                ed.palette_drag = None;
                ed.status.clear();
            }
        }
    });
}

fn tool_button(ui: &mut egui::Ui, ed: &mut Editor, label: &str, tool: Tool, hint: &str) {
    let on = std::mem::discriminant(&ed.tool) == std::mem::discriminant(&tool)
        && match (&ed.tool, &tool) {
            (Tool::FluidPlace(a), Tool::FluidPlace(b)) => a == b,
            _ => true,
        };
    if ui.selectable_label(on, label).on_hover_text(hint).clicked() && !on {
        ed.tool = tool;
    }
}

fn grid_ui(ui: &mut egui::Ui, ed: &mut Editor) {
    ui.checkbox(&mut ed.snap, tx("editor-encaixar"));
    ui.horizontal(|ui| {
        ui.label(tx("editor-passo"));
        ui.add_enabled(
            ed.snap,
            egui::DragValue::new(&mut ed.grid_step)
                .range(0.1..=10.0)
                .speed(0.05)
                .max_decimals(2),
        );
    });
}

/// Left panel of the `Rede` and `Fluidos` layers: their tools.
pub fn tools_ui(ui: &mut egui::Ui, ed: &mut Editor) {
    match ed.layer {
        Layer::Network => {
            ui.heading(tx("camadas-rede_de_caminhos"));
            tool_button(
                ui,
                ed,
                tx("camadas-selecionar"),
                Tool::Select,
                tx("camadas-selecionar_rede_dica"),
            );
            tool_button(ui, ed, tx("camadas-no"), Tool::Node, tx("camadas-no_dica"));
            tool_button(
                ui,
                ed,
                tx("camadas-aresta"),
                Tool::Edge(None),
                tx("camadas-aresta_dica"),
            );
            ui.checkbox(&mut ed.new_two_way, tx("camadas-mao_dupla"));
            ui.separator();
            grid_ui(ui, ed);
            ui.separator();
            ui.weak(tx("camadas-ajuda_aresta"));
            ui.weak(tx("camadas-ajuda_del_rede"));
            ui.weak(tx("camadas-ajuda_atendido"));
        }
        Layer::Fluids => {
            ui.heading(tx("camadas-rede_de_fluidos"));
            tool_button(
                ui,
                ed,
                tx("camadas-selecionar"),
                Tool::Select,
                tx("camadas-selecionar_fluidos_dica"),
            );
            tool_button(
                ui,
                ed,
                tx("camadas-ligar"),
                Tool::FluidLink(None),
                tx("camadas-ligar_dica"),
            );
            ui.separator();
            ui.strong(tx("camadas-colocar"));
            for e in FLUID_PALETTE {
                tool_button(
                    ui,
                    ed,
                    e.label,
                    Tool::FluidPlace(e.id),
                    tx("camadas-colocar_dica"),
                );
            }
            ui.separator();
            grid_ui(ui, ed);
            ui.separator();
            ui.weak(tx("camadas-ajuda_aciclica"));
            ui.weak(tx("camadas-ajuda_del_fluidos"));
        }
        Layer::Objects => {}
    }
}

/// Right panel of the `Rede` and `Fluidos` layers: the selected node or edge, or a list
/// to pick from. Returns the commands to apply.
pub fn properties_ui(ui: &mut egui::Ui, ed: &mut Editor) -> Vec<Command> {
    let mut out = Vec::new();
    match ed.layer {
        Layer::Network => match ed.net_sel.clone() {
            Some(NetSel::Node(name)) => {
                ui.horizontal(|ui| {
                    ui.strong(&name);
                    if ui.button(tx("camadas-aresta_daqui")).clicked() {
                        ed.tool = Tool::Edge(Some(name.clone()));
                    }
                    if ui.button(tx("editor-excluir")).clicked() {
                        out.push(Command::RemoveNode { name: name.clone() });
                    }
                });
                ui.separator();
                out.extend(node_panel(ui, &ed.doc, &name));
            }
            Some(NetSel::Edge(from, to)) => {
                if ui.button(tx("editor-excluir")).clicked() {
                    out.push(Command::RemoveEdge {
                        from: from.clone(),
                        to: to.clone(),
                    });
                }
                ui.separator();
                out.extend(edge_panel(ui, &ed.doc, &from, &to));
            }
            None => network_summary(ui, ed),
        },
        Layer::Fluids => match ed.fluid_sel.clone() {
            Some(name) => {
                ui.horizontal(|ui| {
                    ui.strong(&name);
                    if ui.button(tx("camadas-ligar_daqui")).clicked() {
                        ed.tool = Tool::FluidLink(Some(name.clone()));
                    }
                    if ui.button(tx("editor-excluir")).clicked() {
                        out.push(Command::RemoveFluidNode { name: name.clone() });
                    }
                });
                ui.separator();
                out.extend(fluid_node_panel(ui, &ed.doc, &name));
            }
            None => fluid_summary(ui, ed),
        },
        Layer::Objects => {}
    }
    out
}

fn network_summary(ui: &mut egui::Ui, ed: &mut Editor) {
    let Some(net) = ed.doc.model().network.clone() else {
        ui.label(tx("camadas-sem_rede"));
        ui.weak(tx("camadas-sem_rede_ajuda"));
        return;
    };
    ui.label(fx(
        "camadas-resumo_rede",
        &[
            ("nos", vx(&net.nodes.len())),
            ("arestas", vx(&net.edges.len())),
        ],
    ));
    egui::CollapsingHeader::new(fx("camadas-nos", &[("n", vx(&net.nodes.len()))]))
        .default_open(true)
        .show(ui, |ui| {
            for n in &net.nodes {
                if ui.link(&n.name).clicked() {
                    ed.net_sel = Some(NetSel::Node(n.name.clone()));
                }
            }
        });
    egui::CollapsingHeader::new(fx("camadas-arestas", &[("n", vx(&net.edges.len()))]))
        .default_open(true)
        .show(ui, |ui| {
            for e in &net.edges {
                if ui.link(edge_text(e)).clicked() {
                    ed.net_sel = Some(NetSel::Edge(e.from.clone(), e.to.clone()));
                }
            }
        });
}

fn fluid_summary(ui: &mut egui::Ui, ed: &mut Editor) {
    let Some(def) = ed.doc.model().fluids.clone() else {
        ui.label(tx("camadas-sem_fluidos"));
        ui.weak(tx("camadas-sem_fluidos_ajuda"));
        return;
    };
    ui.label(fx(
        "camadas-resumo_fluidos",
        &[
            ("rede", vx(&def.name)),
            ("nos", vx(&def.nodes.len())),
            ("ligacoes", vx(&def.links.len())),
        ],
    ));
    for n in &def.nodes {
        let kind = fluid_entry(utsim_edit::fluids::fluid_kind_id(&n.kind)).map_or("", |e| e.label);
        ui.horizontal(|ui| {
            if ui.link(&n.name).clicked() {
                ed.fluid_sel = Some(n.name.clone());
            }
            ui.weak(kind);
        });
    }
}

/// Labels of the layers: node names in `Rede`, fluid nodes always (they are drawn in
/// `Editar` like the objects).
pub fn labels(ed: &Editor) -> Vec<Label> {
    let mut out = Vec::new();
    let model = ed.doc.model();
    if ed.layer == Layer::Network
        && let Some(net) = &model.network
    {
        for n in &net.nodes {
            out.push(Label {
                text: n.name.clone(),
                position: layout::to_bevy(n.position),
                height: 0.25,
                selected: ed.net_sel == Some(NetSel::Node(n.name.clone())),
            });
        }
    }
    for node in &ed.fluid_view.nodes {
        let text = if node.shape == NodeShape::Tank {
            format!("{} ({:.0}/{:.0})", node.name, node.initial, node.capacity)
        } else {
            node.name.clone()
        };
        out.push(Label {
            text,
            position: node.pos,
            height: fluids::node_half(node)[1] * 2.0,
            selected: ed.fluid_sel.as_deref() == Some(node.name.as_str()),
        });
    }
    out
}

/// Network nodes in Bevy space, in model order.
fn node_positions(ed: &Editor) -> Vec<(String, Vec3)> {
    ed.doc
        .model()
        .network
        .as_ref()
        .map_or_else(Vec::new, |net| {
            net.nodes
                .iter()
                .map(|n| (n.name.clone(), Vec3::from(layout::to_bevy(n.position))))
                .collect()
        })
}

fn pick_node(ed: &Editor, cursor: Vec2, project: &dyn Fn(Vec3) -> Option<Vec2>) -> Option<String> {
    let nodes = node_positions(ed);
    let points: Vec<Option<[f32; 2]>> = nodes
        .iter()
        .map(|(_, p)| project(*p).map(|v| [v.x, v.y]))
        .collect();
    nearest_point([cursor.x, cursor.y], &points, NODE_PICK_PIXELS).map(|i| nodes[i].0.clone())
}

fn pick_edge(
    ed: &Editor,
    cursor: Vec2,
    project: &dyn Fn(Vec3) -> Option<Vec2>,
) -> Option<(String, String)> {
    let net = ed.doc.model().network.as_ref()?;
    let at = |n: &str| {
        net.nodes
            .iter()
            .find(|x| x.name == n)
            .map(|x| Vec3::from(layout::to_bevy(x.position)))
    };
    let mut screen = Vec::new();
    let mut owners = Vec::new();
    for e in &net.edges {
        if let (Some(a), Some(b)) = (at(&e.from), at(&e.to))
            && let (Some(pa), Some(pb)) = (project(a), project(b))
        {
            screen.push(([pa.x, pa.y], [pb.x, pb.y]));
            owners.push((e.from.clone(), e.to.clone()));
        }
    }
    nearest_segment([cursor.x, cursor.y], &screen, LINE_PICK_PIXELS).map(|k| owners[k].clone())
}

fn pick_fluid(ed: &Editor, ray: Ray3d) -> Option<String> {
    let mut best: Option<(f32, usize)> = None;
    for (i, node) in ed.fluid_view.nodes.iter().enumerate() {
        let half = Vec3::from(fluids::node_half(node)) + Vec3::splat(0.1);
        let center = Vec3::from(node.pos) + Vec3::Y * half.y;
        if let Some(d) = scene::ray_box(ray.origin, *ray.direction, center - half, center + half)
            && best.is_none_or(|(bd, _)| d < bd)
        {
            best = Some((d, i));
        }
    }
    best.map(|(_, i)| ed.fluid_view.nodes[i].name.clone())
}

/// Bevy position of a node of the current layer.
fn node_base(ed: &Editor, name: &str) -> Option<Vec3> {
    match ed.layer {
        Layer::Network => node_positions(ed)
            .into_iter()
            .find(|(n, _)| n == name)
            .map(|(_, p)| p),
        Layer::Fluids => ed
            .fluid_view
            .nodes
            .iter()
            .find(|n| n.name == name)
            .map(|n| Vec3::from(n.pos)),
        Layer::Objects => None,
    }
}

/// `Editar`, `Rede` and `Fluidos` layers: select, drag, place and connect with the left
/// button.
pub fn mouse(
    ed: &mut Editor,
    p: &Pointer,
    cam: &Camera,
    cam_tf: &GlobalTransform,
    lock: &mut CameraLock,
) {
    let project = |v: Vec3| cam.world_to_viewport(cam_tf, v).ok();
    let step = ed.snap_step();
    let ghost = matches!(
        ed.tool,
        Tool::Node | Tool::Edge(_) | Tool::FluidPlace(_) | Tool::FluidLink(Some(_))
    );
    ed.hover = if ghost && !p.over_ui {
        p.ground.map(|g| ground_to_model_snapped(g.x, g.y, step))
    } else {
        None
    };

    if p.just_pressed {
        ed.layer_press = None;
        lock.0 = false;
        if !p.over_ui
            && let (Some(cursor), Some(ray)) = (p.cursor, p.ray)
        {
            let hit = match ed.layer {
                Layer::Network => pick_node(ed, cursor, &project),
                _ => pick_fluid(ed, ray),
            };
            let mut press = LayerPress {
                start: cursor,
                grabbed: None,
                moved: false,
                prepared: false,
            };
            let grabs = matches!(ed.tool, Tool::Select | Tool::Node | Tool::FluidPlace(_));
            if let Some(name) = hit.filter(|_| grabs) {
                if let (Some(base), Some(g)) = (node_base(ed, &name), p.ground) {
                    press.grabbed = Some((name.clone(), Vec2::new(base.x, base.z) - g));
                    lock.0 = true;
                }
                select(ed, &name);
            }
            ed.layer_press = Some(press);
        }
    }

    if p.pressed
        && let Some(cursor) = p.cursor
        && let Some(mut press) = ed.layer_press.take()
    {
        if cursor.distance(press.start) > 4.0 {
            press.moved = true;
        }
        if press.moved
            && let (Some((name, grab)), Some(g)) = (press.grabbed.clone(), p.ground)
        {
            let at = ground_to_model_snapped(g.x + grab.x, g.y + grab.y, step);
            drag(ed, &mut press, &name, at);
        }
        ed.layer_press = Some(press);
    }

    if p.just_released {
        lock.0 = false;
        ed.doc.end_gesture();
        let Some(press) = ed.layer_press.take() else {
            return;
        };
        if press.moved {
            return;
        }
        if let Some(cursor) = p.cursor.filter(|c| c.distance(press.start) <= 4.0) {
            click(ed, p, cursor, &project);
        }
    }
}

fn select(ed: &mut Editor, name: &str) {
    match ed.layer {
        Layer::Network => ed.net_sel = Some(NetSel::Node(name.to_string())),
        Layer::Fluids => ed.fluid_sel = Some(name.to_string()),
        Layer::Objects => {}
    }
}

/// Moves the grabbed node to `at`, one coalesced undo step per drag.
fn drag(ed: &mut Editor, press: &mut LayerPress, name: &str, at: [f64; 3]) {
    let model = ed.doc.model();
    let (current, key, command) = match ed.layer {
        Layer::Network => (
            model
                .network
                .as_ref()
                .and_then(|n| n.nodes.iter().find(|x| x.name == name))
                .map(|x| x.position),
            format!("no:{name}"),
            Command::MoveNode {
                name: name.to_string(),
                position: at,
            },
        ),
        _ => (
            ed.fluid_view
                .nodes
                .iter()
                .find(|x| x.name == name)
                .map(|x| ground_to_model_snapped(x.pos[0], x.pos[2], None)),
            format!("fluido:{name}"),
            Command::MoveFluidNode {
                name: name.to_string(),
                position: at,
            },
        ),
    };
    if current.is_some_and(|c| (c[0] - at[0]).abs() < 1e-9 && (c[1] - at[1]).abs() < 1e-9) {
        return;
    }
    if ed.layer == Layer::Fluids && !press.prepared {
        press.prepared = true;
        if let Some(keep) = fluid_positions_command(ed.doc.model(), &ed.fluid_view)
            && let Err(e) = ed.doc.apply_coalescing(keep, Some(&key))
        {
            ed.status = e;
        }
    }
    if let Err(e) = ed.doc.apply_coalescing(command, Some(&key)) {
        ed.status = e;
    }
    ed.refresh();
}

/// A click (press and release without moving) in the `Rede` or `Fluidos` layer.
fn click(ed: &mut Editor, p: &Pointer, cursor: Vec2, project: &dyn Fn(Vec3) -> Option<Vec2>) {
    let at = p
        .ground
        .map(|g| ground_to_model_snapped(g.x, g.y, ed.snap_step()));
    match ed.layer {
        Layer::Network => network_click(ed, cursor, at, project),
        Layer::Fluids => fluid_click(ed, p, at),
        Layer::Objects => {}
    }
}

fn network_click(
    ed: &mut Editor,
    cursor: Vec2,
    at: Option<[f64; 3]>,
    project: &dyn Fn(Vec3) -> Option<Vec2>,
) {
    let node = pick_node(ed, cursor, project);
    match ed.tool.clone() {
        Tool::Edge(from) => {
            let to = match node {
                Some(n) => n,
                None => {
                    // A click on the floor adds a node there (and the edge to it).
                    let Some(at) = at else { return };
                    let name = ed.doc.unique_node_name(NODE_BASE);
                    let add = Command::AddNode(NodeDef {
                        name: name.clone(),
                        position: at,
                    });
                    let command = match &from {
                        Some(f) => Command::Batch(vec![add, new_edge(ed, f, &name)]),
                        None => add,
                    };
                    ed.exec(command);
                    if ed.status.is_empty() {
                        ed.net_sel = Some(NetSel::Node(name.clone()));
                        ed.tool = Tool::Edge(Some(name));
                    }
                    return;
                }
            };
            match from {
                None => {
                    ed.net_sel = Some(NetSel::Node(to.clone()));
                    ed.tool = Tool::Edge(Some(to));
                }
                Some(f) if f == to => {}
                Some(f) => {
                    let edge = new_edge(ed, &f, &to);
                    ed.exec(edge);
                    if ed.status.is_empty() {
                        ed.net_sel = Some(NetSel::Edge(f, to.clone()));
                    }
                    ed.tool = Tool::Edge(Some(to));
                }
            }
        }
        tool => {
            if node.is_some() {
                // Already selected when the button went down.
                return;
            }
            if let Some((a, b)) = pick_edge(ed, cursor, project) {
                ed.net_sel = Some(NetSel::Edge(a, b));
                return;
            }
            if tool == Tool::Node
                && let Some(at) = at
            {
                let name = ed.doc.unique_node_name(NODE_BASE);
                ed.exec(Command::AddNode(NodeDef {
                    name: name.clone(),
                    position: at,
                }));
                if ed.status.is_empty() {
                    ed.net_sel = Some(NetSel::Node(name));
                }
            } else {
                ed.net_sel = None;
            }
        }
    }
}

fn new_edge(ed: &Editor, from: &str, to: &str) -> Command {
    Command::AddEdge(EdgeDef {
        from: from.to_string(),
        to: to.to_string(),
        two_way: ed.new_two_way,
        capacity: None,
    })
}

fn fluid_click(ed: &mut Editor, p: &Pointer, at: Option<[f64; 3]>) {
    let node = p.ray.and_then(|r| pick_fluid(ed, r));
    match ed.tool.clone() {
        Tool::FluidLink(from) => match (from, node) {
            (None, Some(n)) => {
                ed.fluid_sel = Some(n.clone());
                ed.tool = Tool::FluidLink(Some(n));
            }
            (Some(f), Some(n)) if f != n => {
                ed.exec(Command::AddFluidLink {
                    from: f,
                    to: n.clone(),
                });
                ed.fluid_sel = Some(n.clone());
                ed.tool = Tool::FluidLink(Some(n));
            }
            (Some(_), Some(_)) => {}
            (_, None) => ed.tool = Tool::FluidLink(None),
        },
        Tool::FluidPlace(id) => {
            if node.is_some() {
                return;
            }
            let (Some(at), Some(entry)) = (at, fluid_entry(id)) else {
                return;
            };
            // A converter starts on the selected object, else the first discrete one.
            let objects = converter_objects(&ed.doc);
            let object = ed
                .selected
                .clone()
                .filter(|s| objects.contains(s))
                .or_else(|| objects.first().cloned())
                .unwrap_or_default();
            let Some(kind) = fluid_template(id, &object) else {
                return;
            };
            let name = ed.doc.unique_fluid_name(entry.name_base);
            let add = Command::AddFluidNode(FluidNode {
                name: name.clone(),
                kind,
                position: at,
            });
            let command = match fluid_positions_command(ed.doc.model(), &ed.fluid_view) {
                Some(Command::Batch(mut keep)) => {
                    keep.push(add);
                    Command::Batch(keep)
                }
                _ => add,
            };
            ed.exec(command);
            if ed.status.is_empty() {
                ed.fluid_sel = Some(name);
            }
            if !p.shift {
                ed.tool = Tool::Select;
            }
        }
        _ => {
            if node.is_none() {
                ed.fluid_sel = None;
            }
        }
    }
}

/// Gizmos of the layers: connectors from the served objects to the network (always in
/// `Editar`), the selection, the edge or link being drawn and the ghost of a new node.
pub fn draw(ed: &Editor, gizmos: &mut Gizmos) {
    let lift = Vec3::Y * 0.04;
    for (a, b) in connector_segments(ed.doc.model()) {
        for (p, q) in view::dashes(a, b, 0.3, 0.25) {
            gizmos.line(Vec3::from(p) + lift, Vec3::from(q) + lift, CONNECTOR);
        }
    }
    let hover = ed.hover.map(|h| Vec3::from(layout::to_bevy(h)) + lift);
    match ed.layer {
        Layer::Network => {
            let nodes = node_positions(ed);
            let at = |n: &str| nodes.iter().find(|(m, _)| m == n).map(|(_, p)| *p + lift);
            match &ed.net_sel {
                Some(NetSel::Node(n)) => {
                    if let Some(p) = at(n) {
                        gizmos.sphere(Isometry3d::from_translation(p), 0.3, SELECTED);
                    }
                }
                Some(NetSel::Edge(a, b)) => {
                    if let (Some(pa), Some(pb)) = (at(a), at(b)) {
                        let side = (pb - pa).cross(Vec3::Y).normalize_or_zero() * 0.2;
                        gizmos.line(pa + side, pb + side, SELECTED);
                        gizmos.line(pa - side, pb - side, SELECTED);
                    }
                }
                None => {}
            }
            if let Tool::Edge(Some(from)) = &ed.tool
                && let Some(p) = at(from)
            {
                gizmos.sphere(Isometry3d::from_translation(p), 0.35, PENDING);
                if let Some(h) = hover {
                    for (a, b) in view::dashes(p.to_array(), h.to_array(), 0.3, 0.2) {
                        gizmos.line(Vec3::from(a), Vec3::from(b), PENDING);
                    }
                }
            }
            if matches!(ed.tool, Tool::Node | Tool::Edge(_))
                && let Some(h) = hover
            {
                gizmos.sphere(Isometry3d::from_translation(h), 0.2, GHOST);
            }
        }
        Layer::Fluids => {
            let outline = |gizmos: &mut Gizmos, name: &str, color: Color| {
                if let Some(node) = ed.fluid_view.nodes.iter().find(|n| n.name == name) {
                    let half = fluids::node_half(node);
                    gizmos.cube(
                        Transform {
                            translation: Vec3::from(node.pos) + Vec3::Y * half[1],
                            rotation: Quat::IDENTITY,
                            scale: Vec3::from(half) * 2.0 + Vec3::splat(0.3),
                        },
                        color,
                    );
                }
            };
            if let Some(n) = &ed.fluid_sel {
                outline(gizmos, n, SELECTED);
            }
            if let Tool::FluidLink(Some(from)) = &ed.tool {
                outline(gizmos, from, PENDING);
                if let (Some(node), Some(h)) =
                    (ed.fluid_view.nodes.iter().find(|n| &n.name == from), hover)
                {
                    let p = Vec3::from(node.pos) + Vec3::Y * fluids::port_height(node);
                    for (a, b) in view::dashes(p.to_array(), h.to_array(), 0.3, 0.2) {
                        gizmos.line(Vec3::from(a), Vec3::from(b), PENDING);
                    }
                }
            }
            if matches!(ed.tool, Tool::FluidPlace(_))
                && let Some(h) = hover
            {
                gizmos.cube(
                    Transform::from_translation(h + Vec3::Y * 0.35).with_scale(Vec3::splat(0.7)),
                    GHOST,
                );
            }
        }
        Layer::Objects => {}
    }
}
