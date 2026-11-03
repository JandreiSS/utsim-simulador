//! The token flows of the model in 2D, in egui (pure: no Godot types, unit tested). It
//! replaces `fluxos.gd`.
//!
//! Each flow (`Model.flows`) is a graph of its activities (layered by the bridge along
//! `next`), with the tokens in each activity at the playback time written on its box, and
//! below the counts of every activity over the whole run, with a line at the current
//! time. The boxes are as wide as the longest text (nothing is cut) and the graph keeps
//! that size: it scrolls, and zooms with the buttons or Ctrl + wheel; a new graph starts
//! fitted to the width, never below [`MIN_FIT`].
//!
//! Names: `PainelFluxos`, `Fluxos_Escolha`, `Fluxos_Fechar`, `Fluxos_ZoomMenos`,
//! `Fluxos_ZoomMais`, `Fluxos_Ajustar`, `Fluxos_Rolagem`, `Fluxos_Grafo` (its text has
//! `atividades=`, `pico=`, `sobreposicoes=`, `cortadas=` and `zoom=` for the checks) and
//! `Fluxos_Grafico`.

use egui::{
    Align, Align2, Color32, FontId, Layout, Pos2, Rect, RichText, ScrollArea, Sense, Shape, Stroke,
    StrokeKind, Ui, UiBuilder, Vec2, pos2, vec2,
};
use utsim_ui_tema::{colors, sizes};

use crate::egui_widgets::{Registry, WidgetInfo};
use crate::extras::FlowGraph;
use crate::ui_props::named;
use crate::ui_results::{Chart, Curve, button, draw_chart, explain};

/// Instants of the counts over the run.
pub const SAMPLES: usize = 300;
/// Width of the window, in points.
pub const WIDTH: f32 = 560.0;
/// The smallest zoom a new graph is fitted to (the text stays legible; the rest scrolls).
pub const MIN_FIT: f32 = 0.75;
const NAME_SIZE: f32 = 13.0;
const LABEL_SIZE: f32 = 12.0;
const PAD: f32 = 8.0;
const GAP: Vec2 = vec2(46.0, 18.0);
const PALETTE: [Color32; 7] = [
    Color32::from_rgb(90, 150, 240),
    Color32::from_rgb(250, 204, 89),
    Color32::from_rgb(115, 217, 128),
    Color32::from_rgb(255, 140, 115),
    Color32::from_rgb(191, 140, 242),
    Color32::from_rgb(102, 217, 230),
    Color32::from_rgb(230, 179, 140),
];

/// The flows of a playback with their counts over the run.
#[derive(Clone, Debug, PartialEq)]
pub struct FlowsView {
    pub graphs: Vec<FlowGraph>,
    /// For each graph: the instants and the tokens of each activity at them.
    pub series: Vec<(Vec<f64>, Vec<Vec<u32>>)>,
    /// The tokens of each activity of the chosen graph at the playback time.
    pub counts: Vec<u32>,
    pub t: f64,
}

/// What the window keeps: the flow chosen and the zoom (`None`: fit on the next frame).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FlowsMemory {
    pub flow: usize,
    pub zoom: Option<f32>,
}

/// The boxes of `g` at zoom 1 (in graph coordinates) for `name_w(text)`/`label_w(text)`
/// widths, and the size of the whole graph.
pub fn layout(
    g: &FlowGraph,
    name_w: impl Fn(&str) -> f32,
    label_w: impl Fn(&str) -> f32,
) -> (Vec<Rect>, Vec2) {
    if g.nodes.is_empty() {
        return (Vec::new(), Vec2::ZERO);
    }
    let mut w: f32 = 0.0;
    let mut low = f32::INFINITY;
    for n in &g.nodes {
        w = w.max(name_w(&n.name));
        // Room for counts of up to 4 digits.
        w = w.max(label_w(&format!("{}: 0000", n.label)));
        low = low.min(n.lane);
    }
    let size = vec2((w + PAD * 2.0).ceil(), NAME_SIZE + LABEL_SIZE + PAD * 2.5);
    let mut content = Vec2::ZERO;
    let boxes = g
        .nodes
        .iter()
        .map(|n| {
            let at = pos2(
                PAD + n.layer as f32 * (size.x + GAP.x),
                PAD + (n.lane - low) * (size.y + GAP.y),
            );
            let r = Rect::from_min_size(at, size);
            content = content.max(r.max.to_vec2() + vec2(PAD, PAD + GAP.y));
            r
        })
        .collect();
    (boxes, content)
}

