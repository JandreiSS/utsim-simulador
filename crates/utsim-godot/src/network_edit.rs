//! Editing the path network in Editar (G3), with no Godot types: the rows the overlay
//! draws (nodes, edges with direction, capacity and length) and the edits the "Rede"
//! tool makes, each one a `utsim_edit::Command` (or a `Batch`, one undo step).
//!
//! The rules (an edge needs two different existing nodes, no edge twice in the same
//! direction, capacity at least 1) are the ones of `utsim_edit::network`; a network that
//! is not connected yet is a problem of the model, not a refused edit, so it can be drawn
//! one edge at a time.

use utsim_edit::network::{NODE_BASE, edge_index, edge_length, nearest_node};
use utsim_edit::{Command, EdgeDef, NodeDef};
use utsim_model::Model;

use crate::document::{Editor, snap_to};

/// One node of the network, model coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeRow {
    pub name: String,
    pub position: [f64; 3],
}

/// One edge as stored (`from -> to`; both ways when `two_way`).
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeRow {
    pub from: String,
    pub to: String,
    pub two_way: bool,
    /// Vehicles at the same time (AGV traffic control); `None`: no limit.
    pub capacity: Option<usize>,
    /// Straight length between the nodes, metres.
    pub length: f64,
}

impl EdgeRow {
    /// A controlled edge (with a capacity) is drawn in red, as in the Bevy viewer.
    pub fn controlled(&self) -> bool {
        self.capacity.is_some()
    }
}

pub fn nodes(model: &Model) -> Vec<NodeRow> {
    model.network.as_ref().map_or_else(Vec::new, |net| {
        net.nodes
            .iter()
            .map(|n| NodeRow {
                name: n.name.clone(),
                position: n.position,
            })
            .collect()
    })
}

pub fn edges(model: &Model) -> Vec<EdgeRow> {
    model.network.as_ref().map_or_else(Vec::new, |net| {
        net.edges
            .iter()
            .map(|e| EdgeRow {
                from: e.from.clone(),
                to: e.to.clone(),
                two_way: e.two_way,
                capacity: e.capacity,
                length: edge_length(net, &e.from, &e.to).unwrap_or(0.0),
            })
            .collect()
    })
}

fn snapped(p: [f64; 3], step: f64) -> [f64; 3] {
    [snap_to(p[0], step), snap_to(p[1], step), 0.0]
}

impl Editor {
    fn edge(&self, from: &str, to: &str) -> Result<EdgeDef, String> {
        let net = self
            .model()
            .network
            .as_ref()
            .ok_or("o modelo não tem rede de caminhos")?;
        edge_index(net, from, to)
            .map(|i| net.edges[i].clone())
            .ok_or_else(|| format!("trecho {from} → {to} não existe"))
    }

    /// Adds a node at `p` (snapped to `step`, 0: no grid) and, when `from` is a node,
    /// the edge `from -> new` in the same undo step (drawing a path one click at a
    /// time). Returns the new node's name.
    pub fn add_node(
        &mut self,
        p: [f64; 3],
        step: f64,
        from: Option<&str>,
        two_way: bool,
    ) -> Result<String, String> {
        let name = self.doc.unique_node_name(NODE_BASE);
        let node = Command::AddNode(NodeDef {
            name: name.clone(),
            position: snapped(p, step),
        });
        let command = match from {
            Some(f) => Command::Batch(vec![
                node,
                Command::AddEdge(EdgeDef {
                    from: f.to_string(),
                    to: name.clone(),
                    two_way,
                    capacity: None,
                }),
            ]),
            None => node,
        };
        self.apply(command)?;
        Ok(name)
    }

    pub fn move_node(&mut self, name: &str, p: [f64; 3], step: f64) -> Result<(), String> {
        self.apply(Command::MoveNode {
            name: name.to_string(),
            position: snapped(p, step),
        })
    }

    pub fn rename_node(&mut self, from: &str, to: &str) -> Result<(), String> {
        let to = to.trim();
        if to.is_empty() {
            return Err("o nome do nó não pode ficar vazio".into());
        }
        if from == to {
            return Ok(());
        }
        if nodes(self.model()).iter().any(|n| n.name == to) {
            return Err(format!("já existe um nó chamado \"{to}\""));
        }
        self.apply(Command::RenameNode {
            from: from.to_string(),
            to: to.to_string(),
        })
    }

    /// Removes the nodes (and their edges) in one undo step.
    pub fn remove_nodes(&mut self, names: &[String]) -> Result<(), String> {
        if names.is_empty() {
            return Err("nenhum nó selecionado".into());
        }
        self.apply(Command::Batch(
            names
                .iter()
                .map(|n| Command::RemoveNode { name: n.clone() })
                .collect(),
        ))
    }

    pub fn add_edge(&mut self, from: &str, to: &str, two_way: bool) -> Result<(), String> {
        if from == to {
            return Err("um trecho liga dois nós diferentes".into());
        }
        self.apply(Command::AddEdge(EdgeDef {
            from: from.to_string(),
            to: to.to_string(),
            two_way,
            capacity: None,
        }))
    }

