//! The egui side of the property editor. Every widget edits a local copy and
//! returns what the user committed; [`Editor`] turns commits into `Command::Set`
//! (and `Rename`) carrying RON text from [`super::ron_text`].

use std::collections::BTreeMap;

use bevy_egui::egui::{self, Color32, Id, Ui, emath::Numeric};
use utsim_core::{Dist, Downtime, OffWindow, OutputRule, ShiftTable};
use utsim_edit::{Command, Document};
use utsim_model::experiment::patch;
use utsim_model::{ObjectDef, ObjectKind};

use super::ron_text as ron;
use super::spec::{self, FAMILIES};

const RED: Color32 = Color32::from_rgb(225, 80, 80);

// ---------------------------------------------------------------------------
// Transient widget state (egui memory)
// ---------------------------------------------------------------------------

fn temp<T: Clone + Send + Sync + 'static>(ui: &Ui, id: Id) -> Option<T> {
    ui.data(|d| d.get_temp::<T>(id))
}

fn set_temp<T: Clone + Send + Sync + 'static>(ui: &Ui, id: Id, value: T) {
    ui.data_mut(|d| d.insert_temp(id, value));
}

fn clear_temp<T: Clone + Default + Send + Sync + 'static>(ui: &Ui, id: Id) {
    ui.data_mut(|d| d.remove_temp::<T>(id));
}

/// A number box. Edits live in egui memory until committed (Enter, focus lost or
/// drag released); returns true only at the commit, with `value` updated.
fn num<N: Numeric + Default + PartialEq + Send + Sync + 'static>(
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
fn float(ui: &mut Ui, id: Id, value: &mut f64) -> bool {
    num(ui, id, value, float_speed(*value), f64::MIN)
}

fn labeled_float(ui: &mut Ui, id: Id, label: &str, value: &mut f64) -> bool {
    ui.label(label);
    float(ui, id, value)
}

