//! Pure logic of the model editor (phase 3): ground grid, naming, hit-testing of
//! connections, the automatic layout as one undoable command, run preparation and
//! issue grouping. No window and no GPU, so it is all unit-tested; the Bevy systems
//! in the binary only translate mouse and keyboard into calls to these functions.

use utsim_core::RunReport;
use utsim_edit::{Command, Issue};
use utsim_model::Model;
use utsim_model::replay::{Replay, kind_name};

use crate::layout;

/// Ground grid step, in model units.
pub const GRID: f64 = 1.0;
/// Largest `stop_time` simulated by default when the user presses "Simular".
pub const DEFAULT_STOP_CAP: f64 = 2000.0;
/// Distance, in pixels, within which a click grabs a connection line.
pub const LINE_PICK_PIXELS: f32 = 8.0;

/// Snaps a coordinate to the grid. Never returns `-0.0`.
pub fn snap(v: f64) -> f64 {
    (v / GRID).round() * GRID + 0.0
}

/// A point of the Bevy ground plane `(x, z)` as a snapped model position. Model `y`
/// is Bevy `-z` and the height is 0 (see `layout`).
pub fn ground_to_model(x: f32, z: f32) -> [f64; 3] {
    [snap(x as f64), snap(-(z as f64)), 0.0]
}

/// Portuguese base name for a new object of `kind` (`Document::unique_name` makes it unique).
pub fn name_base(kind: &str) -> &'static str {
    match kind {
        "Source" => "chegada",
        "Queue" => "fila",
        "Processor" => "processo",
        "Combiner" => "montagem",
        "Separator" => "desmontagem",
        "Sink" => "saida",
        "Transporter" => "transportador",
        "Dispatcher" => "despachante",
        "Conveyor" => "esteira",
        "Rack" => "porta_paletes",
        _ => "objeto",
    }
}

/// Mobile resources are not wired by ports: the message shown when the user tries to
/// connect one (`None` for ordinary blocks).
pub fn connect_blocked(kind: &str) -> Option<&'static str> {
    match kind {
        "Transporter" | "Dispatcher" => Some(
            "Transportadores e despachantes não usam ligações. Use o campo \"transport\" \
             (nos objetos que enviam itens) ou \"operators\" (nos processos).",
        ),
        _ => None,
    }
}

/// One edge of the path network, in Bevy space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NetSeg {
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub two_way: bool,
}

/// The model's path network (nodes and edges) ready to draw on the ground.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkView {
    pub nodes: Vec<[f32; 3]>,
    pub segments: Vec<NetSeg>,
}

impl NetworkView {
    /// Edges naming a missing node are skipped.
    pub fn of(model: &Model) -> Self {
        let Some(net) = &model.network else {
            return Self::default();
        };
        let at = |name: &str| {
            net.nodes
                .iter()
                .find(|n| n.name == name)
                .map(|n| layout::to_bevy(n.position))
        };
        NetworkView {
            nodes: net
                .nodes
                .iter()
                .map(|n| layout::to_bevy(n.position))
                .collect(),
            segments: net
                .edges
                .iter()
                .filter_map(|e| {
                    Some(NetSeg {
                        a: at(&e.from)?,
                        b: at(&e.to)?,
                        two_way: e.two_way,
                    })
                })
                .collect(),
        }
    }
}

/// Distance from point `p` to the segment `a`-`b`.
pub fn point_segment_distance(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    let (abx, aby) = (b[0] - a[0], b[1] - a[1]);
    let len2 = abx * abx + aby * aby;
    let t = if len2 == 0.0 {
        0.0
    } else {
        (((p[0] - a[0]) * abx + (p[1] - a[1]) * aby) / len2).clamp(0.0, 1.0)
    };
    let (cx, cy) = (a[0] + t * abx, a[1] + t * aby);
    ((p[0] - cx).powi(2) + (p[1] - cy).powi(2)).sqrt()
}