    /// Changes direction and capacity of the edge `from -> to` (`capacity` 0: no limit).
    pub fn set_edge(
        &mut self,
        from: &str,
        to: &str,
        two_way: bool,
        capacity: usize,
    ) -> Result<(), String> {
        let mut edge = self.edge(from, to)?;
        edge.two_way = two_way;
        edge.capacity = (capacity > 0).then_some(capacity);
        self.apply(Command::SetEdge {
            from: from.to_string(),
            to: to.to_string(),
            edge,
        })
    }

    /// Turns a one-way edge around (`to -> from`).
    pub fn reverse_edge(&mut self, from: &str, to: &str) -> Result<(), String> {
        let mut edge = self.edge(from, to)?;
        std::mem::swap(&mut edge.from, &mut edge.to);
        self.apply(Command::SetEdge {
            from: from.to_string(),
            to: to.to_string(),
            edge,
        })
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) -> Result<(), String> {
        self.apply(Command::RemoveEdge {
            from: from.to_string(),
            to: to.to_string(),
        })
    }

    /// The node within `radius` metres of `p` (model plan coordinates), nearest first.
    pub fn node_near(&self, p: [f64; 3], radius: f64) -> Option<String> {
        let net = self.model().network.as_ref()?;
        let i = nearest_node(net, [p[0], p[1], 0.0])?;
        let n = &net.nodes[i];
        let d = ((n.position[0] - p[0]).powi(2) + (n.position[1] - p[1]).powi(2)).sqrt();
        (d <= radius).then(|| n.name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_drawn_click_by_click_is_one_undo_step_per_click() {
        let mut e = Editor::new("rede");
        let a = e.add_node([0.2, 0.1, 0.0], 1.0, None, true).unwrap();
        let b = e.add_node([4.9, 0.0, 0.0], 1.0, Some(&a), true).unwrap();
        let c = e.add_node([5.0, 3.2, 0.0], 1.0, Some(&b), false).unwrap();
        assert_eq!(nodes(e.model()).len(), 3);
        assert_eq!(nodes(e.model())[1].position, [5.0, 0.0, 0.0]);
        let list = edges(e.model());
        assert_eq!(list.len(), 2);
        assert!(list[0].two_way && !list[1].two_way);
        assert!((list[0].length - 5.0).abs() < 1e-9);
        assert_eq!(
            (list[1].from.as_str(), list[1].to.as_str()),
            (b.as_str(), c.as_str())
        );
        // Undo removes the last node together with its edge.
        assert!(e.doc.undo());
        assert_eq!(nodes(e.model()).len(), 2);
        assert_eq!(edges(e.model()).len(), 1);
        assert!(e.doc.undo());
        assert!(e.doc.undo());
        assert!(e.model().network.is_none() || nodes(e.model()).is_empty());
    }

    #[test]
    fn edges_change_direction_capacity_and_go() {
        let mut e = Editor::new("rede");
        let a = e.add_node([0.0; 3], 0.0, None, true).unwrap();
        let b = e.add_node([3.0, 0.0, 0.0], 0.0, Some(&a), false).unwrap();
        e.set_edge(&a, &b, false, 2).unwrap();
        assert!(edges(e.model())[0].controlled());
        assert_eq!(edges(e.model())[0].capacity, Some(2));
        e.set_edge(&a, &b, true, 0).unwrap();
        assert!(!edges(e.model())[0].controlled() && edges(e.model())[0].two_way);
        e.reverse_edge(&a, &b).unwrap();
        assert_eq!(edges(e.model())[0].from, b);
        assert!(e.add_edge(&a, &a, true).is_err());
        // Two-way b -> a already covers a -> b.
        assert!(e.add_edge(&a, &b, true).is_err());
        e.remove_edge(&b, &a).unwrap();
        assert!(edges(e.model()).is_empty());
        assert!(e.set_edge(&a, &b, true, 1).is_err());
    }

    #[test]
    fn nodes_move_rename_are_found_near_a_point_and_are_removed_with_their_edges() {
        let mut e = Editor::new("rede");
        let a = e.add_node([0.0; 3], 0.5, None, true).unwrap();
        let b = e.add_node([4.0, 0.0, 0.0], 0.5, Some(&a), true).unwrap();
        e.move_node(&b, [4.2, 1.1, 0.0], 0.5).unwrap();
        assert_eq!(nodes(e.model())[1].position, [4.0, 1.0, 0.0]);
        assert_eq!(
            e.node_near([3.8, 1.2, 0.0], 0.6).as_deref(),
            Some(b.as_str())
        );
        assert_eq!(e.node_near([2.0, 0.0, 0.0], 0.6), None);
        e.rename_node(&b, "doca").unwrap();
        assert_eq!(edges(e.model())[0].to, "doca");
        assert!(e.rename_node("doca", &a).is_err());
        assert!(e.rename_node("doca", " ").is_err());
        e.remove_nodes(&["doca".to_string()]).unwrap();
        assert_eq!(nodes(e.model()).len(), 1);
        assert!(edges(e.model()).is_empty());
        assert!(e.remove_nodes(&[]).is_err());
    }
}
