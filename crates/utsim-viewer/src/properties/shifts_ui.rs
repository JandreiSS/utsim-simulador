//! Shifts in the property editor: the object's "Turnos" section (a calendar of the
//! model or a custom table, and what happens at the end of a shift) and the model's
//! "Calendários" section, shown with the run settings.

use bevy_egui::egui::{self, Id, Ui};
use utsim_core::{OffWindow, ShiftEnd, ShiftTable};
use utsim_edit::{Command, Document};
use utsim_model::ShiftSpec;

use super::panel::{Editor, RED, clear_temp, labeled_float, set_temp, temp};
use super::ron_text as ron;

/// `(value, label, hover)` of the end-of-shift choices.
const ENDS: [(ShiftEnd, &str, &str); 2] = [
    (
        ShiftEnd::Pause,
        "Pausar",
        "O trabalho em andamento congela no fim do turno e continua quando o turno volta",
    ),
    (
        ShiftEnd::Finish,
        "Terminar",
        "No fim do turno o objeto não aceita itens novos, termina o item em andamento \
         (hora extra) e só então sai de turno; o turno seguinte começa na hora de sempre",
    ),
];

fn end_label(end: ShiftEnd) -> &'static str {
    ENDS.iter()
        .find(|(e, ..)| *e == end)
        .map_or("", |(_, l, _)| l)
}

/// "Ao fim do turno" combo. `inherit` adds the calendar's own choice as the first
/// option (`None`). Returns the new choice when it changed.
fn end_combo(
    ui: &mut Ui,
    id: Id,
    current: Option<ShiftEnd>,
    inherit: Option<ShiftEnd>,
) -> Option<Option<ShiftEnd>> {
    let mut picked = current;
    let shown = match (current, inherit) {
        (Some(e), _) => end_label(e).to_string(),
        (None, Some(e)) => format!("como no calendário ({})", end_label(e).to_lowercase()),
        (None, None) => end_label(ShiftEnd::Pause).to_string(),
    };
    ui.horizontal(|ui| {
        ui.label("Ao fim do turno");
        egui::ComboBox::from_id_salt(id)
            .selected_text(shown)
            .show_ui(ui, |ui| {
                if let Some(e) = inherit {
                    let text = format!("como no calendário ({})", end_label(e).to_lowercase());
                    ui.selectable_value(&mut picked, None, text);
                }
                for (e, label, hover) in ENDS {
                    ui.selectable_value(&mut picked, Some(e), label)
                        .on_hover_text(hover);
                }
            });
    });
    (picked != current).then_some(picked)
}

/// Editor of the windows of a table (period and off windows). Returns true when a change
/// was committed; `t` holds the new table.
fn windows_editor(ui: &mut Ui, id: Id, t: &mut ShiftTable) -> bool {
    let mut changed = false;
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
    changed
}

/// Summary line (or the validation error) of a table.
fn table_info(ui: &mut Ui, t: &ShiftTable) {
    match t.validate() {
        Ok(()) => {
            // `+ 0.0` turns -0.0 into 0.0 ("fora de turno: -0,0%" in the pilot).
            ui.weak(format!(
                "fora de turno: {:.1}% do período",
                100.0 * t.off_fraction() + 0.0
            ));
        }
        Err(e) => {
            ui.colored_label(RED, e);
        }
    }
}

fn default_table() -> ShiftTable {
    ShiftTable {
        period: 1440.0,
        off: Vec::new(),
        on_end: ShiftEnd::Pause,
    }
}

/// A name for a new calendar that is not taken yet.
fn free_name(doc: &Document, base: &str) -> String {
    let taken = |n: &str| doc.model().calendars.contains_key(n);
    if !taken(base) {
        return base.to_string();
    }
    (2..)
        .map(|k| format!("{base}_{k}"))
        .find(|n| !taken(n))
        .expect("some name is free")
}

