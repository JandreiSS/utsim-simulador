//! Properties of the path network (a node or an edge) and of the fluid network (a
//! node), for the "Rede" and "Fluidos" layers of `Editar`. Like the object panel they
//! read the [`Document`] and return [`Command`]s; nothing is changed here.

use bevy_egui::egui::{self, Id, Ui};
use utsim_edit::fluids::{FLUID_PALETTE, fluid_entry, fluid_kind_id, fluid_template};
use utsim_edit::network::{attachments, edge_index, edge_length};
use utsim_edit::{Command, Document, EdgeDef};
use utsim_fluid::{FluidKind, FluidNode};

use super::panel::{RED, float, num, text_commit};

/// Direction of an edge as offered by the panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    TwoWay,
    /// One way, as stored (`from -> to`).
    Forward,
    /// One way, the other way round (`to -> from`).
    Backward,
}

/// The edge with its direction changed (a backward edge is stored reversed).
pub fn with_direction(edge: &EdgeDef, direction: Direction) -> EdgeDef {
    let mut e = edge.clone();
    match direction {
        Direction::TwoWay => e.two_way = true,
        Direction::Forward => e.two_way = false,
        Direction::Backward => {
            e.two_way = false;
            std::mem::swap(&mut e.from, &mut e.to);
        }
    }
    e
}

/// Text of an edge in lists: `a <-> b` or `a -> b`.
pub fn edge_text(edge: &EdgeDef) -> String {
    let arrow = if edge.two_way { "<->" } else { "->" };
    format!("{} {arrow} {}", edge.from, edge.to)
}

fn id(parts: (&str, &str, &str)) -> Id {
    Id::new(("utsim-rede", parts))
}

fn position_fields(ui: &mut Ui, key: (&str, &str), position: [f64; 3], out: &mut Vec<[f64; 3]>) {
    ui.horizontal(|ui| {
        let (mut x, mut y) = (position[0], position[1]);
        ui.label("x");
        let cx = float(ui, id((key.0, key.1, "x")), &mut x);
        ui.label("y");
        let cy = float(ui, id((key.0, key.1, "y")), &mut y);
        if cx || cy {
            out.push([x, y, position[2]]);
        }
    });
}

/// Name field shared by both networks; `taken` says whether a name is used.
fn name_field(
    ui: &mut Ui,
    key: (&str, &str),
    current: &str,
    taken: &dyn Fn(&str) -> bool,
) -> Option<String> {
    let check = |t: &str| {
        if t.trim().is_empty() {
            Err("o nome não pode ficar vazio".to_string())
        } else if t != current && taken(t) {
            Err(format!("já existe um nó chamado \"{t}\""))
        } else {
            Ok(())
        }
    };
    let mut to = None;
    ui.horizontal(|ui| {
        ui.label("Nome");
        to = text_commit(ui, id((key.0, key.1, "#nome")), current, false, &check);
    });
    to
}

/// A node of the path network: name, position, its edges and the objects that enter
/// the network through it.
pub fn node_panel(ui: &mut Ui, doc: &Document, name: &str) -> Vec<Command> {
    let mut out = Vec::new();
    let Some(net) = &doc.model().network else {
        return out;
    };
    let Some(node) = net.nodes.iter().find(|n| n.name == name) else {
        return out;
    };
    let taken = |t: &str| net.nodes.iter().any(|n| n.name == t);
    if let Some(to) = name_field(ui, ("no", name), name, &taken) {
        out.push(Command::RenameNode {
            from: name.to_string(),
            to,
        });
    }
    ui.weak("Nó da rede de caminhos");
    let mut moved = Vec::new();
    position_fields(ui, ("no", name), node.position, &mut moved);
    for position in moved {
        out.push(Command::MoveNode {
            name: name.to_string(),
            position,
        });
    }
    ui.separator();
    ui.strong("Arestas");
    let edges: Vec<&EdgeDef> = net
        .edges
        .iter()
        .filter(|e| e.from == name || e.to == name)
        .collect();
    if edges.is_empty() {
        ui.colored_label(
            RED,
            "Nenhuma: use a ferramenta \"Aresta\" e clique em outro nó.",
        );
    }
    for e in edges {
        ui.horizontal(|ui| {
            let len = edge_length(net, &e.from, &e.to).unwrap_or(0.0);
            ui.label(format!("{} ({len:.2} m)", edge_text(e)));
            if ui
                .small_button("x")
                .on_hover_text("Remover aresta")
                .clicked()
            {
                out.push(Command::RemoveEdge {
                    from: e.from.clone(),
                    to: e.to.clone(),
                });
            }
        });
    }
    let attached: Vec<String> = attachments(doc.model())
        .into_iter()
        .filter(|(_, n)| n == name)
        .map(|(o, _)| o)
        .collect();
    ui.separator();
    ui.strong("Objetos ligados a este nó");
    if attached.is_empty() {
        ui.weak("Nenhum (cada objeto atendido entra pelo nó mais próximo).");
    } else {
        ui.label(attached.join(", "));
    }
    out
}

