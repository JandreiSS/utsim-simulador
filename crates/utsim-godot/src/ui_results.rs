//! The "Resultados" window of the Godot interface, in egui (pure: no Godot types, unit
//! tested). It replaced `resultados.gd` and the charts it drew with `grafico.gd`.
//!
//! Five tabs over the 3D view, with the numbers of the last run ([`RunOutput`], the same
//! as `utsim run`):
//!
//! - **Resumo:** cards with the run settings and the two bottlenecks, then the table per
//!   object (header row, numbers aligned to the right, alternating rows, the bottleneck in
//!   the warning colour) and the CSV export;
//! - **Gráficos:** utilisation (with the 95% whiskers), states (the colours of the 3D
//!   view), the content over time and the pooled histograms, drawn with `egui_plot`;
//! - **Análise:** warm-up (Welch and MSER-5, "Aplicar" writes `run.warmup`), replications
//!   needed ("Rodar mais"), Little's law with a light per object and the shifting
//!   bottleneck;
//! - **Cenários:** scenarios written as text, compared with common random numbers;
//! - **Otimização:** one objective, integer or continuous variables, a method.
//!
//! The window changes nothing itself. A run it needs is built here ([`task`]) and started
//! by the main scene as every other job (`main.start_job`, with progress and "Cancelar");
//! the bridge hands the finished job back ([`done`]). Other actions go out in text:
//! `resultados:fechar`, `resultados:exportar`, `resultados:ligar_amostras`,
//! `resultados:aplicar_aquecimento:<t>` and `resultados:trabalho:<kind>`.
//!
//! The widgets keep the names of the Godot controls they replaced (`PainelResultados`,
//! `Res_Fechar`, `Res_Resumo`, `Res_Tabela`, `Res_TipoGrafico`, `Res_Grafico`,
//! `Res_AquecAnalisar`...), so the scripted runs (`g3_resultados.json`) do not change;
//! the tabs are `Res_Abas_<título>` and every chart records how many things it draws
//! (bars, rows, curves or bins) for the `grafico` checks.

use std::cell::RefCell;
use std::collections::BTreeMap;

use egui::text::CCursorRange;
use egui::{
    Align, Align2, Color32, ComboBox, FontId, Id, Label, Layout, Rect, RichText, ScrollArea, Sense,
    Stroke, StrokeKind, TextEdit, TextStyle, TextWrapMode, Ui, UiBuilder, Vec2,
};
use egui_plot::{Bar, BarChart, GridMark, Line, Plot, PlotPoint, PlotPoints, Text, VLine};
use serde_json::Value;
use utsim_model::analysis::WarmupSeries;
use utsim_ui_tema::{colors, sizes, widgets};

use crate::egui_widgets::{Registry, WidgetInfo};
use crate::jobs::Task;
use crate::results::{self, OptForm, RunOutput};
use crate::ui_props::{named, named_list, named_toggle};

/// The tabs, in order.
pub const TABS: [&str; 5] = ["Resumo", "Gráficos", "Análise", "Cenários", "Otimização"];
/// The charts of "Gráficos": id (for `choose_chart`) and label.
pub const CHARTS: [(&str, &str); 4] = [
    ("ocupacao", "Ocupação"),
    ("estados", "Estados"),
    ("serie", "Série temporal"),
    ("histograma", "Histograma"),
];
/// The utilisation shown: work, within shift, work plus blocked.
pub const UTILIZATION: [&str; 3] = ["trabalho", "turno", "bloqueio"];
/// The optimisation methods.
pub const METHODS: [&str; 3] = ["grade", "recozimento", "genetico"];
/// Gap between the window and the panels around it, in points.
pub const MARGIN: f32 = 12.0;
/// Colour of the bars (the accent of the theme).
const BAR: Color32 = Color32::from_rgb(90, 150, 240);
/// The single replications under the mean.
const FAINT: Color32 = Color32::from_rgba_premultiplied(54, 69, 92, 150);

// ---------------------------------------------------------------------------------------
// Numbers
// ---------------------------------------------------------------------------------------

/// A number with about four significant digits, no trailing zeros (`-` when not finite).
pub fn num(x: f64) -> String {
    if !x.is_finite() {
        return "-".into();
    }
    if x == 0.0 {
        return "0".into();
    }
    let digits = (3 - x.abs().log10().floor() as i32).clamp(0, 6) as usize;
    let s = format!("{x:.digits$}");
    let s = if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        s
    };
    if s == "-0" { "0".into() } else { s }
}

fn f(v: &Value) -> Option<f64> {
    v.as_f64().filter(|x| x.is_finite())
}

/// `mean ± half_width` of a `{mean, half_width}` value (`percent`: times 100, with `%`).
pub fn mh(s: &Value, percent: bool) -> String {
    let Some(m) = f(&s["mean"]) else {
        return "-".into();
    };
    let (k, unit) = if percent { (100.0, "%") } else { (1.0, "") };
    match f(&s["half_width"]) {
        Some(h) => format!("{} ± {}{unit}", num(m * k), num(h * k)),
        None => format!("{}{unit}", num(m * k)),
    }
}

fn text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

// ---------------------------------------------------------------------------------------
// What is shown and what is kept
// ---------------------------------------------------------------------------------------

/// The run the window shows.
pub struct ResultsView {
    pub run: RunOutput,
    /// The run can be run again (a recorded replay cannot: no scenarios, optimisation or
    /// "Ligar e rodar de novo").
    pub can_run: bool,
    summary: Value,
    table: Value,
    /// The objects of the table, in order.
    pub objects: Vec<String>,
    /// The chart of "Gráficos" for its key (kind, object, detail), built once.
    cache: RefCell<Option<(String, Chart)>>,
}

impl ResultsView {
    pub fn new(run: RunOutput, can_run: bool) -> Self {
        let summary = run.summary();
        let table = run.table();
        let objects = run.stats.iter().map(|s| s.name.clone()).collect();
        ResultsView {
            run,
            can_run,
            summary,
            table,
            objects,
            cache: RefCell::new(None),
        }
    }

    pub fn bottleneck(&self) -> Option<String> {
        self.summary["bottleneck"].as_str().map(str::to_owned)
    }

    /// The first line of the summary (`modelo: 5 replicação(ões), ...`) and the
    /// bottlenecks, as the scripts read them.
    pub fn summary_text(&self) -> String {
        let s = &self.summary;
        let b = s["bottleneck"]
            .as_str()
            .unwrap_or("nenhum recurso ocupado")
            .to_owned();
        let bs = s["bottleneck_in_shift"].as_str().unwrap_or("-").to_owned();
        format!(
            "{}: {} replicação(ões), tempo de parada {}, aquecimento {}, semente {}.\nGargalo (geral): {b}\nGargalo (dentro do turno): {bs}",
            text(&s["model"]),
            text(&s["replications"]),
            num(f(&s["stop_time"]).unwrap_or(0.0)),
            num(f(&s["warmup"]).unwrap_or(0.0)),
            text(&s["seed"]),
        )
    }
}

/// What the window keeps between frames: the tab, the forms and the analyses done.
#[derive(Clone, Debug, PartialEq)]
pub struct ResultsMemory {
    pub tab: usize,
    pub export_dir: String,
    /// Index into [`CHARTS`].
    pub chart: usize,
    pub object: String,
    /// Index into [`UTILIZATION`].
    pub utilization: usize,
    pub tally: String,
    pub warm_series: String,
    pub warm_window: String,
    pub warm: Option<Value>,
    pub warm_error: String,
    pub prec_target: String,
    pub prec: Option<Value>,
    pub prec_error: String,
    pub little: Option<Value>,
    pub little_error: String,
    pub shifting: Option<Value>,
    pub shifting_error: String,
    pub sc_text: String,
    pub sc_measures: String,
    pub sc_reps: String,
    pub sc: Option<Value>,
    pub sc_error: String,
    pub op_metric: String,
    pub op_max: bool,
    pub op_vars: String,
    /// Index into [`METHODS`].
    pub op_method: usize,
    pub op_reps: String,
    pub op_budget: String,
    pub op: Option<Value>,
    pub op_error: String,
}

impl Default for ResultsMemory {
    fn default() -> Self {
        ResultsMemory {
            tab: 0,
            export_dir: String::new(),
            chart: 0,
            object: String::new(),
            utilization: 0,
            tally: String::new(),
            warm_series: "wip".into(),
            warm_window: "5".into(),
            warm: None,
            warm_error: String::new(),
            prec_target: "5%".into(),
            prec: None,
            prec_error: String::new(),
            little: None,
            little_error: String::new(),
            shifting: None,
            shifting_error: String::new(),
            sc_text: String::new(),
            sc_measures: String::new(),
            sc_reps: "5".into(),
            sc: None,
            sc_error: String::new(),
            op_metric: String::new(),
            op_max: false,
            op_vars: String::new(),
            op_method: 0,
            op_reps: "5".into(),
            op_budget: "50".into(),
            op: None,
            op_error: String::new(),
        }
    }
}

impl ResultsMemory {
    /// A new run is shown: the forms stay, the replications needed are computed again on
    /// request, and the chart's object is kept when it still exists (else the bottleneck).
    pub fn on_new_results(&mut self, v: &ResultsView, export_dir: &str) {
        self.prec = None;
        self.prec_error.clear();
        if self.export_dir.trim().is_empty() {
            self.export_dir = export_dir.to_owned();
        }
        if !v.objects.contains(&self.object) {
            self.object = v
                .bottleneck()
                .or_else(|| v.objects.first().cloned())
                .unwrap_or_default();
        }
    }

    /// The chart kind by id (`ocupacao`, `estados`, `serie`, `histograma`).
    pub fn choose_chart(&mut self, id: &str) -> bool {
        match CHARTS.iter().position(|c| c.0 == id) {
            Some(k) => {
                self.chart = k;
                self.tab = 1;
                true
            }
            None => false,
        }
    }

    /// Computes the replications needed for the target typed.
    pub fn precision(&mut self, v: &ResultsView) {
        match v.run.precision(self.prec_target.trim(), &[]) {
            Ok(p) => {
                self.prec = Some(p);
                self.prec_error.clear();
            }
            Err(e) => {
                self.prec = None;
                self.prec_error = e;
            }
        }
    }

