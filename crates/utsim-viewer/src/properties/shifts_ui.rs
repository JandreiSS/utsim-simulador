//! Shifts in the property editor: the object's "Turnos" section (a calendar of the
//! model or a custom table, and what happens at the end of a shift) and the model's
//! "Calendários" section, shown with the run settings.

use crate::texto::{dx, fx, tx, vx};
use bevy_egui::egui::{self, Id, Ui};
use utsim_core::{OffWindow, ShiftEnd, ShiftTable};
use utsim_edit::{Command, Document};
use utsim_model::ShiftSpec;

use super::panel::{Editor, RED, clear_temp, labeled_float, set_temp, temp};
use super::ron_text as ron;

/// `(value, label key, hover key)` of the end-of-shift choices.
const ENDS: [(ShiftEnd, &str, &str); 2] = [
    (
        ShiftEnd::Pause,
        "prop-turno-pausar",
        "prop-turno-pausar-dica",
    ),
    (
        ShiftEnd::Finish,
        "prop-turno-terminar",
        "prop-turno-terminar-dica",
    ),
];

fn end_label(end: ShiftEnd) -> &'static str {
    ENDS.iter()
        .find(|(e, ..)| *e == end)
        .map_or("", |(_, l, _)| tx(l))
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
        (None, Some(e)) => fx(
            "prop-shifts-ui-como_no_calendario",
            &[("to_lowercase", vx(&end_label(e).to_lowercase()))],
        ),
        (None, None) => end_label(ShiftEnd::Pause).to_string(),
    };
    ui.horizontal(|ui| {
        ui.label(tx("prop-shifts-ui-ao_fim_do_turno"));
        egui::ComboBox::from_id_salt(id)
            .selected_text(shown)
            .show_ui(ui, |ui| {
                if let Some(e) = inherit {
                    let text = fx(
                        "prop-shifts-ui-como_no_calendario",
                        &[("to_lowercase", vx(&end_label(e).to_lowercase()))],
                    );
                    ui.selectable_value(&mut picked, None, text);
                }
                for (e, label, hover) in ENDS {
                    ui.selectable_value(&mut picked, Some(e), tx(label))
                        .on_hover_text(tx(hover));
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
        changed |= labeled_float(
            ui,
            id.with("period"),
            tx("prop-shifts-ui-periodo"),
            &mut t.period,
        );
    });
    let mut remove = None;
    for (i, w) in t.off.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            changed |= labeled_float(
                ui,
                id.with(("s", i)),
                tx("prop-shifts-ui-inicio"),
                &mut w.start,
            );
            changed |= labeled_float(
                ui,
                id.with(("d", i)),
                tx("prop-shifts-ui-duracao"),
                &mut w.duration,
            );
            if ui.small_button(tx("prop-shifts-ui-remover")).clicked() {
                remove = Some(i);
            }
        });
    }
    if let Some(i) = remove {
        t.off.remove(i);
        changed = true;
    }
    if ui
        .small_button(tx("prop-shifts-ui-adicionar_janela"))
        .clicked()
    {
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
            ui.weak(fx(
                "prop-shifts-ui-fora_de_turno_do_periodo",
                &[(
                    "off_fraction",
                    dx(format!("{:.1}", 100.0 * t.off_fraction() + 0.0)),
                )],
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
            .checkbox(&mut on, tx("prop-shifts-ui-turnos_janelas_fora_de_turno"))
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
                    egui::RadioButton::new(using.is_some(), tx("prop-shifts-ui-usar_calendario")),
                );
                let r = if has {
                    r
                } else {
                    r.on_disabled_hover_text(tx(
                        "prop-shifts-ui-crie_calendarios_na_secao_calendarios",
                    ))
                };
                if r.clicked() && using.is_none() {
                    let spec = ShiftSpec::calendar(calendars[0].clone());
                    self.set(field, ron::shift_spec(&spec));
                }
                if ui
                    .radio(using.is_none(), tx("prop-shifts-ui-personalizado"))
                    .clicked()
                    && using.is_some()
                {
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
                            ui.weak(fx(
                                "prop-shifts-ui-periodo_janela_s_fora_de",
                                &[
                                    ("period", vx(&ron::float(t.period))),
                                    ("off", vx(&t.off.len())),
                                ],
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
                            ui.colored_label(
                                RED,
                                fx(
                                    "prop-shifts-ui-o_calendario_nao_existe",
                                    &[("name", vx(&name))],
                                ),
                            );
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
                    .hint_text(tx("prop-shifts-ui-nome_do_calendario"))
                    .desired_width(120.0),
            );
            if r.changed() {
                set_temp(ui, name_id, name.clone());
            }
            let name = name.trim().to_string();
            let free = !name.is_empty() && !self.doc.model().calendars.contains_key(&name);
            if ui
                .add_enabled(
                    free,
                    egui::Button::new(tx("prop-shifts-ui-salvar_como_calendario")),
                )
                .on_hover_text(tx("prop-shifts-ui-outros_objetos_podem_usar_o"))
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
    egui::CollapsingHeader::new(fx(
        "prop-shifts-ui-calendarios",
        &[("calendars", vx(&model.calendars.len()))],
    ))
    .id_salt("utsim-calendarios")
    .default_open(true)
    .show(ui, |ui| {
        ui.weak(tx("prop-shifts-ui-tabelas_de_turno_com_nome"));
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
                    if let Some(Some(e)) = end_combo(ui, id.with("end"), Some(table.on_end), None) {
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
                        ui.weak(tx("prop-shifts-ui-nenhum_objeto_usa_este_calendario"));
                    } else {
                        ui.weak(fx(
                            "prop-shifts-ui-usado_por",
                            &[("users", vx(&users.join(", ")))],
                        ));
                    }
                    let r = ui.add_enabled(
                        users.is_empty(),
                        egui::Button::new(tx("prop-shifts-ui-remover_calendario")),
                    );
                    if !users.is_empty() {
                        r.on_disabled_hover_text(tx("prop-shifts-ui-troque_os_objetos_para_outro"));
                    } else if r.clicked() {
                        out.push(Command::Set {
                            key: format!("calendars.{name}"),
                            value: "None".into(),
                        });
                    }
                });
        }
        if ui.button(tx("prop-shifts-ui-novo_calendario")).clicked() {
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
            Err(tx("prop-shifts-ui-o_nome_nao_pode_ficar").to_string())
        } else if t != name && doc.model().calendars.contains_key(t) {
            Err(fx(
                "prop-shifts-ui-ja_existe_um_calendario_chamado",
                &[("t", vx(&t))],
            ))
        } else {
            Ok(())
        }
    };
    ui.horizontal(|ui| {
        ui.label(tx("prop-shifts-ui-nome"));
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