impl Editor<'_> {
    /// The object's "Turnos" section.
    pub(super) fn shifts_field(&mut self, ui: &mut Ui, current: Option<&ShiftSpec>) {
        let field = "shifts";
        let id = self.id(field);
        let calendars: Vec<String> = self.doc.model().calendars.keys().cloned().collect();
        let mut on = current.is_some();
        if ui
            .checkbox(&mut on, "Turnos (janelas fora de turno)")
            .changed()
        {
            let text = match (on, calendars.first()) {
                (false, _) => "None".to_string(),
                (true, Some(name)) => ron::shift_spec(&ShiftSpec::calendar(name.clone())),
                (true, None) => ron::shifts(&default_table()),
            };
            self.set(field, text);
        }
        let Some(spec) = current else { return };
        ui.indent(id, |ui| {
            let using = spec.calendar_name().map(str::to_string);
            ui.horizontal(|ui| {
                let has = !calendars.is_empty();
                let r = ui.add_enabled(
                    has,
                    egui::RadioButton::new(using.is_some(), "usar calendário"),
                );
                let r = if has {
                    r
                } else {
                    r.on_disabled_hover_text(
                        "Crie calendários na seção Calendários do painel Simulação \
                         (clique num ponto vazio da cena)",
                    )
                };
                if r.clicked() && using.is_none() {
                    let spec = ShiftSpec::calendar(calendars[0].clone());
                    self.set(field, ron::shift_spec(&spec));
                }
                if ui.radio(using.is_none(), "personalizado").clicked() && using.is_some() {
                    // Start the custom table from the calendar it used.
                    let table = match spec {
                        ShiftSpec::Calendar { name, on_end } => {
                            let mut t = self
                                .doc
                                .model()
                                .calendars
                                .get(name)
                                .cloned()
                                .unwrap_or_else(default_table);
                            if let Some(e) = on_end {
                                t.on_end = *e;
                            }
                            t
                        }
                        ShiftSpec::Inline(t) => t.clone(),
                    };
                    self.set(field, ron::shifts(&table));
                }
            });
            match spec {
                ShiftSpec::Calendar { name, on_end } => {
                    let mut picked = name.clone();
                    egui::ComboBox::from_id_salt(id.with("cal"))
                        .selected_text(name.as_str())
                        .show_ui(ui, |ui| {
                            for c in &calendars {
                                ui.selectable_value(&mut picked, c.clone(), c.as_str());
                            }
                        });
                    if picked != *name {
                        let spec = ShiftSpec::Calendar {
                            name: picked,
                            on_end: *on_end,
                        };
                        self.set(field, ron::shift_spec(&spec));
                    }
                    match self.doc.model().calendars.get(name) {
                        Some(t) => {
                            ui.weak(format!(
                                "período {}, {} janela(s) fora de turno",
                                ron::float(t.period),
                                t.off.len()
                            ));
                            table_info(ui, t);
                            if let Some(new) =
                                end_combo(ui, id.with("end"), *on_end, Some(t.on_end))
                            {
                                let spec = ShiftSpec::Calendar {
                                    name: name.clone(),
                                    on_end: new,
                                };
                                self.set(field, ron::shift_spec(&spec));
                            }
                        }
                        None => {
                            ui.colored_label(RED, format!("o calendário '{name}' não existe"));
                        }
                    }
                }
                ShiftSpec::Inline(table) => {
                    let mut t = table.clone();
                    let mut changed = windows_editor(ui, id, &mut t);
                    table_info(ui, table);
                    if let Some(Some(e)) = end_combo(ui, id.with("end"), Some(t.on_end), None) {
                        t.on_end = e;
                        changed = true;
                    }
                    if changed {
                        self.set(field, ron::shifts(&t));
                    }
                    self.save_as_calendar(ui, id, table);
                }
            }
        });
    }

    /// "Salvar como calendário": moves the custom table to a new calendar of the model
    /// and makes the object use it.
    fn save_as_calendar(&mut self, ui: &mut Ui, id: Id, table: &ShiftTable) {
        let name_id = id.with("new-cal");
        ui.horizontal(|ui| {
            let mut name = temp::<String>(ui, name_id).unwrap_or_default();
            let r = ui.add(
                egui::TextEdit::singleline(&mut name)
                    .hint_text("nome do calendário")
                    .desired_width(120.0),
            );
            if r.changed() {
                set_temp(ui, name_id, name.clone());
            }
            let name = name.trim().to_string();
            let free = !name.is_empty() && !self.doc.model().calendars.contains_key(&name);
            if ui
                .add_enabled(free, egui::Button::new("salvar como calendário"))
                .on_hover_text("Outros objetos podem usar o mesmo calendário")
                .clicked()
            {
                let spec = ShiftSpec::calendar(name.clone());
                self.out.push(Command::Batch(vec![
                    Command::Set {
                        key: format!("calendars.{name}"),
                        value: ron::shifts(table),
                    },
                    Command::Set {
                        key: self.key("shifts"),
                        value: ron::shift_spec(&spec),
                    },
                ]));
                clear_temp::<String>(ui, name_id);
            }
        });
    }
}