    /// Replications "Rodar mais" adds (0: none needed or not computed).
    pub fn more(&self) -> u32 {
        self.prec
            .as_ref()
            .and_then(|p| p["more"].as_u64())
            .unwrap_or(0) as u32
    }

    fn error_of(&mut self, kind: &str) -> &mut String {
        match kind {
            "aquecimento" => &mut self.warm_error,
            "mais" | "replicacoes" => &mut self.prec_error,
            "little" => &mut self.little_error,
            "gargalo_movel" => &mut self.shifting_error,
            "cenarios" => &mut self.sc_error,
            _ => &mut self.op_error,
        }
    }

    /// Shows `e` under the section of `kind` (a run that did not start).
    pub fn set_error(&mut self, kind: &str, e: &str) {
        *self.error_of(kind) = e.to_owned();
    }
}

// ---------------------------------------------------------------------------------------
// Runs
// ---------------------------------------------------------------------------------------

fn parse_count(text: &str, what: &str, lo: u64, hi: u64) -> Result<u64, String> {
    text.trim()
        .parse::<u64>()
        .map(|n| n.clamp(lo, hi))
        .map_err(|_| format!("{what} \"{}\" inválido: use um número inteiro", text.trim()))
}

/// The run of the button `kind` (`aquecimento`, `mais`, `little`, `gargalo_movel`,
/// `cenarios`, `otimizacao`) with what the forms hold, or why it cannot start.
pub fn task(kind: &str, v: &ResultsView, mem: &ResultsMemory) -> Result<Task, String> {
    let run = &v.run;
    match kind {
        "aquecimento" => {
            let series = match mem.warm_series.trim() {
                "" | "wip" => WarmupSeries::Wip,
                other => WarmupSeries::parse(other)
                    .ok_or_else(|| format!("série \"{other}\" inválida: use objeto.medida"))?,
            };
            let window = parse_count(&mem.warm_window, "janela", 1, 200)? as usize;
            Ok(Task::Warmup {
                model: run.model.clone(),
                series,
                window,
            })
        }
        "mais" => {
            let extra = mem.more();
            if extra == 0 {
                return Err("calcule antes quantas replicações faltam".into());
            }
            Ok(Task::More {
                base: Box::new(run.clone()),
                extra: extra.min(10_000),
            })
        }
        "little" => Ok(Task::Little(run.model.clone())),
        "gargalo_movel" => Ok(Task::ShiftingBottleneck(run.model.clone())),
        "cenarios" => {
            if !v.can_run {
                return Err("Simule o modelo antes de comparar cenários.".into());
            }
            let scenarios = results::parse_scenarios(&mem.sc_text)?;
            let reps = parse_count(&mem.sc_reps, "replicações", 2, 1000)? as u32;
            let mut model = run.model.clone();
            model.run.replications = reps;
            Ok(Task::Scenarios {
                model,
                scenarios,
                measures: mem
                    .sc_measures
                    .split(',')
                    .map(|m| m.trim().to_owned())
                    .filter(|m| !m.is_empty())
                    .collect(),
            })
        }
        "otimizacao" => {
            if !v.can_run {
                return Err("Simule o modelo antes de otimizar.".into());
            }
            let form = OptForm {
                metric: mem.op_metric.trim().to_owned(),
                maximize: mem.op_max,
                variables: mem.op_vars.clone(),
                method: METHODS[mem.op_method.min(METHODS.len() - 1)].to_owned(),
                replications: parse_count(&mem.op_reps, "replicações", 2, 200)? as u32,
                budget: parse_count(&mem.op_budget, "orçamento", 1, 5000)? as usize,
            };
            Ok(Task::Optimize {
                model: run.model.clone(),
                problem: Box::new(form.build()?),
            })
        }
        other => Err(format!("trabalho desconhecido: {other}")),
    }
}

/// The text under the warm-up chart.
pub fn warm_text(d: &Value) -> String {
    match f(&d["suggested"]) {
        None => format!(
            "Série curta demais para o MSER-5 ({} pontos): aumente o tempo de parada ou a amostragem.",
            text(&d["points"])
        ),
        Some(cut) if d["too_short"].as_bool() == Some(true) => format!(
            "Corte sugerido: {}, mas o mínimo ficou na segunda metade da série: a rodada é curta ou não se estabiliza.",
            num(cut)
        ),
        Some(cut) => format!(
            "Corte sugerido (MSER-5): {}. Antes dele a média ainda sobe ou desce.",
            num(cut)
        ),
    }
}

/// The resources of the shifting bottleneck, most of the time first.
fn shifting_first(d: &Value) -> Option<String> {
    d.as_array()?.first()?["name"].as_str().map(str::to_owned)
}

/// The line under the shifting bottleneck chart.
pub fn shifting_text(d: &Value) -> String {
    match shifting_first(d) {
        Some(n) => format!("Gargalo móvel: {n} na maior parte do tempo."),
        None => "Nenhum recurso ativo.".into(),
    }
}

fn little_rows(d: &Value) -> Vec<Value> {
    let mut rows: Vec<Value> = d["objects"].as_array().cloned().unwrap_or_default();
    if d["system"].is_object() {
        let mut s = d["system"].clone();
        s["name"] = Value::from("sistema");
        rows.push(s);
    }
    rows
}

/// A finished run of the window (`kind` as the job names it: `aquecimento`, `little`,
/// `gargalo_movel`, `cenarios`, `otimizacao`, `replicacoes`): keeps what it found and
/// returns the status line.
pub fn done(kind: &str, data: Option<&Value>, v: &ResultsView, mem: &mut ResultsMemory) -> String {
    if kind == "replicacoes" {
        // "Rodar mais": the new run is already shown; how many are still missing.
        mem.precision(v);
        return format!(
            "{} replicações; faltam {} para o alvo.",
            v.run.reports.len(),
            mem.more()
        );
    }
    let Some(d) = data.filter(|d| !d.is_null()) else {
        return String::new();
    };
    mem.error_of(kind).clear();
    match kind {
        "aquecimento" => {
            mem.warm = Some(d.clone());
            format!("Aquecimento analisado: {}", warm_text(d))
        }
        "little" => {
            mem.little = Some(d.clone());
            let mut n: BTreeMap<&str, usize> = BTreeMap::new();
            let rows = little_rows(d);
            for r in &rows {
                *n.entry(r["light"].as_str().unwrap_or("vermelho"))
                    .or_default() += 1;
            }
            let get = |k: &str| n.get(k).copied().unwrap_or(0);
            format!(
                "Lei de Little: {} verde(s), {} amarelo(s), {} vermelho(s).",
                get("verde"),
                get("amarelo"),
                get("vermelho")
            )
        }
        "gargalo_movel" => {
            mem.shifting = Some(d.clone());
            shifting_text(d)
        }
        "cenarios" => {
            mem.sc = Some(d.clone());
            format!(
                "{} cenário(s) comparados.",
                d["scenarios"].as_array().map_or(0, Vec::len)
            )
        }
        "otimizacao" => {
            mem.op = Some(d.clone());
            format!(
                "Otimização: melhor ponto {}.",
                d["best"].as_str().unwrap_or("nenhum")
            )
        }
        _ => String::new(),
    }
}

// ---------------------------------------------------------------------------------------
// Charts
// ---------------------------------------------------------------------------------------

/// One horizontal bar: a value with its 95% half-width; `hot` is the bottleneck.
#[derive(Clone, Debug, PartialEq)]
pub struct BarRow {
    pub label: String,
    pub value: f64,
    pub half: Option<f64>,
    pub hot: bool,
}

/// One piece of a stacked bar (a state).
#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub label: String,
    pub fraction: f64,
    pub color: Color32,
}

/// One curve.
#[derive(Clone, Debug, PartialEq)]
pub struct Curve {
    pub name: String,
    pub points: Vec<[f64; 2]>,
    pub color: Color32,
    pub width: f32,
}

/// What a chart draws (the numbers come ready from [`RunOutput`] and the analyses).
#[derive(Clone, Debug, PartialEq)]
pub enum Chart {
    /// Horizontal bars in percent.
    Bars(Vec<BarRow>),
    /// One stacked bar per object (`label`, pieces).
    Stacked(Vec<(String, Vec<Segment>)>),
    /// Curves over a common x axis, with vertical markers `(x, label)`.
    Lines {
        curves: Vec<Curve>,
        markers: Vec<(f64, String)>,
        x: String,
        y: String,
    },
    /// The counts of a histogram from `min`, bins of `width`.
    Histogram {
        min: f64,
        width: f64,
        counts: Vec<f64>,
    },
}

impl Chart {
    /// How many things are drawn (bars, rows, curves or bins): the scripted checks.
    pub fn count(&self) -> usize {
        match self {
            Chart::Bars(r) => r.len(),
            Chart::Stacked(r) => r.len(),
            Chart::Lines { curves, .. } => curves.iter().filter(|c| !c.points.is_empty()).count(),
            Chart::Histogram { counts, .. } => counts.len(),
        }
    }
}

fn points(v: &Value) -> Vec<[f64; 2]> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|p| Some([f(&p[0])?, f(&p[1])?]))
                .collect()
        })
        .unwrap_or_default()
}

