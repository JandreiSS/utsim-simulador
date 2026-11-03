//! The "Resultados" window (MVP phase 10): runs N replications of the open model, the
//! summary table with the bottleneck, charts, scenario comparison and optimisation,
//! and the exports, without leaving the editor.
//!
//! Runs go to a background thread on the desktop and run in place in the browser (no
//! threads there), like "Simular". The thread reports progress through a channel and
//! stops at the next replication or point when "Cancelar" sets the shared flag. All
//! numbers come from `utsim_model::analysis` and `utsim_opt`; the pure reshaping for
//! the screen is in `utsim_viewer::results`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::{Arc, Mutex};

use bevy_egui::egui::{self, Color32, RichText};
use egui_plot::{Bar, BarChart, Legend, Line, Plot, PlotPoints};
use utsim_core::RunReport;
use utsim_core::stats::Summary;
use utsim_edit::Command;
use utsim_model::analysis::{self, ObjectStats, bottleneck};
use utsim_model::experiment::{ExperimentResult, run_scenarios};
use utsim_model::{Model, ModelError};
use utsim_opt::{OptProblem, OptResult};
use utsim_viewer::results::{
    self as logic, Column, DomainKind, Method, OptForm, RunForm, ScenarioForm, UtilKind,
    VariableRow, WhiskerBar,
};

/// Orange of the bottleneck and of significant differences.
const HOT: Color32 = Color32::from_rgb(255, 150, 60);
const RED: Color32 = Color32::from_rgb(255, 120, 110);
const BAR: Color32 = Color32::from_rgb(90, 150, 240);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Summary,
    Charts,
    Scenarios,
    Optimize,
}

impl Tab {
    pub fn label(self) -> &'static str {
        match self {
            Tab::Summary => "Resumo",
            Tab::Charts => "Gráficos",
            Tab::Scenarios => "Cenários",
            Tab::Optimize => "Otimização",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Chart {
    #[default]
    Utilization,
    States,
    Series,
    Histogram,
}

impl Chart {
    const ALL: [Chart; 4] = [
        Chart::Utilization,
        Chart::States,
        Chart::Series,
        Chart::Histogram,
    ];

    fn label(self) -> &'static str {
        match self {
            Chart::Utilization => "Ocupação",
            Chart::States => "Estados",
            Chart::Series => "Série temporal",
            Chart::Histogram => "Histograma",
        }
    }
}

/// Results of the latest run.
pub struct RunOutput {
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))] // read by the exports (desktop)
    pub model: Model,
    pub form: RunForm,
    pub reports: Vec<RunReport>,
    pub stats: Vec<ObjectStats>,
    pub metrics: BTreeMap<String, Summary>,
}

enum Msg {
    Progress(usize, usize),
    Run(Result<Box<RunOutput>, String>),
    Scenarios(Result<ExperimentResult, String>),
    Optimized(Result<Box<(OptProblem, OptResult)>, String>),
}

struct Job {
    what: &'static str,
    rx: Mutex<Receiver<Msg>>,
    cancel: Arc<AtomicBool>,
    done: usize,
    total: usize,
}

#[derive(Default)]
pub struct Results {
    pub open: bool,
    pub tab: Tab,
    form: Option<RunForm>,
    pub run: Option<RunOutput>,
    sort: Option<(Column, bool)>,
    kind: Option<String>,
    chart: Chart,
    util: UtilKind,
    object: Option<String>,
    tally: String,
    scenarios: ScenarioForm,
    pick: String,
    metric_filter: String,
    pub experiment: Option<ExperimentResult>,
    bar_metric: String,
    opt: OptForm,
    opt_pick: String,
    pub optimized: Option<(OptProblem, OptResult)>,
    job: Option<Job>,
    status: String,
    errors: Vec<String>,
    export_dir: String,
    experiment_path: String,
    /// Every file written by the exports (full paths).
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))] // exports are desktop-only
    pub exported: Vec<String>,
}

impl Results {
    pub fn running(&self) -> bool {
        self.job.is_some()
    }

    #[cfg_attr(target_arch = "wasm32", allow(dead_code))] // read by scripts (desktop)
    pub fn bottleneck(&self) -> Option<String> {
        let run = self.run.as_ref()?;
        bottleneck(&run.stats).map(|i| run.stats[i].name.clone())
    }

    #[cfg_attr(target_arch = "wasm32", allow(dead_code))] // read by scripts (desktop)
    pub fn best_point(&self) -> Option<String> {
        let (_, r) = self.optimized.as_ref()?;
        r.best_evaluation().map(|e| logic::point_text(&e.point))
    }

    /// Opens (or closes) the window; the first time, the run settings come from `model`.
    pub fn toggle(&mut self, model: &Model, path: Option<&Path>) {
        self.open = !self.open;
        if self.form.is_none() {
            self.form = Some(RunForm::of(&model.run));
        }
        if self.scenarios.rows.is_empty() {
            self.scenarios = ScenarioForm::new(&model.name);
        }
        if self.export_dir.is_empty() {
            let (dir, stem) = place(path, &model.name);
            self.export_dir = dir.join(format!("{stem}_resultados")).display().to_string();
            self.experiment_path = dir
                .join(format!("{stem}_experimento.ron"))
                .display()
                .to_string();
        }
    }

    /// Starts `work` on another thread (in place in the browser). `work` gets a
    /// progress function: `(done, total)`, returning false once "Cancelar" was pressed.
    fn spawn<F>(&mut self, what: &'static str, total: usize, work: F)
    where
        F: FnOnce(&(dyn Fn(usize, usize) -> bool + Sync)) -> Msg + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let flag = cancel.clone();
        let job = move || {
            let progress_tx = tx.clone();
            let progress = move |done: usize, total: usize| {
                let _ = progress_tx.send(Msg::Progress(done, total));
                !flag.load(Ordering::Relaxed)
            };
            let msg = work(&progress);
            let _ = tx.send(msg);
        };
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(job);
        #[cfg(target_arch = "wasm32")]
        job();
        self.errors.clear();
        self.status = format!("{what}...");
        self.job = Some(Job {
            what,
            rx: Mutex::new(rx),
            cancel,
            done: 0,
            total,
        });
    }

