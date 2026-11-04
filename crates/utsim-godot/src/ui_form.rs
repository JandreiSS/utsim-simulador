//! A panel described by the main scene as rows (pure: no Godot types, unit tested): the
//! right panel of the Rede and Corredor tools and of a selected link, which the 3D editors
//! (`rede.gd`, `corredores.gd`) fill with what they hold. It replaces the Godot controls
//! the main scene built for them.
//!
//! The rows come as JSON (`JSON.stringify` of an array of dictionaries), one per line of
//! the panel, each with a `kind`:
//!
//! - `title`, `section`: `text`;
//! - `info`, `help`, `warning`, `error`: `text` (wrapped; `name` for the scripts);
//! - `check`: `name`, `text`, `on`, `action`;
//! - `number`: `name`, `label`, `value`, `min`, `max`, `step`, `action`;
//! - `line`: `name`, `label`, `value`, `action` (sent with Enter or when it loses focus);
//! - `choice`: `name`, `label`, `items` (`[[label, value], ...]`), `value`, `action`;
//! - `button`: `name`, `text`, `enabled` (default true), `action`.
//!
//! The panel changes nothing: an edit goes out as `form:<action>:<value>` (`1`/`0` for a
//! check, the number, the text or the choice's value; nothing for a button).

use std::collections::HashMap;

use egui::text::CCursorRange;
use egui::{Align, Label, Layout, RichText, Ui};
use serde_json::Value;
use utsim_ui_tema::{colors, sizes};

use crate::egui_widgets::Registry;
use crate::ui_props::{named, named_list, named_toggle};

/// One line of the panel.
#[derive(Clone, Debug, PartialEq)]
pub enum FormRow {
    Title(String),
    Section(String),
    /// Text in a tone: `info`, `help`, `warning` or `error`.
    Text {
        name: String,
        text: String,
        tone: String,
    },
    Check {
        name: String,
        text: String,
        on: bool,
        action: String,
    },
    Number {
        name: String,
        label: String,
        value: f64,
        min: f64,
        max: f64,
        step: f64,
        action: String,
    },
    Line {
        name: String,
        label: String,
        value: String,
        action: String,
    },
    Choice {
        name: String,
        label: String,
        items: Vec<(String, String)>,
        value: String,
        action: String,
    },
    Button {
        name: String,
        text: String,
        enabled: bool,
        action: String,
    },
}

