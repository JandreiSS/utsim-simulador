//! The properties panel of Editar, in egui (pure: no Godot types, unit tested).
//!
//! It draws the schema of [`crate::document::Editor::properties`] directly (no dictionary
//! in between): one row per field, the label on the left and the control on the right in
//! two aligned columns, by collapsible section (a closed section stays closed for every
//! object), problems in red and warnings in yellow (clickable), long text wrapping, and
//! "Aplicar" as the primary button. It changes nothing itself: the edits come out as
//! [`PropAction::Set`] with the same paths and values the Godot panel sent, which the bridge
//! applies through `Editor::set_property` (the `schema.rs` validation, one undo step).
//!
//! Widgets keep the names of the Godot panel (`Prop_<path>` with dots as underscores, and
//! the suffixes `_ligado`, `_familia`, `_texto`, `_ron`, `_fonte`, `_aplicar`, `_busca`,
//! `_miniatura`; `Secao_<título>`, `Problemas_<k>`, `Avisos_<k>`), so the scripted runs
//! find them. Text fields select all their text when they get the focus and commit on
//! Enter or when they lose it (Esc discards), as the Godot fields did.
//!
//! The subpanels of the Rede and Corredor tools, of a selected link and the real-data form
//! stay in Godot until their step (`docs/plano_interface_rust.md`): the main scene shows the
//! Godot panel instead of this one for them.

use std::collections::{HashMap, HashSet};

use egui::text::CCursorRange;
use egui::{
    Align, Button, Color32, ComboBox, Id, Label, Layout, Rect, Response, RichText, ScrollArea,
    Sense, TextEdit, TextWrapMode, TextureId, Ui, UiBuilder, Vec2,
};
use utsim_ui_tema::{colors, sizes, widgets};

use crate::document::Properties;
use crate::egui_widgets::Registry;
use crate::schema::{Field, FieldType, Value};

/// The section of the Qualidade, Segurança and Manutenção tabs.
pub const S_ABAS: &str = "Qualidade, Segurança e Manutenção";
/// Width of the panel, in points.
pub const WIDTH: f32 = 420.0;
/// Width of the label column: a share of the row, between a minimum and a maximum (the
/// value column takes the rest). A label wider than the column wraps to more lines; it is
/// never cut, and its tooltip has the whole text with the help.
const LABEL_SHARE: f32 = 0.4;
const LABEL_MIN: f32 = 140.0;
const LABEL_MAX: f32 = 200.0;

/// The width of the label column for a row `width` points wide.
pub fn label_width(width: f32) -> f32 {
    (width * LABEL_SHARE).clamp(LABEL_MIN, LABEL_MAX)
}
/// Indent per nesting level.
const INDENT: f32 = 14.0;
/// The texture of the asset miniature (a Godot `SubViewport` the bridge registers).
pub const MINIATURE: TextureId = TextureId::User(1);

/// What the panel shows (built by the bridge from the document).
#[derive(Clone, Debug)]
pub struct PropsView {
    pub props: Properties,
    /// The distribution families, in the schema's order (`schema::FAMILIES`).
    pub families: Vec<String>,
    /// Distribution fields of the object that real data can feed (a "Fonte" row).
    pub data_paths: Vec<String>,
    /// A line under the title (several objects selected); the align buttons come with it.
    pub extra: String,
    /// The seated driver of a forklift: -1 none, 0 hidden, 1 shown.
    pub driver: i32,
    /// Whether the miniature texture exists.
    pub miniature: bool,
    /// The fields fed by real data (`model.data`), by path inside the object: the entry of
    /// `dados::data_fields` (config, family, view...).
    pub data: HashMap<String, serde_json::Value>,
    /// The families a field with data can be fixed to.
    pub data_families: Vec<String>,
    /// The folder of the model (relative data files are read from it).
    pub base_dir: Option<std::path::PathBuf>,
    /// The Qualidade, Segurança and Manutenção tabs of the object (`ui_abas`).
    pub abas: Vec<(utsim_edit::abas::Aba, Vec<utsim_edit::abas::Campo>)>,
    /// Who edits the model when it is not us ("Em edição por ..."): the tabs are read only.
    pub somente_leitura: Option<String>,
    /// What the last run says about the object, by tab (`indicadores.rs`).
    pub simulado: Option<crate::ui_abas::Simulado>,
    /// The fatigue curves of a person with a time study (`ui_fadiga`).
    pub fadiga: Vec<crate::ui_fadiga::CurvaFadiga>,
}

/// What the user did in the panel.
#[derive(Clone, Debug, PartialEq)]
pub enum PropAction {
    /// Set `path` of the object to `value`; `end_gesture` closes the undo step (false for
    /// the colour picker while it is dragged).
    Set {
        path: String,
        value: Value,
        end_gesture: bool,
    },
    /// The continuous gesture (colour picker) ended.
    EndGesture,
    /// A problem or warning was clicked.
    Issue(String),
    /// Align or distribute the selection (`alinhar:esquerda`, `distribuir:x`).
    Arrange(String),
    /// Move output `index` by `delta` in the port order.
    Reorder(usize, i32),
    /// Remove the output to this object.
    Disconnect(String),
    /// "Mostrar motorista".
    Driver(bool),
    /// The real-data form of `path` asked `what` (`aplicar`, `analisar`, `para_dist`,
    /// `relatorio`, `escolher_arquivo`) with its choices (the keys the main scene reads).
    Data {
        what: String,
        path: String,
        draft: serde_json::Value,
    },
    /// "Ver as primeiras chegadas" of the object shown: run its source alone.
    ArrivalsPreview,
    /// The miniature should show this asset (the one chosen, or the one under the mouse).
    Miniature(String),
    /// An edit in the Qualidade, Segurança or Manutenção tab.
    Aba(crate::ui_abas::AbaAcao),
}

/// What the panel keeps between frames.
#[derive(Default, Debug)]
pub struct PropsState {
    /// Sections closed by the user (by title).
    pub closed: HashSet<String>,
    /// Distribution fields switched by their "{ }" button away from the default mode (the
    /// form, or the code when [`PropsState::code_default`]), until another object is shown.
    flipped: HashSet<String>,
    /// The distribution fields open as code (RON text): the preference of Configurações.
    pub code_default: bool,
    /// Text being typed, by widget name.
    buffers: HashMap<String, String>,
    asset_filter: String,
    /// A colour is being dragged (one undo step until the button is released).
    gesture: bool,
    miniature_sent: Option<String>,
    /// The object shown last (its text drafts are dropped when it changes).
    object: String,
    /// Distribution fields switched to "Dados reais" and not applied yet.
    data_mode: HashSet<String>,
    /// The choices of each real-data form, and the config of the entry they came from.
    drafts: HashMap<String, DataDraft>,
    drafts_seen: HashMap<String, serde_json::Value>,
    /// Entries whose analysis was asked (once).
    requested: HashSet<String>,
    /// The arrival table and schedule editors (`ui_arrivals`).
    pub(crate) arrivals: crate::ui_arrivals::ArrivalsState,
    /// The Qualidade, Segurança and Manutenção tabs.
    pub abas: crate::ui_abas::AbasState,
}

impl PropsState {
    /// The file chosen in the dialog for the real-data form of `path`.
    pub fn set_data_file(&mut self, path: &str, file: &str) {
        let d = self.drafts.entry(path.to_owned()).or_default();
        d.source = file.to_owned();
        d.origin = "arquivo".into();
        self.buffers
            .remove(&format!("{}_arquivo", widget_name(path)));
    }
}

pub fn widget_name(path: &str) -> String {
    format!("Prop_{}", path.replace('.', "_"))
}

/// Records a widget clipped to what is visible, and scrolls to it when a script asked.
pub(crate) fn named(ui: &Ui, reg: &mut Registry, name: &str, r: &Response, text: &str) {
    if reg.take_reveal(name) {
        r.scroll_to_me(Some(Align::Center));
        ui.ctx().request_repaint();
    }
    let mut info = crate::egui_widgets::WidgetInfo {
        rect: r.rect.intersect(ui.clip_rect()),
        text: text.to_owned(),
        enabled: r.enabled(),
        checked: None,
        choices: Vec::new(),
        count: None,
    };
    if !info.rect.is_positive() {
        info.rect = Rect::NOTHING;
    }
    reg.put_info(name, info);
}

pub(crate) fn named_toggle(
    ui: &Ui,
    reg: &mut Registry,
    name: &str,
    r: &Response,
    text: &str,
    on: bool,
) {
    named(ui, reg, name, r, text);
    if let Some(w) = reg.get(name).cloned() {
        reg.put_info(
            name,
            crate::egui_widgets::WidgetInfo {
                checked: Some(on),
                ..w
            },
        );
    }
}

