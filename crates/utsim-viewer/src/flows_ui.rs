//! The "Fluxos" panel (phase 6): list of flows, 2D node diagram of the selected flow,
//! activity editor, resources. Pure egui (no Bevy): it reads the
//! [`Document`] and returns [`Command`]s, always a whole-flow `Set`, so undo/redo and
//! saving work like for any other edit. In `Simular` the same diagram shows live token
//! counts per activity.

use std::collections::HashSet;

use bevy_egui::egui::{
    self, Align2, Color32, ComboBox, CornerRadius, FontId, Id, Panel, Pos2, Rect, Sense, Shape,
    Stroke, StrokeKind, Ui, vec2,
};
use utsim_core::Dist;
use utsim_edit::{Command, Document};
use utsim_flow::{Activity, ActivityDef, BlockEvent, DecideRule, FlowDef, ResourceDef};
use utsim_script::compile_expr;

use crate::flows::{self, Cell, KINDS};
use crate::properties::panel::{
    RED, clear_temp, dist_info, dist_widget, labeled_float, num, set_temp, temp, text_commit,
};
use crate::timeline::Timeline;

/// What the panel remembers between frames.
#[derive(Default)]
pub struct FlowsState {
    pub open: bool,
    /// Selected flow.
    pub flow: Option<String>,
    /// Selected activity of that flow.
    pub activity: Option<String>,
    /// Index in [`KINDS`] of the kind "Adicionar" creates.
    pub add_kind: usize,
}

/// Token counts of a running replay, for the badges of the diagram.
pub struct Live<'a> {
    pub timeline: &'a Timeline,
    /// Replay block index of each flow, by name.
    pub flow_block: &'a dyn Fn(&str) -> Option<usize>,
    pub t: f64,
}

const NODE_W: f32 = 128.0;
const NODE_H: f32 = 44.0;
const GAP_X: f32 = 44.0;
const GAP_Y: f32 = 28.0;
const MARGIN: f32 = 16.0;
/// Extra room under the diagram for the arcs of edges that go backwards.
const LOOP_ROOM: f32 = 52.0;

/// Body of the Fluxos panel (the caller provides the docked frame). Returns the
/// commands to apply, in order.
pub fn panel_body(
    ui: &mut Ui,
    state: &mut FlowsState,
    doc: &Document,
    live: Option<Live>,
    editable: bool,
) -> Vec<Command> {
    let mut out = Vec::new();
    let model = doc.model();
    if state
        .flow
        .as_ref()
        .is_none_or(|n| !model.flows.iter().any(|f| &f.name == n))
    {
        state.flow = model.flows.first().map(|f| f.name.clone());
        state.activity = None;
    }
    let flow = state
        .flow
        .as_ref()
        .and_then(|n| model.flows.iter().find(|f| &f.name == n))
        .cloned();
    if let Some(f) = &flow
        && state
            .activity
            .as_ref()
            .is_some_and(|a| !f.activities.iter().any(|x| &x.name == a))
    {
        state.activity = None;
    }
    let objects: Vec<String> = model.objects.iter().map(|o| o.name.clone()).collect();
    let problems: Vec<String> = flow
        .as_ref()
        .map(|f| f.validate(&objects.iter().map(String::as_str).collect::<HashSet<_>>()))
        .unwrap_or_default();

    Panel::left("fluxos_lista")
        .default_size(165.0)
        .size_range(130.0..=320.0)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("fluxos_lista_rolagem")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    list_column(ui, state, doc, flow.as_ref(), &objects, editable, &mut out)
                });
        });
    Panel::right("fluxos_atividade")
        .default_size(300.0)
        .size_range(260.0..=520.0)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("fluxos_atividade_rolagem")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    activity_column(
                        ui,
                        state,
                        flow.as_ref(),
                        &objects,
                        live.as_ref(),
                        editable,
                        &mut out,
                    )
                });
        });
    egui::CentralPanel::default().show(ui, |ui| {
        center_column(
            ui,
            state,
            flow.as_ref(),
            &objects,
            &problems,
            live.as_ref(),
            editable,
            &mut out,
        )
    });
    out
}

// ---------------------------------------------------------------------------
// Left column: flows and resources
// ---------------------------------------------------------------------------

