//! The PERT/CPM window of the selection, in egui (pure: no Godot types, unit tested). It
//! replaces `pert.gd`.
//!
//! It opens in the **simulated** mode (the critical path observed in the simulation
//! itself, with queues, transporters and resources); the classic PERT and the Monte Carlo
//! over the network are one click away. Tabs: **Rede** (activity on node, boxes with
//! ES/EF/LS/LF and the float, the critical path in red, arrows as thick as the
//! criticality), **Gantt** (ES→EF, the float as a light bar), **Distribuição** (histogram of
//! the total time with the deadline), **Tabela** and **Texto** (the report of `utsim pert`).
//!
//! Every number and coordinate comes from [`crate::pert_view::PertOutput::to_json`]; the
//! window only draws. It changes nothing itself: the main scene starts the run
//! (`pert:calcular:<modo>`, with the deadline and the samples of [`PertMemory`]) and hands
//! the result back; `pert:fechar`, `pert:exportar_html` and `pert:exportar_csv` go out too.
//!
//! The widgets keep the names of the Godot controls (`PainelPert`, `Pert_Modo_<modo>`,
//! `Pert_Fechar`, `Pert_Prazo`, `Pert_Sorteios`, `Pert_Recalcular`, `Pert_Arquivo`,
//! `Pert_ExportarHTML`, `Pert_ExportarCSV`, `Pert_Resumo`, `Pert_Diagrama`, `Pert_Gantt`,
//! `Pert_DistribuicaoTexto`, `Pert_Histograma`, `Pert_Tabela`, `Pert_Texto`); the tabs are
//! `Pert_Abas_<título>`.

use crate::texto::{fx, tx, vx};
use egui::{
    Align, Align2, Color32, FontId, Id, Label, Layout, Rect, RichText, ScrollArea, Sense, Shape,
    Stroke, StrokeKind, TextEdit, TextStyle, Ui, UiBuilder, Vec2, pos2, vec2,
};
use serde_json::Value;
use utsim_ui_tema::{colors, sizes};

use crate::egui_widgets::{Registry, WidgetInfo};
use crate::ui_props::{named, named_toggle};
use crate::ui_results::{
    Cell, Row, Table, button, draw_table, error, explain, field, field_response, histogram_with,
    num, tab_row,
};