pub(crate) fn named_list(
    ui: &Ui,
    reg: &mut Registry,
    name: &str,
    r: &Response,
    text: &str,
    choices: Vec<String>,
) {
    named(ui, reg, name, r, text);
    if let Some(w) = reg.get(name).cloned() {
        reg.put_info(name, crate::egui_widgets::WidgetInfo { choices, ..w });
    }
}

/// Draws the panel in `rect` (points) and pushes the user's actions to `out`.
pub fn show(
    ui: &mut Ui,
    rect: Rect,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
) {
    if st.object != v.props.name {
        st.object = v.props.name.clone();
        st.buffers.clear();
        st.flipped.clear();
        st.data_mode.clear();
        st.drafts.clear();
        st.drafts_seen.clear();
        st.requested.clear();
        st.arrivals = Default::default();
        st.miniature_sent = None;
        st.abas.outro_objeto();
    }
    // The end of a colour drag: the button is up again.
    if st.gesture && !ui.input(|i| i.pointer.any_down()) {
        st.gesture = false;
        out.push(PropAction::EndGesture);
    }
    ui.painter().rect_filled(rect, 0.0, colors::PANEL);
    ui.painter().line_segment(
        [rect.left_top(), rect.left_bottom()],
        egui::Stroke::new(1.0, colors::BORDER),
    );
    let panel = ui.allocate_rect(rect, Sense::hover());
    named(ui, reg, "PainelPropriedades", &panel, "");
    let inner = rect.shrink2(Vec2::new(12.0, 10.0));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(inner.expand(2.0));
    ScrollArea::vertical()
        .id_salt("utsim_propriedades")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_width(inner.width() - 12.0);
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
            contents(ui, v, st, reg, out);
        });
}

fn contents(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
) {
    let p = &v.props;
    let title = if p.kind == "run" {
        "Modelo (nada selecionado)".to_owned()
    } else {
        format!("Propriedades: {}", p.name)
    };
    let r = ui.add(
        Label::new(
            RichText::new(&title)
                .size(sizes::HEADING)
                .strong()
                .color(colors::HEADING),
        )
        .wrap(),
    );
    named(ui, reg, "Titulo_Propriedades", &r, &title);
    ui.label(RichText::new(p.kind_label).color(colors::TEXT_WEAK));
    if !v.extra.is_empty() {
        ui.add(Label::new(RichText::new(&v.extra).color(colors::WARNING)).wrap());
        arrange_buttons(ui, reg, out);
    }
    issues(ui, reg, out, "Problemas", &p.problems, colors::ERROR);
    issues(ui, reg, out, "Avisos", &p.warnings, colors::WARNING);
    let mut section = "";
    for f in &p.fields {
        if f.section != section {
            section = f.section;
            section_header(ui, reg, st, section);
        }
        if st.closed.contains(section) {
            continue;
        }
        field(ui, v, st, reg, out, f);
        if f.path == "visual.reset" && v.driver >= 0 {
            let mut on = v.driver == 1;
            let r = ui.checkbox(&mut on, "Mostrar motorista").on_hover_text(
                "Desenha o operador sentado e animado na empilhadeira. Só a aparência: não muda a simulação nem é gravado no modelo.",
            );
            named_toggle(
                ui,
                reg,
                "Prop_visual_motorista",
                &r,
                "Mostrar motorista",
                v.driver == 1,
            );
            if r.changed() {
                out.push(PropAction::Driver(on));
            }
        }
        if f.path == "output" {
            outputs(ui, reg, out, p);
        }
    }
    if p.kind != "run" && !v.fadiga.is_empty() {
        let title = crate::texto::tx("fadiga-secao");
        section_header(ui, reg, st, title);
        if !st.closed.contains(title) && crate::ui_fadiga::show(ui, &v.fadiga, reg) {
            out.push(PropAction::Aba(crate::ui_abas::AbaAcao::AbrirApp(
                "estatistica".into(),
            )));
        }
    }
    if p.kind != "run" && !v.abas.is_empty() {
        section_header(ui, reg, st, S_ABAS);
        if !st.closed.contains(S_ABAS) {
            crate::ui_abas::show(ui, v, st, reg, out);
        }
    }
    ui.add_space(8.0);
}

/// A row of the align buttons: label, kind and `(what, text)` of each button.
type ButtonRow<'a> = (&'a str, &'a str, &'a [(&'a str, &'a str)]);

fn arrange_buttons(ui: &mut Ui, reg: &mut Registry, out: &mut Vec<PropAction>) {
    let rows: [ButtonRow; 3] = [
        (
            "Alinhar",
            "alinhar",
            &[
                ("esquerda", "⇤ esq."),
                ("centro_x", "centro X"),
                ("direita", "dir. ⇥"),
            ],
        ),
        (
            "",
            "alinhar",
            &[
                ("topo", "⤒ topo"),
                ("centro_y", "centro Y"),
                ("base", "base ⤓"),
            ],
        ),
        (
            "Distribuir",
            "distribuir",
            &[("x", "em X ↔"), ("y", "em Y ↕")],
        ),
    ];
    for (label, kind, buttons) in rows {
        ui.horizontal(|ui| {
            ui.add_sized([80.0, 24.0], Label::new(label));
            let w = ((ui.available_width() - 6.0 * (buttons.len() as f32 - 1.0))
                / buttons.len() as f32)
                .max(40.0);
            for (what, text) in buttons {
                let r = ui.add_sized([w, 26.0], Button::new(*text));
                let name = if kind == "distribuir" {
                    format!("Distribuir_{what}")
                } else {
                    format!("Alinhar_{what}")
                };
                named(ui, reg, &name, &r, text);
                if r.clicked() {
                    out.push(PropAction::Arrange(format!("{kind}:{what}")));
                }
            }
        });
    }
}

fn issues(
    ui: &mut Ui,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    title: &str,
    list: &[String],
    color: Color32,
) {
    if list.is_empty() {
        return;
    }
    let head = ui.label(
        RichText::new(format!("{title} ({})", list.len()))
            .color(color)
            .size(sizes::BODY + 1.0)
            .strong(),
    );
    named(ui, reg, title, &head, title);
    for (k, m) in list.iter().enumerate() {
        let r = ui
            .add(
                Label::new(RichText::new(format!("• {m}")).color(color))
                    .wrap()
                    .sense(Sense::click()),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text("Clique para destacar na cena o que este aviso cita");
        named(ui, reg, &format!("{title}_{k}"), &r, m);
        if r.clicked() {
            out.push(PropAction::Issue(m.clone()));
        }
    }
}

/// A section title that opens and closes it (▾ open, ▸ closed).
fn section_header(ui: &mut Ui, reg: &mut Registry, st: &mut PropsState, title: &str) {
    ui.add_space(2.0);
    ui.separator();
    let closed = st.closed.contains(title);
    let text = title.to_owned();
    // A full-width row, left aligned: the triangle (drawn, not a glyph: the UI fonts lack
    // the small ones) and the title.
    let (rect, r) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 26.0), Sense::click());
    if r.hovered() {
        ui.painter().rect_filled(rect, 4.0, colors::HOVER);
    }
    widgets::paint_chevron(
        ui.painter(),
        rect.left_center() + Vec2::new(10.0, 0.0),
        !closed,
        colors::HEADING,
    );
    ui.painter().text(
        rect.left_center() + Vec2::new(24.0, 0.0),
        egui::Align2::LEFT_CENTER,
        title,
        egui::FontId::proportional(sizes::BODY),
        colors::HEADING,
    );
    let r = r
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(format!(
            "{} a seção {title}",
            if closed { "Abre" } else { "Fecha" }
        ));
    named(
        ui,
        reg,
        &format!("Secao_{}", title.replace(' ', "_")),
        &r,
        &text,
    );
    if r.clicked() {
        if closed {
            st.closed.remove(title);
        } else {
            st.closed.insert(title.to_owned());
        }
        ui.ctx().request_repaint();
    }
}

/// One row: the label (wrapping, indented by depth) and `widget` in the right column,
/// which gets the column's width.
fn row<R>(ui: &mut Ui, f: &Field, widget: impl FnOnce(&mut Ui, f32) -> R) -> R {
    row_label(ui, &f.label, &f.help, f.depth, widget)
}