    /// Picks up progress and results of the running job.
    pub fn poll(&mut self) {
        let Some(job) = self.job.as_mut() else {
            return;
        };
        let mut finished: Option<Msg> = None;
        loop {
            let received = match job.rx.lock() {
                Ok(rx) => rx.try_recv(),
                Err(_) => Err(TryRecvError::Disconnected),
            };
            match received {
                Ok(Msg::Progress(d, t)) => {
                    job.done = d;
                    job.total = t;
                }
                Ok(other) => {
                    finished = Some(other);
                    break;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    finished = Some(Msg::Run(Err("a execução terminou sem resultado".into())));
                    break;
                }
            }
        }
        let Some(msg) = finished else {
            return;
        };
        let what = job.what;
        self.job = None;
        let outcome = match msg {
            Msg::Progress(..) => Ok(()),
            Msg::Run(r) => r.map(|out| {
                self.status = format!(
                    "{} replicação(ões) concluídas{}",
                    out.reports.len(),
                    bottleneck(&out.stats)
                        .map(|i| format!("; gargalo: {}", out.stats[i].name))
                        .unwrap_or_default()
                );
                if self.object.is_none() {
                    self.object = bottleneck(&out.stats).map(|i| out.stats[i].name.clone());
                }
                self.run = Some(*out);
            }),
            Msg::Scenarios(r) => r.map(|exp| {
                self.status = format!("{} cenário(s) comparados", exp.scenarios.len());
                let names = exp.metric_names();
                if !names.contains(&self.bar_metric) {
                    self.bar_metric = names.first().cloned().unwrap_or_default();
                }
                self.experiment = Some(exp);
            }),
            Msg::Optimized(r) => r.map(|b| {
                let (p, res) = *b;
                self.status = format!(
                    "{} ponto(s) avaliados{}",
                    res.evaluations.len(),
                    res.best_evaluation()
                        .map(|e| format!("; melhor: {}", logic::point_text(&e.point)))
                        .unwrap_or_default()
                );
                self.optimized = Some((p, res));
            }),
        };
        if let Err(e) = outcome {
            self.status = if e == ModelError::Cancelled.to_string() {
                format!("{what}: cancelado")
            } else {
                format!("{what}: erro")
            };
            if e != ModelError::Cancelled.to_string() {
                self.errors = vec![e];
            }
        }
    }

    fn start_run(&mut self, base: &Model) {
        let Some(form) = self.form.clone() else {
            return;
        };
        let model = match form.apply(base) {
            Ok(m) => m,
            Err(e) => {
                self.errors = vec![e];
                return;
            }
        };
        let total = model.run.replications as usize;
        self.spawn("Rodando replicações", total, move |progress| {
            let result = model
                .run_replications_with(&|done| progress(done as usize, total))
                .map_err(|e| e.to_string())
                .map(|reports| RunOutput {
                    stats: analysis::object_stats(&model, &reports),
                    metrics: utsim_core::summarize(&reports),
                    model,
                    form,
                    reports,
                });
            Msg::Run(result.map(Box::new))
        });
    }

    fn start_scenarios(&mut self, base: &Model) {
        let Some(form) = self.form.clone() else {
            return;
        };
        let scenarios = match self.scenarios.build(base) {
            Ok(s) => s,
            Err(e) => {
                self.errors = e;
                return;
            }
        };
        let model = match form.apply(base) {
            Ok(mut m) => {
                // Comparisons need neither series nor histograms.
                m.run.sample_interval = None;
                m.run.histogram_bins = None;
                m
            }
            Err(e) => {
                self.errors = vec![e];
                return;
            }
        };
        let name = self.scenarios.name.clone();
        let measures = self.scenarios.measures.clone();
        let reps = model.run.replications as usize;
        let total = reps * scenarios.len();
        self.spawn("Comparando cenários", total, move |progress| {
            let result = run_scenarios(&model, &scenarios, &measures, &|i, done| {
                progress(i * reps + done as usize, total)
            })
            .map_err(|e| e.to_string())
            .map(|scenarios| ExperimentResult {
                name,
                measures,
                scenarios,
            });
            Msg::Scenarios(result)
        });
    }

    fn start_optimize(&mut self, base: &Model, model_file: &str) {
        let problem = match self.opt.build(base, model_file) {
            Ok(p) => p,
            Err(e) => {
                self.errors = e;
                return;
            }
        };
        let base = base.clone();
        let total = utsim_opt::planned_evaluations(&problem);
        self.spawn("Otimizando", total, move |progress| {
            let result = utsim_opt::optimize_with(&problem, &base, &mut |d, t| progress(d, t))
                .map_err(|e| e.to_string())
                .map(|r| (problem, r));
            Msg::Optimized(result.map(Box::new))
        });
    }
}

/// Folder and file stem used for the default export paths.
fn place(path: Option<&Path>, name: &str) -> (PathBuf, String) {
    match path {
        Some(p) => (
            p.parent().map(Path::to_path_buf).unwrap_or_default(),
            p.file_stem()
                .map_or("modelo".into(), |s| s.to_string_lossy().to_string()),
        ),
        None => (
            PathBuf::from("."),
            name.chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect(),
        ),
    }
}

/// What the window asks the editor to do.
pub enum Request {
    Exec(Command),
}

/// The window. `model` is the document's model, `path` its file, `blocked` the model's
/// issues (nothing runs while there are any).
pub fn window(
    ctx: &egui::Context,
    res: &mut Results,
    model: &Model,
    path: Option<&Path>,
    blocked: usize,
) -> Vec<Request> {
    res.poll();
    let mut requests = Vec::new();
    if !res.open {
        return requests;
    }
    let mut open = true;
    egui::Window::new("Resultados")
        .id(egui::Id::new("janela_resultados"))
        .open(&mut open)
        .default_size([1000.0, 640.0])
        .default_pos([150.0, 60.0])
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                for tab in [Tab::Summary, Tab::Charts, Tab::Scenarios, Tab::Optimize] {
                    if ui.selectable_label(res.tab == tab, tab.label()).clicked() {
                        res.tab = tab;
                    }
                }
            });
            ui.separator();
            job_bar(ui, res);
            if blocked > 0 {
                ui.colored_label(
                    RED,
                    format!(
                        "O modelo tem {blocked} problema(s): corrija-os em Editar antes de rodar."
                    ),
                );
            }
            for e in &res.errors {
                ui.colored_label(RED, e);
            }
            ui.separator();
            let can_run = blocked == 0 && !res.running();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| match res.tab {
                    Tab::Summary => summary_tab(ui, res, model, path, can_run, &mut requests),
                    Tab::Charts => charts_tab(ui, res, model, can_run),
                    Tab::Scenarios => scenarios_tab(ui, res, model, path, can_run),
                    Tab::Optimize => optimize_tab(ui, res, model, path, can_run, &mut requests),
                });
        });
    if !open {
        res.open = false;
    }
    if res.running() {
        ctx.request_repaint();
    }
    requests
}

fn job_bar(ui: &mut egui::Ui, res: &mut Results) {
    if let Some(job) = &res.job {
        ui.horizontal(|ui| {
            let f = if job.total > 0 {
                job.done as f32 / job.total as f32
            } else {
                0.0
            };
            ui.add(
                egui::ProgressBar::new(f)
                    .desired_width(320.0)
                    .text(format!("{}: {} de {}", job.what, job.done, job.total)),
            );
            if ui.button("Cancelar").clicked() {
                job.cancel.store(true, Ordering::Relaxed);
            }
        });
    } else if !res.status.is_empty() {
        ui.label(&res.status);
    }
}

