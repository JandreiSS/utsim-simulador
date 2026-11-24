//! The top bar of the Godot interface, in egui (pure egui: no Godot types, unit tested).
//!
//! Groups, left to right: the brand, *arquivo* (Novo, Abrir, Salvar, Salvar como),
//! *editar* (Desfazer, Refazer, Organizar), *modos* (Editar | Simular and the editing tools
//! as segmented toggles), *vista* (the grid and the shortcuts) and *análise* (Resultados,
//! PERT/CPM and its variants); the model name on the right. When the window is too narrow,
//! whole groups move to the "Mais" menu, least used first ([`fit`]), so nothing is cut.
//!
//! The bar changes nothing itself: it reads a [`BarState`] (set by the main scene) and
//! returns actions (`"novo"`, `"modo:simular"`, `"ferramenta:ligar"`...) that the main scene
//! runs as the old buttons did. Every widget keeps the name of the Godot node it replaced
//! (`Modo_Editar`, `Botao_Novo`...), recorded in the [`Registry`].

use crate::texto::tx;
use egui::{Align, ComboBox, Frame, Layout, Margin, RichText, Stroke, Ui};
use utsim_ui_tema::{colors, sizes, widgets};

use crate::egui_widgets::Registry;

/// Snapping steps of the grid, in metres.
pub const STEPS: [f64; 5] = [0.25, 0.5, 1.0, 2.0, 5.0];

/// Width kept for the model name, at least.
const TITLE_MIN: f32 = 120.0;

/// What the bar shows (the main scene's state).
#[derive(Clone, Debug, PartialEq)]
pub struct BarState {
    /// Editar (true) or Simular.
    pub editing: bool,
    /// The editor's tool: `selecionar`, `ligar`, `rede`, `corredor`, `zona` or `colocar`.
    pub tool: String,
    pub snap: bool,
    pub step: f64,
    pub can_undo: bool,
    pub can_redo: bool,
    /// The model name (with ` *` when it has unsaved changes).
    pub title: String,
    /// The title's tooltip: name and file.
    pub title_tip: String,
    /// The kinds of team and request lines hidden (`equipe`, `transporte`...).
    pub links_hidden: Vec<String>,
}

/// The team and request lines the "Ligações" menu shows or hides: kind and label.
pub fn link_kinds() -> [(&'static str, &'static str); 6] {
    [
        ("equipe", tx("godot-ui-bar-equipe_despachante_membro")),
        ("transporte", tx("godot-ui-bar-transporte_pedido")),
        ("operadores", tx("godot-ui-bar-operadores_pedidos")),
        ("manutencao", tx("godot-ui-bar-equipe_de_manutencao")),
        ("casa", tx("godot-ui-bar-casa")),
        ("pausa", tx("godot-ui-bar-local_de_pausa")),
    ]
}

impl Default for BarState {
    fn default() -> Self {
        BarState {
            editing: true,
            tool: "selecionar".into(),
            snap: true,
            step: 1.0,
            can_undo: false,
            can_redo: false,
            title: String::new(),
            title_tip: String::new(),
            links_hidden: Vec::new(),
        }
    }
}

/// The groups that can move to the "Mais" menu, in bar order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Arquivo,
    Historico,
    Organizar,
    Grade,
    Atalhos,
    Analise,
}

pub const UNITS: [Unit; 6] = [
    Unit::Arquivo,
    Unit::Historico,
    Unit::Organizar,
    Unit::Grade,
    Unit::Atalhos,
    Unit::Analise,
];

/// The order in which groups leave the bar when it is too narrow (first, the least used).
pub const REMOVAL: [Unit; 6] = [
    Unit::Atalhos,
    Unit::Organizar,
    Unit::Arquivo,
    Unit::Grade,
    Unit::Analise,
    Unit::Historico,
];

fn index(u: Unit) -> usize {
    UNITS.iter().position(|x| *x == u).unwrap_or(0)
}

/// Whether a group is in the bar in this mode (the editing groups only in Editar).
pub fn present(u: Unit, editing: bool) -> bool {
    match u {
        Unit::Historico | Unit::Organizar | Unit::Grade => editing,
        _ => true,
    }
}

