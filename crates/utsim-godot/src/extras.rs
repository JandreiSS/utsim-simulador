//! Fluids and token flows of a playback (G3), with no Godot types.
//!
//! - **Fluids:** the nodes of the model's fluid network as the Bevy viewer lays them out
//!   (`utsim_scene::fluids::FluidView`: the model's positions, or a strip below the
//!   blocks), tanks sized by their capacity, the pipes between nodes and the converters'
//!   lines to their discrete object; per frame, the level of each tank (fraction and
//!   percentage), the rate and its colour, and which links carry flow.
//! - **Token flows:** each flow of the model as a 2D graph (layered layout of the
//!   activities along `next`), the tokens in each activity at `t` and the counts sampled
//!   over the whole run, for the panel's chart.
//!
//! Positions are Godot coordinates (the scene's layout space is the same space).

use utsim_flow::Activity;
use utsim_scene::fluids::{self, NodeShape};
use utsim_scene::layout;

use crate::playback::Playback;

/// Colour of the liquid of a tank by its rate, and of the marker of the other nodes
/// (the Bevy viewer's palette; a pipe is lit while fluid goes through it).
pub fn fluid_color(shape: NodeShape, rate: f64) -> [f32; 3] {
    const EPS: f64 = 1e-9;
    match shape {
        NodeShape::Tank if rate > EPS => [0.15, 0.70, 0.95],
        NodeShape::Tank if rate < -EPS => [0.95, 0.55, 0.20],
        NodeShape::Tank => [0.12, 0.45, 0.95],
        NodeShape::Source => [0.25, 0.75, 0.35],
        NodeShape::Sink => [0.80, 0.32, 0.32],
        NodeShape::Pipe if rate.abs() > EPS => [0.25, 0.85, 0.95],
        NodeShape::Pipe => [0.95, 0.68, 0.20],
        NodeShape::Converter => [0.75, 0.45, 0.95],
    }
}

pub fn shape_name(shape: NodeShape) -> &'static str {
    match shape {
        NodeShape::Tank => "tank",
        NodeShape::Source => "source",
        NodeShape::Sink => "sink",
        NodeShape::Pipe => "pipe",
        NodeShape::Converter => "converter",
    }
}

/// One fluid node, drawn once.
#[derive(Clone, Debug, PartialEq)]
pub struct FluidNodeInfo {
    pub name: String,
    pub kind_label: &'static str,
    pub shape: NodeShape,
    pub position: [f32; 3],
    /// Tanks: radius and height of the cylinder; others: half the marker's edge twice.
    pub radius: f32,
    pub height: f32,
    pub capacity: f64,
    /// Height where the pipes meet the node.
    pub port: f32,
}

pub fn fluid_nodes(p: &Playback) -> Vec<FluidNodeInfo> {
    p.scene
        .fluid
        .nodes
        .iter()
        .map(|n| {
            let half = fluids::node_half(n);
            FluidNodeInfo {
                name: n.name.clone(),
                kind_label: n.kind_label,
                shape: n.shape,
                position: n.pos,
                radius: half[0],
                height: half[1] * 2.0,
                capacity: n.capacity,
                port: fluids::port_height(n),
            }
        })
        .collect()
}

/// Pipes as `(from, to)` node indices.
pub fn fluid_links(p: &Playback) -> Vec<(usize, usize)> {
    p.scene.fluid.links.clone()
}

/// Converters: `(node, drawn block, items become fluid)`.
pub fn fluid_converters(p: &Playback) -> Vec<(usize, usize, bool)> {
    p.scene
        .fluid
        .converters
        .iter()
        .filter_map(|c| Some((c.node, p.drawn_index(c.block)?, c.into_fluid)))
        .collect()
}

/// The fluid network at one instant (one entry per node, one per link).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FluidFrame {
    /// Tanks: filled fraction `0..=1` (0 for the other nodes).
    pub fill: Vec<f32>,
    pub level: Vec<f64>,
    pub rate: Vec<f64>,
    pub color: Vec<[f32; 3]>,
    /// `"tanque: 25.0% enchendo"`.
    pub text: Vec<String>,
    pub link_active: Vec<bool>,
}

pub fn fluid_frame(p: &Playback, t: f64) -> FluidFrame {
    const EPS: f64 = 1e-9;
    let view = &p.scene.fluid;
    let mut f = FluidFrame::default();
    for (i, n) in view.nodes.iter().enumerate() {
        let s = p.scene.fluid_sample(i, t);
        let tank = n.shape == NodeShape::Tank;
        f.fill.push(if tank {
            fluids::fill_fraction(s.level, n.capacity)
        } else {
            0.0
        });
        f.level.push(s.level);
        f.rate.push(s.rate);
        f.color.push(fluid_color(n.shape, s.rate));
        f.text.push(if tank {
            format!(
                "{}: {} {}",
                n.name,
                fluids::percent_text(s.level, n.capacity),
                fluids::rate_label(s.rate)
            )
        } else if s.rate.abs() > EPS {
            format!("{}: {:.2}/t", n.name, s.rate.abs())
        } else {
            n.name.clone()
        });
    }
    for &(a, b) in &view.links {
        // A link carries flow when the node that is not a tank moves fluid; between two
        // tanks, when either one is filling or draining (the Bevy viewer's rule).
        let pass: Vec<usize> = [a, b]
            .into_iter()
            .filter(|&i| view.nodes[i].shape != NodeShape::Tank)
            .collect();
        let moving = |i: usize| f.rate[i].abs() > EPS;
        f.link_active.push(if pass.is_empty() {
            moving(a) || moving(b)
        } else {
            pass.iter().any(|&i| moving(i))
        });
    }
    f
}

