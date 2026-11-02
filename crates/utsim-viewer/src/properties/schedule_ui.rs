//! The "Chegadas por período" table of a source: cycle, one row per period (start, end,
//! mode and value), the expected arrivals per period and per cycle, and the labels of
//! each period. The whole table is committed as one `schedule` value.

use std::collections::BTreeMap;

use bevy_egui::egui::{self, Id, Ui};
use utsim_core::Dist;
use utsim_model::{ArrivalSchedule, PeriodArrivals};

use super::panel::{Editor, RED, clear_temp, dist_block, mean_text, num, set_temp, temp};
use super::ron_text as ron;
use super::schedule::{
    MODES, Unit, clock, default_schedule, mode_index, next_period, parse_time, switch_mode,
    unit_of_cycle,
};

/// Short text of a distribution for the table ("Exp, média 6").
fn dist_summary(d: &Dist) -> String {
    let family = super::spec::FAMILIES[super::spec::family_index(d)];
    format!("{family}, média {}", mean_text(d.mean()))
}

fn count_text(v: f64) -> String {
    if v.is_finite() {
        format!("{v:.1}")
    } else {
        "?".into()
    }
}

/// A time box: hh:mm text when the cycle suggests a unit, a number otherwise.
/// Returns true when a new value was committed into `t`.
fn time_box(ui: &mut Ui, id: Id, t: &mut f64, cycle: f64) -> bool {
    let Some(shown) = clock(*t, cycle) else {
        return num(ui, id, t, 1.0, 0.0);
    };
    let editing = temp::<String>(ui, id);
    let mut text = editing.unwrap_or_else(|| shown.clone());
    let width = if shown.starts_with("dia") { 82.0 } else { 46.0 };
    let r = ui.add(egui::TextEdit::singleline(&mut text).desired_width(width));
    if r.changed() {
        set_temp(ui, id, text.clone());
    }
    let pending = temp::<String>(ui, id).is_some();
    let parsed = parse_time(&text, cycle);
    if pending && let Err(e) = &parsed {
        ui.colored_label(RED, "!").on_hover_text(e.as_str());
    }
    if pending && r.lost_focus() {
        clear_temp::<String>(ui, id);
        if let Ok(v) = parsed
            && v != *t
        {
            *t = v;
            return true;
        }
    }
    false
}