/// Pairs of boxes that overlap (the scripted checks want none).
pub fn overlaps(boxes: &[Rect]) -> usize {
    let mut n = 0;
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            if boxes[i].intersects(boxes[j]) && boxes[i].intersect(boxes[j]).area() > 0.0 {
                n += 1;
            }
        }
    }
    n
}

/// The zoom that fits `content` in `room` points of width, between [`MIN_FIT`] and 1.
pub fn fit(room: f32, content: Vec2) -> f32 {
    if room <= 0.0 || content.x <= 0.0 {
        return 1.0;
    }
    ((room - 4.0) / content.x).clamp(MIN_FIT, 1.0)
}

/// Draws the window in `rect` (points).
pub fn show(
    ui: &mut Ui,
    rect: Rect,
    v: &FlowsView,
    mem: &mut FlowsMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let painter = ui.painter().clone();
    painter.rect_filled(rect, sizes::RADIUS, colors::PANEL);
    painter.rect_stroke(
        rect,
        sizes::RADIUS,
        Stroke::new(1.0, colors::BORDER),
        StrokeKind::Inside,
    );
    let panel = ui.allocate_rect(rect, Sense::hover());
    named(ui, reg, "PainelFluxos", &panel, "Fluxos de tokens");
    let inner = rect.shrink2(Vec2::new(14.0, 12.0));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(inner.expand(2.0));
    let ui = &mut child;
    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
    mem.flow = mem.flow.min(v.graphs.len().saturating_sub(1));
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Fluxos de tokens")
                .size(sizes::HEADING)
                .strong()
                .color(colors::HEADING),
        );
        let names: Vec<String> = v.graphs.iter().map(|g| g.name.clone()).collect();
        if !names.is_empty() {
            let mut pick = reg
                .take_request("Fluxos_Escolha")
                .and_then(|n| names.iter().position(|x| *x == n));
            let r = egui::ComboBox::from_id_salt("utsim_fluxos_escolha")
                .selected_text(&names[mem.flow])
                .show_ui(ui, |ui| {
                    for (k, n) in names.iter().enumerate() {
                        if ui.selectable_label(k == mem.flow, n).clicked() {
                            pick = Some(k);
                        }
                    }
                });
            crate::ui_props::named_list(
                ui,
                reg,
                "Fluxos_Escolha",
                &r.response,
                &names[mem.flow],
                names.clone(),
            );
            if let Some(k) = pick.filter(|k| *k != mem.flow) {
                mem.flow = k;
                mem.zoom = None;
                out.push(format!("fluxos:escolher:{k}"));
            }
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if button(ui, reg, "Fluxos_Fechar", "Fechar", "Fechar os fluxos", true) {
                out.push("fluxos:fechar".into());
            }
        });
    });
    let Some(g) = v.graphs.get(mem.flow) else {
        explain(ui, "O modelo não tem fluxos de tokens.");
        return;
    };
    explain(
        ui,
        "Tokens em cada atividade no instante da reprodução; embaixo, as contagens ao longo da rodada (a linha vertical t é o instante atual).",
    );
    let width_of = |text: &str, size: f32| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), FontId::proportional(size), colors::TEXT)
            .size()
            .x
    };
    let (boxes, content) = layout(g, |t| width_of(t, NAME_SIZE), |t| width_of(t, LABEL_SIZE));
    let room = ui.available_width();
    let zoom = *mem.zoom.get_or_insert_with(|| fit(room, content));
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Zoom (Ctrl + roda no grafo)")
                .small()
                .color(colors::TEXT_WEAK),
        );
        if button(ui, reg, "Fluxos_ZoomMenos", "−", "Diminui", true) {
            mem.zoom = Some((zoom / 1.25).clamp(0.3, 2.5));
        }
        if button(ui, reg, "Fluxos_ZoomMais", "+", "Aumenta", true) {
            mem.zoom = Some((zoom * 1.25).clamp(0.3, 2.5));
        }
        if button(
            ui,
            reg,
            "Fluxos_Ajustar",
            "Ajustar",
            "Cabe na largura",
            true,
        ) {
            mem.zoom = Some(fit(room, content));
        }
    });
    let zoom = mem.zoom.unwrap_or(1.0);
    let graph_h = (ui.available_height() * 0.5).max(160.0);
    let mut graph_rect = Rect::NOTHING;
    let scroll = ScrollArea::both()
        .id_salt("utsim_fluxos_grafo")
        .max_height(graph_h)
        .min_scrolled_height(graph_h)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let size = (content * zoom).max(vec2(ui.available_width(), 40.0));
            let (r, resp) = ui.allocate_exact_size(size, Sense::hover());
            graph_rect = r;
            draw_graph(ui, r.min, zoom, g, &boxes, &v.counts);
            resp
        });
    // Ctrl + wheel over the graph zooms (egui turns it into a zoom; the wheel scrolls).
    let factor = ui.input(|i| i.zoom_delta());
    if scroll.inner.hovered() && (factor - 1.0).abs() > 1e-4 {
        mem.zoom = Some((zoom * factor).clamp(0.3, 2.5));
        ui.ctx().request_repaint();
    }
    reg.put_info(
        "Fluxos_Rolagem",
        WidgetInfo {
            rect: scroll.inner_rect.intersect(ui.clip_rect()),
            text: String::new(),
            enabled: true,
            checked: None,
            choices: Vec::new(),
            count: None,
        },
    );
    let peak = v
        .series
        .get(mem.flow)
        .map_or(0, |s| s.1.iter().flatten().copied().max().unwrap_or(0));
    reg.put_info(
        "Fluxos_Grafo",
        WidgetInfo {
            rect: graph_rect
                .intersect(scroll.inner_rect)
                .intersect(ui.clip_rect()),
            text: format!(
                "atividades={} pico={peak} sobreposicoes={} cortadas=0 zoom={zoom}",
                g.nodes.len(),
                overlaps(&boxes)
            ),
            enabled: true,
            checked: None,
            choices: Vec::new(),
            count: Some(g.nodes.len()),
        },
    );
    if let Some((times, all)) = v.series.get(mem.flow) {
        let curves: Vec<Curve> = all
            .iter()
            .enumerate()
            .map(|(a, row)| Curve {
                name: g.nodes.get(a).map_or(String::new(), |n| n.name.clone()),
                points: times
                    .iter()
                    .zip(row)
                    .map(|(t, c)| [*t, *c as f64])
                    .collect(),
                color: PALETTE[a % PALETTE.len()],
                width: 1.5,
            })
            .collect();
        let h = (ui.available_height() - 30.0).max(140.0);
        let chart = Chart::Lines {
            curves,
            markers: vec![(v.t, "t".into())],
            x: "tempo".into(),
            y: "tokens".into(),
        };
        draw_chart(ui, reg, "Fluxos_Grafico", &chart, "sem contagens", h);
    }
}