fn s(v: &Value, k: &str) -> String {
    match &v[k] {
        Value::String(x) => x.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn n(v: &Value, k: &str, default: f64) -> f64 {
    v[k].as_f64().filter(|x| x.is_finite()).unwrap_or(default)
}

/// The rows of `json` (an array of dictionaries); unknown kinds are skipped.
pub fn parse(json: &str) -> Vec<FormRow> {
    let Ok(Value::Array(list)) = serde_json::from_str::<Value>(json) else {
        return Vec::new();
    };
    list.iter()
        .filter_map(|r| {
            Some(match r["kind"].as_str()? {
                "title" => FormRow::Title(s(r, "text")),
                "section" => FormRow::Section(s(r, "text")),
                tone @ ("info" | "help" | "warning" | "error") => FormRow::Text {
                    name: s(r, "name"),
                    text: s(r, "text"),
                    tone: tone.to_owned(),
                },
                "check" => FormRow::Check {
                    name: s(r, "name"),
                    text: s(r, "text"),
                    on: r["on"].as_bool().unwrap_or(false),
                    action: s(r, "action"),
                },
                "number" => FormRow::Number {
                    name: s(r, "name"),
                    label: s(r, "label"),
                    value: n(r, "value", 0.0),
                    min: n(r, "min", f64::NEG_INFINITY),
                    max: n(r, "max", f64::INFINITY),
                    step: n(r, "step", 1.0),
                    action: s(r, "action"),
                },
                "line" => FormRow::Line {
                    name: s(r, "name"),
                    label: s(r, "label"),
                    value: s(r, "value"),
                    action: s(r, "action"),
                },
                "choice" => FormRow::Choice {
                    name: s(r, "name"),
                    label: s(r, "label"),
                    items: r["items"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .map(|i| match i {
                            Value::Array(p) => {
                                let t = |k: usize| match p.get(k) {
                                    Some(Value::String(x)) => x.clone(),
                                    Some(other) => other.to_string(),
                                    None => String::new(),
                                };
                                let label = t(0);
                                let value = if p.len() > 1 { t(1) } else { label.clone() };
                                (label, value)
                            }
                            Value::String(x) => (x.clone(), x.clone()),
                            other => (other.to_string(), other.to_string()),
                        })
                        .collect(),
                    value: s(r, "value"),
                    action: s(r, "action"),
                },
                "button" => FormRow::Button {
                    name: s(r, "name"),
                    text: s(r, "text"),
                    enabled: r["enabled"].as_bool().unwrap_or(true),
                    action: s(r, "action"),
                },
                _ => return None,
            })
        })
        .collect()
}

/// What the panel keeps between frames: the text being typed in each `line` and the
/// value of each `number` while it is dragged.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormMemory {
    lines: HashMap<String, (String, String)>,
}

impl FormMemory {
    /// The rows changed: a field not being typed in shows the new value.
    pub fn sync(&mut self, rows: &[FormRow]) {
        let mut keep = HashMap::new();
        for r in rows {
            let (name, value) = match r {
                FormRow::Line { name, value, .. } => (name, value.clone()),
                FormRow::Number { name, value, .. } => (name, number_text(*value)),
                _ => continue,
            };
            let entry = match self.lines.remove(name) {
                // Typed text survives while the value it started from is the same.
                Some((from, typed)) if from == value => (from, typed),
                _ => (value.clone(), value),
            };
            keep.insert(name.clone(), entry);
        }
        self.lines = keep;
    }
}

/// A one-line field over the text kept for `name` (it starts from `value`): selects all
/// when it gets the focus, Esc goes back, and the text is returned when Enter is pressed or
/// the field loses the focus changed.
fn entry(
    ui: &mut Ui,
    mem: &mut FormMemory,
    reg: &mut Registry,
    name: &str,
    value: &str,
    width: f32,
) -> Option<String> {
    let e = mem
        .lines
        .entry(name.to_owned())
        .or_insert_with(|| (value.to_owned(), value.to_owned()));
    let id = egui::Id::new(("utsim_form", name));
    let mut output = egui::TextEdit::singleline(&mut e.1)
        .id(id)
        .desired_width(width)
        .show(ui);
    let r = output.response.response.clone();
    if r.gained_focus() {
        output
            .state
            .cursor
            .set_char_range(Some(CCursorRange::select_all(&output.galley)));
        output.state.store(ui.ctx(), id);
    }
    named(ui, reg, name, &r, &e.1);
    if !r.lost_focus() {
        return None;
    }
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        e.1 = e.0.clone();
        return None;
    }
    let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
    (enter || e.1 != e.0).then(|| e.1.clone())
}

fn out_action(out: &mut Vec<String>, action: &str, value: &str) {
    if !action.is_empty() {
        out.push(format!("form:{action}:{value}"));
    }
}

fn number_text(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{x}")
    }
}

/// Draws the rows as the right panel in `rect` (where the properties go), scrolled. It
/// keeps the panel's name (`PainelPropriedades`).
pub fn show_panel(
    ui: &mut Ui,
    rect: egui::Rect,
    rows: &[FormRow],
    mem: &mut FormMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    ui.painter().rect_filled(rect, 0.0, colors::PANEL);
    ui.painter().line_segment(
        [rect.left_top(), rect.left_bottom()],
        egui::Stroke::new(1.0, colors::BORDER),
    );
    let panel = ui.allocate_rect(rect, egui::Sense::hover());
    named(ui, reg, "PainelPropriedades", &panel, "");
    let inner = rect.shrink2(egui::Vec2::new(12.0, 10.0));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(inner.expand(2.0));
    egui::ScrollArea::vertical()
        .id_salt("utsim_formulario")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_width(inner.width() - 12.0);
            show(ui, rows, mem, reg, out);
        });
}