impl Editor<'_> {
    /// "Chegadas por período" of a source.
    pub(super) fn schedule_field(
        &mut self,
        ui: &mut Ui,
        default: &Dist,
        labels: &BTreeMap<String, Dist>,
        current: Option<&ArrivalSchedule>,
    ) {
        let field = "schedule";
        let id = self.id(field);
        ui.strong("Chegadas por período");
        let mut on = current.is_some();
        if ui
            .checkbox(&mut on, "a taxa de chegada muda com a hora")
            .on_hover_text(
                "Uma tabela de períodos que se repete a cada ciclo (um dia, uma semana). \
                 Fora dos períodos não há chegadas.",
            )
            .changed()
        {
            let text = if on {
                ron::schedule(&default_schedule())
            } else {
                "None".into()
            };
            self.set(field, text);
        }
        let Some(s) = current else {
            ui.weak("Desligado: o tempo entre chegadas acima vale o tempo todo.");
            return;
        };
        let mut t = s.clone();
        let mut changed = false;
        let unit = unit_of_cycle(t.cycle);
        ui.horizontal(|ui| {
            ui.label("ciclo");
            changed |= num(ui, id.with("cycle"), &mut t.cycle, 1.0, 0.0);
            for (label, v) in [("dia", 1440.0), ("semana", 10080.0)] {
                if ui
                    .small_button(label)
                    .on_hover_text(format!("ciclo de {v} (tempo do modelo em minutos)"))
                    .clicked()
                    && t.cycle != v
                {
                    t.cycle = v;
                    changed = true;
                }
            }
        });
        match unit {
            Some(u) => ui.weak(format!(
                "Ciclo de {}: tempo do modelo em {}. Horários em hh:mm, taxas por hora.",
                ron::float(t.cycle),
                u.name()
            )),
            None => ui.weak(
                "Horários e taxas em unidades do modelo (com ciclo 1440, um dia em minutos, \
                 a tabela mostra hh:mm).",
            ),
        };
        let rate_label = match unit {
            Some(_) => "taxa por hora",
            None => "taxa",
        };
        let mut remove = None;
        egui::Grid::new(id.with("grid"))
            .striped(true)
            .num_columns(7)
            .spacing([6.0, 4.0])
            .show(ui, |ui| {
                for h in ["#", "início", "fim", "modo", "valor", "esperado", ""] {
                    ui.weak(h);
                }
                ui.end_row();
                for (i, p) in t.periods.iter_mut().enumerate() {
                    let row = id.with(("p", i));
                    ui.label(format!("{}", i + 1));
                    changed |= time_box(ui, row.with("s"), &mut p.start, s.cycle);
                    changed |= time_box(ui, row.with("e"), &mut p.end, s.cycle);
                    let index = mode_index(&p.arrivals);
                    let mut picked = index;
                    let name = |k: usize| if k == 2 { rate_label } else { MODES[k] };
                    egui::ComboBox::from_id_salt(row.with("mode"))
                        .selected_text(name(index))
                        .width(110.0)
                        .show_ui(ui, |ui| {
                            for k in 0..MODES.len() {
                                ui.selectable_value(&mut picked, k, name(k));
                            }
                        });
                    if picked != index {
                        p.arrivals = switch_mode(&p.arrivals, picked, default);
                        changed = true;
                    }
                    match &mut p.arrivals {
                        PeriodArrivals::Default => {
                            ui.weak(dist_summary(default));
                        }
                        PeriodArrivals::Interarrival(d) => {
                            ui.label(dist_summary(d));
                        }
                        PeriodArrivals::Rate(r) => {
                            let k = unit.map_or(1.0, Unit::per_hour);
                            let mut shown = *r * k;
                            ui.horizontal(|ui| {
                                if num(ui, row.with("rate"), &mut shown, 0.1, 0.0) {
                                    *r = shown / k;
                                    changed = true;
                                }
                                ui.weak(if unit.is_some() { "/h" } else { "/un." });
                            });
                        }
                        PeriodArrivals::Closed => {
                            ui.weak("-");
                        }
                    }
                    let n = if s.periods.get(i) == Some(p) {
                        s.validate()
                            .is_empty()
                            .then(|| s.expected_in_period(i, default))
                    } else {
                        None
                    };
                    ui.label(n.map_or("-".into(), count_text));
                    ui.horizontal(|ui| {
                        if ui
                            .small_button("copiar do padrão")
                            .on_hover_text(
                                "Usa no período uma cópia do tempo entre chegadas padrão, \
                                 para ajustar só aqui",
                            )
                            .clicked()
                        {
                            p.arrivals = PeriodArrivals::Interarrival(default.clone());
                            changed = true;
                        }
                        if ui
                            .small_button("x")
                            .on_hover_text("remover o período")
                            .clicked()
                        {
                            remove = Some(i);
                        }
                    });
                    ui.end_row();
                }
            });
        if let Some(i) = remove {
            t.periods.remove(i);
            changed = true;
        }
        ui.horizontal(|ui| {
            let next = next_period(&t);
            let r = ui.add_enabled(next.is_some(), egui::Button::new("adicionar período"));
            if r.clicked()
                && let Some(p) = next
            {
                t.periods.push(p);
                changed = true;
            }
        });
        let errors = s.validate();
        if errors.is_empty() {
            let per_cycle = s.expected_per_cycle(default);
            let hint = match unit {
                Some(_) if s.cycle == 1440.0 => " (por dia)".to_string(),
                Some(_) if s.cycle == 10080.0 => " (por semana)".to_string(),
                _ => String::new(),
            };
            ui.label(format!(
                "Esperado por ciclo{hint}: {} chegadas",
                count_text(per_cycle)
            ));
            ui.weak(
                "Na troca de período a chegada pendente é sorteada de novo com o processo do \
                 novo período (períodos seguidos com o mesmo processo mantêm a chegada).",
            );
        } else {
            for e in errors {
                ui.colored_label(RED, e);
            }
        }
        // Distribution of each "distribuição" period, with the full editor.
        let with_dist: Vec<usize> = (0..t.periods.len())
            .filter(|&i| matches!(t.periods[i].arrivals, PeriodArrivals::Interarrival(_)))
            .collect();
        if !with_dist.is_empty() {
            egui::CollapsingHeader::new("Distribuições dos períodos")
                .id_salt(id.with("dists"))
                .default_open(true)
                .show(ui, |ui| {
                    for i in with_dist {
                        let title = format!("Período {}: tempo entre chegadas", i + 1);
                        if let PeriodArrivals::Interarrival(d) = &t.periods[i].arrivals
                            && let Some(new) = dist_block(ui, id.with(("d", i)), &title, d)
                        {
                            t.periods[i].arrivals = PeriodArrivals::Interarrival(new);
                            changed = true;
                        }
                    }
                });
        }
        if changed {
            self.set(field, ron::schedule(&t));
            return;
        }
        egui::CollapsingHeader::new("Rótulos por período")
            .id_salt(id.with("labels"))
            .default_open(false)
            .show(ui, |ui| {
                ui.weak(
                    "Os rótulos de um período substituem os rótulos padrão de mesmo nome \
                     (por exemplo, mais urgentes à noite); os outros continuam valendo.",
                );
                if labels.is_empty() {
                    ui.weak("A fonte não tem rótulos padrão.");
                }
                for (i, p) in s.periods.iter().enumerate() {
                    let span = match (clock(p.start, s.cycle), clock(p.end, s.cycle)) {
                        (Some(a), Some(b)) => format!("{a} a {b}"),
                        _ => format!("{} a {}", ron::float(p.start), ron::float(p.end)),
                    };
                    ui.label(format!("Período {} ({span})", i + 1));
                    ui.indent(id.with(("l", i)), |ui| {
                        self.labels_field(ui, &format!("schedule[{}].labels", i + 1), &p.labels);
                    });
                }
            });
    }
}
