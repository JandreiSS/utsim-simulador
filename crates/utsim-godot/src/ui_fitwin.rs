//! "Valores reais…": the large window where the real values of a distribution field are
//! typed, pasted or imported, and every candidate distribution is compared by every
//! measure at once (AIC, BIC, log-likelihood, Kolmogorov-Smirnov, Anderson-Darling and
//! χ², with their p-values and the position of each candidate in each one). The user
//! picks a row and "Usar esta distribuição"; the criterion is only the first suggestion.
//! Each measure, plot and family has an ⓘ with a short explanation
//! ([`utsim_fit::compare::EXPLANATIONS`]).
//!
//! The fit runs on its own thread ([`FitWin::start`]); the window polls it each frame.
//! Actions go to the bridge as text: `valores:usar\t<família>`, `valores:empirica`,
//! `valores:csv`, `valores:relatorio`, `valores:fechar`.

use crate::texto::{fx, tx, vx};
use std::sync::{Arc, Mutex};

use egui::{
    Align2, Color32, Id, Label, Order, RichText, ScrollArea, Sense, Stroke, TextEdit, Ui, Vec2,
};
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints, Points};
use utsim_fit::auto::{AutoConfig, AutoFit, auto_fit};
use utsim_fit::compare::{self, Comparison, Measure};
use utsim_ui_tema::{colors, sizes, widgets};

use crate::egui_widgets::{Registry, WidgetInfo};
use crate::ui_props::{named, named_toggle};
use crate::ui_results::num;

/// The result of a fit run on the thread: the data it was run with and the fit.
type Slot = Arc<Mutex<Option<(Vec<f64>, Result<AutoFit, String>)>>>;

/// The state of the window (open while `Some` in the host).
pub struct FitWin {
    /// The object and the distribution field (`process`, `interarrival`...).
    pub object: String,
    pub path: String,
    /// The values as typed, one per row.
    pub rows: Vec<String>,
    /// The text box where values are pasted (several lines, decimal comma).
    pub paste: String,
    /// The values are increasing instants: the fit uses the differences.
    pub timestamps: bool,
    running: Option<Slot>,
    /// The data and the fit of the last run.
    pub fit: Option<(Vec<f64>, Arc<AutoFit>)>,
    pub error: String,
    /// The order of the table.
    pub sort: Measure,
    /// The candidate chosen in the table (index into the ranking), `None` = empirical.
    pub selected: Option<usize>,
    /// Q-Q instead of P-P.
    pub qq: bool,
    /// The explanation open (a key of [`compare::EXPLANATIONS`]).
    pub info: Option<String>,
    /// A message for the user (import, report).
    pub note: String,
}

impl FitWin {
    pub fn new(object: &str, path: &str, rows: Vec<String>, timestamps: bool) -> FitWin {
        FitWin {
            object: object.to_owned(),
            path: path.to_owned(),
            rows,
            paste: String::new(),
            timestamps,
            running: None,
            fit: None,
            error: String::new(),
            sort: Measure::Aic,
            selected: None,
            qq: false,
            info: None,
            note: String::new(),
        }
    }

    /// The field, as the model names it (`objeto.campo`).
    pub fn field(&self) -> String {
        format!("{}.{}", self.object, self.path)
    }

    /// The values typed (decimal comma accepted), or why they cannot be read.
    pub fn raw_values(&self) -> Result<Vec<f64>, String> {
        let mut out = Vec::new();
        for (k, r) in self.rows.iter().enumerate() {
            let t = r.trim();
            if t.is_empty() {
                continue;
            }
            match parse_number(t) {
                Some(x) => out.push(x),
                None => {
                    return Err(fx(
                        "godot-ui-fitwin-linha_nao_e_um_numero",
                        &[("k", vx(&(k + 1))), ("t", vx(&t))],
                    ));
                }
            }
        }
        Ok(out)
    }

    /// The sample the fit uses: the values, or the intervals between the instants.
    pub fn sample(&self) -> Result<Vec<f64>, String> {
        let v = self.raw_values()?;
        if !self.timestamps {
            return Ok(v);
        }
        if v.windows(2).any(|w| w[1] < w[0]) {
            return Err(tx("godot-ui-fitwin-os_instantes_precisam_ser_crescentes").into());
        }
        Ok(v.windows(2).map(|w| w[1] - w[0]).collect())
    }

    /// Adds the values of `text` (one per line, or separated by `;`, tabs or spaces; a
    /// header or other text is skipped). Returns how many were added.
    pub fn add_text(&mut self, text: &str) -> usize {
        let before = self.rows.len();
        for cell in text.split(['\n', '\r', ';', '\t', ' ']) {
            let t = cell.trim();
            if !t.is_empty() && parse_number(t).is_some() {
                self.rows.push(t.to_owned());
            }
        }
        self.rows.len() - before
    }