/// Which groups stay in the bar: `widths` of each group (with its separator), `fixed`
/// the width of what never leaves (brand, modes, title), `avail` the bar's width and
/// `more` the width of the "Mais" button. Groups leave in [`REMOVAL`] order.
pub fn fit(widths: &[f32; 6], shown: [bool; 6], fixed: f32, avail: f32, more: f32) -> [bool; 6] {
    let mut shown = shown;
    let total = |s: &[bool; 6]| -> f32 {
        fixed
            + widths
                .iter()
                .zip(s)
                .filter(|(_, on)| **on)
                .map(|(w, _)| *w)
                .sum::<f32>()
    };
    let mut removed = false;
    for u in REMOVAL {
        let extra = if removed { more } else { 0.0 };
        if total(&shown) + extra <= avail {
            break;
        }
        let k = index(u);
        if shown[k] {
            shown[k] = false;
            removed = true;
        }
    }
    shown
}

/// Widths measured in earlier frames (a group's width is known once it was drawn).
#[derive(Clone, Debug, Default)]
pub struct BarMemory {
    pub widths: [f32; 6],
    pub fixed: f32,
    /// Groups in the bar in the last frame drawn (the rest were in "Mais").
    pub shown: [bool; 6],
}

/// Draws the bar at the top of `ui` and pushes the actions clicked to `out`.
pub fn show(
    ui: &mut Ui,
    s: &BarState,
    mem: &mut BarMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let frame = Frame::new()
        .fill(colors::BAR)
        .inner_margin(Margin::symmetric(10, 0))
        .stroke(Stroke::NONE);
    egui::Panel::top("utsim_barra_superior")
        .exact_size(sizes::BAR_HEIGHT)
        .resizable(false)
        .show_separator_line(true)
        .frame(frame)
        .show(ui, |ui| {
            ui.horizontal_centered(|ui| bar_contents(ui, s, mem, reg, out));
        });
}

fn bar_contents(
    ui: &mut Ui,
    s: &BarState,
    mem: &mut BarMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    ui.spacing_mut().item_spacing.x = 4.0;
    ui.spacing_mut().button_padding = egui::vec2(6.0, 4.0);
    let avail = ui.available_width();
    let mut present_now = [false; 6];
    for (k, u) in UNITS.iter().enumerate() {
        present_now[k] = present(*u, s.editing);
    }
    // Groups never drawn yet count as wide: the first frame shows them all to measure.
    let widths = mem.widths.map(|w| if w > 0.0 { w } else { 0.0 });
    let fixed = if mem.fixed > 0.0 { mem.fixed } else { 0.0 };
    let shown = fit(&widths, present_now, fixed + TITLE_MIN, avail, 80.0);

    let mut fixed_now = 0.0;
    let brand = ui.label(
        RichText::new("UTSim")
            .size(sizes::HEADING + 1.0)
            .strong()
            .color(colors::ACCENT),
    );
    fixed_now += brand.rect.width() + ui.spacing().item_spacing.x;

    for (k, u) in UNITS.iter().enumerate() {
        // The modes sit between "editar" and "vista".
        if *u == Unit::Grade {
            let r = ui.horizontal(|ui| {
                ui.separator();
                modes(ui, s, reg, out);
            });
            fixed_now += r.response.rect.width() + ui.spacing().item_spacing.x;
        }
        if !shown[k] {
            continue;
        }
        let r = ui.horizontal(|ui| {
            if *u != Unit::Arquivo {
                ui.separator();
            } else {
                ui.add_space(4.0);
            }
            unit(ui, *u, s, reg, out);
        });
        mem.widths[k] = r.response.rect.width() + ui.spacing().item_spacing.x;
    }
    let overflow: Vec<Unit> = UNITS
        .iter()
        .enumerate()
        .filter(|(k, _)| present_now[*k] && !shown[*k])
        .map(|(_, u)| *u)
        .collect();
    if !overflow.is_empty() {
        ui.separator();
        let r = ui.menu_button((tx("godot-ui-bar-mais"), widgets::down_arrow()), |ui| {
            ui.set_min_width(220.0);
            for (n, u) in overflow.iter().enumerate() {
                if n > 0 {
                    ui.separator();
                }
                ui.label(
                    RichText::new(unit_title(*u))
                        .small()
                        .color(colors::TEXT_WEAK),
                );
                unit(ui, *u, s, reg, out);
            }
        });
        reg.put("Menu_Mais", &r.response, "Mais");
        r.response
            .on_hover_text(tx("godot-ui-bar-os_grupos_que_nao_cabem"));
    }
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        let title = ui
            .add(egui::Label::new(RichText::new(&s.title).color(colors::TEXT)).truncate())
            .on_hover_text(&s.title_tip);
        reg.put("Titulo", &title, &s.title);
    });
    mem.fixed = fixed_now;
    mem.shown = shown;
    // A group measured for the first time (or grown) may not fit: lay the bar out again in
    // this frame with the new widths (egui runs the pass once more).
    let again = fit(&mem.widths, present_now, mem.fixed + TITLE_MIN, avail, 80.0);
    if again != shown {
        ui.ctx()
            .request_discard("a barra superior mudou de largura");
    }
}