/// Index of the segment closest to `p`, if it is within `max` of it.
pub fn nearest_segment(p: [f32; 2], segments: &[([f32; 2], [f32; 2])], max: f32) -> Option<usize> {
    segments
        .iter()
        .enumerate()
        .map(|(i, (a, b))| (i, point_segment_distance(p, *a, *b)))
        .filter(|(_, d)| *d <= max)
        .min_by(|x, y| x.1.total_cmp(&y.1))
        .map(|(i, _)| i)
}

/// What the editor draws: objects in model order, positions already in Bevy space,
/// connections as indices (connections naming a missing object are dropped).
pub struct EditView {
    pub names: Vec<String>,
    pub kinds: Vec<&'static str>,
    pub positions: Vec<[f32; 3]>,
    pub connections: Vec<(usize, usize)>,
}

impl EditView {
    pub fn of(model: &Model) -> Self {
        let index = |name: &str| model.objects.iter().position(|o| o.name == name);
        EditView {
            names: model.objects.iter().map(|o| o.name.clone()).collect(),
            kinds: model.objects.iter().map(|o| kind_name(&o.kind)).collect(),
            positions: model
                .objects
                .iter()
                .map(|o| layout::to_bevy(o.position))
                .collect(),
            connections: model
                .connections
                .iter()
                .filter_map(|(a, b)| Some((index(a)?, index(b)?)))
                .collect(),
        }
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }
}

/// The automatic layout as a single undoable `Batch` of `Move` commands, only when
/// the model has several objects and every one sits at (0,0,0). Positions are
/// snapped to the grid.
pub fn auto_layout_command(model: &Model) -> Option<Command> {
    let raw: Vec<[f64; 3]> = model.objects.iter().map(|o| o.position).collect();
    if raw.len() < 2 || !layout::all_at_origin(&raw) {
        return None;
    }
    let view = EditView::of(model);
    let bevy = layout::auto_layout(raw.len(), &view.connections);
    let moves = model
        .objects
        .iter()
        .zip(bevy)
        .map(|(o, p)| Command::Move {
            name: o.name.clone(),
            position: ground_to_model(p[0], p[2]),
        })
        .collect();
    Some(Command::Batch(moves))
}

/// Default `stop_time` of a quick simulation run.
pub fn default_stop(stop_time: f64) -> f64 {
    if stop_time.is_finite() && stop_time > 0.0 {
        stop_time.min(DEFAULT_STOP_CAP)
    } else {
        DEFAULT_STOP_CAP
    }
}

/// The model to simulate for a run limited to `stop` (the warm-up only affects
/// statistics, so it is dropped when it would make the run invalid).
pub fn run_model(model: &Model, stop: f64) -> Model {
    let mut m = model.clone();
    m.run.stop_time = stop;
    if m.run.warmup >= stop {
        m.run.warmup = 0.0;
    }
    m
}

/// Issues grouped for display: model-wide ones first (`None`), then one entry per
/// object in order of appearance.
pub fn group_issues(issues: &[Issue]) -> Vec<(Option<String>, Vec<String>)> {
    let mut groups: Vec<(Option<String>, Vec<String>)> = Vec::new();
    for issue in issues.iter().filter(|i| i.object.is_none()) {
        match groups.first_mut().filter(|g| g.0.is_none()) {
            Some(g) => g.1.push(issue.message.clone()),
            None => groups.insert(0, (None, vec![issue.message.clone()])),
        }
    }
    for issue in issues.iter().filter(|i| i.object.is_some()) {
        match groups.iter_mut().find(|g| g.0 == issue.object) {
            Some(g) => g.1.push(issue.message.clone()),
            None => groups.push((issue.object.clone(), vec![issue.message.clone()])),
        }
    }
    groups
}

/// A replay with nothing in it: placeholder data while the viewer is in `Editar`
/// and no simulation has run yet.
pub fn empty_replay() -> Replay {
    Replay {
        model: String::new(),
        replication: 0,
        stop_time: 0.0,
        blocks: Vec::new(),
        connections: Vec::new(),
        records: Vec::new(),
        report: RunReport {
            replication: 0,
            seed: 0,
            stats_from: 0.0,
            end_time: 0.0,
            events: 0,
            blocks: Vec::new(),
        },
    }
}