// ---------------------------------------------------------------------------------------
// Token flows
// ---------------------------------------------------------------------------------------

/// Portuguese label of an activity kind (the Bevy flow editor's names).
pub fn activity_label(kind: &Activity) -> &'static str {
    match kind {
        Activity::Source { .. } => "Gerar tokens",
        Activity::Schedule { .. } => "Agenda",
        Activity::OnEvent { .. } => "Ao evento",
        Activity::Delay { .. } => "Esperar",
        Activity::Decide { .. } => "Decidir",
        Activity::Assign { .. } => "Atribuir",
        Activity::Acquire { .. } => "Adquirir",
        Activity::Release { .. } => "Liberar",
        Activity::WaitFor { .. } => "Aguardar evento",
        Activity::CreateItem { .. } => "Criar item",
        Activity::Batch { .. } => "Lote",
        Activity::Sink => "Fim",
    }
}

/// One activity of a flow graph, at `(x, y)` of the 2D panel (metres of the layered
/// layout; the panel scales them).
#[derive(Clone, Debug, PartialEq)]
pub struct FlowNode {
    pub name: String,
    pub label: &'static str,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlowGraph {
    pub name: String,
    /// Replay block of the flow (where its token counts are recorded).
    pub block: Option<usize>,
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<(usize, usize)>,
}

pub fn flow_graphs(p: &Playback) -> Vec<FlowGraph> {
    let Some(model) = p.model.as_ref() else {
        return Vec::new();
    };
    model
        .flows
        .iter()
        .map(|f| {
            let index = |n: &str| f.activities.iter().position(|a| a.name == n);
            let edges: Vec<(usize, usize)> = f
                .activities
                .iter()
                .enumerate()
                .flat_map(|(i, a)| a.next.iter().filter_map(move |n| Some((i, index(n)?))))
                .collect();
            let at = layout::auto_layout(f.activities.len(), &edges);
            FlowGraph {
                name: f.name.clone(),
                block: p.scene.flow_block(&f.name),
                nodes: f
                    .activities
                    .iter()
                    .zip(at)
                    .map(|(a, q)| FlowNode {
                        name: a.name.clone(),
                        label: activity_label(&a.kind),
                        x: q[0],
                        y: q[2],
                    })
                    .collect(),
                edges,
            }
        })
        .collect()
}

/// Tokens in each activity of `graph` at `t`.
pub fn flow_counts(p: &Playback, graph: &FlowGraph, t: f64) -> Vec<u32> {
    (0..graph.nodes.len())
        .map(|a| {
            graph
                .block
                .map_or(0, |b| p.scene.timeline.tokens_at(b, a, t))
        })
        .collect()
}

/// Tokens of every activity at `samples` instants over the run: `(times, counts per
/// activity)`.
pub fn flow_series(p: &Playback, graph: &FlowGraph, samples: usize) -> (Vec<f64>, Vec<Vec<u32>>) {
    let n = samples.max(2);
    let end = p.end().max(0.0);
    let times: Vec<f64> = (0..n).map(|k| end * k as f64 / (n - 1) as f64).collect();
    let counts = (0..graph.nodes.len())
        .map(|a| {
            times
                .iter()
                .map(|&t| {
                    graph
                        .block
                        .map_or(0, |b| p.scene.timeline.tokens_at(b, a, t))
                })
                .collect()
        })
        .collect();
    (times, counts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::tests::example;

    fn playback(name: &str) -> Playback {
        let m = example(name);
        let replay = m.replay(0).unwrap();
        Playback::new(&replay, Some(&m), None, false)
    }

    #[test]
    fn tanks_have_levels_rates_and_colours() {
        let p = playback("tanques.ron");
        let nodes = fluid_nodes(&p);
        assert!(nodes.iter().any(|n| n.shape == NodeShape::Tank));
        assert!(!fluid_links(&p).is_empty());
        let end = p.end();
        let mut changed = false;
        let first = fluid_frame(&p, 0.0);
        assert_eq!(first.fill.len(), nodes.len());
        assert_eq!(first.link_active.len(), fluid_links(&p).len());
        for k in 1..20 {
            let f = fluid_frame(&p, end * k as f64 / 20.0);
            for (i, n) in nodes.iter().enumerate() {
                assert!((0.0..=1.0).contains(&f.fill[i]));
                if n.shape == NodeShape::Tank && (f.fill[i] - first.fill[i]).abs() > 1e-3 {
                    changed = true;
                    assert!(f.text[i].contains('%'));
                }
            }
        }
        assert!(changed, "algum tanque muda de nível");
        assert_eq!(fluid_color(NodeShape::Tank, 1.0), [0.15, 0.70, 0.95]);
        assert_ne!(
            fluid_color(NodeShape::Tank, -1.0),
            fluid_color(NodeShape::Tank, 0.0)
        );
    }

    #[test]
    fn flows_are_graphs_with_token_counts_over_time() {
        let p = playback("fluxo_docas.ron");
        let graphs = flow_graphs(&p);
        assert!(!graphs.is_empty());
        let g = &graphs[0];
        assert!(g.block.is_some());
        assert!(g.nodes.len() >= 2 && !g.edges.is_empty());
        let (times, counts) = flow_series(&p, g, 50);
        assert_eq!(times.len(), 50);
        assert!(
            counts.iter().flatten().any(|&c| c > 0),
            "algum token no tempo"
        );
        let mid = flow_counts(&p, g, p.end() / 2.0);
        assert_eq!(mid.len(), g.nodes.len());
    }
}