fn rgb(v: &Value) -> Color32 {
    let c = |k: usize| (f(&v[k]).unwrap_or(0.5).clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgb(c(0), c(1), c(2))
}

/// Utilisation bars of `run.utilization(which)`.
pub fn utilization_chart(run: &RunOutput, which: &str) -> Chart {
    let rows = run.utilization(which);
    Chart::Bars(
        rows.as_array()
            .into_iter()
            .flatten()
            .map(|r| BarRow {
                label: text(&r["name"]),
                value: f(&r["mean"]).unwrap_or(0.0) * 100.0,
                half: f(&r["half_width"]).map(|h| h * 100.0),
                hot: r["bottleneck"].as_bool() == Some(true),
            })
            .collect(),
    )
}

pub fn states_chart(run: &RunOutput) -> Chart {
    Chart::Stacked(
        run.states()
            .as_array()
            .into_iter()
            .flatten()
            .map(|r| {
                let segs = r["segments"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .map(|s| Segment {
                        label: text(&s["label"]),
                        fraction: f(&s["fraction"]).unwrap_or(0.0),
                        color: rgb(&s["color"]),
                    })
                    .collect();
                (text(&r["name"]), segs)
            })
            .collect(),
    )
}

pub fn series_chart(run: &RunOutput, object: &str) -> Chart {
    let s = run.series(object);
    let mut curves: Vec<Curve> = s["replications"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|c| Curve {
            name: "replicações".into(),
            points: points(c),
            color: FAINT,
            width: 1.0,
        })
        .collect();
    let mean = points(&s["mean"]);
    if !mean.is_empty() {
        curves.push(Curve {
            name: "média".into(),
            points: mean,
            color: colors::WARNING,
            width: 2.5,
        });
    }
    Chart::Lines {
        curves,
        markers: Vec::new(),
        x: "tempo".into(),
        y: format!("conteúdo de {object}"),
    }
}

pub fn histogram_chart(run: &RunOutput, object: &str, tally: &str) -> Chart {
    let h = run.histogram(object, tally);
    Chart::Histogram {
        min: f(&h["min"]).unwrap_or(0.0),
        width: f(&h["bin_width"]).unwrap_or(1.0),
        counts: h["counts"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_f64)
            .collect(),
    }
}

pub fn warm_chart(d: &Value) -> Chart {
    let mut markers = Vec::new();
    if let Some(cut) = f(&d["suggested"]) {
        markers.push((cut, format!("MSER-5: {}", num(cut))));
    }
    Chart::Lines {
        curves: vec![
            Curve {
                name: "média entre replicações".into(),
                points: points(&d["mean"]),
                color: Color32::from_rgb(102, 128, 166),
                width: 1.0,
            },
            Curve {
                name: "Welch".into(),
                points: points(&d["welch"]),
                color: colors::WARNING,
                width: 2.5,
            },
        ],
        markers,
        x: "tempo".into(),
        y: text(&d["series"]),
    }
}

pub fn shifting_chart(d: &Value) -> Chart {
    let first = shifting_first(d);
    Chart::Bars(
        d.as_array()
            .into_iter()
            .flatten()
            .map(|r| {
                let pct = |v: &Value| f(&v["mean"]).map_or("-".into(), |x| num(x * 100.0));
                BarRow {
                    label: format!(
                        "{} (único {}%, compartilhado {}%)",
                        text(&r["name"]),
                        pct(&r["sole"]),
                        pct(&r["shifting"])
                    ),
                    value: f(&r["total"]).unwrap_or(0.0) * 100.0,
                    half: None,
                    hot: first.as_deref() == r["name"].as_str(),
                }
            })
            .collect(),
    )
}

pub fn history_chart(d: &Value) -> Chart {
    Chart::Lines {
        curves: vec![Curve {
            name: "melhor objetivo".into(),
            points: points(&d["history"]),
            color: colors::WARNING,
            width: 2.0,
        }],
        markers: Vec::new(),
        x: "pontos avaliados".into(),
        y: "melhor objetivo".into(),
    }
}

/// The chart of "Gráficos" for what the memory chose.
pub fn chosen_chart(v: &ResultsView, mem: &ResultsMemory) -> Chart {
    let key = format!(
        "{}|{}|{}|{}",
        mem.chart, mem.object, mem.utilization, mem.tally
    );
    if let Some((k, c)) = v.cache.borrow().as_ref()
        && *k == key
    {
        return c.clone();
    }
    let c = match CHARTS[mem.chart.min(CHARTS.len() - 1)].0 {
        "ocupacao" => utilization_chart(&v.run, UTILIZATION[mem.utilization.min(2)]),
        "estados" => states_chart(&v.run),
        "serie" => series_chart(&v.run, &mem.object),
        _ => histogram_chart(&v.run, &mem.object, &mem.tally),
    };
    *v.cache.borrow_mut() = Some((key, c.clone()));
    c
}

// ---------------------------------------------------------------------------------------
// The window
// ---------------------------------------------------------------------------------------

/// Draws the window in `rect` (points) and pushes the actions to `out`.
pub fn show(
    ui: &mut Ui,
    rect: Rect,
    v: &ResultsView,
    mem: &mut ResultsMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let painter = ui.painter().clone();
    painter.rect_filled(rect, sizes::RADIUS, colors::PANEL);
    painter.rect_stroke(
        rect,
        sizes::RADIUS,
        Stroke::new(1.0, colors::BORDER),
        StrokeKind::Inside,
    );
    let panel = ui.allocate_rect(rect, Sense::hover());
    named(ui, reg, "PainelResultados", &panel, "Resultados");
    let inner = rect.shrink2(Vec2::new(16.0, 12.0));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(inner.expand(2.0));
    let ui = &mut child;
    ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
    header(ui, v, reg, out);
    tabs(ui, mem, reg);
    if mem.tab != 0 {
        // The summary of a tab not shown is still read (as a hidden Godot tab was).
        reg.put_info(
            "Res_Resumo",
            WidgetInfo {
                rect: Rect::NOTHING,
                text: v.summary_text(),
                enabled: true,
                checked: None,
                choices: Vec::new(),
                count: None,
            },
        );
    }
    let mut scroll = egui::style::ScrollStyle::floating();
    scroll.dormant_handle_opacity = 0.5;
    ui.spacing_mut().scroll = scroll;
    ScrollArea::vertical()
        .id_salt(("utsim_resultados", mem.tab))
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width() - 12.0);
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);
            match mem.tab {
                0 => summary_tab(ui, v, mem, reg, out),
                1 => charts_tab(ui, v, mem, reg, out),
                2 => analysis_tab(ui, v, mem, reg, out),
                3 => scenarios_tab(ui, v, mem, reg, out),
                _ => optimize_tab(ui, v, mem, reg, out),
            }
            ui.add_space(8.0);
        });
}

fn header(ui: &mut Ui, v: &ResultsView, reg: &mut Registry, out: &mut Vec<String>) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Resultados")
                .size(sizes::HEADING)
                .strong()
                .color(colors::HEADING),
        );
        ui.label(
            RichText::new(format!(
                "{} · {} replicação(ões)",
                text(&v.summary["model"]),
                v.run.reports.len()
            ))
            .color(colors::TEXT_WEAK),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let r = ui
                .button("Fechar")
                .on_hover_text("Fechar os resultados (a cena continua)");
            named(ui, reg, "Res_Fechar", &r, "Fechar");
            if r.clicked() {
                out.push("resultados:fechar".into());
            }
        });
    });
}

fn tabs(ui: &mut Ui, mem: &mut ResultsMemory, reg: &mut Registry) {
    tab_row(ui, reg, "Res_Abas", &TABS, &mut mem.tab);
}

/// A row of tabs named `<abas>_<título>` (the row itself is `abas`); `at` is the one shown.
pub(crate) fn tab_row(
    ui: &mut Ui,
    reg: &mut Registry,
    abas: &str,
    titles: &[&str],
    at: &mut usize,
) {
    let row = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (k, title) in titles.iter().enumerate() {
            let on = *at == k;
            let name = format!("{abas}_{title}");
            let text = RichText::new(*title).color(if on {
                colors::TEXT_STRONG
            } else {
                colors::TEXT_WEAK
            });
            let r = ui.add(
                egui::Button::new(text)
                    .fill(if on {
                        colors::BUTTON
                    } else {
                        Color32::TRANSPARENT
                    })
                    .stroke(Stroke::NONE)
                    .min_size(Vec2::new(0.0, 30.0)),
            );
            if on {
                let b = r.rect;
                ui.painter().line_segment(
                    [b.left_bottom(), b.right_bottom()],
                    Stroke::new(2.0, colors::ACCENT),
                );
            }
            named_toggle(ui, reg, &name, &r, title, on);
            if r.clicked() {
                *at = k;
            }
        }
    });
    named(ui, reg, abas, &row.response, titles[*at]);
    let y = row.response.rect.max.y;
    ui.painter().line_segment(
        [
            egui::pos2(ui.max_rect().min.x, y),
            egui::pos2(ui.max_rect().max.x, y),
        ],
        Stroke::new(1.0, colors::BORDER),
    );
}

pub(crate) fn heading(ui: &mut Ui, text: &str) {
    ui.add_space(4.0);
    ui.label(
        RichText::new(text)
            .size(sizes::HEADING - 1.0)
            .strong()
            .color(colors::HEADING),
    );
}

pub(crate) fn explain(ui: &mut Ui, text: &str) -> egui::Response {
    ui.add(Label::new(RichText::new(text).color(colors::TEXT_WEAK)).wrap())
}

pub(crate) fn error(ui: &mut Ui, text: &str) {
    if !text.is_empty() {
        ui.add(Label::new(RichText::new(text).color(colors::ERROR)).wrap());
    }
}

pub(crate) fn button(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    label: &str,
    tip: &str,
    enabled: bool,
) -> bool {
    let r = ui
        .add_enabled(enabled, egui::Button::new(label))
        .on_hover_text(tip);
    named(ui, reg, name, &r, label);
    r.clicked()
}

pub(crate) fn primary(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    label: &str,
    tip: &str,
    enabled: bool,
) -> bool {
    let r = ui
        .add_enabled(enabled, widgets::primary(label))
        .on_hover_text(tip);
    named(ui, reg, name, &r, label);
    r.clicked()
}

/// A one-line field bound to `buf`: selects all when it gets the focus.
pub(crate) fn field(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    buf: &mut String,
    width: f32,
    hint: &str,
) {
    let _ = field_response(ui, reg, name, buf, width, hint);
}

/// [`field`], with its response (Enter: `lost_focus` with the key pressed).
pub(crate) fn field_response(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    buf: &mut String,
    width: f32,
    hint: &str,
) -> egui::Response {
    let id = Id::new(("utsim_res", name));
    let mut output = TextEdit::singleline(buf)
        .id(id)
        .desired_width(width)
        .hint_text(RichText::new(hint).color(colors::TEXT_DISABLED))
        .show(ui);
    let r = output.response.response.clone();
    if r.gained_focus() {
        output
            .state
            .cursor
            .set_char_range(Some(CCursorRange::select_all(&output.galley)));
        output.state.store(ui.ctx(), id);
    }
    named(ui, reg, name, &r, buf);
    r
}