/// The name `selected` has after `commands` ran (a `Rename` of it, possibly in a batch).
pub fn renamed(commands: &[Command], selected: &str) -> String {
    let mut name = selected.to_string();
    for c in commands {
        match c {
            Command::Rename { from, to } if *from == name => name = to.clone(),
            Command::Batch(inner) => name = renamed(inner, &name),
            _ => {}
        }
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;
    use utsim_edit::{Document, template};

    #[test]
    fn snapping_rounds_to_the_grid_without_negative_zero() {
        assert_eq!(snap(2.4), 2.0);
        assert_eq!(snap(2.5), 3.0);
        assert_eq!(snap(-0.2).to_bits(), 0.0f64.to_bits());
        assert_eq!(ground_to_model(3.6, -2.2), [4.0, 2.0, 0.0]);
        // Round trip with the layout convention: model (x, y, 0) -> Bevy (x, 0, -y).
        let p = layout::to_bevy([4.0, 2.0, 0.0]);
        assert_eq!(ground_to_model(p[0], p[2]), [4.0, 2.0, 0.0]);
    }

    #[test]
    fn every_library_kind_has_a_name_base() {
        for (kind, _) in utsim_edit::LIBRARY {
            assert_ne!(name_base(kind), "objeto", "{kind}");
        }
        assert_eq!(name_base("Queue"), "fila");
        assert_eq!(name_base("???"), "objeto");
    }

    #[test]
    fn point_to_segment_distance() {
        let (a, b) = ([0.0, 0.0], [10.0, 0.0]);
        assert_eq!(point_segment_distance([5.0, 3.0], a, b), 3.0);
        // Beyond the ends the closest point is the endpoint.
        assert_eq!(point_segment_distance([13.0, 4.0], a, b), 5.0);
        assert_eq!(point_segment_distance([-3.0, 4.0], a, b), 5.0);
        // Degenerate segment.
        assert_eq!(point_segment_distance([3.0, 4.0], a, a), 5.0);
    }

    #[test]
    fn nearest_segment_respects_the_limit() {
        let segs = [([0.0, 0.0], [10.0, 0.0]), ([0.0, 6.0], [10.0, 6.0])];
        assert_eq!(nearest_segment([5.0, 1.0], &segs, 8.0), Some(0));
        assert_eq!(nearest_segment([5.0, 5.0], &segs, 8.0), Some(1));
        assert_eq!(nearest_segment([5.0, 30.0], &segs, 8.0), None);
        assert_eq!(nearest_segment([5.0, 0.0], &[], 8.0), None);
    }

    fn chain(positions: bool) -> Document {
        let mut doc = Document::new("t");
        for (i, (kind, name)) in [("Source", "a"), ("Queue", "b"), ("Sink", "c")]
            .into_iter()
            .enumerate()
        {
            let p = if positions {
                [i as f64 * 3.0, 0.0, 0.0]
            } else {
                [0.0; 3]
            };
            doc.apply(Command::Add(Box::new(template(kind, name, p).unwrap())))
                .unwrap();
        }
        for (a, b) in [("a", "b"), ("b", "c")] {
            doc.apply(Command::Connect {
                from: a.into(),
                to: b.into(),
            })
            .unwrap();
        }
        doc
    }

    #[test]
    fn auto_layout_is_one_undoable_batch_on_the_grid() {
        let mut doc = chain(false);
        let cmd = auto_layout_command(doc.model()).expect("all at origin");
        let before = doc.model().clone();
        doc.apply(cmd).unwrap();
        let xs: Vec<f64> = doc.model().objects.iter().map(|o| o.position[0]).collect();
        assert_eq!(xs, vec![0.0, 5.0, 10.0]);
        assert!(
            doc.model()
                .objects
                .iter()
                .all(|o| o.position[0].fract() == 0.0 && o.position[1].fract() == 0.0)
        );
        assert!(auto_layout_command(doc.model()).is_none(), "only once");
        assert!(doc.undo());
        assert_eq!(doc.model(), &before);
    }

    #[test]
    fn auto_layout_skips_positioned_and_tiny_models() {
        assert!(auto_layout_command(chain(true).model()).is_none());
        assert!(auto_layout_command(Document::new("v").model()).is_none());
    }

    #[test]
    fn edit_view_converts_positions_and_drops_dangling_connections() {
        let doc = chain(true);
        let view = EditView::of(doc.model());
        assert_eq!(view.names, vec!["a", "b", "c"]);
        assert_eq!(view.kinds, vec!["Source", "Queue", "Sink"]);
        assert_eq!(view.positions[1], [3.0, 0.0, 0.0]);
        assert_eq!(view.connections, vec![(0, 1), (1, 2)]);
        assert_eq!(view.index_of("c"), Some(2));
    }

    #[test]
    fn run_preparation_caps_the_stop_and_fixes_warmup() {
        assert_eq!(default_stop(10_000.0), 2000.0);
        assert_eq!(default_stop(300.0), 300.0);
        assert_eq!(default_stop(f64::INFINITY), 2000.0);
        let mut doc = chain(true);
        doc.apply(Command::Set {
            key: "run.warmup".into(),
            value: "500.0".into(),
        })
        .unwrap();
        let m = run_model(doc.model(), 100.0);
        assert_eq!(m.run.stop_time, 100.0);
        assert_eq!(m.run.warmup, 0.0);
        assert_eq!(doc.model().run.stop_time, 10000.0, "document untouched");
    }

    #[test]
    fn issues_are_grouped_by_object() {
        let issue = |o: Option<&str>, m: &str| Issue {
            object: o.map(str::to_string),
            message: m.to_string(),
        };
        let groups = group_issues(&[
            issue(Some("x"), "um"),
            issue(None, "geral"),
            issue(Some("y"), "dois"),
            issue(Some("x"), "três"),
        ]);
        assert_eq!(
            groups,
            vec![
                (None, vec!["geral".to_string()]),
                (
                    Some("x".to_string()),
                    vec!["um".to_string(), "três".to_string()]
                ),
                (Some("y".to_string()), vec!["dois".to_string()]),
            ]
        );
    }

    #[test]
    fn the_placeholder_replay_builds_a_timeline() {
        let tl = crate::timeline::Timeline::build(
            &empty_replay(),
            crate::timeline::TransitSetting::Auto,
        );
        assert_eq!(tl.block_count(), 0);
        assert_eq!(tl.item_count(), 0);
    }

    #[test]
    fn mobile_resources_refuse_port_connections_in_portuguese() {
        for kind in ["Transporter", "Dispatcher"] {
            let msg = connect_blocked(kind).unwrap();
            assert!(msg.contains("transport") && msg.contains("operators"));
        }
        assert!(connect_blocked("Queue").is_none());
    }

    #[test]
    fn network_view_maps_nodes_and_edges_to_bevy_space() {
        let text = r#"Model(
            name: "n",
            run: (stop_time: 10.0),
            objects: [(name: "s", kind: Sink)],
            network: Some((
                nodes: [(name: "a", position: (1.0, 2.0, 0.0)), (name: "b", position: (5.0, 2.0, 0.0))],
                edges: [(from: "a", to: "b", two_way: false), (from: "a", to: "b")],
            )),
        )"#;
        let view = NetworkView::of(&Model::parse(text).unwrap());
        assert_eq!(view.nodes, vec![[1.0, 0.0, -2.0], [5.0, 0.0, -2.0]]);
        assert_eq!(view.segments.len(), 2);
        assert!(!view.segments[0].two_way && view.segments[1].two_way);
        assert_eq!(view.segments[0].b, [5.0, 0.0, -2.0]);
        assert_eq!(NetworkView::of(chain(true).model()), NetworkView::default());
    }

    #[test]
    fn rename_is_tracked_through_batches() {
        let cmds = [Command::Batch(vec![Command::Rename {
            from: "a".into(),
            to: "b".into(),
        }])];
        assert_eq!(renamed(&cmds, "a"), "b");
        assert_eq!(renamed(&cmds, "z"), "z");
    }
}
