//! Pure logic of the flow editor (phase 6): activity kinds and their Portuguese
//! labels, the automatic layered layout of a flow diagram, whole-flow commands and
//! the structural edits (add, remove, rename, connect). Nothing here needs a window.

use crate::texto::{fx, tx, vx};
use std::collections::BTreeMap;

use utsim_core::Dist;
use utsim_edit::Command;
use utsim_flow::{Activity, ActivityDef, BlockEvent, DecideRule, FlowDef, ResourceDef};

use crate::layout;

pub use utsim_scene::tokens::TokenTrack;

/// Activity kinds offered when adding an activity: RON variant name and the key
/// (`editor.ftl`) of its label.
pub const KINDS: [(&str, &str); 12] = [
    ("Source", "fluxo-tipo-gerar-tokens"),
    ("Schedule", "fluxo-tipo-agenda"),
    ("OnEvent", "fluxo-tipo-ao-evento"),
    ("Delay", "fluxo-tipo-esperar"),
    ("Decide", "fluxo-tipo-decidir"),
    ("Assign", "fluxo-tipo-atribuir"),
    ("Acquire", "fluxo-tipo-adquirir"),
    ("Release", "fluxo-tipo-liberar"),
    ("WaitFor", "fluxo-tipo-aguardar-evento"),
    ("CreateItem", "fluxo-tipo-criar-item"),
    ("Batch", "fluxo-tipo-lote"),
    ("Sink", "fluxo-tipo-fim"),
];

/// RON variant name of an activity kind.
pub fn kind_key(kind: &Activity) -> &'static str {
    match kind {
        Activity::Source { .. } => "Source",
        Activity::Schedule { .. } => "Schedule",
        Activity::OnEvent { .. } => "OnEvent",
        Activity::Delay { .. } => "Delay",
        Activity::Decide { .. } => "Decide",
        Activity::Assign { .. } => "Assign",
        Activity::Acquire { .. } => "Acquire",
        Activity::Release { .. } => "Release",
        Activity::WaitFor { .. } => "WaitFor",
        Activity::CreateItem { .. } => "CreateItem",
        Activity::Batch { .. } => "Batch",
        Activity::Sink => "Sink",
    }
}

/// Portuguese label of an activity kind.
pub fn kind_label(kind: &Activity) -> &'static str {
    let key = kind_key(kind);
    KINDS
        .iter()
        .find(|(k, _)| *k == key)
        .map_or("?", |(_, label)| tx(label))
}

pub fn event_label(e: BlockEvent) -> &'static str {
    match e {
        BlockEvent::Entry => "entrada",
        BlockEvent::Exit => "saída",
    }
}

/// A new activity of `kind` (one of [`KINDS`]) with sensible defaults, no successors.
pub fn new_activity(
    kind: &str,
    name: &str,
    objects: &[String],
    resources: &[ResourceDef],
) -> Option<ActivityDef> {
    let object = objects.first().cloned().unwrap_or_default();
    let resource = resources
        .first()
        .map(|r| r.name.clone())
        .unwrap_or_default();
    let kind = match kind {
        "Source" => Activity::Source {
            interarrival: Dist::Exp { mean: 10.0 },
            first_at_zero: false,
            limit: None,
        },
        "Schedule" => Activity::Schedule {
            times: vec![(0.0, 1)],
        },
        "OnEvent" => Activity::OnEvent {
            block: object,
            event: BlockEvent::Entry,
        },
        "Delay" => Activity::Delay {
            time: Dist::Const(1.0),
        },
        "Decide" => Activity::Decide {
            rule: DecideRule::Probability(Vec::new()),
        },
        "Assign" => Activity::Assign {
            labels: BTreeMap::from([("tipo".to_string(), "1".to_string())]),
        },
        "Acquire" => Activity::Acquire { resource, count: 1 },
        "Release" => Activity::Release { resource, count: 1 },
        "WaitFor" => Activity::WaitFor {
            block: object,
            event: BlockEvent::Entry,
        },
        "CreateItem" => Activity::CreateItem {
            into: object,
            port: 0,
            labels: BTreeMap::new(),
        },
        "Batch" => Activity::Batch { size: 2 },
        "Sink" => Activity::Sink,
        _ => return None,
    };
    Some(ActivityDef {
        name: name.to_string(),
        kind,
        next: Vec::new(),
    })
}

