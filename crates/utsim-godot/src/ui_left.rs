//! The left panel of the Godot interface, in egui (pure: no Godot types, unit tested).
//!
//! In Editar: the object library ("Biblioteca": a search over every entry, then a rail of
//! [`groups`] on the left, which joins the core categories and the lines' presets in a few
//! groups plus "Recentes", and the entries of the chosen group beside it; a click on the open
//! group folds the library to a row of group icons), the list of the model's objects with a
//! search field, and "Mostrar obstáculos e folgas". In Simular: the replications and "Simular de novo", the
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
//! `Simular_Rotulos`, `Simular_Sombras`, `Simular_Resumo`, `Biblioteca_Busca`,
//! `Biblioteca_Grupo_<id>`.
//!
//! A library entry is armed by a click (then a click on the floor places it) and dragged
//! out of the panel to place it where the button is released: the drag starts here
//! (`biblioteca_arrastar:<id>`, once the pointer moved 8 points from the press) and the
//! 3D view resolves the drop (the editor follows the mouse and places the object when the
//! button is released over the free central area).

use crate::texto::{fx, tx, vx};
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
pub fn cameras() -> [(&'static str, &'static str); 5] {
    [
        ("orbita", tx("godot-ui-left-orbita")),
        ("planta", tx("godot-ui-left-planta_2d")),
        ("seguir", tx("godot-ui-left-seguir")),
        ("primeira_pessoa", tx("godot-ui-left-primeira_pessoa")),
        ("recurso", tx("godot-ui-left-do_recurso")),
    ]
}

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
    /// The camera mode (an id of [`cameras`]).
    pub camera: String,
    pub labels: bool,
    pub shadows: bool,
    /// Camada "Segurança" (zonas, EPI, quase-conflitos, itens com defeito) e mapa de calor.
    pub seguranca: bool,
    pub calor: bool,
    /// Quase-conflitos simulados `(instante, texto)`, para o salto da reprodução.
    pub quase: Vec<(f64, String)>,
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
            seguranca: false,
            calor: false,
            quase: Vec::new(),
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
    /// The library search (all groups).
    pub library_search: String,
    /// The library group shown (`None`: "fluxo").
    pub group: Option<String>,
    /// The library folded to a row of group icons (a click on the open group).
    pub folded: bool,
    /// The entries used last, newest first (at most [`RECENT_MAX`]).
    pub recent: Vec<String>,
}

/// How many entries "Recentes" keeps.
pub const RECENT_MAX: usize = 8;
/// Width of the rail of library groups.
pub const RAIL: f32 = 64.0;

/// One group of the library rail: id (in `Biblioteca_Grupo_<id>`), short label, title,
/// and the entry whose icon stands for it.
pub struct Group {
    pub id: &'static str,
    pub short: &'static str,
    pub title: &'static str,
    pub icon_of: &'static str,
}

/// The library groups, in rail order: "Recentes", then the core categories and the lines'
/// categories joined in six (see [`group_of`]).
pub fn groups() -> [Group; 7] {
    [
        Group {
            id: "recentes",
            short: tx("godot-ui-left-recentes"),
            title: tx("godot-ui-left-usados_por_ultimo"),
            icon_of: "",
        },
        Group {
            id: "fluxo",
            short: tx("godot-ui-left-fluxo"),
            title: tx("godot-ui-left-fluxo_de_itens_e_montagem"),
            icon_of: "Processor",
        },
        Group {
            id: "movimento",
            short: tx("godot-ui-left-mover"),
            title: tx("godot-ui-left-movimentacao_recursos_e_logistica"),
            icon_of: "Forklift",
        },
        Group {
            id: "fabrica",
            short: tx("godot-ui-left-fabrica"),
            title: tx("godot-ui-left-manufatura"),
            icon_of: "Combiner",
        },
        Group {
            id: "servicos",
            short: tx("godot-ui-left-servicos"),
            title: tx("godot-ui-left-servicos_e_saude"),
            icon_of: "Operator",
        },
        Group {
            id: "escritorio",
            short: tx("godot-ui-left-escritorio"),
            title: tx("godot-ui-left-escritorio_e_processos"),
            icon_of: "Dispatcher",
        },
        Group {
            id: "qs",
            short: tx("godot-ui-left-q_e_s"),
            title: tx("godot-ui-left-qualidade_e_seguranca"),
            icon_of: "Separator",
        },
    ]
}

