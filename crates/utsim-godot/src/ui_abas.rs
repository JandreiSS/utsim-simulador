//! The Qualidade, Segurança and Manutenção tabs of a node in the properties panel (owner's
//! direction of 2026-11-08, `docs/plano_integracao_familia.md`, section 3): the fields of
//! `utsim_edit::abas`, the same UTSim Qualidade and Segurança edit, drawn in the panel's
//! two columns. It changes nothing itself: an edit comes out as [`AbaAcao`], which the
//! bridge turns into the `Set*` command of `abas::editar` / `adicionar` / `remover` (one
//! undo step). Read only while another app or person holds the model's lock.
//!
//! Widget names for the scripted runs: `Aba_Qualidade`, `Aba_Seguranca`, `Aba_Manutencao`
//! (the tabs), `AbaCampo_<path>` (a field: dots and brackets as underscores, so
//! `safety.hazards[0].severity` is `AbaCampo_safety_hazards_0_severity`),
//! `AbaAdicionar_<list>`, `AbaRemover_<item>`, `AbaItem_<item>` (opens and closes an item),
//! `AbaTrava` (who edits), `AbaErro` (the last refusal) and `AbaSim_<k>` (line `k` of what
//! the last run says, `indicadores.rs`; `AbaSimNota` above them).

use std::collections::HashSet;

use egui::{Button, Label, RichText, Sense, Ui, Vec2};
use utsim_edit::abas::{Aba, Campo, Tipo};
use utsim_ui_tema::{colors, widgets};

use crate::egui_widgets::Registry;
use crate::ui_props::{
    PropAction, PropsState, PropsView, combo, line_edit, named, named_toggle, row_label,
};

/// What the last run says about the object: the lines of each tab (in the order of
/// `PropsView::abas`) and the note above them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Simulado {
    pub nota: String,
    pub linhas: Vec<Vec<crate::indicadores::Linha>>,
    /// The quality and safety numbers are still being computed.
    pub calculando: bool,
}

/// What the user did in a tab.
#[derive(Clone, Debug, PartialEq)]
pub enum AbaAcao {
    /// Set field `caminho` to `valor` (as typed).
    Editar { caminho: String, valor: String },
    /// Add an item to list `lista` (`safety.hazards`...).
    Adicionar(String),
    /// Remove item `indice` of list `lista`.
    Remover(String, usize),
    /// "Abrir no UTSim Qualidade / Segurança": the app (`qualidade`, `seguranca`).
    AbrirApp(String),
}

/// What the tabs keep between frames.
#[derive(Debug, Default)]
pub struct AbasState {
    /// The tab shown (index in `Aba::TODAS`); kept from one object to the next.
    pub aba: usize,
    /// List items closed by the user (by path).
    fechados: HashSet<String>,
    /// The last refusal: the path and why.
    pub erro: Option<(String, String)>,
}

impl AbasState {
    /// A new object: its items open and no error.
    pub fn outro_objeto(&mut self) {
        self.fechados.clear();
        self.erro = None;
    }
}

/// The widget name of field `caminho`.
pub fn nome_campo(prefixo: &str, caminho: &str) -> String {
    let mut s = String::with_capacity(caminho.len());
    for ch in caminho.chars() {
        match ch {
            '.' | '[' => s.push('_'),
            ']' => {}
            c => s.push(c),
        }
    }
    format!("{prefixo}_{s}")
}

fn nome_aba(a: Aba) -> &'static str {
    match a {
        Aba::Qualidade => "Aba_Qualidade",
        Aba::Seguranca => "Aba_Seguranca",
        Aba::Manutencao => "Aba_Manutencao",
    }
}

/// `safety.hazards[2]` is (`safety.hazards`, 2).
pub fn separar_item(caminho: &str) -> Option<(String, usize)> {
    let i = caminho.rfind('[')?;
    let n = caminho[i + 1..].strip_suffix(']')?.parse().ok()?;
    Some((caminho[..i].to_string(), n))
}