/// The tabs, in order.
pub fn tabs() -> [&'static str; 5] {
    [
        tx("godot-ui-pert-rede"),
        tx("godot-ui-pert-gantt"),
        tx("godot-ui-pert-distribuicao"),
        tx("godot-ui-pert-tabela"),
        tx("godot-ui-pert-texto"),
    ]
}
/// The modes: id (as the bridge names it), label and tip.
pub fn modes() -> [(&'static str, &'static str, &'static str); 3] {
    [
        (
            "simulado",
            tx("godot-ui-pert-simulado"),
            tx("godot-ui-pert-caminho_critico_observado_na_simulacao"),
        ),
        (
            "classico",
            tx("godot-ui-pert-classico"),
            tx("godot-ui-pert-pert_classico_estimativas_a_m"),
        ),
        (
            "montecarlo",
            tx("godot-ui-pert-monte_carlo"),
            tx("godot-ui-pert-sorteios_das_duracoes_sobre_a"),
        ),
    ]
}
/// The critical path and the boxes on it.
const HOT: Color32 = Color32::from_rgb(255, 107, 77);
const LINE: Color32 = Color32::from_rgb(140, 153, 173);
const BAR: Color32 = Color32::from_rgb(90, 150, 240);
const BOX: Color32 = Color32::from_rgb(41, 46, 56);

/// A computed PERT/CPM: what `PertOutput::to_json` gives and the report as text.
pub struct PertView {
    pub data: Value,
    pub text: String,
}

/// What the window keeps: the selection's title, the mode, the forms and the line shown
/// while it calculates (or why it did not).
#[derive(Clone, Debug, PartialEq)]
pub struct PertMemory {
    pub tab: usize,
    pub mode: String,
    pub title: String,
    pub deadline: String,
    pub samples: String,
    pub file: String,
    /// "Calculando..." or the error of the last request; empty once a result is shown.
    pub info: String,
    pub info_error: bool,
}

impl Default for PertMemory {
    fn default() -> Self {
        PertMemory {
            tab: 0,
            mode: "simulado".into(),
            title: String::new(),
            deadline: String::new(),
            samples: "10000".into(),
            file: String::new(),
            info: String::new(),
            info_error: false,
        }
    }
}

impl PertMemory {
    /// The deadline typed (`0`: none; a comma works as the decimal point).
    pub fn deadline_value(&self) -> f64 {
        self.deadline
            .trim()
            .replace(',', ".")
            .parse::<f64>()
            .ok()
            .filter(|d| d.is_finite() && *d > 0.0)
            .unwrap_or(0.0)
    }

    /// The Monte Carlo samples typed, between 100 and a million (10000 when unreadable).
    pub fn samples_value(&self) -> i64 {
        self.samples
            .trim()
            .parse::<i64>()
            .map_or(10_000, |n| n.clamp(100, 1_000_000))
    }
}

fn f(v: &Value) -> Option<f64> {
    v.as_f64().filter(|x| x.is_finite())
}

fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// `mean ± half_width` of the total time.
fn mh(s: &Value) -> String {
    crate::ui_results::mh(s, false)
}

/// The summary lines over the tabs, with their colours.
pub fn summary(d: &Value) -> Vec<(String, Color32)> {
    let t = &d["total"];
    let mut lines = vec![(
        format!("{} ({}).", text(&d["mode_label"]), text(&d["selection"])),
        colors::TEXT_STRONG,
    )];
    let spread = f(&t["sd"]).map_or(String::new(), |sd| {
        fx("godot-ui-pert-desvio", &[("sd", vx(&num(sd)))])
    });
    let units = if t["units"].is_null() {
        String::new()
    } else {
        fx(
            "godot-ui-pert-unidades_sorteios",
            &[("units", vx(&text(&t["units"])))],
        )
    };
    lines.push((
        fx(
            "godot-ui-pert-tempo_total_cpm_nas_medias",
            &[
                ("t", vx(&mh(t))),
                ("spread", vx(&spread)),
                ("units", vx(&units)),
                ("project", vx(&num(f(&d["project"]).unwrap_or(f64::NAN)))),
            ],
        ),
        colors::TEXT,
    ));
    let dl = &t["deadline"];
    if dl.is_object() {
        let mut p = format!(
            "{}%",
            num(f(&dl["probability"]).unwrap_or(f64::NAN) * 100.0)
        );
        if let (Some(lo), Some(hi)) = (f(&dl["low"]), f(&dl["high"])) {
            p.push_str(&fx(
                "godot-ui-pert-ic_a",
                &[("lo", vx(&num(lo * 100.0))), ("hi", vx(&num(hi * 100.0)))],
            ));
        }
        lines.push((
            fx(
                "godot-ui-pert-p_t",
                &[
                    ("deadline", vx(&num(f(&dl["deadline"]).unwrap_or(f64::NAN)))),
                    ("p", vx(&p)),
                ],
            ),
            colors::TEXT,
        ));
    }
    if let Some(path) = d["paths"].as_array().and_then(|p| p.first()) {
        let names: Vec<String> = path["names"]
            .as_array()
            .into_iter()
            .flatten()
            .map(text)
            .collect();
        lines.push((
            fx(
                "godot-ui-pert-caminho_critico",
                &[
                    ("names", vx(&names.join(" → "))),
                    (
                        "index",
                        vx(&num(f(&path["index"]).unwrap_or(f64::NAN) * 100.0)),
                    ),
                ],
            ),
            HOT,
        ));
    }
    for w in d["warnings"].as_array().into_iter().flatten() {
        lines.push((
            fx("godot-ui-pert-aviso", &[("w", vx(&text(w)))]),
            colors::WARNING,
        ));
    }
    lines
}

/// The line under the "Distribuição" tab's title.
pub fn distribution_text(d: &Value) -> String {
    let source = match d["mode"].as_str() {
        Some("classico") => tx("godot-ui-pert-aproximacao_normal_do_pert_classico"),
        Some("montecarlo") => "sorteios",
        _ => tx("godot-ui-pert-unidades_observadas"),
    };
    let marker = if d["total"]["deadline"].is_object() {
        tx("godot-ui-pert-a_linha_amarela_e_o")
    } else {
        ""
    };
    fx(
        "godot-ui-pert-distribuicao_do_tempo_total",
        &[("source", vx(&source)), ("marker", vx(&marker))],
    )
}

/// The status line after a calculation.
pub fn status_line(d: &Value) -> String {
    fx(
        "godot-ui-pert-pert_cpm_tempo_total",
        &[
            ("mode", vx(&text(&d["mode"]))),
            ("total", vx(&mh(&d["total"]))),
        ],
    )
}

/// The objects on the critical path (outlined in red in the 3D view).
pub fn critical_objects(d: &Value) -> Vec<String> {
    d["critical_objects"]
        .as_array()
        .into_iter()
        .flatten()
        .map(text)
        .collect()
}

/// The activities' table.
pub fn activity_table(d: &Value) -> Table {
    let keys = ["mean", "es", "ef", "ls", "lf", "total_float", "free_float"];
    let rows = d["activities"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|a| {
            let crit = a["critical"].as_bool() == Some(true);
            let mut cells = vec![if crit {
                Cell::colored(text(&a["name"]), HOT)
            } else {
                Cell::plain(text(&a["name"]))
            }];
            for k in keys {
                cells.push(Cell::plain(f(&a[k]).map_or("-".into(), num)));
            }
            cells.push(Cell::plain(
                f(&a["criticality"]).map_or("-".into(), |c| format!("{}%", num(c * 100.0))),
            ));
            Row { cells, hot: false }
        })
        .collect();
    Table {
        header: [
            tx("godot-ui-pert-atividade"),
            tx("godot-ui-pert-duracao"),
            "ES",
            "EF",
            "LS",
            "LF",
            tx("godot-ui-pert-folga_total"),
            tx("godot-ui-pert-folga_livre"),
            tx("godot-ui-pert-criticidade"),
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        right: vec![false, true, true, true, true, true, true, true, true],
        rows,
    }
}

fn record(reg: &mut Registry, name: &str, rect: Rect, text: String, count: Option<usize>) {
    reg.put_info(
        name,
        WidgetInfo {
            rect,
            text,
            enabled: true,
            checked: None,
            choices: Vec::new(),
            count,
        },
    );
}

/// Draws the window in `rect` (points); `view` is `None` until the first result.
pub fn show(
    ui: &mut Ui,
    rect: Rect,
    view: Option<&PertView>,
    mem: &mut PertMemory,
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
    named(ui, reg, "PainelPert", &panel, &mem.title);
    let inner = rect.shrink2(Vec2::new(16.0, 12.0));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(inner.expand(2.0));
    let ui = &mut child;
    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
    header(ui, mem, reg, out);
    options(ui, view, mem, reg, out);
    // The summary (or what is happening).
    let block = ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 2.0;
        if !mem.info.is_empty() {
            if mem.info_error {
                error(ui, &mem.info);
            } else {
                explain(ui, &mem.info);
            }
        }
        if let Some(v) = view {
            for (line, color) in summary(&v.data) {
                ui.add(Label::new(RichText::new(line).color(color)).wrap());
            }
        }
    });
    let shown = {
        let mut lines: Vec<String> = vec![mem.info.clone()];
        if let Some(v) = view {
            lines.extend(summary(&v.data).into_iter().map(|l| l.0));
        }
        lines.join("\n")
    };
    named(ui, reg, "Pert_Resumo", &block.response, shown.trim());
    tab_row(ui, reg, "Pert_Abas", &tabs(), &mut mem.tab);
    let Some(v) = view else {
        // The diagram is there, empty, until the first result.
        record(reg, "Pert_Diagrama", Rect::NOTHING, String::new(), Some(0));
        return;
    };
    let d = &v.data;
    let mut scroll = egui::style::ScrollStyle::floating();
    scroll.dormant_handle_opacity = 0.5;
    ui.spacing_mut().scroll = scroll;
    let boxes = d["diagram"]["boxes"].as_array().map_or(0, Vec::len);
    match mem.tab {
        0 => {
            ScrollArea::both()
                .id_salt("utsim_pert_rede")
                .auto_shrink([false, false])
                .show(ui, |ui| diagram(ui, reg, &d["diagram"]));
        }
        1 => {
            ScrollArea::vertical()
                .id_salt("utsim_pert_gantt")
                .auto_shrink([false, false])
                .show(ui, |ui| gantt(ui, reg, d));
        }
        2 => {
            ScrollArea::vertical()
                .id_salt("utsim_pert_dist")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let t = distribution_text(d);
                    let r = explain(ui, &t);
                    named(ui, reg, "Pert_DistribuicaoTexto", &r, &t);
                    let h = &d["histogram"];
                    let counts: Vec<f64> = h["counts"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_f64)
                        .collect();
                    let deadline = f(&d["total"]["deadline"]["deadline"]);
                    let label =
                        deadline.map(|x| fx("godot-ui-pert-prazo", &[("num", vx(&num(x)))]));
                    let r = if counts.is_empty() {
                        explain(ui, tx("godot-ui-pert-sem_distribuicao_para_este_modo"))
                    } else {
                        histogram_with(
                            ui,
                            "Pert_Histograma",
                            f(&h["min"]).unwrap_or(0.0),
                            f(&h["bin_width"]).unwrap_or(1.0),
                            &counts,
                            300.0,
                            deadline.zip(label.as_deref()),
                        )
                    };
                    record(
                        reg,
                        "Pert_Histograma",
                        r.rect.intersect(ui.clip_rect()),
                        counts.len().to_string(),
                        Some(counts.len()),
                    );
                });
        }
        3 => {
            ScrollArea::vertical()
                .id_salt("utsim_pert_tabela")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    draw_table(ui, reg, "Pert_Tabela", &activity_table(d));
                    for h in d["assumptions"].as_array().into_iter().flatten() {
                        explain(ui, &fx("godot-ui-pert-hipotese", &[("h", vx(&text(h)))]));
                    }
                });
        }
        _ => {
            let mut shown = v.text.as_str();
            let r = ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(&mut shown)
                    .id(Id::new("utsim_pert_texto"))
                    .font(TextStyle::Monospace),
            );
            named(ui, reg, "Pert_Texto", &r, &v.text);
        }
    }
    if mem.tab != 0 {
        // The diagram of a tab not shown is still counted (as a hidden Godot tab was).
        record(
            reg,
            "Pert_Diagrama",
            Rect::NOTHING,
            String::new(),
            Some(boxes),
        );
    }
}