/// A text area bound to `buf` (scenarios, variables), in the monospace font.
pub(crate) fn area(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    buf: &mut String,
    rows: usize,
    hint: &str,
) {
    let r = ui.add(
        TextEdit::multiline(buf)
            .id(Id::new(("utsim_res", name)))
            .font(TextStyle::Monospace)
            .desired_rows(rows)
            .desired_width(f32::INFINITY)
            .hint_text(RichText::new(hint).color(colors::TEXT_DISABLED)),
    );
    named(ui, reg, name, &r, buf);
}

/// A list: returns the index picked (by the user or by a scripted `escolher`).
fn combo(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    selected: usize,
    items: &[String],
    width: f32,
) -> Option<usize> {
    let mut pick = reg
        .take_request(name)
        .and_then(|item| items.iter().position(|c| *c == item));
    let shown = items.get(selected).cloned().unwrap_or_default();
    let r = ComboBox::from_id_salt(("utsim_res", name))
        .selected_text(&shown)
        .width(width)
        .wrap_mode(TextWrapMode::Truncate)
        .show_ui(ui, |ui| {
            for (k, item) in items.iter().enumerate() {
                if ui.selectable_label(k == selected, item).clicked() {
                    pick = Some(k);
                }
            }
        });
    named_list(ui, reg, name, &r.response, &shown, items.to_vec());
    pick.filter(|&k| k != selected)
}

// ----- Tables ---------------------------------------------------------------------------

/// One cell: its text and colour, an optional tag after it (`gargalo`, `Δ ...`) and an
/// optional dot before it (the light of Little's law).
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub text: String,
    pub color: Color32,
    pub tag: Option<(String, Color32)>,
    pub dot: Option<Color32>,
}

impl Cell {
    pub(crate) fn plain(text: impl Into<String>) -> Cell {
        Cell {
            text: text.into(),
            color: colors::TEXT,
            tag: None,
            dot: None,
        }
    }

    pub(crate) fn colored(text: impl Into<String>, color: Color32) -> Cell {
        Cell {
            color,
            ..Cell::plain(text)
        }
    }

    pub(crate) fn tagged(mut self, tag: impl Into<String>, color: Color32) -> Cell {
        self.tag = Some((tag.into(), color));
        self
    }

    /// The text the scripts read: `texto (tag)`, `● texto`.
    pub fn shown(&self) -> String {
        let mut s = String::new();
        if self.dot.is_some() {
            s.push_str("● ");
        }
        s.push_str(&self.text);
        if let Some((t, _)) = &self.tag {
            s.push_str(&format!(" ({t})"));
        }
        s
    }
}

/// One row; `hot` rows (the bottleneck) get the warning tint and a mark on the left.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub hot: bool,
}

/// A table: `right[k]` aligns column `k` to the right (numbers).
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub header: Vec<String>,
    pub right: Vec<bool>,
    pub rows: Vec<Row>,
}

impl Table {
    /// Everything the table shows, one row per line (the scripted `texto` checks).
    pub fn text(&self) -> String {
        let mut lines = vec![self.header.join(" | ")];
        for r in &self.rows {
            lines.push(
                r.cells
                    .iter()
                    .map(Cell::shown)
                    .collect::<Vec<_>>()
                    .join(" | "),
            );
        }
        lines.join("\n")
    }
}

const ROW_H: f32 = 26.0;
const PAD: f32 = 10.0;
const TAG_GAP: f32 = 8.0;
const DOT: f32 = 14.0;

pub(crate) fn draw_table(ui: &mut Ui, reg: &mut Registry, name: &str, t: &Table) {
    let body = FontId::proportional(sizes::BODY);
    let small = FontId::proportional(sizes::SMALL);
    let painter = ui.painter().clone();
    let width_of = |s: &str, font: &FontId| {
        painter
            .layout_no_wrap(s.to_owned(), font.clone(), colors::TEXT)
            .size()
            .x
    };
    let cell_width = |c: &Cell| {
        width_of(&c.text, &body)
            + c.tag
                .as_ref()
                .map_or(0.0, |(s, _)| TAG_GAP + width_of(s, &small) + 10.0)
            + if c.dot.is_some() { DOT } else { 0.0 }
    };
    let n = t.header.len();
    let mut widths: Vec<f32> = t.header.iter().map(|h| width_of(h, &body) + 2.0).collect();
    for r in &t.rows {
        for (k, c) in r.cells.iter().enumerate().take(n) {
            widths[k] = widths[k].max(cell_width(c));
        }
    }
    let total: f32 = widths.iter().map(|w| w + 2.0 * PAD).sum::<f32>() + 4.0;
    let full = total.max(ui.available_width());
    // The spare width goes to the first column (the names), so numbers stay together.
    if let Some(w) = widths.first_mut() {
        *w += full - total;
    }
    let height = ROW_H * (t.rows.len() + 1) as f32;
    let resp = ScrollArea::horizontal()
        .id_salt(("utsim_res_tabela", name))
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(full, height), Sense::hover());
            let p = ui.painter();
            p.rect_filled(rect, sizes::RADIUS, colors::FIELD);
            let head = Rect::from_min_size(rect.min, Vec2::new(full, ROW_H));
            p.rect_filled(head, sizes::RADIUS, colors::BAR);
            let row_rect = |k: usize| {
                Rect::from_min_size(
                    rect.min + Vec2::new(0.0, ROW_H * (k + 1) as f32),
                    Vec2::new(full, ROW_H),
                )
            };
            for (k, r) in t.rows.iter().enumerate() {
                let rr = row_rect(k);
                if r.hot {
                    p.rect_filled(rr, 0.0, colors::WARNING.gamma_multiply(0.14));
                    p.rect_filled(
                        Rect::from_min_size(rr.min, Vec2::new(3.0, ROW_H)),
                        0.0,
                        colors::WARNING,
                    );
                } else if k % 2 == 1 {
                    p.rect_filled(rr, 0.0, colors::PANEL);
                }
            }
            p.line_segment(
                [head.left_bottom(), head.right_bottom()],
                Stroke::new(1.0, colors::BORDER),
            );
            let mut x = rect.min.x + 4.0;
            for (k, w) in widths.iter().enumerate() {
                let (left, right) = (x + PAD, x + PAD + w);
                let align_right = t.right.get(k).copied().unwrap_or(false);
                let place = |y: f32, content_w: f32| {
                    egui::pos2(if align_right { right - content_w } else { left }, y)
                };
                let hy = head.center().y;
                let hw = width_of(&t.header[k], &body);
                p.text(
                    place(hy, hw),
                    Align2::LEFT_CENTER,
                    &t.header[k],
                    body.clone(),
                    colors::HEADING,
                );
                for (j, r) in t.rows.iter().enumerate() {
                    let Some(c) = r.cells.get(k) else { continue };
                    let cy = row_rect(j).center().y;
                    let mut at = place(cy, cell_width(c));
                    if let Some(dc) = c.dot {
                        p.circle_filled(egui::pos2(at.x + 5.0, cy), 5.0, dc);
                        at.x += DOT;
                    }
                    let tr = p.text(at, Align2::LEFT_CENTER, &c.text, body.clone(), c.color);
                    if let Some((tag, tc)) = &c.tag {
                        let tw = width_of(tag, &small);
                        let badge = Rect::from_center_size(
                            egui::pos2(tr.max.x + TAG_GAP + tw / 2.0 + 5.0, cy),
                            Vec2::new(tw + 10.0, 18.0),
                        );
                        p.rect_filled(badge, 9.0, tc.gamma_multiply(0.2));
                        p.text(
                            badge.center(),
                            Align2::CENTER_CENTER,
                            tag,
                            small.clone(),
                            *tc,
                        );
                    }
                }
                x += w + 2.0 * PAD;
            }
            resp
        })
        .inner;
    named(ui, reg, name, &resp, &t.text());
}

/// The table per object of the summary.
pub fn object_table(v: &ResultsView) -> Table {
    let rows = v
        .table
        .as_array()
        .into_iter()
        .flatten()
        .map(|r| {
            let (b, bs) = (
                r["bottleneck"].as_bool() == Some(true),
                r["bottleneck_in_shift"].as_bool() == Some(true),
            );
            let name = text(&r["name"]);
            let first = if b {
                Cell::colored(name, colors::WARNING).tagged("gargalo", colors::WARNING)
            } else if bs {
                Cell::colored(name, colors::WARNING).tagged("gargalo no turno", colors::WARNING)
            } else {
                Cell::plain(name)
            };
            let mut cells = vec![
                first,
                Cell::colored(text(&r["kind_label"]), colors::TEXT_WEAK),
            ];
            for k in ["input", "output", "content", "staytime"] {
                cells.push(Cell::plain(mh(&r[k], false)));
            }
            cells.push(if r["resource"].as_bool() == Some(true) {
                Cell::colored(
                    mh(&r["utilization"], true),
                    if b { colors::WARNING } else { colors::TEXT },
                )
            } else {
                Cell::colored("-", colors::TEXT_DISABLED)
            });
            Row {
                cells,
                hot: b || bs,
            }
        })
        .collect();
    Table {
        header: [
            "Objeto",
            "Tipo",
            "Entradas",
            "Saídas",
            "Conteúdo médio",
            "Permanência",
            "Ocupação",
        ]
        .map(String::from)
        .to_vec(),
        right: vec![false, false, true, true, true, true, true],
        rows,
    }
}

// ----- Resumo ---------------------------------------------------------------------------

