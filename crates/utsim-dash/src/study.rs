//! Runs the replications of a study in the background and keeps the dashboard data
//! up to date while they finish: the intervals shrink as replications arrive.
//!
//! Each replication is built and driven exactly as [`Model::run_replications`] does
//! (`Model::build(r)` then `Model::drive`), so the results are the same as `utsim run`;
//! only the scheduling is ours, to publish partial results.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use utsim_core::RunReport;
use utsim_model::Model;

use crate::config::Dashboard;
use crate::data::{Package, Progress};

/// Samples per replication when the model does not set `sample_interval` (series, Gantt).
pub const DEFAULT_SAMPLES: f64 = 120.0;
/// Histogram bins when the model does not set `histogram_bins` (percentiles).
pub const DEFAULT_BINS: usize = 40;
/// Partial results are rebuilt at most this often.
const REBUILD_EVERY: Duration = Duration::from_millis(300);

/// A study being run. Cheap to clone (shared state).
#[derive(Clone)]
pub struct Study {
    inner: Arc<Inner>,
}

struct Inner {
    model: Model,
    cfg: Dashboard,
    origin: String,
    total: u32,
    done: AtomicU32,
    cancel: AtomicBool,
    finished: AtomicBool,
    error: Mutex<Option<String>>,
    reports: Mutex<Vec<RunReport>>,
    package: RwLock<Arc<Package>>,
    last_rebuild: Mutex<Instant>,
}

/// Fills in the settings the dashboard needs and the model leaves open: histograms (for
/// percentiles) and a sampling interval (for series and the Gantt chart).
pub fn prepare_model(mut model: Model) -> Model {
    if model.run.histogram_bins.is_none() {
        model.run.histogram_bins = Some(DEFAULT_BINS);
    }
    if model.run.sample_interval.is_none() && model.run.stop_time.is_finite() {
        model.run.sample_interval = Some(model.run.stop_time / DEFAULT_SAMPLES);
    }
    model
}

impl Study {
    /// Starts the replications in background threads and returns at once.
    pub fn start(
        model: Model,
        cfg: Dashboard,
        reps: Option<u32>,
        config_origin: &str,
    ) -> Result<Study, String> {
        let mut model = prepare_model(model);
        if let Some(n) = reps {
            model.run.replications = n.max(1);
        }
        model.validate().map_err(|e| e.to_string())?;
        cfg.validate()?;
        let total = model.run.replications;
        let empty = Package::build(
            &model,
            &cfg,
            &[],
            Progress {
                total,
                ..Progress::default()
            },
            config_origin,
        );
        let inner = Arc::new(Inner {
            model,
            cfg,
            origin: config_origin.to_string(),
            total,
            done: AtomicU32::new(0),
            cancel: AtomicBool::new(false),
            finished: AtomicBool::new(false),
            error: Mutex::new(None),
            reports: Mutex::new(Vec::new()),
            package: RwLock::new(Arc::new(empty)),
            last_rebuild: Mutex::new(Instant::now()),
        });
        let worker = Arc::clone(&inner);
        thread::spawn(move || worker.run());
        Ok(Study { inner })
    }

    /// Runs everything on the calling thread's behalf and returns the finished study.
    pub fn run(
        model: Model,
        cfg: Dashboard,
        reps: Option<u32>,
        config_origin: &str,
    ) -> Result<Study, String> {
        let s = Study::start(model, cfg, reps, config_origin)?;
        s.wait();
        match s.error() {
            Some(e) => Err(e),
            None => Ok(s),
        }
    }

    /// Blocks until the replications end (or are cancelled).
    pub fn wait(&self) {
        while !self.inner.finished.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(20));
        }
    }

    /// Stops starting new replications; the finished ones stay.
    pub fn cancel(&self) {
        self.inner.cancel.store(true, Ordering::Relaxed);
    }

    pub fn progress(&self) -> Progress {
        Progress {
            feitas: self.inner.done.load(Ordering::Relaxed),
            total: self.inner.total,
            concluido: self.inner.finished.load(Ordering::Acquire),
            cancelado: self.inner.cancel.load(Ordering::Relaxed),
        }
    }

    pub fn error(&self) -> Option<String> {
        self.inner.error.lock().ok()?.clone()
    }

    /// The data published so far.
    pub fn package(&self) -> Arc<Package> {
        match self.inner.package.read() {
            Ok(p) => Arc::clone(&p),
            Err(e) => Arc::clone(&e.into_inner()),
        }
    }

    pub fn model(&self) -> &Model {
        &self.inner.model
    }

    pub fn config(&self) -> &Dashboard {
        &self.inner.cfg
    }

    /// Finished replications, in replication order.
    pub fn reports(&self) -> Vec<RunReport> {
        self.inner
            .reports
            .lock()
            .map(|r| r.clone())
            .unwrap_or_default()
    }
}