/// Draws the rows (the caller gives the width and the scroll).
pub fn show(
    ui: &mut Ui,
    rows: &[FormRow],
    mem: &mut FormMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    ui.spacing_mut().item_spacing.y = 8.0;
    for row in rows {
        match row {
            FormRow::Title(t) => {
                ui.label(
                    RichText::new(t)
                        .size(sizes::HEADING)
                        .strong()
                        .color(colors::HEADING),
                );
            }
            FormRow::Section(t) => {
                ui.add_space(4.0);
                ui.label(RichText::new(t).strong().color(colors::HEADING));
            }
            FormRow::Text { name, text, tone } => {
                let color = match tone.as_str() {
                    "help" => colors::TEXT_WEAK,
                    "warning" => colors::WARNING,
                    "error" => colors::ERROR,
                    _ => colors::TEXT,
                };
                let r = ui.add(Label::new(RichText::new(text).color(color)).wrap());
                if !name.is_empty() {
                    named(ui, reg, name, &r, text);
                }
            }
            FormRow::Check {
                name,
                text,
                on,
                action,
            } => {
                let mut v = *on;
                let r = ui.checkbox(&mut v, text);
                named_toggle(ui, reg, name, &r, text, v);
                if r.changed() {
                    out_action(out, action, if v { "1" } else { "0" });
                }
            }
            FormRow::Number {
                name,
                label,
                value,
                min,
                max,
                step: _,
                action,
            } => {
                let start = number_text(*value);
                ui.horizontal(|ui| {
                    ui.add(Label::new(RichText::new(label).color(colors::TEXT)).wrap());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if let Some(typed) = entry(ui, mem, reg, name, &start, 90.0) {
                            match typed.replace(',', ".").parse::<f64>() {
                                Ok(v) if v.is_finite() => {
                                    let v = v.clamp(*min, *max);
                                    if (v - value).abs() > 1e-12 {
                                        out_action(out, action, &number_text(v));
                                    }
                                }
                                _ => {
                                    // Not a number: back to the value.
                                    if let Some(e) = mem.lines.get_mut(name) {
                                        e.1 = e.0.clone();
                                    }
                                }
                            }
                        }
                    });
                });
            }
            FormRow::Line {
                name,
                label,
                value,
                action,
            } => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(label).color(colors::TEXT));
                    let w = ui.available_width();
                    if let Some(typed) = entry(ui, mem, reg, name, value, w) {
                        out_action(out, action, typed.trim());
                    }
                });
            }
            FormRow::Choice {
                name,
                label,
                items,
                value,
                action,
            } => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(label).color(colors::TEXT));
                    let shown = items
                        .iter()
                        .find(|i| i.1 == *value)
                        .map_or(value.clone(), |i| i.0.clone());
                    let mut picked: Option<String> = None;
                    if let Some(want) = reg.take_request(name) {
                        picked = items.iter().find(|i| i.0 == want).map(|i| i.1.clone());
                    }
                    let r = egui::ComboBox::from_id_salt(("utsim_form", name.as_str()))
                        .selected_text(shown.clone())
                        .width(ui.available_width().min(240.0))
                        .show_ui(ui, |ui| {
                            for (l, v) in items {
                                if ui.selectable_label(v == value, l).clicked() {
                                    picked = Some(v.clone());
                                }
                            }
                        });
                    named_list(
                        ui,
                        reg,
                        name,
                        &r.response,
                        &shown,
                        items.iter().map(|i| i.0.clone()).collect(),
                    );
                    if let Some(v) = picked.filter(|v| v != value) {
                        out_action(out, action, &v);
                    }
                });
            }
            FormRow::Button {
                name,
                text,
                enabled,
                action,
            } => {
                let r = ui.add_enabled(*enabled, egui::Button::new(text));
                named(ui, reg, name, &r, text);
                if r.clicked() {
                    out_action(out, action, "");
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------
// Modal dialogs
// ---------------------------------------------------------------------------------------

/// A modal dialog over everything (the confirmation to discard changes, the shortcuts):
/// a title, a text, optional `(key, description)` lines in two columns and the buttons
/// `(name, label, action)`, the first one primary. A button sends `dialogo:<action>`; Esc
/// or a click outside sends the last button's action (the "back" one).
#[derive(Clone, Debug, PartialEq)]
pub struct Dialog {
    pub name: String,
    pub title: String,
    pub text: String,
    pub lines: Vec<(String, String)>,
    pub buttons: Vec<(String, String, String)>,
}

/// The dialog of `json` (`{name, title, text, lines: [[k, v]], buttons: [[name, label,
/// action]]}`), or `None` for an empty text or anything else.
pub fn parse_dialog(json: &str) -> Option<Dialog> {
    let v: Value = serde_json::from_str(json).ok()?;
    let pairs = |k: &str| -> Vec<Vec<String>> {
        v[k].as_array()
            .into_iter()
            .flatten()
            .map(|p| {
                p.as_array()
                    .into_iter()
                    .flatten()
                    .map(|x| x.as_str().unwrap_or_default().to_owned())
                    .collect()
            })
            .collect()
    };
    Some(Dialog {
        name: s(&v, "name"),
        title: s(&v, "title"),
        text: s(&v, "text"),
        lines: pairs("lines")
            .into_iter()
            .map(|p| {
                (
                    p.first().cloned().unwrap_or_default(),
                    p.get(1).cloned().unwrap_or_default(),
                )
            })
            .collect(),
        buttons: pairs("buttons")
            .into_iter()
            .filter(|p| p.len() >= 3)
            .map(|p| (p[0].clone(), p[1].clone(), p[2].clone()))
            .collect(),
    })
    .filter(|d| !d.title.is_empty() || !d.text.is_empty())
}

/// Draws `d` as an egui modal; the action of the button pressed (`dialogo:<action>`).
pub fn show_dialog(ctx: &egui::Context, d: &Dialog, reg: &mut Registry) -> Option<String> {
    let mut picked = None;
    let modal = egui::Modal::new(egui::Id::new(("utsim_dialogo", d.name.as_str())))
        .frame(
            egui::Frame::popup(&ctx.global_style())
                .fill(colors::PANEL)
                .inner_margin(egui::Margin::same(18)),
        )
        .show(ctx, |ui| {
            let screen = ctx.content_rect();
            ui.set_max_width((screen.width() - 80.0).clamp(280.0, 720.0));
            ui.set_min_width(320.0);
            ui.spacing_mut().item_spacing.y = 10.0;
            ui.label(
                RichText::new(&d.title)
                    .size(sizes::HEADING)
                    .strong()
                    .color(colors::HEADING),
            );
            if !d.text.is_empty() {
                ui.add(Label::new(RichText::new(&d.text).color(colors::TEXT)).wrap());
            }
            if !d.lines.is_empty() {
                let h = (screen.height() - 220.0).clamp(120.0, 520.0);
                egui::ScrollArea::vertical()
                    .max_height(h)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        egui::Grid::new(("utsim_dialogo_linhas", d.name.as_str()))
                            .num_columns(2)
                            .spacing([18.0, 6.0])
                            .striped(true)
                            .show(ui, |ui| {
                                for (k, v) in &d.lines {
                                    ui.label(RichText::new(k).color(colors::ACCENT));
                                    ui.add(Label::new(RichText::new(v).color(colors::TEXT)).wrap());
                                    ui.end_row();
                                }
                            });
                    });
            }
            ui.add_space(4.0);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // Right to left: the primary (first) button ends at the right.
                for (k, (name, label, action)) in d.buttons.iter().enumerate() {
                    let r = if k == 0 {
                        ui.add(utsim_ui_tema::widgets::primary(label))
                    } else {
                        ui.button(label)
                    };
                    named(ui, reg, name, &r, label);
                    if r.clicked() {
                        picked = Some(format!("dialogo:{action}"));
                    }
                }
            });
        });
    reg.put(&d.name, &modal.response, &d.title);
    if picked.is_none()
        && modal.should_close()
        && let Some((_, _, action)) = d.buttons.last()
    {
        picked = Some(format!("dialogo:{action}"));
    }
    picked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_are_read_from_json() {
        let rows = parse(
            r#"[
                {"kind": "title", "text": "Rede de caminhos"},
                {"kind": "info", "name": "Rede_Resumo", "text": "2 nó(s)"},
                {"kind": "check", "name": "Rede_TrechoMaoDupla", "text": "Mão dupla", "on": true, "action": "mao_dupla"},
                {"kind": "number", "name": "Rede_TrechoCapacidade", "label": "Capacidade", "value": 2, "min": 0, "max": 100, "action": "capacidade"},
                {"kind": "line", "name": "Rede_NoNome", "label": "Nome", "value": "n1", "action": "renomear"},
                {"kind": "choice", "name": "Corredor_Perfil", "label": "Perfil", "items": [["Padrão do modelo", ""], ["Branco", "Branco"]], "value": "", "action": "perfil"},
                {"kind": "button", "name": "Rede_TrechoInverter", "text": "Inverter sentido", "enabled": false, "action": "inverter"},
                {"kind": "nada"}
            ]"#,
        );
        assert_eq!(rows.len(), 7);
        assert_eq!(rows[0], FormRow::Title("Rede de caminhos".into()));
        assert!(matches!(&rows[2], FormRow::Check { on: true, .. }));
        assert!(
            matches!(&rows[3], FormRow::Number { value, max, .. } if *value == 2.0 && *max == 100.0)
        );
        match &rows[5] {
            FormRow::Choice { items, .. } => {
                assert_eq!(items[0], ("Padrão do modelo".into(), String::new()));
            }
            other => panic!("{other:?}"),
        }
        assert!(matches!(&rows[6], FormRow::Button { enabled: false, .. }));
        assert!(parse("não é json").is_empty());
    }

    #[test]
    fn typed_text_survives_until_the_value_changes() {
        let line = |v: &str| {
            vec![FormRow::Line {
                name: "Rede_NoNome".into(),
                label: "Nome".into(),
                value: v.into(),
                action: "renomear".into(),
            }]
        };
        let mut m = FormMemory::default();
        m.sync(&line("n1"));
        m.lines.get_mut("Rede_NoNome").unwrap().1 = "n9".into();
        m.sync(&line("n1"));
        assert_eq!(m.lines["Rede_NoNome"].1, "n9");
        m.sync(&line("n2"));
        assert_eq!(m.lines["Rede_NoNome"], ("n2".into(), "n2".into()));
        assert_eq!(number_text(3.0), "3");
        assert_eq!(number_text(0.25), "0.25");
    }
}