    /// Starts the comparison of every candidate on its own thread.
    pub fn start(&mut self) {
        let data = match self.sample() {
            Ok(d) if d.len() >= 2 => d,
            Ok(_) => {
                self.error = tx("godot-ui-fitwin-digite_ou_cole_pelo_menos").into();
                return;
            }
            Err(e) => {
                self.error = e;
                return;
            }
        };
        self.error.clear();
        let slot: Slot = Arc::new(Mutex::new(None));
        let out = slot.clone();
        let name = self.field();
        std::thread::spawn(move || {
            let r = auto_fit(&name, &data, &AutoConfig::default());
            if let Ok(mut s) = out.lock() {
                *s = Some((data, r));
            }
        });
        self.running = Some(slot);
    }

    pub fn running(&self) -> bool {
        self.running.is_some()
    }

    /// Takes the result of the thread when it is done. True when something changed.
    pub fn poll(&mut self) -> bool {
        let Some(slot) = &self.running else {
            return false;
        };
        let done = slot.lock().ok().and_then(|mut s| s.take());
        let Some((data, r)) = done else {
            return false;
        };
        self.running = None;
        match r {
            Ok(fit) => {
                // The criterion's choice is only the first suggestion.
                self.selected = fit.chosen;
                self.fit = Some((data, Arc::new(fit)));
            }
            Err(e) => {
                self.fit = None;
                self.error = e;
            }
        }
        true
    }

    /// Waits for the thread (the tests and the scripted runs).
    pub fn wait(&mut self) {
        while self.running.is_some() {
            if !self.poll() {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        }
    }

    /// The fit is for the values now in the table.
    pub fn current(&self) -> bool {
        match (&self.fit, self.sample()) {
            (Some((d, _)), Ok(s)) => *d == s,
            _ => false,
        }
    }

    /// The name of the family selected ("Empírica" for none).
    pub fn selected_family(&self) -> String {
        match (&self.fit, self.selected) {
            (Some((_, f)), Some(i)) => f.report.ranking.candidates[i].family.clone(),
            _ => utsim_fit::auto::EMPIRICAL.to_owned(),
        }
    }

    /// The draft the panel's "aplicar" takes for the family chosen (`Empirical` or a
    /// family name), with the values typed.
    pub fn draft(&self, choose: &str) -> serde_json::Value {
        serde_json::json!({
            "origem": "valores", "source": "", "column": "",
            "text": self.rows.iter().map(|r| r.trim()).filter(|r| !r.is_empty()).collect::<Vec<_>>().join("\n"),
            "timestamps": self.timestamps, "choose": choose, "criterion": "aic",
        })
    }

    /// Writes the HTML report of the comparison, with the chosen distribution, into `dir`.
    /// Returns the file written.
    pub fn write_report(&self, dir: &std::path::Path) -> Result<std::path::PathBuf, String> {
        let (data, _) = self
            .fit
            .as_ref()
            .ok_or(tx("godot-ui-fitwin-compare_as_distribuicoes_antes"))?;
        let choose = match (&self.fit, self.selected) {
            (Some((_, f)), Some(i)) => {
                utsim_fit::auto::Choice::Family(f.report.ranking.candidates[i].family.clone())
            }
            _ => utsim_fit::auto::Choice::Empirical,
        };
        let config = AutoConfig {
            choose,
            ..AutoConfig::default()
        };
        let fit = auto_fit(&self.field(), data, &config)?;
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let file = dir.join(format!(
            "ajuste_{}.html",
            self.field().replace(['.', '/', '\\'], "_")
        ));
        std::fs::write(
            &file,
            fit.to_html(tx("godot-ui-fitwin-utsim_3d_valores_reais")),
        )
        .map_err(|e| format!("{}: {e}", file.display()))?;
        Ok(file)
    }
}

/// A number typed by a person: decimal comma or point, thousands not accepted.
pub fn parse_number(t: &str) -> Option<f64> {
    let t = t.trim();
    let t = if t.contains(',') && !t.contains('.') {
        t.replace(',', ".")
    } else {
        t.to_owned()
    };
    t.parse::<f64>().ok().filter(|x| x.is_finite())
}

/// Descriptive statistics of the sample for the left column.
pub fn describe(v: &[f64]) -> Vec<(&'static str, String)> {
    if v.is_empty() {
        return Vec::new();
    }
    let n = v.len() as f64;
    let mean = v.iter().sum::<f64>() / n;
    let var = if v.len() > 1 {
        v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
    } else {
        0.0
    };
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    let q = |p: f64| {
        let h = (s.len() - 1) as f64 * p;
        let (lo, hi) = (h.floor() as usize, h.ceil() as usize);
        s[lo] + (s[hi] - s[lo]) * (h - lo as f64)
    };
    let sd = var.sqrt();
    vec![
        ("n", format!("{}", v.len())),
        ("média", num(mean)),
        (tx("godot-ui-fitwin-desvio_padrao"), num(sd)),
        (
            "CV",
            if mean.abs() > 0.0 {
                num(sd / mean.abs())
            } else {
                "-".into()
            },
        ),
        ("mínimo", num(s[0])),
        ("Q1", num(q(0.25))),
        ("mediana", num(q(0.5))),
        ("Q3", num(q(0.75))),
        ("máximo", num(s[s.len() - 1])),
    ]
}

/// Bins of the histogram: (start, width, counts), about √n bins between 5 and 30.
pub fn bins(v: &[f64]) -> (f64, f64, Vec<f64>) {
    if v.is_empty() {
        return (0.0, 1.0, Vec::new());
    }
    let lo = v.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let k = ((v.len() as f64).sqrt().round() as usize).clamp(5, 30);
    let width = if hi > lo { (hi - lo) / k as f64 } else { 1.0 };
    let mut counts = vec![0.0; k];
    for x in v {
        let j = (((x - lo) / width).floor() as usize).min(k - 1);
        counts[j] += 1.0;
    }
    (lo, width, counts)
}

const CURVE: Color32 = Color32::from_rgb(255, 160, 60);
const CURVE_DIM: Color32 = Color32::from_rgba_premultiplied(120, 130, 150, 120);
const BAR: Color32 = Color32::from_rgb(90, 150, 240);

/// Draws the window over everything (a dim backdrop keeps the rest from clicks).
pub fn show(ctx: &egui::Context, w: &mut FitWin, reg: &mut Registry, out: &mut Vec<String>) {
    if w.poll() {
        ctx.request_repaint();
    }
    if w.running() {
        ctx.request_repaint_after(std::time::Duration::from_millis(150));
    }
    let screen = ctx.content_rect();
    egui::Area::new(Id::new("utsim_valores_fundo"))
        .order(Order::Middle)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(screen.size(), Sense::click_and_drag());
            ui.painter()
                .rect_filled(rect, 0.0, Color32::from_black_alpha(140));
        });
    let size = Vec2::new(
        (screen.width() - 60.0).clamp(640.0, 1500.0),
        (screen.height() - 60.0).clamp(420.0, 1000.0),
    );
    let mut open = true;
    egui::Window::new(fx(
        "godot-ui-fitwin-valores_reais",
        &[("field", vx(&w.field()))],
    ))
    .id(Id::new("utsim_valores_janela"))
    .order(Order::Foreground)
    .collapsible(false)
    .resizable(true)
    .default_size(size)
    .min_size(Vec2::new(600.0, 380.0))
    .max_size(Vec2::new(screen.width() - 20.0, screen.height() - 20.0))
    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
    .frame(
        egui::Frame::window(&ctx.global_style())
            .fill(colors::PANEL)
            .inner_margin(egui::Margin::same(12)),
    )
    .open(&mut open)
    .show(ctx, |ui| {
        reg.put_info(
            "Valores_Janela",
            WidgetInfo {
                rect: ui.max_rect(),
                text: w.field(),
                enabled: true,
                ..Default::default()
            },
        );
        body(ui, w, reg, out);
    });
    if !open {
        out.push("valores:fechar".into());
    }
}

