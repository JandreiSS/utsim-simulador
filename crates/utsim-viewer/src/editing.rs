//! Pure logic of the model editor (phase 3): ground grid, naming, hit-testing of
//! connections, the automatic layout as one undoable command, run preparation and
//! issue grouping. No window and no GPU, so it is all unit-tested; the Bevy systems
//! in the binary only translate mouse and keyboard into calls to these functions.

use utsim_core::RunReport;
use utsim_edit::{Command, Issue};
use utsim_model::Model;
use utsim_model::replay::{Replay, kind_name};

use crate::geometry::{self, Geom};
use crate::handling::Handling;
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

/// Snaps `v` to a grid of `step` (no snapping when `step` is not positive). Never
/// returns `-0.0`.
pub fn snap_to(v: f64, step: f64) -> f64 {
    if step.is_finite() && step > 0.0 {
        (v / step).round() * step + 0.0
    } else {
        v + 0.0
    }
}

/// A point of the Bevy ground plane `(x, z)` as a model position, snapped to `step`
/// when given (the editor's "Encaixar na grade").
pub fn ground_to_model_snapped(x: f32, z: f32, step: Option<f64>) -> [f64; 3] {
    let s = step.unwrap_or(0.0);
    [snap_to(x as f64, s), snap_to(-(z as f64), s), 0.0]
}

/// Portuguese base name for a new object of palette entry `id` (`Document::unique_name`
/// makes it unique).
pub fn name_base(id: &str) -> &'static str {
    utsim_edit::palette_entry(id).map_or("objeto", |e| e.name_base)
}

/// Icon drawn before each palette entry (glyphs of egui's default fonts).
pub fn palette_icon(id: &str) -> &'static str {
    match id {
        "Source" => "➡",
        "Queue" => "☰",
        "Processor" => "⚙",
        "Sink" => "⏏",
        "Combiner" => "🔧",
        "Separator" => "⊗",
        "Conveyor" => "⏩",
        "Rack" => "📦",
        "Transporter" => "🚚",
        "Operator" => "☺",
        "Forklift" => "🚜",
        "Agv" => "⛭",
        "Dispatcher" => "☎",
        _ => "□",
    }
}

/// Every selected object: the primary one first, then the others.
pub fn selection_of(primary: &Option<String>, others: &[String]) -> Vec<String> {
    primary.iter().chain(others).cloned().collect()
}

/// Shift-click on `name`: removes it from the selection when it is there (the next one
/// becomes the primary), else adds it and makes it the primary.
pub fn toggle_selected(primary: &mut Option<String>, others: &mut Vec<String>, name: &str) {
    if primary.as_deref() == Some(name) {
        *primary = (!others.is_empty()).then(|| others.remove(0));
    } else if let Some(k) = others.iter().position(|o| o == name) {
        others.remove(k);
    } else {
        if let Some(p) = primary.take() {
            others.insert(0, p);
        }
        *primary = Some(name.to_string());
    }
}

/// Replaces the selection by `names` (the first is the primary), or adds them to it
/// when `add`.
pub fn select_names(
    primary: &mut Option<String>,
    others: &mut Vec<String>,
    names: Vec<String>,
    add: bool,
) {
    if !add {
        *primary = None;
        others.clear();
    }
    for n in names {
        if primary.as_deref() != Some(n.as_str()) && !others.contains(&n) {
            if primary.is_none() {
                *primary = Some(n);
            } else {
                others.push(n);
            }
        }
    }
}

/// Indices of the `points` (window pixels) inside the box with corners `a` and `b`.
pub fn inside_box(points: &[Option<[f32; 2]>], a: [f32; 2], b: [f32; 2]) -> Vec<usize> {
    let (x0, x1) = (a[0].min(b[0]), a[0].max(b[0]));
    let (y0, y1) = (a[1].min(b[1]), a[1].max(b[1]));
    points
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let p = (*p)?;
            (p[0] >= x0 && p[0] <= x1 && p[1] >= y0 && p[1] <= y1).then_some(i)
        })
        .collect()
}

