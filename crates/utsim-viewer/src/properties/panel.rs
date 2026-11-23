//! The egui side of the property editor. Every widget edits a local copy and
//! returns what the user committed; [`Editor`] turns commits into `Command::Set`
//! (and `Rename`) carrying RON text from [`super::ron_text`].

use crate::texto::{fx, tx, vx};
use std::collections::BTreeMap;

use bevy_egui::egui::{self, Color32, Id, Ui, emath::Numeric};
use utsim_core::{Dist, Downtime, OutputRule, TransferDef, TransferPath};
use utsim_edit::{Command, Document};
use utsim_model::experiment::patch;
use utsim_model::{ObjectDef, ObjectKind, VisualDef};

use super::ron_text as ron;
use super::spec::{self, FAMILIES};
use crate::{terceiros, visual};

pub(crate) const RED: Color32 = Color32::from_rgb(225, 80, 80);

// ---------------------------------------------------------------------------
// Transient widget state (egui memory)
// ---------------------------------------------------------------------------

pub(crate) fn temp<T: Clone + Send + Sync + 'static>(ui: &Ui, id: Id) -> Option<T> {
    ui.data(|d| d.get_temp::<T>(id))
}

pub(crate) fn set_temp<T: Clone + Send + Sync + 'static>(ui: &Ui, id: Id, value: T) {
    ui.data_mut(|d| d.insert_temp(id, value));
}

pub(crate) fn clear_temp<T: Clone + Default + Send + Sync + 'static>(ui: &Ui, id: Id) {
    ui.data_mut(|d| d.remove_temp::<T>(id));
}

/// Lists with more entries than this get a search box.
const LONG_LIST: usize = 10;

/// A search box at the top of a long list (drawn only when there are more than
/// [`LONG_LIST`] entries); returns the lower-case text to filter by.
pub(crate) fn list_filter(ui: &mut Ui, id: Id, entries: usize) -> String {
    if entries <= LONG_LIST {
        return String::new();
    }
    let mut text = temp::<String>(ui, id).unwrap_or_default();
    let response = ui.add(
        egui::TextEdit::singleline(&mut text)
            .hint_text(tx("prop-panel-buscar"))
            .desired_width(160.0),
    );
    if response.changed() {
        set_temp(ui, id, text.clone());
    }
    text.trim().to_lowercase()
}

/// Whether `name` passes the lower-case search text `wanted` (empty: everything).
pub(crate) fn matches_filter(name: &str, wanted: &str) -> bool {
    wanted.is_empty() || name.to_lowercase().contains(wanted)
}

/// A number box. Edits live in egui memory until committed (Enter, focus lost or
/// drag released); returns true only at the commit, with `value` updated.
pub(crate) fn num<N: Numeric + Default + PartialEq + Send + Sync + 'static>(
    ui: &mut Ui,
    id: Id,
    value: &mut N,
    speed: f64,
    min: N,
) -> bool {
    let editing = temp::<N>(ui, id);
    let mut shown = editing.unwrap_or(*value);
    let response = ui.add(
        egui::DragValue::new(&mut shown)
            .speed(speed)
            .range(min..=N::MAX),
    );
    if response.changed() {
        set_temp(ui, id, shown);
    }
    let pending = temp::<N>(ui, id).is_some();
    if pending && (response.drag_stopped() || response.lost_focus()) {
        clear_temp::<N>(ui, id);
        if shown != *value && shown.to_f64().is_finite() {
            *value = shown;
            return true;
        }
    }
    false
}

fn float_speed(v: f64) -> f64 {
    (v.abs() * 0.01).max(0.01)
}

/// A float box without lower bound.
pub(crate) fn float(ui: &mut Ui, id: Id, value: &mut f64) -> bool {
    num(ui, id, value, float_speed(*value), f64::MIN)
}

pub(crate) fn labeled_float(ui: &mut Ui, id: Id, label: &str, value: &mut f64) -> bool {
    ui.label(label);
    float(ui, id, value)
}

/// A text box committed on Enter / focus loss, only when `check` accepts it.
/// The error of `check` is shown in red while the text is being edited.
pub(crate) fn text_commit(
    ui: &mut Ui,
    id: Id,
    current: &str,
    multiline: bool,
    check: &dyn Fn(&str) -> Result<(), String>,
) -> Option<String> {
    let editing = temp::<String>(ui, id);
    let mut text = editing.unwrap_or_else(|| current.to_string());
    // Text that does not parse stays in the box (also after the box loses focus or the
    // panel shows another object) and is drawn in red, with the reason below it: the
    // model keeps the previous value until the text is valid.
    let invalid = temp::<String>(ui, id).is_some() && text != current && check(&text).is_err();
    let response = if multiline {
        let mut edit = egui::TextEdit::multiline(&mut text)
            .desired_width(f32::INFINITY)
            .desired_rows(2);
        if invalid {
            edit = edit.text_color(RED);
        }
        ui.add(edit)
    } else {
        let mut edit = egui::TextEdit::singleline(&mut text).desired_width(160.0);
        if invalid {
            edit = edit.text_color(RED);
        }
        ui.add(edit)
    };
    if response.changed() {
        set_temp(ui, id, text.clone());
    }
    let pending = temp::<String>(ui, id).is_some();
    let error = if pending && text != current {
        check(&text).err()
    } else {
        None
    };
    if let Some(error) = &error {
        ui.colored_label(
            RED,
            fx(
                "prop-panel-texto_invalido_nao_aplicado",
                &[("error", vx(&error))],
            ),
        );
        if ui
            .small_button(tx("prop-panel-descartar_texto"))
            .on_hover_text(tx("prop-panel-volta_ao_valor_que_esta"))
            .clicked()
        {
            clear_temp::<String>(ui, id);
            return None;
        }
    }
    if pending && response.lost_focus() {
        if text == current {
            clear_temp::<String>(ui, id);
        } else if error.is_none() {
            clear_temp::<String>(ui, id);
            return Some(text);
        }
    }
    None
}

/// A multi-line text box for user code (scripts, expressions). The text is committed
/// when the box loses focus, even if it does not compile: the compile error from
/// `check` stays visible under the box (the model reports it among the issues), so
/// the modeller can save work in progress. Returns the committed text.
pub(crate) fn script_box(
    ui: &mut Ui,
    id: Id,
    current: &str,
    rows: usize,
    check: &dyn Fn(&str) -> Result<(), String>,
) -> Option<String> {
    let editing = temp::<String>(ui, id);
    let mut text = editing.unwrap_or_else(|| current.to_string());
    let response = ui.add(
        egui::TextEdit::multiline(&mut text)
            .code_editor()
            .desired_width(f32::INFINITY)
            .desired_rows(rows),
    );
    if response.changed() {
        set_temp(ui, id, text.clone());
    }
    if let Err(error) = check(&text) {
        ui.colored_label(RED, error);
    }
    let pending = temp::<String>(ui, id).is_some();
    if pending && response.lost_focus() {
        clear_temp::<String>(ui, id);
        if text != current {
            return Some(text);
        }
    }
    None
}