/// `x` when it is a finite size, else `fallback` (egui measures a new window with an
/// infinite space first).
fn finite(x: f32, fallback: f32) -> f32 {
    if x.is_finite() { x } else { fallback }
}

fn body(ui: &mut Ui, w: &mut FitWin, reg: &mut Registry, out: &mut Vec<String>) {
    let avail = Vec2::new(
        finite(ui.available_width(), 1000.0),
        finite(ui.available_height(), 600.0),
    );
    ui.set_max_height(avail.y);
    let left_w = (avail.x * 0.28).clamp(240.0, 360.0);
    ui.horizontal_top(|ui| {
        ui.allocate_ui_with_layout(
            Vec2::new(left_w, avail.y),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(left_w);
                ScrollArea::vertical()
                    .id_salt("utsim_valores_esquerda")
                    .max_height(avail.y)
                    .auto_shrink([false, false])
                    .show(ui, |ui| values_column(ui, w, reg, out));
            },
        );
        ui.separator();
        ui.vertical(|ui| {
            compare_column(ui, w, reg, out);
        });
    });
}

fn small(text: &str) -> RichText {
    RichText::new(text)
        .size(sizes::SMALL)
        .color(colors::TEXT_WEAK)
}

fn heading(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).strong().color(colors::HEADING));
}

/// A small round "i" that opens the explanation of `key`.
fn info_button(ui: &mut Ui, w: &mut FitWin, reg: &mut Registry, key: &str) {
    let (rect, r) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::click());
    let on = w.info.as_deref() == Some(key);
    let c = if on || r.hovered() {
        colors::ACCENT
    } else {
        colors::TEXT_WEAK
    };
    ui.painter()
        .circle_stroke(rect.center(), 7.0, Stroke::new(1.2, c));
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        "i",
        egui::FontId::proportional(11.0),
        c,
    );
    let title = compare::explain(key).map_or(key, |e| e.title);
    let r = r.on_hover_text(fx("godot-ui-fitwin-o_que_e", &[("title", vx(&title))]));
    named(
        ui,
        reg,
        &format!("Valores_Info_{}", key_name(key)),
        &r,
        title,
    );
    if r.clicked() {
        w.info = if on { None } else { Some(key.to_owned()) };
    }
}