/// An edge of the path network: ends, length (computed), direction and AGV capacity.
pub fn edge_panel(ui: &mut Ui, doc: &Document, from: &str, to: &str) -> Vec<Command> {
    let mut out = Vec::new();
    let Some(net) = &doc.model().network else {
        return out;
    };
    let Some(edge) = edge_index(net, from, to).map(|i| &net.edges[i]) else {
        return out;
    };
    let set = |e: EdgeDef| Command::SetEdge {
        from: from.to_string(),
        to: to.to_string(),
        edge: e,
    };
    ui.strong(format!("Aresta {}", edge_text(edge)));
    let len = edge_length(net, from, to).unwrap_or(0.0);
    ui.label(format!(
        "Comprimento: {len:.2} m (calculado pelas posições dos nós)"
    ));
    ui.separator();
    let current = if edge.two_way {
        Direction::TwoWay
    } else {
        Direction::Forward
    };
    let options = [
        (Direction::TwoWay, format!("Mão dupla ({from} <-> {to})")),
        (Direction::Forward, format!("Mão única ({from} -> {to})")),
        (Direction::Backward, format!("Mão única ({to} -> {from})")),
    ];
    ui.horizontal(|ui| {
        ui.label("Sentido");
        let shown = options
            .iter()
            .find(|(d, _)| *d == current)
            .map(|(_, t)| t.clone())
            .unwrap_or_default();
        egui::ComboBox::from_id_salt(id(("aresta", from, to)))
            .selected_text(shown)
            .show_ui(ui, |ui| {
                for (d, text) in &options {
                    if ui.selectable_label(*d == current, text).clicked() && *d != current {
                        out.push(set(with_direction(edge, *d)));
                    }
                }
            });
    });
    ui.horizontal(|ui| {
        let mut on = edge.capacity.is_some();
        if ui
            .checkbox(&mut on, "Limitar veículos (AGV)")
            .on_hover_text("Quantos AGVs podem estar na aresta ao mesmo tempo")
            .changed()
        {
            out.push(set(EdgeDef {
                capacity: on.then_some(1),
                ..edge.clone()
            }));
        }
        match edge.capacity {
            Some(mut c) => {
                if num(ui, id(("capacidade", from, to)), &mut c, 0.1, 1) {
                    out.push(set(EdgeDef {
                        capacity: Some(c),
                        ..edge.clone()
                    }));
                }
            }
            None => {
                ui.weak("sem limite");
            }
        }
    });
    ui.weak("Só AGVs (agv: true) respeitam a capacidade; os demais passam livremente.");
    out
}

/// Discrete objects a converter can read from or write to (not mobile resources).
pub fn converter_objects(doc: &Document) -> Vec<String> {
    doc.model()
        .objects
        .iter()
        .filter(|o| !utsim_model::is_mobile(&o.kind))
        .map(|o| o.name.clone())
        .collect()
}

/// Number box that can be switched off (`None`): returns the new value on a commit.
fn opt_float(
    ui: &mut Ui,
    key: Id,
    label: &str,
    current: Option<f64>,
    default: f64,
) -> Option<Option<f64>> {
    let mut result = None;
    ui.horizontal(|ui| {
        let mut on = current.is_some();
        if ui.checkbox(&mut on, label).changed() {
            result = Some(on.then_some(default));
        }
        match current {
            Some(mut v) => {
                if float(ui, key, &mut v) {
                    result = Some(Some(v));
                }
            }
            None => {
                ui.weak("sem limite");
            }
        }
    });
    result
}

fn labeled(ui: &mut Ui, key: Id, label: &str, value: f64) -> Option<f64> {
    let mut v = value;
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label);
        changed = float(ui, key, &mut v);
    });
    changed.then_some(v)
}

fn object_picker(ui: &mut Ui, key: Id, doc: &Document, current: &str) -> Option<String> {
    let objects = converter_objects(doc);
    let mut chosen = None;
    ui.horizontal(|ui| {
        ui.label("Objeto discreto");
        let shown = if current.is_empty() {
            "(escolha)"
        } else {
            current
        };
        egui::ComboBox::from_id_salt(key)
            .selected_text(shown)
            .show_ui(ui, |ui| {
                for name in &objects {
                    if ui.selectable_label(name == current, name).clicked() && name != current {
                        chosen = Some(name.clone());
                    }
                }
            });
    });
    if !current.is_empty() && !objects.iter().any(|o| o == current) {
        ui.colored_label(
            RED,
            format!("\"{current}\" não é um objeto discreto do modelo"),
        );
    }
    chosen
}