/// The model's named calendars ("Calendários"): add, rename, edit and remove.
pub fn calendars_section(ui: &mut Ui, doc: &Document) -> Vec<Command> {
    let mut out = Vec::new();
    let model = doc.model();
    egui::CollapsingHeader::new(format!("Calendários ({})", model.calendars.len()))
        .id_salt("utsim-calendarios")
        .default_open(true)
        .show(ui, |ui| {
            ui.weak(
                "Tabelas de turno com nome. Um objeto usa uma delas em Turnos > usar \
                 calendário; mudar o calendário muda todos os objetos que o usam.",
            );
            for (name, table) in &model.calendars {
                let users: Vec<&str> = model
                    .objects
                    .iter()
                    .filter(|o| o.shifts.as_ref().and_then(ShiftSpec::calendar_name) == Some(name))
                    .map(|o| o.name.as_str())
                    .collect();
                let id = Id::new(("utsim-calendario", name));
                egui::CollapsingHeader::new(name)
                    .id_salt(id)
                    .default_open(true)
                    .show(ui, |ui| {
                        rename_calendar(ui, doc, id, name, table, &mut out);
                        let mut t = table.clone();
                        let mut changed = windows_editor(ui, id, &mut t);
                        table_info(ui, table);
                        if let Some(Some(e)) =
                            end_combo(ui, id.with("end"), Some(table.on_end), None)
                        {
                            t.on_end = e;
                            changed = true;
                        }
                        if changed {
                            out.push(Command::Set {
                                key: format!("calendars.{name}"),
                                value: ron::shifts(&t),
                            });
                        }
                        if users.is_empty() {
                            ui.weak("nenhum objeto usa este calendário");
                        } else {
                            ui.weak(format!("usado por: {}", users.join(", ")));
                        }
                        let r = ui.add_enabled(
                            users.is_empty(),
                            egui::Button::new("remover calendário"),
                        );
                        if !users.is_empty() {
                            r.on_disabled_hover_text(
                                "Troque os objetos para outro calendário ou para personalizado antes",
                            );
                        } else if r.clicked() {
                            out.push(Command::Set {
                                key: format!("calendars.{name}"),
                                value: "None".into(),
                            });
                        }
                    });
            }
            if ui.button("novo calendário").clicked() {
                out.push(Command::Set {
                    key: format!("calendars.{}", free_name(doc, "turno")),
                    value: ron::shifts(&default_table()),
                });
            }
        });
    out
}

/// Name box of a calendar; renaming also updates every object that uses it.
fn rename_calendar(
    ui: &mut Ui,
    doc: &Document,
    id: Id,
    name: &str,
    table: &ShiftTable,
    out: &mut Vec<Command>,
) {
    let check = |t: &str| {
        let t = t.trim();
        if t.is_empty() {
            Err("o nome não pode ficar vazio".to_string())
        } else if t != name && doc.model().calendars.contains_key(t) {
            Err(format!("já existe um calendário chamado \"{t}\""))
        } else {
            Ok(())
        }
    };
    ui.horizontal(|ui| {
        ui.label("Nome");
        let Some(new) = super::panel::text_commit(ui, id.with("name"), name, false, &check) else {
            return;
        };
        let new = new.trim().to_string();
        let mut batch = vec![Command::Set {
            key: format!("calendars.{new}"),
            value: ron::shifts(table),
        }];
        for o in &doc.model().objects {
            if let Some(ShiftSpec::Calendar { name: used, on_end }) = &o.shifts
                && used == name
            {
                let spec = ShiftSpec::Calendar {
                    name: new.clone(),
                    on_end: *on_end,
                };
                batch.push(Command::Set {
                    key: format!("{}.shifts", o.name),
                    value: ron::shift_spec(&spec),
                });
            }
        }
        batch.push(Command::Set {
            key: format!("calendars.{name}"),
            value: "None".into(),
        });
        out.push(Command::Batch(batch));
    });
}