fn list_column(
    ui: &mut Ui,
    state: &mut FlowsState,
    doc: &Document,
    flow: Option<&FlowDef>,
    _objects: &[String],
    editable: bool,
    out: &mut Vec<Command>,
) {
    let model = doc.model();
    ui.heading("Fluxos");
    if model.flows.is_empty() {
        ui.weak("Nenhum fluxo neste modelo.");
    }
    for f in &model.flows {
        let on = state.flow.as_deref() == Some(f.name.as_str());
        if ui.selectable_label(on, &f.name).clicked() && !on {
            state.flow = Some(f.name.clone());
            state.activity = None;
        }
    }
    let taken = |n: &str| {
        model.flows.iter().any(|f| f.name == n) || model.objects.iter().any(|o| o.name == n)
    };
    ui.horizontal(|ui| {
        if ui
            .add_enabled(editable, egui::Button::new("Novo fluxo"))
            .on_hover_text("Cria um fluxo Gerar tokens, Esperar, Fim")
            .clicked()
        {
            let name = flows::unique("fluxo", taken);
            out.push(flows::set_flow(&flows::template_flow(&name)));
            state.flow = Some(name);
            state.activity = None;
        }
        if ui
            .add_enabled(editable && flow.is_some(), egui::Button::new("Remover"))
            .on_hover_text("Remove o fluxo selecionado (Ctrl+Z desfaz)")
            .clicked()
            && let Some(f) = flow
        {
            out.push(flows::remove_flow(&f.name));
        }
    });
    let Some(f) = flow else { return };
    ui.separator();
    ui.add_enabled_ui(editable, |ui| {
        ui.horizontal(|ui| {
            ui.label("Nome");
            let check = |t: &str| {
                if t.trim().is_empty() {
                    Err("o nome não pode ficar vazio".to_string())
                } else if t != f.name && taken(t) {
                    Err(format!("já existe um objeto ou fluxo chamado \"{t}\""))
                } else {
                    Ok(())
                }
            };
            let id = Id::new(("fluxo-nome", &f.name));
            if let Some(to) = text_commit(ui, id, &f.name, false, &check) {
                out.push(flows::rename_flow(f, &to));
                state.flow = Some(to);
            }
        });
        ui.separator();
        resources_editor(ui, f, out);
    });
}

fn resources_editor(ui: &mut Ui, f: &FlowDef, out: &mut Vec<Command>) {
    ui.strong("Recursos");
    ui.weak("Contadores compartilhados (ex.: docas).");
    let mut new = f.clone();
    let mut changed = false;
    let mut remove = None;
    for (i, r) in f.resources.iter().enumerate() {
        ui.horizontal(|ui| {
            let id = Id::new(("fluxo-recurso", &f.name, i));
            let taken = |t: &str| t != r.name && f.resources.iter().any(|x| x.name == t);
            let (commit, _) = line(ui, id.with("nome"), &r.name, 80.0);
            if let Some(to) = commit
                && !to.trim().is_empty()
                && !taken(&to)
            {
                flows::rename_resource(&mut new, &r.name, &to);
                changed = true;
            }
            ui.label("cap.");
            let mut cap = r.capacity;
            if num(ui, id.with("cap"), &mut cap, 1.0, 1usize) {
                new.resources[i].capacity = cap;
                changed = true;
            }
            if ui
                .small_button("x")
                .on_hover_text("Remover recurso")
                .clicked()
            {
                remove = Some(i);
            }
        });
    }
    if let Some(i) = remove {
        new.resources.remove(i);
        changed = true;
    }
    if ui.small_button("+ recurso").clicked() {
        let name = flows::unique("recurso", |n| f.resources.iter().any(|r| r.name == n));
        new.resources.push(ResourceDef { name, capacity: 1 });
        changed = true;
    }
    if changed {
        out.push(flows::set_flow(&new));
    }
}

// ---------------------------------------------------------------------------
// Center: toolbar, problems and diagram
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)] // UI plumbing: each argument is one piece of panel state
fn center_column(
    ui: &mut Ui,
    state: &mut FlowsState,
    flow: Option<&FlowDef>,
    objects: &[String],
    problems: &[String],
    live: Option<&Live>,
    editable: bool,
    out: &mut Vec<Command>,
) {
    let Some(f) = flow else {
        ui.add_space(8.0);
        ui.label("Crie um fluxo com \"Novo fluxo\" para desenhar a lógica do modelo.");
        return;
    };
    ui.horizontal_wrapped(|ui| {
        ui.strong(&f.name);
        ui.separator();
        ui.add_enabled_ui(editable, |ui| {
            ui.label("Adicionar:");
            ComboBox::from_id_salt("fluxos_tipo")
                .selected_text(KINDS[state.add_kind].1)
                .show_ui(ui, |ui| {
                    for (i, (_, label)) in KINDS.iter().enumerate() {
                        ui.selectable_value(&mut state.add_kind, i, *label);
                    }
                });
            if ui.button("Adicionar atividade").clicked() {
                let (key, label) = KINDS[state.add_kind];
                let base = label.to_lowercase().replace(' ', "_");
                let name = flows::unique_activity_name(f, &base);
                if let Some(act) = flows::new_activity(key, &name, objects, &f.resources) {
                    let mut new = f.clone();
                    // Chains the new activity after the selected one when that one is open.
                    if let Some(sel) = state.activity.clone()
                        && let Some(s) = new.activities.iter_mut().find(|a| a.name == sel)
                        && s.next.is_empty()
                        && !matches!(s.kind, Activity::Sink)
                    {
                        s.next.push(name.clone());
                        flows::sync_rule(s);
                    }
                    new.activities.push(act);
                    out.push(flows::set_flow(&new));
                    state.activity = Some(name);
                }
            }
            if ui
                .add_enabled(
                    state.activity.is_some(),
                    egui::Button::new("Excluir atividade"),
                )
                .on_hover_text("Remove a atividade e as ligações que chegam nela")
                .clicked()
                && let Some(sel) = state.activity.take()
            {
                let mut new = f.clone();
                flows::remove_activity(&mut new, &sel);
                out.push(flows::set_flow(&new));
            }
        });
    });
    for p in problems.iter().take(3) {
        ui.colored_label(RED, format!("- {p}"));
    }
    if problems.len() > 3 {
        ui.colored_label(
            RED,
            format!("... e mais {} problema(s)", problems.len() - 3),
        );
    }
    if let Some(l) = live {
        ui.weak(format!(
            "Simulação em t = {:.2}: o número em cada caixa é a quantidade de tokens ali.",
            l.t
        ));
    }
    diagram(ui, state, f, problems, live);
}