/// The parameters of a fluid node kind; returns the edited kind on a commit.
fn kind_fields(ui: &mut Ui, doc: &Document, name: &str, kind: &FluidKind) -> Option<FluidKind> {
    let key = |f: &str| id(("fluido", name, f));
    let mut next = None;
    match kind {
        FluidKind::Source { rate } => {
            if let Some(rate) = labeled(ui, key("rate"), "Vazão", *rate) {
                next = Some(FluidKind::Source { rate });
            }
        }
        FluidKind::Tank {
            capacity,
            initial,
            max_in,
            max_out,
        } => {
            let tank = |capacity, initial, max_in, max_out| FluidKind::Tank {
                capacity,
                initial,
                max_in,
                max_out,
            };
            if let Some(c) = labeled(ui, key("capacity"), "Capacidade", *capacity) {
                next = Some(tank(c, *initial, *max_in, *max_out));
            }
            if let Some(v) = labeled(ui, key("initial"), "Nível inicial", *initial) {
                next = Some(tank(*capacity, v, *max_in, *max_out));
            }
            if let Some(v) = opt_float(ui, key("max_in"), "Entrada máxima", *max_in, 1.0) {
                next = Some(tank(*capacity, *initial, v, *max_out));
            }
            if let Some(v) = opt_float(ui, key("max_out"), "Saída máxima", *max_out, 1.0) {
                next = Some(tank(*capacity, *initial, *max_in, v));
            }
        }
        FluidKind::Pipe { max_rate } => {
            if let Some(max_rate) = labeled(ui, key("max_rate"), "Vazão máxima", *max_rate) {
                next = Some(FluidKind::Pipe { max_rate });
            }
        }
        FluidKind::Sink { max_rate } => {
            if let Some(max_rate) = opt_float(ui, key("max_rate"), "Vazão máxima", *max_rate, 1.0)
            {
                next = Some(FluidKind::Sink { max_rate });
            }
        }
        FluidKind::FromItems { object, volume } => {
            ui.weak("Cada item que entra no objeto vira este volume, entregue de uma vez.");
            if let Some(o) = object_picker(ui, key("object"), doc, object) {
                next = Some(FluidKind::FromItems {
                    object: o,
                    volume: *volume,
                });
            }
            if let Some(v) = labeled(ui, key("volume"), "Volume por item", *volume) {
                next = Some(FluidKind::FromItems {
                    object: object.clone(),
                    volume: v,
                });
            }
        }
        FluidKind::ToItems {
            object,
            volume,
            max_rate,
        } => {
            ui.weak("A cada volume completo, cria um item dentro do objeto.");
            let to_items = |object: &str, volume, max_rate| FluidKind::ToItems {
                object: object.to_string(),
                volume,
                max_rate,
            };
            if let Some(o) = object_picker(ui, key("object"), doc, object) {
                next = Some(to_items(&o, *volume, *max_rate));
            }
            if let Some(v) = labeled(ui, key("volume"), "Volume por item", *volume) {
                next = Some(to_items(object, v, *max_rate));
            }
            if let Some(v) = opt_float(ui, key("max_rate"), "Vazão máxima", *max_rate, 1.0) {
                next = Some(to_items(object, *volume, v));
            }
        }
    }
    next
}

/// Object of a converter kind ("" for the others).
fn converter_object(kind: &FluidKind) -> &str {
    match kind {
        FluidKind::FromItems { object, .. } | FluidKind::ToItems { object, .. } => object,
        _ => "",
    }
}

