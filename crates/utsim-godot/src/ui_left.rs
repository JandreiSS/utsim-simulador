//! The left panel of the Godot interface, in egui (pure: no Godot types, unit tested).
//!
//! In Editar: the object library ("Biblioteca", the entries of [`crate::document::palette`]
//! by category, with their icons), the list of the model's objects with a search field, and
//! "Mostrar obstáculos e folgas". In Simular: the replications and "Simular de novo", the
//! status line, the blocks (click to focus or to follow), the camera, "Fluxos" and "Gravar
//! replay", the labels and shadows switches and the results summary with the bottleneck.
//!
//! Like the top bar, the panel changes nothing itself: it reads an [`EditView`] or a
//! [`SimView`] and returns actions in text (`biblioteca:Queue`, `biblioteca_arrastar:Sink`,
//! `objeto:fila`, `objeto_focar:fila`, `obstaculos:ligar`, `replicacoes:10`,
//! `rodar_de_novo`, `bloco:3`, `camera:seguir`, `analisar:fluxos`, `gravar_replay`,
//! `rotulos:desligar`, `sombras:ligar`), which the main scene runs as the old Godot
//! controls did. The widgets keep the names of the Godot nodes (`Biblioteca_<id>`,
//! `Objetos_Busca`, `Objetos_Lista`, `Ver_Obstaculos`, `Simular_Replicacoes`,
//! `Botao_RodarDeNovo`, `Simular_Blocos`, `Simular_Camera`, `Botao_Fluxos`,
//! `Botao_GravarReplay`); new ones: `Objeto_<nome>`, `Bloco_<k>`, `Simular_Status`,
//! `Simular_Rotulos`, `Simular_Sombras`, `Simular_Resumo`.
//!
//! A library entry is armed by a click (then a click on the floor places it) and dragged
//! out of the panel to place it where the button is released: the drag starts here
//! (`biblioteca_arrastar:<id>`, once the pointer moved 8 points from the press) and the
//! 3D view resolves the drop (the editor follows the mouse and places the object when the
//! button is released over the free central area).

use egui::text::CCursorRange;
use egui::{
    Align, Color32, ComboBox, FontSelection, Id, Label, Layout, Rect, RichText, ScrollArea, Sense,
    Stroke, TextEdit, TextWrapMode, TextureId, Ui, UiBuilder, Vec2, WidgetText,
};
use utsim_ui_tema::{colors, sizes, widgets};

use crate::egui_widgets::{Registry, WidgetInfo};
use crate::ui_props::{named, named_list, named_toggle};

/// Width of the panel, in points (the Godot panel had the same).
pub const WIDTH: f32 = 290.0;
/// The pointer must move this far from the press for a library entry to be dragged.
pub const DRAG_START: f32 = 8.0;
/// The bottleneck line of the summary (the orange of the Godot panel).
pub const BOTTLENECK: Color32 = Color32::from_rgb(255, 154, 61);
/// The cameras of Simular: id and label.
pub const CAMERAS: [(&str, &str); 4] = [
    ("orbita", "Órbita"),
    ("seguir", "Seguir"),
    ("primeira_pessoa", "Primeira pessoa"),
    ("recurso", "Do recurso"),
];

/// One entry of the library.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    pub label: String,
    pub category: String,
    /// The icon (`res://icones/<id>.svg`), registered by the bridge.
    pub icon: Option<TextureId>,
}

/// One object of the list: `name (kind)`, red with problems, yellow with warnings.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectItem {
    pub name: String,
    pub kind_label: String,
    pub problems: usize,
    pub warnings: usize,
}

/// What the panel shows in Editar.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct EditView {
    pub library: Vec<Entry>,
    pub categories: Vec<String>,
    /// The entry armed for placing (`""`: none).
    pub armed: String,
    pub objects: Vec<ObjectItem>,
    pub selected: Vec<String>,
    /// "Mostrar obstáculos e folgas".
    pub obstacles: bool,
}

/// The results summary of the last run.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Summary {
    /// `Resultados (5 replicações)`; empty: no run yet.
    pub title: String,
    pub bottleneck: String,
    /// `processo: 12.3 saídas`, one per block.
    pub lines: Vec<String>,
}

