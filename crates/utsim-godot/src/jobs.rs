//! Long runs on a thread of the bridge (G3), so the interface keeps drawing: the
//! replications, more replications appended to a run, scenarios, optimisation, the runs
//! of the output analyses and the PERT/CPM. No Godot types: the Godot class `UtsimJob`
//! (`lib.rs`) only polls [`Job::progress`] every frame, calls [`Job::cancel`] and takes
//! the [`Outcome`].
//!
//! Progress counts finished replications (or points, for the optimisation); a task that
//! cannot count (the PERT observed in the simulation, the warm-up of one measure) reports
//! `total = 0`, which the interface shows as "running". Cancelling stops at the next
//! replication; the outcome is then the error [`CANCELLED`].

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use serde_json::Value;
use utsim_core::{RunReport, TraceRecord};
use utsim_model::analysis::{self, WarmupSeries};
use utsim_model::dados::DataInput;
use utsim_model::experiment::{Scenario, run_scenarios};
use utsim_model::{Model, ModelError};
use utsim_opt::OptProblem;
use utsim_stats::DEFAULT_LEVEL;
use utsim_stats::little::DEFAULT_TOLERANCE;

use crate::dados::{self, FitCache};
use crate::pert_view::{self, PertOutput, PertRequest};
use crate::results::{self, RunOutput};

/// The error text of a cancelled job.
pub const CANCELLED: &str = "cancelado";

/// What a job does.
pub enum Task {
    /// `run.replications` replications (`Model::run_replications_with`).
    Run(Model),
    /// `extra` replications after the ones of `base` (their own seeds), appended.
    More {
        base: Box<RunOutput>,
        extra: u32,
    },
    Scenarios {
        model: Model,
        scenarios: Vec<Scenario>,
        measures: Vec<String>,
    },
    Optimize {
        model: Model,
        problem: Box<OptProblem>,
    },
    /// Replications from time 0 without warm-up, for Welch and MSER-5.
    Warmup {
        model: Model,
        series: WarmupSeries,
        window: usize,
    },
    Little(Model),
    ShiftingBottleneck(Model),
    Pert {
        model: Model,
        request: PertRequest,
    },
    /// The automatic fit of the real data of one field (the view the panel draws).
    DataFit {
        input: Box<DataInput>,
        base: Option<PathBuf>,
        cache: FitCache,
    },
    /// The HTML reports of the real data of a model, written into `dir`.
    DataReport {
        model: Box<Model>,
        base: Option<PathBuf>,
        dir: PathBuf,
    },
}

impl Task {
    pub fn label(&self) -> &'static str {
        match self {
            Task::Run(_) => "Rodando replicações",
            Task::More { .. } => "Rodando mais replicações",
            Task::Scenarios { .. } => "Comparando cenários",
            Task::Optimize { .. } => "Otimizando",
            Task::Warmup { .. } => "Analisando o aquecimento",
            Task::Little(_) => "Conferindo a lei de Little",
            Task::ShiftingBottleneck(_) => "Procurando o gargalo móvel",
            Task::Pert { .. } => "Calculando o PERT/CPM",
            Task::DataFit { .. } => "Ajustando a distribuição aos dados",
            Task::DataReport { .. } => "Gravando o relatório dos dados",
        }
    }

    /// Replications (or points) it will run, 0 when unknown.
    fn total(&self) -> usize {
        match self {
            Task::Run(m) | Task::Little(m) | Task::ShiftingBottleneck(m) => {
                m.run.replications as usize
            }
            Task::More { extra, .. } => *extra as usize,
            Task::Scenarios {
                model, scenarios, ..
            } => model.run.replications as usize * scenarios.len(),
            Task::Optimize { problem, .. } => utsim_opt::planned_evaluations(problem),
            Task::Warmup { model, series, .. } => match series {
                WarmupSeries::Wip => model.run.replications as usize,
                WarmupSeries::Measure { .. } => 0,
            },
            Task::Pert { .. } | Task::DataFit { .. } | Task::DataReport { .. } => 0,
        }
    }
}

/// What a finished job produced.
pub enum Outcome {
    /// Replications (also after [`Task::More`]).
    Run(Box<RunOutput>),
    /// Scenarios, optimisation and the output analyses, as JSON for the panel.
    Data(Value),
    Pert(Box<PertOutput>),
}

type Shared = Arc<Mutex<Option<Result<Outcome, String>>>>;