/// A node of the fluid network: name, kind, position, parameters and links.
pub fn fluid_node_panel(ui: &mut Ui, doc: &Document, name: &str) -> Vec<Command> {
    let mut out = Vec::new();
    let Some(def) = &doc.model().fluids else {
        return out;
    };
    let Some(node): Option<&FluidNode> = def.nodes.iter().find(|n| n.name == name) else {
        return out;
    };
    let prefix = format!("fluidos.{name}:");
    for issue in doc.issues() {
        if issue.message.starts_with(&prefix) || issue.message.contains(&format!("'{name}'")) {
            ui.colored_label(RED, format!("Problema: {}", issue.message));
        }
    }
    let taken = |t: &str| def.nodes.iter().any(|n| n.name == t);
    if let Some(to) = name_field(ui, ("fluido", name), name, &taken) {
        out.push(Command::RenameFluidNode {
            from: name.to_string(),
            to,
        });
    }
    let current = fluid_kind_id(&node.kind);
    ui.horizontal(|ui| {
        ui.label("Tipo");
        let shown = fluid_entry(current).map_or(current, |e| e.label);
        egui::ComboBox::from_id_salt(id(("fluido", name, "#tipo")))
            .selected_text(shown)
            .show_ui(ui, |ui| {
                for e in FLUID_PALETTE {
                    if ui.selectable_label(e.id == current, e.label).clicked()
                        && e.id != current
                        && let Some(kind) = fluid_template(e.id, converter_object(&node.kind))
                    {
                        out.push(Command::SetFluidKind {
                            name: name.to_string(),
                            kind,
                        });
                    }
                }
            });
    });
    let mut moved = Vec::new();
    position_fields(ui, ("fluido", name), node.position, &mut moved);
    for position in moved {
        out.push(Command::MoveFluidNode {
            name: name.to_string(),
            position,
        });
    }
    ui.separator();
    if let Some(kind) = kind_fields(ui, doc, name, &node.kind) {
        out.push(Command::SetFluidKind {
            name: name.to_string(),
            kind,
        });
    }
    ui.separator();
    ui.strong("Ligações");
    let links: Vec<&(String, String)> = def
        .links
        .iter()
        .filter(|(a, b)| a == name || b == name)
        .collect();
    if links.is_empty() {
        ui.weak("Nenhuma: use a ferramenta \"Ligar\" e clique no próximo nó.");
    }
    for (a, b) in links {
        ui.horizontal(|ui| {
            ui.label(format!("{a} -> {b}"));
            if ui
                .small_button("x")
                .on_hover_text("Remover ligação")
                .clicked()
            {
                out.push(Command::RemoveFluidLink {
                    from: a.clone(),
                    to: b.clone(),
                });
            }
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn render(f: impl Fn(&mut Ui) -> Vec<Command>) -> Vec<Command> {
        let ctx = egui::Context::default();
        let mut commands = Vec::new();
        for _ in 0..2 {
            ctx.run_ui(egui::RawInput::default(), |ui| commands = f(ui))
                .drop_without_applying_deltas();
        }
        commands
    }

    fn open(name: &str) -> Document {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples")
            .join(name);
        Document::open(path).unwrap()
    }

    #[test]
    fn drawing_every_node_edge_and_fluid_node_emits_nothing() {
        for file in ["rede_de_caminhos.ron", "agvs.ron", "tanques.ron"] {
            let doc = open(file);
            if let Some(net) = &doc.model().network {
                for n in &net.nodes {
                    assert_eq!(render(|ui| node_panel(ui, &doc, &n.name)), Vec::new());
                }
                for e in &net.edges {
                    assert_eq!(
                        render(|ui| edge_panel(ui, &doc, &e.from, &e.to)),
                        Vec::new()
                    );
                }
            }
            if let Some(def) = &doc.model().fluids {
                for n in &def.nodes {
                    assert_eq!(render(|ui| fluid_node_panel(ui, &doc, &n.name)), Vec::new());
                }
            }
            // Unknown names draw nothing and emit nothing.
            assert_eq!(render(|ui| node_panel(ui, &doc, "???")), Vec::new());
            assert_eq!(render(|ui| fluid_node_panel(ui, &doc, "???")), Vec::new());
        }
    }

    #[test]
    fn directions_map_to_stored_edges() {
        let e = EdgeDef {
            from: "a".into(),
            to: "b".into(),
            two_way: true,
            capacity: Some(2),
        };
        let back = with_direction(&e, Direction::Backward);
        assert_eq!(
            (back.from.as_str(), back.to.as_str(), back.two_way),
            ("b", "a", false)
        );
        assert_eq!(back.capacity, Some(2));
        assert!(!with_direction(&e, Direction::Forward).two_way);
        assert_eq!(edge_text(&e), "a <-> b");
        assert_eq!(edge_text(&back), "b -> a");
        // Every direction is accepted by the document.
        let mut doc = open("rede_de_caminhos.ron");
        let net = doc.model().network.clone().unwrap();
        let first = &net.edges[0];
        for d in [Direction::TwoWay, Direction::Backward] {
            let current = doc.model().network.as_ref().unwrap().edges[0].clone();
            let next = with_direction(&current, d);
            doc.apply(Command::SetEdge {
                from: current.from.clone(),
                to: current.to.clone(),
                edge: next,
            })
            .unwrap();
        }
        let now = &doc.model().network.as_ref().unwrap().edges[0];
        assert_eq!(
            (now.from.as_str(), now.to.as_str()),
            (first.to.as_str(), first.from.as_str())
        );
    }

    #[test]
    fn converters_pick_discrete_objects_only() {
        let doc = open("agvs.ron");
        let objects = converter_objects(&doc);
        assert!(!objects.is_empty());
        for o in &objects {
            let def = doc.model().objects.iter().find(|x| &x.name == o).unwrap();
            assert!(!utsim_model::is_mobile(&def.kind));
        }
    }
}