fn summary_tab(
    ui: &mut egui::Ui,
    res: &mut Results,
    model: &Model,
    path: Option<&Path>,
    can_run: bool,
    requests: &mut Vec<Request>,
) {
    let form = res.form.get_or_insert_with(|| RunForm::of(&model.run));
    ui.strong("Rodada");
    egui::Grid::new("res_rodada")
        .num_columns(4)
        .spacing([12.0, 4.0])
        .show(ui, |ui| {
            ui.label("Replicações");
            ui.add(egui::DragValue::new(&mut form.replications).range(1..=1000));
            ui.label("Semente");
            ui.add(egui::DragValue::new(&mut form.seed));
            ui.end_row();
            ui.label("Tempo de parada");
            ui.add(
                egui::DragValue::new(&mut form.stop_time)
                    .range(1e-6..=f64::MAX)
                    .speed(10.0),
            );
            ui.label("Aquecimento");
            ui.add(
                egui::DragValue::new(&mut form.warmup)
                    .range(0.0..=f64::MAX)
                    .speed(10.0),
            );
            ui.end_row();
            ui.checkbox(&mut form.series, "Séries temporais a cada");
            ui.add_enabled(
                form.series,
                egui::DragValue::new(&mut form.sample_interval)
                    .range(1e-6..=f64::MAX)
                    .speed(1.0),
            );
            ui.checkbox(&mut form.histograms, "Histogramas com classes");
            ui.add_enabled(
                form.histograms,
                egui::DragValue::new(&mut form.bins).range(1..=200),
            );
            ui.end_row();
        });
    ui.horizontal(|ui| {
        if ui
            .add_enabled(can_run, egui::Button::new("Rodar replicações"))
            .clicked()
        {
            res.start_run(model);
        }
        if ui
            .button("Gravar no modelo")
            .on_hover_text("Grava replicações, tempos e semente no modelo (Ctrl+Z desfaz)")
            .clicked()
            && let Some(f) = &res.form
        {
            requests.push(Request::Exec(f.commands()));
        }
        if ui.button("Restaurar do modelo").clicked() {
            res.form = Some(RunForm::of(&model.run));
        }
    });
    let Some(run) = &res.run else {
        ui.separator();
        ui.weak("Rode as replicações para ver o resumo por objeto, o gargalo e os gráficos.");
        return;
    };
    ui.separator();
    ui.label(format!(
        "{}. Cada valor: média entre replicações ± meia-amplitude do IC de 95%.",
        run.form.describe()
    ));
    let hot = bottleneck(&run.stats);
    if let Some(i) = hot {
        let s = &run.stats[i];
        ui.horizontal(|ui| {
            ui.label("Gargalo (geral):");
            ui.label(RichText::new(&s.name).color(HOT).strong());
            ui.label(format!(
                "(ocupação {}; a maior entre processos e recursos)",
                logic::pct(&s.utilization.unwrap_or(s.content))
            ));
        });
    }
    if let Some(i) = analysis::bottleneck_in_shift(&run.stats) {
        let s = &run.stats[i];
        ui.horizontal(|ui| {
            ui.label("Gargalo (dentro do turno):");
            ui.label(RichText::new(&s.name).color(HOT).strong());
            ui.label(format!(
                "(ocupação {} no tempo em turno)",
                logic::pct(
                    &s.utilization_in_shift
                        .or(s.utilization)
                        .unwrap_or(s.content)
                )
            ));
        });
    }
    ui.horizontal(|ui| {
        ui.label("Tipo:");
        let text = res
            .kind
            .as_deref()
            .map_or("todos".to_string(), |k| logic::kind_label(k).to_string());
        egui::ComboBox::from_id_salt("res_tipo")
            .selected_text(text)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut res.kind, None, "todos");
                for k in logic::kinds(&run.stats) {
                    let label = logic::kind_label(&k);
                    ui.selectable_value(&mut res.kind, Some(k.clone()), label);
                }
            });
        ui.weak("Clique no título de uma coluna para ordenar.");
    });
    let (column, desc) = res.sort.unwrap_or((Column::Name, false));
    let order = match res.sort {
        Some(_) => logic::table_order(&run.stats, res.kind.as_deref(), column, desc),
        None => (0..run.stats.len())
            .filter(|&i| res.kind.as_deref().is_none_or(|k| run.stats[i].kind == k))
            .collect(),
    };
    let mut clicked: Option<Column> = None;
    egui::Grid::new("res_tabela")
        .striped(true)
        .spacing([14.0, 3.0])
        .show(ui, |ui| {
            for c in Column::ALL {
                let mark = match res.sort {
                    Some((sc, d)) if sc == c => {
                        if d {
                            " (decr.)"
                        } else {
                            " (cresc.)"
                        }
                    }
                    _ => "",
                };
                let mut button = ui.add(
                    egui::Button::new(RichText::new(format!("{}{mark}", c.label())).strong())
                        .frame(false),
                );
                if let Some(k) = c.util_kind() {
                    button = button.on_hover_text(k.help());
                }
                if button.clicked() {
                    clicked = Some(c);
                }
            }
            ui.strong("Estados");
            ui.end_row();
            for &i in &order {
                let s = &run.stats[i];
                let color = (hot == Some(i)).then_some(HOT);
                let text = |t: String| {
                    let r = RichText::new(t);
                    match color {
                        Some(c) => r.color(c).strong(),
                        None => r,
                    }
                };
                ui.label(text(s.name.clone()));
                ui.label(text(logic::kind_label(&s.kind).to_string()));
                ui.label(text(logic::cell(&s.input)));
                ui.label(text(logic::cell(&s.output)));
                ui.label(text(logic::cell(&s.content)));
                ui.label(text(logic::cell(&s.staytime)));
                for c in &Column::ALL[6..] {
                    let u = c.util_kind().and_then(|k| k.pick(s));
                    ui.label(text(u.as_ref().map(logic::pct).unwrap_or_default()));
                }
                let states: Vec<String> = s
                    .states
                    .iter()
                    .map(|(k, v)| format!("{} {}", logic::state_label(k), logic::pct(v)))
                    .collect();
                ui.label(states.join(" | "));
                ui.end_row();
            }
        });
    if let Some(c) = clicked {
        res.sort = Some(match res.sort {
            Some((sc, d)) if sc == c => (c, !d),
            // Numbers start from the largest; names from A.
            _ => (c, !matches!(c, Column::Name | Column::Kind)),
        });
    }
    // Each tally gives its mean and, when the run recorded it, how many observations;
    // `model.*` are the totals over every source that can lose items.
    let own: Vec<(String, Summary)> = run
        .stats
        .iter()
        .flat_map(|s| {
            let counts = &run.metrics;
            s.extra
                .iter()
                .flat_map(move |(k, v)| {
                    let count = format!("{}.{k}.count", s.name);
                    [
                        Some((format!("{}.{k}.mean", s.name), *v)),
                        counts.get(&count).map(|c| (count, *c)),
                    ]
                })
                .flatten()
                .chain(
                    s.custom
                        .iter()
                        // `servers.*` only feed the utilisation columns.
                        .filter(|(k, _)| !k.starts_with("servers."))
                        .map(move |(k, v)| (format!("{}.{k}", s.name), *v)),
                )
        })
        .chain(analysis::model_stats(&run.reports))
        .collect();
    if !own.is_empty() {
        ui.separator();
        ui.strong("Métricas próprias (tempo no sistema, fluxos, níveis de tanques...)");
        egui::Grid::new("res_proprias")
            .striped(true)
            .show(ui, |ui| {
                for (k, v) in &own {
                    ui.monospace(k);
                    ui.label(logic::cell(v));
                    ui.end_row();
                }
            });
    }
    export_ui(ui, res, path);
}