fn kind_color(kind: &Activity) -> Color32 {
    match kind {
        Activity::Source { .. } | Activity::Schedule { .. } | Activity::OnEvent { .. } => {
            Color32::from_rgb(38, 96, 66)
        }
        Activity::Delay { .. }
        | Activity::Decide { .. }
        | Activity::Assign { .. }
        | Activity::Batch { .. } => Color32::from_rgb(40, 72, 120),
        Activity::Acquire { .. } | Activity::Release { .. } => Color32::from_rgb(120, 78, 32),
        Activity::WaitFor { .. } | Activity::CreateItem { .. } => Color32::from_rgb(88, 56, 118),
        Activity::Sink => Color32::from_rgb(78, 78, 88),
    }
}

/// Box of a cell; `z` shrinks the whole diagram to fit narrow panels.
fn node_rect(origin: Pos2, c: Cell, z: f32) -> Rect {
    Rect::from_min_size(
        origin
            + vec2(
                MARGIN + c.layer as f32 * (NODE_W + GAP_X),
                MARGIN + c.row as f32 * (NODE_H + GAP_Y),
            ) * z,
        vec2(NODE_W, NODE_H) * z,
    )
}

/// Point of a cubic Bézier curve.
fn bezier(p: [Pos2; 4], t: f32) -> Pos2 {
    let u = 1.0 - t;
    let w = [u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t];
    Pos2::new(
        p[0].x * w[0] + p[1].x * w[1] + p[2].x * w[2] + p[3].x * w[3],
        p[0].y * w[0] + p[1].y * w[1] + p[2].y * w[2] + p[3].y * w[3],
    )
}

/// Control points of the arrow between two boxes: from the right side to the left side
/// when it goes forward, otherwise an arc under the boxes.
fn edge_points(from: Rect, to: Rect, forward: bool, self_loop: bool, z: f32) -> [Pos2; 4] {
    if self_loop {
        let p0 = from.right_center();
        let p3 = Pos2::new(from.right() - 28.0 * z, from.bottom());
        return [p0, p0 + vec2(46.0 * z, 0.0), p3 + vec2(0.0, 46.0 * z), p3];
    }
    if forward {
        let p0 = from.right_center();
        let p3 = to.left_center();
        let dx = ((p3.x - p0.x) * 0.5).max(24.0 * z);
        [p0, p0 + vec2(dx, 0.0), p3 - vec2(dx, 0.0), p3]
    } else {
        let p0 = from.center_bottom();
        let p3 = to.center_bottom();
        let drop = 44.0 * z;
        [p0, p0 + vec2(0.0, drop), p3 + vec2(0.0, drop), p3]
    }
}