fn header(ui: &mut Ui, mem: &mut PertMemory, reg: &mut Registry, out: &mut Vec<String>) {
    ui.horizontal(|ui| {
        ui.add(
            Label::new(
                RichText::new(&mem.title)
                    .size(sizes::HEADING)
                    .strong()
                    .color(colors::HEADING),
            )
            .truncate(),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if button(
                ui,
                reg,
                "Pert_Fechar",
                tx("godot-ui-pert-fechar"),
                tx("godot-ui-pert-fechar_a_janela_pert_cpm"),
                true,
            ) {
                out.push("pert:fechar".into());
            }
            ui.add_space(8.0);
            // Right to left: the last mode first.
            for (id, label, tip) in modes().iter().rev() {
                let on = mem.mode == *id;
                let r = ui
                    .add(egui::Button::selectable(on, *label))
                    .on_hover_text(*tip);
                named_toggle(ui, reg, &format!("Pert_Modo_{id}"), &r, label, on);
                if r.clicked() {
                    mem.mode = id.to_string();
                    out.push(format!("pert:calcular:{id}"));
                }
            }
        });
    });
}

fn options(
    ui: &mut Ui,
    view: Option<&PertView>,
    mem: &mut PertMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(tx("godot-ui-pert-prazo-2")).color(colors::TEXT));
        let r = field_response(
            ui,
            reg,
            "Pert_Prazo",
            &mut mem.deadline,
            90.0,
            tx("godot-ui-pert-sem_prazo"),
        );
        if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            out.push(format!("pert:calcular:{}", mem.mode));
        }
        ui.label(RichText::new(tx("godot-ui-pert-sorteios_monte_carlo")).color(colors::TEXT));
        field(ui, reg, "Pert_Sorteios", &mut mem.samples, 80.0, "10000");
        if button(
            ui,
            reg,
            "Pert_Recalcular",
            tx("godot-ui-pert-recalcular"),
            tx("godot-ui-pert-calcula_de_novo_no_modo"),
            true,
        ) {
            out.push(format!("pert:calcular:{}", mem.mode));
        }
        ui.add_space(16.0);
        let w = (ui.available_width() - 190.0).clamp(160.0, 420.0);
        field(
            ui,
            reg,
            "Pert_Arquivo",
            &mut mem.file,
            w,
            tx("godot-ui-pert-relatorio_html"),
        );
        let ready = view.is_some();
        if button(
            ui,
            reg,
            "Pert_ExportarHTML",
            tx("godot-ui-pert-exportar_html"),
            tx("godot-ui-pert-grava_o_relatorio_html_do"),
            ready,
        ) {
            out.push("pert:exportar_html".into());
        }
        if button(
            ui,
            reg,
            "Pert_ExportarCSV",
            tx("godot-ui-pert-csv"),
            tx("godot-ui-pert-grava_as_tabelas_em_csv"),
            ready,
        ) {
            out.push("pert:exportar_csv".into());
        }
    });
}