/// Undo grouping of a command from the panels: the look edits of one gesture (dragging
/// in the colour picker sends one `SetVisual` per frame) merge into one undo step until
/// the mouse button is released. Other commands are steps of their own.
pub fn coalesce_key(command: &Command) -> Option<String> {
    match command {
        Command::SetVisual { name, .. } => Some(format!("aparencia:{name}")),
        Command::Batch(inner)
            if !inner.is_empty()
                && inner.iter().all(|c| matches!(c, Command::SetVisual { .. })) =>
        {
            let names: Vec<&str> = inner
                .iter()
                .filter_map(|c| match c {
                    Command::SetVisual { name, .. } => Some(name.as_str()),
                    _ => None,
                })
                .collect();
            Some(format!("aparencia:{}", names.join(",")))
        }
        _ => None,
    }
}

/// Offset of the `n`-th paste (n >= 1) of the same clipboard: two metres right and two
/// down per paste, so the copies do not land on top of each other.
pub fn paste_offset(n: u32) -> [f64; 3] {
    let k = 2.0 * n.max(1) as f64;
    [k, -k, 0.0]
}

/// The keyboard and mouse shortcuts of `Editar`, as shown in the "Atalhos" window
/// (shortcut, what it does).
pub const SHORTCUTS: [(&str, &str); 24] = [
    ("Clique", "seleciona o bloco (ou remove a ligação clicada)"),
    ("Shift + clique", "põe ou tira o bloco da seleção"),
    (
        "Shift + arrastar no chão",
        "seleção por caixa (soma à seleção)",
    ),
    ("Arrastar um bloco", "move o bloco ou a seleção inteira"),
    ("Arrastar no chão", "desloca a vista"),
    ("Botão direito + arrastar", "gira a vista"),
    ("Roda do mouse", "aproxima e afasta"),
    (
        "Biblioteca: clique e clique no chão",
        "coloca um objeto (Shift mantém a ferramenta)",
    ),
    (
        "Biblioteca: arrastar até o chão",
        "coloca um objeto onde soltar",
    ),
    ("Setas", "move a seleção um passo da grade"),
    ("Del", "exclui a seleção"),
    ("C", "liga o bloco selecionado ao próximo clicado"),
    ("Esc", "cancela a ferramenta e fecha os diálogos"),
    ("F", "enquadra o modelo"),
    ("F1", "mostra ou esconde esta janela"),
    ("Ctrl + A", "seleciona todos os objetos"),
    ("Ctrl + C", "copia a seleção"),
    ("Ctrl + X", "recorta a seleção"),
    (
        "Ctrl + V",
        "cola (nomes únicos, ligações internas preservadas)",
    ),
    ("Ctrl + D", "duplica a seleção"),
    ("Ctrl + Z", "desfaz"),
    ("Ctrl + Y ou Ctrl + Shift + Z", "refaz"),
    ("Ctrl + S", "salva"),
    ("Ctrl + N", "novo modelo"),
];

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

pub use utsim_scene::network::{NetSeg, NetworkView};

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
    /// Per connection: the source hands its items to a transporter (`transport` set), so
    /// the link is only logical flow and the vehicle carries the item.
    pub logical: Vec<bool>,
    /// Footprint and orientation of each object (belts and racks take it from the model).
    pub geoms: Vec<Geom>,
}

impl EditView {
    pub fn of(model: &Model) -> Self {
        let index = |name: &str| model.objects.iter().position(|o| o.name == name);
        let links: Vec<((usize, usize), bool)> = model
            .connections
            .iter()
            .filter_map(|(a, b)| {
                let ia = index(a)?;
                Some(((ia, index(b)?), model.objects[ia].transport.is_some()))
            })
            .collect();
        let kinds: Vec<&'static str> = model.objects.iter().map(|o| kind_name(&o.kind)).collect();
        let positions: Vec<[f32; 3]> = model
            .objects
            .iter()
            .map(|o| layout::to_bevy(o.position))
            .collect();
        let connections: Vec<(usize, usize)> = links.iter().map(|l| l.0).collect();
        let handlings: Vec<Option<Handling>> = model
            .objects
            .iter()
            .map(|o| Handling::of(&o.kind))
            .collect();
        let geoms = geometry::build(&kinds, &handlings, &positions, &connections);
        EditView {
            names: model.objects.iter().map(|o| o.name.clone()).collect(),
            kinds,
            positions,
            connections,
            logical: links.iter().map(|l| l.1).collect(),
            geoms,
        }
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }
}