/// What the panel shows in Simular.
#[derive(Clone, Debug, PartialEq)]
pub struct SimView {
    pub reps: i64,
    /// The status line (the same as the status bar's while in Simular).
    pub status: String,
    /// The blocks of the scene: `name (kind)`, in scene order.
    pub blocks: Vec<String>,
    /// The block the camera follows (index into `blocks`).
    pub follow: Option<usize>,
    /// The camera mode (an id of [`CAMERAS`]).
    pub camera: String,
    pub labels: bool,
    pub shadows: bool,
    pub summary: Summary,
}

impl Default for SimView {
    fn default() -> Self {
        SimView {
            reps: 5,
            status: String::new(),
            blocks: Vec::new(),
            follow: None,
            camera: "orbita".into(),
            labels: true,
            shadows: true,
            summary: Summary::default(),
        }
    }
}

/// What the panel keeps between frames.
#[derive(Default, Debug)]
pub struct LeftMemory {
    /// The object search.
    pub search: String,
    /// The replications being typed.
    reps: Option<String>,
    /// The library entry being dragged (its drag action already went out).
    dragging: Option<String>,
}

/// Draws the panel in `rect` (points): Editar when `editing`, Simular otherwise.
#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut Ui,
    rect: Rect,
    editing: bool,
    edit: &EditView,
    sim: &SimView,
    mem: &mut LeftMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    ui.painter().rect_filled(rect, 0.0, colors::PANEL);
    ui.painter().line_segment(
        [rect.right_top(), rect.right_bottom()],
        Stroke::new(1.0, colors::BORDER),
    );
    let panel = ui.allocate_rect(rect, Sense::hover());
    named(ui, reg, "PainelEsquerdo", &panel, "");
    let inner = rect.shrink2(Vec2::new(12.0, 10.0));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(inner.expand(2.0));
    child.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
    // The lists show a thin scroll handle even at rest, so a cut list is seen to scroll.
    let mut scroll = egui::style::ScrollStyle::floating();
    scroll.dormant_handle_opacity = 0.5;
    child.spacing_mut().scroll = scroll;
    if editing {
        mem.reps = None;
        edit_panel(&mut child, edit, mem, reg, out);
    } else {
        mem.dragging = None;
        sim_panel(&mut child, sim, mem, reg, out);
    }
}

fn heading(ui: &mut Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .size(sizes::HEADING)
            .strong()
            .color(colors::HEADING),
    );
}

/// A scroll area recorded under `name` (the scripts scroll it with the wheel).
fn named_area(ui: &Ui, reg: &mut Registry, name: &str, rect: Rect) {
    let rect = rect.intersect(ui.clip_rect());
    reg.put_info(
        name,
        WidgetInfo {
            rect: if rect.is_positive() {
                rect
            } else {
                Rect::NOTHING
            },
            text: String::new(),
            enabled: true,
            checked: None,
            choices: Vec::new(),
        },
    );
}

// ---------------------------------------------------------------------------
// Editar
// ---------------------------------------------------------------------------

fn edit_panel(
    ui: &mut Ui,
    v: &EditView,
    mem: &mut LeftMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    heading(ui, "Biblioteca");
    // The library takes what it needs, up to 60 % of the panel; the object list gets the
    // rest.
    let lib_h = (ui.available_height() * 0.6).max(150.0);
    let area = ScrollArea::vertical()
        .id_salt("utsim_biblioteca")
        .max_height(lib_h)
        .auto_shrink([false, true])
        .show(ui, |ui| library(ui, v, mem, reg, out));
    named_area(ui, reg, "Biblioteca", area.inner_rect);
    ui.separator();
    heading(ui, "Objetos");
    search(ui, mem, reg);
    let list_h = (ui.available_height() - 34.0).max(60.0);
    let wanted = mem.search.trim().to_lowercase();
    let area = ScrollArea::vertical()
        .id_salt("utsim_objetos")
        .max_height(list_h)
        .min_scrolled_height(list_h)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            for o in &v.objects {
                let text = format!("{} ({})", o.name, o.kind_label);
                if !wanted.is_empty() && !text.to_lowercase().contains(&wanted) {
                    continue;
                }
                object_row(ui, o, &text, v.selected.contains(&o.name), reg, out);
            }
        });
    named_area(ui, reg, "Objetos_Lista", area.inner_rect);
    let mut on = v.obstacles;
    let r = ui.checkbox(&mut on, "Mostrar obstáculos e folgas").on_hover_text(
        "Mostra a área ocupada dos objetos, as paredes, a folga dos recursos e o ponto de acesso de cada objeto (arraste-o com a ferramenta Selecionar)",
    );
    named_toggle(
        ui,
        reg,
        "Ver_Obstaculos",
        &r,
        "Mostrar obstáculos e folgas",
        v.obstacles,
    );
    if r.changed() {
        out.push(format!(
            "obstaculos:{}",
            if on { "ligar" } else { "desligar" }
        ));
    }
}