/// A widget name from an explanation key (family names have spaces and accents).
pub fn key_name(key: &str) -> String {
    key.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn values_column(ui: &mut Ui, w: &mut FitWin, reg: &mut Registry, out: &mut Vec<String>) {
    heading(ui, tx("godot-ui-fitwin-valores_observados"));
    ui.label(small(tx("godot-ui-fitwin-digite_cole_da_planilha_varias")));
    let r = ui.add(
        TextEdit::multiline(&mut w.paste)
            .id(Id::new("utsim_valores_colar"))
            .desired_rows(3)
            .desired_width(f32::INFINITY)
            .hint_text(tx("godot-ui-fitwin-cole_aqui_e_clique_adicionar")),
    );
    named(ui, reg, "Valores_Colar", &r, &w.paste);
    ui.horizontal_wrapped(|ui| {
        let r = ui.add(widgets::primary(tx("godot-ui-fitwin-adicionar")));
        named(ui, reg, "Valores_Adicionar", &r, "Adicionar");
        if r.clicked() {
            let text = std::mem::take(&mut w.paste);
            let n = w.add_text(&text);
            w.note = fx("godot-ui-fitwin-valor_es_adicionados", &[("n", vx(&n))]);
        }
        let r = ui
            .button(tx("godot-ui-fitwin-importar_csv"))
            .on_hover_text(tx("godot-ui-fitwin-le_os_valores_de_um"));
        named(ui, reg, "Valores_Csv", &r, "Importar CSV…");
        if r.clicked() {
            out.push("valores:csv".into());
        }
        let r = ui
            .button(tx("godot-ui-fitwin-limpar"))
            .on_hover_text(tx("godot-ui-fitwin-apaga_todos_os_valores_da"));
        named(ui, reg, "Valores_Limpar", &r, "Limpar");
        if r.clicked() {
            w.rows.clear();
        }
    });
    let r = ui
        .checkbox(
            &mut w.timestamps,
            tx("godot-ui-fitwin-sao_instantes_crescentes_usar_os"),
        )
        .on_hover_text(tx("godot-ui-fitwin-marque_quando_os_valores_sao"));
    named_toggle(ui, reg, "Valores_Instantes", &r, "instantes", w.timestamps);
    if !w.note.is_empty() {
        ui.label(small(&w.note));
    }
    // The table of values: edit in place, × removes the row.
    let n = w.rows.len();
    let r = ui.label(small(&fx("godot-ui-fitwin-valor_es", &[("n", vx(&n))])));
    named(ui, reg, "Valores_Contagem", &r, &n.to_string());
    let mut remove = None;
    ScrollArea::vertical()
        .id_salt("utsim_valores_tabela")
        .max_height(220.0)
        .auto_shrink([false, true])
        .show_rows(ui, 22.0, n, |ui, range| {
            for k in range {
                ui.horizontal(|ui| {
                    ui.label(small(&format!("{:>4}", k + 1)));
                    let bad = parse_number(&w.rows[k]).is_none() && !w.rows[k].trim().is_empty();
                    let mut edit = TextEdit::singleline(&mut w.rows[k])
                        .id(Id::new(("utsim_valores_linha", k)))
                        .desired_width(finite(ui.available_width(), 200.0) - 30.0);
                    if bad {
                        edit = edit.text_color(colors::ERROR);
                    }
                    let r = ui.add(edit);
                    named(ui, reg, &format!("Valores_Linha_{k}"), &r, &w.rows[k]);
                    let r = ui
                        .small_button("×")
                        .on_hover_text(tx("godot-ui-fitwin-apaga_este_valor"));
                    named(ui, reg, &format!("Valores_Remover_{k}"), &r, "×");
                    if r.clicked() {
                        remove = Some(k);
                    }
                });
            }
        });
    if let Some(k) = remove {
        w.rows.remove(k);
    }
    ui.add_space(6.0);
    match w.sample() {
        Ok(s) if !s.is_empty() => {
            heading(
                ui,
                if w.timestamps {
                    tx("godot-ui-fitwin-intervalos")
                } else {
                    tx("godot-ui-fitwin-estatisticas")
                },
            );
            let d = describe(&s);
            let text: Vec<String> = d.iter().map(|(k, v)| format!("{k} {v}")).collect();
            egui::Grid::new("utsim_valores_desc")
                .num_columns(2)
                .spacing([12.0, 2.0])
                .show(ui, |ui| {
                    for (k, v) in &d {
                        ui.label(small(k));
                        ui.label(RichText::new(v).color(colors::TEXT));
                        ui.end_row();
                    }
                });
            reg.put_info(
                "Valores_Resumo",
                WidgetInfo {
                    rect: ui.min_rect(),
                    text: text.join("; "),
                    enabled: true,
                    ..Default::default()
                },
            );
        }
        Ok(_) => {}
        Err(e) => {
            ui.label(RichText::new(e).color(colors::ERROR));
        }
    }
}

fn compare_column(ui: &mut Ui, w: &mut FitWin, reg: &mut Registry, out: &mut Vec<String>) {
    ui.horizontal(|ui| {
        heading(ui, tx("godot-ui-fitwin-comparacao_das_distribuicoes"));
        let label = if w.fit.is_some() {
            tx("godot-ui-fitwin-comparar_de_novo")
        } else {
            tx("godot-ui-fitwin-comparar_todas")
        };
        let enabled = !w.running() && !w.rows.is_empty();
        let r = ui
            .add_enabled(enabled, widgets::primary(label))
            .on_hover_text(tx(
                "godot-ui-fitwin-ajusta_todas_as_distribuicoes_candidatas",
            ));
        named(ui, reg, "Valores_Comparar", &r, label);
        if r.clicked() {
            w.start();
        }
        if w.running() {
            ui.spinner();
            ui.label(small(tx("godot-ui-fitwin-ajustando_todas_as_candidatas")));
        }
    });
    if !w.error.is_empty() {
        let r = ui.add(Label::new(RichText::new(&w.error).color(colors::ERROR)).wrap());
        named(ui, reg, "Valores_Erro", &r, &w.error);
    }
    if let Some(key) = w.info.clone() {
        explanation_box(ui, w, reg, &key);
    }
    let Some((data, fit)) = w.fit.clone() else {
        ui.label(small(tx("godot-ui-fitwin-depois_de_comparar_a_tabela")));
        return;
    };
    if !w.current() {
        ui.label(
            RichText::new(tx("godot-ui-fitwin-os_valores_mudaram_compare_de"))
                .color(colors::WARNING),
        );
    }
    let cands = &fit.report.ranking.candidates;
    let cmp = Comparison::of(cands);
    let suggested = fit.chosen;
    ui.horizontal_wrapped(|ui| {
        ui.label(small(&fx(
            "godot-ui-fitwin-ordenada_por_sugestao_inicial_aic",
            &[
                ("label", vx(&w.sort.label())),
                (
                    "family",
                    vx(&suggested.map_or(utsim_fit::auto::EMPIRICAL, |i| cands[i].family.as_str())),
                ),
            ],
        )));
    });
    let avail_h = finite(ui.available_height(), 600.0);
    let table_h = (avail_h * 0.45).max(140.0);
    comparison_table(ui, w, reg, cands, &cmp, table_h);
    ui.add_space(6.0);
    // The plots of the row selected.
    let plot_h = (finite(ui.available_height(), 300.0) - 44.0).max(120.0);
    ui.horizontal_top(|ui| {
        let half = (finite(ui.available_width(), 700.0) - 12.0) / 2.0;
        let down = egui::Layout::top_down(egui::Align::Min);
        ui.allocate_ui_with_layout(Vec2::new(half, plot_h), down, |ui| {
            ui.set_width(half);
            fit_histogram(ui, w, reg, &data, &fit, plot_h - 26.0);
        });
        ui.allocate_ui_with_layout(Vec2::new(half, plot_h), down, |ui| {
            ui.set_width(half);
            prob_plot(ui, w, reg, &data, &fit, plot_h - 26.0);
        });
    });
    ui.horizontal(|ui| {
        let fam = w.selected_family();
        let label = fx(
            "godot-ui-fitwin-usar_esta_distribuicao",
            &[("fam", vx(&fam))],
        );
        let r = ui
            .add(widgets::primary(&label))
            .on_hover_text(tx("godot-ui-fitwin-passa_a_usar_esta_distribuicao"));
        named(ui, reg, "Valores_Usar", &r, &label);
        if r.clicked() {
            if w.selected.is_some() {
                out.push(format!("valores:usar\t{fam}"));
            } else {
                out.push("valores:empirica".into());
            }
        }
        let r = ui
            .button(tx("godot-ui-fitwin-usar_a_empirica"))
            .on_hover_text(tx("godot-ui-fitwin-usa_os_proprios_valores_observados"));
        named(ui, reg, "Valores_Empirica", &r, "Usar a empírica");
        if r.clicked() {
            out.push("valores:empirica".into());
        }
        let r = ui
            .button(tx("godot-ui-fitwin-gerar_relatorio"))
            .on_hover_text(tx("godot-ui-fitwin-grava_o_relatorio_html_com"));
        named(ui, reg, "Valores_Relatorio", &r, "Gerar relatório");
        if r.clicked() {
            out.push("valores:relatorio".into());
        }
        let r = ui.button(tx("godot-ui-fitwin-fechar"));
        named(ui, reg, "Valores_Fechar", &r, "Fechar");
        if r.clicked() {
            out.push("valores:fechar".into());
        }
    });
}

fn explanation_box(ui: &mut Ui, w: &mut FitWin, reg: &mut Registry, key: &str) {
    let Some(e) = compare::explain(key) else {
        return;
    };
    egui::Frame::group(ui.style())
        .fill(colors::FIELD)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(e.title).strong().color(colors::ACCENT));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let r = ui
                        .small_button("×")
                        .on_hover_text(tx("godot-ui-fitwin-fecha_a_explicacao"));
                    named(ui, reg, "Valores_Explicacao_Fechar", &r, "×");
                    if r.clicked() {
                        w.info = None;
                    }
                });
            });
            let text = fx(
                "godot-ui-fitwin-como_ler_quando_usar_cuidados",
                &[
                    ("what", vx(&e.what)),
                    ("read", vx(&e.read)),
                    ("when", vx(&e.when)),
                    ("care", vx(&e.care)),
                ],
            );
            for (k, line) in text.lines().enumerate() {
                let rt = if k == 0 {
                    RichText::new(line).color(colors::TEXT)
                } else {
                    RichText::new(line).color(colors::TEXT_WEAK)
                };
                ui.add(Label::new(rt).wrap());
            }
            reg.put_info(
                "Valores_Explicacao",
                WidgetInfo {
                    rect: ui.min_rect(),
                    text: format!("{}\n{text}", e.title),
                    enabled: true,
                    ..Default::default()
                },
            );
        });
}