/// The cards at the top of the summary: `(title, value, line under it, warning)`.
pub fn cards(v: &ResultsView) -> Vec<(String, String, String, bool)> {
    let s = &v.summary;
    let stat = |name: &str| v.run.stats.iter().find(|x| x.name == name);
    let util = |name: &str, shift: bool| {
        stat(name)
            .and_then(|x| {
                if shift {
                    x.utilization_in_shift.or(x.utilization)
                } else {
                    x.utilization
                }
            })
            .map_or(String::new(), |u| {
                format!("ocupação {}", mh(&results::summary(&u), true))
            })
    };
    let exits: f64 = v
        .run
        .stats
        .iter()
        .filter(|x| x.kind == "sink")
        .map(|x| x.input.mean)
        .filter(|m| m.is_finite())
        .sum();
    let b = s["bottleneck"].as_str();
    let bs = s["bottleneck_in_shift"].as_str();
    vec![
        (
            "Replicações".into(),
            v.run.reports.len().to_string(),
            format!("semente {}", text(&s["seed"])),
            false,
        ),
        (
            "Tempo de parada".into(),
            num(f(&s["stop_time"]).unwrap_or(0.0)),
            format!("aquecimento {}", num(f(&s["warmup"]).unwrap_or(0.0))),
            false,
        ),
        (
            "Gargalo".into(),
            b.unwrap_or("nenhum").to_owned(),
            b.map_or("nenhum recurso ocupado".into(), |n| util(n, false)),
            b.is_some(),
        ),
        (
            "Gargalo no turno".into(),
            bs.unwrap_or("-").to_owned(),
            bs.map_or(String::new(), |n| util(n, true)),
            bs.is_some(),
        ),
        (
            "Saídas por replicação".into(),
            num(exits),
            "soma das saídas do modelo".into(),
            false,
        ),
    ]
}

fn summary_tab(
    ui: &mut Ui,
    v: &ResultsView,
    mem: &mut ResultsMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let block = ui.scope(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
            for (k, (title, value, sub, warn)) in cards(v).into_iter().enumerate() {
                let frame = egui::Frame::new()
                    .fill(colors::FIELD)
                    .stroke(Stroke::new(
                        1.0,
                        if warn {
                            colors::WARNING.gamma_multiply(0.6)
                        } else {
                            colors::BORDER
                        },
                    ))
                    .corner_radius(sizes::RADIUS)
                    .inner_margin(egui::Margin::symmetric(12, 8));
                let r = frame.show(ui, |ui| {
                    ui.set_width(150.0);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(
                            RichText::new(&title)
                                .size(sizes::SMALL)
                                .color(colors::TEXT_WEAK),
                        );
                        ui.add(
                            Label::new(
                                RichText::new(&value)
                                    .size(20.0)
                                    .strong()
                                    .color(if warn { colors::WARNING } else { colors::TEXT_STRONG }),
                            )
                            .truncate(),
                        );
                        ui.add(
                            Label::new(
                                RichText::new(if sub.is_empty() { " " } else { &sub })
                                    .size(sizes::SMALL)
                                    .color(colors::TEXT_WEAK),
                            )
                            .truncate(),
                        );
                    });
                });
                named(ui, reg, &format!("Res_Cartao_{k}"), &r.response, &value);
            }
        });
        explain(
            ui,
            "O gargalo é o recurso mais ocupado; dentro do turno conta só o tempo em que ele trabalha (um posto parado meio dia parece ocioso no geral, mas pode estar saturado no turno).",
        );
    });
    named(ui, reg, "Res_Resumo", &block.response, &v.summary_text());
    heading(ui, "Por objeto");
    explain(ui, "Média ± semiamplitude do IC de 95 % entre replicações.");
    draw_table(ui, reg, "Res_Tabela", &object_table(v));
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("Exportar CSV para").color(colors::TEXT));
        let w = (ui.available_width() - 110.0).max(160.0);
        field(ui, reg, "Res_PastaCSV", &mut mem.export_dir, w, "pasta");
        if button(
            ui,
            reg,
            "Res_ExportarCSV",
            "Exportar",
            "Grava resumo, replicações, séries e histogramas em CSV",
            true,
        ) {
            out.push("resultados:exportar".into());
        }
    });
}

// ----- Gráficos -------------------------------------------------------------------------

fn charts_tab(
    ui: &mut Ui,
    v: &ResultsView,
    mem: &mut ResultsMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let kind = CHARTS[mem.chart.min(CHARTS.len() - 1)].0;
    let tallies = if kind == "histograma" {
        v.run.tallies(&mem.object)
    } else {
        Vec::new()
    };
    if kind == "histograma" && !tallies.contains(&mem.tally) {
        mem.tally = tallies.first().cloned().unwrap_or_default();
    }
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Gráfico").color(colors::TEXT));
        let labels: Vec<String> = CHARTS.iter().map(|c| c.1.to_owned()).collect();
        if let Some(k) = combo(ui, reg, "Res_TipoGrafico", mem.chart, &labels, 170.0) {
            mem.chart = k;
        }
        if matches!(kind, "serie" | "histograma") {
            ui.label(RichText::new("Objeto").color(colors::TEXT));
            let at = v.objects.iter().position(|o| *o == mem.object).unwrap_or(0);
            if let Some(k) = combo(ui, reg, "Res_ObjetoGrafico", at, &v.objects, 180.0) {
                mem.object = v.objects[k].clone();
            }
        }
        if kind == "ocupacao" {
            let items: Vec<String> = UTILIZATION.iter().map(|s| s.to_string()).collect();
            if let Some(k) = combo(
                ui,
                reg,
                "Res_DetalheGrafico",
                mem.utilization,
                &items,
                120.0,
            ) {
                mem.utilization = k;
            }
        } else if kind == "histograma" && !tallies.is_empty() {
            let at = tallies.iter().position(|t| *t == mem.tally).unwrap_or(0);
            if let Some(k) = combo(ui, reg, "Res_DetalheGrafico", at, &tallies, 150.0) {
                mem.tally = tallies[k].clone();
            }
        }
    });
    let chart = chosen_chart(v, mem);
    let mut note = match kind {
        "ocupacao" => "Fração do tempo trabalhando (turno: só o tempo dentro do turno; bloqueio: somando o tempo bloqueado). A barra amarela é o gargalo; o traço é o IC de 95 %.".to_owned(),
        "estados" => "Fração do tempo em cada estado, com as cores da cena 3D.".to_owned(),
        "serie" => format!("Conteúdo de {} no tempo: cada replicação em cinza e a média em amarelo.", mem.object),
        _ => format!("Histograma de {}.{}, juntando todas as replicações.", mem.object, mem.tally),
    };
    let offer = matches!(kind, "serie" | "histograma") && chart.count() == 0 && v.can_run;
    if offer {
        note.push_str(&format!(
            " O modelo não grava {}: \"Ligar e rodar de novo\" liga e simula outra vez.",
            if kind == "serie" {
                "a série do conteúdo"
            } else {
                "os histogramas"
            }
        ));
    }
    let r = explain(ui, &note);
    named(ui, reg, "Res_NotaGrafico", &r, &note);
    if offer
        && primary(
            ui,
            reg,
            "Res_LigarAmostras",
            "Ligar e rodar de novo",
            "Liga no modelo a série do conteúdo (run.sample_interval, pelo tempo de simulação) e os histogramas (run.histogram_bins = 20), num passo que se desfaz com Ctrl+Z, e roda as replicações de novo",
            true,
        )
    {
        out.push("resultados:ligar_amostras".into());
    }
    let empty = match kind {
        "serie" => "sem série: ligue run.sample_interval no modelo",
        "histograma" => "sem histograma: ligue run.histogram_bins no modelo",
        _ => "sem dados",
    };
    draw_chart(ui, reg, "Res_Grafico", &chart, empty, 340.0);
}

/// The scrolls and drags of the window stay the window's: the charts do not zoom or pan.
fn still(p: Plot<'_>) -> Plot<'_> {
    p.allow_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .allow_double_click_reset(false)
        .show_x(false)
        .show_y(false)
}

pub(crate) fn draw_chart(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    chart: &Chart,
    empty: &str,
    height: f32,
) {
    let count = chart.count();
    let resp = if count == 0 {
        let (rect, resp) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 120.0), Sense::hover());
        ui.painter().rect_filled(rect, sizes::RADIUS, colors::FIELD);
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            empty,
            FontId::proportional(sizes::BODY),
            colors::TEXT_WEAK,
        );
        resp
    } else {
        match chart {
            Chart::Bars(rows) => bars(ui, name, rows),
            Chart::Stacked(rows) => stacked(ui, name, rows),
            Chart::Lines {
                curves,
                markers,
                x,
                y,
            } => lines(ui, name, curves, markers, x, y, height),
            Chart::Histogram { min, width, counts } => {
                histogram(ui, name, *min, *width, counts, height)
            }
        }
    };
    named(ui, reg, name, &resp, &count.to_string());
    if let Some(w) = reg.read(name).cloned() {
        reg.put_info(
            name,
            WidgetInfo {
                count: Some(count),
                ..w
            },
        );
    }
}

/// The y marks of a chart of rows: one per row, labelled by `labels`.
fn row_axis<'a>(p: Plot<'a>, labels: Vec<String>, width: f32) -> Plot<'a> {
    let n = labels.len();
    p.y_grid_spacer(move |_| {
        (0..n)
            .map(|i| GridMark {
                value: i as f64,
                step_size: 1.0,
            })
            .collect()
    })
    .y_axis_formatter(move |m, _| {
        let i = m.value.round();
        if (m.value - i).abs() > 1e-6 || i < 0.0 {
            return String::new();
        }
        // Rows go top-down: the first one has the largest argument.
        labels
            .get(n.saturating_sub(1 + i as usize))
            .cloned()
            .unwrap_or_default()
    })
    .y_axis_min_width(width)
    .show_grid([true, false])
}

fn label_width(ui: &Ui, labels: &[String]) -> f32 {
    let font = FontId::proportional(sizes::SMALL);
    labels
        .iter()
        .map(|l| {
            ui.painter()
                .layout_no_wrap(l.clone(), font.clone(), colors::TEXT)
                .size()
                .x
        })
        .fold(60.0, f32::max)
        .min(ui.available_width() * 0.45)
        + 8.0
}