fn diagram(
    ui: &mut Ui,
    state: &mut FlowsState,
    f: &FlowDef,
    problems: &[String],
    live: Option<&Live>,
) {
    if f.activities.is_empty() {
        ui.weak("Fluxo vazio: adicione uma atividade.");
        return;
    }
    let cells = flows::layout_cells(f);
    let layers = cells.iter().map(|c| c.layer).max().map_or(1, |m| m + 1);
    let rows = cells.iter().map(|c| c.row).max().map_or(1, |m| m + 1);
    let arrows = flows::edges(f);
    let has_back = arrows
        .iter()
        .any(|e| cells[e.to].layer <= cells[e.from].layer);
    let natural = vec2(
        MARGIN * 2.0 + layers as f32 * (NODE_W + GAP_X) - GAP_X,
        MARGIN * 2.0 + rows as f32 * (NODE_H + GAP_Y) - GAP_Y
            + if has_back { LOOP_ROOM } else { 0.0 },
    );
    // Wide flows shrink (down to 60%) to fit the panel before they start to scroll.
    let z = ((ui.available_width() - 14.0) / natural.x).clamp(0.6, 1.0);
    let size = natural * z;
    let flow_block = live.and_then(|l| (l.flow_block)(&f.name));
    egui::ScrollArea::both()
        .id_salt(("fluxos_diagrama", &f.name))
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let (area, background) = ui.allocate_exact_size(size, Sense::click());
            if background.clicked() {
                state.activity = None;
            }
            let painter = ui.painter_at(area);
            let rects: Vec<Rect> = cells.iter().map(|&c| node_rect(area.min, c, z)).collect();
            let selected = state
                .activity
                .as_ref()
                .and_then(|s| f.activities.iter().position(|a| &a.name == s));

            let calm = Color32::from_rgb(140, 150, 175);
            let bright = Color32::from_rgb(235, 238, 250);
            for e in &arrows {
                let forward = cells[e.to].layer > cells[e.from].layer;
                let points = edge_points(rects[e.from], rects[e.to], forward, e.from == e.to, z);
                let color = if selected == Some(e.from) || selected == Some(e.to) {
                    bright
                } else {
                    calm
                };
                painter.add(Shape::CubicBezier(
                    egui::epaint::CubicBezierShape::from_points_stroke(
                        points,
                        false,
                        Color32::TRANSPARENT,
                        Stroke::new(1.6, color),
                    ),
                ));
                let tip = points[3];
                let dir = (tip - points[2]).normalized();
                let dir = if dir.length() < 0.5 {
                    vec2(1.0, 0.0)
                } else {
                    dir
                };
                let side = vec2(-dir.y, dir.x);
                painter.add(Shape::convex_polygon(
                    vec![
                        tip,
                        tip - dir * 10.0 * z + side * 5.0 * z,
                        tip - dir * 10.0 * z - side * 5.0 * z,
                    ],
                    color,
                    Stroke::NONE,
                ));
                if let Some(n) = e.branch {
                    let at = bezier(points, 0.5);
                    painter.circle(
                        at,
                        8.0,
                        Color32::from_rgb(28, 30, 38),
                        Stroke::new(1.0, color),
                    );
                    painter.text(
                        at,
                        Align2::CENTER_CENTER,
                        n.to_string(),
                        FontId::proportional(11.0 * z),
                        Color32::WHITE,
                    );
                }
            }

            let mut clicked = None;
            for (i, a) in f.activities.iter().enumerate() {
                let r = rects[i];
                let count = flow_block
                    .zip(live)
                    .map(|(fb, l)| l.timeline.tokens_at(fb, i, l.t));
                let busy = count.is_some_and(|c| c > 0);
                let broken = problems
                    .iter()
                    .any(|p| p.starts_with(&format!("{}.{}:", f.name, a.name)));
                let mut fill = kind_color(&a.kind);
                if busy {
                    fill = fill.lerp_to_gamma(Color32::from_rgb(255, 170, 40), 0.35);
                }
                painter.rect_filled(r, CornerRadius::same((9.0 * z) as u8), fill);
                let (width, color) = if selected == Some(i) {
                    (2.5, Color32::WHITE)
                } else if broken {
                    (2.0, RED)
                } else if busy {
                    (2.5, Color32::from_rgb(255, 170, 40))
                } else {
                    (1.0, Color32::from_gray(25))
                };
                painter.rect_stroke(
                    r,
                    CornerRadius::same((9.0 * z) as u8),
                    Stroke::new(width, color),
                    StrokeKind::Inside,
                );
                let text = painter.with_clip_rect(r.shrink(3.0));
                text.text(
                    r.center_top() + vec2(0.0, 12.0 * z),
                    Align2::CENTER_CENTER,
                    flows::kind_label(&a.kind),
                    FontId::proportional(11.0 * z),
                    Color32::from_gray(200),
                );
                text.text(
                    r.center_bottom() - vec2(0.0, 14.0 * z),
                    Align2::CENTER_CENTER,
                    &a.name,
                    FontId::proportional(14.0 * z),
                    Color32::WHITE,
                );
                if let Some(c) = count {
                    let at = r.right_top() + vec2(-2.0, 2.0);
                    let (bg, fg) = if c > 0 {
                        (Color32::from_rgb(255, 170, 40), Color32::BLACK)
                    } else {
                        (Color32::from_gray(70), Color32::from_gray(200))
                    };
                    painter.circle_filled(at, 11.0 * z, bg);
                    painter.text(
                        at,
                        Align2::CENTER_CENTER,
                        c.to_string(),
                        FontId::proportional(12.0 * z),
                        fg,
                    );
                }
                let response =
                    ui.interact(r, Id::new(("fluxo-no", &f.name, &a.name)), Sense::click());
                if response.clicked() {
                    clicked = Some(a.name.clone());
                }
            }
            if clicked.is_some() {
                state.activity = clicked;
            }
        });
}

// ---------------------------------------------------------------------------
// Right column: activity editor
// ---------------------------------------------------------------------------

