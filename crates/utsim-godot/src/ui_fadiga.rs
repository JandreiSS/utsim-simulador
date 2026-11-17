//! The fatigue curve of a person in the properties panel (fase P4 of the people's time
//! study): for each task and method of the `study` of the person, the means measured at
//! each hour of the shift and the curve fitted by `utsim_fit::fatigue` (the same fit as
//! `utsim pessoa ajustar`), with the pauses shaded. Read only: the readings and the fit are
//! edited in UTSim Estatística (or with the command). "Abrir no UTSim Estatística" hands it
//! the readings as a spreadsheet ([`csv`]).

use egui::{Button, Color32, Label, RichText, Ui};
use egui_plot::{Line, Plot, PlotPoints, Points, Polygon};
use utsim_model::{Model, ObjectDef};
use utsim_ui_tema::colors;

use crate::egui_widgets::Registry;
use crate::texto::{fx, tx};
use crate::ui_props::named;

/// One task and method of the study, fitted (or why not).
#[derive(Clone, Debug, PartialEq)]
pub struct CurvaFadiga {
    /// "Tarefa torno, método A: 60 leituras em 5 horários".
    pub titulo: String,
    /// Hour of the shift and mean measured time at each hour.
    pub medias: Vec<[f64; 2]>,
    /// The fitted curve, hour by hour (small steps).
    pub curva: Vec<[f64; 2]>,
    /// The pauses of the study (hours).
    pub pausas: Vec<(f64, f64)>,
    /// t0, the increase and the rates, in a line.
    pub resumo: String,
    pub avisos: Vec<String>,
    /// Why the fit failed (then the curve is empty).
    pub erro: Option<String>,
}

/// Step of the drawn curve, hours.
const STEP: f64 = 0.05;

fn round(x: f64, places: i32) -> f64 {
    let k = 10f64.powi(places);
    (x * k).round() / k
}

/// What the curves of `name` depend on (its name, its study and the language): the same
/// key, the same curves.
pub fn chave(model: &Model, name: &str) -> String {
    let study = model
        .objects
        .iter()
        .find(|o| o.name == name)
        .and_then(|o| o.person.as_deref())
        .and_then(|p| p.study.as_ref());
    format!("{name}|{study:?}|{}", tx("fadiga-tarefa"))
}

/// The curves of the person `name` of `model`, one per task and method of its study, in
/// the order of the readings; empty without a study.
pub fn curvas(model: &Model, name: &str) -> Vec<CurvaFadiga> {
    let Some(person) = model
        .objects
        .iter()
        .find(|o| o.name == name)
        .and_then(|o: &ObjectDef| o.person.as_deref())
    else {
        return Vec::new();
    };
    let Some(study) = person.study.as_ref() else {
        return Vec::new();
    };
    let mut pairs: Vec<(&str, &str)> = Vec::new();
    for r in &study.readings {
        let p = (r.task.as_str(), r.method.as_str());
        if !pairs.contains(&p) {
            pairs.push(p);
        }
    }
    pairs
        .into_iter()
        .map(|(task, method)| {
            let readings: Vec<&utsim_core::Reading> = study
                .readings
                .iter()
                .filter(|r| r.task == task && r.method == method)
                .collect();
            let hours = {
                let mut h: Vec<f64> = readings.iter().map(|r| r.hour).collect();
                h.sort_by(f64::total_cmp);
                h.dedup();
                h.len()
            };
            let n: usize = readings.iter().map(|r| r.times.len()).sum();
            let titulo = if method.is_empty() {
                fx(
                    "fadiga-tarefa",
                    &[
                        ("tarefa", task.into()),
                        ("leituras", n.into()),
                        ("horarios", hours.into()),
                    ],
                )
            } else {
                fx(
                    "fadiga-tarefa-metodo",
                    &[
                        ("tarefa", task.into()),
                        ("metodo", method.into()),
                        ("leituras", n.into()),
                        ("horarios", hours.into()),
                    ],
                )
            };
            let mut c = CurvaFadiga {
                titulo,
                medias: Vec::new(),
                curva: Vec::new(),
                pausas: study.pauses.clone(),
                resumo: String::new(),
                avisos: Vec::new(),
                erro: None,
            };
            match utsim_fit::fatigue::fit_curve(&readings, &study.pauses, 0.95) {
                Ok(fit) => {
                    c.medias = fit.points.iter().map(|p| [p.hour, p.mean]).collect();
                    let last = fit.points.iter().map(|p| p.hour).fold(0.0, f64::max);
                    let end = (last + 0.5).max(1.0);
                    let steps = (end / STEP).ceil() as usize;
                    c.curva = (0..=steps)
                        .map(|k| {
                            let h = (k as f64 * STEP).min(end);
                            [h, fit.t0 * (1.0 + fit.beta * fit.level(h, &study.pauses))]
                        })
                        .collect();
                    c.resumo = fx(
                        "fadiga-resumo",
                        &[
                            ("t0", round(fit.t0, 2).into()),
                            ("acrescimo", round(fit.beta * 100.0, 1).into()),
                            ("lambda", round(fit.work_per_hour, 3).into()),
                            ("mu", round(fit.recovery_per_hour, 3).into()),
                            (
                                "origem",
                                if fit.recovery_estimated {
                                    tx("fadiga-estimada")
                                } else {
                                    tx("fadiga-padrao")
                                }
                                .into(),
                            ),
                        ],
                    );
                    c.avisos = fit.warnings.clone();
                }
                Err(e) => c.erro = Some(fx("fadiga-erro", &[("erro", e.into())])),
            }
            c
        })
        .collect()
}