pub(crate) fn row_label<R>(
    ui: &mut Ui,
    label: &str,
    help: &str,
    depth: u8,
    widget: impl FnOnce(&mut Ui, f32) -> R,
) -> R {
    let full = ui.available_width();
    ui.horizontal(|ui| {
        let indent = INDENT * depth as f32;
        if indent > 0.0 {
            ui.add_space(indent);
        }
        let lw = label_width(full) - indent;
        ui.allocate_ui_with_layout(
            Vec2::new(lw, 26.0),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.set_width(lw);
                ui.set_min_height(26.0);
                let tip = if help.is_empty() {
                    label.to_owned()
                } else {
                    format!("{label}\n{help}")
                };
                ui.add(Label::new(RichText::new(label).color(colors::TEXT)).wrap())
                    .on_hover_text(tip);
            },
        );
        let w = ui.available_width().max(60.0);
        widget(ui, w)
    })
    .inner
}

/// A one-line text field with its own draft: selects all on focus, commits on Enter or
/// when the focus leaves (Esc discards). Returns the committed text, if it changed.
#[allow(clippy::too_many_arguments)]
pub(crate) fn line_edit(
    ui: &mut Ui,
    st: &mut PropsState,
    reg: &mut Registry,
    name: &str,
    current: &str,
    width: f32,
    mono: bool,
    hint: &str,
) -> Option<String> {
    let id = Id::new(("utsim_prop", name));
    let mut buf = st
        .buffers
        .get(name)
        .cloned()
        .unwrap_or_else(|| current.to_owned());
    let mut edit = TextEdit::singleline(&mut buf)
        .id(id)
        .desired_width(width)
        .hint_text(hint);
    if mono {
        edit = edit.font(egui::TextStyle::Monospace);
    }
    let mut output = edit.show(ui);
    let r = output.response.response.clone();
    if r.gained_focus() || (r.clicked() && buf == current) {
        output
            .state
            .cursor
            .set_char_range(Some(CCursorRange::select_all(&output.galley)));
        output.state.store(ui.ctx(), id);
    }
    named(ui, reg, name, &r, &buf);
    let mut commit = None;
    if r.lost_focus() {
        let cancelled = ui.input(|i| i.key_pressed(egui::Key::Escape));
        if !cancelled && buf != current {
            commit = Some(buf.clone());
        }
        st.buffers.remove(name);
    } else if r.has_focus() {
        st.buffers.insert(name.to_owned(), buf);
    } else {
        st.buffers.remove(name);
    }
    commit
}

/// A number with its unit: a text field that sends the number typed.
#[allow(clippy::too_many_arguments)]
fn number(
    ui: &mut Ui,
    st: &mut PropsState,
    reg: &mut Registry,
    name: &str,
    value: &Value,
    integer: bool,
    unit: &str,
    width: f32,
) -> Option<Value> {
    let current = match value {
        Value::Int(i) => i.to_string(),
        Value::Float(x) => num_text(*x),
        other => other.as_f64().map(num_text).unwrap_or_default(),
    };
    let unit_w = if unit.is_empty() { 0.0 } else { 44.0 };
    let typed = line_edit(
        ui,
        st,
        reg,
        name,
        &current,
        (width - unit_w).max(50.0),
        false,
        "",
    );
    if !unit.is_empty() {
        ui.label(RichText::new(unit).color(colors::TEXT_WEAK));
    }
    let typed = typed?;
    let t = typed.trim().replace(',', ".");
    Some(if integer {
        t.parse::<i64>()
            .map(Value::Int)
            .unwrap_or(Value::Text(typed))
    } else {
        t.parse::<f64>()
            .map(Value::Float)
            .unwrap_or(Value::Text(typed))
    })
}

/// A number as the Godot spin box showed it (no trailing zeros).
pub fn num_text(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        let s = format!("{x:.6}");
        s.trim_end_matches('0').trim_end_matches('.').to_owned()
    }
}

/// A list: `choices` are `(value, label, help)`; returns the value picked (by the user or
/// by a scripted `escolher` with the label).
pub(crate) fn combo(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    selected: &str,
    choices: &[(String, String, String)],
    width: f32,
    help: &str,
) -> Option<String> {
    let label_of = |v: &str| {
        choices
            .iter()
            .find(|c| c.0 == v)
            .map(|c| c.1.clone())
            .unwrap_or_default()
    };
    let mut pick = reg
        .take_request(name)
        .and_then(|item| choices.iter().find(|c| c.1 == item).map(|c| c.0.clone()));
    let shown = label_of(selected);
    let r = ComboBox::from_id_salt(("utsim_prop", name))
        .selected_text(&shown)
        .width(width)
        .wrap_mode(TextWrapMode::Truncate)
        .show_ui(ui, |ui| {
            for (value, label, tip) in choices {
                let item = ui.selectable_label(value == selected, label);
                let item = if tip.is_empty() {
                    item
                } else {
                    item.on_hover_text(tip)
                };
                if item.clicked() {
                    pick = Some(value.clone());
                }
            }
        });
    let tip = if help.is_empty() {
        shown.clone()
    } else {
        format!("{shown}\n{help}")
    };
    let resp = r.response.on_hover_text(tip);
    named_list(
        ui,
        reg,
        name,
        &resp,
        &shown,
        choices.iter().map(|c| c.1.clone()).collect(),
    );
    pick.filter(|p| p != selected)
}

/// The on/off switch of an optional field (`path.on`).
pub(crate) fn toggle(ui: &mut Ui, reg: &mut Registry, out: &mut Vec<PropAction>, f: &Field) {
    let mut on = f.enabled;
    let r = ui
        .checkbox(&mut on, "")
        .on_hover_text(format!("Liga ou desliga {}", f.label.to_lowercase()));
    named_toggle(
        ui,
        reg,
        &format!("{}_ligado", widget_name(&f.path)),
        &r,
        "",
        f.enabled,
    );
    if r.changed() {
        out.push(set(&format!("{}.on", f.path), Value::Bool(on)));
    }
}

pub(crate) fn set(path: &str, value: Value) -> PropAction {
    PropAction::Set {
        path: path.to_owned(),
        value,
        end_gesture: true,
    }
}