fn library(
    ui: &mut Ui,
    v: &EditView,
    mem: &mut LeftMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let gap = 4.0;
    let w = ((ui.available_width() - gap) / 2.0).floor();
    // The drag ends when the button is up again (the drop is the 3D view's).
    if mem.dragging.is_some() && !ui.input(|i| i.pointer.primary_down()) {
        mem.dragging = None;
    }
    for cat in &v.categories {
        ui.add_space(2.0);
        ui.label(RichText::new(cat).small().strong().color(colors::TEXT_WEAK));
        let entries: Vec<&Entry> = v.library.iter().filter(|e| &e.category == cat).collect();
        for pair in entries.chunks(2) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = gap;
                for e in pair {
                    entry_button(ui, e, w, v.armed == e.id, mem, reg, out);
                }
            });
        }
    }
}

/// A library entry: icon and label, highlighted when armed; click arms it, a drag places it.
fn entry_button(
    ui: &mut Ui,
    e: &Entry,
    w: f32,
    armed: bool,
    mem: &mut LeftMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let (rect, r) = ui.allocate_exact_size(Vec2::new(w, 32.0), Sense::click_and_drag());
    let visuals = ui.style().interact_selectable(&r, armed);
    let fill = if armed {
        colors::SELECTED
    } else if r.hovered() {
        colors::HOVER
    } else {
        colors::BUTTON
    };
    let stroke = if armed {
        Stroke::new(1.0, colors::ACCENT)
    } else {
        visuals.bg_stroke
    };
    ui.painter().rect(
        rect,
        sizes::RADIUS as f32,
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    let mut x = rect.min.x + 6.0;
    if let Some(tex) = e.icon {
        let icon = Rect::from_center_size(egui::pos2(x + 10.0, rect.center().y), Vec2::splat(20.0));
        ui.painter().image(
            tex,
            icon,
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        x += 25.0;
    }
    let galley = WidgetText::from(RichText::new(&e.label).color(colors::TEXT)).into_galley(
        ui,
        Some(TextWrapMode::Truncate),
        rect.max.x - x - 3.0,
        FontSelection::Default,
    );
    ui.painter().galley(
        egui::pos2(x, rect.center().y - galley.size().y / 2.0),
        galley,
        colors::TEXT,
    );
    let tip = format!(
        "{}: clique e depois clique no chão, ou arraste até o chão",
        e.label
    );
    let r = r
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tip);
    let name = format!("Biblioteca_{}", e.id);
    named_toggle(ui, reg, &name, &r, &e.label, armed);
    if r.dragged() && mem.dragging.is_none() {
        let moved = ui.input(
            |i| match (i.pointer.press_origin(), i.pointer.latest_pos()) {
                (Some(a), Some(b)) => a.distance(b),
                _ => 0.0,
            },
        );
        if moved > DRAG_START {
            mem.dragging = Some(e.id.clone());
            out.push(format!("biblioteca_arrastar:{}", e.id));
        }
    }
    if r.clicked() && mem.dragging.is_none() {
        out.push(format!("biblioteca:{}", e.id));
    }
}

/// The object search: filters the list by name or kind (`×` clears it).
fn search(ui: &mut Ui, mem: &mut LeftMemory, reg: &mut Registry) {
    ui.horizontal(|ui| {
        let clear_w = if mem.search.is_empty() { 0.0 } else { 30.0 };
        let r = ui.add(
            TextEdit::singleline(&mut mem.search)
                .id(Id::new("utsim_objetos_busca"))
                .hint_text("Buscar por nome ou tipo")
                .desired_width(ui.available_width() - clear_w),
        );
        named(ui, reg, "Objetos_Busca", &r, &mem.search.clone());
        if !mem.search.is_empty() {
            let x = ui.button("×").on_hover_text("Limpa a busca");
            named(ui, reg, "Objetos_Limpar", &x, "×");
            if x.clicked() {
                mem.search.clear();
            }
        }
    });
}

fn object_row(
    ui: &mut Ui,
    o: &ObjectItem,
    text: &str,
    selected: bool,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let w = ui.available_width();
    let (rect, r) = ui.allocate_exact_size(Vec2::new(w, 24.0), Sense::click());
    if selected {
        ui.painter()
            .rect_filled(rect, sizes::RADIUS as f32, colors::SELECTED);
    } else if r.hovered() {
        ui.painter()
            .rect_filled(rect, sizes::RADIUS as f32, colors::HOVER);
    }
    let color = if o.problems > 0 {
        colors::ERROR
    } else if o.warnings > 0 {
        colors::WARNING
    } else {
        colors::TEXT
    };
    let galley = WidgetText::from(RichText::new(text).color(color)).into_galley(
        ui,
        Some(TextWrapMode::Truncate),
        rect.width() - 12.0,
        FontSelection::Default,
    );
    let elided = galley.elided;
    ui.painter().galley(
        egui::pos2(rect.min.x + 6.0, rect.center().y - galley.size().y / 2.0),
        galley,
        color,
    );
    let mut tip = String::new();
    if elided {
        tip.push_str(text);
    }
    let count = if o.problems > 0 {
        format!("{} problema(s)", o.problems)
    } else if o.warnings > 0 {
        format!("{} aviso(s)", o.warnings)
    } else {
        String::new()
    };
    if !count.is_empty() {
        if !tip.is_empty() {
            tip.push('\n');
        }
        tip.push_str(&count);
    }
    let r = if tip.is_empty() {
        r
    } else {
        r.on_hover_text(tip)
    };
    named_toggle(ui, reg, &format!("Objeto_{}", o.name), &r, text, selected);
    if r.clicked() {
        out.push(format!("objeto:{}", o.name));
    }
    if r.double_clicked() {
        out.push(format!("objeto_focar:{}", o.name));
    }
}

// ---------------------------------------------------------------------------
// Simular
// ---------------------------------------------------------------------------

fn sim_panel(
    ui: &mut Ui,
    v: &SimView,
    mem: &mut LeftMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    replications(ui, v, mem, reg, out);
    let w = ui.available_width();
    let r = ui
        .add_sized([w, 32.0], widgets::primary("Simular de novo"))
        .on_hover_text("Roda as replicações outra vez");
    named(ui, reg, "Botao_RodarDeNovo", &r, "Simular de novo");
    if r.clicked() {
        out.push("rodar_de_novo".into());
    }
    if !v.status.is_empty() {
        let r = ui.add(Label::new(RichText::new(&v.status).color(colors::TEXT_WEAK)).wrap());
        named(ui, reg, "Simular_Status", &r, &v.status);
    }
    ui.label(RichText::new("Blocos (clique para focar)").color(colors::TEXT));
    // Below the list: the camera, the buttons, the switches and the summary.
    let summary_h = (ui.available_height() * 0.32).clamp(110.0, 220.0);
    let below = 3.0 * 32.0 + summary_h + 4.0 * 6.0;
    let list_h = (ui.available_height() - below).max(90.0);
    let area = ScrollArea::vertical()
        .id_salt("utsim_blocos")
        .max_height(list_h)
        .min_scrolled_height(list_h)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            for (k, b) in v.blocks.iter().enumerate() {
                block_row(ui, k, b, v.follow == Some(k), reg, out);
            }
        });
    named_area(ui, reg, "Simular_Blocos", area.inner_rect);
    camera(ui, v, reg, out);
    ui.horizontal(|ui| {
        let r = ui
            .button("Fluxos")
            .on_hover_text("Fluxos de tokens em 2D, com as contagens no tempo");
        named(ui, reg, "Botao_Fluxos", &r, "Fluxos");
        if r.clicked() {
            out.push("analisar:fluxos".into());
        }
        let r = ui
            .button("Gravar replay")
            .on_hover_text("Grava o replay desta replicação em .json (abre de novo com Abrir)");
        named(ui, reg, "Botao_GravarReplay", &r, "Gravar replay");
        if r.clicked() {
            out.push("gravar_replay".into());
        }
    });
    ui.horizontal(|ui| {
        for (name, text, on, what, tip) in [
            (
                "Simular_Rotulos",
                "Rótulos",
                v.labels,
                "rotulos",
                "Os nomes dos blocos sobre a cena",
            ),
            (
                "Simular_Sombras",
                "Sombras",
                v.shadows,
                "sombras",
                "As sombras do sol (desligadas, a cena fica mais leve)",
            ),
        ] {
            let mut now = on;
            let r = ui.checkbox(&mut now, text).on_hover_text(tip);
            named_toggle(ui, reg, name, &r, text, on);
            if r.changed() {
                out.push(format!("{what}:{}", if now { "ligar" } else { "desligar" }));
            }
        }
    });
    summary(ui, &v.summary, summary_h, reg);
}