/// The readings of the time study of `name` as a spreadsheet for the UTSim Estatística:
/// one row per measured time, `tarefa;metodo;hora;tempo`, with `;` and the decimal comma
/// (what its importer reads); `None` without a study.
pub fn csv(model: &Model, name: &str) -> Option<String> {
    let study = model
        .objects
        .iter()
        .find(|o| o.name == name)
        .and_then(|o| o.person.as_deref())
        .and_then(|p| p.study.as_ref())?;
    let num = |x: f64| format!("{x}").replace('.', ",");
    // A `;` or a quote inside a task or method name goes quoted.
    let cell = |s: &str| {
        if s.contains([';', '"', '\n']) {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    };
    let mut out = String::from("tarefa;metodo;hora;tempo\n");
    for r in &study.readings {
        for &t in &r.times {
            out.push_str(&format!(
                "{};{};{};{}\n",
                cell(&r.task),
                cell(&r.method),
                num(r.hour),
                num(t)
            ));
        }
    }
    Some(out)
}

/// The file name of the spreadsheet of `person` of `model` (the file's stem): letters,
/// digits, `-` and `_` only.
pub fn csv_name(model: &str, person: &str) -> String {
    let clean = |s: &str| -> String {
        s.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    };
    format!("{}-{}-estudo.csv", clean(model), clean(person))
}

/// Draws the curves (under the section title, already drawn by the panel). True when
/// "Abrir no UTSim Estatística" was clicked.
pub fn show(ui: &mut Ui, curvas: &[CurvaFadiga], reg: &mut Registry) -> bool {
    ui.add(
        Label::new(
            RichText::new(tx("fadiga-nota"))
                .small()
                .color(colors::TEXT_WEAK),
        )
        .wrap(),
    );
    let texto = tx("fadiga-abrir");
    let r = ui
        .add_sized([ui.available_width(), 26.0], Button::new(texto))
        .on_hover_text(tx("fadiga-abrir-ajuda"));
    named(ui, reg, "Fadiga_AbrirEstatistica", &r, texto);
    let clicked = r.clicked();
    for (k, c) in curvas.iter().enumerate() {
        let r = ui.add(Label::new(RichText::new(&c.titulo).color(colors::TEXT).strong()).wrap());
        named(ui, reg, &format!("Fadiga_Curva_{k}"), &r, &c.titulo);
        if let Some(e) = &c.erro {
            ui.add(Label::new(RichText::new(e).color(colors::WARNING)).wrap());
            continue;
        }
        let top = c
            .medias
            .iter()
            .chain(&c.curva)
            .map(|p| p[1])
            .fold(f64::MIN, f64::max);
        let low = c
            .medias
            .iter()
            .chain(&c.curva)
            .map(|p| p[1])
            .fold(f64::MAX, f64::min);
        let pad = ((top - low) * 0.15).max(0.5);
        let r = Plot::new(format!("utsim_fadiga_{k}"))
            .height(150.0)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .x_axis_label(tx("fadiga-hora"))
            .include_y(low - pad)
            .include_y(top + pad)
            .include_x(0.0)
            .legend(egui_plot::Legend::default())
            .show(ui, |p| {
                for &(a, b) in &c.pausas {
                    p.polygon(
                        Polygon::new(
                            tx("fadiga-pausa"),
                            PlotPoints::new(vec![
                                [a, low - pad],
                                [b, low - pad],
                                [b, top + pad],
                                [a, top + pad],
                            ]),
                        )
                        .fill_color(Color32::from_rgba_unmultiplied(120, 160, 220, 40))
                        .stroke(egui::Stroke::NONE),
                    );
                }
                p.line(
                    Line::new(tx("fadiga-curva"), PlotPoints::new(c.curva.clone()))
                        .color(colors::ACCENT)
                        .width(2.0),
                );
                p.points(
                    Points::new(tx("fadiga-media"), PlotPoints::new(c.medias.clone()))
                        .radius(4.0)
                        .color(colors::WARNING),
                );
            });
        named(
            ui,
            reg,
            &format!("Fadiga_Grafico_{k}"),
            &r.response,
            &c.titulo,
        );
        let r = ui.add(Label::new(RichText::new(&c.resumo).color(colors::TEXT)).wrap());
        named(ui, reg, &format!("Fadiga_Resumo_{k}"), &r, &c.resumo);
        for w in &c.avisos {
            ui.add(
                Label::new(
                    RichText::new(format!("⚠ {w}"))
                        .small()
                        .color(colors::WARNING),
                )
                .wrap(),
            );
        }
    }
    clicked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_curve_of_a_person_with_a_study_follows_the_fit() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../examples/pessoas_tempos_proprios.ron"
        );
        let model = Model::load(std::path::Path::new(path)).unwrap();
        let cs = curvas(&model, "ana");
        assert_eq!(cs.len(), 1);
        let c = &cs[0];
        assert!(c.erro.is_none(), "{:?}", c.erro);
        assert_eq!(c.medias.len(), 5);
        assert_eq!(c.pausas, vec![(4.0, 4.5)]);
        assert!(c.titulo.contains("torno"), "{}", c.titulo);
        assert!(!c.resumo.is_empty());
        // The curve covers the hours of the readings and passes near the means.
        assert!(c.curva.first().unwrap()[0] == 0.0 && c.curva.last().unwrap()[0] >= 7.5);
        let at = |h: f64| {
            c.curva
                .iter()
                .min_by(|a, b| (a[0] - h).abs().total_cmp(&(b[0] - h).abs()))
                .unwrap()[1]
        };
        for m in &c.medias {
            assert!((at(m[0]) - m[1]).abs() < 4.0, "{m:?} vs {}", at(m[0]));
        }
        // Rested after the pause: the curve falls across it.
        assert!(at(4.6) < at(3.9));
        // The spreadsheet for the UTSim Estatística: one row per measured time.
        let text = csv(&model, "ana").unwrap();
        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("tarefa;metodo;hora;tempo"));
        let rows: Vec<&str> = lines.collect();
        let times: usize = model
            .objects
            .iter()
            .find(|o| o.name == "ana")
            .and_then(|o| o.person.as_deref())
            .and_then(|p| p.study.as_ref())
            .map(|s| s.readings.iter().map(|r| r.times.len()).sum())
            .unwrap();
        assert_eq!(rows.len(), times);
        assert!(
            rows.iter()
                .all(|r| r.starts_with("torno;") && r.split(';').count() == 4),
            "{rows:?}"
        );
        assert!(rows.iter().any(|r| r.contains(',')), "decimal comma");
        assert!(csv(&model, "bruno").is_none());
        assert_eq!(csv_name("Linha 1", "ana/b"), "Linha_1-ana_b-estudo.csv");
        // No study, no curve.
        assert!(curvas(&model, "bruno").is_empty());
        assert!(curvas(&model, "expedicao").is_empty());
    }
}