fn field(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    f: &Field,
) {
    let name = widget_name(&f.path);
    let path = f.path.as_str();
    match f.ty {
        FieldType::Group => {
            row(ui, f, |ui, _| {
                if f.optional {
                    toggle(ui, reg, out, f);
                }
            });
        }
        FieldType::Info => {
            row(ui, f, |ui, _| {
                let text = value_text(&f.value);
                let r = ui.add(Label::new(RichText::new(&text).color(colors::TEXT_WEAK)).wrap());
                named(ui, reg, &name, &r, &text);
            });
        }
        FieldType::Text => {
            let current = value_text(&f.value);
            if let Some(t) = row(ui, f, |ui, w| {
                line_edit(ui, st, reg, &name, &current, w, false, "")
            }) {
                out.push(set(path, Value::Text(t)));
            }
        }
        FieldType::Script => {
            row(ui, f, |ui, _| {
                if f.optional {
                    toggle(ui, reg, out, f);
                }
            });
            if matches!(f.value, Value::Null) {
                return;
            }
            let current = value_text(&f.value);
            let id = Id::new(("utsim_prop", name.as_str()));
            let mut buf = st
                .buffers
                .get(&name)
                .cloned()
                .unwrap_or_else(|| current.clone());
            let r = ui.add(
                TextEdit::multiline(&mut buf)
                    .id(id)
                    .desired_width(ui.available_width())
                    .desired_rows(3)
                    .font(if path == "description" {
                        egui::TextStyle::Body
                    } else {
                        egui::TextStyle::Monospace
                    }),
            );
            named(ui, reg, &name, &r, &buf);
            // The draft stays until applied (or another object is shown).
            if buf != current {
                st.buffers.insert(name.clone(), buf.clone());
            } else {
                st.buffers.remove(&name);
            }
            let label = if path == "description" {
                "Aplicar"
            } else {
                "Aplicar script"
            };
            let r = ui.add_enabled(buf != current, widgets::primary(label));
            named(ui, reg, &format!("{name}_aplicar"), &r, label);
            if r.clicked() {
                st.buffers.remove(&name);
                out.push(set(path, Value::Text(buf)));
            }
        }
        FieldType::Float | FieldType::Int => {
            let integer = f.ty == FieldType::Int;
            let picked = row(ui, f, |ui, w| {
                let mut w = w;
                if f.optional {
                    toggle(ui, reg, out, f);
                    w -= 30.0;
                }
                if matches!(f.value, Value::Null) {
                    return None;
                }
                number(ui, st, reg, &name, &f.value, integer, f.unit, w)
            });
            if let Some(value) = picked {
                out.push(set(path, value));
            }
        }
        FieldType::Bool => {
            let on = f.value.as_bool().unwrap_or(false);
            row(ui, f, |ui, _| {
                let mut now = on;
                let r = ui.checkbox(&mut now, "").on_hover_text(&f.help);
                named_toggle(ui, reg, &name, &r, "", on);
                if r.changed() {
                    out.push(set(path, Value::Bool(now)));
                }
            });
        }
        FieldType::Choice => {
            let selected = value_text(&f.value);
            let choices: Vec<(String, String, String)> = f
                .choices
                .iter()
                .map(|c| (c.value.clone(), c.label.clone(), c.help.clone()))
                .collect();
            if let Some(v) = row(ui, f, |ui, w| {
                combo(ui, reg, &name, &selected, &choices, w - 8.0, &f.help)
            }) {
                out.push(set(path, Value::Text(v)));
            }
        }
        // The arrival table and the schedule have their own editors (with "{ }" for the
        // plain RON line).
        FieldType::Ron if crate::ui_arrivals::handles(path) => {
            crate::ui_arrivals::field(ui, v, st, reg, out, f);
        }
        FieldType::Ron => {
            let picked = row(ui, f, |ui, w| {
                let mut w = w;
                if f.optional {
                    toggle(ui, reg, out, f);
                    w -= 30.0;
                }
                if matches!(f.value, Value::Null) {
                    return None;
                }
                line_edit(ui, st, reg, &name, &value_text(&f.value), w, true, "")
            });
            if let Some(t) = picked {
                out.push(set(path, Value::Text(t)));
            }
        }
        FieldType::Set => {
            let members: Vec<String> = match &f.value {
                Value::List(l) => l.clone(),
                _ => Vec::new(),
            };
            row(ui, f, |ui, _| {
                ui.vertical(|ui| {
                    if f.choices.is_empty() {
                        ui.label(
                            RichText::new("Nenhum transportador no modelo.")
                                .color(colors::TEXT_WEAK),
                        );
                    }
                    for c in &f.choices {
                        let on = members.contains(&c.value);
                        let mut now = on;
                        let r = ui.checkbox(&mut now, &c.label);
                        named_toggle(
                            ui,
                            reg,
                            &widget_name(&format!("{path}.{}", c.value)),
                            &r,
                            &c.label,
                            on,
                        );
                        if r.changed() {
                            out.push(set(&format!("{path}.{}", c.value), Value::Bool(now)));
                        }
                    }
                });
            });
        }
        FieldType::Dist => dist(ui, v, st, reg, out, f),
        FieldType::Color => {
            row(ui, f, |ui, _| {
                toggle(ui, reg, out, f);
                if matches!(f.value, Value::Null) {
                    return;
                }
                let hex = value_text(&f.value);
                let mut rgb = parse_hex(&hex).unwrap_or([255, 255, 255]);
                let r = ui.color_edit_button_srgb(&mut rgb);
                named(ui, reg, &name, &r, &hex);
                ui.label(RichText::new(&hex).color(colors::TEXT_WEAK));
                if r.changed() {
                    st.gesture = true;
                    out.push(PropAction::Set {
                        path: path.to_owned(),
                        value: Value::Text(format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])),
                        end_gesture: false,
                    });
                }
            });
        }
        FieldType::Asset => asset(ui, v, st, reg, out, f),
        FieldType::Action => {
            let enabled = f.value.as_bool().unwrap_or(false);
            let r = ui
                .add_enabled(enabled, Button::new(&f.label))
                .on_hover_text(&f.help)
                .on_disabled_hover_text(&f.help);
            named(ui, reg, &name, &r, &f.label);
            if r.clicked() {
                out.push(set(path, Value::Null));
            }
        }
    }
}

/// A distribution: family, parameters and the mean, or its RON text ("editar como
/// texto"), which is also the only way for tables (Discreta, Empírica).
fn dist(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    f: &Field,
) {
    let name = widget_name(&f.path);
    let path = f.path.as_str();
    let Some(view) = &f.dist else {
        row(ui, f, |ui, _| {
            if f.optional {
                toggle(ui, reg, out, f);
            }
        });
        return;
    };
    let entry = v.data.get(path);
    let using_data = entry.is_some() || st.data_mode.contains(path);
    if using_data {
        // The field comes from data: the seal instead of the family; the form below.
        row(ui, f, |ui, _| {
            if f.optional {
                toggle(ui, reg, out, f);
            }
            if let Some(e) = entry {
                let label = e["label"].as_str().unwrap_or_default();
                let r = ui
                    .add(Label::new(
                        RichText::new(" dados ")
                            .small()
                            .color(colors::TEXT_STRONG)
                            .background_color(colors::BADGE),
                    ))
                    .on_hover_text(format!("Este campo vem de dados reais: {label}"));
                named(ui, reg, &format!("{name}_selo"), &r, "dados");
                ui.add(
                    Label::new(RichText::new(label).small().color(colors::TEXT_WEAK)).truncate(),
                );
            }
        });
    }
    // The default mode, unless its "{ }" switched it; a table (no parameters) is text only.
    let text_on = (st.code_default != st.flipped.contains(path)) || view.params.is_empty();
    let families: Vec<(String, String, String)> = v
        .families
        .iter()
        .enumerate()
        .map(|(k, n)| (k.to_string(), n.clone(), String::new()))
        .collect();
    if !using_data {
        row(ui, f, |ui, w| {
            let mut w = w;
            if f.optional {
                toggle(ui, reg, out, f);
                w -= 30.0;
            }
            let picked = combo(
                ui,
                reg,
                &format!("{name}_familia"),
                &view.family.to_string(),
                &families,
                (w - 52.0).max(60.0),
                "",
            );
            if let Some(k) = picked.and_then(|k| k.parse::<i64>().ok()) {
                out.push(set(&format!("{path}.family"), Value::Int(k)));
            }
            let r = ui
                .add(Button::new("{ }").selected(text_on))
                .on_hover_text(if text_on {
                    crate::texto::tx("props-editar_como_formulario")
                } else {
                    crate::texto::tx("props-editar_como_texto")
                });
            named_toggle(ui, reg, &format!("{name}_texto"), &r, "{ }", text_on);
            if r.clicked() && !view.params.is_empty() {
                if !st.flipped.remove(path) {
                    st.flipped.insert(path.to_owned());
                }
                ui.ctx().request_repaint();
            }
        });
    }
    let depth = f.depth + 1;
    if v.data_paths.iter().any(|p| p == path) {
        let choices = [
            ("dist".to_owned(), "Distribuição".to_owned(), String::new()),
            ("dados".to_owned(), "Dados reais".to_owned(), String::new()),
        ];
        let picked = row_label(ui, "Fonte", "", depth, |ui, w| {
            combo(
                ui,
                reg,
                &format!("{name}_fonte"),
                if using_data { "dados" } else { "dist" },
                &choices,
                w - 8.0,
                "De onde vem a distribuição deste campo: parâmetros informados ou valores reais, ajustados automaticamente",
            )
        });
        match picked.as_deref() {
            Some("dados") => {
                st.data_mode.insert(path.to_owned());
                ui.ctx().request_repaint();
            }
            Some("dist") if entry.is_some() => {
                st.data_mode.remove(path);
                out.push(PropAction::Data {
                    what: "para_dist".into(),
                    path: path.to_owned(),
                    draft: serde_json::Value::Null,
                });
            }
            Some("dist") => {
                st.data_mode.remove(path);
                ui.ctx().request_repaint();
            }
            _ => {}
        }
    }
    if using_data {
        data_form(ui, v, st, reg, out, path, entry, depth);
        return;
    }
    if text_on {
        let picked = ui
            .horizontal(|ui| {
                ui.add_space(INDENT * depth as f32);
                let w = ui.available_width();
                line_edit(ui, st, reg, &format!("{name}_ron"), &view.text, w, true, "")
            })
            .inner;
        if let Some(t) = picked {
            out.push(set(path, Value::Text(t)));
        }
    } else {
        for p in &view.params {
            let pname = widget_name(&format!("{path}.{}", p.key));
            let value = if p.integer {
                Value::Int(p.value.round() as i64)
            } else {
                Value::Float(p.value)
            };
            let picked = row_label(ui, p.label, "", depth, |ui, w| {
                number(ui, st, reg, &pname, &value, p.integer, "", w)
            });
            if let Some(value) = picked {
                out.push(set(&format!("{path}.{}", p.key), value));
            }
        }
    }
    ui.horizontal(|ui| {
        ui.add_space(INDENT * depth as f32);
        ui.label(
            RichText::new(format!("média {}", num_text(view.mean)))
                .small()
                .color(colors::TEXT_WEAK),
        );
    });
}