/// "Replicações" with a field and − +: the number is sent when typed (Enter or focus out)
/// or stepped, and "Simular de novo" runs it.
fn replications(
    ui: &mut Ui,
    v: &SimView,
    mem: &mut LeftMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Replicações").color(colors::TEXT));
        let id = Id::new("utsim_simular_replicacoes");
        let current = v.reps.to_string();
        let mut buf = mem.reps.clone().unwrap_or_else(|| current.clone());
        let mut output = TextEdit::singleline(&mut buf)
            .id(id)
            .desired_width(70.0)
            .horizontal_align(Align::Max)
            .show(ui);
        let r = output.response.response.clone();
        if r.gained_focus() {
            output
                .state
                .cursor
                .set_char_range(Some(CCursorRange::select_all(&output.galley)));
            output.state.store(ui.ctx(), id);
        }
        named(ui, reg, "Simular_Replicacoes", &r, &buf);
        if r.lost_focus() {
            let cancelled = ui.input(|i| i.key_pressed(egui::Key::Escape));
            if !cancelled
                && buf != current
                && let Ok(n) = buf.trim().parse::<i64>()
            {
                out.push(format!("replicacoes:{}", n.clamp(1, 1000)));
            }
            mem.reps = None;
        } else if r.has_focus() {
            mem.reps = Some(buf);
        } else {
            mem.reps = None;
        }
        for (name, text, d, tip) in [
            ("Simular_Menos", "−", -1, "Uma replicação a menos"),
            ("Simular_Mais", "+", 1, "Uma replicação a mais"),
        ] {
            let r = ui
                .add_enabled(
                    (d < 0 && v.reps > 1) || (d > 0 && v.reps < 1000),
                    egui::Button::new(text).min_size(Vec2::new(26.0, 0.0)),
                )
                .on_hover_text(tip);
            named(ui, reg, name, &r, text);
            if r.clicked() {
                out.push(format!("replicacoes:{}", (v.reps + d).clamp(1, 1000)));
            }
        }
    });
}

