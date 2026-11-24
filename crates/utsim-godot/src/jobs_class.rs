//! Godot classes of the results (G3): `UtsimJob` (a run on a thread of the bridge, polled
//! every frame), `UtsimResults` (the replications of a run and their analysis) and
//! `UtsimPert` (a computed PERT/CPM). Every method is a thin conversion: the numbers come
//! from [`crate::results`], [`crate::jobs`] and [`crate::pert_view`] as JSON values,
//! turned into dictionaries and arrays by [`variant_of_json`].

use std::path::PathBuf;

use godot::prelude::*;
use serde_json::Value;
use utsim_model::analysis::WarmupSeries;

use crate::jobs::{CANCELLED, Job, Outcome, Task};
use crate::pert_view::PertOutput;
use crate::results::RunOutput;
use crate::{put, strings};

/// A JSON value as a Godot value: objects become dictionaries, arrays become arrays,
/// whole numbers `int` and the others `float`.
pub fn variant_of_json(v: &Value) -> Variant {
    match v {
        Value::Null => Variant::nil(),
        Value::Bool(b) => b.to_variant(),
        Value::Number(n) => match n.as_i64() {
            Some(i) => i.to_variant(),
            None => n.as_f64().unwrap_or(f64::NAN).to_variant(),
        },
        Value::String(s) => GString::from(s.as_str()).to_variant(),
        Value::Array(list) => {
            let mut a = VarArray::new();
            for x in list {
                a.push(&variant_of_json(x));
            }
            a.to_variant()
        }
        Value::Object(map) => {
            let mut d = VarDictionary::new();
            for (k, x) in map {
                d.set(&GString::from(k.as_str()).to_variant(), &variant_of_json(x));
            }
            d.to_variant()
        }
    }
}

pub(crate) fn job_object(task: Task) -> Gd<UtsimJob> {
    let kind = match &task {
        Task::Run(_) | Task::More { .. } => "replicacoes",
        Task::Scenarios { .. } => "cenarios",
        Task::Optimize { .. } => "otimizacao",
        Task::Warmup { .. } => "aquecimento",
        Task::Little(_) => "little",
        Task::ShiftingBottleneck(_) => "gargalo_movel",
        Task::Pert { .. } => "pert",
        Task::DataFit { .. } => "ajuste",
        Task::DataReport { .. } => "relatorio_dados",
        Task::Playback { .. } => "reproducao",
    };
    let job = Job::start(task);
    Gd::from_init_fn(|base| UtsimJob {
        job: Some(job),
        outcome: None,
        kind: kind.to_string(),
        base,
    })
}

/// A run on a thread of the bridge: poll [`UtsimJob::is_finished`] every frame, show
/// [`UtsimJob::progress`], and take the outcome with `results()`, `data()` or `pert()`.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimJob {
    job: Option<Job>,
    outcome: Option<Result<Outcome, String>>,
    kind: String,
    base: Base<RefCounted>,
}

impl UtsimJob {
    /// The kind of run, as [`UtsimJob::kind`] names it.
    pub(crate) fn kind_name(&self) -> &str {
        &self.kind
    }

    /// The table of a finished analysis, scenario comparison or optimisation.
    pub(crate) fn data_json(&self) -> Option<&Value> {
        match &self.outcome {
            Some(Ok(Outcome::Data(v))) => Some(v),
            _ => None,
        }
    }

    fn poll(&mut self) {
        if self.outcome.is_some() {
            return;
        }
        if let Some(job) = self.job.as_mut()
            && job.is_finished()
        {
            self.outcome = job.take();
        }
    }
}

#[godot_api]
impl UtsimJob {
    /// `replicacoes`, `cenarios`, `otimizacao`, `aquecimento`, `little`,
    /// `gargalo_movel` or `pert`.
    #[func]
    fn kind(&self) -> GString {
        GString::from(self.kind.as_str())
    }

    /// What is running, in Portuguese ("Rodando replicações").
    #[func]
    pub(crate) fn what(&self) -> GString {
        GString::from(self.job.as_ref().map_or("", |j| j.what))
    }

    /// `{done, total, finished}`; `total` 0 when the job cannot count.
    #[func]
    pub(crate) fn progress(&mut self) -> VarDictionary {
        self.poll();
        let (done, total) = self.job.as_ref().map_or((0, 0), Job::progress);
        let mut d = VarDictionary::new();
        put(&mut d, "done", done as i64);
        put(&mut d, "total", total as i64);
        put(&mut d, "finished", self.outcome.is_some());
        d
    }

    #[func]
    fn is_finished(&mut self) -> bool {
        self.poll();
        self.outcome.is_some()
    }