fn bars(ui: &mut Ui, name: &str, rows: &[BarRow]) -> egui::Response {
    let n = rows.len();
    let labels: Vec<String> = rows.iter().map(|r| r.label.clone()).collect();
    let width = label_width(ui, &labels);
    // Room for the value written after the longest bar (100% when one is saturated).
    let top = rows
        .iter()
        .map(|r| r.value + r.half.unwrap_or(0.0))
        .fold(1.0, f64::max);
    let plot = row_axis(still(Plot::new(("utsim_res_plot", name))), labels, width)
        .height((30.0 * n as f32 + 50.0).max(140.0))
        .include_x(0.0)
        .include_x(top * 1.1)
        .include_y(-0.6)
        .include_y(n as f64 - 0.4)
        .x_axis_formatter(|m, _| format!("{}%", num(m.value)));
    plot.show(ui, |p| {
        let arg = |k: usize| (n - 1 - k) as f64;
        let list: Vec<Bar> = rows
            .iter()
            .enumerate()
            .map(|(k, r)| {
                Bar::new(arg(k), r.value)
                    .width(0.62)
                    .fill(if r.hot { colors::WARNING } else { BAR })
                    .stroke(Stroke::NONE)
                    .name(&r.label)
            })
            .collect();
        p.bar_chart(BarChart::new("ocupação", list).horizontal());
        for (k, r) in rows.iter().enumerate() {
            let y = arg(k);
            let mut end = r.value;
            if let Some(h) = r.half.filter(|h| *h > 0.0) {
                let s = Stroke::new(1.5, colors::TEXT);
                p.line(
                    Line::new(
                        "IC 95 %",
                        PlotPoints::new(vec![[r.value - h, y], [r.value + h, y]]),
                    )
                    .stroke(s),
                );
                for x in [r.value - h, r.value + h] {
                    p.line(
                        Line::new(
                            "IC 95 %",
                            PlotPoints::new(vec![[x, y - 0.14], [x, y + 0.14]]),
                        )
                        .stroke(s),
                    );
                }
                end += h;
            }
            p.text(
                Text::new(
                    "valor",
                    PlotPoint::new(end + top * 0.015, y),
                    RichText::new(format!("{}%", num(r.value)))
                        .size(sizes::SMALL)
                        .color(colors::TEXT),
                )
                .anchor(Align2::LEFT_CENTER),
            );
        }
    })
    .response
}

fn stacked(ui: &mut Ui, name: &str, rows: &[(String, Vec<Segment>)]) -> egui::Response {
    let n = rows.len();
    let labels: Vec<String> = rows.iter().map(|r| r.0.clone()).collect();
    let width = label_width(ui, &labels);
    // One series per state, so the legend lists each state once with its colour.
    let mut by_state: Vec<(String, Color32, Vec<Bar>)> = Vec::new();
    for (k, (_, segs)) in rows.iter().enumerate() {
        let mut at = 0.0;
        for s in segs {
            let pct = s.fraction * 100.0;
            let bar = Bar::new((n - 1 - k) as f64, pct)
                .base_offset(at)
                .width(0.62)
                .fill(s.color)
                .stroke(Stroke::new(0.5, colors::PANEL))
                .name(format!("{}: {}%", s.label, num(pct)));
            at += pct;
            match by_state.iter_mut().find(|e| e.0 == s.label) {
                Some(e) => e.2.push(bar),
                None => by_state.push((s.label.clone(), s.color, vec![bar])),
            }
        }
    }
    let entries: Vec<(String, Color32)> = by_state.iter().map(|e| (e.0.clone(), e.1)).collect();
    legend_row(ui, "", &entries);
    let plot = row_axis(still(Plot::new(("utsim_res_plot", name))), labels, width)
        .height((30.0 * n as f32 + 50.0).max(140.0))
        .include_x(0.0)
        .include_x(100.0)
        .include_y(-0.6)
        .include_y(n as f64 - 0.4)
        .x_axis_formatter(|m, _| format!("{}%", num(m.value)));
    plot.show(ui, |p| {
        for (label, color, list) in by_state {
            p.bar_chart(BarChart::new(label, list).color(color).horizontal());
        }
    })
    .response
}

fn lines(
    ui: &mut Ui,
    name: &str,
    curves: &[Curve],
    markers: &[(f64, String)],
    x: &str,
    y: &str,
    height: f32,
) -> egui::Response {
    // The legend and the name of the y axis go above the chart (inside, they covered the
    // curves; a long name on the axis ran over the text above).
    let mut entries: Vec<(String, Color32)> = Vec::new();
    for c in curves.iter().filter(|c| !c.points.is_empty()) {
        if !entries.iter().any(|e| e.0 == c.name) {
            entries.push((c.name.clone(), c.color));
        }
    }
    for (_, label) in markers {
        entries.push((label.clone(), colors::ACCENT));
    }
    legend_row(ui, y, &entries);
    let (lo, hi) = y_range(curves);
    let mut plot = still(Plot::new(("utsim_res_plot", name)))
        .height(height)
        .x_axis_label(RichText::new(x).size(sizes::SMALL).color(colors::TEXT_WEAK))
        .include_y(lo)
        .include_y(hi);
    for (x, _) in markers {
        plot = plot.include_x(*x);
    }
    plot.show(ui, |p| {
        for c in curves {
            p.line(
                Line::new(c.name.clone(), PlotPoints::new(c.points.clone()))
                    .color(c.color)
                    .width(c.width),
            );
        }
        for (x, label) in markers {
            p.vline(
                VLine::new(label.clone(), *x)
                    .color(colors::ACCENT)
                    .width(1.5),
            );
        }
    })
    .response
}

/// The y range of `curves`, padded; a flat curve gets a range around its value (egui's
/// own bounds of a flat line ran to thousands).
pub fn y_range(curves: &[Curve]) -> (f64, f64) {
    let ys = curves.iter().flat_map(|c| c.points.iter().map(|p| p[1]));
    let (lo, hi) = ys.fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), y| {
        (a.min(y), b.max(y))
    });
    if !lo.is_finite() {
        return (0.0, 1.0);
    }
    let span = hi - lo;
    let pad = if span > 1e-9 {
        span * 0.06
    } else {
        lo.abs().max(1.0) * 0.5
    };
    // Counts and contents start at zero.
    let lo = if lo >= 0.0 { 0.0 } else { lo - pad };
    (lo, hi + pad)
}

/// A small line above a chart: the name of its y axis and a dot per series.
fn legend_row(ui: &mut Ui, y: &str, entries: &[(String, Color32)]) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let small = |t: &str| RichText::new(t).size(sizes::SMALL);
        if !y.is_empty() {
            ui.label(small(y).color(colors::TEXT_WEAK));
            ui.add_space(10.0);
        }
        for (name, color) in entries {
            let (r, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
            ui.painter().circle_filled(r.center(), 4.5, *color);
            ui.label(small(name).color(colors::TEXT));
            ui.add_space(8.0);
        }
    });
}

fn histogram(
    ui: &mut Ui,
    name: &str,
    min: f64,
    width: f64,
    counts: &[f64],
    height: f32,
) -> egui::Response {
    histogram_with(ui, name, min, width, counts, height, None)
}

/// A histogram from `min` with bins of `width`, and a vertical marker `(x, label)`.
pub(crate) fn histogram_with(
    ui: &mut Ui,
    name: &str,
    min: f64,
    width: f64,
    counts: &[f64],
    height: f32,
    marker: Option<(f64, &str)>,
) -> egui::Response {
    if let Some((_, label)) = marker {
        legend_row(ui, "", &[(label.to_owned(), colors::WARNING)]);
    }
    let mut plot = still(Plot::new(("utsim_res_plot", name)))
        .height(height)
        .include_y(0.0)
        .y_axis_label(
            RichText::new("contagem")
                .size(sizes::SMALL)
                .color(colors::TEXT_WEAK),
        );
    if let Some((x, _)) = marker {
        plot = plot.include_x(x);
    }
    plot.show(ui, |p| {
        let list: Vec<Bar> = counts
            .iter()
            .enumerate()
            .map(|(k, c)| {
                let lo = min + k as f64 * width;
                Bar::new(lo + width / 2.0, *c)
                    .width(width * 0.92)
                    .fill(BAR)
                    .stroke(Stroke::NONE)
                    .name(format!("{} a {}: {}", num(lo), num(lo + width), num(*c)))
            })
            .collect();
        p.bar_chart(BarChart::new("contagem", list));
        if let Some((x, label)) = marker {
            p.vline(VLine::new(label, x).color(colors::WARNING).width(2.0));
        }
    })
    .response
}

// ----- Análise --------------------------------------------------------------------------