fn fmt(b: &Value, k: &str) -> String {
    f(&b[k]).map_or("-".into(), num)
}

/// The activity-on-node diagram: the boxes and arrows laid out by the bridge.
fn diagram(ui: &mut Ui, reg: &mut Registry, d: &Value) {
    const PAD: f32 = 10.0;
    let w = f(&d["width"]).unwrap_or(300.0) as f32 + 2.0 * PAD;
    let h = f(&d["height"]).unwrap_or(200.0) as f32 + 2.0 * PAD;
    let (rect, resp) = ui.allocate_exact_size(vec2(w.max(ui.available_width()), h), Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, sizes::RADIUS, colors::FIELD);
    let o = rect.min + vec2(PAD, PAD);
    let at = |v: &Value| {
        pos2(
            o.x + f(&v[0]).unwrap_or(0.0) as f32,
            o.y + f(&v[1]).unwrap_or(0.0) as f32,
        )
    };
    for a in d["arrows"].as_array().into_iter().flatten() {
        let pts: Vec<egui::Pos2> = a["points"]
            .as_array()
            .into_iter()
            .flatten()
            .map(at)
            .collect();
        let (Some(&first), Some(&last)) = (pts.first(), pts.last()) else {
            continue;
        };
        let crit = a["critical"].as_bool() == Some(true);
        let color = if crit { HOT } else { LINE };
        let width = 1.0 + 4.0 * f(&a["weight"]).unwrap_or(0.0) as f32;
        p.line_segment([first, last], Stroke::new(width, color));
        let dir = (last - first).normalized();
        let side = vec2(-dir.y, dir.x) * 5.0;
        p.add(Shape::convex_polygon(
            vec![last, last - dir * 10.0 + side, last - dir * 10.0 - side],
            color,
            Stroke::NONE,
        ));
    }
    let name_font = FontId::proportional(12.5);
    let small = FontId::proportional(10.5);
    let list = d["boxes"].as_array().cloned().unwrap_or_default();
    for b in &list {
        let r = Rect::from_min_size(
            o + vec2(
                f(&b["x"]).unwrap_or(0.0) as f32,
                f(&b["y"]).unwrap_or(0.0) as f32,
            ),
            vec2(
                f(&b["width"]).unwrap_or(120.0) as f32,
                f(&b["height"]).unwrap_or(60.0) as f32,
            ),
        );
        let crit = b["critical"].as_bool() == Some(true);
        p.rect_filled(r, 4.0, BOX);
        p.rect_stroke(
            r,
            4.0,
            Stroke::new(if crit { 2.5 } else { 1.0 }, if crit { HOT } else { LINE }),
            StrokeKind::Inside,
        );
        let clip = p.with_clip_rect(r.shrink(2.0).intersect(p.clip_rect()));
        clip.text(
            r.min + vec2(6.0, r.height() * 0.06),
            Align2::LEFT_TOP,
            text(&b["name"]),
            name_font.clone(),
            colors::TEXT_STRONG,
        );
        if b["milestone"].as_bool() == Some(true) {
            continue;
        }
        clip.text(
            r.min + vec2(6.0, r.height() * 0.33),
            Align2::LEFT_TOP,
            format!("ES {}  EF {}", fmt(b, "es"), fmt(b, "ef")),
            small.clone(),
            LINE,
        );
        clip.text(
            r.min + vec2(6.0, r.height() * 0.53),
            Align2::LEFT_TOP,
            format!("LS {}  LF {}", fmt(b, "ls"), fmt(b, "lf")),
            small.clone(),
            LINE,
        );
        let c = f(&b["criticality"]).unwrap_or(0.0);
        clip.text(
            r.min + vec2(6.0, r.height() * 0.73),
            Align2::LEFT_TOP,
            format!(
                "folga {}  crít. {}%",
                fmt(b, "total_float"),
                (c * 100.0).round()
            ),
            small.clone(),
            if crit { HOT } else { LINE },
        );
    }
    record(
        reg,
        "Pert_Diagrama",
        resp.rect.intersect(ui.clip_rect()),
        list.iter()
            .map(|b| text(&b["name"]))
            .collect::<Vec<_>>()
            .join(", "),
        Some(list.len()),
    );
}