fn unit_title(u: Unit) -> &'static str {
    match u {
        Unit::Arquivo => tx("godot-ui-bar-arquivo"),
        Unit::Historico => tx("godot-ui-bar-editar"),
        Unit::Organizar => tx("godot-ui-bar-organizar"),
        Unit::Grade => tx("godot-ui-bar-grade"),
        Unit::Atalhos => tx("godot-ui-bar-ajuda"),
        Unit::Analise => tx("godot-ui-bar-analise"),
    }
}

/// A plain button with its stable name and tooltip; true when clicked.
fn button(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    text: &str,
    tip: &str,
    enabled: bool,
) -> bool {
    let r = ui
        .add_enabled(enabled, egui::Button::new(text))
        .on_hover_text(tip)
        .on_disabled_hover_text(tip);
    reg.put(name, &r, text);
    r.clicked()
}

fn unit(ui: &mut Ui, u: Unit, s: &BarState, reg: &mut Registry, out: &mut Vec<String>) {
    let mut act = |a: &str| out.push(a.to_owned());
    match u {
        Unit::Arquivo => {
            for (name, text, tip, a) in [
                (
                    "Botao_Novo",
                    tx("godot-ui-bar-novo"),
                    tx("godot-ui-bar-novo_modelo_ctrl_n"),
                    "novo",
                ),
                (
                    "Botao_Abrir",
                    tx("godot-ui-bar-abrir"),
                    tx("godot-ui-bar-abrir_um_modelo_ou_um"),
                    "abrir",
                ),
                (
                    "Botao_Salvar",
                    tx("godot-ui-bar-salvar"),
                    tx("godot-ui-bar-salvar_ctrl_s"),
                    "salvar",
                ),
                (
                    "Botao_SalvarComo",
                    tx("godot-ui-bar-salvar_como"),
                    tx("godot-ui-bar-salvar_com_outro_nome_ctrl"),
                    "salvar_como",
                ),
            ] {
                if button(ui, reg, name, text, tip, true) {
                    act(a);
                    ui.close();
                }
            }
        }
        Unit::Historico => {
            if button(
                ui,
                reg,
                "Botao_Desfazer",
                tx("godot-ui-bar-desfazer"),
                tx("godot-ui-bar-desfazer_ctrl_z"),
                s.can_undo,
            ) {
                act("desfazer");
            }
            if button(
                ui,
                reg,
                "Botao_Refazer",
                tx("godot-ui-bar-refazer"),
                tx("godot-ui-bar-refazer_ctrl_y"),
                s.can_redo,
            ) {
                act("refazer");
            }
        }
        Unit::Organizar => {
            // Inside "Mais" it is a submenu, which has its own arrow.
            let in_menu = egui::containers::menu::is_in_menu(ui);
            let contents = |ui: &mut Ui| {
                ui.set_min_width(230.0);
                let groups: [&[(&str, &str)]; 3] = [
                    &[
                        (tx("godot-ui-bar-alinhar_a_esquerda"), "alinhar:esquerda"),
                        (tx("godot-ui-bar-alinhar_ao_centro_x"), "alinhar:centro_x"),
                        (tx("godot-ui-bar-alinhar_a_direita"), "alinhar:direita"),
                        (tx("godot-ui-bar-alinhar_ao_topo"), "alinhar:topo"),
                        (tx("godot-ui-bar-alinhar_ao_centro_y"), "alinhar:centro_y"),
                        (tx("godot-ui-bar-alinhar_a_base"), "alinhar:base"),
                    ],
                    &[
                        (tx("godot-ui-bar-distribuir_em_x"), "distribuir:x"),
                        (tx("godot-ui-bar-distribuir_em_y"), "distribuir:y"),
                    ],
                    &[
                        (tx("godot-ui-bar-copiar_ctrl_c"), "copiar"),
                        (tx("godot-ui-bar-recortar_ctrl_x"), "recortar"),
                        (tx("godot-ui-bar-colar_ctrl_v"), "colar"),
                        (tx("godot-ui-bar-duplicar_ctrl_d"), "duplicar"),
                        (tx("godot-ui-bar-excluir_del"), "excluir"),
                        (tx("godot-ui-bar-selecionar_tudo_ctrl_a"), "tudo"),
                    ],
                ];
                for (g, items) in groups.iter().enumerate() {
                    if g > 0 {
                        ui.separator();
                    }
                    for (text, what) in *items {
                        let name = format!("Organizar_{}", what.replace(':', "_"));
                        let r = ui.button(*text);
                        reg.put(&name, &r, *text);
                        if r.clicked() {
                            act(&format!("organizar:{what}"));
                            ui.close();
                        }
                    }
                }
            };
            let r = if in_menu {
                ui.menu_button(tx("godot-ui-bar-organizar"), contents)
            } else {
                ui.menu_button(
                    (tx("godot-ui-bar-organizar"), widgets::down_arrow()),
                    contents,
                )
            };
            reg.put("Menu_Organizar", &r.response, "Organizar");
            r.response
                .on_hover_text(tx("godot-ui-bar-alinhar_distribuir_copiar_colar_e"));
        }
        Unit::Grade => {
            let mut on = s.snap;
            let r = ui
                .checkbox(&mut on, tx("godot-ui-bar-grade"))
                .on_hover_text(tx("godot-ui-bar-encaixar_na_grade_ao_por"));
            reg.put_toggle("Grade_Encaixe", &r, "Grade", s.snap);
            if r.changed() {
                act(if on { "grade:ligar" } else { "grade:desligar" });
            }
            let label = |v: f64| format!("{} m", trim(v));
            let choices: Vec<String> = STEPS.iter().map(|v| label(*v)).collect();
            let mut pick: Option<f64> = reg
                .take_request("Grade_Passo")
                .and_then(|item| STEPS.iter().copied().find(|v| label(*v) == item));
            let r = ComboBox::from_id_salt("Grade_Passo")
                .selected_text(label(s.step))
                .width(70.0)
                .show_ui(ui, |ui| {
                    for v in STEPS {
                        let item = ui.selectable_label((v - s.step).abs() < 1e-9, label(v));
                        reg.put(&format!("Grade_Passo_{}", label(v)), &item, label(v));
                        if item.clicked() {
                            pick = Some(v);
                        }
                    }
                });
            reg.put_list("Grade_Passo", &r.response, &label(s.step), choices);
            r.response.on_hover_text(tx("godot-ui-bar-passo_da_grade"));
            if let Some(v) = pick {
                act(&format!("passo:{}", trim(v)));
            }
        }
        Unit::Atalhos => {
            if button(
                ui,
                reg,
                "Botao_Atalhos",
                "?",
                tx("godot-ui-bar-atalhos_f1_a_lista_dos"),
                true,
            ) {
                act("atalhos");
                ui.close();
            }
        }
        Unit::Analise => {
            if button(
                ui,
                reg,
                "Botao_Resultados",
                tx("godot-ui-bar-resultados"),
                tx("godot-ui-bar-tabela_graficos_analises_cenarios_e"),
                true,
            ) {
                act("analisar:resultados");
                ui.close();
            }
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing.x = 1.0;
                if button(
                    ui,
                    reg,
                    "Botao_Pert",
                    tx("godot-ui-bar-pert_cpm"),
                    tx("godot-ui-bar-pert_cpm_da_selecao_com"),
                    true,
                ) {
                    act("analisar:pert_auto");
                    ui.close();
                }
                let r = ui.menu_button(widgets::down_arrow(), |ui| {
                    ui.set_min_width(300.0);
                    for (text, what) in [
                        (tx("godot-ui-bar-resultados"), "resultados"),
                        (tx("godot-ui-bar-pert_cpm_da_selecao"), "pert"),
                        (tx("godot-ui-bar-pert_cpm_de_ate_dois"), "pert_trecho"),
                        (tx("godot-ui-bar-fluxos_de_tokens"), "fluxos"),
                    ] {
                        let r = ui.button(text);
                        reg.put(&format!("Analisar_{what}"), &r, text);
                        if r.clicked() {
                            act(&format!("analisar:{what}"));
                            ui.close();
                        }
                    }
                    ui.separator();
                    links_menu(ui, s, reg, &mut |a: String| act(&a));
                    ui.separator();
                    let r = ui
                        .button(tx("godot-ui-bar-modo_missao_utsim_ensino"))
                        .on_hover_text(tx("godot-ui-bar-abre_as_missoes_do_utsim"));
                    reg.put("Ensino_Abrir", &r, "Modo missão (UTSim Ensino)");
                    if r.clicked() {
                        act("ensino:abrir");
                        ui.close();
                    }
                });
                reg.put("Menu_Analisar", &r.response, "Analisar");
                r.response
                    .on_hover_text(tx("godot-ui-bar-outras_analises_pert_cpm_de"));
            });
        }
    }
}