/// Whether the model has several objects and every one sits at (0,0,0).
pub fn needs_layout(model: &Model) -> bool {
    let raw: Vec<[f64; 3]> = model.objects.iter().map(|o| o.position).collect();
    raw.len() >= 2 && layout::all_at_origin(&raw)
}

/// The automatic layout as a single undoable `Batch` of `Move` commands, only when
/// the model has several objects and every one sits at (0,0,0). Positions are
/// snapped to the grid. Models with a path network are left alone: the network has
/// its own coordinates and the blocks must be placed at its nodes.
pub fn auto_layout_command(model: &Model) -> Option<Command> {
    if !needs_layout(model) || model.network.is_some() {
        return None;
    }
    let raw: Vec<[f64; 3]> = model.objects.iter().map(|o| o.position).collect();
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

/// For each connection of `replay`, whether its source object hands items to a
/// transporter in `model` (a logical link, see [`EditView::logical`]).
pub fn logical_flags(model: &Model, replay: &Replay) -> Vec<bool> {
    replay
        .connections
        .iter()
        .map(|&(a, _)| {
            replay.blocks.get(a).is_some_and(|block| {
                model
                    .objects
                    .iter()
                    .any(|o| o.name == block.name && o.transport.is_some())
            })
        })
        .collect()
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
        fluids: None,
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
        for e in utsim_edit::PALETTE {
            assert_ne!(palette_icon(e.id), "□", "{}", e.id);
        }
        assert_eq!(name_base("Queue"), "fila");
        assert_eq!(name_base("Operator"), "operador");
        assert_eq!(name_base("???"), "objeto");
    }

    #[test]
    fn snapping_follows_the_configured_step_or_none() {
        assert_eq!(snap_to(2.6, 0.5), 2.5);
        assert_eq!(snap_to(2.6, 2.0), 2.0);
        assert_eq!(snap_to(2.6, 0.0), 2.6);
        assert_eq!(snap_to(-0.1, 0.5).to_bits(), 0.0f64.to_bits());
        assert_eq!(
            ground_to_model_snapped(3.3, -1.2, Some(0.5)),
            [3.5, 1.0, 0.0]
        );
        let free = ground_to_model_snapped(3.3, -1.2, None);
        assert!((free[0] - 3.3).abs() < 1e-6 && (free[1] - 1.2).abs() < 1e-6);
    }

    #[test]
    fn shift_click_adds_and_removes_and_the_box_selects() {
        let (mut p, mut o) = (Some("a".to_string()), Vec::new());
        toggle_selected(&mut p, &mut o, "b");
        assert_eq!(selection_of(&p, &o), vec!["b", "a"]);
        toggle_selected(&mut p, &mut o, "a");
        assert_eq!(selection_of(&p, &o), vec!["b"]);
        toggle_selected(&mut p, &mut o, "b");
        assert_eq!((p.clone(), o.len()), (None, 0));
        toggle_selected(&mut p, &mut o, "c");
        select_names(&mut p, &mut o, vec!["d".into(), "c".into()], true);
        assert_eq!(selection_of(&p, &o), vec!["c", "d"]);
        select_names(&mut p, &mut o, vec!["e".into()], false);
        assert_eq!(selection_of(&p, &o), vec!["e"]);

        let points = [
            Some([10.0, 10.0]),
            Some([50.0, 40.0]),
            None,
            Some([200.0, 5.0]),
        ];
        // Corners in any order.
        assert_eq!(inside_box(&points, [60.0, 0.0], [0.0, 50.0]), vec![0, 1]);
        assert!(inside_box(&points, [0.0, 0.0], [1.0, 1.0]).is_empty());
    }

    #[test]
    fn look_edits_of_one_gesture_share_an_undo_step() {
        let look = |n: &str| Command::SetVisual {
            name: n.into(),
            visual: None,
        };
        assert_eq!(coalesce_key(&look("a")).as_deref(), Some("aparencia:a"));
        let batch = Command::Batch(vec![look("a"), look("b")]);
        assert_eq!(coalesce_key(&batch).as_deref(), Some("aparencia:a,b"));
        let mixed = Command::Batch(vec![look("a"), Command::Remove { name: "b".into() }]);
        assert_eq!(coalesce_key(&mixed), None);
        assert_eq!(coalesce_key(&Command::Batch(Vec::new())), None);

        let mut doc = chain(true);
        for c in ["#100000", "#200000", "#300000"] {
            let cmd = Command::SetVisual {
                name: "b".into(),
                visual: Some(utsim_model::VisualDef {
                    color: Some(c.into()),
                    ..Default::default()
                }),
            };
            let key = coalesce_key(&cmd);
            doc.apply_coalescing(cmd, key.as_deref()).unwrap();
        }
        doc.end_gesture();
        assert!(doc.undo());
        assert_eq!(doc.model().objects[1].visual, None);
    }

    #[test]
    fn pastes_step_away_and_shortcuts_are_listed_once() {
        assert_eq!(paste_offset(1), [2.0, -2.0, 0.0]);
        assert_eq!(paste_offset(3), [6.0, -6.0, 0.0]);
        assert_eq!(paste_offset(0), paste_offset(1));
        let mut keys: Vec<&str> = SHORTCUTS.iter().map(|s| s.0).collect();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), SHORTCUTS.len());
        for key in ["Ctrl + C", "Ctrl + V", "Ctrl + D", "Shift + clique", "F1"] {
            assert!(SHORTCUTS.iter().any(|s| s.0 == key), "{key}");
        }
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
    fn auto_layout_is_not_applied_to_models_with_a_network() {
        let text = r#"Model(
            name: "n", run: (stop_time: 10.0),
            objects: [(name: "s", kind: Source(interarrival: Const(1.0))), (name: "k", kind: Sink)],
            connections: [("s", "k")],
            network: Some((
                nodes: [(name: "a", position: (1.0, 2.0, 0.0)), (name: "b", position: (5.0, 2.0, 0.0))],
                edges: [(from: "a", to: "b")],
            )),
        )"#;
        let model = Model::parse(text).unwrap();
        assert!(needs_layout(&model), "still needs positions");
        assert!(auto_layout_command(&model).is_none());
        assert!(auto_layout_command(chain(false).model()).is_some());
    }

    #[test]
    fn transported_sources_make_logical_connections() {
        let text = r#"Model(
            name: "n", run: (stop_time: 10.0),
            objects: [
                (name: "q", kind: Queue(), transport: "d", position: (0.0, 0.0, 0.0)),
                (name: "s", kind: Sink, position: (5.0, 0.0, 0.0)),
                (name: "v", kind: Transporter(speed: 1.0), position: (0.0, 0.0, 0.0)),
                (name: "d", kind: Dispatcher(team: ["v"]), position: (0.0, 0.0, 0.0)),
            ],
            connections: [("q", "s")],
        )"#;
        let model = Model::parse(text).unwrap();
        assert_eq!(EditView::of(&model).logical, vec![true]);
        let replay = model.replay(0).unwrap();
        assert_eq!(logical_flags(&model, &replay), vec![true]);
        assert_eq!(
            EditView::of(chain(true).model()).logical,
            vec![false, false]
        );
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
                edges: [(from: "a", to: "b", two_way: false), (from: "a", to: "b", capacity: Some(2))],
            )),
        )"#;
        let view = NetworkView::of(&Model::parse(text).unwrap());
        assert_eq!(view.nodes, vec![[1.0, 0.0, -2.0], [5.0, 0.0, -2.0]]);
        assert_eq!(view.segments.len(), 2);
        assert!(!view.segments[0].two_way && view.segments[1].two_way);
        assert_eq!(view.segments[0].capacity, None);
        assert_eq!(view.segments[1].capacity, Some(2));
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