// ---------------------------------------------------------------------------------------
// Real data in a distribution field ("Fonte: Dados reais")
// ---------------------------------------------------------------------------------------

/// The criteria that rank the families accepted by the tests: key and label.
const CRITERIA: [(&str, &str); 4] = [
    ("aic", "AIC"),
    ("bic", "BIC"),
    ("ks", "Kolmogorov-Smirnov"),
    ("ad", "Anderson-Darling"),
];

/// The choices of the real-data form of one field, kept while the panel is redrawn.
#[derive(Clone, Debug, PartialEq)]
pub struct DataDraft {
    /// `arquivo` or `valores`.
    pub origin: String,
    pub source: String,
    pub column: String,
    pub text: String,
    pub timestamps: bool,
    pub choose: String,
    pub criterion: String,
}

impl Default for DataDraft {
    fn default() -> Self {
        DataDraft {
            origin: "arquivo".into(),
            source: String::new(),
            column: String::new(),
            text: String::new(),
            timestamps: false,
            choose: "Best".into(),
            criterion: "aic".into(),
        }
    }
}

impl DataDraft {
    /// The draft of an applied entry (its `config`).
    pub fn of_config(c: &serde_json::Value) -> DataDraft {
        let values: Vec<String> = c["values"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|v| match v.as_f64() {
                Some(x) => num_text(x),
                None => v.to_string(),
            })
            .collect();
        let s = |k: &str| c[k].as_str().unwrap_or_default().to_owned();
        DataDraft {
            origin: if values.is_empty() {
                "arquivo"
            } else {
                "valores"
            }
            .into(),
            source: s("source"),
            column: s("column"),
            text: values.join("\n"),
            timestamps: c["timestamps"].as_bool().unwrap_or(false),
            choose: c["choose"].as_str().unwrap_or("Best").to_owned(),
            criterion: c["criterion"].as_str().unwrap_or("aic").to_owned(),
        }
    }

    /// As the main scene reads it (`_data_config`): the keys of the old Godot form.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "origem": self.origin, "source": self.source, "column": self.column,
            "text": self.text, "timestamps": self.timestamps, "choose": self.choose,
            "criterion": self.criterion,
        })
    }
}

/// The real-data form of the distribution field `path` (under its "Fonte" row) and, once
/// applied, the result: family, parameters, n, the histogram with the curve, the warnings
/// and the candidates to swap with one click.
#[allow(clippy::too_many_arguments)]
fn data_form(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    path: &str,
    entry: Option<&serde_json::Value>,
    depth: u8,
) {
    let name = widget_name(path);
    let applied = entry.is_some();
    // The draft follows the applied entry when it changes (undo, redo, another fit).
    if let Some(e) = entry {
        let config = &e["config"];
        if st.drafts_seen.get(path) != Some(config) {
            st.drafts_seen.insert(path.to_owned(), config.clone());
            st.drafts
                .insert(path.to_owned(), DataDraft::of_config(config));
            st.requested.remove(path);
        }
    }
    let mut draft = st.drafts.get(path).cloned().unwrap_or_default();
    let send = |out: &mut Vec<PropAction>, what: &str, d: &DataDraft| {
        out.push(PropAction::Data {
            what: what.to_owned(),
            path: path.to_owned(),
            draft: d.to_json(),
        });
    };
    let origins = [
        (
            "arquivo".to_owned(),
            "Arquivo CSV".to_owned(),
            String::new(),
        ),
        (
            "valores".to_owned(),
            "Valores colados".to_owned(),
            String::new(),
        ),
    ];
    ui.horizontal(|ui| {
        ui.add_space(INDENT * depth as f32);
        let r = ui.add(widgets::primary("Valores reais…")).on_hover_text(
            "Abre a janela grande para digitar, colar ou importar os valores e comparar todas as distribuições de uma vez",
        );
        named(ui, reg, &format!("{name}_janela"), &r, "Valores reais…");
        if r.clicked() {
            send(out, "janela", &draft);
        }
    });
    if let Some(o) = row_label(ui, "Origem", "", depth, |ui, w| {
        combo(
            ui,
            reg,
            &format!("{name}_origem"),
            &draft.origin,
            &origins,
            w - 8.0,
            "",
        )
    }) {
        draft.origin = o;
    }
    if draft.origin == "arquivo" {
        let typed = row_label(ui, "Arquivo", "", depth, |ui, w| {
            let t = line_edit(
                ui,
                st,
                reg,
                &format!("{name}_arquivo"),
                &draft.source,
                (w - 96.0).max(60.0),
                false,
                "arquivo .csv ou .txt",
            );
            let r = ui
                .button("Escolher...")
                .on_hover_text("Escolhe o arquivo de dados (CSV ou TXT)");
            named(ui, reg, &format!("{name}_escolher"), &r, "Escolher...");
            (t, r.clicked())
        });
        if let Some(t) = typed.0 {
            draft.source = t.trim().to_owned();
        }
        if typed.1 {
            send(out, "escolher_arquivo", &draft);
        }
        let columns = if draft.source.trim().is_empty() {
            Vec::new()
        } else {
            let p = std::path::PathBuf::from(draft.source.trim());
            let p = match &v.base_dir {
                Some(b) if p.is_relative() => b.join(p),
                _ => p,
            };
            crate::dados::columns_of(&p).unwrap_or_default()
        };
        let mut items = vec![(String::new(), "(automática)".to_owned(), String::new())];
        items.extend(
            columns
                .iter()
                .map(|c| (c.clone(), c.clone(), String::new())),
        );
        if let Some(c) = row_label(ui, "Coluna", "", depth, |ui, w| {
            ui.add_enabled_ui(!columns.is_empty(), |ui| {
                combo(
                    ui,
                    reg,
                    &format!("{name}_coluna"),
                    &draft.column,
                    &items,
                    w - 8.0,
                    "Coluna do arquivo com os valores",
                )
            })
            .inner
        }) {
            draft.column = c;
        }
    } else {
        ui.horizontal(|ui| {
            ui.add_space(INDENT * depth as f32);
            let r = ui.add(
                TextEdit::multiline(&mut draft.text)
                    .id(Id::new(("utsim_prop", format!("{name}_valores"))))
                    .desired_rows(3)
                    .desired_width(f32::INFINITY)
                    .hint_text("um valor por linha (ou separados por ; ou espaço)"),
            );
            named(ui, reg, &format!("{name}_valores"), &r, &draft.text);
        });
    }
    ui.horizontal(|ui| {
        ui.add_space(INDENT * depth as f32);
        let r = ui
            .checkbox(
                &mut draft.timestamps,
                "São instantes (usar os intervalos entre eles)",
            )
            .on_hover_text("Marque quando os valores são instantes crescentes (como horas de chegada): o ajuste usa as diferenças entre eles. Desmarcado, os valores são durações.");
        named_toggle(
            ui,
            reg,
            &format!("{name}_instantes"),
            &r,
            "instantes",
            draft.timestamps,
        );
    });
    let mut choices = vec![
        (
            "Best".to_owned(),
            "Melhor automática".to_owned(),
            String::new(),
        ),
        (
            "Empirical".to_owned(),
            "Empírica (os próprios valores)".to_owned(),
            String::new(),
        ),
    ];
    choices.extend(
        v.data_families
            .iter()
            .map(|f| (f.clone(), format!("Só {f}"), String::new())),
    );
    if let Some(c) = row_label(ui, "Escolha", "", depth, |ui, w| {
        combo(
            ui,
            reg,
            &format!("{name}_escolha"),
            &draft.choose,
            &choices,
            w - 8.0,
            "",
        )
    }) {
        draft.choose = c;
    }
    let criteria: Vec<(String, String, String)> = CRITERIA
        .iter()
        .map(|(k, l)| (k.to_string(), l.to_string(), String::new()))
        .collect();
    if let Some(c) = row_label(ui, "Critério", "", depth, |ui, w| {
        combo(
            ui,
            reg,
            &format!("{name}_criterio"),
            &draft.criterion,
            &criteria,
            w - 8.0,
            "O que ordena as famílias aceitas pelos testes",
        )
    }) {
        draft.criterion = c;
    }
    ui.horizontal(|ui| {
        ui.add_space(INDENT * depth as f32);
        let label = if applied {
            "Reajustar com estas opções"
        } else {
            "Aplicar"
        };
        let r = ui.add(widgets::primary(label)).on_hover_text(
            "Ajusta a distribuição aos dados (em segundo plano) e passa a usá-la neste campo",
        );
        named(ui, reg, &format!("{name}_aplicar"), &r, label);
        if r.clicked() {
            send(out, "aplicar", &draft);
        }
    });
    st.drafts.insert(path.to_owned(), draft.clone());
    let Some(e) = entry else {
        return;
    };
    // The result of the applied entry.
    ui.separator();
    let note = |ui: &mut Ui, text: &str, color: Color32| {
        ui.horizontal(|ui| {
            ui.add_space(INDENT * depth as f32);
            ui.add(Label::new(RichText::new(text).color(color)).wrap())
        })
        .inner
    };
    let err = e["error"].as_str().unwrap_or_default();
    if !err.is_empty() {
        note(
            ui,
            &format!("Não foi possível ler os dados: {err}"),
            colors::ERROR,
        );
    }
    if e["stale"].as_bool() == Some(true) {
        note(
            ui,
            "Os dados mudaram desde o último ajuste.",
            colors::WARNING,
        );
        ui.horizontal(|ui| {
            ui.add_space(INDENT * depth as f32);
            let r = ui.button("Reajustar aos dados atuais");
            named(
                ui,
                reg,
                &format!("{name}_reajustar"),
                &r,
                "Reajustar aos dados atuais",
            );
            if r.clicked() {
                send(out, "aplicar", &draft);
            }
        });
    } else if let Some(c) = e["changed_on_load"].as_str().filter(|c| !c.is_empty()) {
        note(
            ui,
            &format!("{c}. O campo foi reajustado; salve para registrar."),
            colors::WARNING,
        );
    }
    let s = |k: &str| match &e[k] {
        serde_json::Value::String(x) => x.clone(),
        serde_json::Value::Null => "?".into(),
        other => other.to_string(),
    };
    let res = format!("{} · n = {} · {}", s("family"), s("n"), s("label"));
    let r = note(ui, &res, colors::TEXT);
    named(ui, reg, &format!("{name}_resultado"), &r, &res);
    let view = &e["view"];
    if !view.is_object() {
        if err.is_empty() {
            note(ui, "Analisando os dados...", colors::TEXT_WEAK);
            if st.requested.insert(path.to_owned()) {
                send(out, "analisar", &draft);
            }
        }
        return;
    }
    let params: Vec<String> = view["params"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|p| {
            format!(
                "{} = {}",
                p["label"].as_str().unwrap_or_default(),
                num_text(p["value"].as_f64().unwrap_or(f64::NAN))
            )
        })
        .collect();
    if !params.is_empty() {
        let t = params.join("   ");
        let r = note(ui, &t, colors::TEXT_WEAK);
        named(ui, reg, &format!("{name}_parametros"), &r, &t);
    }
    ui.horizontal(|ui| {
        ui.add_space(INDENT * depth as f32);
        mini_histogram(ui, reg, &format!("{name}_grafico"), &view["chart"]);
    });
    for w in view["warnings"].as_array().into_iter().flatten() {
        note(
            ui,
            &format!("• {}", w.as_str().unwrap_or_default()),
            colors::WARNING,
        );
    }
    let rank = view["ranking"].as_array().cloned().unwrap_or_default();
    if !rank.is_empty() {
        note(
            ui,
            &format!(
                "Candidatas ({})",
                view["criterion"].as_str().unwrap_or_default()
            ),
            colors::HEADING,
        );
    }
    for (k, r) in rank.iter().enumerate() {
        let fam = r["family"].as_str().unwrap_or_default().to_owned();
        let text = format!(
            "{}. {}   {}   {}",
            k + 1,
            fam,
            num_text(r["criterion"].as_f64().unwrap_or(f64::NAN)),
            r["status"].as_str().unwrap_or_default()
        );
        let tip = format!(
            "{}\nAIC {}   BIC {}\nUsa esta família neste campo",
            r["params"].as_str().unwrap_or_default(),
            num_text(r["aic"].as_f64().unwrap_or(f64::NAN)),
            num_text(r["bic"].as_f64().unwrap_or(f64::NAN))
        );
        ui.horizontal(|ui| {
            ui.add_space(INDENT * depth as f32);
            let chosen = r["chosen"].as_bool() == Some(true);
            let b = ui
                .add_enabled(!chosen, Button::new(&text).truncate())
                .on_hover_text(tip);
            named(ui, reg, &format!("{name}_cand_{k}"), &b, &text);
            if b.clicked() {
                let mut d = draft.clone();
                d.choose = fam.clone();
                send(out, "aplicar", &d);
            }
        });
    }
    if view["empirical"].as_bool() != Some(true) {
        ui.horizontal(|ui| {
            ui.add_space(INDENT * depth as f32);
            let b = ui.button("Usar a empírica (os próprios valores)");
            named(ui, reg, &format!("{name}_cand_empirica"), &b, "empírica");
            if b.clicked() {
                let mut d = draft.clone();
                d.choose = "Empirical".into();
                send(out, "aplicar", &d);
            }
        });
    }
    ui.horizontal(|ui| {
        ui.add_space(INDENT * depth as f32);
        let b = ui.button("Relatório").on_hover_text(
            "Grava o relatório completo do ajuste (HTML) numa pasta e abre no navegador",
        );
        named(ui, reg, &format!("{name}_relatorio"), &b, "Relatório");
        if b.clicked() {
            send(out, "relatorio", &draft);
        }
    });
}