fn comparison_table(
    ui: &mut Ui,
    w: &mut FitWin,
    reg: &mut Registry,
    cands: &[utsim_fit::CandidateResult],
    cmp: &Comparison,
    height: f32,
) {
    let order = cmp.order_by(w.sort);
    let mut summary = Vec::new();
    let color_of = |c: &utsim_fit::CandidateResult| {
        if c.fit_error.is_some() {
            colors::TEXT_WEAK
        } else if c.accepted() {
            colors::TEXT
        } else {
            colors::WARNING
        }
    };
    // The family column stays put; the measures scroll sideways beside it. Both grids
    // have the same row height, so the rows line up.
    ScrollArea::vertical()
        .id_salt("utsim_valores_comparacao")
        .max_height(height)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                egui::Grid::new("utsim_valores_familias")
                    .striped(true)
                    .min_row_height(ROW)
                    .spacing([14.0, 4.0])
                    .show(ui, |ui| {
                        ui.label(small("família"));
                        ui.end_row();
                        for (pos, &i) in order.iter().enumerate() {
                            let c = &cands[i];
                            let sel = w.selected == Some(i);
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 6.0;
                                let fam = if sel {
                                    RichText::new(format!("▶ {}", c.family))
                                        .strong()
                                        .color(colors::ACCENT)
                                } else {
                                    RichText::new(&c.family).color(color_of(c))
                                };
                                let r = ui
                                    .add_enabled(
                                        c.fit_error.is_none(),
                                        egui::Button::new(fam).frame(false),
                                    )
                                    .on_hover_text(if c.accepted() {
                                        tx("godot-ui-fitwin-aceita_pelos_testes_clique_para")
                                    } else if c.fit_error.is_some() {
                                        tx("godot-ui-fitwin-nao_se_aplica_a_estes")
                                    } else {
                                        tx("godot-ui-fitwin-rejeitada_por_algum_teste_valor")
                                    });
                                named(ui, reg, &format!("Valores_Cand_{pos}"), &r, &c.family);
                                if r.clicked() {
                                    w.selected = Some(i);
                                }
                                if compare::explain(&c.family).is_some() {
                                    info_button(ui, w, reg, &c.family);
                                }
                            });
                            ui.end_row();
                        }
                    });
                ui.separator();
                ScrollArea::horizontal()
                    .id_salt("utsim_valores_medidas")
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        egui::Grid::new("utsim_valores_grade")
                            .striped(true)
                            .min_row_height(ROW)
                            .spacing([14.0, 4.0])
                            .show(ui, |ui| {
                                // Header: the measures sort the table; each one has its ⓘ.
                                ui.label(small("parâmetros"));
                                for m in Measure::ALL {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 4.0;
                                        let on = w.sort == m;
                                        let text = if on {
                                            RichText::new(format!("{} ▼", m.short()))
                                                .strong()
                                                .color(colors::ACCENT)
                                        } else {
                                            RichText::new(m.short()).color(colors::TEXT_WEAK)
                                        };
                                        let r = ui
                                            .add(egui::Button::new(text).frame(false))
                                            .on_hover_text(fx(
                                                "godot-ui-fitwin-ordena_por",
                                                &[
                                                    ("label", vx(&m.label())),
                                                    (
                                                        "melhor",
                                                        vx(&if m.lower_is_better() {
                                                            "menor é melhor"
                                                        } else {
                                                            "maior é melhor"
                                                        }),
                                                    ),
                                                ],
                                            ));
                                        named(
                                            ui,
                                            reg,
                                            &format!("Valores_Ordem_{}", m.key()),
                                            &r,
                                            m.short(),
                                        );
                                        if r.clicked() {
                                            w.sort = m;
                                        }
                                        info_button(ui, w, reg, m.key());
                                    });
                                    if m.p_value_applies() {
                                        ui.label(small(&format!("{} p", m.short())));
                                    }
                                }
                                ui.label(small(tx("godot-ui-fitwin-escolhida_por")));
                                ui.end_row();
                                for &i in &order {
                                    let c = &cands[i];
                                    let color = color_of(c);
                                    let params = match &c.fit_error {
                                        Some(e) => e.clone(),
                                        None => c
                                            .params
                                            .iter()
                                            .map(|(k, v)| format!("{k} = {}", num(*v)))
                                            .collect::<Vec<_>>()
                                            .join(", "),
                                    };
                                    ui.label(RichText::new(params).size(sizes::SMALL).color(color));
                                    for (m, measure) in Measure::ALL.iter().enumerate() {
                                        let v = measure.value(c).map_or("-".into(), |v| {
                                            format!("{} ({})", num(v), cmp.ranks[i][m].unwrap_or(0))
                                        });
                                        let mut t = RichText::new(v).color(color);
                                        if cmp.winners[m] == Some(i) {
                                            t = t.strong();
                                        }
                                        ui.label(t);
                                        if measure.p_value_applies() {
                                            let p = measure.p_value(c);
                                            let pc = match p {
                                                Some(p) if p < 0.05 => colors::WARNING,
                                                _ => color,
                                            };
                                            ui.label(
                                                RichText::new(p.map_or("-".into(), num)).color(pc),
                                            );
                                        }
                                    }
                                    let by: Vec<&str> =
                                        cmp.chosen_by(i).iter().map(|m| m.short()).collect();
                                    ui.label(RichText::new(by.join(", ")).color(colors::ACCENT));
                                    ui.end_row();
                                    summary.push(format!("{}:{}", c.family, by.join("+")));
                                }
                            });
                    });
            });
        });
    let text = format!(
        "linhas={} ordem={} escolhida={} criterios={}",
        cands.len(),
        w.sort.key(),
        w.selected_family(),
        summary
            .iter()
            .filter(|s| !s.ends_with(':'))
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    );
    reg.put_info(
        "Valores_Tabela",
        WidgetInfo {
            rect: ui.min_rect(),
            text,
            enabled: true,
            count: Some(cands.len()),
            ..Default::default()
        },
    );
}