fn block_row(
    ui: &mut Ui,
    k: usize,
    text: &str,
    selected: bool,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let w = ui.available_width();
    let (rect, r) = ui.allocate_exact_size(Vec2::new(w, 24.0), Sense::click());
    if selected {
        ui.painter()
            .rect_filled(rect, sizes::RADIUS as f32, colors::SELECTED);
    } else if r.hovered() {
        ui.painter()
            .rect_filled(rect, sizes::RADIUS as f32, colors::HOVER);
    }
    let galley = WidgetText::from(RichText::new(text).color(colors::TEXT)).into_galley(
        ui,
        Some(TextWrapMode::Truncate),
        rect.width() - 12.0,
        FontSelection::Default,
    );
    let elided = galley.elided;
    ui.painter().galley(
        egui::pos2(rect.min.x + 6.0, rect.center().y - galley.size().y / 2.0),
        galley,
        colors::TEXT,
    );
    let r = if elided { r.on_hover_text(text) } else { r };
    named_toggle(ui, reg, &format!("Bloco_{k}"), &r, text, selected);
    if r.clicked() {
        out.push(format!("bloco:{k}"));
    }
}

fn camera(ui: &mut Ui, v: &SimView, reg: &mut Registry, out: &mut Vec<String>) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Câmera").color(colors::TEXT));
        let shown = CAMERAS
            .iter()
            .find(|c| c.0 == v.camera)
            .map(|c| c.1)
            .unwrap_or("Órbita");
        let mut pick = reg
            .take_request("Simular_Camera")
            .and_then(|item| CAMERAS.iter().find(|c| c.1 == item).map(|c| c.0));
        let r = ComboBox::from_id_salt("utsim_simular_camera")
            .selected_text(shown)
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                for (id, label) in CAMERAS {
                    if ui.selectable_label(id == v.camera, label).clicked() {
                        pick = Some(id);
                    }
                }
            });
        let resp = r.response.on_hover_text(
            "Seguir, Primeira pessoa e Do recurso acompanham o bloco escolhido na lista",
        );
        named_list(
            ui,
            reg,
            "Simular_Camera",
            &resp,
            shown,
            CAMERAS.iter().map(|c| c.1.to_owned()).collect(),
        );
        if let Some(id) = pick {
            out.push(format!("camera:{id}"));
        }
    });
}