fn activity_column(
    ui: &mut Ui,
    state: &mut FlowsState,
    flow: Option<&FlowDef>,
    objects: &[String],
    live: Option<&Live>,
    editable: bool,
    out: &mut Vec<Command>,
) {
    ui.heading("Atividade");
    let (Some(f), Some(name)) = (flow, state.activity.clone()) else {
        ui.weak("Clique numa atividade do diagrama para ver e editar seus campos.");
        return;
    };
    let Some(idx) = f.activities.iter().position(|a| a.name == name) else {
        return;
    };
    if let Some(l) = live
        && let Some(fb) = (l.flow_block)(&f.name)
    {
        ui.label(format!(
            "Tokens agora: {}",
            l.timeline.tokens_at(fb, idx, l.t)
        ));
    }
    if !editable {
        ui.weak("Volte para Editar para alterar o fluxo.");
    }
    ui.add_enabled_ui(editable, |ui| {
        if let Some(edit) = edit_activity(ui, f, idx, objects) {
            out.push(flows::set_flow(&edit.flow));
            if let Some(sel) = edit.select {
                state.activity = Some(sel);
            }
        }
    });
}

/// A changed flow, and the new name of the activity when it was renamed.
pub struct Edit {
    pub flow: FlowDef,
    pub select: Option<String>,
}

/// A single-line box committed when it loses focus (Enter or click elsewhere), even
/// when its content is not valid yet. Returns the committed text and the text now shown.
fn line(ui: &mut Ui, id: Id, current: &str, width: f32) -> (Option<String>, String) {
    let mut text = temp::<String>(ui, id).unwrap_or_else(|| current.to_string());
    let response = ui.add(egui::TextEdit::singleline(&mut text).desired_width(width));
    if response.changed() {
        set_temp(ui, id, text.clone());
    }
    let mut commit = None;
    if response.lost_focus() && temp::<String>(ui, id).is_some() {
        clear_temp::<String>(ui, id);
        if text != current {
            commit = Some(text.clone());
        }
    }
    (commit, text)
}

/// An expression box with its compile error shown right under it.
fn expr_box(ui: &mut Ui, id: Id, current: &str, width: f32) -> Option<String> {
    let (commit, shown) = line(ui, id, current, width);
    if let Err(e) = compile_expr(&shown) {
        ui.colored_label(RED, e);
    }
    commit
}

fn object_combo(ui: &mut Ui, id: Id, value: &mut String, objects: &[String]) -> bool {
    let mut changed = false;
    let text = if value.is_empty() {
        "(escolha)".to_string()
    } else {
        value.clone()
    };
    ComboBox::from_id_salt(id)
        .selected_text(text)
        .show_ui(ui, |ui| {
            for o in objects {
                if ui.selectable_label(value == o, o).clicked() {
                    *value = o.clone();
                    changed = true;
                }
            }
        });
    if !value.is_empty() && !objects.contains(value) {
        ui.colored_label(RED, format!("objeto \"{value}\" não existe"));
    }
    changed
}

fn event_combo(ui: &mut Ui, id: Id, event: &mut BlockEvent) -> bool {
    let mut changed = false;
    ComboBox::from_id_salt(id)
        .selected_text(flows::event_label(*event))
        .show_ui(ui, |ui| {
            for e in [BlockEvent::Entry, BlockEvent::Exit] {
                if ui
                    .selectable_label(*event == e, flows::event_label(e))
                    .clicked()
                {
                    *event = e;
                    changed = true;
                }
            }
        });
    changed
}

fn dist_editor(ui: &mut Ui, id: Id, d: &mut Dist) -> bool {
    let mut copy = d.clone();
    let changed = dist_widget(ui, id, &mut copy);
    if changed {
        *d = copy;
    }
    dist_info(ui, d);
    changed
}

/// `name = expression` rows for `Assign` and `CreateItem`.
fn labels_editor(
    ui: &mut Ui,
    id: Id,
    labels: &mut std::collections::BTreeMap<String, String>,
) -> bool {
    let mut changed = false;
    let mut remove = None;
    let mut rename: Option<(String, String)> = None;
    let mut set: Option<(String, String)> = None;
    for (k, v) in labels.iter() {
        let mut shown = String::new();
        ui.horizontal(|ui| {
            let (c, _) = line(ui, id.with(("k", k)), k, 80.0);
            if let Some(new_key) = c
                && !new_key.trim().is_empty()
                && !labels.contains_key(&new_key)
            {
                rename = Some((k.clone(), new_key));
            }
            ui.label("=");
            let (c, text) = line(ui, id.with(("v", k)), v, 120.0);
            shown = text;
            if let Some(new_value) = c {
                set = Some((k.clone(), new_value));
            }
            if ui
                .small_button("x")
                .on_hover_text("Remover rótulo")
                .clicked()
            {
                remove = Some(k.clone());
            }
        });
        if let Err(e) = compile_expr(&shown) {
            ui.colored_label(RED, e);
        }
    }
    if let Some((from, to)) = rename
        && let Some(v) = labels.remove(&from)
    {
        labels.insert(to, v);
        changed = true;
    }
    if let Some((k, v)) = set {
        labels.insert(k, v);
        changed = true;
    }
    if let Some(k) = remove {
        labels.remove(&k);
        changed = true;
    }
    if ui.small_button("+ rótulo").clicked() {
        let key = flows::unique("rotulo", |n| labels.contains_key(n));
        labels.insert(key, "1".to_string());
        changed = true;
    }
    changed
}