#[cfg(not(target_arch = "wasm32"))]
fn export_ui(ui: &mut egui::Ui, res: &mut Results, _path: Option<&Path>) {
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Pasta de exportação:");
        ui.add(egui::TextEdit::singleline(&mut res.export_dir).desired_width(420.0));
    });
    ui.horizontal(|ui| {
        let ready = res.run.is_some();
        if ui
            .add_enabled(ready, egui::Button::new("Exportar CSV"))
            .on_hover_text("resumo.csv, objetos.csv, replicacoes.csv e, se houver, series.csv e histogramas.csv")
            .clicked()
        {
            export_csv(res);
        }
        if ui
            .add_enabled(ready, egui::Button::new("Exportar HTML"))
            .on_hover_text("Relatório com tabelas e gráficos (inclui a comparação de cenários, se houver)")
            .clicked()
        {
            export_html(res);
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn export_ui(ui: &mut egui::Ui, _res: &mut Results, _path: Option<&Path>) {
    ui.separator();
    ui.weak("A exportação de arquivos existe só no aplicativo de mesa.");
}

#[cfg(not(target_arch = "wasm32"))]
fn export_csv(res: &mut Results) {
    let Some(run) = &res.run else { return };
    let dir = PathBuf::from(res.export_dir.trim());
    let result = utsim_model::csv::export(&dir, &run.reports).and_then(|mut files| {
        std::fs::write(
            dir.join("objetos.csv"),
            utsim_model::csv::objects_csv(&run.stats),
        )?;
        files.insert(1, "objetos.csv".to_string());
        Ok(files)
    });
    match result {
        Ok(files) => {
            res.status = format!("CSV gravado em {}: {}", dir.display(), files.join(", "));
            res.exported
                .extend(files.iter().map(|f| dir.join(f).display().to_string()));
            res.errors.clear();
        }
        Err(e) => res.errors = vec![format!("não foi possível gravar em {}: {e}", dir.display())],
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn export_html(res: &mut Results) {
    let Some(run) = &res.run else { return };
    let color = |s: &str| logic::state_color(s);
    let label = |s: &str| logic::state_label(s);
    let html = utsim_model::html::report(&utsim_model::html::ReportInput {
        title: &run.model.name,
        settings: run.form.describe(),
        stats: &run.stats,
        reports: &run.reports,
        state_color: &color,
        state_label: &label,
        experiment: res.experiment.as_ref(),
    });
    let dir = PathBuf::from(res.export_dir.trim());
    let file = dir.join("relatorio.html");
    match std::fs::create_dir_all(&dir).and_then(|()| std::fs::write(&file, html)) {
        Ok(()) => {
            res.status = format!("Relatório gravado em {}", file.display());
            res.exported.push(file.display().to_string());
            res.errors.clear();
        }
        Err(e) => res.errors = vec![format!("não foi possível gravar {}: {e}", file.display())],
    }
}

fn charts_tab(ui: &mut egui::Ui, res: &mut Results, model: &Model, can_run: bool) {
    ui.horizontal(|ui| {
        for c in Chart::ALL {
            if ui.selectable_label(res.chart == c, c.label()).clicked() {
                res.chart = c;
            }
        }
    });
    let Some(run) = &res.run else {
        ui.weak("Rode as replicações na aba Resumo para ver os gráficos.");
        return;
    };
    ui.separator();
    match res.chart {
        Chart::Utilization => {
            let bars = logic::utilization_bars(&run.stats, res.util);
            if bars.is_empty() {
                ui.weak("O modelo não tem processos nem recursos móveis.");
                return;
            }
            ui.horizontal(|ui| {
                for k in UtilKind::ALL {
                    if ui
                        .selectable_label(res.util == k, k.label())
                        .on_hover_text(k.help())
                        .clicked()
                    {
                        res.util = k;
                    }
                }
            });
            ui.label(format!(
                "{} de cada recurso (%), com o intervalo de 95%, nunca acima de 100% por \
                 servidor. Em laranja, o gargalo (maior ocupação).",
                res.util.label()
            ));
            ui.weak(res.util.help());
            whisker_chart(ui, "graf_ocupacao", &bars, 100.0, "%");
        }
        Chart::States => {
            let rows: Vec<usize> = (0..run.stats.len())
                .filter(|&i| res.kind.as_deref().is_none_or(|k| run.stats[i].kind == k))
                .collect();
            ui.label("Fração do tempo em cada estado (%), com as cores do 3D.");
            states_chart(ui, &logic::state_bars(&run.stats, &rows));
        }
        Chart::Series | Chart::Histogram => {
            let names: Vec<String> = run.stats.iter().map(|s| s.name.clone()).collect();
            let mut object = res
                .object
                .clone()
                .filter(|o| names.contains(o))
                .unwrap_or_else(|| names.first().cloned().unwrap_or_default());
            ui.horizontal(|ui| {
                ui.label("Objeto:");
                egui::ComboBox::from_id_salt("graf_objeto")
                    .selected_text(&object)
                    .show_ui(ui, |ui| {
                        for n in &names {
                            ui.selectable_value(&mut object, n.clone(), n);
                        }
                    });
            });
            res.object = Some(object.clone());
            if res.chart == Chart::Series {
                series_chart(ui, &run.reports, &object, run.form.series);
                if !run.form.series {
                    offer_rerun(ui, res, model, can_run, true);
                }
            } else {
                let tallies = analysis::histogram_tallies(&run.reports, &object);
                if tallies.is_empty() {
                    ui.weak(if run.form.histograms {
                        "Este objeto não tem observações de permanência."
                    } else {
                        "A rodada foi feita sem histogramas."
                    });
                    if !run.form.histograms {
                        offer_rerun(ui, res, model, can_run, false);
                    }
                    return;
                }
                if !tallies.contains(&res.tally) {
                    res.tally = tallies[0].clone();
                }
                ui.horizontal(|ui| {
                    ui.label("Medida:");
                    for t in &tallies {
                        let label = if t == "staytime" {
                            "permanência"
                        } else {
                            t.as_str()
                        };
                        if ui.selectable_label(res.tally == *t, label).clicked() {
                            res.tally = t.clone();
                        }
                    }
                });
                if let Some(h) = analysis::pooled_histogram(&run.reports, &object, &res.tally) {
                    histogram_chart(ui, &h);
                }
            }
        }
    }
}

/// "Turn it on and run again", for charts the last run had no data for.
fn offer_rerun(ui: &mut egui::Ui, res: &mut Results, model: &Model, can_run: bool, series: bool) {
    let label = if series {
        "Ativar séries temporais e rodar de novo"
    } else {
        "Ativar histogramas e rodar de novo"
    };
    if ui.add_enabled(can_run, egui::Button::new(label)).clicked() {
        if let Some(f) = res.form.as_mut() {
            if series {
                f.series = true;
            } else {
                f.histograms = true;
            }
        }
        res.start_run(model);
    }
}

fn color32(c: [f32; 3]) -> Color32 {
    let b = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgb(b(c[0]), b(c[1]), b(c[2]))
}

/// Horizontal bars with interval whiskers; `scale` multiplies every value (100 for %).
fn whisker_chart(ui: &mut egui::Ui, id: &str, bars: &[WhiskerBar], scale: f64, unit: &str) {
    let n = bars.len();
    let names: Vec<String> = bars.iter().map(|b| b.label.clone()).collect();
    // The first bar on top.
    let y = |i: usize| (n - 1 - i) as f64;
    let chart = BarChart::new(
        "média",
        bars.iter()
            .enumerate()
            .map(|(i, b)| {
                Bar::new(y(i), b.mean * scale)
                    .width(0.6)
                    .name(&b.label)
                    .fill(if b.hot { HOT } else { BAR })
            })
            .collect(),
    )
    .horizontal()
    .element_formatter(Box::new(move |bar, _| {
        format!("{}: {:.3}", bar.name, bar.value)
    }));
    let unit = unit.to_string();
    Plot::new(id.to_string())
        .height((40.0 + 34.0 * n as f32).min(520.0))
        .allow_scroll(false)
        .allow_zoom(false)
        .allow_drag(false)
        .y_axis_formatter(move |mark, _| {
            let k = mark.value.round();
            if (mark.value - k).abs() < 1e-6 && k >= 0.0 && (k as usize) < n {
                names[n - 1 - k as usize].clone()
            } else {
                String::new()
            }
        })
        .x_axis_formatter(move |mark, _| format!("{}{unit}", mark.value))
        .y_axis_min_width(90.0)
        .include_x(0.0)
        .show(ui, |plot| {
            plot.bar_chart(chart);
            for (i, b) in bars.iter().enumerate() {
                if b.hi > b.lo {
                    let (lo, hi, yy) = (b.lo * scale, b.hi * scale, y(i));
                    let stroke = Color32::from_gray(230);
                    plot.line(
                        Line::new("IC 95%", PlotPoints::new(vec![[lo, yy], [hi, yy]]))
                            .color(stroke)
                            .width(1.5),
                    );
                    for x in [lo, hi] {
                        plot.line(
                            Line::new(
                                "IC 95%",
                                PlotPoints::new(vec![[x, yy - 0.15], [x, yy + 0.15]]),
                            )
                            .color(stroke)
                            .width(1.5),
                        );
                    }
                }
            }
        });
}

fn states_chart(ui: &mut egui::Ui, rows: &[(String, Vec<logic::StateSegment>)]) {
    if rows.is_empty() {
        ui.weak("Sem estados registrados.");
        return;
    }
    let n = rows.len();
    let names: Vec<String> = rows.iter().map(|(n, _)| n.clone()).collect();
    let mut by_state: Vec<(String, Vec<Bar>, [f32; 3])> = Vec::new();
    for (i, (_, segs)) in rows.iter().enumerate() {
        let y = (n - 1 - i) as f64;
        for s in segs {
            let bar = Bar::new(y, s.width * 100.0)
                .base_offset(s.start * 100.0)
                .width(0.6)
                .name(&names[i])
                .fill(color32(s.color))
                .stroke(egui::Stroke::new(1.0, Color32::from_gray(20)));
            match by_state.iter_mut().find(|(k, _, _)| *k == s.state) {
                Some((_, bars, _)) => bars.push(bar),
                None => by_state.push((s.state.clone(), vec![bar], s.color)),
            }
        }
    }
    Plot::new("graf_estados")
        .height((60.0 + 28.0 * n as f32).min(560.0))
        .legend(Legend::default())
        .allow_scroll(false)
        .allow_zoom(false)
        .allow_drag(false)
        .include_x(0.0)
        .include_x(100.0)
        .y_axis_min_width(90.0)
        .y_axis_formatter(move |mark, _| {
            let k = mark.value.round();
            if (mark.value - k).abs() < 1e-6 && k >= 0.0 && (k as usize) < n {
                names[n - 1 - k as usize].clone()
            } else {
                String::new()
            }
        })
        .x_axis_formatter(|mark, _| format!("{}%", mark.value))
        .show(ui, |plot| {
            for (state, bars, color) in by_state {
                let label = logic::state_label(&state);
                let shown = label.clone();
                plot.bar_chart(
                    BarChart::new(label, bars)
                        .horizontal()
                        .color(color32(color))
                        .element_formatter(Box::new(move |bar, _| {
                            format!("{}, {shown}: {:.1}%", bar.name, bar.value)
                        })),
                );
            }
        });
}

fn series_chart(ui: &mut egui::Ui, reports: &[RunReport], object: &str, sampled: bool) {
    let series = analysis::content_series(reports, object);
    if series.mean.is_empty() {
        ui.weak(if sampled {
            "Este objeto não tem pontos amostrados."
        } else {
            "A rodada foi feita sem séries temporais (run.sample_interval desligado)."
        });
        return;
    }
    ui.label(format!(
        "Conteúdo de {object} ao longo do tempo: média das replicações (linha forte) e cada replicação."
    ));
    Plot::new("graf_serie")
        .height(360.0)
        .legend(Legend::default())
        .x_axis_label("tempo")
        .y_axis_label("conteúdo")
        .show(ui, |plot| {
            for s in series.replications.iter().take(20) {
                plot.line(
                    Line::new("replicações", PlotPoints::new(s.clone()))
                        .color(Color32::from_rgba_unmultiplied(150, 160, 175, 90))
                        .width(1.0),
                );
            }
            plot.line(
                Line::new("média", PlotPoints::new(series.mean.clone()))
                    .color(BAR)
                    .width(2.5),
            );
        });
}

fn histogram_chart(ui: &mut egui::Ui, h: &utsim_core::Histogram) {
    let bars = logic::histogram_bars(h)
        .into_iter()
        .map(|(c, w, n)| Bar::new(c, n).width(w * 0.95).fill(BAR))
        .collect();
    let total: u64 = h.counts.iter().sum();
    ui.label(format!(
        "{total} observações de todas as replicações, de {:.3} a {:.3}.",
        h.min, h.max
    ));
    Plot::new("graf_histograma")
        .height(340.0)
        .x_axis_label("valor")
        .y_axis_label("observações")
        .show(ui, |plot| {
            plot.bar_chart(BarChart::new("observações", bars))
        });
}

/// Parameter picker: a combo of the patchable keys (with labels) plus "Adicionar".
fn parameter_picker(
    ui: &mut egui::Ui,
    id: &str,
    model: &Model,
    pick: &mut String,
) -> Option<String> {
    let params = logic::parameters(model);
    let mut add = None;
    ui.horizontal(|ui| {
        ui.label("Parâmetro:");
        let selected = params
            .iter()
            .find(|p| p.key == *pick)
            .map_or_else(|| pick.clone(), |p| p.label.clone());
        egui::ComboBox::from_id_salt(id)
            .width(280.0)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for p in &params {
                    ui.selectable_value(pick, p.key.clone(), &p.label)
                        .on_hover_text(format!("{} (atual: {})", p.key, p.current));
                }
            });
        ui.label("ou chave:");
        ui.add(
            egui::TextEdit::singleline(pick)
                .hint_text("objeto.campo")
                .desired_width(160.0),
        );
        if ui.button("Adicionar parâmetro").clicked() && !pick.trim().is_empty() {
            add = Some(pick.trim().to_string());
        }
    });
    add
}

fn scenarios_tab(
    ui: &mut egui::Ui,
    res: &mut Results,
    model: &Model,
    path: Option<&Path>,
    can_run: bool,
) {
    ui.label(
        "Cada linha é um cenário; cada coluna, um parâmetro. Célula vazia mantém o valor do \
         modelo. Valores em texto RON, como no campo set dos experimentos.",
    );
    ui.horizontal(|ui| {
        ui.label("Experimento:");
        ui.text_edit_singleline(&mut res.scenarios.name);
    });
    if let Some(key) = parameter_picker(ui, "cen_param", model, &mut res.pick) {
        res.scenarios.add_key(&key);
    }
    let form = &mut res.scenarios;
    let mut remove_key = None;
    let mut remove_row = None;
    // Rows can be removed while more than one is left.
    let removable = form.rows.len() > 1;
    egui::Grid::new("cen_grade")
        .striped(true)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            ui.strong("Cenário");
            for (k, key) in form.keys.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.strong(key);
                    if ui
                        .small_button("x")
                        .on_hover_text("Remover parâmetro")
                        .clicked()
                    {
                        remove_key = Some(k);
                    }
                });
            }
            ui.end_row();
            for (r, row) in form.rows.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut row.name).desired_width(130.0));
                    if removable
                        && ui
                            .small_button("x")
                            .on_hover_text("Remover cenário")
                            .clicked()
                    {
                        remove_row = Some(r);
                    }
                });
                row.values.resize(form.keys.len(), String::new());
                for (k, key) in form.keys.iter().enumerate() {
                    let current = logic::current_value(model, key).unwrap_or_default();
                    ui.add(
                        egui::TextEdit::singleline(&mut row.values[k])
                            .hint_text(current)
                            .desired_width(130.0),
                    );
                }
                ui.end_row();
            }
        });
    if let Some(k) = remove_key {
        form.remove_key(k);
    }
    if let Some(r) = remove_row {
        form.rows.remove(r);
    }
    ui.horizontal(|ui| {
        if ui.button("Adicionar cenário").clicked() {
            form.add_row();
        }
        ui.label("Métricas (separadas por vírgula; vazio: todas):");
        let mut text = form.measures.join(", ");
        if ui
            .add(egui::TextEdit::singleline(&mut text).desired_width(300.0))
            .changed()
        {
            form.measures = text
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
        }
    });
    let reps = res
        .form
        .as_ref()
        .map_or(model.run.replications, |f| f.replications);
    ui.horizontal(|ui| {
        if ui
            .add_enabled(can_run, egui::Button::new("Rodar cenários"))
            .on_hover_text(format!(
                "{reps} replicação(ões) por cenário (aba Resumo), mesmas sementes em todos"
            ))
            .clicked()
        {
            res.start_scenarios(model);
        }
    });
    experiment_files(ui, res, path);
    let Some(exp) = &res.experiment else {
        return;
    };
    ui.separator();
    ui.label(format!(
        "{}: média ± IC de 95%. Δ é a diferença pareada (mesmas sementes) para '{}'; ≈ indica \
         diferença não significativa, em laranja as significativas.",
        exp.name,
        exp.scenarios.first().map_or("", |s| s.name.as_str())
    ));
    ui.horizontal(|ui| {
        ui.label("Filtrar métricas:");
        ui.add(egui::TextEdit::singleline(&mut res.metric_filter).desired_width(200.0));
    });
    let rows = logic::comparison_rows(exp);
    // Wide tables (many scenarios) scroll sideways instead of being cut off (drag the
    // bar, or Shift + wheel).
    egui::ScrollArea::horizontal()
        .id_salt("cen_comparacao_rolagem")
        .auto_shrink([false, true])
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
            egui::Grid::new("cen_comparacao")
                .striped(true)
                .spacing([12.0, 3.0])
                .show(ui, |ui| {
                    ui.strong("Métrica");
                    for (i, s) in exp.scenarios.iter().enumerate() {
                        ui.strong(&s.name);
                        if i > 0 {
                            ui.strong(format!("Δ vs {}", exp.scenarios[0].name));
                        }
                    }
                    ui.end_row();
                    for row in rows.iter().filter(|r| {
                        res.metric_filter.is_empty() || r.metric.contains(res.metric_filter.trim())
                    }) {
                        if ui
                            .add(
                                egui::Button::new(RichText::new(&row.metric).monospace())
                                    .frame(false),
                            )
                            .on_hover_text("Mostrar no gráfico")
                            .clicked()
                        {
                            res.bar_metric = row.metric.clone();
                        }
                        for (i, c) in row.cells.iter().enumerate() {
                            ui.label(c);
                            if i > 0 {
                                match &row.deltas[i] {
                                    Some((t, true)) => ui.label(RichText::new(t).color(HOT)),
                                    Some((t, false)) => ui.label(t),
                                    None => ui.label("-"),
                                };
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    ui.separator();
    let names = exp.metric_names();
    ui.horizontal(|ui| {
        ui.label("Gráfico da métrica:");
        egui::ComboBox::from_id_salt("cen_metrica")
            .width(320.0)
            .selected_text(&res.bar_metric)
            .show_ui(ui, |ui| {
                for m in &names {
                    ui.selectable_value(&mut res.bar_metric, m.clone(), m);
                }
            });
    });
    let bars = logic::scenario_bars(exp, &res.bar_metric);
    if !bars.is_empty() {
        whisker_chart(ui, "graf_cenarios", &bars, 1.0, "");
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn experiment_files(ui: &mut egui::Ui, res: &mut Results, path: Option<&Path>) {
    ui.horizontal(|ui| {
        ui.label("Arquivo de experimento:");
        ui.add(egui::TextEdit::singleline(&mut res.experiment_path).desired_width(380.0));
        if ui.button("Abrir experimento").clicked() {
            match utsim_model::experiment::Experiment::load(res.experiment_path.trim()) {
                Ok(exp) => {
                    res.scenarios =
                        ScenarioForm::from_scenarios(&exp.name, &exp.scenarios, &exp.measures);
                    if let Some(r) = exp.replications
                        && let Some(f) = res.form.as_mut()
                    {
                        f.replications = r;
                    }
                    let same = path.is_some_and(|p| same_file(p, &exp.model_path()));
                    res.status = if same {
                        format!("Experimento '{}' aberto", exp.name)
                    } else {
                        format!(
                            "Experimento '{}' aberto; os cenários valem para o modelo aberto \
                             (o arquivo aponta para {})",
                            exp.name,
                            exp.model_path().display()
                        )
                    };
                    res.errors.clear();
                }
                Err(e) => res.errors = vec![format!("não foi possível abrir: {e}")],
            }
        }
        if ui.button("Salvar experimento").clicked() {
            save_experiment(res, path);
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn experiment_files(_ui: &mut egui::Ui, _res: &mut Results, _path: Option<&Path>) {}

#[cfg(not(target_arch = "wasm32"))]
fn same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(x), Ok(y)) => x == y,
        _ => a == b,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_experiment(res: &mut Results, path: Option<&Path>) {
    let target = PathBuf::from(res.experiment_path.trim());
    let Some(model_path) = path else {
        res.errors =
            vec!["salve o modelo antes: o experimento aponta para o arquivo do modelo".into()];
        return;
    };
    // The model path is written relative to the experiment file when they share a folder.
    let dir = target.parent().map(Path::to_path_buf).unwrap_or_default();
    let model = match (model_path.parent(), model_path.file_name()) {
        (Some(p), Some(name)) if same_dir(p, &dir) => name.to_string_lossy().to_string(),
        _ => model_path.display().to_string(),
    };
    let scenarios: Vec<_> = res
        .scenarios
        .rows
        .iter()
        .map(|row| utsim_model::experiment::Scenario {
            name: row.name.trim().to_string(),
            set: res
                .scenarios
                .keys
                .iter()
                .zip(&row.values)
                .filter(|(_, v)| !v.trim().is_empty())
                .map(|(k, v)| (k.clone(), v.trim().to_string()))
                .collect(),
        })
        .collect();
    let exp = utsim_model::experiment::Experiment::new(
        res.scenarios.name.clone(),
        model,
        res.form.as_ref().map(|f| f.replications),
        scenarios,
        res.scenarios.measures.clone(),
    );
    match std::fs::write(&target, exp.to_ron()) {
        Ok(()) => {
            res.status = format!("Experimento salvo em {}", target.display());
            res.exported.push(target.display().to_string());
            res.errors.clear();
        }
        Err(e) => res.errors = vec![format!("não foi possível gravar {}: {e}", target.display())],
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn same_dir(a: &Path, b: &Path) -> bool {
    let norm = |p: &Path| {
        if p.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            p.to_path_buf()
        }
    };
    same_file(&norm(a), &norm(b))
}

fn optimize_tab(
    ui: &mut egui::Ui,
    res: &mut Results,
    model: &Model,
    path: Option<&Path>,
    can_run: bool,
    requests: &mut Vec<Request>,
) {
    ui.label(
        "Variáveis de decisão (mesmas chaves dos cenários), objetivo e restrições. Todos os \
         pontos usam as mesmas sementes; o melhor é comparado com os outros por teste pareado.",
    );
    if let Some(key) = parameter_picker(ui, "opt_param", model, &mut res.opt_pick)
        && !res.opt.variables.iter().any(|v| v.key == key)
    {
        let mut v = VariableRow::new(&key);
        if let Some(n) = logic::current_value(model, &key).and_then(|t| t.parse::<f64>().ok()) {
            v.min = n.max(1.0);
            v.max = v.min + 3.0;
        }
        res.opt.variables.push(v);
    }
    let form = &mut res.opt;
    let mut remove = None;
    egui::Grid::new("opt_variaveis")
        .striped(true)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            for h in [
                "Variável",
                "Domínio",
                "Mínimo",
                "Máximo",
                "Passo / valores",
                "Custo por unidade",
                "",
            ] {
                ui.strong(h);
            }
            ui.end_row();
            for (i, v) in form.variables.iter_mut().enumerate() {
                ui.monospace(&v.key);
                egui::ComboBox::from_id_salt(("opt_dominio", i))
                    .selected_text(match v.kind {
                        DomainKind::Integer => "inteiro",
                        DomainKind::Continuous => "contínuo",
                        DomainKind::Choice => "lista",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut v.kind, DomainKind::Integer, "inteiro");
                        ui.selectable_value(&mut v.kind, DomainKind::Continuous, "contínuo");
                        ui.selectable_value(&mut v.kind, DomainKind::Choice, "lista");
                    });
                if v.kind == DomainKind::Choice {
                    ui.label("-");
                    ui.label("-");
                    ui.add(
                        egui::TextEdit::singleline(&mut v.choices)
                            .hint_text("Fifo; Lifo")
                            .desired_width(200.0),
                    );
                    ui.label("-");
                } else {
                    ui.add(egui::DragValue::new(&mut v.min).speed(0.5));
                    ui.add(egui::DragValue::new(&mut v.max).speed(0.5));
                    if v.kind == DomainKind::Integer {
                        ui.add(
                            egui::DragValue::new(&mut v.step)
                                .range(1.0..=1e9)
                                .speed(0.2),
                        );
                    } else {
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut v.steps).range(1..=100));
                            ui.add(
                                egui::TextEdit::singleline(&mut v.template)
                                    .hint_text("Exp(mean: {x})")
                                    .desired_width(130.0),
                            );
                        });
                    }
                    ui.add(egui::DragValue::new(&mut v.cost).speed(0.5));
                }
                if ui.small_button("x").clicked() {
                    remove = Some(i);
                }
                ui.end_row();
            }
        });
    if let Some(i) = remove {
        form.variables.remove(i);
    }
    let metrics: Vec<String> = res
        .run
        .as_ref()
        .map(|r| r.metrics.keys().cloned().collect())
        .unwrap_or_default();
    ui.horizontal(|ui| {
        ui.label("Objetivo:");
        ui.radio_value(&mut form.maximize, false, "minimizar");
        ui.radio_value(&mut form.maximize, true, "maximizar");
        ui.add(egui::DragValue::new(&mut form.weight).speed(0.1))
            .on_hover_text("peso da métrica");
        ui.label("x");
        metric_field(
            ui,
            egui::Id::new("opt_objetivo"),
            &mut form.objective_metric,
            &metrics,
        );
        ui.label("+ custos das variáveis");
    });
    ui.strong("Restrições (sobre a média da métrica)");
    let mut remove = None;
    for (i, c) in form.constraints.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut c.min)
                    .hint_text("mínimo")
                    .desired_width(70.0),
            );
            ui.label("≤");
            metric_field(ui, egui::Id::new(("opt_restr", i)), &mut c.metric, &metrics);
            ui.label("≤");
            ui.add(
                egui::TextEdit::singleline(&mut c.max)
                    .hint_text("máximo")
                    .desired_width(70.0),
            );
            if ui.small_button("x").clicked() {
                remove = Some(i);
            }
        });
    }
    if let Some(i) = remove {
        form.constraints.remove(i);
    }
    if ui.button("Adicionar restrição").clicked() {
        form.constraints.push(logic::ConstraintRow::default());
    }
    ui.horizontal(|ui| {
        ui.label("Método:");
        ui.radio_value(&mut form.method, Method::Grid, "grade");
        ui.radio_value(&mut form.method, Method::Annealing, "recozimento simulado");
        ui.radio_value(&mut form.method, Method::Genetic, "genético");
        match form.method {
            Method::Grid => {}
            Method::Annealing => {
                ui.label("iterações");
                ui.add(egui::DragValue::new(&mut form.iterations).range(1..=10_000));
            }
            Method::Genetic => {
                ui.label("população");
                ui.add(egui::DragValue::new(&mut form.population).range(2..=500));
                ui.label("gerações");
                ui.add(egui::DragValue::new(&mut form.generations).range(0..=500));
            }
        }
    });
    ui.horizontal(|ui| {
        ui.label("Replicações por ponto");
        ui.add(egui::DragValue::new(&mut form.replications).range(2..=1000));
        ui.label("Orçamento (pontos)");
        ui.add(egui::DragValue::new(&mut form.budget).range(1..=100_000));
        ui.label("Semente da busca");
        ui.add(egui::DragValue::new(&mut form.seed));
        ui.checkbox(&mut form.refine, "Refinar os");
        ui.add_enabled(
            form.refine,
            egui::DragValue::new(&mut form.refine_top).range(1..=100),
        );
        ui.label("melhores com");
        ui.add_enabled(
            form.refine,
            egui::DragValue::new(&mut form.refine_replications).range(2..=1000),
        );
        ui.label("replicações");
    });
    let model_file = path
        .and_then(|p| p.file_name())
        .map_or("modelo.ron".to_string(), |n| {
            n.to_string_lossy().to_string()
        });
    if ui
        .add_enabled(can_run, egui::Button::new("Otimizar"))
        .clicked()
    {
        res.start_optimize(model, &model_file);
    }
    let Some((problem, result)) = &res.optimized else {
        return;
    };
    ui.separator();
    match result.best_evaluation() {
        Some(best) => {
            ui.horizontal(|ui| {
                ui.label("Melhor ponto:");
                ui.label(
                    RichText::new(logic::point_text(&best.point))
                        .color(HOT)
                        .strong(),
                );
                ui.label(format!("objetivo {}", logic::cell(&best.objective)));
                if ui
                    .button("Aplicar ao modelo")
                    .on_hover_text("Grava estes valores no modelo (Ctrl+Z desfaz)")
                    .clicked()
                {
                    requests.push(Request::Exec(logic::apply_point(&best.point)));
                }
            });
            for (i, (metric, s, ok)) in best.constraints.iter().enumerate() {
                let c = &problem.constraints[i];
                let bound = match (c.min, c.max) {
                    (Some(a), Some(b)) => format!("{a} ≤ {metric} ≤ {b}"),
                    (Some(a), None) => format!("{metric} ≥ {a}"),
                    (None, Some(b)) => format!("{metric} ≤ {b}"),
                    (None, None) => metric.clone(),
                };
                // Words, not check marks: egui's fonts have no glyph for them.
                ui.horizontal_wrapped(|ui| {
                    let (word, color) = if *ok {
                        ("atende", Color32::from_rgb(110, 200, 120))
                    } else {
                        ("não atende", RED)
                    };
                    ui.label(RichText::new(word).color(color).strong());
                    ui.label(format!("{bound} (média observada: {})", logic::cell(s)));
                });
            }
        }
        None => {
            ui.colored_label(RED, "Nenhum ponto avaliado atende às restrições.");
        }
    }
    for line in &result.explanation {
        ui.label(line);
    }
    ui.strong("Convergência: melhor objetivo viável por número de pontos avaliados");
    let curve = logic::convergence(result);
    let points: Vec<[f64; 2]> = result
        .evaluations
        .iter()
        .enumerate()
        .map(|(i, e)| [(i + 1) as f64, e.objective.mean])
        .collect();
    Plot::new("graf_convergencia")
        .height(240.0)
        .legend(Legend::default())
        .x_axis_label("pontos avaliados")
        .y_axis_label("objetivo")
        .show(ui, |plot| {
            plot.points(
                egui_plot::Points::new("pontos", PlotPoints::new(points))
                    .radius(2.5)
                    .color(Color32::from_gray(160)),
            );
            plot.line(
                Line::new("melhor até então", PlotPoints::new(curve))
                    .color(HOT)
                    .width(2.0),
            );
        });
    ui.strong("Comparação pareada com o melhor (≈: sem diferença significativa a 95%)");
    let rows = logic::opt_rows(problem, result);
    egui::Grid::new("opt_pontos")
        .striped(true)
        .spacing([12.0, 3.0])
        .show(ui, |ui| {
            for h in ["#", "Ponto", "Objetivo", "Viável", "Δ vs melhor"] {
                ui.strong(h);
            }
            ui.end_row();
            for (i, r) in rows.iter().take(30).enumerate() {
                ui.label((i + 1).to_string());
                let point = RichText::new(&r.point).monospace();
                ui.label(if r.is_best { point.color(HOT) } else { point });
                ui.label(&r.objective);
                ui.label(if r.feasible {
                    RichText::new("sim").color(Color32::from_rgb(110, 200, 120))
                } else {
                    RichText::new("não").color(RED)
                });
                let delta = if r.is_best {
                    "melhor".to_string()
                } else if r.equivalent {
                    format!("{} equivalente", r.delta)
                } else {
                    r.delta.clone()
                };
                ui.label(delta);
                ui.end_row();
            }
        });
}

/// A metric name: free text, plus a picker of the metrics of the latest run.
fn metric_field(ui: &mut egui::Ui, id: egui::Id, value: &mut String, metrics: &[String]) {
    ui.add(
        egui::TextEdit::singleline(value)
            .hint_text("objeto.métrica")
            .desired_width(220.0),
    );
    if !metrics.is_empty() {
        egui::ComboBox::from_id_salt(id)
            .width(24.0)
            .selected_text("")
            .show_ui(ui, |ui| {
                for m in metrics {
                    ui.selectable_value(value, m.clone(), m);
                }
            });
    }
}