    /// Stops at the next replication (the outcome is then "cancelado").
    #[func]
    fn cancel(&self) {
        if let Some(j) = &self.job {
            j.cancel();
        }
    }

    /// Blocks until the job ends (scripts and tests; the interface polls instead).
    #[func]
    fn wait(&mut self) -> bool {
        if self.outcome.is_none()
            && let Some(j) = self.job.as_mut()
        {
            self.outcome = Some(j.wait());
        }
        self.ok()
    }

    #[func]
    pub(crate) fn ok(&self) -> bool {
        matches!(self.outcome, Some(Ok(_)))
    }

    #[func]
    pub(crate) fn cancelled(&self) -> bool {
        matches!(&self.outcome, Some(Err(e)) if e == CANCELLED)
    }

    #[func]
    pub(crate) fn error(&self) -> GString {
        match &self.outcome {
            Some(Err(e)) => GString::from(e.as_str()),
            _ => GString::new(),
        }
    }

    /// The replications of a finished run (`null` for other jobs).
    #[func]
    fn results(&self) -> Option<Gd<UtsimResults>> {
        match &self.outcome {
            Some(Ok(Outcome::Run(out))) => Some(results_object((**out).clone())),
            _ => None,
        }
    }

    /// The table of a finished scenario comparison, optimisation or analysis.
    #[func]
    fn data(&self) -> Variant {
        match &self.outcome {
            Some(Ok(Outcome::Data(v))) => variant_of_json(v),
            _ => Variant::nil(),
        }
    }

    /// The playback of a finished `reproducao` job (takes it: later calls give `null`).
    #[func]
    fn playback(&mut self) -> Option<Gd<crate::UtsimPlayback>> {
        match self.outcome.take() {
            Some(Ok(Outcome::Playback(p))) => {
                self.outcome = Some(Ok(Outcome::Data(Value::Null)));
                Some(Gd::from_init_fn(|base| crate::UtsimPlayback {
                    core: Some(*p),
                    base,
                }))
            }
            other => {
                self.outcome = other;
                None
            }
        }
    }

    /// A finished PERT/CPM (takes it: later calls give `null`).
    #[func]
    fn pert(&mut self) -> Option<Gd<UtsimPert>> {
        match self.outcome.take() {
            Some(Ok(Outcome::Pert(p))) => {
                self.outcome = Some(Ok(Outcome::Data(Value::Null)));
                Some(Gd::from_init_fn(|base| UtsimPert {
                    core: Some(p),
                    base,
                }))
            }
            other => {
                self.outcome = other;
                None
            }
        }
    }
}

pub(crate) fn results_object(out: RunOutput) -> Gd<UtsimResults> {
    Gd::from_init_fn(|base| UtsimResults {
        core: Some(out),
        last_error: String::new(),
        base,
    })
}

/// The replications of a run, for the "Resultados" panel.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimResults {
    core: Option<RunOutput>,
    last_error: String,
    base: Base<RefCounted>,
}

impl UtsimResults {
    /// The replications held.
    pub(crate) fn run(&self) -> Option<&RunOutput> {
        self.core.as_ref()
    }

    fn json(&self, f: impl Fn(&RunOutput) -> Value) -> Variant {
        self.core
            .as_ref()
            .map_or_else(Variant::nil, |c| variant_of_json(&f(c)))
    }

    fn start(&mut self, task: impl FnOnce(&RunOutput) -> Task) -> Option<Gd<UtsimJob>> {
        let core = self.core.as_ref()?;
        Some(job_object(task(core)))
    }
}

#[godot_api]
impl UtsimResults {
    #[func]
    fn replications(&self) -> i64 {
        self.core.as_ref().map_or(0, |c| c.reports.len() as i64)
    }

    #[func]
    fn last_error(&self) -> GString {
        GString::from(self.last_error.as_str())
    }

    /// `{model, replications, seed, stop_time, warmup, end_time, sampled, bottleneck,
    /// bottleneck_in_shift}`.
    #[func]
    fn summary(&self) -> Variant {
        self.json(RunOutput::summary)
    }

    /// One row per object (see `results::RunOutput::table`).
    #[func]
    fn table(&self) -> Variant {
        self.json(RunOutput::table)
    }

    #[func]
    fn metrics(&self) -> Variant {
        self.json(RunOutput::metrics)
    }

    /// `which`: `trabalho`, `turno` or `bloqueio`.
    #[func]
    fn utilization(&self, which: GString) -> Variant {
        let w = which.to_string();
        self.json(|c| c.utilization(&w))
    }