fn analysis_tab(
    ui: &mut Ui,
    v: &ResultsView,
    mem: &mut ResultsMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    heading(ui, "Aquecimento");
    explain(
        ui,
        "Roda as replicações desde o tempo 0, sem aquecimento, e mostra a média entre elas (curva de Welch). O corte do MSER-5 é onde a série deixa de ter o começo vazio; \"Aplicar\" grava esse tempo em run.warmup.",
    );
    let cut = mem.warm.as_ref().and_then(|d| f(&d["suggested"]));
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Série").color(colors::TEXT));
        field(
            ui,
            reg,
            "Res_AquecSerie",
            &mut mem.warm_series,
            200.0,
            "wip ou objeto.medida",
        );
        ui.label(RichText::new("Janela").color(colors::TEXT));
        field(ui, reg, "Res_AquecJanela", &mut mem.warm_window, 50.0, "5");
        if button(
            ui,
            reg,
            "Res_AquecAnalisar",
            "Analisar",
            "Roda e calcula Welch e MSER-5",
            true,
        ) {
            out.push("resultados:trabalho:aquecimento".into());
        }
        if primary(
            ui,
            reg,
            "Res_AquecAplicar",
            "Aplicar",
            "Grava o corte sugerido em run.warmup do modelo",
            cut.is_some() && v.can_run,
        ) && let Some(c) = cut
        {
            out.push(format!("resultados:aplicar_aquecimento:{c}"));
        }
    });
    error(ui, &mem.warm_error);
    let wt = mem.warm.as_ref().map(warm_text).unwrap_or_default();
    if !wt.is_empty() {
        let r = ui.add(Label::new(RichText::new(&wt).color(colors::TEXT)).wrap());
        named(ui, reg, "Res_AquecTexto", &r, &wt);
    }
    if let Some(d) = &mem.warm {
        draw_chart(
            ui,
            reg,
            "Res_AquecGrafico",
            &warm_chart(d),
            "sem série",
            240.0,
        );
    }

    heading(ui, "Replicações necessárias");
    explain(
        ui,
        "Quantas replicações faltam para a semiamplitude do IC de 95 % ficar abaixo do alvo (5% é relativo à média; 0.5 é absoluto). \"Rodar mais\" acrescenta as que faltam às já feitas, com sementes novas.",
    );
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Precisão").color(colors::TEXT));
        field(ui, reg, "Res_Precisao", &mut mem.prec_target, 80.0, "5%");
        if button(
            ui,
            reg,
            "Res_PrecisaoCalcular",
            "Calcular",
            "Calcula as replicações necessárias",
            true,
        ) {
            mem.precision(v);
        }
        let more = mem.more();
        let label = if more > 0 {
            format!("Rodar mais {more}")
        } else {
            "Rodar mais".into()
        };
        if primary(
            ui,
            reg,
            "Res_RodarMais",
            &label,
            "Roda as replicações que faltam e junta às atuais",
            more > 0,
        ) {
            out.push("resultados:trabalho:mais".into());
        }
    });
    error(ui, &mem.prec_error);
    if let Some(p) = &mem.prec {
        let t = precision_table(p);
        draw_table(ui, reg, "Res_PrecisaoTabela", &t);
        let more = mem.more();
        let line = if more > 0 {
            format!("Para todas as medidas chegarem ao alvo faltam {more} replicação(ões).")
        } else {
            "Todas as medidas já estão dentro do alvo.".into()
        };
        let r = ui.add(Label::new(RichText::new(&line).color(colors::TEXT)).wrap());
        named(
            ui,
            reg,
            "Res_PrecisaoTexto",
            &r,
            &format!("{}\n{line}", t.text()),
        );
    }

    heading(ui, "Consistência: lei de Little");
    explain(
        ui,
        "Confere L = λ·W em cada objeto e no sistema. Verde: fecha; amarelo: a diferença é estatisticamente diferente de zero, mas pequena (até 2 %); vermelho: não fecha, o que costuma indicar aquecimento curto ou itens presos no fim.",
    );
    if button(
        ui,
        reg,
        "Res_Little",
        "Conferir",
        "Roda as replicações e confere a lei de Little",
        true,
    ) {
        out.push("resultados:trabalho:little".into());
    }
    error(ui, &mem.little_error);
    if let Some(d) = &mem.little {
        draw_table(ui, reg, "Res_LittleTexto", &little_table(d));
    }

    heading(ui, "Gargalo móvel");
    explain(
        ui,
        "Em cada instante, o gargalo é o recurso no período ativo mais longo; quando dois períodos se sobrepõem, o gargalo é compartilhado. Mostra a fração do tempo de cada recurso como gargalo único e compartilhado.",
    );
    if button(
        ui,
        reg,
        "Res_GargaloMovel",
        "Calcular",
        "Roda as replicações com o rastro e calcula o gargalo móvel",
        true,
    ) {
        out.push("resultados:trabalho:gargalo_movel".into());
    }
    error(ui, &mem.shifting_error);
    if let Some(d) = &mem.shifting {
        let t = shifting_text(d);
        let r = ui.add(Label::new(RichText::new(&t).color(colors::TEXT)).wrap());
        named(ui, reg, "Res_GargaloMovelTexto", &r, &t);
        draw_chart(
            ui,
            reg,
            "Res_GargaloMovelGrafico",
            &shifting_chart(d),
            "sem dados",
            160.0,
        );
    }
}

pub fn precision_table(p: &Value) -> Table {
    Table {
        header: ["Medida", "Atual", "Alvo (±)", "Faltam"]
            .map(String::from)
            .to_vec(),
        right: vec![false, true, true, true],
        rows: p["rows"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|r| {
                let more = r["more"].as_u64();
                Row {
                    cells: vec![
                        Cell::plain(text(&r["metric"])),
                        Cell::plain(mh(&r["current"], false)),
                        Cell::plain(f(&r["target"]).map_or("-".into(), num)),
                        Cell::colored(
                            more.map_or("?".into(), |m| m.to_string()),
                            if more.unwrap_or(0) > 0 {
                                colors::WARNING
                            } else {
                                colors::TEXT
                            },
                        ),
                    ],
                    hot: false,
                }
            })
            .collect(),
    }
}

pub fn little_table(d: &Value) -> Table {
    Table {
        header: ["Objeto", "L", "λ·W", "Diferença", "Semáforo"]
            .map(String::from)
            .to_vec(),
        right: vec![false, true, true, true, false],
        rows: little_rows(d)
            .iter()
            .map(|r| {
                let light = r["light"].as_str().unwrap_or("vermelho");
                let color = match light {
                    "verde" => colors::SUCCESS,
                    "amarelo" => colors::WARNING,
                    _ => colors::ERROR,
                };
                let name = text(&r["name"]);
                let system = name == "sistema";
                Row {
                    cells: vec![
                        if system {
                            Cell::colored(name, colors::HEADING)
                        } else {
                            Cell::plain(name)
                        },
                        Cell::plain(mh(&r["l"], false)),
                        Cell::plain(mh(&r["lambda_w"], false)),
                        Cell::plain(
                            f(&r["relative"])
                                .map_or("-".into(), |x| format!("{}%", num(x * 100.0))),
                        ),
                        Cell {
                            dot: Some(color),
                            ..Cell::colored(light, color)
                        },
                    ],
                    hot: false,
                }
            })
            .collect(),
    }
}

// ----- Cenários -------------------------------------------------------------------------

fn scenarios_tab(
    ui: &mut Ui,
    _v: &ResultsView,
    mem: &mut ResultsMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    explain(
        ui,
        "Um cenário por bloco: [nome] e, embaixo, as mudanças (objeto.campo = valor em RON, ou run.campo). O modelo como está entra primeiro, como base. Todos usam as mesmas sementes (números aleatórios comuns), então a diferença para a base é pareada; em amarelo, as diferenças significativas.",
    );
    area(
        ui,
        reg,
        "Res_CenariosTexto",
        &mut mem.sc_text,
        6,
        "[mais rápido]\nmaquina.process = Exp(mean: 2.0)\n\n[dois servidores]\nmaquina.capacity = 2",
    );
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Medidas (vírgulas; vazio: todas)").color(colors::TEXT));
        field(
            ui,
            reg,
            "Res_CenariosMedidas",
            &mut mem.sc_measures,
            260.0,
            "fila.content.mean, saida.input",
        );
        ui.label(RichText::new("Replicações").color(colors::TEXT));
        field(
            ui,
            reg,
            "Res_CenariosReplicacoes",
            &mut mem.sc_reps,
            50.0,
            "5",
        );
        if primary(
            ui,
            reg,
            "Res_CenariosComparar",
            "Comparar",
            "Roda todos os cenários e compara",
            true,
        ) {
            out.push("resultados:trabalho:cenarios".into());
        }
    });
    error(ui, &mem.sc_error);
    if let Some(d) = &mem.sc {
        draw_table(ui, reg, "Res_CenariosResultado", &scenario_table(d));
    }
}

pub fn scenario_table(d: &Value) -> Table {
    let names: Vec<String> = d["scenarios"]
        .as_array()
        .into_iter()
        .flatten()
        .map(text)
        .collect();
    let mut header = vec!["Medida".to_owned()];
    header.extend(names.iter().cloned());
    let mut right = vec![false];
    right.extend(names.iter().map(|_| true));
    Table {
        header,
        right,
        rows: d["rows"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|r| {
                let mut cells = vec![Cell::plain(text(&r["metric"]))];
                for k in 0..names.len() {
                    let mut c = Cell::plain(mh(&r["values"][k], false));
                    let diff = &r["differences"][k];
                    if diff.is_object() {
                        let sig = diff["significant"].as_bool() == Some(true);
                        c = c.tagged(
                            format!("Δ {}", mh(diff, false)),
                            if sig {
                                colors::WARNING
                            } else {
                                colors::TEXT_WEAK
                            },
                        );
                    }
                    cells.push(c);
                }
                Row { cells, hot: false }
            })
            .collect(),
    }
}

// ----- Otimização -----------------------------------------------------------------------

fn optimize_tab(
    ui: &mut Ui,
    _v: &ResultsView,
    mem: &mut ResultsMemory,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    explain(
        ui,
        "Procura os valores das variáveis que minimizam (ou maximizam) a média de uma métrica. Uma variável por linha: chave mínimo máximo [passo] (inteiros) ou com ponto decimal (contínua, cinco valores na grade). Os pontos não significativamente diferentes do melhor (teste pareado de 95 %) ficam marcados.",
    );
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Objetivo").color(colors::TEXT));
        field(
            ui,
            reg,
            "Res_OtimMetrica",
            &mut mem.op_metric,
            300.0,
            "saida.time_in_system.mean",
        );
        let r = ui.checkbox(&mut mem.op_max, "maximizar");
        named_toggle(ui, reg, "Res_OtimMaximizar", &r, "maximizar", mem.op_max);
    });
    area(
        ui,
        reg,
        "Res_OtimVariaveis",
        &mut mem.op_vars,
        3,
        "maquina.capacity 1 3",
    );
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Método").color(colors::TEXT));
        let items: Vec<String> = METHODS.iter().map(|s| s.to_string()).collect();
        if let Some(k) = combo(ui, reg, "Res_OtimMetodo", mem.op_method, &items, 130.0) {
            mem.op_method = k;
        }
        ui.label(RichText::new("Replicações por ponto").color(colors::TEXT));
        field(ui, reg, "Res_OtimReplicacoes", &mut mem.op_reps, 50.0, "5");
        ui.label(RichText::new("Orçamento (pontos)").color(colors::TEXT));
        field(ui, reg, "Res_OtimOrcamento", &mut mem.op_budget, 60.0, "50");
        if primary(ui, reg, "Res_Otimizar", "Otimizar", "Roda a busca", true) {
            out.push("resultados:trabalho:otimizacao".into());
        }
    });
    error(ui, &mem.op_error);
    if let Some(d) = &mem.op {
        let best = format!(
            "Melhor ponto: {} ({} ponto(s) simulados)",
            d["best"].as_str().unwrap_or("nenhum viável"),
            text(&d["simulated"])
        );
        let r = ui.label(RichText::new(&best).strong().color(colors::TEXT_STRONG));
        for line in d["explanation"].as_array().into_iter().flatten() {
            explain(ui, &text(line));
        }
        let t = optimization_table(d);
        draw_table(ui, reg, "Res_OtimTabela", &t);
        named(
            ui,
            reg,
            "Res_OtimResultado",
            &r,
            &format!("{best}\n{}", t.text()),
        );
        draw_chart(
            ui,
            reg,
            "Res_OtimGrafico",
            &history_chart(d),
            "sem histórico",
            200.0,
        );
    }
}