/// The group of a library entry: the core categories by name, the lines' presets by the
/// line (the part of the id before `/`).
pub fn group_of(e: &Entry) -> &'static str {
    match e.id.split_once('/') {
        Some((line, _)) => match line {
            "manufatura" => "fabrica",
            "logistica" => "movimento",
            "servicos" | "saude" => "servicos",
            "qualidade" | "seguranca" => "qs",
            _ => "escritorio",
        },
        None => {
            if utsim_edit::CATEGORIES[..2].contains(&e.category.as_str()) {
                "fluxo"
            } else {
                "movimento"
            }
        }
    }
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
            count: None,
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
    heading(ui, tx("godot-ui-left-biblioteca"));
    let before = out.len();
    library(ui, v, mem, reg, out);
    remember_used(mem, &out[before..]);
    ui.separator();
    heading(ui, tx("godot-ui-left-objetos"));
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
    let r = ui
        .checkbox(&mut on, tx("godot-ui-left-mostrar_obstaculos_e_folgas"))
        .on_hover_text(tx("godot-ui-left-mostra_a_area_ocupada_dos"));
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

/// Height of one item of the library rail.
const RAIL_ITEM: f32 = 46.0;

/// The library: a search over every entry, then a rail of groups on the left and the
/// entries of the chosen group beside it. A click on the open group folds the library to a
/// row of group icons (more room for the object list); a click on an icon opens it again.
fn library(
    ui: &mut Ui,
    v: &EditView,
    mem: &mut LeftMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    // The drag ends when the button is up again (the drop is the 3D view's).
    if mem.dragging.is_some() && !ui.input(|i| i.pointer.primary_down()) {
        mem.dragging = None;
    }
    library_search(ui, mem, reg);
    let wanted = mem.library_search.trim().to_lowercase();
    if !wanted.is_empty() {
        let found: Vec<&Entry> = v
            .library
            .iter()
            .filter(|e| {
                e.label.to_lowercase().contains(&wanted)
                    || entry_label(e).to_lowercase().contains(&wanted)
                    || e.category.to_lowercase().contains(&wanted)
            })
            .collect();
        let h = (ui.available_height() * 0.5).max(120.0);
        let area = ScrollArea::vertical()
            .id_salt("utsim_biblioteca")
            .max_height(h)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                if found.is_empty() {
                    ui.label(
                        RichText::new(tx("godot-ui-left-nada_com_esse_nome"))
                            .color(colors::TEXT_WEAK),
                    );
                }
                let w = ui.available_width() - 4.0;
                for e in found {
                    entry_button(ui, e, w, v.armed == e.id, mem, reg, out);
                }
            });
        named_area(ui, reg, "Biblioteca", area.inner_rect);
        return;
    }
    let current = mem.group.clone().unwrap_or_else(|| "fluxo".to_string());
    if mem.folded {
        let (row, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::hover());
        let mut x = row.left();
        for g in &groups() {
            let r = Rect::from_min_size(egui::pos2(x, row.top()), Vec2::splat(34.0));
            let resp = ui
                .interact(r, Id::new(("utsim_grupo_dobrado", g.id)), Sense::click())
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text(g.title);
            let fill = if resp.hovered() {
                colors::HOVER
            } else {
                colors::BUTTON
            };
            ui.painter().rect_filled(r, sizes::RADIUS as f32, fill);
            group_icon(ui, r.center(), 20.0, g, v);
            named(
                ui,
                reg,
                &format!("Biblioteca_Grupo_{}", g.id),
                &resp,
                g.short,
            );
            if resp.clicked() {
                mem.group = Some(g.id.to_string());
                mem.folded = false;
            }
            x += 38.0;
        }
        named_area(ui, reg, "Biblioteca", row);
        return;
    }
    let h = groups().len() as f32 * RAIL_ITEM;
    let (all, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::hover());
    let rail = Rect::from_min_size(all.min, Vec2::new(RAIL, h));
    let side = Rect::from_min_max(egui::pos2(rail.right() + 8.0, all.top()), all.max);
    ui.painter().rect_filled(
        rail,
        sizes::RADIUS as f32,
        colors::BUTTON.gamma_multiply(0.6),
    );
    for (k, g) in groups().iter().enumerate() {
        let r = Rect::from_min_size(
            egui::pos2(rail.left(), rail.top() + k as f32 * RAIL_ITEM),
            Vec2::new(RAIL, RAIL_ITEM),
        );
        let on = g.id == current;
        let resp = ui
            .interact(r, Id::new(("utsim_grupo", g.id)), Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text(if on {
                fx(
                    "godot-ui-left-clique_de_novo_para_recolher",
                    &[("title", vx(&g.title))],
                )
            } else {
                g.title.to_string()
            });
        if on {
            ui.painter()
                .rect_filled(r.shrink(2.0), sizes::RADIUS as f32, colors::SELECTED);
            ui.painter().rect_filled(
                Rect::from_min_size(
                    r.min + Vec2::new(0.0, 8.0),
                    Vec2::new(3.0, r.height() - 16.0),
                ),
                2.0,
                colors::ACCENT,
            );
        } else if resp.hovered() {
            ui.painter()
                .rect_filled(r.shrink(2.0), sizes::RADIUS as f32, colors::HOVER);
        }
        group_icon(ui, egui::pos2(r.center().x, r.top() + 16.0), 20.0, g, v);
        ui.painter().text(
            egui::pos2(r.center().x, r.bottom() - 10.0),
            egui::Align2::CENTER_CENTER,
            g.short,
            egui::FontId::proportional(10.5),
            if on { colors::TEXT } else { colors::TEXT_WEAK },
        );
        named(
            ui,
            reg,
            &format!("Biblioteca_Grupo_{}", g.id),
            &resp,
            g.short,
        );
        if resp.clicked() {
            if on {
                mem.folded = true;
            } else {
                mem.group = Some(g.id.to_string());
            }
        }
    }
    let all = groups();
    let group = all.iter().find(|g| g.id == current).unwrap_or(&all[1]);
    let entries: Vec<&Entry> = if group.id == "recentes" {
        mem.recent
            .iter()
            .filter_map(|id| v.library.iter().find(|e| &e.id == id))
            .collect()
    } else {
        v.library
            .iter()
            .filter(|e| group_of(e) == group.id)
            .collect()
    };
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(side)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing.y = 4.0;
    child.label(
        RichText::new(group.title)
            .small()
            .strong()
            .color(colors::TEXT_WEAK),
    );
    let area = ScrollArea::vertical()
        .id_salt("utsim_biblioteca")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            if entries.is_empty() {
                ui.label(
                    RichText::new(tx("godot-ui-left-os_elementos_que_voce_usar"))
                        .small()
                        .color(colors::TEXT_WEAK),
                );
            }
            // The categories inside a group (a line's name) as small titles, when there
            // are more than one.
            let mut cats: Vec<&str> = entries.iter().map(|e| e.category.as_str()).collect();
            cats.dedup();
            let titles = group.id != "recentes" && cats.len() > 1;
            let w = ui.available_width() - 4.0;
            let mut last: Option<&str> = None;
            for e in &entries {
                if titles && last != Some(e.category.as_str()) {
                    if last.is_some() {
                        ui.add_space(4.0);
                    }
                    ui.label(
                        RichText::new(utsim_edit::texto::rotulo_categoria(short_category(
                            &e.category,
                        )))
                        .size(10.5)
                        .color(colors::TEXT_WEAK),
                    );
                    last = Some(e.category.as_str());
                }
                entry_button(ui, e, w, v.armed == e.id, mem, reg, out);
            }
        });
    named_area(ui, reg, "Biblioteca", area.inner_rect);
}