/// The `next` connections (and, for `Decide`, the weight or condition of each branch).
#[allow(clippy::needless_range_loop)] // `k` also indexes the per-branch rule lists
fn next_editor(ui: &mut Ui, id: Id, a: &mut ActivityDef, names: &[String]) -> bool {
    let mut changed = false;
    if matches!(a.kind, Activity::Sink) {
        ui.strong("Próxima atividade");
        ui.weak("Fim do fluxo: o token termina aqui.");
        return false;
    }
    let decide = matches!(a.kind, Activity::Decide { .. });
    ui.strong(if decide {
        "Ramos (para onde o token vai)"
    } else {
        "Próxima atividade"
    });
    let mut remove = None;
    let ActivityDef { kind, next, .. } = a;
    for k in 0..next.len() {
        ui.horizontal(|ui| {
            if decide {
                ui.label(format!("{}.", k + 1));
            }
            ComboBox::from_id_salt(id.with(("n", k)))
                .selected_text(next[k].clone())
                .show_ui(ui, |ui| {
                    for n in names {
                        if ui.selectable_label(next[k] == *n, n).clicked() {
                            next[k] = n.clone();
                            changed = true;
                        }
                    }
                });
            if ui
                .small_button("x")
                .on_hover_text("Remover ligação")
                .clicked()
            {
                remove = Some(k);
            }
        });
        if let Activity::Decide { rule } = kind {
            match rule {
                DecideRule::Probability(w) => {
                    if let Some(weight) = w.get_mut(k) {
                        ui.horizontal(|ui| {
                            changed |= labeled_float(ui, id.with(("w", k)), "     peso", weight);
                        });
                    }
                }
                DecideRule::Condition(c) => {
                    if let Some(cond) = c.get_mut(k) {
                        ui.horizontal(|ui| {
                            ui.label("     se");
                            if let Some(t) = expr_box(ui, id.with(("c", k)), cond, 150.0) {
                                *cond = t;
                                changed = true;
                            }
                        });
                    }
                }
                DecideRule::ByLabel(_) | DecideRule::Agent { .. } => {}
            }
        }
    }
    if let Some(k) = remove {
        flows::remove_next(a, k);
        changed = true;
    }
    if decide || a.next.is_empty() {
        let mut pick = None;
        ComboBox::from_id_salt(id.with("ligar"))
            .selected_text(if decide {
                "+ ligar a outro ramo..."
            } else {
                "+ ligar a..."
            })
            .show_ui(ui, |ui| {
                for n in names.iter().filter(|n| !a.next.contains(n)) {
                    if ui.selectable_label(false, n).clicked() {
                        pick = Some(n.clone());
                    }
                }
            });
        if let Some(n) = pick {
            a.next.push(n);
            changed = true;
        }
    }
    if changed {
        flows::sync_rule(a);
    }
    changed
}