/// Draws the tab strip and the fields of the tab shown.
pub fn show(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
) {
    if v.abas.is_empty() {
        return;
    }
    let n = v.abas.len();
    if st.abas.aba >= n {
        st.abas.aba = 0;
    }
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        let w = ((ui.available_width() - 4.0 * (n as f32 - 1.0)) / n as f32).max(60.0);
        for (k, (aba, _)) in v.abas.iter().enumerate() {
            let on = st.abas.aba == k;
            let r = ui.add_sized(
                [w, 28.0],
                Button::selectable(on, RichText::new(aba.nome()).strong()),
            );
            named_toggle(ui, reg, nome_aba(*aba), &r, aba.nome(), on);
            if r.clicked() && !on {
                st.abas.aba = k;
                st.abas.erro = None;
            }
        }
    });
    if let Some(quem) = &v.somente_leitura {
        let r = ui.add(
            Label::new(
                RichText::new(format!("{quem}. Só leitura até a pessoa fechar o modelo."))
                    .color(colors::WARNING),
            )
            .wrap(),
        );
        named(ui, reg, "AbaTrava", &r, quem);
    }
    if let Some((_, e)) = &st.abas.erro {
        let r = ui.add(Label::new(RichText::new(e).color(colors::ERROR)).wrap());
        named(ui, reg, "AbaErro", &r, e);
    }
    abrir_app(ui, reg, out, v.abas[st.abas.aba].0);
    if let Some(s) = &v.simulado {
        simulado(ui, reg, s, st.abas.aba);
    }
    let editavel = v.somente_leitura.is_none();
    let campos = &v.abas[st.abas.aba].1;
    let mut esconder: Option<u8> = None;
    ui.add_enabled_ui(editavel, |ui| {
        for c in campos {
            if let Some(p) = esconder {
                if c.profundidade > p {
                    continue;
                }
                esconder = None;
            }
            if c.tipo == Tipo::Item && st.abas.fechados.contains(&c.caminho) {
                esconder = Some(c.profundidade);
            }
            campo(ui, st, reg, out, c);
        }
    });
}

/// "Abrir no UTSim Qualidade" on the Qualidade tab, "Abrir no UTSim Segurança" on the
/// Segurança tab (`outro_app.rs`): the app opens on this node of the saved model.
fn abrir_app(ui: &mut Ui, reg: &mut Registry, out: &mut Vec<PropAction>, aba: Aba) {
    let (app, texto, ajuda) = match aba {
        Aba::Qualidade => (
            "qualidade",
            "Abrir no UTSim Qualidade",
            "Abre este nó no UTSim Qualidade (defeitos, inspeção, custos, CEP). O modelo é salvo antes e fica com o outro app para editar; o UTSim 3D relê o que ele gravar.",
        ),
        Aba::Seguranca => (
            "seguranca",
            "Abrir no UTSim Segurança",
            "Abre este nó no UTSim Segurança do Trabalho (perigos, matriz de risco, EPI). O modelo é salvo antes e fica com o outro app para editar; o UTSim 3D relê o que ele gravar.",
        ),
        Aba::Manutencao => return,
    };
    let r = ui
        .add_sized([ui.available_width(), 26.0], Button::new(texto))
        .on_hover_text(ajuda);
    named(ui, reg, &format!("AbaAbrir_{app}"), &r, texto);
    if r.clicked() {
        out.push(PropAction::Aba(AbaAcao::AbrirApp(app.into())));
    }
}

/// The lines of the last run in the tab shown, above the fields.
fn simulado(ui: &mut Ui, reg: &mut Registry, s: &Simulado, aba: usize) {
    let linhas = s.linhas.get(aba).map_or(&[][..], |l| l.as_slice());
    if linhas.is_empty() && !s.calculando {
        return;
    }
    let nota = if linhas.is_empty() {
        "Calculando os números da simulação...".to_owned()
    } else {
        s.nota.clone()
    };
    let r = ui.add(Label::new(RichText::new(&nota).strong().color(colors::HEADING)).wrap());
    named(ui, reg, "AbaSimNota", &r, &nota);
    for (k, l) in linhas.iter().enumerate() {
        row_label(ui, &l.rotulo, &l.ajuda, 0, |ui, _| {
            let r = ui.add(Label::new(RichText::new(&l.valor).monospace()).wrap());
            named(ui, reg, &format!("AbaSim_{k}"), &r, &l.valor);
        });
    }
    ui.separator();
}

fn editar(out: &mut Vec<PropAction>, caminho: &str, valor: String) {
    out.push(PropAction::Aba(AbaAcao::Editar {
        caminho: caminho.to_owned(),
        valor,
    }));
}