fn draw_graph(ui: &Ui, origin: Pos2, zoom: f32, g: &FlowGraph, boxes: &[Rect], counts: &[u32]) {
    let p = ui.painter();
    let at = |r: Rect| Rect::from_min_size(origin + r.min.to_vec2() * zoom, r.size() * zoom);
    let arrow = Color32::from_rgb(140, 153, 173);
    let stroke = Stroke::new(1.5, arrow);
    for &(a, b) in &g.edges {
        let (Some(&ra), Some(&rb)) = (boxes.get(a), boxes.get(b)) else {
            continue;
        };
        let (ra, rb) = (at(ra), at(rb));
        let (mut from, to);
        if rb.min.x > ra.max.x {
            // Forward: from the right side of one box to the left side of the next.
            from = pos2(ra.max.x, ra.center().y);
            to = pos2(rb.min.x, rb.center().y);
        } else {
            // Back (a rework loop) or in the same column: under the boxes.
            let low = ra.max.y.max(rb.max.y) + GAP.y * 0.6 * zoom;
            let a0 = pos2(ra.center().x, ra.max.y);
            to = pos2(rb.center().x, rb.max.y);
            p.add(Shape::line(
                vec![a0, pos2(a0.x, low), pos2(to.x, low)],
                stroke,
            ));
            from = pos2(to.x, low);
        }
        if (to - from).length() < 1e-3 {
            from.y += 1.0;
        }
        p.line_segment([from, to], stroke);
        let d = (to - from).normalized();
        let s = vec2(-d.y, d.x) * 4.0 * zoom;
        p.add(Shape::convex_polygon(
            vec![to, to - d * 9.0 * zoom + s, to - d * 9.0 * zoom - s],
            arrow,
            Stroke::NONE,
        ));
    }
    for (i, n) in g.nodes.iter().enumerate() {
        let Some(&r) = boxes.get(i) else { continue };
        let r = at(r);
        let tokens = counts.get(i).copied().unwrap_or(0);
        let hot = tokens > 0;
        p.rect_filled(r, 3.0, Color32::from_rgb(41, 46, 56));
        p.rect_stroke(
            r,
            3.0,
            Stroke::new(
                if hot { 2.0 } else { 1.0 },
                if hot {
                    colors::WARNING
                } else {
                    Color32::from_rgb(102, 115, 133)
                },
            ),
            StrokeKind::Inside,
        );
        p.text(
            r.min + vec2(PAD, PAD) * zoom,
            Align2::LEFT_TOP,
            &n.name,
            FontId::proportional(NAME_SIZE * zoom),
            colors::TEXT_STRONG,
        );
        p.text(
            r.min + vec2(PAD, PAD * 1.5 + NAME_SIZE) * zoom,
            Align2::LEFT_TOP,
            format!("{}: {tokens}", n.label),
            FontId::proportional(LABEL_SIZE * zoom),
            if hot {
                colors::WARNING
            } else {
                colors::TEXT_WEAK
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extras::FlowNode;

    fn graph(lanes: &[(u32, f32)]) -> FlowGraph {
        FlowGraph {
            name: "f".into(),
            block: None,
            nodes: lanes
                .iter()
                .enumerate()
                .map(|(i, &(layer, lane))| FlowNode {
                    name: format!("atividade_{i}"),
                    label: "tokens",
                    x: 0.0,
                    y: 0.0,
                    layer,
                    lane,
                })
                .collect(),
            edges: (1..lanes.len()).map(|i| (i - 1, i)).collect(),
        }
    }

    #[test]
    fn boxes_fit_their_text_and_never_overlap() {
        let g = graph(&[(0, 0.0), (1, -0.5), (1, 0.5), (2, 0.0)]);
        let w = |t: &str| t.chars().count() as f32 * 7.0;
        let (boxes, content) = layout(&g, w, w);
        assert_eq!(boxes.len(), 4);
        assert_eq!(overlaps(&boxes), 0);
        for (b, n) in boxes.iter().zip(&g.nodes) {
            assert!(b.width() - 2.0 * PAD + 0.5 >= w(&n.name));
            assert!(content.x >= b.max.x && content.y >= b.max.y);
        }
        // The same column, two lanes: one under the other.
        assert!(boxes[1].max.y <= boxes[2].min.y);
    }

    #[test]
    fn a_new_graph_is_fitted_but_stays_legible() {
        assert_eq!(fit(1000.0, vec2(500.0, 100.0)), 1.0);
        assert_eq!(fit(200.0, vec2(2000.0, 100.0)), MIN_FIT);
        let z = fit(404.0, vec2(500.0, 100.0));
        assert!((z - 0.8).abs() < 1e-6, "{z}");
    }
}