fn summary(ui: &mut Ui, s: &Summary, height: f32, reg: &mut Registry) {
    let frame = egui::Frame::new()
        .fill(colors::FIELD)
        .stroke(Stroke::new(1.0, colors::BORDER))
        .corner_radius(sizes::RADIUS)
        .inner_margin(egui::Margin::same(8));
    let r = frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        ScrollArea::vertical()
            .id_salt("utsim_resumo")
            .max_height((height - 16.0).max(40.0))
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 3.0;
                if s.title.is_empty() {
                    ui.add(
                        Label::new(
                            RichText::new("Os resultados aparecem aqui depois da simulação.")
                                .color(colors::TEXT_WEAK),
                        )
                        .wrap(),
                    );
                    return;
                }
                ui.add(Label::new(RichText::new(&s.title).strong().color(colors::HEADING)).wrap());
                if !s.bottleneck.is_empty() {
                    ui.add(
                        Label::new(
                            RichText::new(format!("Gargalo: {}", s.bottleneck)).color(BOTTLENECK),
                        )
                        .wrap(),
                    );
                }
                for l in &s.lines {
                    ui.add(Label::new(RichText::new(l).color(colors::TEXT)).wrap());
                }
                ui.add(
                    Label::new(
                        RichText::new(
                            "A tabela completa, os gráficos e as análises estão em Resultados.",
                        )
                        .color(colors::TEXT_WEAK),
                    )
                    .wrap(),
                );
            });
    });
    let mut text = s.title.clone();
    if !s.bottleneck.is_empty() {
        text.push_str(&format!("\nGargalo: {}", s.bottleneck));
    }
    for l in &s.lines {
        text.push('\n');
        text.push_str(l);
    }
    named(ui, reg, "Simular_Resumo", &r.response, &text);
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Context, Event, Modifiers, PointerButton, Pos2, RawInput};

    struct Panel {
        ctx: Context,
        reg: Registry,
        mem: LeftMemory,
        time: f64,
    }

    const RECT: Rect = Rect {
        min: Pos2::new(0.0, 46.0),
        max: Pos2::new(WIDTH, 680.0),
    };

    impl Panel {
        fn new() -> Panel {
            let ctx = Context::default();
            utsim_ui_tema::install(&ctx);
            Panel {
                ctx,
                reg: Registry::default(),
                mem: LeftMemory::default(),
                time: 0.0,
            }
        }

        fn frame(
            &mut self,
            editing: bool,
            e: &EditView,
            s: &SimView,
            events: Vec<Event>,
        ) -> Vec<String> {
            let mut out = Vec::new();
            self.time += 0.05;
            let input = RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 720.0))),
                time: Some(self.time),
                events,
                ..RawInput::default()
            };
            let (reg, mem) = (&mut self.reg, &mut self.mem);
            self.ctx
                .run_ui(input, |ui| {
                    reg.begin_frame();
                    out.clear();
                    show(ui, RECT, editing, e, s, mem, reg, &mut out);
                })
                .drop_without_applying_deltas();
            out
        }

        fn center(&self, name: &str) -> Pos2 {
            self.reg
                .get(name)
                .map(|w| w.rect.center())
                .unwrap_or_else(|| panic!("{name} ausente: {:?}", self.reg.names()))
        }
    }

    fn moved(p: Pos2) -> Event {
        Event::PointerMoved(p)
    }

    fn press(p: Pos2, pressed: bool) -> Event {
        Event::PointerButton {
            pos: p,
            button: PointerButton::Primary,
            pressed,
            modifiers: Modifiers::NONE,
        }
    }

    fn edit_view() -> EditView {
        EditView {
            library: crate::document::palette()
                .iter()
                .map(|p| Entry {
                    id: p.id.into(),
                    label: p.label.into(),
                    category: p.category.into(),
                    icon: None,
                })
                .collect(),
            categories: utsim_edit::CATEGORIES
                .iter()
                .map(|c| c.to_string())
                .collect(),
            armed: String::new(),
            objects: vec![
                ObjectItem {
                    name: "chegada".into(),
                    kind_label: "Chegada".into(),
                    problems: 1,
                    warnings: 0,
                },
                ObjectItem {
                    name: "fila".into(),
                    kind_label: "Fila".into(),
                    problems: 0,
                    warnings: 0,
                },
            ],
            selected: vec!["fila".into()],
            obstacles: false,
        }
    }

    #[test]
    fn the_library_keeps_the_godot_names_and_fits() {
        let mut p = Panel::new();
        let e = edit_view();
        let s = SimView::default();
        p.frame(true, &e, &s, vec![]);
        p.frame(true, &e, &s, vec![]);
        for name in [
            "Biblioteca",
            "Biblioteca_Source",
            "Biblioteca_Queue",
            "Biblioteca_QueueLifo",
            "Biblioteca_Processor",
            "Biblioteca_Sink",
            "Objetos_Busca",
            "Objetos_Lista",
            "Objeto_fila",
            "Ver_Obstaculos",
        ] {
            assert!(p.reg.knows(name), "{name}: {:?}", p.reg.names());
        }
        for n in p.reg.names() {
            let r = p.reg.get(&n).unwrap().rect;
            assert!(
                r.min.x >= RECT.min.x - 0.5 && r.max.x <= RECT.max.x + 0.5,
                "{n} sai do painel: {r:?}"
            );
            assert!(r.max.y <= RECT.max.y + 0.5, "{n} sai do painel: {r:?}");
        }
        assert_eq!(p.reg.get("Objeto_fila").and_then(|w| w.checked), Some(true));
        assert_eq!(
            p.reg.get("Objeto_chegada").and_then(|w| w.checked),
            Some(false)
        );
    }

    #[test]
    fn a_click_arms_and_a_drag_starts_a_placement() {
        let mut p = Panel::new();
        let e = edit_view();
        let s = SimView::default();
        p.frame(true, &e, &s, vec![]);
        let at = p.center("Biblioteca_Queue");
        let mut out = Vec::new();
        for ev in [moved(at), press(at, true), press(at, false)] {
            out.extend(p.frame(true, &e, &s, vec![ev]));
        }
        assert_eq!(out, vec!["biblioteca:Queue".to_owned()]);
        // A drag past 8 points goes to the 3D view once; its release arms nothing.
        let at = p.center("Biblioteca_Sink");
        let mut out = Vec::new();
        out.extend(p.frame(true, &e, &s, vec![moved(at), press(at, true)]));
        for k in 1..8 {
            out.extend(p.frame(
                true,
                &e,
                &s,
                vec![moved(at + Vec2::new(k as f32 * 30.0, 4.0))],
            ));
        }
        out.extend(p.frame(true, &e, &s, vec![press(at + Vec2::new(400.0, 4.0), false)]));
        out.extend(p.frame(true, &e, &s, vec![]));
        assert_eq!(out, vec!["biblioteca_arrastar:Sink".to_owned()]);
    }

    #[test]
    fn the_search_filters_and_a_click_selects() {
        let mut p = Panel::new();
        let e = edit_view();
        let s = SimView::default();
        p.frame(true, &e, &s, vec![]);
        let at = p.center("Objeto_chegada");
        let mut out = Vec::new();
        for ev in [moved(at), press(at, true), press(at, false)] {
            out.extend(p.frame(true, &e, &s, vec![ev]));
        }
        assert_eq!(out, vec!["objeto:chegada".to_owned()]);
        p.mem.search = "fil".into();
        p.frame(true, &e, &s, vec![]);
        assert!(p.reg.knows("Objeto_fila"));
        assert!(!p.reg.knows("Objeto_chegada"));
    }

    fn sim_view() -> SimView {
        SimView {
            reps: 5,
            status: "5 replicações concluídas. Reproduzindo a replicação 0.".into(),
            blocks: (0..30).map(|k| format!("bloco_{k} (Processo)")).collect(),
            follow: Some(2),
            camera: "seguir".into(),
            labels: true,
            shadows: false,
            summary: Summary {
                title: "Resultados (5 replicações)".into(),
                bottleneck: "processo".into(),
                lines: vec!["processo: 120 saídas".into()],
            },
        }
    }

    #[test]
    fn simular_keeps_the_godot_names_and_fits() {
        let mut p = Panel::new();
        let e = EditView::default();
        let s = sim_view();
        p.frame(false, &e, &s, vec![]);
        p.frame(false, &e, &s, vec![]);
        for name in [
            "Simular_Replicacoes",
            "Botao_RodarDeNovo",
            "Simular_Status",
            "Simular_Blocos",
            "Bloco_0",
            "Simular_Camera",
            "Botao_Fluxos",
            "Botao_GravarReplay",
            "Simular_Rotulos",
            "Simular_Sombras",
            "Simular_Resumo",
        ] {
            assert!(p.reg.get(name).is_some(), "{name}: {:?}", p.reg.names());
        }
        let resumo = p.reg.get("Simular_Resumo").unwrap();
        assert!(resumo.text.contains("Gargalo: processo"), "{}", resumo.text);
        assert!(resumo.rect.max.y <= RECT.max.y + 0.5, "{:?}", resumo.rect);
        assert_eq!(p.reg.get("Simular_Camera").unwrap().text, "Seguir");
        assert_eq!(p.reg.get("Simular_Sombras").unwrap().checked, Some(false));
        assert_eq!(p.reg.get("Bloco_2").unwrap().checked, Some(true));
    }

    #[test]
    fn replications_are_typed_and_the_camera_is_chosen() {
        let mut p = Panel::new();
        let e = EditView::default();
        let s = sim_view();
        p.frame(false, &e, &s, vec![]);
        let at = p.center("Simular_Replicacoes");
        let mut out = Vec::new();
        for ev in [moved(at), press(at, true), press(at, false)] {
            out.extend(p.frame(false, &e, &s, vec![ev]));
        }
        out.extend(p.frame(false, &e, &s, vec![Event::Text("1000".into())]));
        out.extend(p.frame(
            false,
            &e,
            &s,
            vec![Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::NONE,
            }],
        ));
        assert_eq!(out, vec!["replicacoes:1000".to_owned()]);
        assert!(p.reg.request("Simular_Camera", "Primeira pessoa"));
        let out = p.frame(false, &e, &s, vec![]);
        assert_eq!(out, vec!["camera:primeira_pessoa".to_owned()]);
        let at = p.center("Bloco_4");
        let mut out = Vec::new();
        for ev in [moved(at), press(at, true), press(at, false)] {
            out.extend(p.frame(false, &e, &s, vec![ev]));
        }
        assert_eq!(out, vec!["bloco:4".to_owned()]);
    }
}