/// A text box committed on Enter / focus loss, only when `check` accepts it.
/// The error of `check` is shown in red while the text is being edited.
fn text_commit(
    ui: &mut Ui,
    id: Id,
    current: &str,
    multiline: bool,
    check: &dyn Fn(&str) -> Result<(), String>,
) -> Option<String> {
    let editing = temp::<String>(ui, id);
    let mut text = editing.unwrap_or_else(|| current.to_string());
    let response = if multiline {
        ui.add(
            egui::TextEdit::multiline(&mut text)
                .desired_width(f32::INFINITY)
                .desired_rows(2),
        )
    } else {
        ui.add(egui::TextEdit::singleline(&mut text).desired_width(160.0))
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
        ui.colored_label(RED, error);
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

fn parse_numbers(text: &str) -> Result<Vec<f64>, String> {
    text.split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.replace(',', ".")
                .parse::<f64>()
                .map_err(|_| format!("\"{s}\" não é um número"))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Distribution widgets (no document access)
// ---------------------------------------------------------------------------

fn mean_text(m: f64) -> String {
    if m.is_finite() {
        format!("{}", (m * 1e4).round() / 1e4)
    } else {
        "infinita".into()
    }
}

/// Family combo plus the parameter boxes. Returns true when a change was committed.
fn dist_widget(ui: &mut Ui, id: Id, d: &mut Dist) -> bool {
    let mut changed = false;
    let mut index = spec::family_index(d);
    let before = index;
    egui::ComboBox::from_id_salt(id.with("family"))
        .selected_text(FAMILIES[index])
        .show_ui(ui, |ui| {
            for (i, name) in FAMILIES.iter().enumerate() {
                ui.selectable_value(&mut index, i, *name);
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
                    changed |= labeled_float(ui, id.with(("v", i)), "valor", value);
                    changed |= labeled_float(ui, id.with(("w", i)), "peso", weight);
                    if ui.small_button("remover").clicked() {
                        remove = Some(i);
                    }
                });
            }
            if let Some(i) = remove {
                table.remove(i);
                changed = true;
            }
            if ui.small_button("adicionar valor").clicked() {
                let next = table.last().map_or(0.0, |(v, _)| v + 1.0);
                table.push((next, 1.0));
                changed = true;
            }
        }
        Dist::Empirical(xs) => {
            ui.label("Observações em ordem crescente, separadas por espaço:");
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
                changed |= labeled_float(ui, id.with("offset"), "deslocamento", offset);
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
        Dist::Const(v) => f(ui, "valor", v),
        Dist::Uniform { min, max } => {
            f(ui, "mín.", min);
            f(ui, "máx.", max);
        }
        Dist::Exp { mean } | Dist::Poisson { mean } => f(ui, "média", mean),
        Dist::Triangular { min, mode, max } => {
            f(ui, "mín.", min);
            f(ui, "moda", mode);
            f(ui, "máx.", max);
        }
        Dist::Normal { mean, sd } | Dist::LogNormal { mean, sd } => {
            f(ui, "média", mean);
            f(ui, "desvio", sd);
        }
        Dist::Gamma { shape, scale }
        | Dist::Weibull { shape, scale }
        | Dist::LogLogistic { shape, scale }
        | Dist::PearsonV { shape, scale } => {
            f(ui, "forma", shape);
            f(ui, "escala", scale);
        }
        Dist::Beta {
            alpha,
            beta,
            min,
            max,
        } => {
            f(ui, "alfa", alpha);
            f(ui, "beta", beta);
            f(ui, "mín.", min);
            f(ui, "máx.", max);
        }
        Dist::PearsonVI {
            shape1,
            shape2,
            scale,
        } => {
            f(ui, "forma 1", shape1);
            f(ui, "forma 2", shape2);
            f(ui, "escala", scale);
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
            changed |= labeled_float(ui, id.with("média"), "média", mean);
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
fn dist_info(ui: &mut Ui, d: &Dist) {
    match d.validate() {
        Ok(()) => {
            ui.weak(format!("média = {}", mean_text(d.mean())));
        }
        Err(e) => {
            ui.colored_label(RED, e);
        }
    }
}

/// A titled distribution editor; returns the new value when committed.
fn dist_block(ui: &mut Ui, id: Id, title: &str, current: &Dist) -> Option<Dist> {
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

struct Editor<'a> {
    doc: &'a Document,
    /// Object name, or "run".
    owner: String,
    read: Reader<'a>,
    out: Vec<Command>,
}

impl Editor<'_> {
    fn id(&self, field: &str) -> Id {
        Id::new(("utsim-props", &self.owner, field))
    }

    fn key(&self, field: &str) -> String {
        format!("{}.{field}", self.owner)
    }

    /// Emits `Set` only when the text differs from what the model holds.
    fn set(&mut self, field: &str, text: String) {
        if (self.read)(field).as_deref() != Some(text.as_str()) {
            self.out.push(Command::Set {
                key: self.key(field),
                value: text,
            });
        }
    }

    /// Whether the document would accept `text` for `field`.
    fn check(&self, field: &str, text: &str) -> Result<(), String> {
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
                ui.weak("sem limite");
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
                ui.weak("sem limite (velocidade constante)");
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
        ui.label("Equipe (transportadores do modelo):");
        if candidates.is_empty() {
            ui.weak("Nenhum transportador no modelo. Crie um na paleta.");
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
                ui.colored_label(RED, format!("\"{missing}\" não é um transportador"));
                if ui.small_button("remover").clicked() {
                    team.retain(|m| m != missing);
                    changed = true;
                }
            });
        }
        if team.is_empty() {
            ui.colored_label(RED, "A equipe não pode ficar vazia.");
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
        current: Option<(String, usize, bool, bool)>,
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
                egui::Checkbox::new(&mut on, "Precisa de operadores"),
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
        let Some((team, mut count, mut setup, mut process)) = current else {
            if candidates.is_empty() {
                ui.weak("Crie um Transportador ou Despachante para usar como equipe.");
            }
            return;
        };
        let mut new_team = team.clone();
        let mut changed = false;
        ui.indent(id, |ui| {
            ui.horizontal(|ui| {
                ui.label("equipe");
                egui::ComboBox::from_id_salt(id.with("team"))
                    .selected_text(if team.is_empty() { "(nenhuma)" } else { &team })
                    .show_ui(ui, |ui| {
                        for name in &candidates {
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
                    format!("\"{team}\" não é um transportador nem despachante"),
                );
            }
            ui.horizontal(|ui| {
                ui.label("quantidade");
                changed |= num(ui, id.with("count"), &mut count, 0.1, 1);
            });
            changed |= ui.checkbox(&mut setup, "durante a preparação").changed();
            changed |= ui
                .checkbox(&mut process, "durante o processamento")
                .changed();
        });
        if changed {
            self.set(field, ron::operators(&new_team, count, setup, process));
        }
    }

    /// `transport`: which transporter/dispatcher carries items out of this object.
    fn transport_field(&mut self, ui: &mut Ui, field: &str, current: Option<&str>) {
        let candidates: Vec<String> = spec::mobile_names(&self.doc.model().objects)
            .into_iter()
            .map(str::to_string)
            .collect();
        ui.horizontal(|ui| {
            ui.label("Transporte de saída");
            egui::ComboBox::from_id_salt(self.id(field))
                .selected_text(current.unwrap_or("nenhum"))
                .show_ui(ui, |ui| {
                    if ui.selectable_label(current.is_none(), "nenhum").clicked() {
                        self.set(field, "None".into());
                    }
                    for name in &candidates {
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
                ui.weak("Os itens passam direto ao próximo bloco.");
            }
            Some(t) if !candidates.iter().any(|c| c == t) => {
                ui.colored_label(
                    RED,
                    format!("\"{t}\" não é um transportador nem despachante"),
                );
            }
            Some(_) => {}
        }
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
                if ui.checkbox(&mut raw, "editar como texto").changed() {
                    set_temp(ui, raw_id, raw);
                    clear_temp::<String>(ui, id.with("text"));
                }
            });
            if raw {
                let check = |t: &str| self.check(field, t);
                new_text = text_commit(ui, id.with("text"), &ron::dist(current), true, &check);
            } else {
                let mut d = current.clone();
                if dist_widget(ui, id, &mut d) {
                    new_text = Some(ron::dist(&d));
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
        ui.label("Quantidade de itens exigida em cada entrada de componentes:");
        for (i, c) in list.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("entrada {}", i + 1));
                changed |= num(ui, id.with(i), c, 0.1, 1);
                if ui.small_button("remover").clicked() {
                    remove = Some(i);
                }
            });
        }
        if let Some(i) = remove {
            list.remove(i);
            changed = true;
        }
        if ui.small_button("adicionar entrada").clicked() {
            list.push(1);
            changed = true;
        }
        ui.weak(format!(
            "O objeto precisa de {} ligações de entrada (1 contêiner + {}).",
            list.len() + 1,
            list.len()
        ));
        if changed {
            self.set(field, ron::usizes(&list));
        }
    }

    fn labels_field(&mut self, ui: &mut Ui, field: &str, current: &BTreeMap<String, Dist>) {
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
                    if let Some(new) = dist_block(ui, sub, "Valor do rótulo", d) {
                        map.insert(name.clone(), new);
                        changed = true;
                    }
                    if ui.small_button("remover rótulo").clicked() {
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
                    .hint_text("nome do rótulo")
                    .desired_width(120.0),
            );
            if response.changed() {
                set_temp(ui, name_id, name.clone());
            }
            let name = name.trim().to_string();
            let free = !name.is_empty() && !map.contains_key(&name);
            if ui
                .add_enabled(free, egui::Button::new("adicionar rótulo"))
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
        if ui.checkbox(&mut on, "Lote (agrupar itens)").changed() {
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
                ui.label("tamanho do lote");
                changed |= num(ui, id.with("size"), &mut size, 0.1, 1);
            });
            ui.horizontal(|ui| {
                changed |= labeled_float(
                    ui,
                    id.with("wait"),
                    "espera máxima (0 = esperar sempre)",
                    &mut max_wait,
                );
            });
            if ui
                .checkbox(&mut flush, "fechar a entrada até o lote sair")
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
        const RULES: [&str; 5] = [
            "Primeira livre",
            "Rodízio",
            "Aleatória",
            "Pesos",
            "Por rótulo",
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
        };
        let mut picked = index;
        ui.horizontal(|ui| {
            ui.label("Regra de saída");
            egui::ComboBox::from_id_salt(id.with("rule"))
                .selected_text(RULES[index])
                .show_ui(ui, |ui| {
                    for (i, name) in RULES.iter().enumerate() {
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
                _ => OutputRule::ByLabel(self.first_label()),
            };
            self.set(field, ron::output(&rule));
        }
        match current {
            OutputRule::Weighted(weights) => {
                let mut w = weights.clone();
                let mut changed = false;
                ui.label("Peso de cada saída (na ordem das ligações):");
                for (i, weight) in w.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        changed |= labeled_float(
                            ui,
                            id.with(("w", i)),
                            &format!("saída {}", i + 1),
                            weight,
                        );
                    });
                }
                if w.len() != n_out {
                    ui.colored_label(
                        RED,
                        format!("Há {} pesos, mas o objeto tem {n_out} saídas.", w.len()),
                    );
                    if ui.small_button("ajustar ao número de saídas").clicked() {
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
                    ui.label("rótulo com o número da porta");
                    let ok = |t: &str| {
                        if t.trim().is_empty() {
                            Err("o nome do rótulo não pode ficar vazio".to_string())
                        } else {
                            Ok(())
                        }
                    };
                    if let Some(text) = text_commit(ui, id.with("label"), label, false, &ok) {
                        self.set(field, ron::output(&OutputRule::ByLabel(text)));
                    }
                });
            }
            _ => {}
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

    fn downtime_field(&mut self, ui: &mut Ui, current: Option<&Downtime>) {
        let field = "downtime";
        let id = self.id(field);
        let mut on = current.is_some();
        if ui.checkbox(&mut on, "Quebras (MTBF/MTTR)").changed() {
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
            if let Some(d) = dist_block(ui, id.with("mtbf"), "Tempo entre falhas (MTBF)", &dt.mtbf)
            {
                new.mtbf = d;
                changed = true;
            }
            if let Some(d) = dist_block(ui, id.with("mttr"), "Tempo de reparo (MTTR)", &dt.mttr) {
                new.mttr = d;
                changed = true;
            }
        });
        if changed {
            self.set(field, ron::downtime(&new));
        }
    }

    fn shifts_field(&mut self, ui: &mut Ui, current: Option<&ShiftTable>) {
        let field = "shifts";
        let id = self.id(field);
        let mut on = current.is_some();
        if ui
            .checkbox(&mut on, "Turnos (janelas fora de turno)")
            .changed()
        {
            let default = ShiftTable {
                period: 1440.0,
                off: Vec::new(),
            };
            self.set(
                field,
                if on {
                    ron::shifts(&default)
                } else {
                    "None".into()
                },
            );
        }
        let Some(table) = current else { return };
        let mut t = table.clone();
        let mut changed = false;
        ui.indent(id, |ui| {
            ui.horizontal(|ui| {
                changed |= labeled_float(ui, id.with("period"), "período", &mut t.period);
            });
            let mut remove = None;
            for (i, w) in t.off.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    changed |= labeled_float(ui, id.with(("s", i)), "início", &mut w.start);
                    changed |= labeled_float(ui, id.with(("d", i)), "duração", &mut w.duration);
                    if ui.small_button("remover").clicked() {
                        remove = Some(i);
                    }
                });
            }
            if let Some(i) = remove {
                t.off.remove(i);
                changed = true;
            }
            if ui.small_button("adicionar janela").clicked() {
                let start = t.off.last().map_or(0.0, |w| w.start + w.duration);
                let duration = ((t.period - start) / 2.0).max(0.0);
                t.off.push(OffWindow { start, duration });
                changed = true;
            }
            match table.validate() {
                Ok(()) => {
                    ui.weak(format!(
                        "fora de turno: {:.1}% do período",
                        100.0 * table.off_fraction()
                    ));
                }
                Err(e) => {
                    ui.colored_label(RED, e);
                }
            }
        });
        if changed {
            self.set(field, ron::shifts(&t));
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
            } => {
                self.dist_field(ui, "Tempo entre chegadas", "interarrival", interarrival);
                self.bool_field(
                    ui,
                    "Primeira chegada em t = 0",
                    "first_at_zero",
                    *first_at_zero,
                );
                self.opt_uint_field(ui, "Máximo de chegadas", "max_arrivals", *max_arrivals, 100);
                self.choice_field(
                    ui,
                    "Se a saída estiver bloqueada",
                    "on_blocked",
                    &ron::debug(on_blocked),
                    &[("Wait", "Esperar"), ("Drop", "Descartar (perda)")],
                );
                ui.separator();
                ui.strong("Rótulos");
                self.labels_field(ui, "labels", labels);
            }
            ObjectKind::Queue {
                capacity,
                discipline,
                batch,
            } => {
                self.opt_uint_field(
                    ui,
                    "Capacidade limitada",
                    "capacity",
                    capacity.map(|c| c as u64),
                    10,
                );
                self.choice_field(
                    ui,
                    "Disciplina",
                    "discipline",
                    &ron::debug(discipline),
                    &[
                        ("Fifo", "FIFO (primeiro a entrar, primeiro a sair)"),
                        ("Lifo", "LIFO (último a entrar, primeiro a sair)"),
                    ],
                );
                self.batch_field(ui, "batch", batch.map(|b| (b.size, b.max_wait, b.flush)));
            }
            ObjectKind::Processor {
                capacity,
                setup,
                process,
                operators,
            } => {
                self.uint_field(ui, "Capacidade", "capacity", *capacity, 1);
                self.opt_dist_field(
                    ui,
                    "Preparação (setup)",
                    "setup",
                    setup.as_ref(),
                    Dist::Const(1.0),
                );
                self.dist_field(ui, "Tempo de processamento", "process", process);
                ui.separator();
                ui.strong("Operadores");
                self.operators_field(
                    ui,
                    "operators",
                    operators
                        .as_ref()
                        .map(|o| (o.team.clone(), o.count, o.setup, o.process)),
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
                    "Modo",
                    "mode",
                    &ron::debug(mode),
                    &[
                        ("Pack", "Embalar (só o contêiner sai)"),
                        ("Join", "Juntar (componentes destruídos)"),
                        ("Batch", "Lote (todos saem)"),
                    ],
                );
                self.opt_dist_field(
                    ui,
                    "Preparação (setup)",
                    "setup",
                    setup.as_ref(),
                    Dist::Const(1.0),
                );
                self.dist_field(ui, "Tempo de processamento", "process", process);
            }
            ObjectKind::Separator { mode, process } => {
                // `Unpack` or `Split(n)`, as RON text.
                let current = ron::debug(mode);
                let split = current
                    .strip_prefix("Split(")
                    .and_then(|s| s.strip_suffix(')'))
                    .and_then(|s| s.parse::<usize>().ok());
                let options = [("Unpack", "Desembalar"), ("Split(2)", "Dividir em cópias")];
                let shown = if split.is_some() {
                    "Split(2)"
                } else {
                    "Unpack"
                };
                ui.horizontal(|ui| {
                    ui.label("Modo");
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
                        ui.label("número de itens gerados");
                        if num(ui, self.id("split"), &mut n, 0.1, 1) {
                            self.set("mode", format!("Split({n})"));
                        }
                    });
                }
                self.dist_field(ui, "Tempo de processamento", "process", process);
            }
            ObjectKind::Sink => {
                ui.weak("A saída não tem parâmetros.");
            }
            ObjectKind::Transporter {
                speed,
                acceleration,
                deceleration,
                capacity,
                load_time,
                unload_time,
                home,
            } => {
                self.float_field(ui, "Velocidade máxima", "speed", *speed);
                self.opt_float_field(ui, "Aceleração", "acceleration", *acceleration, 1.0);
                self.opt_float_field(ui, "Desaceleração", "deceleration", *deceleration, 1.0);
                self.uint_field(ui, "Capacidade (itens)", "capacity", *capacity, 1);
                self.dist_field(ui, "Tempo de carga", "load_time", load_time);
                self.dist_field(ui, "Tempo de descarga", "unload_time", unload_time);
                self.bool_field(ui, "Voltar à base quando ocioso", "home", *home);
            }
            ObjectKind::Conveyor {
                length,
                speed,
                item_length,
                accumulating,
            } => {
                self.float_field(ui, "Comprimento", "length", *length);
                self.float_field(ui, "Velocidade", "speed", *speed);
                self.float_field(ui, "Comprimento por item", "item_length", *item_length);
                self.bool_field(
                    ui,
                    "Acumula itens quando a saída bloqueia",
                    "accumulating",
                    *accumulating,
                );
                ui.weak(format!(
                    "Capacidade: {} itens",
                    if item_length.is_finite() && *item_length > 0.0 && length.is_finite() {
                        (length / item_length).floor().max(0.0) as u64
                    } else {
                        0
                    }
                ));
            }
            ObjectKind::Rack {
                bays,
                levels,
                dwell,
                placement,
                bay_width,
                level_height,
            } => {
                self.uint_field(ui, "Colunas", "bays", *bays, 1);
                self.uint_field(ui, "Níveis", "levels", *levels, 1);
                self.opt_dist_field(
                    ui,
                    "Permanência mínima",
                    "dwell",
                    dwell.as_ref(),
                    Dist::Const(10.0),
                );
                self.choice_field(
                    ui,
                    "Regra de ocupação",
                    "placement",
                    &ron::debug(placement),
                    &spec::RACK_PLACEMENTS,
                );
                self.float_field(ui, "Largura da coluna", "bay_width", *bay_width);
                self.float_field(ui, "Altura do nível", "level_height", *level_height);
                ui.weak(format!("Capacidade: {} posições", bays * levels));
            }
            ObjectKind::Dispatcher { team, rule } => {
                self.team_field(ui, "team", team);
                self.choice_field(
                    ui,
                    "Regra de escolha",
                    "rule",
                    &ron::debug(rule),
                    &spec::DISPATCH_RULES,
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
            Err("o nome não pode ficar vazio".to_string())
        } else if t != name && taken(t) {
            Err(format!("já existe um objeto chamado \"{t}\""))
        } else {
            Ok(())
        }
    };
    ui.horizontal(|ui| {
        ui.label("Nome");
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
        ui.colored_label(RED, format!("Problema: {}", issue.message));
    }
    if !issues.is_empty() {
        ui.separator();
    }

    name_field(ui, doc, &obj.name, &mut out);
    ui.weak(spec::kind_name(&obj.kind));
    ui.separator();

    let mut editor = Editor {
        doc,
        owner: obj.name.clone(),
        read: Box::new(|field| spec::read_field(obj, field)),
        out,
    };
    editor.kind_fields(ui, &obj.kind);
    ui.separator();
    if !utsim_model::is_mobile(&obj.kind) {
        editor.output_field(ui, obj);
        ui.separator();
        editor.transport_field(ui, "transport", obj.transport.as_deref());
        ui.separator();
    }
    editor.downtime_field(ui, obj.downtime.as_ref());
    editor.shifts_field(ui, obj.shifts.as_ref());
    editor.out
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
    ui.strong("Simulação");
    editor.float_field(ui, "Tempo de simulação", "stop_time", run.stop_time);
    editor.float_field(ui, "Aquecimento", "warmup", run.warmup);
    editor.uint_field(ui, "Semente", "seed", run.seed, 0);
    editor.uint_field(ui, "Replicações", "replications", run.replications, 1);
    for issue in doc.issues().iter().filter(|i| i.object.is_none()) {
        ui.colored_label(RED, format!("Problema: {}", issue.message));
    }
    editor.out
}