fn edit_activity(ui: &mut Ui, flow: &FlowDef, idx: usize, objects: &[String]) -> Option<Edit> {
    let mut new = flow.clone();
    let mut select = None;
    let mut changed = false;
    let act = flow.activities[idx].clone();
    let id = Id::new(("fluxo-ed", &flow.name, &act.name));

    ui.horizontal(|ui| {
        ui.label("Nome");
        let check = |t: &str| {
            if t.trim().is_empty() {
                Err("o nome não pode ficar vazio".to_string())
            } else if t != act.name && flow.activities.iter().any(|a| a.name == t) {
                Err(format!("já existe uma atividade chamada \"{t}\""))
            } else {
                Ok(())
            }
        };
        if let Some(to) = text_commit(ui, id.with("nome"), &act.name, false, &check)
            && flows::rename_activity(&mut new, &act.name, &to).is_ok()
        {
            select = Some(to);
            changed = true;
        }
    });
    ui.weak(flows::kind_label(&act.kind));
    ui.separator();

    let names: Vec<String> = flow.activities.iter().map(|a| a.name.clone()).collect();
    let resources: Vec<String> = flow.resources.iter().map(|r| r.name.clone()).collect();
    let a = &mut new.activities[idx];
    match &mut a.kind {
        Activity::Source {
            interarrival,
            first_at_zero,
            limit,
        } => {
            ui.strong("Intervalo entre tokens");
            changed |= dist_editor(ui, id.with("ia"), interarrival);
            changed |= ui
                .checkbox(first_at_zero, "criar um token em t = 0")
                .changed();
            let mut on = limit.is_some();
            ui.horizontal(|ui| {
                if ui.checkbox(&mut on, "limitar a").changed() {
                    *limit = on.then_some(100);
                    changed = true;
                }
                if let Some(n) = limit {
                    changed |= num(ui, id.with("limite"), n, 1.0, 1u64);
                    ui.label("tokens");
                }
            });
        }
        Activity::Schedule { times } => {
            ui.strong("Horários");
            let mut remove = None;
            for (i, (t, count)) in times.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    changed |= labeled_float(ui, id.with(("t", i)), "t =", t);
                    ui.label("tokens");
                    changed |= num(ui, id.with(("n", i)), count, 1.0, 1u32);
                    if ui
                        .small_button("x")
                        .on_hover_text("Remover horário")
                        .clicked()
                    {
                        remove = Some(i);
                    }
                });
            }
            if let Some(i) = remove {
                times.remove(i);
                changed = true;
            }
            if ui.small_button("+ horário").clicked() {
                let at = times.last().map_or(0.0, |(t, _)| t + 10.0);
                times.push((at, 1));
                changed = true;
            }
        }
        Activity::OnEvent { block, event } | Activity::WaitFor { block, event } => {
            ui.horizontal(|ui| {
                ui.label("Objeto");
                changed |= object_combo(ui, id.with("obj"), block, objects);
            });
            ui.horizontal(|ui| {
                ui.label("Evento");
                changed |= event_combo(ui, id.with("ev"), event);
            });
            ui.weak("O token passa a carregar o item desse evento.");
        }
        Activity::Delay { time } => {
            ui.strong("Tempo de espera");
            changed |= dist_editor(ui, id.with("t"), time);
        }
        Activity::Decide { rule } => {
            const RULES: [&str; 4] = [
                "Probabilidade",
                "Pelo rótulo",
                "Condições",
                "Agente externo",
            ];
            let index = match rule {
                DecideRule::Probability(_) => 0,
                DecideRule::ByLabel(_) => 1,
                DecideRule::Condition(_) => 2,
                DecideRule::Agent { .. } => 3,
            };
            let mut picked = index;
            ui.horizontal(|ui| {
                ui.label("Regra");
                ComboBox::from_id_salt(id.with("regra"))
                    .selected_text(RULES[index])
                    .show_ui(ui, |ui| {
                        for (i, name) in RULES.iter().enumerate() {
                            ui.selectable_value(&mut picked, i, *name);
                        }
                    });
            });
            let n = a.next.len();
            if picked != index {
                *rule = match picked {
                    0 => DecideRule::Probability(vec![1.0; n]),
                    1 => DecideRule::ByLabel("tipo".to_string()),
                    2 => DecideRule::Condition(vec!["true".to_string(); n]),
                    _ => DecideRule::Agent {
                        observation: vec!["now".to_string()],
                    },
                };
                changed = true;
            }
            match rule {
                DecideRule::ByLabel(label) => {
                    ui.horizontal(|ui| {
                        ui.label("rótulo do token (1, 2, ...)");
                        let (c, _) = line(ui, id.with("rotulo"), label, 90.0);
                        if let Some(t) = c
                            && !t.trim().is_empty()
                        {
                            *label = t;
                            changed = true;
                        }
                    });
                }
                DecideRule::Condition(_) => {
                    ui.weak("A primeira condição verdadeira vence; nenhuma = último ramo.");
                }
                DecideRule::Probability(_) => {
                    ui.weak("Pesos relativos, um por ramo.");
                }
                DecideRule::Agent { observation } => {
                    ui.weak("Um agente externo escolhe o ramo; a simulação pausa e mostra estas observações:");
                    for (i, obs) in observation.iter_mut().enumerate() {
                        if let Some(t) = expr_box(ui, id.with(("obs", i)), obs, 180.0) {
                            *obs = t;
                            changed = true;
                        }
                    }
                    if ui.small_button("+ observação").clicked() {
                        observation.push("now".to_string());
                        changed = true;
                    }
                }
            }
        }
        Activity::Assign { labels } => {
            ui.strong("Rótulos");
            ui.weak("nome atribui ao token; item.nome ao item carregado.");
            changed |= labels_editor(ui, id.with("rot"), labels);
        }
        Activity::Acquire { resource, count } | Activity::Release { resource, count } => {
            ui.horizontal(|ui| {
                ui.label("Recurso");
                let text = if resource.is_empty() {
                    "(escolha)".to_string()
                } else {
                    resource.clone()
                };
                ComboBox::from_id_salt(id.with("rec"))
                    .selected_text(text)
                    .show_ui(ui, |ui| {
                        for r in &resources {
                            if ui.selectable_label(resource == r, r).clicked() {
                                *resource = r.clone();
                                changed = true;
                            }
                        }
                    });
            });
            if !resource.is_empty() && !resources.contains(resource) {
                ui.colored_label(RED, format!("recurso \"{resource}\" não existe"));
            }
            if resources.is_empty() {
                ui.weak("Crie um recurso na coluna da esquerda.");
            }
            ui.horizontal(|ui| {
                ui.label("quantidade");
                changed |= num(ui, id.with("qtd"), count, 1.0, 1usize);
            });
        }
        Activity::CreateItem { into, port, labels } => {
            ui.horizontal(|ui| {
                ui.label("Dentro do objeto");
                changed |= object_combo(ui, id.with("obj"), into, objects);
            });
            ui.horizontal(|ui| {
                ui.label("porta de entrada");
                changed |= num(ui, id.with("porta"), port, 1.0, 0usize);
            });
            ui.strong("Rótulos do item");
            changed |= labels_editor(ui, id.with("rot"), labels);
        }
        Activity::Batch { size } => {
            ui.horizontal(|ui| {
                ui.label("Juntar");
                changed |= num(ui, id.with("lote"), size, 1.0, 1usize);
                ui.label("tokens");
            });
        }
        Activity::Sink => {}
    }
    ui.separator();
    changed |= next_editor(ui, id.with("next"), a, &names);
    changed.then_some(Edit { flow: new, select })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flows::new_activity;

    fn doc_with_every_kind() -> Document {
        let mut doc = Document::new("m");
        let mut flow = FlowDef {
            name: "todos".into(),
            resources: vec![ResourceDef {
                name: "docas".into(),
                capacity: 2,
            }],
            activities: Vec::new(),
        };
        for (key, _) in KINDS {
            let mut a = new_activity(key, key, &[], &flow.resources).unwrap();
            if key != "Sink" {
                a.next = vec!["Sink".into()];
                flows::sync_rule(&mut a);
            }
            flow.activities.push(a);
        }
        // A loop must still draw.
        flow.activities[3].next = vec!["Source".into()];
        doc.apply(flows::set_flow(&flow)).unwrap();
        doc
    }

    fn render(doc: &Document, state: &mut FlowsState, editable: bool) -> Vec<Command> {
        let ctx = egui::Context::default();
        let mut commands = Vec::new();
        for _ in 0..2 {
            ctx.run_ui(egui::RawInput::default(), |ui| {
                commands = panel_body(ui, state, doc, None, editable);
            })
            .drop_without_applying_deltas();
        }
        commands
    }

    #[test]
    fn drawing_the_panel_for_every_activity_emits_nothing() {
        let doc = doc_with_every_kind();
        let mut state = FlowsState::default();
        for (key, _) in KINDS {
            state.activity = Some(key.to_string());
            assert_eq!(render(&doc, &mut state, true), Vec::new(), "{key}");
            assert_eq!(
                render(&doc, &mut state, false),
                Vec::new(),
                "{key} (leitura)"
            );
        }
        assert_eq!(state.flow.as_deref(), Some("todos"));
    }

    #[test]
    fn the_diagram_draws_with_live_token_counts() {
        use utsim_core::{TraceEvent, TraceRecord};
        use utsim_model::{Model, replay::ReplayBlock};

        let doc = doc_with_every_kind();
        let mut replay = Model::parse(
            "Model(name: \"r\", run: (stop_time: 5.0), objects: [(name: \"s\", kind: Sink)], connections: [])",
        )
        .unwrap()
        .replay(0)
        .unwrap();
        replay.blocks.push(ReplayBlock {
            name: "todos".into(),
            kind: "Flow".into(),
            position: [0.0; 3],
        });
        for (t, count) in [(1.0, 2), (3.0, 0)] {
            replay.records.push(TraceRecord {
                t,
                event: TraceEvent::Tokens {
                    flow: 1,
                    activity: 3,
                    count,
                },
            });
        }
        let timeline = Timeline::build(&replay, crate::timeline::TransitSetting::Auto);
        assert_eq!(timeline.tokens_at(1, 3, 2.0), 2);
        let lookup = |name: &str| (name == "todos").then_some(1);
        let ctx = egui::Context::default();
        let mut state = FlowsState {
            activity: Some("Delay".into()),
            ..Default::default()
        };
        for t in [0.5, 2.0, 4.0] {
            let mut commands = Vec::new();
            ctx.run_ui(egui::RawInput::default(), |ui| {
                let live = Live {
                    timeline: &timeline,
                    flow_block: &lookup,
                    t,
                };
                commands = panel_body(ui, &mut state, &doc, Some(live), false);
            })
            .drop_without_applying_deltas();
            assert_eq!(commands, Vec::new());
        }
    }

    #[test]
    fn an_empty_model_draws_the_empty_panel() {
        let doc = Document::new("vazio");
        let mut state = FlowsState::default();
        assert_eq!(render(&doc, &mut state, true), Vec::new());
        assert_eq!(state.flow, None);
    }

    #[test]
    fn a_stale_selection_is_dropped() {
        let doc = doc_with_every_kind();
        let mut state = FlowsState {
            flow: Some("some".into()),
            activity: Some("x".into()),
            ..Default::default()
        };
        render(&doc, &mut state, true);
        assert_eq!(state.flow.as_deref(), Some("todos"));
        assert_eq!(state.activity, None);
    }
}