/// Editar | Simular, and the editing tools (only in Editar), as segmented toggles.
fn modes(ui: &mut Ui, s: &BarState, reg: &mut Registry, out: &mut Vec<String>) {
    let r = widgets::segmented(
        ui,
        &[tx("godot-ui-bar-editar"), tx("godot-ui-bar-simular")],
        Some(if s.editing { 0 } else { 1 }),
    );
    let tips = [
        tx("godot-ui-bar-montar_o_modelo_tab_alterna"),
        tx("godot-ui-bar-rodar_as_replicacoes_e_reproduzir"),
    ];
    for (k, (name, mode)) in [("Modo_Editar", "editar"), ("Modo_Simular", "simular")]
        .iter()
        .enumerate()
    {
        let on = s.editing == (k == 0);
        reg.put_toggle(
            name,
            &r[k],
            if k == 0 {
                tx("godot-ui-bar-editar")
            } else {
                tx("godot-ui-bar-simular")
            },
            on,
        );
        if r[k].clicked() {
            out.push(format!("modo:{mode}"));
        }
        r[k].clone().on_hover_text(tips[k]);
    }
    if !s.editing {
        return;
    }
    ui.add_space(8.0);
    let tools = [
        (
            "Ferramenta_Selecionar",
            tx("godot-ui-bar-selecionar"),
            "selecionar",
            tx("godot-ui-bar-selecionar_e_mover_esc"),
        ),
        (
            "Ferramenta_Ligar",
            tx("godot-ui-bar-ligar"),
            "ligar",
            tx("godot-ui-bar-ligar_arraste_da_origem_ao"),
        ),
        (
            "Ferramenta_Rede",
            tx("godot-ui-bar-rede"),
            "rede",
            tx("godot-ui-bar-rede_de_caminhos_nos_e"),
        ),
        (
            "Ferramenta_Corredor",
            tx("godot-ui-bar-corredor"),
            "corredor",
            tx("godot-ui-bar-corredores_de_circulacao_com_faixas"),
        ),
        (
            "Ferramenta_Zona",
            tx("godot-ui-bar-zona"),
            "zona",
            tx("godot-ui-bar-zonas_de_risco_no_piso"),
        ),
        (
            "Ferramenta_Equipe",
            tx("godot-ui-bar-equipe"),
            "equipe",
            tx("godot-ui-bar-equipe_arraste_do_despachante_a"),
        ),
    ];
    let current = match s.tool.as_str() {
        "ligar" => 1,
        "rede" => 2,
        "corredor" => 3,
        "zona" => 4,
        "equipe" => 5,
        _ => 0,
    };
    let labels: Vec<&str> = tools.iter().map(|t| t.1).collect();
    let r = widgets::segmented(ui, &labels, Some(current));
    for (k, (name, text, tool, tip)) in tools.iter().enumerate() {
        reg.put_toggle(name, &r[k], text, k == current);
        if r[k].clicked() {
            out.push(format!("ferramenta:{tool}"));
        }
        r[k].clone().on_hover_text(*tip);
    }
}