fn parse_numbers(text: &str) -> Result<Vec<f64>, String> {
    text.split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.replace(',', ".")
                .parse::<f64>()
                .map_err(|_| fx("prop-panel-nao_e_um_numero", &[("s", vx(&s))]))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Distribution widgets (no document access)
// ---------------------------------------------------------------------------

pub(super) fn mean_text(m: f64) -> String {
    if m.is_finite() {
        format!("{}", (m * 1e4).round() / 1e4)
    } else {
        tx("prop-panel-infinita").into()
    }
}

/// Family combo plus the parameter boxes. Returns true when a change was committed.
pub(crate) fn dist_widget(ui: &mut Ui, id: Id, d: &mut Dist) -> bool {
    let mut changed = false;
    let mut index = spec::family_index(d);
    let before = index;
    egui::ComboBox::from_id_salt(id.with("family"))
        .selected_text(tx(FAMILIES[index]))
        .show_ui(ui, |ui| {
            // Tight rows, so the whole list fits below the box on a normal screen.
            ui.spacing_mut().item_spacing.y = 1.0;
            for (i, name) in FAMILIES.iter().enumerate() {
                // The list is long: headings tell the groups apart at a glance.
                if let Some(group) = spec::family_group(i) {
                    ui.weak(group);
                }
                ui.selectable_value(&mut index, i, tx(name));
            }
        });
    if index != before {
        *d = spec::switch_family(d, index);
        changed = true;
    }

    match d {
        Dist::Discrete(table) => {
            let mut remove = None;
            for (i, (value, weight)) in table.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    changed |= labeled_float(ui, id.with(("v", i)), tx("prop-panel-valor"), value);
                    changed |= labeled_float(ui, id.with(("w", i)), tx("prop-panel-peso"), weight);
                    if ui.small_button(tx("prop-panel-remover")).clicked() {
                        remove = Some(i);
                    }
                });
            }
            if let Some(i) = remove {
                table.remove(i);
                changed = true;
            }
            if ui.small_button(tx("prop-panel-adicionar_valor")).clicked() {
                let next = table.last().map_or(0.0, |(v, _)| v + 1.0);
                table.push((next, 1.0));
                changed = true;
            }
        }
        Dist::Empirical(xs) => {
            ui.label(tx("prop-panel-observacoes_em_ordem_crescente_separadas"));
            let current = xs
                .iter()
                .map(|x| ron::float(*x))
                .collect::<Vec<_>>()
                .join(" ");
            let check = |t: &str| parse_numbers(t).map(|_| ());
            if let Some(text) = text_commit(ui, id.with("obs"), &current, true, &check)
                && let Ok(values) = parse_numbers(&text)
            {
                *xs = values;
                changed = true;
            }
        }
        Dist::Shifted { offset, dist } => {
            ui.horizontal(|ui| {
                changed |=
                    labeled_float(ui, id.with("offset"), tx("prop-panel-deslocamento"), offset);
            });
            ui.indent(id.with("inner"), |ui| {
                changed |= dist_widget(ui, id.with("inner"), dist);
            });
        }
        _ => {
            ui.horizontal_wrapped(|ui| {
                changed |= simple_params(ui, id, d);
            });
        }
    }
    changed
}

/// Parameter boxes of the families that only have scalar parameters.
fn simple_params(ui: &mut Ui, id: Id, d: &mut Dist) -> bool {
    let mut changed = false;
    let mut f = |ui: &mut Ui, name: &str, v: &mut f64| {
        changed |= labeled_float(ui, id.with(name), name, v);
    };
    match d {
        Dist::Const(v) => f(ui, tx("prop-panel-valor"), v),
        Dist::Uniform { min, max } => {
            f(ui, tx("prop-panel-min"), min);
            f(ui, tx("prop-panel-max"), max);
        }
        Dist::Exp { mean } | Dist::Poisson { mean } => f(ui, tx("prop-panel-media"), mean),
        Dist::Triangular { min, mode, max } => {
            f(ui, tx("prop-panel-min"), min);
            f(ui, tx("prop-panel-moda"), mode);
            f(ui, tx("prop-panel-max"), max);
        }
        Dist::Normal { mean, sd } | Dist::LogNormal { mean, sd } => {
            f(ui, tx("prop-panel-media"), mean);
            f(ui, tx("prop-panel-desvio"), sd);
        }
        Dist::Gamma { shape, scale }
        | Dist::Weibull { shape, scale }
        | Dist::LogLogistic { shape, scale }
        | Dist::PearsonV { shape, scale } => {
            f(ui, tx("prop-panel-forma"), shape);
            f(ui, tx("prop-panel-escala"), scale);
        }
        Dist::Beta {
            alpha,
            beta,
            min,
            max,
        } => {
            f(ui, tx("prop-panel-alfa"), alpha);
            f(ui, tx("prop-panel-beta"), beta);
            f(ui, tx("prop-panel-min"), min);
            f(ui, tx("prop-panel-max"), max);
        }
        Dist::PearsonVI {
            shape1,
            shape2,
            scale,
        } => {
            f(ui, tx("prop-panel-forma_1"), shape1);
            f(ui, tx("prop-panel-forma_2"), shape2);
            f(ui, tx("prop-panel-escala"), scale);
        }
        Dist::Geometric { p } => f(ui, "p", p),
        Dist::NegBinomial { r, p } => {
            f(ui, "r", r);
            f(ui, "p", p);
        }
        Dist::Erlang { .. } | Dist::Binomial { .. } => {}
        Dist::Discrete(_) | Dist::Empirical(_) | Dist::Shifted { .. } => {}
    }
    match d {
        Dist::Erlang { k, mean } => {
            ui.label("k");
            changed |= num(ui, id.with("k"), k, 0.1, 1);
            changed |= labeled_float(ui, id.with("mean"), tx("prop-panel-media"), mean);
        }
        Dist::Binomial { n, p } => {
            ui.label("n");
            changed |= num(ui, id.with("n"), n, 0.1, 1);
            changed |= labeled_float(ui, id.with("p"), "p", p);
        }
        _ => {}
    }
    changed
}

/// Mean and validation error of a distribution.
pub(crate) fn dist_info(ui: &mut Ui, d: &Dist) {
    match d.validate() {
        Ok(()) => {
            ui.weak(fx(
                "prop-panel-media-2",
                &[("mean", vx(&mean_text(d.mean())))],
            ));
        }
        Err(e) => {
            ui.colored_label(RED, e);
        }
    }
}

/// A titled distribution editor; returns the new value when committed.
pub(super) fn dist_block(ui: &mut Ui, id: Id, title: &str, current: &Dist) -> Option<Dist> {
    let mut d = current.clone();
    let mut changed = false;
    ui.group(|ui| {
        ui.strong(title);
        changed = dist_widget(ui, id, &mut d);
        dist_info(ui, &d);
    });
    changed.then_some(d)
}

// ---------------------------------------------------------------------------
// Editor
// ---------------------------------------------------------------------------

/// Current RON text of a field of the edited owner.
type Reader<'a> = Box<dyn Fn(&str) -> Option<String> + 'a>;

pub(super) struct Editor<'a> {
    pub(super) doc: &'a Document,
    /// Object name, or "run".
    pub(super) owner: String,
    pub(super) read: Reader<'a>,
    pub(super) out: Vec<Command>,
}