fn campo(
    ui: &mut Ui,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    c: &Campo,
) {
    let name = nome_campo("AbaCampo", &c.caminho);
    let depth = c.profundidade;
    match c.tipo {
        // A note ("Este tipo de objeto não tem dados de qualidade.").
        Tipo::Info if c.caminho.is_empty() && c.valor.is_empty() && c.rotulo.ends_with('.') => {
            ui.add(Label::new(RichText::new(&c.rotulo).color(colors::TEXT_WEAK)).wrap());
        }
        Tipo::Info => {
            let shown = if c.valor.is_empty() {
                "nenhuma"
            } else {
                c.valor.as_str()
            };
            row_label(ui, &c.rotulo, &c.ajuda, depth, |ui, _| {
                let r = ui.add(Label::new(RichText::new(shown).color(colors::TEXT_WEAK)).wrap());
                if !c.caminho.is_empty() {
                    named(ui, reg, &name, &r, &c.valor);
                }
            });
        }
        Tipo::Bloco => {
            ui.horizontal(|ui| {
                ui.add_space(14.0 * depth as f32);
                let on = c.valor == "sim";
                let mut v = on;
                let mut r = ui.checkbox(&mut v, RichText::new(&c.rotulo).strong());
                if !c.ajuda.is_empty() {
                    r = r.on_hover_text(&c.ajuda);
                }
                named_toggle(ui, reg, &name, &r, &c.rotulo, on);
                if r.changed() {
                    editar(out, &c.caminho, if v { "sim" } else { "não" }.into());
                }
            });
        }
        Tipo::Booleano => {
            row_label(ui, &c.rotulo, &c.ajuda, depth, |ui, _| {
                let on = c.valor == "sim";
                let mut v = on;
                let r = ui.checkbox(&mut v, "");
                named_toggle(ui, reg, &name, &r, "", on);
                if r.changed() {
                    editar(out, &c.caminho, if v { "sim" } else { "não" }.into());
                }
            });
        }
        Tipo::Escolha => {
            let choices: Vec<(String, String, String)> = c
                .escolhas
                .iter()
                .map(|e| {
                    let label = if e.is_empty() { "(nenhum)" } else { e };
                    (e.clone(), label.to_owned(), String::new())
                })
                .collect();
            row_label(ui, &c.rotulo, &c.ajuda, depth, |ui, w| {
                if let Some(p) = combo(ui, reg, &name, &c.valor, &choices, w - 4.0, &c.ajuda) {
                    editar(out, &c.caminho, p);
                }
            });
        }
        Tipo::Conjunto => {
            let help = if c.escolhas.is_empty() {
                c.ajuda.clone()
            } else {
                format!(
                    "{}{}Separe por vírgulas. Opções: {}.",
                    c.ajuda,
                    if c.ajuda.is_empty() { "" } else { "\n" },
                    c.escolhas.join(", ")
                )
            };
            row_label(ui, &c.rotulo, &help, depth, |ui, w| {
                if let Some(t) = line_edit(ui, st, reg, &name, &c.valor, w - 4.0, false, "nenhum") {
                    editar(out, &c.caminho, t);
                }
            });
        }
        Tipo::Inteiro
        | Tipo::Real
        | Tipo::RealOpcional
        | Tipo::InteiroOpcional
        | Tipo::Texto
        | Tipo::Distribuicao => {
            let mono = c.tipo == Tipo::Distribuicao;
            let hint = match c.tipo {
                Tipo::RealOpcional | Tipo::InteiroOpcional => "vazio: não usa",
                _ => "",
            };
            row_label(ui, &c.rotulo, &c.ajuda, depth, |ui, w| {
                let unit_w = if c.unidade.is_empty() { 0.0 } else { 44.0 };
                let typed = line_edit(
                    ui,
                    st,
                    reg,
                    &name,
                    &c.valor,
                    (w - unit_w - 4.0).max(50.0),
                    mono,
                    hint,
                );
                if !c.unidade.is_empty() {
                    ui.label(RichText::new(&c.unidade).color(colors::TEXT_WEAK));
                }
                if let Some(t) = typed {
                    editar(out, &c.caminho, t);
                }
            });
        }
        Tipo::Lista => {
            ui.horizontal(|ui| {
                ui.add_space(14.0 * depth as f32);
                ui.label(RichText::new(&c.rotulo).strong().color(colors::HEADING));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let r = ui.button("+ Adicionar").on_hover_text(format!(
                        "Acrescenta um item em {}",
                        c.rotulo.to_lowercase()
                    ));
                    named(
                        ui,
                        reg,
                        &nome_campo("AbaAdicionar", &c.caminho),
                        &r,
                        "+ Adicionar",
                    );
                    if r.clicked() {
                        out.push(PropAction::Aba(AbaAcao::Adicionar(c.caminho.clone())));
                    }
                });
            });
        }
        Tipo::Item => {
            let fechado = st.abas.fechados.contains(&c.caminho);
            ui.horizontal(|ui| {
                ui.add_space(14.0 * depth as f32);
                let (rect, r) = ui.allocate_exact_size(Vec2::new(18.0, 22.0), Sense::click());
                widgets::paint_chevron(ui.painter(), rect.center(), !fechado, colors::HEADING);
                let r = r
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text(if fechado {
                        "Abre o item"
                    } else {
                        "Fecha o item"
                    });
                named(ui, reg, &nome_campo("AbaItem", &c.caminho), &r, &c.rotulo);
                if r.clicked() {
                    if fechado {
                        st.abas.fechados.remove(&c.caminho);
                    } else {
                        st.abas.fechados.insert(c.caminho.clone());
                    }
                    ui.ctx().request_repaint();
                }
                let titulo = if c.rotulo.is_empty() {
                    "(sem nome)"
                } else {
                    c.rotulo.as_str()
                };
                ui.label(RichText::new(titulo).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let r = ui
                        .button("Remover")
                        .on_hover_text("Tira este item da lista");
                    named(
                        ui,
                        reg,
                        &nome_campo("AbaRemover", &c.caminho),
                        &r,
                        "Remover",
                    );
                    if r.clicked()
                        && let Some((lista, i)) = separar_item(&c.caminho)
                    {
                        out.push(PropAction::Aba(AbaAcao::Remover(lista, i)));
                    }
                });
            });
        }
    }
    if let Some((p, e)) = &st.abas.erro
        && *p == c.caminho
    {
        ui.horizontal(|ui| {
            ui.add_space(14.0 * depth as f32);
            ui.add(Label::new(RichText::new(e).color(colors::ERROR)).wrap());
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Editor;

    #[test]
    fn widget_names_flatten_the_path() {
        assert_eq!(
            nome_campo("AbaCampo", "safety.hazards[0].severity"),
            "AbaCampo_safety_hazards_0_severity"
        );
        assert_eq!(nome_campo("AbaCampo", "downtime"), "AbaCampo_downtime");
        assert_eq!(
            separar_item("safety.hazards[12]"),
            Some(("safety.hazards".into(), 12))
        );
        assert_eq!(separar_item("safety.hazards"), None);
    }

    #[test]
    fn a_new_node_has_the_three_tabs_empty_and_edits_undo_in_one_step() {
        let mut e = Editor::new("teste");
        let p = e.add("Processor", [0.0; 3]).unwrap();
        let tabs = e.tab_fields(&p);
        assert_eq!(
            tabs.iter().map(|(a, _)| a.nome()).collect::<Vec<_>>(),
            ["Qualidade", "Segurança", "Manutenção"]
        );
        // Empty by default: every block switched off.
        for (_, campos) in &tabs {
            assert!(
                campos
                    .iter()
                    .filter(|c| c.tipo == Tipo::Bloco)
                    .all(|c| c.valor == "não")
            );
        }
        let before = e.model().clone();
        e.tab_edit(&p, "quality", "sim").unwrap();
        e.tab_add(&p, "quality.defects").unwrap();
        e.tab_edit(&p, "quality.defects[0].probability", "0,05")
            .unwrap();
        let q = e.model().objects[0].quality.as_ref().unwrap();
        assert_eq!(q.defects.len(), 1);
        assert!((q.defects[0].probability - 0.05).abs() < 1e-12);
        assert!(
            e.tab_edit(&p, "quality.defects[0].probability", "dois")
                .is_err()
        );
        e.tab_remove(&p, "quality.defects", 0).unwrap();
        assert!(
            e.model().objects[0]
                .quality
                .as_ref()
                .unwrap()
                .defects
                .is_empty()
        );
        for _ in 0..4 {
            assert!(e.doc.undo());
        }
        assert_eq!(*e.model(), before, "four edits, four undo steps");
        // Manutenção shows the breakdowns the node already has.
        e.tab_edit(&p, "downtime", "sim").unwrap();
        let manut = &e.tab_fields(&p)[2].1;
        assert!(manut.iter().any(|c| c.caminho == "downtime.mtbf"));
    }
}