pub struct Job {
    pub what: &'static str,
    done: Arc<AtomicUsize>,
    total: Arc<AtomicUsize>,
    cancel: Arc<AtomicBool>,
    result: Shared,
    handle: Option<JoinHandle<()>>,
}

/// Counts finished pieces and answers whether to go on.
struct Progress {
    done: Arc<AtomicUsize>,
    total: Arc<AtomicUsize>,
    cancel: Arc<AtomicBool>,
}

impl Progress {
    fn tick(&self, done: usize, total: usize) -> bool {
        self.done.store(done, Ordering::Relaxed);
        if total > 0 {
            self.total.store(total, Ordering::Relaxed);
        }
        !self.cancel.load(Ordering::Relaxed)
    }

    fn cancelled(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }
}

fn text(e: ModelError) -> String {
    match e {
        ModelError::Cancelled => CANCELLED.to_string(),
        other => other.to_string(),
    }
}

impl Job {
    /// Starts `task` on a thread of its own.
    pub fn start(task: Task) -> Job {
        let what = task.label();
        let done = Arc::new(AtomicUsize::new(0));
        let total = Arc::new(AtomicUsize::new(task.total()));
        let cancel = Arc::new(AtomicBool::new(false));
        let result: Shared = Arc::new(Mutex::new(None));
        let p = Progress {
            done: done.clone(),
            total: total.clone(),
            cancel: cancel.clone(),
        };
        let slot = result.clone();
        let handle = std::thread::spawn(move || {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| execute(task, &p)))
                .unwrap_or_else(|_| Err("erro interno durante a execução".to_string()));
            let r = if p.cancelled() && r.is_ok() {
                Err(CANCELLED.to_string())
            } else {
                r
            };
            if let Ok(mut s) = slot.lock() {
                *s = Some(r);
            }
        });
        Job {
            what,
            done,
            total,
            cancel,
            result,
            handle: Some(handle),
        }
    }

    /// `(done, total)`; `total` 0 when the job cannot count.
    pub fn progress(&self) -> (usize, usize) {
        (
            self.done.load(Ordering::Relaxed),
            self.total.load(Ordering::Relaxed),
        )
    }

    pub fn is_finished(&self) -> bool {
        self.result.lock().map(|r| r.is_some()).unwrap_or(true)
    }

    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Relaxed);
    }

    /// The outcome, once (later calls give `None`).
    pub fn take(&mut self) -> Option<Result<Outcome, String>> {
        let r = self.result.lock().ok()?.take()?;
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
        Some(r)
    }

    /// Blocks until the job ends and returns the outcome (tests, scripts).
    pub fn wait(&mut self) -> Result<Outcome, String> {
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
        self.result
            .lock()
            .ok()
            .and_then(|mut r| r.take())
            .unwrap_or_else(|| Err("a execução terminou sem resultado".into()))
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        // A job nobody waits for anymore stops at the next replication.
        self.cancel();
    }
}

/// Runs `f(k)` for `k` in `0..count` on the available cores (in index order in the
/// result), calling `progress(done)` after each; stops when it returns `false`.
fn par_map<T: Send>(
    count: u32,
    f: &(dyn Fn(u32) -> Result<T, ModelError> + Sync),
    progress: &(dyn Fn(usize) -> bool + Sync),
) -> Result<Vec<T>, ModelError> {
    let next = AtomicU32::new(0);
    let done = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);
    let slots: Vec<Mutex<Option<Result<T, ModelError>>>> =
        (0..count).map(|_| Mutex::new(None)).collect();
    let workers = std::thread::available_parallelism()
        .map_or(1, |n| n.get())
        .min(count.max(1) as usize);
    std::thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| {
                loop {
                    if stop.load(Ordering::Relaxed) {
                        break;
                    }
                    let k = next.fetch_add(1, Ordering::Relaxed);
                    if k >= count {
                        break;
                    }
                    let r = f(k);
                    let failed = r.is_err();
                    if let Ok(mut slot) = slots[k as usize].lock() {
                        *slot = Some(r);
                    }
                    let n = done.fetch_add(1, Ordering::Relaxed) + 1;
                    if failed || !progress(n) {
                        stop.store(true, Ordering::Relaxed);
                    }
                }
            });
        }
    });
    let mut out = Vec::with_capacity(count as usize);
    for slot in slots {
        match slot.into_inner().ok().flatten() {
            Some(Ok(v)) => out.push(v),
            Some(Err(e)) => return Err(e),
            None => return Err(ModelError::Cancelled),
        }
    }
    Ok(out)
}