/// The height of a row of the comparison (both halves, so they line up).
const ROW: f32 = 24.0;

fn still(p: Plot<'_>) -> Plot<'_> {
    p.allow_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .allow_double_click_reset(false)
}

/// The histogram with the curves of the fitted candidates (the one selected bright).
fn fit_histogram(
    ui: &mut Ui,
    w: &mut FitWin,
    reg: &mut Registry,
    data: &[f64],
    fit: &AutoFit,
    h: f32,
) {
    ui.label(small(&fx(
        "godot-ui-fitwin-histograma_e_curvas_em_destaque",
        &[("selected_family", vx(&w.selected_family()))],
    )));
    let (lo, width, counts) = bins(data);
    let hi = lo + width * counts.len() as f64;
    let cands = &fit.report.ranking.candidates;
    let mut curves = 0;
    let r = still(Plot::new("utsim_valores_hist"))
        .height(h.max(80.0))
        .include_y(0.0)
        .show(ui, |p| {
            let bars: Vec<Bar> = counts
                .iter()
                .enumerate()
                .map(|(k, c)| {
                    Bar::new(lo + (k as f64 + 0.5) * width, *c)
                        .width(width * 0.92)
                        .fill(BAR.gamma_multiply(0.7))
                        .stroke(Stroke::NONE)
                })
                .collect();
            p.bar_chart(BarChart::new("dados", bars));
            let discrete = fit.support.integer;
            for (i, c) in cands.iter().enumerate() {
                if c.fit_error.is_some() || (!c.accepted() && w.selected != Some(i)) {
                    continue;
                }
                let disc = matches!(
                    c.family.as_str(),
                    "Poisson" | "Geométrica" | "Binomial" | "Binomial negativa"
                );
                if disc && !discrete {
                    continue;
                }
                let pts = compare::density_curve(c, lo, hi, 120, data.len(), width, disc);
                if pts.is_empty() {
                    continue;
                }
                let sel = w.selected == Some(i);
                let line = Line::new(c.family.clone(), PlotPoints::new(pts))
                    .color(if sel { CURVE } else { CURVE_DIM })
                    .width(if sel { 2.5 } else { 1.0 });
                p.line(line);
                curves += 1;
            }
        })
        .response;
    reg.put_info(
        "Valores_Grafico_Ajuste",
        WidgetInfo {
            rect: r.rect,
            text: format!("barras={} curvas={curves}", counts.len()),
            enabled: true,
            count: Some(counts.len()),
            ..Default::default()
        },
    );
}

/// P-P or Q-Q of the selected candidate.
fn prob_plot(ui: &mut Ui, w: &mut FitWin, reg: &mut Registry, data: &[f64], fit: &AutoFit, h: f32) {
    ui.horizontal(|ui| {
        for (qq, label, key) in [(false, "P-P", "pp"), (true, "Q-Q", "qq")] {
            let r = ui.selectable_label(w.qq == qq, label);
            named(
                ui,
                reg,
                &format!("Valores_{}", label.replace('-', "")),
                &r,
                label,
            );
            if r.clicked() {
                w.qq = qq;
            }
            info_button(ui, w, reg, key);
        }
    });
    let Some(i) = w.selected else {
        ui.label(small(tx("godot-ui-fitwin-a_empirica_reproduz_os_proprios")));
        return;
    };
    let c = &fit.report.ranking.candidates[i];
    let pts = if w.qq {
        compare::qq_points(data, c, 400)
    } else {
        compare::pp_points(data, c, 400)
    };
    let (lo, hi) = pts
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), p| {
            (l.min(p[0]).min(p[1]), h.max(p[0]).max(p[1]))
        });
    let n = pts.len();
    let r = still(Plot::new(("utsim_valores_prob", w.qq)))
        .height(h.max(80.0))
        .data_aspect(1.0)
        .show(ui, |p| {
            if lo.is_finite() && hi.is_finite() {
                p.line(
                    Line::new("diagonal", PlotPoints::new(vec![[lo, lo], [hi, hi]]))
                        .color(colors::TEXT_WEAK)
                        .width(1.0),
                );
            }
            p.points(
                Points::new(c.family.clone(), PlotPoints::new(pts))
                    .color(CURVE)
                    .radius(2.0),
            );
        })
        .response;
    reg.put_info(
        "Valores_Grafico_Prob",
        WidgetInfo {
            rect: r.rect,
            text: format!(
                "{} {} pontos={n}",
                if w.qq { "Q-Q" } else { "P-P" },
                c.family
            ),
            enabled: true,
            count: Some(n),
            ..Default::default()
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exp_values(n: usize) -> Vec<String> {
        let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
        (0..n)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                let u = (x >> 11) as f64 / (1u64 << 53) as f64;
                format!("{:.3}", -4.0 * (1.0 - u).ln()).replace('.', ",")
            })
            .collect()
    }

    #[test]
    fn values_are_pasted_with_decimal_comma_and_instants_become_intervals() {
        let mut w = FitWin::new("torno", "process", Vec::new(), false);
        assert_eq!(w.add_text("tempo\n1,5\n2,25; 3\n\t4.5 x"), 4);
        assert_eq!(w.raw_values().unwrap(), vec![1.5, 2.25, 3.0, 4.5]);
        w.timestamps = true;
        assert_eq!(w.sample().unwrap(), vec![0.75, 0.75, 1.5]);
        w.rows[1] = "0,5".into();
        assert!(w.sample().unwrap_err().contains("crescentes"));
        w.rows[1] = "abc".into();
        assert!(w.raw_values().unwrap_err().contains("linha 2"));
        let d = describe(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(d[1], ("média", num(2.5)));
        let (lo, width, counts) = bins(&[0.0, 1.0, 2.0, 10.0]);
        assert_eq!(lo, 0.0);
        assert_eq!(counts.iter().sum::<f64>(), 4.0);
        assert!((width * counts.len() as f64 - 10.0).abs() < 1e-9);
    }

    #[test]
    fn the_comparison_runs_on_a_thread_and_the_choice_is_only_a_suggestion() {
        let mut w = FitWin::new("torno", "process", exp_values(80), false);
        w.start();
        assert!(w.running());
        w.wait();
        assert!(w.error.is_empty(), "{}", w.error);
        let (_, fit) = w.fit.clone().unwrap();
        assert_eq!(w.selected, fit.chosen);
        assert!(w.current());
        // Another row: the user picks it; the draft carries it and the values.
        let other = fit
            .report
            .ranking
            .candidates
            .iter()
            .position(|c| {
                c.fit_error.is_none()
                    && Some(c.family.as_str())
                        != w.selected
                            .map(|i| fit.report.ranking.candidates[i].family.as_str())
            })
            .unwrap();
        w.selected = Some(other);
        let fam = w.selected_family();
        let d = w.draft(&fam);
        assert_eq!(d["choose"], fam);
        assert_eq!(d["origem"], "valores");
        assert_eq!(d["text"].as_str().unwrap().lines().count(), 80);
        // The report has the comparison and the choice.
        let dir = std::env::temp_dir().join(format!("utsim_valores_{}", std::process::id()));
        let file = w.write_report(&dir).unwrap();
        let html = std::fs::read_to_string(&file).unwrap();
        assert!(html.contains("Comparação de todas as medidas"));
        assert!(html.contains(&format!("{fam} (usada)")));
        let _ = std::fs::remove_dir_all(&dir);
        // Changing a value makes the fit stale.
        w.rows[0] = "99".into();
        assert!(!w.current());
    }
}