/// The toggles of the team and request lines (ligações) the 3D view draws, inside the
/// "Analisar" menu (present in both modes; the bar has no room for a menu of its own).
fn links_menu(ui: &mut Ui, s: &BarState, reg: &mut Registry, act: &mut dyn FnMut(String)) {
    ui.label(RichText::new(tx("godot-ui-bar-ligacoes_na_vista_3d")).strong());
    for (kind, label) in link_kinds() {
        let mut on = !s.links_hidden.iter().any(|h| h == kind);
        let c = ui.checkbox(&mut on, label);
        reg.put_toggle(&format!("Ligacoes_{kind}"), &c, label, on);
        if c.changed() {
            act(format!(
                "ligacoes:{kind}:{}",
                if on { "ligar" } else { "desligar" }
            ));
        }
    }
    ui.separator();
    let mut on = !s.links_hidden.iter().any(|h| h == "simulacao");
    let label = tx("godot-ui-bar-tambem_durante_a_simulacao");
    let c = ui
        .checkbox(&mut on, label)
        .on_hover_text(tx("godot-ui-bar-as_linhas_tracejadas_entre_operadores"));
    reg.put_toggle("Ligacoes_simulacao", &c, label, on);
    if c.changed() {
        act(format!(
            "ligacoes:simulacao:{}",
            if on { "ligar" } else { "desligar" }
        ));
    }
}