/// Replications `from..from + count` of `model` (each with its own seed, so they extend
/// a run of `from` replications: the first `from` are not repeated).
pub fn run_range(
    model: &Model,
    from: u32,
    count: u32,
    progress: &(dyn Fn(usize) -> bool + Sync),
) -> Result<Vec<RunReport>, ModelError> {
    model.validate()?;
    par_map(
        count,
        &|k| {
            let mut sim = model.build(from + k);
            model.drive(&mut sim)
        },
        progress,
    )
}

/// Every replication with its trace (Little with transfers, the shifting bottleneck).
fn run_traced(
    model: &Model,
    progress: &(dyn Fn(usize) -> bool + Sync),
) -> Result<Vec<(RunReport, Vec<TraceRecord>)>, ModelError> {
    model.validate()?;
    par_map(
        model.run.replications,
        &|r| model.replay(r).map(|rp| (rp.report, rp.records)),
        progress,
    )
}

fn execute(task: Task, p: &Progress) -> Result<Outcome, String> {
    match task {
        Task::Run(model) => {
            let total = model.run.replications as usize;
            let reports = model
                .run_replications_with(&|done| p.tick(done as usize, total))
                .map_err(text)?;
            Ok(Outcome::Run(Box::new(RunOutput::new(model, reports))))
        }
        Task::More { base, extra } => {
            let from = base.reports.len() as u32;
            let more = run_range(&base.model, from, extra, &|d| p.tick(d, extra as usize))
                .map_err(text)?;
            Ok(Outcome::Run(Box::new(base.with_more(more))))
        }
        Task::Scenarios {
            mut model,
            scenarios,
            measures,
        } => {
            model.run.sample_interval = None;
            model.run.histogram_bins = None;
            let reps = model.run.replications as usize;
            let total = reps * scenarios.len();
            let list = run_scenarios(&model, &scenarios, &measures, &|i, done| {
                p.tick(i * reps + done as usize, total)
            })
            .map_err(text)?;
            let r = utsim_model::experiment::ExperimentResult {
                name: "cenarios".into(),
                measures,
                scenarios: list,
            };
            Ok(Outcome::Data(results::scenarios(&r)))
        }
        Task::Optimize { model, problem } => {
            let r = utsim_opt::optimize_with(&problem, &model, &mut |d, t| p.tick(d, t))
                .map_err(text)?;
            Ok(Outcome::Data(results::optimized(&r)))
        }
        Task::Warmup {
            model,
            series,
            window,
        } => {
            let m = analysis::warmup_model(&model, &series);
            let total = m.run.replications as usize;
            let reports = match &series {
                WarmupSeries::Wip => m.run_replications_with(&|d| p.tick(d as usize, total)),
                WarmupSeries::Measure { object, measure } => {
                    if m.objects.iter().all(|o| &o.name != object) {
                        return Err(format!("objeto \"{object}\" não existe no modelo"));
                    }
                    m.run_replications_timed(&[(object.clone(), measure.clone())])
                }
            }
            .map_err(text)?;
            let a = analysis::warmup(&reports, &series, window.max(1));
            Ok(Outcome::Data(results::warmup(&a, &series.label())))
        }
        Task::Little(model) => {
            let total = model.run.replications as usize;
            let report = if analysis::has_transfers(&model) {
                let traced = run_traced(&model, &|d| p.tick(d, total)).map_err(text)?;
                let transit: Vec<f64> = traced
                    .iter()
                    .map(|(r, rec)| analysis::transit_mean(rec, r.stats_from, r.end_time))
                    .collect();
                let reports: Vec<RunReport> = traced.into_iter().map(|(r, _)| r).collect();
                analysis::little(&reports, Some(&transit), DEFAULT_LEVEL, DEFAULT_TOLERANCE)
            } else {
                let reports = model
                    .run_replications_with(&|d| p.tick(d as usize, total))
                    .map_err(text)?;
                analysis::little(&reports, None, DEFAULT_LEVEL, DEFAULT_TOLERANCE)
            };
            Ok(Outcome::Data(results::little(&report)))
        }
        Task::ShiftingBottleneck(model) => {
            let total = model.run.replications as usize;
            let traced = run_traced(&model, &|d| p.tick(d, total)).map_err(text)?;
            let rows = analysis::shifting_bottleneck(&traced, DEFAULT_LEVEL);
            Ok(Outcome::Data(results::shifting(&rows)))
        }
        Task::Pert { model, request } => {
            let out = pert_view::run(&model, &request)?;
            Ok(Outcome::Pert(Box::new(out)))
        }
        Task::DataFit { input, base, cache } => {
            let fit = dados::fit_cached(&input, base.as_deref(), &cache)?;
            Ok(Outcome::Data(fit.view.clone()))
        }
        Task::DataReport { model, base, dir } => {
            dados::write_report(&model, base.as_deref(), &dir).map(Outcome::Data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::tests::example;

    fn run_output(job: &mut Job) -> RunOutput {
        match job.wait() {
            Ok(Outcome::Run(r)) => *r,
            Ok(_) => panic!("outro resultado"),
            Err(e) => panic!("{e}"),
        }
    }

    fn data(job: &mut Job) -> Value {
        match job.wait() {
            Ok(Outcome::Data(v)) => v,
            Ok(_) => panic!("outro resultado"),
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn a_job_gives_the_same_replications_as_a_direct_run() {
        let mut m = example("mm1.ron");
        m.run.replications = 4;
        let mut job = Job::start(Task::Run(m.clone()));
        let out = run_output(&mut job);
        assert_eq!(job.progress(), (4, 4));
        let direct = m.run_replications().unwrap();
        assert_eq!(out.reports.len(), 4);
        let (a, b) = (
            utsim_core::summarize(&out.reports),
            utsim_core::summarize(&direct),
        );
        assert_eq!(a.len(), b.len());
        for (k, s) in &a {
            assert_eq!(s.mean, b[k].mean, "{k}");
        }
    }

    #[test]
    fn more_replications_in_a_job_equal_a_longer_run() {
        let mut m = example("mm1.ron");
        m.run.replications = 2;
        let mut job = Job::start(Task::Run(m.clone()));
        let base = run_output(&mut job);
        let mut more = Job::start(Task::More {
            base: Box::new(base),
            extra: 3,
        });
        let merged = run_output(&mut more);
        m.run.replications = 5;
        let direct = m.run_replications().unwrap();
        assert_eq!(merged.reports.len(), 5);
        for (a, b) in merged.reports.iter().zip(&direct) {
            assert_eq!(a.replication, b.replication);
            assert_eq!(a.events, b.events);
        }
    }

    #[test]
    fn a_cancelled_job_ends_with_cancelado() {
        let mut m = example("mm1.ron");
        m.run.replications = 400;
        m.run.stop_time *= 20.0;
        let mut job = Job::start(Task::Run(m));
        job.cancel();
        match job.wait() {
            Err(e) => assert_eq!(e, CANCELLED),
            Ok(_) => panic!("devia cancelar"),
        }
    }

    #[test]
    fn analyses_run_in_jobs() {
        let mut m = example("mm1.ron");
        m.run.replications = 3;
        let w = data(&mut Job::start(Task::Warmup {
            model: m.clone(),
            series: WarmupSeries::Wip,
            window: 5,
        }));
        assert!(w["points"].as_u64().unwrap() > 20, "{w}");
        assert!(!w["welch"].as_array().unwrap().is_empty());
        let l = data(&mut Job::start(Task::Little(m.clone())));
        assert!(!l["objects"].as_array().unwrap().is_empty());
        assert!(l["system"].is_object());
        let s = data(&mut Job::start(Task::ShiftingBottleneck(m.clone())));
        assert_eq!(s[0]["name"], "maquina");
        let sc = data(&mut Job::start(Task::Scenarios {
            model: m.clone(),
            scenarios: results::parse_scenarios("[rapida]\nmaquina.process = Exp(mean: 0.5)")
                .unwrap(),
            measures: vec!["fila.content.mean".into()],
        }));
        assert_eq!(sc["scenarios"].as_array().unwrap().len(), 2);
        let diff = &sc["rows"][0]["differences"][1];
        assert!(diff["mean"].as_f64().unwrap() < 0.0, "{sc}");
    }

    #[test]
    fn optimisation_runs_in_a_job() {
        let mut m = example("servidores.ron");
        m.run.stop_time = 200.0;
        m.run.warmup = 0.0;
        let form = results::OptForm {
            metric: "fila.content.mean".into(),
            maximize: false,
            variables: "maquina.capacity 1 3".into(),
            method: "grade".into(),
            replications: 2,
            budget: 10,
        };
        let v = data(&mut Job::start(Task::Optimize {
            model: m,
            problem: Box::new(form.build().unwrap()),
        }));
        assert_eq!(v["evaluations"].as_array().unwrap().len(), 3);
        assert_eq!(v["best"], "maquina.capacity = 3");
    }
}