/// The Gantt of the CPM on the means: ES→EF, and the total float after it.
fn gantt(ui: &mut Ui, reg: &mut Registry, d: &Value) {
    const LEFT: f32 = 190.0;
    const ROW: f32 = 24.0;
    let rows = d["activities"].as_array().cloned().unwrap_or_default();
    let project = f(&d["project"]).unwrap_or(1.0).max(1e-9);
    let height = ROW * rows.len() as f32 + 50.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, sizes::RADIUS, colors::FIELD);
    let w = (rect.width() - LEFT - 24.0).max(100.0);
    let k = w as f64 / project;
    let body = FontId::proportional(sizes::SMALL + 1.0);
    for (i, a) in rows.iter().enumerate() {
        let y = rect.min.y + 10.0 + i as f32 * ROW;
        if i % 2 == 1 {
            p.rect_filled(
                Rect::from_min_size(pos2(rect.min.x, y), vec2(rect.width(), ROW)),
                0.0,
                colors::PANEL,
            );
        }
        let crit = a["critical"].as_bool() == Some(true);
        p.with_clip_rect(Rect::from_min_size(
            pos2(rect.min.x, y),
            vec2(LEFT - 10.0, ROW),
        ))
        .text(
            pos2(rect.min.x + 8.0, y + ROW / 2.0),
            Align2::LEFT_CENTER,
            text(&a["name"]),
            body.clone(),
            if crit { HOT } else { colors::TEXT },
        );
        let es = f(&a["es"]).unwrap_or(0.0);
        let ef = f(&a["ef"]).unwrap_or(es);
        let tf = f(&a["total_float"]).unwrap_or(0.0);
        let x0 = rect.min.x + LEFT + (es * k) as f32;
        p.rect_filled(
            Rect::from_min_size(
                pos2(x0, y + 5.0),
                vec2((((ef - es) * k) as f32).max(1.5), 14.0),
            ),
            2.0,
            if crit { HOT } else { BAR },
        );
        if tf > 1e-9 {
            p.rect_filled(
                Rect::from_min_size(
                    pos2(rect.min.x + LEFT + (ef * k) as f32, y + 9.0),
                    vec2((tf * k) as f32, 6.0),
                ),
                1.0,
                BAR.gamma_multiply(0.35),
            );
        }
    }
    let base = rect.min.y + 14.0 + rows.len() as f32 * ROW;
    let x0 = rect.min.x + LEFT;
    p.line_segment([pos2(x0, base), pos2(x0 + w, base)], Stroke::new(1.0, LINE));
    for j in 0..=4 {
        let x = x0 + w * j as f32 / 4.0;
        p.line_segment([pos2(x, base), pos2(x, base + 4.0)], Stroke::new(1.0, LINE));
        p.text(
            pos2(x, base + 6.0),
            Align2::CENTER_TOP,
            num(project * j as f64 / 4.0),
            FontId::proportional(sizes::SMALL),
            colors::TEXT_WEAK,
        );
    }
    record(
        reg,
        "Pert_Gantt",
        resp.rect.intersect(ui.clip_rect()),
        String::new(),
        Some(rows.len()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pert_view::{PertRequest, run};
    use std::path::Path;
    use utsim_model::Model;
    use utsim_pert::Mode;

    pub(crate) fn data(mode: Mode, deadline: Option<f64>) -> Value {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/mm1.ron");
        let model = Model::load(&path).unwrap();
        let mut req = PertRequest::of(&["chegada".into(), "saida".into()], true, mode).unwrap();
        req.deadline = deadline;
        req.samples = 500;
        run(&model, &req).unwrap().to_json()
    }

    #[test]
    fn the_summary_names_the_mode_and_the_critical_path() {
        let d = data(Mode::Classic, Some(2.0));
        let lines: Vec<String> = summary(&d).into_iter().map(|l| l.0).collect();
        assert!(lines[0].contains("PERT clássico"), "{lines:?}");
        assert!(
            lines.iter().any(|l| l.starts_with("Tempo total:")),
            "{lines:?}"
        );
        assert!(lines.iter().any(|l| l.starts_with("P(T ≤ 2")), "{lines:?}");
        assert!(
            lines.iter().any(|l| l.starts_with("Caminho crítico:")),
            "{lines:?}"
        );
        assert!(distribution_text(&d).contains("prazo"));
        assert!(status_line(&d).starts_with("PERT/CPM (classico)"));
        assert!(critical_objects(&d).contains(&"maquina".to_string()));
    }

    #[test]
    fn the_table_has_one_row_per_activity() {
        let d = data(Mode::Classic, None);
        let t = activity_table(&d);
        assert_eq!(t.header.len(), t.right.len());
        assert_eq!(t.rows.len(), d["activities"].as_array().unwrap().len());
        assert!(t.text().contains("maquina"));
        assert!(!distribution_text(&d).contains("prazo"));
    }

    #[test]
    fn the_forms_read_the_deadline_and_the_samples() {
        let mut m = PertMemory::default();
        assert_eq!(m.deadline_value(), 0.0);
        m.deadline = " 2,5 ".into();
        assert_eq!(m.deadline_value(), 2.5);
        m.deadline = "-1".into();
        assert_eq!(m.deadline_value(), 0.0);
        m.samples = "5".into();
        assert_eq!(m.samples_value(), 100);
        m.samples = "x".into();
        assert_eq!(m.samples_value(), 10_000);
    }
}