/// A step without trailing zeros (`0.25`, `1`).
fn trim(v: f64) -> String {
    let s = format!("{v}");
    s.strip_suffix(".0").map(str::to_owned).unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Context, Pos2, RawInput, Rect, Vec2};

    #[test]
    fn groups_leave_in_order_and_come_back() {
        let w = [270.0, 160.0, 110.0, 150.0, 80.0, 220.0];
        let all = [true; 6];
        assert_eq!(fit(&w, all, 500.0, 2000.0, 80.0), all);
        // Too narrow by a little: Atalhos goes first, and "Mais" takes its place.
        let total: f32 = w.iter().sum::<f32>() + 500.0;
        let s = fit(&w, all, 500.0, total - 10.0, 80.0);
        assert!(!s[index(Unit::Atalhos)]);
        assert!(s[index(Unit::Arquivo)] && s[index(Unit::Analise)]);
        // Much narrower: more groups go, the history (undo) last.
        let s = fit(&w, all, 500.0, 900.0, 80.0);
        assert!(!s[index(Unit::Organizar)] && !s[index(Unit::Arquivo)]);
        assert!(s[index(Unit::Historico)]);
        // A group absent in this mode is never shown.
        let mut sim = all;
        sim[index(Unit::Grade)] = false;
        assert!(!fit(&w, sim, 0.0, 9999.0, 80.0)[index(Unit::Grade)]);
    }

    fn run(width: f32, state: &BarState, frames: usize) -> (Registry, BarMemory, Vec<String>) {
        let ctx = Context::default();
        utsim_ui_tema::install(&ctx);
        let mut reg = Registry::default();
        let mut mem = BarMemory::default();
        let mut out = Vec::new();
        for _ in 0..frames {
            let input = RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(width, 720.0))),
                ..RawInput::default()
            };
            ctx.run_ui(input, |ui| {
                reg.begin_frame();
                show(ui, state, &mut mem, &mut reg, &mut out);
            })
            .drop_without_applying_deltas();
        }
        (reg, mem, out)
    }

    fn state() -> BarState {
        BarState {
            title: "tutorial_01: um caixa".into(),
            title_tip: "tutorial_01.ron".into(),
            ..BarState::default()
        }
    }

    fn assert_no_overlap(reg: &Registry, width: f32) {
        let names = reg.names();
        let bar: Vec<_> = names
            .iter()
            .filter(|n| {
                !n.starts_with("Grade_Passo_")
                    && !n.starts_with("Organizar_")
                    && !n.starts_with("Analisar_")
            })
            .collect();
        for a in &bar {
            let ra = reg.get(a).unwrap().rect;
            assert!(ra.max.x <= width + 0.5, "{a} sai da barra: {ra:?}");
            assert!(
                ra.max.y <= sizes::BAR_HEIGHT + 0.5,
                "{a} sai da barra: {ra:?}"
            );
            for b in &bar {
                if a < b {
                    let rb = reg.get(b).unwrap().rect;
                    let inter = ra.intersect(rb);
                    assert!(
                        !(inter.width() > 1.0 && inter.height() > 1.0),
                        "{a} {ra:?} cobre {b} {rb:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn everything_fits_at_1600() {
        let (reg, mem, _) = run(1600.0, &state(), 3);
        for name in [
            "Botao_Novo",
            "Botao_Abrir",
            "Botao_Salvar",
            "Botao_SalvarComo",
            "Botao_Desfazer",
            "Botao_Refazer",
            "Menu_Organizar",
            "Modo_Editar",
            "Modo_Simular",
            "Ferramenta_Selecionar",
            "Ferramenta_Ligar",
            "Ferramenta_Rede",
            "Ferramenta_Corredor",
            "Grade_Encaixe",
            "Grade_Passo",
            "Botao_Atalhos",
            "Botao_Resultados",
            "Botao_Pert",
            "Menu_Analisar",
            "Titulo",
        ] {
            assert!(
                reg.get(name).is_some(),
                "{name} não está na barra: {:?}",
                reg.names()
            );
        }
        assert!(reg.get("Menu_Mais").is_none(), "{:?}", mem.shown);
        assert_no_overlap(&reg, 1600.0);
        let title = reg.get("Titulo").unwrap();
        assert!(title.rect.width() >= TITLE_MIN - 1.0, "{:?}", title.rect);
    }

    #[test]
    fn narrow_windows_use_the_more_menu() {
        let (reg, _, _) = run(1280.0, &state(), 3);
        assert!(reg.get("Menu_Mais").is_some(), "{:?}", reg.names());
        // The modes and tools never leave the bar.
        for name in [
            "Modo_Editar",
            "Modo_Simular",
            "Ferramenta_Ligar",
            "Botao_Desfazer",
        ] {
            assert!(reg.get(name).is_some(), "{name}: {:?}", reg.names());
        }
        assert_no_overlap(&reg, 1280.0);
    }

    #[test]
    fn simular_hides_the_editing_groups() {
        let s = BarState {
            editing: false,
            ..state()
        };
        let (reg, _, _) = run(1280.0, &s, 3);
        for name in [
            "Botao_Desfazer",
            "Ferramenta_Ligar",
            "Menu_Organizar",
            "Grade_Encaixe",
        ] {
            assert!(reg.get(name).is_none(), "{name} no Simular");
        }
        assert!(reg.get("Botao_Resultados").is_some());
        assert_eq!(reg.get("Modo_Simular").and_then(|w| w.checked), Some(true));
        assert_eq!(reg.get("Modo_Editar").and_then(|w| w.checked), Some(false));
        assert!(
            reg.get("Menu_Mais").is_none(),
            "no Simular tudo cabe a 1280"
        );
    }

    #[test]
    fn the_step_list_takes_a_scripted_choice() {
        let ctx = Context::default();
        utsim_ui_tema::install(&ctx);
        let mut reg = Registry::default();
        let mut mem = BarMemory::default();
        let mut out = Vec::new();
        let s = state();
        for k in 0..3 {
            if k == 2 {
                assert!(reg.request("Grade_Passo", "0.5 m"));
            }
            ctx.run_ui(RawInput::default(), |ui| {
                reg.begin_frame();
                show(ui, &s, &mut mem, &mut reg, &mut out);
            })
            .drop_without_applying_deltas();
        }
        assert_eq!(out, vec!["passo:0.5".to_owned()]);
    }

    #[test]
    fn steps_print_without_zeros() {
        assert_eq!(trim(1.0), "1");
        assert_eq!(trim(0.25), "0.25");
    }
}