pub fn optimization_table(d: &Value) -> Table {
    Table {
        header: ["Ponto", "Objetivo", ""].map(String::from).to_vec(),
        right: vec![false, true, false],
        rows: d["evaluations"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|e| {
                let is = |k: &str| e[k].as_bool() == Some(true);
                let mark = if is("best") {
                    Cell::colored("melhor", colors::WARNING)
                } else if is("equivalent") {
                    Cell::colored("equivalente", colors::TEXT)
                } else if !is("feasible") {
                    Cell::colored("inviável", colors::ERROR)
                } else {
                    Cell::plain("")
                };
                Row {
                    cells: vec![
                        Cell::plain(text(&e["point"])),
                        Cell::plain(mh(&e["objective"], false)),
                        mark,
                    ],
                    hot: is("best"),
                }
            })
            .collect(),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::results::tests::example;

    pub(crate) fn view(name: &str, reps: u32) -> ResultsView {
        let mut m = example(name);
        m.run.replications = reps;
        let reports = m.run_replications().unwrap();
        ResultsView::new(RunOutput::new(m, reports), true)
    }

    #[test]
    fn flat_curves_get_a_sensible_range() {
        let flat = |y: f64| Curve {
            name: "média".into(),
            points: vec![[0.0, y], [10.0, y]],
            color: colors::WARNING,
            width: 1.0,
        };
        assert_eq!(y_range(&[flat(0.0)]), (0.0, 0.5));
        let (lo, hi) = y_range(&[flat(1.0)]);
        assert!(lo == 0.0 && hi > 1.0 && hi < 2.0, "{lo} {hi}");
        let (lo, hi) = y_range(&[flat(-3.0), flat(5.0)]);
        assert!(lo < -3.0 && hi > 5.0 && hi < 6.0, "{lo} {hi}");
        assert_eq!(y_range(&[]), (0.0, 1.0));
    }

    #[test]
    fn numbers_keep_four_significant_digits() {
        assert_eq!(num(0.0), "0");
        assert_eq!(num(12.3456), "12.35");
        assert_eq!(num(1234.6), "1235");
        assert_eq!(num(0.012345), "0.01235");
        assert_eq!(num(2.0), "2");
        assert_eq!(num(f64::NAN), "-");
        let s = serde_json::json!({"mean": 0.5, "half_width": 0.02});
        assert_eq!(mh(&s, true), "50 ± 2%");
        assert_eq!(
            mh(&serde_json::json!({"mean": 3.0, "half_width": null}), false),
            "3"
        );
        assert_eq!(mh(&Value::Null, false), "-");
    }

    #[test]
    fn the_table_marks_the_bottleneck_and_aligns_numbers() {
        let v = view("servidores.ron", 3);
        let t = object_table(&v);
        assert_eq!(t.header.len(), 7);
        assert_eq!(t.right, vec![false, false, true, true, true, true, true]);
        let b = v.bottleneck().unwrap();
        let hot: Vec<_> = t.rows.iter().filter(|r| r.hot).collect();
        assert!(!hot.is_empty());
        assert!(t.text().contains(&format!("{b} (gargalo)")), "{}", t.text());
        assert!(t.text().contains(" ± "));
        let row = t.rows.iter().find(|r| r.cells[0].text == b).unwrap();
        assert_eq!(row.cells[0].color, colors::WARNING);
        assert!(v.summary_text().contains(&format!("Gargalo (geral): {b}")));
        assert!(v.summary_text().contains("3 replicação(ões)"));
        let c = cards(&v);
        assert_eq!(c[2].1, b);
        assert!(c[2].3);
    }

    #[test]
    fn charts_count_what_they_draw() {
        let v = view("servidores.ron", 2);
        let mut mem = ResultsMemory::default();
        mem.on_new_results(&v, "/tmp/x_resultados");
        assert_eq!(mem.export_dir, "/tmp/x_resultados");
        assert_eq!(Some(mem.object.clone()), v.bottleneck());
        assert!(chosen_chart(&v, &mem).count() > 0);
        assert!(mem.choose_chart("estados"));
        assert_eq!(mem.tab, 1);
        let states = chosen_chart(&v, &mem);
        assert!(states.count() > 1);
        if let Chart::Stacked(rows) = &states {
            for (_, segs) in rows {
                let total: f64 = segs.iter().map(|s| s.fraction).sum();
                assert!((total - 1.0).abs() < 1e-6);
            }
        }
        // The model records no series: nothing to draw, and the chart says so.
        mem.choose_chart("serie");
        assert_eq!(chosen_chart(&v, &mem).count(), 0);
        mem.choose_chart("histograma");
        assert_eq!(chosen_chart(&v, &mem).count(), 0);
        assert!(!mem.choose_chart("outro"));
    }

    #[test]
    fn runs_are_built_from_the_forms() {
        let v = view("servidores.ron", 2);
        let mut mem = ResultsMemory::default();
        assert!(matches!(
            task("aquecimento", &v, &mem),
            Ok(Task::Warmup { window: 5, .. })
        ));
        mem.warm_window = "x".into();
        assert!(task("aquecimento", &v, &mem).is_err());
        assert!(
            task("mais", &v, &mem).is_err(),
            "sem calcular, nada a rodar"
        );
        mem.prec_target = "1%".into();
        mem.precision(&v);
        assert!(mem.more() > 0);
        assert!(matches!(task("mais", &v, &mem), Ok(Task::More { .. })));
        mem.prec_target = "muito".into();
        mem.precision(&v);
        assert!(!mem.prec_error.is_empty());
        assert!(task("cenarios", &v, &mem).is_err(), "sem cenários");
        mem.sc_text = "[dois]\nmaquina.capacity = 2".into();
        mem.sc_measures = "fila.content.mean, saida.input".into();
        match task("cenarios", &v, &mem) {
            Ok(Task::Scenarios {
                scenarios,
                measures,
                model,
            }) => {
                assert_eq!(scenarios.len(), 2);
                assert_eq!(measures.len(), 2);
                assert_eq!(model.run.replications, 5);
            }
            _ => panic!("cenários"),
        }
        mem.op_metric = "fila.content.mean".into();
        mem.op_vars = "maquina.capacity 1 3".into();
        mem.op_reps = "2".into();
        assert!(matches!(
            task("otimizacao", &v, &mem),
            Ok(Task::Optimize { .. })
        ));
        let replay = ResultsView::new(v.run.clone(), false);
        assert!(task("otimizacao", &replay, &mem).is_err());
        assert!(task("little", &v, &mem).is_ok());
    }

    #[test]
    fn finished_runs_fill_the_tabs() {
        let v = view("servidores.ron", 2);
        let mut mem = ResultsMemory::default();
        let little = serde_json::json!({
            "objects": [{"name": "fila", "l": {"mean": 1.0}, "lambda_w": {"mean": 1.0}, "relative": 0.0, "light": "verde"}],
            "system": {"l": {"mean": 2.0}, "lambda_w": {"mean": 2.2}, "relative": 0.1, "light": "vermelho"},
        });
        let s = done("little", Some(&little), &v, &mut mem);
        assert_eq!(s, "Lei de Little: 1 verde(s), 0 amarelo(s), 1 vermelho(s).");
        let t = little_table(mem.little.as_ref().unwrap()).text();
        assert!(t.contains("sistema") && t.contains("● verde"), "{t}");
        let sc = serde_json::json!({
            "scenarios": ["base", "dois servidores"],
            "rows": [{"metric": "fila.content.mean",
                      "values": [{"mean": 3.0, "half_width": 0.2}, {"mean": 1.0, "half_width": 0.1}],
                      "differences": [null, {"mean": -2.0, "half_width": 0.3, "significant": true}]}],
        });
        assert_eq!(
            done("cenarios", Some(&sc), &v, &mut mem),
            "2 cenário(s) comparados."
        );
        let t = scenario_table(mem.sc.as_ref().unwrap());
        assert!(t.text().contains("dois servidores") && t.text().contains("Δ "));
        assert_eq!(t.rows[0].cells[2].tag.as_ref().unwrap().1, colors::WARNING);
        let warm = serde_json::json!({"series": "wip", "mean": [[0.0, 1.0], [1.0, 2.0]],
            "welch": [[0.0, 1.0], [1.0, 2.0]], "suggested": 12.5, "too_short": false, "points": 2});
        assert!(
            done("aquecimento", Some(&warm), &v, &mut mem)
                .contains("Corte sugerido (MSER-5): 12.5")
        );
        assert_eq!(warm_chart(&warm).count(), 2);
        let op = serde_json::json!({"best": "maquina.capacity = 3", "simulated": 3,
            "evaluations": [{"point": "maquina.capacity = 3", "objective": {"mean": 0.5}, "feasible": true, "best": true, "equivalent": false}],
            "history": [[1.0, 2.0], [2.0, 0.5]], "explanation": ["grade"]});
        assert!(done("otimizacao", Some(&op), &v, &mut mem).contains("maquina.capacity = 3"));
        assert!(optimization_table(&op).rows[0].hot);
        let sh = serde_json::json!([{"name": "maquina", "sole": {"mean": 0.8}, "shifting": {"mean": 0.1}, "total": 0.9}]);
        assert_eq!(
            done("gargalo_movel", Some(&sh), &v, &mut mem),
            "Gargalo móvel: maquina na maior parte do tempo."
        );
        assert_eq!(shifting_chart(&sh).count(), 1);
        assert_eq!(done("little", None, &v, &mut mem), "");
    }
}