    #[func]
    fn states(&self) -> Variant {
        self.json(RunOutput::states)
    }

    #[func]
    fn series(&self, object: GString) -> Variant {
        let o = object.to_string();
        self.json(|c| c.series(&o))
    }

    #[func]
    fn tallies(&self, object: GString) -> PackedStringArray {
        self.core.as_ref().map_or_else(PackedStringArray::new, |c| {
            strings(&c.tallies(&object.to_string()))
        })
    }

    #[func]
    fn histogram(&self, object: GString, tally: GString) -> Variant {
        let (o, t) = (object.to_string(), tally.to_string());
        self.json(|c| c.histogram(&o, &t))
    }

    /// Replications needed for `target` (`"2%"` or an absolute half-width) on `metrics`
    /// (empty: the default ones); `null` on a bad target (see `last_error`).
    #[func]
    fn precision(&mut self, target: GString, metrics: PackedStringArray) -> Variant {
        let Some(c) = &self.core else {
            return Variant::nil();
        };
        let m: Vec<String> = metrics.as_slice().iter().map(|s| s.to_string()).collect();
        match c.precision(&target.to_string(), &m) {
            Ok(v) => {
                self.last_error.clear();
                variant_of_json(&v)
            }
            Err(e) => {
                self.last_error = e;
                Variant::nil()
            }
        }
    }

    /// Writes the CSV files of `utsim run --csv` into `dir`; the names written.
    #[func]
    fn export_csv(&mut self, dir: GString) -> PackedStringArray {
        let Some(c) = &self.core else {
            return PackedStringArray::new();
        };
        match utsim_model::csv::export(&PathBuf::from(dir.to_string()), &c.reports) {
            Ok(files) => strings(&files),
            Err(e) => {
                self.last_error = e.to_string();
                PackedStringArray::new()
            }
        }
    }

    /// Runs `extra` replications more, after the ones already run (their own seeds).
    #[func]
    fn start_more(&mut self, extra: i64) -> Option<Gd<UtsimJob>> {
        let extra = extra.clamp(1, 10_000) as u32;
        self.start(|c| Task::More {
            base: Box::new(c.clone()),
            extra,
        })
    }

    /// Warm-up analysis of `series` (`""` or `"wip"`: the work in process; else
    /// `objeto.medida`) with Welch's window `window`.
    #[func]
    fn start_warmup(&mut self, series: GString, window: i64) -> Option<Gd<UtsimJob>> {
        let s = series.to_string();
        let series = match s.trim() {
            "" | "wip" => WarmupSeries::Wip,
            other => match WarmupSeries::parse(other) {
                Some(x) => x,
                None => {
                    self.last_error = format!("série \"{other}\" inválida: use objeto.medida");
                    return None;
                }
            },
        };
        let window = window.max(1) as usize;
        self.start(|c| Task::Warmup {
            model: c.model.clone(),
            series,
            window,
        })
    }

    #[func]
    fn start_little(&mut self) -> Option<Gd<UtsimJob>> {
        self.start(|c| Task::Little(c.model.clone()))
    }

    #[func]
    fn start_shifting(&mut self) -> Option<Gd<UtsimJob>> {
        self.start(|c| Task::ShiftingBottleneck(c.model.clone()))
    }
}

/// A computed PERT/CPM, for the PERT/CPM window.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimPert {
    core: Option<Box<PertOutput>>,
    base: Base<RefCounted>,
}

impl UtsimPert {
    /// The computed PERT/CPM.
    pub(crate) fn output(&self) -> Option<&PertOutput> {
        self.core.as_deref()
    }
}

#[godot_api]
impl UtsimPert {
    /// Everything the window draws (see `pert_view::PertOutput::to_json`).
    #[func]
    fn data(&self) -> Variant {
        self.core
            .as_ref()
            .map_or_else(Variant::nil, |c| variant_of_json(&c.to_json()))
    }

    /// The report of `utsim pert`, as text.
    #[func]
    fn text(&self) -> GString {
        self.core
            .as_ref()
            .map_or_else(GString::new, |c| GString::from(c.text().as_str()))
    }

    #[func]
    fn save_html(&self, path: GString) -> bool {
        self.core
            .as_ref()
            .is_some_and(|c| std::fs::write(path.to_string(), c.html()).is_ok())
    }

    #[func]
    fn save_csv(&self, dir: GString) -> PackedStringArray {
        self.core
            .as_ref()
            .and_then(|c| c.write_csv(&PathBuf::from(dir.to_string())).ok())
            .map_or_else(PackedStringArray::new, |f| strings(&f))
    }
}