/// The starting point offered by "Novo fluxo": Source -> Delay -> Sink.
pub fn template_flow(name: &str) -> FlowDef {
    let activity = |kind, name: &str, next: Option<&str>| ActivityDef {
        name: name.to_string(),
        kind,
        next: next.map(str::to_string).into_iter().collect(),
    };
    FlowDef {
        name: name.to_string(),
        resources: Vec::new(),
        activities: vec![
            activity(
                Activity::Source {
                    interarrival: Dist::Exp { mean: 10.0 },
                    first_at_zero: false,
                    limit: None,
                },
                "gerar",
                Some("esperar"),
            ),
            activity(
                Activity::Delay {
                    time: Dist::Const(5.0),
                },
                "esperar",
                Some("fim"),
            ),
            activity(Activity::Sink, "fim", None),
        ],
    }
}

/// `base`, or `base_2`, `base_3`... whichever `taken` does not report.
pub fn unique(base: &str, taken: impl Fn(&str) -> bool) -> String {
    if !taken(base) {
        return base.to_string();
    }
    (2..)
        .map(|i| format!("{base}_{i}"))
        .find(|n| !taken(n))
        .expect("some suffix is free")
}

pub fn unique_activity_name(flow: &FlowDef, base: &str) -> String {
    unique(base, |n| flow.activities.iter().any(|a| a.name == n))
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// RON text of a whole flow, as `Command::Set` on `flow.<name>` expects it.
pub fn to_ron(flow: &FlowDef) -> String {
    ron::to_string(flow).expect("flow serializes")
}

/// Replaces (or adds) the whole flow.
pub fn set_flow(flow: &FlowDef) -> Command {
    Command::Set {
        key: format!("flow.{}", flow.name),
        value: to_ron(flow),
    }
}

pub fn remove_flow(name: &str) -> Command {
    Command::Set {
        key: format!("flow.{name}"),
        value: "None".into(),
    }
}

/// Renames a flow: sets the renamed copy and removes the old one, as one undo step.
pub fn rename_flow(flow: &FlowDef, to: &str) -> Command {
    let mut renamed = flow.clone();
    renamed.name = to.to_string();
    Command::Batch(vec![set_flow(&renamed), remove_flow(&flow.name)])
}

// ---------------------------------------------------------------------------
// Structural edits
// ---------------------------------------------------------------------------

/// Keeps the per-branch lists of a `Decide` (weights, conditions) as long as `next`.
pub fn sync_rule(a: &mut ActivityDef) {
    let n = a.next.len();
    if let Activity::Decide { rule } = &mut a.kind {
        match rule {
            DecideRule::Probability(w) => w.resize(n, 1.0),
            DecideRule::Condition(c) => c.resize(n, "true".to_string()),
            DecideRule::ByLabel(_) | DecideRule::Agent { .. } => {}
        }
    }
}

/// Removes branch `k` of `a.next`; a `Decide` also loses the weight or condition of
/// that branch.
pub fn remove_next(a: &mut ActivityDef, k: usize) {
    if k >= a.next.len() {
        return;
    }
    a.next.remove(k);
    if let Activity::Decide { rule } = &mut a.kind {
        match rule {
            DecideRule::Probability(w) if k < w.len() => {
                w.remove(k);
            }
            DecideRule::Condition(c) if k < c.len() => {
                c.remove(k);
            }
            _ => {}
        }
    }
}

/// Removes an activity and every reference to it. Returns false when the name does
/// not exist.
pub fn remove_activity(flow: &mut FlowDef, name: &str) -> bool {
    let before = flow.activities.len();
    flow.activities.retain(|a| a.name != name);
    if flow.activities.len() == before {
        return false;
    }
    for a in &mut flow.activities {
        while let Some(k) = a.next.iter().position(|n| n == name) {
            remove_next(a, k);
        }
    }
    true
}

/// Renames a resource, updating the `Acquire`/`Release` activities that use it.
pub fn rename_resource(flow: &mut FlowDef, from: &str, to: &str) {
    for r in &mut flow.resources {
        if r.name == from {
            r.name = to.to_string();
        }
    }
    for a in &mut flow.activities {
        if let Activity::Acquire { resource, .. } | Activity::Release { resource, .. } = &mut a.kind
            && resource == from
        {
            *resource = to.to_string();
        }
    }
}

/// Renames an activity, updating the `next` lists. Fails on an empty or taken name.
pub fn rename_activity(flow: &mut FlowDef, from: &str, to: &str) -> Result<(), String> {
    if to.trim().is_empty() {
        return Err(tx("flows-o_nome_nao_pode_ficar").into());
    }
    if from != to && flow.activities.iter().any(|a| a.name == to) {
        return Err(fx(
            "flows-ja_existe_uma_atividade_chamada",
            &[("to", vx(&to))],
        ));
    }
    for a in &mut flow.activities {
        if a.name == from {
            a.name = to.to_string();
        }
        for n in &mut a.next {
            if n == from {
                *n = to.to_string();
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Diagram layout
// ---------------------------------------------------------------------------

/// An arrow of the diagram, between activity indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    /// 1-based branch number, only for activities that choose among several `next`.
    pub branch: Option<usize>,
}

/// Arrows of the flow, in `next` order; targets that do not exist are skipped.
pub fn edges(flow: &FlowDef) -> Vec<Edge> {
    let mut out = Vec::new();
    for (from, a) in flow.activities.iter().enumerate() {
        let numbered = matches!(a.kind, Activity::Decide { .. }) && a.next.len() > 1;
        for (k, n) in a.next.iter().enumerate() {
            if let Some(to) = flow.activities.iter().position(|b| &b.name == n) {
                out.push(Edge {
                    from,
                    to,
                    branch: numbered.then_some(k + 1),
                });
            }
        }
    }
    out
}

/// Grid cell of an activity: `layer` is the column, `row` the line inside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub layer: usize,
    pub row: usize,
}

/// Layered layout: the layer is the longest path from the sources (edges that close a
/// cycle are ignored, so loops are fine); inside a layer the activities follow the
/// rows of their predecessors (and the branch order), then their definition order.
pub fn layout_cells(flow: &FlowDef) -> Vec<Cell> {
    let n = flow.activities.len();
    let arrows = edges(flow);
    let pairs: Vec<(usize, usize)> = arrows.iter().map(|e| (e.from, e.to)).collect();
    let layers = layout::layers(n, &pairs);
    let depth = layers.iter().copied().max().map_or(0, |m| m + 1);
    let mut row: Vec<Option<usize>> = vec![None; n];
    for layer in 0..depth {
        let mut members: Vec<(f64, usize)> = (0..n)
            .filter(|&v| layers[v] == layer)
            .map(|v| {
                let keys: Vec<f64> = arrows
                    .iter()
                    .filter(|e| e.to == v && layers[e.from] < layer)
                    .filter_map(|e| {
                        row[e.from].map(|r| r as f64 + 0.001 * e.branch.unwrap_or(0) as f64)
                    })
                    .collect();
                let key = if keys.is_empty() {
                    f64::MAX
                } else {
                    keys.iter().sum::<f64>() / keys.len() as f64
                };
                (key, v)
            })
            .collect();
        members.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        for (r, (_, v)) in members.into_iter().enumerate() {
            row[v] = Some(r);
        }
    }
    (0..n)
        .map(|v| Cell {
            layer: layers[v],
            row: row[v].unwrap_or(0),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utsim_model::Model;

    fn flow3() -> FlowDef {
        template_flow("f")
    }

    fn names(flow: &FlowDef) -> Vec<&str> {
        flow.activities.iter().map(|a| a.name.as_str()).collect()
    }

    #[test]
    fn template_is_a_valid_chain() {
        let f = flow3();
        assert!(f.validate(&Default::default()).is_empty());
        let cells = layout_cells(&f);
        assert_eq!(
            cells,
            vec![
                Cell { layer: 0, row: 0 },
                Cell { layer: 1, row: 0 },
                Cell { layer: 2, row: 0 }
            ]
        );
    }

    #[test]
    fn labels_are_portuguese_and_cover_every_kind() {
        let objects = vec!["maq".to_string()];
        let resources = vec![ResourceDef {
            name: "docas".into(),
            capacity: 2,
        }];
        for (key, label) in KINDS {
            let a = new_activity(key, "x", &objects, &resources).unwrap();
            assert_eq!(kind_key(&a.kind), key);
            assert_eq!(kind_label(&a.kind), tx(label));
        }
        assert!(new_activity("Nada", "x", &objects, &resources).is_none());
    }

    #[test]
    fn decide_branches_stack_in_branch_order_and_loops_do_not_hang() {
        let mut f = flow3();
        f.activities[1].kind = Activity::Decide {
            rule: DecideRule::Probability(vec![1.0, 1.0]),
        };
        f.activities[1].next = vec!["fim".into(), "volta".into()];
        // A rework loop: volta -> esperar.
        f.activities.push(ActivityDef {
            name: "volta".into(),
            kind: Activity::Delay {
                time: Dist::Const(1.0),
            },
            next: vec!["esperar".into()],
        });
        let cells = layout_cells(&f);
        let by = |n: &str| cells[f.activities.iter().position(|a| a.name == n).unwrap()];
        assert_eq!(by("gerar").layer, 0);
        assert_eq!(by("esperar").layer, 1);
        // Both branches sit after the decision; the back edge does not push layers.
        assert_eq!(by("fim").layer, 2);
        assert_eq!(by("volta").layer, 2);
        // Branch 1 (fim) above branch 2 (volta).
        assert!(by("fim").row < by("volta").row);
        let e = edges(&f);
        assert!(
            e.iter()
                .any(|e| e.branch == Some(2) && f.activities[e.to].name == "volta")
        );
    }

    #[test]
    fn disconnected_activities_get_their_own_rows() {
        let mut f = flow3();
        f.activities.push(ActivityDef {
            name: "solto".into(),
            kind: Activity::Sink,
            next: vec![],
        });
        let cells = layout_cells(&f);
        let rows: Vec<usize> = cells
            .iter()
            .filter(|c| c.layer == 0)
            .map(|c| c.row)
            .collect();
        assert_eq!(rows.len(), 2);
        assert_ne!(rows[0], rows[1]);
    }

    #[test]
    fn removing_an_activity_removes_references_and_branch_data() {
        let mut f = flow3();
        f.activities[0].kind = Activity::Decide {
            rule: DecideRule::Condition(vec!["true".into(), "false".into()]),
        };
        f.activities[0].next = vec!["esperar".into(), "fim".into()];
        assert!(remove_activity(&mut f, "esperar"));
        assert_eq!(names(&f), vec!["gerar", "fim"]);
        assert_eq!(f.activities[0].next, vec!["fim".to_string()]);
        match &f.activities[0].kind {
            Activity::Decide {
                rule: DecideRule::Condition(c),
            } => assert_eq!(c, &vec!["false".to_string()]),
            other => panic!("{other:?}"),
        }
        assert!(!remove_activity(&mut f, "esperar"));
    }

    #[test]
    fn renaming_updates_next_and_rejects_duplicates() {
        let mut f = flow3();
        rename_activity(&mut f, "esperar", "atraso").unwrap();
        assert_eq!(f.activities[0].next, vec!["atraso".to_string()]);
        assert!(rename_activity(&mut f, "atraso", "fim").is_err());
        assert!(rename_activity(&mut f, "atraso", " ").is_err());
    }

    #[test]
    fn sync_rule_follows_next() {
        let mut a = new_activity("Decide", "d", &[], &[]).unwrap();
        a.next = vec!["a".into(), "b".into(), "c".into()];
        sync_rule(&mut a);
        assert_eq!(
            a.kind,
            Activity::Decide {
                rule: DecideRule::Probability(vec![1.0; 3])
            }
        );
    }

    #[test]
    fn whole_flow_commands_roundtrip_through_the_document() {
        let mut doc = utsim_edit::Document::new("m");
        let f = flow3();
        doc.apply(set_flow(&f)).unwrap();
        assert_eq!(doc.model().flows, vec![f.clone()]);
        // Every kind survives the RON round trip.
        let mut all = FlowDef {
            name: "todos".into(),
            resources: vec![ResourceDef {
                name: "docas".into(),
                capacity: 3,
            }],
            activities: Vec::new(),
        };
        for (key, _) in KINDS {
            let mut a = new_activity(key, key, &["maq".to_string()], &all.resources).unwrap();
            a.next = vec!["Sink".into()];
            all.activities.push(a);
        }
        all.activities[0].kind = Activity::Source {
            interarrival: Dist::Exp { mean: 3.0 },
            first_at_zero: true,
            limit: Some(5),
        };
        doc.apply(set_flow(&all)).unwrap();
        assert_eq!(doc.model().flows[1], all);
        // Rename then remove, each one undo step.
        doc.apply(rename_flow(&all, "outro")).unwrap();
        assert_eq!(doc.model().flows.len(), 2);
        assert_eq!(doc.model().flows[1].name, "outro");
        doc.apply(remove_flow("outro")).unwrap();
        assert_eq!(doc.model().flows.len(), 1);
        doc.undo();
        assert_eq!(doc.model().flows.len(), 2);
        doc.undo();
        assert_eq!(doc.model().flows[1].name, "todos");
        // Saved text parses back.
        let text = doc.to_text();
        assert_eq!(Model::parse(&text).unwrap().flows.len(), 2);
    }

    #[test]
    fn unique_names() {
        let f = flow3();
        assert_eq!(unique_activity_name(&f, "novo"), "novo");
        assert_eq!(unique_activity_name(&f, "fim"), "fim_2");
    }
}