/// The small histogram of the data with the curve of the distribution chosen. It records
/// `barras=<n> curva=<m>` for the scripted checks.
fn mini_histogram(ui: &mut Ui, reg: &mut Registry, name: &str, chart: &serde_json::Value) {
    let width = ui.available_width().max(120.0);
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 110.0), Sense::hover());
    let resp = resp.on_hover_text("Histograma dos dados e a curva da distribuição escolhida");
    let p = ui.painter();
    let area = Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.max.y - 16.0));
    p.rect_filled(area, 3.0, colors::FIELD);
    let density: Vec<f64> = chart["density"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|d| d.as_f64())
        .collect();
    let curve: Vec<[f64; 2]> = chart["curve"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|q| Some([q[0].as_f64()?, q[1].as_f64()?]))
        .collect();
    let top = density
        .iter()
        .copied()
        .chain(curve.iter().map(|q| q[1]))
        .fold(0.0, f64::max)
        * 1.08;
    if !density.is_empty() && top > 0.0 {
        let w = area.width() / density.len() as f32;
        for (k, d) in density.iter().enumerate() {
            let h = area.height() * (*d / top) as f32;
            p.rect_filled(
                Rect::from_min_max(
                    egui::pos2(area.min.x + k as f32 * w + 1.0, area.max.y - h),
                    egui::pos2(area.min.x + (k + 1) as f32 * w - 1.0, area.max.y),
                ),
                0.0,
                colors::ACCENT.gamma_multiply(0.55),
            );
        }
        let lo = chart["lo"].as_f64().unwrap_or(0.0);
        let hi = chart["hi"].as_f64().unwrap_or(1.0);
        let span = (hi - lo).max(1e-12);
        if curve.len() > 1 {
            let pts: Vec<egui::Pos2> = curve
                .iter()
                .map(|q| {
                    egui::pos2(
                        area.min.x + area.width() * ((q[0] - lo) / span) as f32,
                        area.max.y - area.height() * ((q[1] / top).min(1.0)) as f32,
                    )
                })
                .collect();
            p.add(egui::Shape::line(
                pts,
                egui::Stroke::new(2.0, colors::WARNING),
            ));
        }
        let font = egui::FontId::proportional(sizes::SMALL);
        p.text(
            egui::pos2(rect.min.x + 2.0, rect.max.y - 2.0),
            egui::Align2::LEFT_BOTTOM,
            num_text(lo),
            font.clone(),
            colors::TEXT_WEAK,
        );
        p.text(
            egui::pos2(rect.max.x - 2.0, rect.max.y - 2.0),
            egui::Align2::RIGHT_BOTTOM,
            num_text(hi),
            font,
            colors::TEXT_WEAK,
        );
    }
    let mut info = crate::egui_widgets::WidgetInfo {
        rect: resp.rect.intersect(ui.clip_rect()),
        text: format!("barras={} curva={}", density.len(), curve.len()),
        enabled: true,
        checked: None,
        choices: Vec::new(),
        count: Some(density.len()),
    };
    if !info.rect.is_positive() {
        info.rect = Rect::NOTHING;
    }
    reg.put_info(name, info);
}