/// The label of a library entry in the language of now: the core entries by id (the queue
/// is "Espera" in this editor), the lines' presets as their file names them.
fn entry_label(e: &Entry) -> String {
    match e.id.as_str() {
        "Queue" | "QueueLifo" => tx("godot-ui-left-espera").to_string(),
        id => utsim_edit::texto::rotulo_de(id, &e.label).to_string(),
    }
}

/// A line's category without the part in parentheses ("Lean Office (processos
/// administrativos)" → "Lean Office"), to fit the narrow column.
fn short_category(c: &str) -> &str {
    c.split(" (").next().unwrap_or(c).trim()
}

/// The icon of a group: the icon of its entry, a clock for "Recentes", or its initial.
fn group_icon(ui: &Ui, center: egui::Pos2, size: f32, g: &Group, v: &EditView) {
    let p = ui.painter();
    if g.id == "recentes" {
        let r = size * 0.42;
        p.circle_stroke(center, r, Stroke::new(1.6, colors::TEXT));
        p.line_segment(
            [center, center + Vec2::new(0.0, -r * 0.65)],
            Stroke::new(1.6, colors::TEXT),
        );
        p.line_segment(
            [center, center + Vec2::new(r * 0.5, r * 0.2)],
            Stroke::new(1.6, colors::TEXT),
        );
        return;
    }
    match v
        .library
        .iter()
        .find(|e| e.id == g.icon_of)
        .and_then(|e| e.icon)
    {
        Some(tex) => {
            p.image(
                tex,
                Rect::from_center_size(center, Vec2::splat(size)),
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        None => {
            p.rect_filled(
                Rect::from_center_size(center, Vec2::splat(size)),
                4.0,
                colors::HOVER,
            );
            p.text(
                center,
                egui::Align2::CENTER_CENTER,
                g.short.chars().next().unwrap_or('?'),
                egui::FontId::proportional(12.0),
                colors::TEXT,
            );
        }
    }
}

/// The library search: filters every entry by name or line (`×` clears it).
fn library_search(ui: &mut Ui, mem: &mut LeftMemory, reg: &mut Registry) {
    ui.horizontal(|ui| {
        let clear_w = if mem.library_search.is_empty() {
            0.0
        } else {
            30.0
        };
        let r = ui.add(
            TextEdit::singleline(&mut mem.library_search)
                .id(Id::new("utsim_biblioteca_busca"))
                .hint_text(tx("godot-ui-left-buscar_elemento"))
                .desired_width(ui.available_width() - clear_w),
        );
        named(ui, reg, "Biblioteca_Busca", &r, &mem.library_search.clone());
        if !mem.library_search.is_empty() {
            let x = ui
                .button("×")
                .on_hover_text(tx("godot-ui-left-limpa_a_busca"));
            named(ui, reg, "Biblioteca_Limpar", &x, "×");
            if x.clicked() {
                mem.library_search.clear();
            }
        }
    });
}

/// The entries just armed or dragged go to the front of "Recentes".
fn remember_used(mem: &mut LeftMemory, actions: &[String]) {
    for a in actions {
        let id = a
            .strip_prefix("biblioteca_arrastar:")
            .or_else(|| a.strip_prefix("biblioteca:"));
        if let Some(id) = id {
            mem.recent.retain(|r| r != id);
            mem.recent.insert(0, id.to_string());
            mem.recent.truncate(RECENT_MAX);
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
    let label = entry_label(e);
    let galley = WidgetText::from(RichText::new(&label).color(colors::TEXT)).into_galley(
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
    let tip = fx(
        "godot-ui-left-clique_e_depois_clique_no",
        &[("label", vx(&label))],
    );
    let r = r
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tip);
    let name = format!("Biblioteca_{}", e.id);
    named_toggle(ui, reg, &name, &r, &label, armed);
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
                .hint_text(tx("godot-ui-left-buscar_por_nome_ou_tipo"))
                .desired_width(ui.available_width() - clear_w),
        );
        named(ui, reg, "Objetos_Busca", &r, &mem.search.clone());
        if !mem.search.is_empty() {
            let x = ui
                .button("×")
                .on_hover_text(tx("godot-ui-left-limpa_a_busca"));
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
        fx("godot-ui-left-problema_s", &[("problems", vx(&o.problems))])
    } else if o.warnings > 0 {
        fx("godot-ui-left-aviso_s", &[("warnings", vx(&o.warnings))])
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
        .add_sized(
            [w, 32.0],
            widgets::primary(tx("godot-ui-left-simular_de_novo")),
        )
        .on_hover_text(tx("godot-ui-left-roda_as_replicacoes_outra_vez"));
    named(ui, reg, "Botao_RodarDeNovo", &r, "Simular de novo");
    if r.clicked() {
        out.push("rodar_de_novo".into());
    }
    if !v.status.is_empty() {
        let r = ui.add(Label::new(RichText::new(&v.status).color(colors::TEXT_WEAK)).wrap());
        named(ui, reg, "Simular_Status", &r, &v.status);
    }
    ui.label(RichText::new(tx("godot-ui-left-blocos_clique_para_focar")).color(colors::TEXT));
    // Below the list: the camera, the buttons, the switches and the summary.
    let summary_h = (ui.available_height() * 0.32).clamp(110.0, 220.0);
    let lista_quase = v.seguranca && !v.quase.is_empty();
    let below = 3.0 * 32.0 + summary_h + 4.0 * 6.0 + 30.0 + if lista_quase { 110.0 } else { 0.0 };
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
            .button(tx("godot-ui-left-fluxos"))
            .on_hover_text(tx("godot-ui-left-fluxos_de_tokens_em_2d"));
        named(ui, reg, "Botao_Fluxos", &r, "Fluxos");
        if r.clicked() {
            out.push("analisar:fluxos".into());
        }
        let r = ui
            .button(tx("godot-ui-left-gravar_replay"))
            .on_hover_text(tx("godot-ui-left-grava_o_replay_desta_replicacao"));
        named(ui, reg, "Botao_GravarReplay", &r, "Gravar replay");
        if r.clicked() {
            out.push("gravar_replay".into());
        }
    });
    ui.horizontal(|ui| {
        for (name, text, on, what, tip) in [
            (
                "Simular_Rotulos",
                tx("godot-ui-left-rotulos"),
                v.labels,
                "rotulos",
                tx("godot-ui-left-os_nomes_dos_blocos_sobre"),
            ),
            (
                "Simular_Sombras",
                tx("godot-ui-left-sombras"),
                v.shadows,
                "sombras",
                tx("godot-ui-left-as_sombras_do_sol_desligadas"),
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
    camadas(ui, v, reg, out);
    summary(ui, &v.summary, summary_h, reg);
}

/// A camada de segurança e o mapa de calor (QS6): duas chaves e, com a camada ligada, a
/// lista dos quase-conflitos simulados, cada um um botão que salta a reprodução até ele.
fn camadas(ui: &mut Ui, v: &SimView, reg: &mut Registry, out: &mut Vec<String>) {
    ui.horizontal(|ui| {
        for (name, text, on, what, tip) in [
            (
                "Simular_Seguranca",
                tx("godot-ui-left-seguranca"),
                v.seguranca,
                "seguranca",
                tx("godot-ui-left-zonas_de_risco_epi_das"),
            ),
            (
                "Simular_Calor",
                tx("godot-ui-left-calor"),
                v.calor,
                "calor",
                tx("godot-ui-left-mapa_de_calor_da_presenca"),
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
    if v.seguranca && !v.quase.is_empty() {
        let area = ScrollArea::vertical()
            .id_salt("utsim_quase")
            .max_height(100.0)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.label(
                    RichText::new(fx(
                        "godot-ui-left-quase_conflitos_simulados",
                        &[("quase", vx(&v.quase.len()))],
                    ))
                    .color(colors::TEXT_WEAK),
                );
                for (k, (t, text)) in v.quase.iter().enumerate().take(200) {
                    let r = ui.small_button(text.as_str());
                    named(ui, reg, &format!("Simular_Quase_{k}"), &r, text);
                    if r.clicked() {
                        out.push(format!("quase:{t}"));
                    }
                }
            });
        named_area(ui, reg, "Simular_Quase", area.inner_rect);
    }
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
        ui.label(RichText::new(tx("godot-ui-left-replicacoes")).color(colors::TEXT));
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
            (
                "Simular_Menos",
                "−",
                -1,
                tx("godot-ui-left-uma_replicacao_a_menos"),
            ),
            (
                "Simular_Mais",
                "+",
                1,
                tx("godot-ui-left-uma_replicacao_a_mais"),
            ),
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
        ui.label(RichText::new(tx("godot-ui-left-camera")).color(colors::TEXT));
        let shown = cameras()
            .iter()
            .find(|c| c.0 == v.camera)
            .map(|c| c.1)
            .unwrap_or(tx("godot-ui-left-orbita"));
        let mut pick = reg
            .take_request("Simular_Camera")
            .and_then(|item| cameras().iter().find(|c| c.1 == item).map(|c| c.0));
        let r = ComboBox::from_id_salt("utsim_simular_camera")
            .selected_text(shown)
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                for (id, label) in cameras() {
                    if ui.selectable_label(id == v.camera, label).clicked() {
                        pick = Some(id);
                    }
                }
            });
        let resp = r
            .response
            .on_hover_text(tx("godot-ui-left-seguir_primeira_pessoa_e_do"));
        named_list(
            ui,
            reg,
            "Simular_Camera",
            &resp,
            shown,
            cameras().iter().map(|c| c.1.to_owned()).collect(),
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
                            RichText::new(tx("godot-ui-left-os_resultados_aparecem_aqui_depois"))
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
                            RichText::new(fx(
                                "godot-ui-left-gargalo",
                                &[("bottleneck", vx(&s.bottleneck))],
                            ))
                            .color(BOTTLENECK),
                        )
                        .wrap(),
                    );
                }
                for l in &s.lines {
                    ui.add(Label::new(RichText::new(l).color(colors::TEXT)).wrap());
                }
                ui.add(
                    Label::new(
                        RichText::new(tx("godot-ui-left-a_tabela_completa_os_graficos"))
                            .color(colors::TEXT_WEAK),
                    )
                    .wrap(),
                );
            });
    });
    let mut text = s.title.clone();
    if !s.bottleneck.is_empty() {
        text.push_str(&fx(
            "godot-ui-left-gargalo-2",
            &[("bottleneck", vx(&s.bottleneck))],
        ));
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
                    kind_label: "Espera".into(),
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
            ..SimView::default()
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

    /// The whole library (the core entries and every line's presets), as the bridge
    /// builds it.
    fn full_view() -> EditView {
        let mut e = edit_view();
        e.library
            .extend(crate::linha::entries().into_iter().map(|l| Entry {
                id: l.id,
                label: l.label,
                category: l.category,
                icon: None,
            }));
        e.categories.extend(crate::linha::categories());
        e
    }

    /// With `UTSIM_CAPTURAS=<pasta>`: pictures of the Editar panel with the whole library.
    #[test]
    fn library_pictures() {
        let Some(d) = std::env::var_os("UTSIM_CAPTURAS") else {
            return;
        };
        let d = std::path::PathBuf::from(d);
        std::fs::create_dir_all(&d).unwrap();
        let e = full_view();
        let s = SimView::default();
        for (file, setup) in PICTURES {
            let ctx = Context::default();
            utsim_ui_tema::install(&ctx);
            let size = Vec2::new(WIDTH, 820.0);
            let mut cap = utsim_ui_tema::captura::Captura::new(ctx, size);
            let mut reg = Registry::default();
            let mut mem = LeftMemory::default();
            setup(&mut mem);
            let rect = Rect::from_min_size(Pos2::ZERO, size);
            let img = cap.imagem(3, |ui| {
                reg.begin_frame();
                let mut out = Vec::new();
                show(ui, rect, true, &e, &s, &mut mem, &mut reg, &mut out);
            });
            img.salvar_png(&d.join(file)).unwrap();
        }
    }

    type Setup = fn(&mut LeftMemory);
    const PICTURES: [(&str, Setup); 5] = [
        ("godot_biblioteca_fluxo.png", |_| {}),
        ("godot_biblioteca_escritorio.png", |m| {
            m.group = Some("escritorio".into())
        }),
        ("godot_biblioteca_recentes.png", |m| {
            m.group = Some("recentes".into());
            m.recent = vec![
                "Operator".into(),
                "lean_office/posto".into(),
                "Queue".into(),
            ];
        }),
        ("godot_biblioteca_busca.png", |m| {
            m.library_search = "posto".into()
        }),
        ("godot_biblioteca_recolhida.png", |m| m.folded = true),
    ];

    #[test]
    fn the_rail_shows_one_group_and_folds() {
        let mut p = Panel::new();
        let e = full_view();
        let s = SimView::default();
        p.frame(true, &e, &s, vec![]);
        // The first group is Fluxo: its entries are there, the others are not.
        assert!(p.reg.knows("Biblioteca_Source"));
        assert!(!p.reg.knows("Biblioteca_Operator"));
        for g in &groups() {
            assert!(p.reg.knows(&format!("Biblioteca_Grupo_{}", g.id)));
        }
        let click = |p: &mut Panel, name: &str| {
            let at = p.center(name);
            for ev in [moved(at), press(at, true), press(at, false)] {
                p.frame(true, &e, &s, vec![ev]);
            }
            p.frame(true, &e, &s, vec![]);
        };
        click(&mut p, "Biblioteca_Grupo_movimento");
        assert!(p.reg.knows("Biblioteca_Operator"));
        assert!(!p.reg.knows("Biblioteca_Source"));
        // Arming an entry puts it in Recentes.
        click(&mut p, "Biblioteca_Operator");
        assert_eq!(p.mem.recent, vec!["Operator".to_string()]);
        // A click on the open group folds the library; a group icon opens it again.
        click(&mut p, "Biblioteca_Grupo_movimento");
        assert!(p.mem.folded);
        assert!(!p.reg.knows("Biblioteca_Operator"));
        click(&mut p, "Biblioteca_Grupo_recentes");
        assert!(!p.mem.folded);
        assert!(p.reg.knows("Biblioteca_Operator"));
        // The search looks into every group.
        p.mem.library_search = "posto".into();
        p.frame(true, &e, &s, vec![]);
        assert!(p.reg.knows("Biblioteca_lean_office/posto"));
    }

    #[test]
    fn every_entry_has_a_group() {
        let e = full_view();
        for x in &e.library {
            let g = group_of(x);
            assert!(groups().iter().any(|k| k.id == g), "{}: {g}", x.id);
        }
    }
}