impl Editor<'_> {
    pub(super) fn id(&self, field: &str) -> Id {
        Id::new(("utsim-props", &self.owner, field))
    }

    pub(super) fn key(&self, field: &str) -> String {
        format!("{}.{field}", self.owner)
    }

    /// Emits `Set` only when the text differs from what the model holds.
    pub(super) fn set(&mut self, field: &str, text: String) {
        if (self.read)(field).as_deref() != Some(text.as_str()) {
            self.out.push(Command::Set {
                key: self.key(field),
                value: text,
            });
        }
    }

    /// Whether the document would accept `text` for `field`.
    pub(super) fn check(&self, field: &str, text: &str) -> Result<(), String> {
        let mut model = self.doc.model().clone();
        patch(&mut model, &self.key(field), text).map_err(|e| e.to_string())
    }

    fn float_field(&mut self, ui: &mut Ui, label: &str, field: &str, current: f64) {
        let mut v = current;
        ui.horizontal(|ui| {
            if labeled_float(ui, self.id(field), label, &mut v) {
                self.set(field, ron::float(v));
            }
        });
    }

    fn uint_field<N: Numeric + Default + PartialEq + Send + Sync + std::fmt::Display + 'static>(
        &mut self,
        ui: &mut Ui,
        label: &str,
        field: &str,
        current: N,
        min: N,
    ) {
        let mut v = current;
        ui.horizontal(|ui| {
            ui.label(label);
            if num(ui, self.id(field), &mut v, 0.1, min) {
                self.set(field, v.to_string());
            }
        });
    }

    fn opt_uint_field(
        &mut self,
        ui: &mut Ui,
        label: &str,
        field: &str,
        current: Option<u64>,
        default: u64,
    ) {
        ui.horizontal(|ui| {
            let mut on = current.is_some();
            if ui.checkbox(&mut on, label).changed() {
                self.set(
                    field,
                    if on {
                        default.to_string()
                    } else {
                        "None".into()
                    },
                );
            }
            if let Some(mut v) = current {
                if num(ui, self.id(field), &mut v, 0.1, 1) {
                    self.set(field, v.to_string());
                }
            } else {
                ui.weak(tx("prop-panel-sem_limite"));
            }
        });
    }

    fn bool_field(&mut self, ui: &mut Ui, label: &str, field: &str, current: bool) {
        let mut v = current;
        if ui.checkbox(&mut v, label).changed() {
            self.set(field, v.to_string());
        }
    }

    /// A number that can be switched off (`None`).
    fn opt_float_field(
        &mut self,
        ui: &mut Ui,
        label: &str,
        field: &str,
        current: Option<f64>,
        default: f64,
    ) {
        ui.horizontal(|ui| {
            let mut on = current.is_some();
            if ui.checkbox(&mut on, label).changed() {
                self.set(
                    field,
                    if on {
                        ron::float(default)
                    } else {
                        "None".into()
                    },
                );
            }
            if let Some(mut v) = current {
                if float(ui, self.id(field), &mut v) {
                    self.set(field, ron::float(v));
                }
            } else {
                ui.weak(tx("prop-panel-sem_limite_velocidade_constante"));
            }
        });
    }

    /// Dispatcher team: one checkbox per transporter of the model. Members that no
    /// longer exist are listed in red so they can be removed.
    fn team_field(&mut self, ui: &mut Ui, field: &str, current: &[String]) {
        let objects = &self.doc.model().objects;
        let candidates: Vec<String> = spec::transporter_names(objects)
            .into_iter()
            .map(str::to_string)
            .collect();
        let mut team = current.to_vec();
        let mut changed = false;
        ui.label(tx("prop-panel-equipe_transportadores_do_modelo"));
        if candidates.is_empty() {
            ui.weak(tx("prop-panel-nenhum_transportador_no_modelo_crie"));
        }
        for name in &candidates {
            let mut on = team.contains(name);
            if ui.checkbox(&mut on, name).changed() {
                if on {
                    team.push(name.clone());
                } else {
                    team.retain(|m| m != name);
                }
                changed = true;
            }
        }
        for missing in current.iter().filter(|m| !candidates.contains(m)) {
            ui.horizontal(|ui| {
                ui.colored_label(
                    RED,
                    fx(
                        "prop-panel-nao_e_um_transportador",
                        &[("missing", vx(&missing))],
                    ),
                );
                if ui.small_button(tx("prop-panel-remover")).clicked() {
                    team.retain(|m| m != missing);
                    changed = true;
                }
            });
        }
        if team.is_empty() {
            ui.colored_label(RED, tx("prop-panel-a_equipe_nao_pode_ficar"));
        }
        if changed {
            self.set(field, ron::strings(&team));
        }
    }

    /// Processor `operators`: team (transporter or dispatcher), count and phases.
    fn operators_field(
        &mut self,
        ui: &mut Ui,
        field: &str,
        current: Option<(String, usize, bool, bool, Option<String>)>,
    ) {
        let id = self.id(field);
        let candidates: Vec<String> = spec::mobile_names(&self.doc.model().objects)
            .into_iter()
            .map(str::to_string)
            .collect();
        let mut on = current.is_some();
        if ui
            .add_enabled(
                on || !candidates.is_empty(),
                egui::Checkbox::new(&mut on, tx("prop-panel-precisa_de_operadores")),
            )
            .changed()
        {
            let text = if on {
                ron::operators(&candidates[0], 1, true, true)
            } else {
                "None".into()
            };
            self.set(field, text);
        }
        let Some((team, mut count, mut setup, mut process, requires)) = current else {
            if candidates.is_empty() {
                ui.weak(tx("prop-panel-crie_um_transportador_ou_despachante"));
            }
            return;
        };
        let mut new_team = team.clone();
        let mut changed = false;
        ui.indent(id, |ui| {
            ui.horizontal(|ui| {
                ui.label(tx("prop-panel-equipe"));
                egui::ComboBox::from_id_salt(id.with("team"))
                    .selected_text(if team.is_empty() {
                        tx("prop-panel-nenhuma")
                    } else {
                        &team
                    })
                    .show_ui(ui, |ui| {
                        let wanted = list_filter(ui, id.with("team_busca"), candidates.len());
                        for name in candidates.iter().filter(|n| matches_filter(n, &wanted)) {
                            if ui.selectable_label(*name == team, name).clicked() {
                                new_team = name.clone();
                                changed = true;
                            }
                        }
                    });
            });
            if !candidates.contains(&team) {
                ui.colored_label(
                    RED,
                    fx(
                        "prop-panel-nao_e_um_transportador_nem",
                        &[("team", vx(&team))],
                    ),
                );
            }
            ui.horizontal(|ui| {
                ui.label(tx("prop-panel-quantidade"));
                changed |= num(ui, id.with("count"), &mut count, 0.1, 1);
            });
            changed |= ui
                .checkbox(&mut setup, tx("prop-panel-durante_a_preparacao"))
                .changed();
            changed |= ui
                .checkbox(&mut process, tx("prop-panel-durante_o_processamento"))
                .changed();
        });
        if changed {
            self.set(
                field,
                ron::operators_requiring(&new_team, count, setup, process, requires.as_deref()),
            );
        }
    }

    /// `transport`: which transporter/dispatcher carries items out of this object.
    fn transport_field(&mut self, ui: &mut Ui, field: &str, current: Option<&str>) {
        let id = self.id(field);
        let candidates: Vec<String> = spec::mobile_names(&self.doc.model().objects)
            .into_iter()
            .map(str::to_string)
            .collect();
        ui.horizontal(|ui| {
            ui.label(tx("prop-panel-transporte_de_saida"));
            egui::ComboBox::from_id_salt(self.id(field))
                .selected_text(current.unwrap_or(tx("prop-panel-nenhum")))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(current.is_none(), tx("prop-panel-nenhum"))
                        .clicked()
                    {
                        self.set(field, "None".into());
                    }
                    let wanted = list_filter(ui, id.with("busca"), candidates.len());
                    for name in candidates.iter().filter(|n| matches_filter(n, &wanted)) {
                        if ui
                            .selectable_label(current == Some(name.as_str()), name)
                            .clicked()
                        {
                            self.set(field, ron::string(name));
                        }
                    }
                });
        });
        match current {
            None => {
                ui.weak(tx("prop-panel-os_itens_passam_direto_ao"));
            }
            Some(t) if !candidates.iter().any(|c| c == t) => {
                ui.colored_label(
                    RED,
                    fx("prop-panel-nao_e_um_transportador_nem-2", &[("t", vx(&t))]),
                );
            }
            Some(_) => {}
        }
    }

    /// Base of a transporter on the path network: choosing a node moves the transporter
    /// onto it and turns `home` on (it starts there and goes back there when idle), in
    /// one undo step. The current base is the node at the transporter's position.
    fn home_node_field(&mut self, ui: &mut Ui, obj: &ObjectDef) {
        let Some(net) = &self.doc.model().network else {
            ui.weak(tx("prop-panel-sem_rede_de_caminhos_a"));
            return;
        };
        let at = net
            .nodes
            .iter()
            .find(|n| utsim_core::task::distance(n.position, obj.position) < 1e-6)
            .map(|n| n.name.clone());
        let mut chosen = None;
        ui.horizontal(|ui| {
            ui.label(tx("prop-panel-base_no_da_rede"));
            egui::ComboBox::from_id_salt(self.id("#base"))
                .selected_text(at.as_deref().unwrap_or(tx("prop-panel-fora_da_rede")))
                .show_ui(ui, |ui| {
                    let wanted = list_filter(ui, self.id("#base_busca"), net.nodes.len());
                    for n in net
                        .nodes
                        .iter()
                        .filter(|n| matches_filter(&n.name, &wanted))
                    {
                        let on = at.as_deref() == Some(n.name.as_str());
                        if ui.selectable_label(on, &n.name).clicked() && !on {
                            chosen = Some(n.position);
                        }
                    }
                });
        });
        if let Some(position) = chosen {
            self.out.push(Command::Batch(vec![
                Command::Move {
                    name: self.owner.clone(),
                    position,
                },
                Command::Set {
                    key: self.key("home"),
                    value: "true".into(),
                },
            ]));
        }
        ui.weak(tx("prop-panel-escolher_um_no_poe_o"));
    }

    /// Combo over `(ron text, label)` options; `current` is the RON text of the value.
    fn choice_field(
        &mut self,
        ui: &mut Ui,
        label: &str,
        field: &str,
        current: &str,
        options: &[(&str, &str)],
    ) {
        let shown = options
            .iter()
            .find(|(ron, _)| *ron == current)
            .map_or(current, |(_, l)| *l);
        ui.horizontal(|ui| {
            ui.label(label);
            egui::ComboBox::from_id_salt(self.id(field))
                .selected_text(shown)
                .show_ui(ui, |ui| {
                    for (ron, l) in options {
                        if ui.selectable_label(*ron == current, *l).clicked() {
                            self.set(field, (*ron).to_string());
                        }
                    }
                });
        });
    }

    /// Distribution field with the "edit as text" toggle.
    fn dist_field(&mut self, ui: &mut Ui, label: &str, field: &str, current: &Dist) {
        let id = self.id(field);
        let raw_id = id.with("raw");
        let mut raw = temp::<bool>(ui, raw_id).unwrap_or(false);
        let mut new_text = None;
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong(label);
                // The pending text is kept when switching modes, so invalid text is not lost.
                if ui
                    .checkbox(&mut raw, tx("prop-panel-editar_como_texto"))
                    .changed()
                {
                    set_temp(ui, raw_id, raw);
                }
            });
            if raw {
                let check = |t: &str| self.check(field, t);
                new_text = text_commit(ui, id.with("text"), &ron::dist(current), true, &check);
            } else {
                let mut d = current.clone();
                if dist_widget(ui, id, &mut d) {
                    new_text = Some(ron::dist(&d));
                    // The boxes won over any text left pending in the text mode.
                    clear_temp::<String>(ui, id.with("text"));
                }
            }
            dist_info(ui, current);
        });
        if let Some(text) = new_text {
            self.set(field, text);
        }
    }

    fn opt_dist_field(
        &mut self,
        ui: &mut Ui,
        label: &str,
        field: &str,
        current: Option<&Dist>,
        default: Dist,
    ) {
        let mut on = current.is_some();
        if ui.checkbox(&mut on, label).changed() {
            let text = if on {
                ron::dist(&default)
            } else {
                "None".into()
            };
            self.set(field, text);
        }
        if let Some(d) = current {
            self.dist_field(ui, label, field, d);
        }
    }

    fn components_field(&mut self, ui: &mut Ui, field: &str, current: &[usize]) {
        let id = self.id(field);
        let mut list = current.to_vec();
        let mut changed = false;
        let mut remove = None;
        ui.label(tx("prop-panel-quantidade_de_itens_exigida_em"));
        for (i, c) in list.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(fx("prop-panel-entrada", &[("i", vx(&(i + 1)))]));
                changed |= num(ui, id.with(i), c, 0.1, 1);
                if ui.small_button(tx("prop-panel-remover")).clicked() {
                    remove = Some(i);
                }
            });
        }
        if let Some(i) = remove {
            list.remove(i);
            changed = true;
        }
        if ui
            .small_button(tx("prop-panel-adicionar_entrada"))
            .clicked()
        {
            list.push(1);
            changed = true;
        }
        ui.weak(fx(
            "prop-panel-o_objeto_precisa_de_ligacoes",
            &[("list", vx(&(list.len() + 1))), ("list2", vx(&list.len()))],
        ));
        if changed {
            self.set(field, ron::usizes(&list));
        }
    }

    pub(super) fn labels_field(
        &mut self,
        ui: &mut Ui,
        field: &str,
        current: &BTreeMap<String, Dist>,
    ) {
        let id = self.id(field);
        let mut map = current.clone();
        let mut changed = false;
        let mut remove = None;
        for (name, d) in current {
            let sub = id.with(name);
            egui::CollapsingHeader::new(name)
                .id_salt(sub)
                .default_open(true)
                .show(ui, |ui| {
                    if let Some(new) = dist_block(ui, sub, tx("prop-panel-valor_do_rotulo"), d) {
                        map.insert(name.clone(), new);
                        changed = true;
                    }
                    if ui.small_button(tx("prop-panel-remover_rotulo")).clicked() {
                        remove = Some(name.clone());
                    }
                });
        }
        if let Some(name) = remove {
            map.remove(&name);
            changed = true;
        }
        ui.horizontal(|ui| {
            let name_id = id.with("new-name");
            let mut name = temp::<String>(ui, name_id).unwrap_or_default();
            let response = ui.add(
                egui::TextEdit::singleline(&mut name)
                    .hint_text(tx("prop-panel-nome_do_rotulo"))
                    .desired_width(120.0),
            );
            if response.changed() {
                set_temp(ui, name_id, name.clone());
            }
            let name = name.trim().to_string();
            let free = !name.is_empty() && !map.contains_key(&name);
            if ui
                .add_enabled(free, egui::Button::new(tx("prop-panel-adicionar_rotulo")))
                .clicked()
            {
                map.insert(name, Dist::Const(1.0));
                clear_temp::<String>(ui, name_id);
                changed = true;
            }
        });
        if changed {
            self.set(field, ron::labels(&map));
        }
    }

    /// `current` is `(size, max_wait, flush)` of the queue's batch, if it has one.
    fn batch_field(&mut self, ui: &mut Ui, field: &str, current: Option<(usize, f64, bool)>) {
        let id = self.id(field);
        let mut on = current.is_some();
        if ui
            .checkbox(&mut on, tx("prop-panel-lote_agrupar_itens"))
            .changed()
        {
            self.set(
                field,
                if on {
                    ron::batch(2, 0.0, false)
                } else {
                    "None".into()
                },
            );
        }
        let Some((mut size, mut max_wait, mut flush)) = current else {
            return;
        };
        let mut changed = false;
        ui.indent(id, |ui| {
            ui.horizontal(|ui| {
                ui.label(tx("prop-panel-tamanho_do_lote"));
                changed |= num(ui, id.with("size"), &mut size, 0.1, 1);
            });
            ui.horizontal(|ui| {
                changed |= labeled_float(
                    ui,
                    id.with("wait"),
                    tx("prop-panel-espera_maxima_0_esperar_sempre"),
                    &mut max_wait,
                );
            });
            if ui
                .checkbox(&mut flush, tx("prop-panel-fechar_a_entrada_ate_o"))
                .changed()
            {
                changed = true;
            }
        });
        if changed {
            self.set(field, ron::batch(size, max_wait, flush));
        }
    }

    fn output_field(&mut self, ui: &mut Ui, obj: &ObjectDef) {
        let rules: [&str; 6] = [
            tx("prop-panel-primeira_livre"),
            tx("prop-panel-rodizio"),
            tx("prop-panel-aleatoria"),
            tx("prop-panel-pesos"),
            tx("prop-panel-por_rotulo"),
            tx("prop-panel-script"),
        ];
        let field = "output";
        let id = self.id(field);
        let n_out = self
            .doc
            .model()
            .connections
            .iter()
            .filter(|(from, _)| *from == obj.name)
            .count();
        let current = &obj.output;
        let index = match current {
            OutputRule::FirstAvailable => 0,
            OutputRule::RoundRobin => 1,
            OutputRule::Random => 2,
            OutputRule::Weighted(_) => 3,
            OutputRule::ByLabel(_) => 4,
            OutputRule::Script(_) => 5,
        };
        let mut picked = index;
        ui.horizontal(|ui| {
            ui.label(tx("prop-panel-regra_de_saida"));
            egui::ComboBox::from_id_salt(id.with("rule"))
                .selected_text(rules[index])
                .show_ui(ui, |ui| {
                    for (i, name) in rules.iter().enumerate() {
                        ui.selectable_value(&mut picked, i, *name);
                    }
                });
        });
        if picked != index {
            let rule = match picked {
                0 => OutputRule::FirstAvailable,
                1 => OutputRule::RoundRobin,
                2 => OutputRule::Random,
                3 => OutputRule::Weighted(vec![1.0; n_out]),
                4 => OutputRule::ByLabel(self.first_label()),
                // A script returning the first port; edited as text (phase 6 UI front).
                _ => OutputRule::Script("1".into()),
            };
            self.set(field, ron::output(&rule));
        }
        match current {
            OutputRule::Weighted(weights) => {
                let mut w = weights.clone();
                let mut changed = false;
                ui.label(tx("prop-panel-peso_de_cada_saida_na"));
                // The destination of each port, so the weights are not read blindly.
                let dests: Vec<&String> = self
                    .doc
                    .model()
                    .connections
                    .iter()
                    .filter(|(from, _)| *from == obj.name)
                    .map(|(_, to)| to)
                    .collect();
                for (i, weight) in w.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        let label = match dests.get(i) {
                            Some(to) => {
                                fx("prop-panel-saida", &[("i", vx(&(i + 1))), ("to", vx(&to))])
                            }
                            None => fx("prop-panel-saida-2", &[("i", vx(&(i + 1)))]),
                        };
                        changed |= labeled_float(ui, id.with(("w", i)), &label, weight);
                    });
                }
                if w.len() != n_out {
                    ui.colored_label(
                        RED,
                        fx(
                            "prop-panel-ha_pesos_mas_o_objeto",
                            &[("w", vx(&w.len())), ("n_out", vx(&n_out))],
                        ),
                    );
                    if ui
                        .small_button(tx("prop-panel-ajustar_ao_numero_de_saidas"))
                        .clicked()
                    {
                        w.resize(n_out, 1.0);
                        changed = true;
                    }
                }
                if changed {
                    self.set(field, ron::output(&OutputRule::Weighted(w)));
                }
            }
            OutputRule::ByLabel(label) => {
                ui.horizontal(|ui| {
                    ui.label(tx("prop-panel-rotulo_com_o_numero_da"));
                    let ok = |t: &str| {
                        if t.trim().is_empty() {
                            Err(tx("prop-panel-o_nome_do_rotulo_nao").to_string())
                        } else {
                            Ok(())
                        }
                    };
                    if let Some(text) = text_commit(ui, id.with("label"), label, false, &ok) {
                        self.set(field, ron::output(&OutputRule::ByLabel(text)));
                    }
                });
            }
            OutputRule::Script(src) => {
                ui.label(tx("prop-panel-script_que_devolve_o_numero"));
                let check = |t: &str| utsim_script::compile_port_script(t).map(|_| ());
                if let Some(text) = script_box(ui, id.with("script"), src, 4, &check) {
                    self.set(field, ron::output(&OutputRule::Script(text)));
                }
            }
            _ => {}
        }
    }

    /// `on_entry`: optional script run for every item entering the object.
    fn on_entry_field(&mut self, ui: &mut Ui, field: &str, current: Option<&str>) {
        let id = self.id(field);
        let mut on = current.is_some();
        if ui
            .checkbox(&mut on, tx("prop-panel-script_ao_entrar_um_item"))
            .changed()
        {
            self.set(
                field,
                if on {
                    ron::string("")
                } else {
                    "None".to_string()
                },
            );
        }
        if let Some(src) = current {
            ui.weak(tx("prop-panel-roda_para_cada_item_que"));
            let check = |t: &str| utsim_script::compile_item_script(t).map(|_| ());
            if let Some(text) = script_box(ui, id.with("script"), src, 5, &check) {
                self.set(field, ron::string(&text));
            }
        }
    }

    /// A label name defined by some Source, as a starting point for `ByLabel`.
    fn first_label(&self) -> String {
        self.doc
            .model()
            .objects
            .iter()
            .find_map(|o| match &o.kind {
                ObjectKind::Source { labels, .. } => labels.keys().next().cloned(),
                _ => None,
            })
            .unwrap_or_else(|| "tipo".into())
    }

    fn downtime_field(&mut self, ui: &mut Ui, current: Option<&utsim_model::DowntimeDef>) {
        let field = "downtime";
        let id = self.id(field);
        let mut on = current.is_some();
        if ui
            .checkbox(&mut on, tx("prop-panel-quebras_mtbf_mttr"))
            .changed()
        {
            let default = Downtime {
                mtbf: Dist::Exp { mean: 100.0 },
                mttr: Dist::Exp { mean: 10.0 },
            };
            self.set(
                field,
                if on {
                    ron::downtime(&default)
                } else {
                    "None".into()
                },
            );
        }
        let Some(dt) = current else { return };
        let mut new = dt.clone();
        let mut changed = false;
        ui.indent(id, |ui| {
            if let Some(d) = dist_block(
                ui,
                id.with("mtbf"),
                tx("prop-panel-tempo_entre_falhas_mtbf"),
                &dt.mtbf,
            ) {
                new.mtbf = d;
                changed = true;
            }
            if let Some(d) = dist_block(
                ui,
                id.with("mttr"),
                tx("prop-panel-tempo_de_reparo_mttr"),
                &dt.mttr,
            ) {
                new.mttr = d;
                changed = true;
            }
        });
        if changed {
            self.set(field, ron::downtime(&new));
        }
    }

    /// `transfer`: travel time of items leaving by direct connections
    /// (`delay + distância / speed`), straight or over the path network.
    fn transfer_field(&mut self, ui: &mut Ui, current: Option<&TransferDef>) {
        let field = "transfer";
        let id = self.id(field);
        let mut on = current.is_some();
        if ui
            .checkbox(&mut on, tx("prop-panel-transferencia_pela_distancia"))
            .on_hover_text(tx("prop-panel-os_itens_levam_atraso_distancia"))
            .changed()
        {
            let default = TransferDef {
                speed: 1.0,
                delay: 0.0,
                path: TransferPath::Straight,
            };
            self.set(
                field,
                if on {
                    ron::transfer(&default)
                } else {
                    "None".into()
                },
            );
        }
        let Some(t) = current else { return };
        let mut new = *t;
        ui.indent(id, |ui| {
            ui.horizontal(|ui| {
                ui.label(tx("prop-panel-velocidade"));
                let mut v = t.speed;
                if num(ui, id.with("speed"), &mut v, float_speed(t.speed), 0.001) {
                    new.speed = v;
                }
                ui.label(tx("prop-panel-atraso"));
                let mut d = t.delay;
                if num(ui, id.with("delay"), &mut d, float_speed(t.delay), 0.0) {
                    new.delay = d;
                }
            });
            ui.horizontal(|ui| {
                ui.label(tx("prop-panel-caminho"));
                for (label, path) in [
                    (tx("prop-panel-reta"), TransferPath::Straight),
                    (tx("prop-panel-pela_rede"), TransferPath::Network),
                ] {
                    if ui.selectable_label(t.path == path, label).clicked() {
                        new.path = path;
                    }
                }
            });
        });
        if new != *t {
            self.set(field, ron::transfer(&new));
        }
    }

    fn kind_fields(&mut self, ui: &mut Ui, kind: &ObjectKind) {
        match kind {
            ObjectKind::Source {
                interarrival,
                first_at_zero,
                max_arrivals,
                labels,
                on_blocked,
                schedule,
                ..
            } => {
                self.dist_field(
                    ui,
                    tx("prop-panel-tempo_entre_chegadas"),
                    "interarrival",
                    interarrival,
                );
                self.bool_field(
                    ui,
                    tx("prop-panel-primeira_chegada_em_t_0"),
                    "first_at_zero",
                    *first_at_zero,
                );
                self.opt_uint_field(
                    ui,
                    tx("prop-panel-maximo_de_chegadas"),
                    "max_arrivals",
                    *max_arrivals,
                    100,
                );
                self.choice_field(
                    ui,
                    tx("prop-panel-se_a_saida_estiver_bloqueada"),
                    "on_blocked",
                    &ron::debug(on_blocked),
                    &[
                        ("Wait", tx("prop-panel-esperar")),
                        ("Drop", tx("prop-panel-descartar_perda")),
                    ],
                );
                ui.separator();
                ui.strong(tx("prop-panel-rotulos"));
                self.labels_field(ui, "labels", labels);
                ui.separator();
                self.schedule_field(ui, interarrival, labels, schedule.as_ref());
            }
            ObjectKind::Queue {
                capacity,
                discipline,
                batch,
                ..
            } => {
                self.opt_uint_field(
                    ui,
                    tx("prop-panel-capacidade_limitada"),
                    "capacity",
                    capacity.map(|c| c as u64),
                    10,
                );
                self.choice_field(
                    ui,
                    tx("prop-panel-disciplina"),
                    "discipline",
                    &ron::debug(discipline),
                    &[
                        ("Fifo", tx("prop-panel-fifo_primeiro_a_entrar_primeiro")),
                        ("Lifo", tx("prop-panel-lifo_ultimo_a_entrar_primeiro")),
                    ],
                );
                self.batch_field(ui, "batch", batch.map(|b| (b.size, b.max_wait, b.flush)));
            }
            ObjectKind::Processor {
                capacity,
                setup,
                process,
                operators,
                ..
            } => {
                self.uint_field(ui, tx("prop-panel-capacidade"), "capacity", *capacity, 1);
                self.opt_dist_field(
                    ui,
                    tx("prop-panel-preparacao_setup"),
                    "setup",
                    setup.as_ref(),
                    Dist::Const(1.0),
                );
                self.dist_field(
                    ui,
                    tx("prop-panel-tempo_de_processamento"),
                    "process",
                    process,
                );
                ui.separator();
                ui.strong(tx("prop-panel-operadores"));
                self.operators_field(
                    ui,
                    "operators",
                    operators.as_ref().map(|o| {
                        (
                            o.team.clone(),
                            o.count,
                            o.setup,
                            o.process,
                            o.requires.clone(),
                        )
                    }),
                );
            }
            ObjectKind::Combiner {
                components,
                mode,
                setup,
                process,
            } => {
                self.components_field(ui, "components", components);
                self.choice_field(
                    ui,
                    tx("prop-panel-modo"),
                    "mode",
                    &ron::debug(mode),
                    &[
                        ("Pack", tx("prop-panel-embalar_so_o_conteiner_sai")),
                        ("Join", tx("prop-panel-juntar_componentes_destruidos")),
                        ("Batch", tx("prop-panel-lote_todos_saem")),
                    ],
                );
                self.opt_dist_field(
                    ui,
                    tx("prop-panel-preparacao_setup"),
                    "setup",
                    setup.as_ref(),
                    Dist::Const(1.0),
                );
                self.dist_field(
                    ui,
                    tx("prop-panel-tempo_de_processamento"),
                    "process",
                    process,
                );
            }
            ObjectKind::Separator { mode, process } => {
                // `Unpack` or `Split(n)`, as RON text.
                let current = ron::debug(mode);
                let split = current
                    .strip_prefix("Split(")
                    .and_then(|s| s.strip_suffix(')'))
                    .and_then(|s| s.parse::<usize>().ok());
                let options = [
                    ("Unpack", tx("prop-panel-desembalar")),
                    ("Split(2)", tx("prop-panel-dividir_em_copias")),
                ];
                let shown = if split.is_some() {
                    "Split(2)"
                } else {
                    "Unpack"
                };
                ui.horizontal(|ui| {
                    ui.label(tx("prop-panel-modo"));
                    egui::ComboBox::from_id_salt(self.id("mode"))
                        .selected_text(
                            options
                                .iter()
                                .find(|(r, _)| *r == shown)
                                .map_or("", |(_, l)| *l),
                        )
                        .show_ui(ui, |ui| {
                            for (r, l) in options {
                                if ui.selectable_label(r == shown, l).clicked() && r != shown {
                                    self.set("mode", r.to_string());
                                }
                            }
                        });
                });
                if let Some(mut n) = split {
                    ui.horizontal(|ui| {
                        ui.label(tx("prop-panel-numero_de_itens_gerados"));
                        if num(ui, self.id("split"), &mut n, 0.1, 1) {
                            self.set("mode", format!("Split({n})"));
                        }
                    });
                }
                self.dist_field(
                    ui,
                    tx("prop-panel-tempo_de_processamento"),
                    "process",
                    process,
                );
            }
            ObjectKind::Sink => {
                ui.weak(tx("prop-panel-a_saida_nao_tem_parametros"));
            }
            ObjectKind::Transporter {
                speed,
                acceleration,
                deceleration,
                capacity,
                load_time,
                unload_time,
                home,
                agv,
                ..
            } => {
                self.float_field(ui, tx("prop-panel-velocidade_maxima"), "speed", *speed);
                self.opt_float_field(
                    ui,
                    tx("prop-panel-aceleracao"),
                    "acceleration",
                    *acceleration,
                    1.0,
                );
                self.opt_float_field(
                    ui,
                    tx("prop-panel-desaceleracao"),
                    "deceleration",
                    *deceleration,
                    1.0,
                );
                self.uint_field(
                    ui,
                    tx("prop-panel-capacidade_itens"),
                    "capacity",
                    *capacity,
                    1,
                );
                self.dist_field(ui, tx("prop-panel-tempo_de_carga"), "load_time", load_time);
                self.dist_field(
                    ui,
                    tx("prop-panel-tempo_de_descarga"),
                    "unload_time",
                    unload_time,
                );
                self.bool_field(
                    ui,
                    tx("prop-panel-voltar_a_base_quando_ocioso"),
                    "home",
                    *home,
                );
                self.bool_field(ui, tx("prop-panel-agv_controle_de_trafego_na"), "agv", *agv);
            }
            ObjectKind::Conveyor {
                length,
                speed,
                item_length,
                accumulating,
                ..
            } => {
                self.float_field(ui, tx("prop-panel-comprimento"), "length", *length);
                self.float_field(ui, tx("prop-panel-velocidade"), "speed", *speed);
                self.float_field(
                    ui,
                    tx("prop-panel-comprimento_por_item"),
                    "item_length",
                    *item_length,
                );
                self.bool_field(
                    ui,
                    tx("prop-panel-acumula_itens_quando_a_saida"),
                    "accumulating",
                    *accumulating,
                );
                ui.weak(fx(
                    "prop-panel-capacidade_itens-2",
                    &[(
                        "item_length",
                        vx(
                            &if item_length.is_finite() && *item_length > 0.0 && length.is_finite()
                            {
                                (length / item_length).floor().max(0.0) as u64
                            } else {
                                0
                            },
                        ),
                    )],
                ));
            }
            ObjectKind::Rack {
                bays,
                levels,
                dwell,
                placement,
                bay_width,
                level_height,
                ..
            } => {
                self.uint_field(ui, tx("prop-panel-colunas"), "bays", *bays, 1);
                self.uint_field(ui, tx("prop-panel-niveis"), "levels", *levels, 1);
                self.opt_dist_field(
                    ui,
                    tx("prop-panel-permanencia_minima"),
                    "dwell",
                    dwell.as_ref(),
                    Dist::Const(10.0),
                );
                self.choice_field(
                    ui,
                    tx("prop-panel-regra_de_ocupacao"),
                    "placement",
                    &ron::debug(placement),
                    &spec::RACK_PLACEMENTS.map(|(r, k)| (r, tx(k))),
                );
                self.float_field(
                    ui,
                    tx("prop-panel-largura_da_coluna"),
                    "bay_width",
                    *bay_width,
                );
                self.float_field(
                    ui,
                    tx("prop-panel-altura_do_nivel"),
                    "level_height",
                    *level_height,
                );
                ui.weak(fx(
                    "prop-panel-capacidade_posicoes",
                    &[("levels", vx(&(bays * levels)))],
                ));
            }
            // The crane is edited in the 3D app (UTSim 3D), not in this viewer.
            ObjectKind::Crane { .. } => {}
            ObjectKind::Dispatcher { team, rule, .. } => {
                self.team_field(ui, "team", team);
                self.choice_field(
                    ui,
                    tx("prop-panel-regra_de_escolha"),
                    "rule",
                    &ron::debug(rule),
                    &spec::DISPATCH_RULES.map(|(r, k)| (r, tx(k))),
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Entry points
// ---------------------------------------------------------------------------

fn name_field(ui: &mut Ui, doc: &Document, name: &str, out: &mut Vec<Command>) {
    let taken = |t: &str| doc.model().objects.iter().any(|o| o.name == t);
    let check = |t: &str| {
        if t.trim().is_empty() {
            Err(tx("prop-panel-o_nome_nao_pode_ficar").to_string())
        } else if t != name && taken(t) {
            Err(fx(
                "prop-panel-ja_existe_um_objeto_chamado",
                &[("t", vx(&t))],
            ))
        } else {
            Ok(())
        }
    };
    ui.horizontal(|ui| {
        ui.label(tx("prop-panel-nome"));
        let id = Id::new(("utsim-props", name, "#name"));
        if let Some(to) = text_commit(ui, id, name, false, &check) {
            out.push(Command::Rename {
                from: name.to_string(),
                to,
            });
        }
    });
}

/// Draws the editable properties of object `selected` and returns what the user changed.
///
/// When `selected` is not an object of the document, shows the run settings instead.
pub fn properties_panel(ui: &mut Ui, doc: &Document, selected: &str) -> Vec<Command> {
    properties_panel_for(ui, doc, selected, true)
}

/// The same panel for several selected objects of one kind: what belongs to one object
/// alone (its position and rotation) is left out.
pub fn properties_panel_shared(ui: &mut Ui, doc: &Document, selected: &str) -> Vec<Command> {
    properties_panel_for(ui, doc, selected, false)
}

fn properties_panel_for(
    ui: &mut Ui,
    doc: &Document,
    selected: &str,
    with_position: bool,
) -> Vec<Command> {
    let Some(obj) = doc.model().objects.iter().find(|o| o.name == selected) else {
        return run_panel(ui, doc);
    };
    let mut out = Vec::new();

    let issues: Vec<_> = doc
        .issues()
        .into_iter()
        .filter(|i| i.object.as_deref() == Some(selected))
        .collect();
    for issue in &issues {
        ui.colored_label(
            RED,
            fx("prop-panel-problema", &[("message", vx(&issue.message))]),
        );
    }
    if !issues.is_empty() {
        ui.separator();
    }

    name_field(ui, doc, &obj.name, &mut out);
    ui.weak(spec::kind_label(&obj.kind));
    if with_position {
        ui.separator();
        super::position::position_field(ui, obj, &mut out);
    }
    ui.separator();

    let mut editor = Editor {
        doc,
        owner: obj.name.clone(),
        read: Box::new(|field| spec::read_field(obj, field)),
        out,
    };
    editor.kind_fields(ui, &obj.kind);
    if matches!(obj.kind, ObjectKind::Transporter { .. }) {
        editor.home_node_field(ui, obj);
    }
    ui.separator();
    if !utsim_model::is_mobile(&obj.kind) {
        editor.output_field(ui, obj);
        ui.separator();
        editor.transport_field(ui, "transport", obj.transport.as_deref());
        ui.separator();
        editor.transfer_field(ui, obj.transfer.as_ref());
        ui.separator();
    }
    if !utsim_model::is_mobile(&obj.kind) {
        editor.on_entry_field(ui, "on_entry", obj.on_entry.as_deref());
        ui.separator();
    }
    editor.downtime_field(ui, obj.downtime.as_ref());
    editor.shifts_field(ui, obj.shifts.as_ref());
    ui.separator();
    let mut out = editor.out;
    appearance_field(ui, obj, !with_position, &mut out);
    out
}

/// Text shown for "no asset": the default look of the kind.
/// Shown for "no asset chosen" (the kind's default look).
pub fn default_asset() -> &'static str {
    tx("prop-panel-padrao_do_tipo")
}

/// The "Aparência" section: `visual` of the object (asset from the built-in library or
/// the third-party catalogue, colour, scale, yaw) and "Restaurar padrão". Emits one
/// `SetVisual` when something changed.
fn appearance_field(ui: &mut Ui, obj: &ObjectDef, with_yaw: bool, out: &mut Vec<Command>) {
    let current = obj.visual.clone().unwrap_or_default();
    let mut next = current.clone();
    let id = |field: &str| Id::new(("utsim-props", &obj.name, "#visual", field));
    egui::CollapsingHeader::new(tx("prop-panel-aparencia"))
        .id_salt(id("secao"))
        .default_open(true)
        .show(ui, |ui| {
            let shown = current.asset.as_deref().unwrap_or(default_asset());
            ui.horizontal(|ui| {
                ui.label(tx("prop-panel-ativo"));
                ui.strong(shown);
            });
            if let Some(e) = current.asset.as_deref().and_then(terceiros::by_name) {
                ui.weak(format!("{}. {}", e.credit(), e.uso));
                if !e.clipes.is_empty() {
                    ui.weak(tx("prop-panel-clipes_de_animacao_ainda_nao"));
                }
            }
            let filter_id = id("busca");
            let mut filter = temp::<String>(ui, filter_id).unwrap_or_default();
            ui.horizontal(|ui| {
                ui.label(tx("prop-panel-buscar"));
                let r = ui.add(egui::TextEdit::singleline(&mut filter).desired_width(150.0));
                if r.changed() {
                    set_temp(ui, filter_id, filter.clone());
                }
            });
            let wanted = filter.trim().to_lowercase();
            let matches = |n: &str| wanted.is_empty() || n.to_lowercase().contains(&wanted);
            egui::ScrollArea::vertical()
                .id_salt(id("lista"))
                .max_height(180.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    if matches(default_asset())
                        && ui
                            .selectable_label(current.asset.is_none(), default_asset())
                            .clicked()
                    {
                        next.asset = None;
                    }
                    ui.weak(tx("prop-panel-biblioteca_do_utsim"));
                    for spec in utsim_assets::Spec::defaults() {
                        let n = spec.name();
                        if !matches(n) {
                            continue;
                        }
                        let on = current.asset.as_deref() == Some(n);
                        if ui
                            .selectable_label(on, n)
                            .on_hover_text(tx("prop-panel-utsim_gerado_por_utsim_assets"))
                            .clicked()
                        {
                            next.asset = Some(n.to_string());
                        }
                    }
                    ui.weak(tx("prop-panel-terceiros_assets_terceiros"));
                    for e in terceiros::entries() {
                        if !matches(&e.nome) {
                            continue;
                        }
                        let on = current.asset.as_deref() == Some(e.nome.as_str());
                        let size = e.real_size();
                        let hint = format!(
                            "{}\n{}\n{} ({:.2} x {:.2} x {:.2} m)",
                            e.credit(),
                            e.uso,
                            e.arquivo,
                            size[0],
                            size[2],
                            size[1]
                        );
                        if ui
                            .selectable_label(on, &e.nome)
                            .on_hover_text(hint)
                            .clicked()
                        {
                            next.asset = Some(e.nome.clone());
                        }
                    }
                });
            ui.horizontal(|ui| {
                let mut on = current.color.is_some();
                if ui.checkbox(&mut on, tx("prop-panel-cor")).changed() {
                    next.color = on.then(|| "#d08030".to_string());
                }
                if let Some(rgb) = current.color.as_deref().and_then(visual::parse_color) {
                    let mut c = rgb.map(|v| (v * 255.0).round() as u8);
                    if ui.color_edit_button_srgb(&mut c).changed() {
                        next.color = Some(format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2]));
                    }
                    ui.weak(current.color.as_deref().unwrap_or(""));
                }
            });
            ui.horizontal(|ui| {
                ui.label(tx("prop-panel-escala-2"));
                let mut s = current.scale.unwrap_or(1.0);
                if num(ui, id("escala"), &mut s, 0.01, 0.01) {
                    next.scale = (s != 1.0).then_some(s);
                }
                // For one object the rotation lives in "Posição".
                if with_yaw {
                    ui.label(tx("prop-panel-giro_graus"));
                    let mut y = current.yaw.unwrap_or(0.0);
                    if num(ui, id("giro"), &mut y, 1.0, -360.0) {
                        next.yaw = (y != 0.0).then_some(y);
                    }
                }
            });
            let custom = obj.visual.is_some();
            if ui
                .add_enabled(custom, egui::Button::new(tx("prop-panel-restaurar_padrao")))
                .on_hover_text(tx("prop-panel-volta_a_aparencia_padrao_do"))
                .clicked()
            {
                next = VisualDef::default();
            }
        });
    if next != current {
        out.push(Command::SetVisual {
            name: obj.name.clone(),
            visual: Some(next),
        });
    }
}

/// Simulation settings (`run.*`).
pub fn run_panel(ui: &mut Ui, doc: &Document) -> Vec<Command> {
    let run = &doc.model().run;
    let mut editor = Editor {
        doc,
        owner: "run".into(),
        read: Box::new(|field| spec::read_run(run, field)),
        out: Vec::new(),
    };
    ui.strong(tx("prop-panel-simulacao"));
    editor.float_field(
        ui,
        tx("prop-panel-tempo_de_simulacao"),
        "stop_time",
        run.stop_time,
    );
    editor.float_field(ui, tx("prop-panel-aquecimento"), "warmup", run.warmup);
    editor.uint_field(ui, tx("prop-panel-semente"), "seed", run.seed, 0);
    editor.uint_field(
        ui,
        tx("prop-panel-replicacoes"),
        "replications",
        run.replications,
        1,
    );
    for issue in doc.issues().iter().filter(|i| i.object.is_none()) {
        ui.colored_label(
            RED,
            fx("prop-panel-problema", &[("message", vx(&issue.message))]),
        );
    }
    let mut out = editor.out;
    ui.separator();
    out.extend(super::shifts_ui::calendars_section(ui, doc));
    out
}