/// The asset of the look: the list (library and catalogue, in groups, with the credits as
/// tooltips), a search box and the miniature, which follows the entry under the mouse while
/// the list is open.
fn asset(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    f: &Field,
) {
    let name = widget_name(&f.path);
    let current = value_text(&f.value);
    let wanted = st.asset_filter.trim().to_lowercase();
    let shown: Vec<&crate::schema::Choice> = f
        .choices
        .iter()
        .filter(|c| {
            wanted.is_empty() || c.value == current || c.label.to_lowercase().contains(&wanted)
        })
        .collect();
    let mut hover: Option<String> = None;
    let mut pick = reg.take_request(&name).and_then(|item| {
        shown
            .iter()
            .find(|c| c.label == item)
            .map(|c| c.value.clone())
    });
    let label_of = |val: &str| {
        f.choices
            .iter()
            .find(|c| c.value == val)
            .map(|c| c.label.clone())
            .unwrap_or_default()
    };
    let selected_label = label_of(&current);
    row(ui, f, |ui, w| {
        let r = ComboBox::from_id_salt(("utsim_prop", name.as_str()))
            .selected_text(&selected_label)
            .width(w - 8.0)
            .wrap_mode(TextWrapMode::Truncate)
            .height(360.0)
            .show_ui(ui, |ui| {
                let mut group = "";
                for c in &shown {
                    if c.group != group {
                        group = &c.group;
                        ui.label(
                            RichText::new(group)
                                .small()
                                .strong()
                                .color(colors::TEXT_WEAK),
                        );
                    }
                    let item = ui.selectable_label(c.value == current, &c.label);
                    if item.hovered() {
                        hover = Some(c.value.clone());
                    }
                    let item = if c.help.is_empty() {
                        item
                    } else {
                        item.on_hover_text(&c.help)
                    };
                    if item.clicked() {
                        pick = Some(c.value.clone());
                    }
                }
            });
        named_list(
            ui,
            reg,
            &name,
            &r.response,
            &selected_label,
            shown.iter().map(|c| c.label.clone()).collect(),
        );
    });
    if let Some(p) = pick.filter(|p| *p != current) {
        out.push(set(&f.path, Value::Text(p)));
    }
    let typed = ui
        .horizontal(|ui| {
            ui.add_space(label_width(ui.available_width()) + 6.0);
            let w = ui.available_width();
            let filter = st.asset_filter.clone();
            line_edit(
                ui,
                st,
                reg,
                &format!("{name}_busca"),
                &filter,
                w,
                false,
                "Buscar ativo",
            )
        })
        .inner;
    if let Some(t) = typed {
        st.asset_filter = t;
        ui.ctx().request_repaint();
    }
    if !f.help.is_empty() {
        ui.add(Label::new(RichText::new(&f.help).small().color(colors::TEXT_WEAK)).wrap());
    }
    let size = Vec2::new(ui.available_width(), 150.0);
    let r = if v.miniature {
        ui.add(egui::Image::new((MINIATURE, size)).sense(Sense::hover()))
    } else {
        let (rect, r) = ui.allocate_exact_size(size, Sense::hover());
        ui.painter().rect_filled(rect, 4.0, colors::FIELD);
        r
    };
    let r = r.on_hover_text(
        "Miniatura do objeto com o ativo escolhido (ou o que está sob o mouse na lista)",
    );
    named(ui, reg, &format!("{name}_miniatura"), &r, &current);
    let want = hover.unwrap_or(current);
    if st.miniature_sent.as_deref() != Some(want.as_str()) {
        st.miniature_sent = Some(want.clone());
        out.push(PropAction::Miniature(want));
    }
}

/// Outputs in port order, with ▲ ▼ to reorder and ✕ to remove; then the inputs.
fn outputs(ui: &mut Ui, reg: &mut Registry, out: &mut Vec<PropAction>, p: &Properties) {
    ui.label(
        RichText::new("Ligações de saída (ordem das portas)")
            .strong()
            .color(colors::HEADING),
    );
    if p.outputs.is_empty() {
        ui.label(RichText::new("Nenhuma. Use a ferramenta Ligar (L).").color(colors::TEXT_WEAK));
    }
    let n = p.outputs.len();
    for (k, to) in p.outputs.iter().enumerate() {
        ui.horizontal(|ui| {
            let text = format!("{} → {to}", k + 1);
            ui.allocate_ui_with_layout(
                Vec2::new(ui.available_width() - 3.0 * 34.0, 24.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.add(Label::new(&text).truncate());
                },
            );
            for (suffix, label, tip, enabled, action) in [
                (
                    "Subir",
                    "▲",
                    "Sobe esta saída na ordem das portas",
                    k > 0,
                    PropAction::Reorder(k, -1),
                ),
                (
                    "Descer",
                    "▼",
                    "Desce esta saída na ordem das portas",
                    k + 1 < n,
                    PropAction::Reorder(k, 1),
                ),
                (
                    "Remover",
                    "×",
                    "Remove esta ligação",
                    true,
                    PropAction::Disconnect(to.clone()),
                ),
            ] {
                let r = ui
                    .add_enabled(enabled, Button::new(label).min_size(Vec2::new(28.0, 24.0)))
                    .on_hover_text(tip);
                named(ui, reg, &format!("Saida_{}_{suffix}", k + 1), &r, label);
                if r.clicked() {
                    out.push(action);
                }
            }
        });
    }
    if !p.inputs.is_empty() {
        ui.add(
            Label::new(
                RichText::new(format!("Entradas: {}", p.inputs.join(", ")))
                    .color(colors::TEXT_WEAK),
            )
            .wrap(),
        );
    }
}

fn value_text(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(x) => num_text(*x),
        Value::Text(t) => t.clone(),
        Value::List(l) => l.join(", "),
    }
}