impl Inner {
    fn progress(&self, finished: bool) -> Progress {
        Progress {
            feitas: self.done.load(Ordering::Relaxed),
            total: self.total,
            concluido: finished,
            cancelado: self.cancel.load(Ordering::Relaxed),
        }
    }

    fn rebuild(&self, reports: &[RunReport], finished: bool) {
        let package = Package::build(
            &self.model,
            &self.cfg,
            reports,
            self.progress(finished),
            &self.origin,
        );
        if let Ok(mut slot) = self.package.write() {
            *slot = Arc::new(package);
        }
    }

    fn run(&self) {
        let workers = thread::available_parallelism()
            .map_or(1, usize::from)
            .min(self.total as usize)
            .max(1);
        let next = AtomicU32::new(0);
        thread::scope(|scope| {
            for _ in 0..workers {
                scope.spawn(|| {
                    loop {
                        if self.cancel.load(Ordering::Relaxed) {
                            return;
                        }
                        let r = next.fetch_add(1, Ordering::Relaxed);
                        if r >= self.total {
                            return;
                        }
                        let mut sim = self.model.build(r);
                        match self.model.drive(&mut sim) {
                            Ok(report) => {
                                self.done.fetch_add(1, Ordering::Relaxed);
                                self.push(report);
                            }
                            Err(e) => {
                                if let Ok(mut slot) = self.error.lock() {
                                    *slot = Some(e.to_string());
                                }
                                self.cancel.store(true, Ordering::Relaxed);
                                return;
                            }
                        }
                    }
                });
            }
        });
        let snapshot = self.reports.lock().map(|r| r.clone()).unwrap_or_default();
        self.rebuild(&snapshot, true);
        self.finished.store(true, Ordering::Release);
    }

    fn push(&self, report: RunReport) {
        let Ok(mut reports) = self.reports.lock() else {
            return;
        };
        reports.push(report);
        reports.sort_by_key(|r| r.replication);
        let due = self
            .last_rebuild
            .lock()
            .map(|mut t| {
                let due = t.elapsed() >= REBUILD_EVERY;
                if due {
                    *t = Instant::now();
                }
                due
            })
            .unwrap_or(false);
        if due && reports.len() < self.total as usize {
            self.rebuild(&reports, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MM1: &str = r#"Model(
        name: "M/M/1",
        run: (stop_time: 20000.0, warmup: 500.0, seed: 7, replications: 4),
        objects: [
            (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0))),
            (name: "fila", kind: Queue()),
            (name: "maquina", kind: Processor(process: Exp(mean: 0.8))),
            (name: "saida", kind: Sink),
        ],
        connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
    )"#;

    #[test]
    fn replications_match_the_plain_run_and_publish_progress() {
        let model = Model::parse(MM1).unwrap();
        let study = Study::run(model.clone(), Dashboard::default(), None, "padrão").unwrap();
        let p = study.progress();
        assert_eq!((p.feitas, p.total, p.concluido), (4, 4, true));
        // Same reports as the library's own runner (with the settings the panel adds).
        let plain = prepare_model(model).run_replications().unwrap();
        let ours = study.reports();
        assert_eq!(ours.len(), 4);
        for (a, b) in ours.iter().zip(&plain) {
            assert_eq!(a.replication, b.replication);
            assert_eq!(a.events, b.events);
            assert_eq!(
                a.block("saida").unwrap().input,
                b.block("saida").unwrap().input
            );
        }
        assert!(study.package().routes.contains_key("/api/kpis"));
    }

    #[test]
    fn cancel_ends_early_and_errors_surface() {
        let mut model = Model::parse(MM1).unwrap();
        model.run.replications = 200;
        model.run.stop_time = 200_000.0;
        let study = Study::start(model, Dashboard::default(), None, "padrão").unwrap();
        study.cancel();
        study.wait();
        assert!(study.progress().feitas < 200 && study.progress().cancelado);
        // An invalid model is refused up front.
        let bad = Model::parse("Model(name: \"x\", run: (stop_time: -1.0), objects: [])").unwrap();
        assert!(Study::start(bad, Dashboard::default(), None, "").is_err());
    }
}