fn parse_hex(s: &str) -> Option<[u8; 3]> {
    let h = s.trim().trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let b = |k: usize| u8::from_str_radix(&h[k..k + 2], 16).ok();
    Some([b(0)?, b(2)?, b(4)?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Editor;
    use crate::schema::FAMILIES;
    use egui::{Context, Event, Modifiers, PointerButton, Pos2, RawInput};

    /// A small test harness: a context, the panel's state and what it registered.
    struct Panel {
        ctx: Context,
        st: PropsState,
        reg: Registry,
        time: f64,
    }

    impl Panel {
        fn new() -> Panel {
            let ctx = Context::default();
            utsim_ui_tema::install(&ctx);
            Panel {
                ctx,
                st: PropsState::default(),
                reg: Registry::default(),
                time: 0.0,
            }
        }

        fn frame(&mut self, v: &PropsView, events: Vec<Event>) -> Vec<PropAction> {
            self.time += 0.1;
            let input = RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1600.0, 900.0))),
                time: Some(self.time),
                events,
                ..RawInput::default()
            };
            let mut out = Vec::new();
            let (st, reg) = (&mut self.st, &mut self.reg);
            let rect =
                Rect::from_min_max(Pos2::new(1600.0 - WIDTH, 46.0), Pos2::new(1600.0, 860.0));
            self.ctx
                .run_ui(input, |ui| {
                    reg.begin_frame();
                    out.clear();
                    show(ui, rect, v, st, reg, &mut out);
                })
                .drop_without_applying_deltas();
            out
        }

        fn center(&self, name: &str) -> Pos2 {
            self.reg
                .get(name)
                .unwrap_or_else(|| panic!("{name} ausente: {:?}", self.reg.names()))
                .rect
                .center()
        }

        fn click(&mut self, v: &PropsView, name: &str) -> Vec<PropAction> {
            let at = self.center(name);
            let mut out = self.frame(v, vec![Event::PointerMoved(at)]);
            out.extend(self.frame(v, vec![press(at, true)]));
            out.extend(self.frame(v, vec![press(at, false)]));
            out
        }

        fn type_in(&mut self, v: &PropsView, name: &str, text: &str) -> Vec<PropAction> {
            let mut out = self.click(v, name);
            out.extend(self.frame(v, vec![Event::Text(text.to_owned())]));
            out.extend(self.frame(v, vec![key(egui::Key::Enter)]));
            out.extend(self.frame(v, vec![]));
            out
        }
    }

    fn press(at: Pos2, pressed: bool) -> Event {
        Event::PointerButton {
            pos: at,
            button: PointerButton::Primary,
            pressed,
            modifiers: Modifiers::NONE,
        }
    }

    fn key(k: egui::Key) -> Event {
        Event::Key {
            key: k,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }
    }

    fn view(e: &Editor, object: &str) -> PropsView {
        PropsView {
            props: e.properties(object),
            families: FAMILIES.iter().map(|s| s.to_string()).collect(),
            data_paths: Vec::new(),
            extra: String::new(),
            driver: -1,
            miniature: false,
            data: HashMap::new(),
            data_families: Vec::new(),
            base_dir: None,
            abas: e.tab_fields(object),
            somente_leitura: None,
            fadiga: Vec::new(),
            simulado: None,
        }
    }

    fn editor_with_processor() -> (Editor, String) {
        let mut e = Editor::new("teste");
        let name = e.add("Processor", [0.0, 0.0, 0.0]).expect("processo");
        (e, name)
    }

    #[test]
    fn distribution_fields_open_in_the_default_mode_on_every_object() {
        let mut e = Editor::new("teste");
        let a = e.add("Processor", [0.0, 0.0, 0.0]).unwrap();
        let b = e.add("Processor", [4.0, 0.0, 0.0]).unwrap();
        let (va, vb) = (view(&e, &a), view(&e, &b));
        let mut p = Panel::new();
        p.frame(&va, vec![]);
        p.frame(&va, vec![]);
        // The form: the parameters, no RON line.
        let form = |p: &Panel| p.reg.get("Prop_process_ron").is_none();
        assert!(form(&p), "{:?}", p.reg.names());
        // "{ }" switches this field to code...
        p.click(&va, "Prop_process_texto");
        p.frame(&va, vec![]);
        assert!(!form(&p));
        // ...and another object opens in the form again (it used to stay as code).
        p.frame(&vb, vec![]);
        p.frame(&vb, vec![]);
        assert!(form(&p));
        // With the preference "Código", every field opens as code; "{ }" goes back to the
        // form for that one.
        p.st.code_default = true;
        p.frame(&vb, vec![]);
        assert!(!form(&p));
        p.click(&vb, "Prop_process_texto");
        p.frame(&vb, vec![]);
        assert!(form(&p));
    }

    #[test]
    fn fields_keep_the_godot_names() {
        let (e, name) = editor_with_processor();
        let v = view(&e, &name);
        let mut p = Panel::new();
        p.frame(&v, vec![]);
        p.frame(&v, vec![]);
        for n in [
            "PainelPropriedades",
            "Prop_name",
            "Prop_process_familia",
            "Prop_process_texto",
            "Secao_Objeto",
        ] {
            assert!(p.reg.get(n).is_some(), "{n}: {:?}", p.reg.names());
        }
        // Label and control columns line up from row to row.
        let a = p.reg.get("Prop_name").unwrap().rect.min.x;
        let b = p.reg.get("Prop_process_familia").unwrap().rect.min.x;
        assert!((a - b).abs() < 30.0, "{a} {b}");
    }

    #[test]
    fn typing_a_name_selects_all_and_commits_on_enter() {
        let (e, name) = editor_with_processor();
        let v = view(&e, &name);
        let mut p = Panel::new();
        p.frame(&v, vec![]);
        let out = p.type_in(&v, "Prop_name", "torno");
        assert!(
            out.contains(&set("name", Value::Text("torno".into()))),
            "o texto inteiro é trocado: {out:?}"
        );
    }

    #[test]
    fn a_number_typed_becomes_a_number() {
        let mut e = Editor::new("teste");
        let src = e.add("Source", [0.0, 0.0, 0.0]).unwrap();
        let v = view(&e, &src);
        let mut p = Panel::new();
        p.frame(&v, vec![]);
        let field = v
            .props
            .fields
            .iter()
            .find(|f| f.ty == FieldType::Dist)
            .expect("distribuição")
            .clone();
        let key = field.dist.as_ref().unwrap().params[0].key;
        let pname = widget_name(&format!("{}.{key}", field.path));
        let out = p.type_in(&v, &pname, "7,5");
        assert!(
            out.contains(&set(&format!("{}.{key}", field.path), Value::Float(7.5))),
            "{out:?}"
        );
        // The edit goes through the schema's validation.
        let mut e2 = e;
        if let Some(PropAction::Set { path, value, .. }) = out.first() {
            e2.set_property(&src, path, value, &[]).expect("válido");
        }
    }

    #[test]
    fn sections_close_and_stay_closed() {
        let (e, name) = editor_with_processor();
        let v = view(&e, &name);
        let mut p = Panel::new();
        p.frame(&v, vec![]);
        assert!(p.reg.get("Prop_name").is_some());
        p.click(&v, "Secao_Objeto");
        p.frame(&v, vec![]);
        assert!(p.reg.get("Prop_name").is_none(), "seção fechada");
        assert!(p.st.closed.contains("Objeto"));
    }

    #[test]
    fn choices_are_picked_by_label() {
        let (e, name) = editor_with_processor();
        let v = view(&e, &name);
        let mut p = Panel::new();
        p.frame(&v, vec![]);
        let fam = "Prop_process_familia";
        assert!(p.reg.request(fam, "Uniforme"));
        let out = p.frame(&v, vec![]);
        let k = FAMILIES.iter().position(|f| *f == "Uniforme").unwrap() as i64;
        assert_eq!(out, vec![set("process.family", Value::Int(k))]);
    }

    #[test]
    fn the_run_settings_show_without_a_selection() {
        let e = Editor::new("teste");
        let v = view(&e, "run");
        let mut p = Panel::new();
        p.frame(&v, vec![]);
        p.frame(&v, vec![]);
        assert_eq!(
            p.reg.get("Titulo_Propriedades").map(|w| w.text.as_str()),
            Some("Modelo (nada selecionado)")
        );
        assert!(p.reg.knows("Prop_stop_time"), "{:?}", p.reg.names());
    }

    #[test]
    fn widgets_below_the_fold_can_be_revealed() {
        let e = Editor::new("teste");
        let v = view(&e, "run");
        let mut p = Panel::new();
        p.frame(&v, vec![]);
        let last = p
            .reg
            .names()
            .into_iter()
            .chain(v.props.fields.iter().map(|f| widget_name(&f.path)))
            .rfind(|n| p.reg.knows(n) && p.reg.get(n).is_none());
        if let Some(name) = last {
            assert!(p.reg.reveal(&name));
            for _ in 0..6 {
                p.frame(&v, vec![]);
            }
            assert!(p.reg.get(&name).is_some(), "{name} rolado para a vista");
        }
    }

    impl Panel {
        /// Scrolls `name` into view and clicks it.
        fn reveal_click(&mut self, v: &PropsView, name: &str) -> Vec<PropAction> {
            self.frame(v, vec![]);
            if self.reg.get(name).is_none_or(|w| !w.rect.is_positive()) {
                assert!(self.reg.reveal(name), "{name}: {:?}", self.reg.names());
                for _ in 0..6 {
                    self.frame(v, vec![]);
                }
            }
            self.click(v, name)
        }
    }

    #[test]
    fn the_tabs_switch_and_send_their_edits() {
        use crate::ui_abas::AbaAcao;
        let (e, name) = editor_with_processor();
        let mut v = view(&e, &name);
        let mut p = Panel::new();
        p.frame(&v, vec![]);
        // Qualidade first: its switch is off.
        assert!(p.reg.knows("Aba_Qualidade"), "{:?}", p.reg.names());
        p.reveal_click(&v, "Aba_Seguranca");
        p.frame(&v, vec![]);
        assert_eq!(p.st.abas.aba, 1);
        let out = p.reveal_click(&v, "AbaCampo_safety");
        assert_eq!(
            out,
            vec![PropAction::Aba(AbaAcao::Editar {
                caminho: "safety".into(),
                valor: "sim".into()
            })]
        );
        // Someone else edits the model: the tab says who and sends nothing.
        v.somente_leitura = Some("Em edição por bruno no UTSim Qualidade desde 10:00".into());
        p.frame(&v, vec![]);
        assert!(p.reg.knows("AbaTrava"));
        let out = p.reveal_click(&v, "AbaCampo_safety");
        assert!(out.is_empty(), "{out:?}");
        // The run settings have no tabs.
        let run = view(&e, "run");
        let mut p = Panel::new();
        p.frame(&run, vec![]);
        assert!(!p.reg.knows("Aba_Qualidade"));
    }

    #[test]
    fn numbers_print_like_the_spin_box() {
        assert_eq!(num_text(2.0), "2");
        assert_eq!(num_text(2.5), "2.5");
        assert_eq!(num_text(0.1 + 0.2), "0.3");
        assert_eq!(parse_hex("#ff8000"), Some([255, 128, 0]));
        assert_eq!(parse_hex("x"), None);
    }
}
