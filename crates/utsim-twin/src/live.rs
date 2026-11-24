//! The live loop: drains the connectors every tick, gives each reading its twin time,
//! feeds the [`Twin`] and closes the windows by the clock.
//!
//! Readings are ordered by their effective instant (the source's timestamp, OPC UA
//! SourceTimestamp or Sparkplug `timestamp`, when present and within 2 s of the arrival;
//! else the arrival), then source and sequence; each waits [`REORDER`] after arrival so
//! that the values of one notification fall in order. The twin time is that instant on
//! the local clock (`Wall`) or the last value of the clock tag before it (`Tag`); it
//! never goes back. Before the clock tag's first value the time is not known: those
//! readings wait and take the time of that first value (a twin that joins a running
//! line does not put them at 0). The observation is recorded with that time,
//! so the replay does not depend on when the packets arrived.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel, sync_channel};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::config::ClockSpec;
use crate::connect::{CHANNEL_CAPACITY, Connector, Health, Sink};
use crate::forecast::{self, Snapshot};
use crate::obs::{Quality, RawObservation, now_ms};
use crate::twin::Twin;

/// What the panel reads while the twin runs.
pub struct Shared {
    pub twin: Mutex<Twin>,
    pub health: Vec<(String, Arc<Mutex<Health>>)>,
}

impl Shared {
    /// The report plus the health of each connection, as JSON.
    pub fn json(&self, with_state: bool) -> serde_json::Value {
        let report = self.twin.lock().expect("twin lock").report(with_state);
        let mut v = serde_json::to_value(report).unwrap_or_default();
        let health: serde_json::Map<String, serde_json::Value> = self
            .health
            .iter()
            .map(|(n, h)| {
                (
                    n.clone(),
                    serde_json::to_value(h.lock().expect("health lock").clone())
                        .unwrap_or_default(),
                )
            })
            .collect();
        v["conexoes"] = serde_json::Value::Object(health);
        v
    }
}

/// Running connectors.
pub struct Started {
    pub rx: Receiver<RawObservation>,
    pub health: Vec<(String, Arc<Mutex<Health>>)>,
    pub handles: Vec<JoinHandle<()>>,
    /// Instant the clock counts from.
    pub origin: Instant,
    pub origin_ms: u64,
}

/// Starts the connectors.
pub fn start(
    connectors: Vec<Box<dyn Connector>>,
    cancel: &Arc<AtomicBool>,
) -> Result<Started, String> {
    let (tx, rx) = sync_channel(CHANNEL_CAPACITY);
    let origin = Instant::now();
    let origin_ms = now_ms();
    let mut health = Vec::new();
    let mut handles = Vec::new();
    for c in connectors {
        let name = c.name().to_string();
        let (sink, h) = Sink::new(&name, tx.clone(), cancel.clone());
        handles.push(c.start(sink)?);
        health.push((name, h));
    }
    Ok(Started {
        rx,
        health,
        handles,
        origin,
        origin_ms,
    })
}

/// How long a reading waits after arrival before it is processed (reordering by the
/// source timestamps).
pub const REORDER: Duration = Duration::from_millis(100);
/// Largest difference between a source timestamp and the arrival still trusted.
pub const MAX_SKEW_US: u64 = 2_000_000;

#[derive(Clone, Debug)]
pub struct LiveOptions {
    /// Length of one tick of the loop.
    pub tick: Duration,
    /// Twin time at which the session ends (absent: until cancelled).
    pub until: Option<f64>,
    /// Longest wall time (tests; absent: no limit).
    pub max_wall: Option<Duration>,
    pub cancel: Arc<AtomicBool>,
}

/// Runs the loop until `until`, the wall limit or the cancel flag; then stops the
/// connectors and ends the twin (writing the end of the recording).
///
/// With the forecast on ([`Twin::enable_forecast`]), a worker thread computes the
/// forecast of every snapshot the twin takes, outside the twin's lock, so the loop is
/// never held up by it; the results go back to the twin (the panel reads them there).
/// The session waits for the forecasts still running when it ends.
pub fn run(shared: &Shared, started: Started, opts: &LiveOptions) -> f64 {
    let job = shared.twin.lock().expect("twin lock").forecast_job();
    std::thread::scope(|sc| {
        let (tx, rx) = channel::<Snapshot>();
        let worker = job.map(|(model, settings, unit)| {
            sc.spawn(move || {
                for snap in rx {
                    let r = forecast::of_snapshot(&model, &settings, unit, &snap);
                    match &r {
                        Ok(f) => println!("[previsão t={:.1}] {}", f.instante, f.resumo),
                        Err(e) => eprintln!("[previsão t={:.1}] erro: {e}", snap.t),
                    }
                    shared.twin.lock().expect("twin lock").add_forecast(r);
                }
            })
        });
        let end = run_loop(shared, started, opts, &tx);
        drop(tx);
        if let Some(w) = worker {
            let _ = w.join();
        }
        end
    })
}

fn run_loop(
    shared: &Shared,
    started: Started,
    opts: &LiveOptions,
    forecasts: &Sender<Snapshot>,
) -> f64 {
    let Started {
        rx,
        handles,
        origin,
        origin_ms,
        ..
    } = started;
    let clock = shared
        .twin
        .lock()
        .expect("twin lock")
        .config()
        .clock
        .clone();
    let origin_us = origin_ms * 1000;
    let mut now_t = 0.0f64;
    let mut clock_value: Option<f64> = None;
    let mut last_flush = Instant::now();
    let mut pending: Vec<(u64, RawObservation)> = Vec::new();
    let mut skewed: Vec<String> = Vec::new();
    // Readings before the clock tag's first value (`Tag`), waiting for it.
    let mut early: Vec<(u64, RawObservation)> = Vec::new();
    loop {
        let stop = opts.cancel.load(Ordering::Relaxed)
            || opts.max_wall.is_some_and(|m| origin.elapsed() >= m);
        // Effective instant of each reading: the source's timestamp, unless it is more
        // than 2 s away from the arrival (clocks out of step: the arrival is used).
        for raw in rx.try_iter() {
            let arrival = origin_us + raw.at.saturating_duration_since(origin).as_micros() as u64;
            let eff = match raw.source_us {
                Some(ts) if ts.abs_diff(arrival) <= MAX_SKEW_US => ts,
                Some(_) => {
                    if !skewed.contains(&raw.source) {
                        eprintln!(
                            "aviso: o relógio da conexão '{}' difere do local em mais de 2 s; usando a hora de chegada",
                            raw.source
                        );
                        skewed.push(raw.source.clone());
                    }
                    arrival
                }
                None => arrival,
            };
            pending.push((eff, raw));
        }
        pending.sort_by(|(ea, a), (eb, b)| {
            ea.cmp(eb)
                .then_with(|| a.source.cmp(&b.source))
                .then(a.seq.cmp(&b.seq))
        });
        // Readings wait `REORDER` after arrival, so that late packets still fall in order.
        let ready = if stop {
            0
        } else {
            pending
                .iter()
                .position(|(_, r)| r.at.elapsed() < REORDER)
                .unwrap_or(pending.len())
        };
        let batch: Vec<(u64, RawObservation)> = pending.drain(..ready).collect();
        {
            let mut twin = shared.twin.lock().expect("twin lock");
            for (eff, raw) in batch {
                let t = match &clock {
                    ClockSpec::Wall { unit_seconds } => {
                        eff.saturating_sub(origin_us) as f64 / 1e6 / unit_seconds
                    }
                    ClockSpec::Tag { source, tag } => {
                        if raw.source == *source && raw.tag == *tag && raw.quality != Quality::Bad {
                            if let Some(v) = raw.value.as_f64() {
                                clock_value = Some(clock_value.map_or(v, |c| c.max(v)));
                            }
                        }
                        match clock_value {
                            Some(c) => c,
                            None => {
                                early.push((eff, raw));
                                continue;
                            }
                        }
                    }
                };
                for (eff, raw) in early.drain(..).chain(std::iter::once((eff, raw))) {
                    if let Some(u) = opts.until.filter(|u| t > *u) {
                        // Past the end of the session: not processed nor recorded.
                        now_t = now_t.max(u);
                        continue;
                    }
                    now_t = now_t.max(t);
                    let obs = twin.stamp(
                        &raw.source,
                        &raw.tag,
                        raw.value,
                        raw.quality,
                        raw.seq,
                        now_t,
                        eff / 1000,
                    );
                    twin.ingest(obs);
                }
            }
            if let ClockSpec::Wall { unit_seconds } = &clock {
                let settled = origin.elapsed().saturating_sub(REORDER);
                now_t = now_t.max(settled.as_secs_f64() / unit_seconds);
            }
            if let Some(u) = opts.until {
                now_t = now_t.min(u.max(twin.now()));
            }
            twin.advance(now_t);
            for snap in twin.take_snapshots() {
                let _ = forecasts.send(snap);
            }
            if last_flush.elapsed() >= Duration::from_secs(1) {
                twin.flush();
                last_flush = Instant::now();
            }
        }
        if stop || opts.until.is_some_and(|u| now_t >= u) {
            break;
        }
        std::thread::sleep(opts.tick);
    }
    opts.cancel.store(true, Ordering::Relaxed);
    for h in handles {
        let _ = h.join();
    }
    let mut twin = shared.twin.lock().expect("twin lock");
    twin.finish(now_t);
    for snap in twin.take_snapshots() {
        let _ = forecasts.send(snap);
    }
    now_t
}

/// Sets `flag` when the process receives Ctrl-C (the session then ends cleanly: the end
/// of the recording is written).
pub fn cancel_on_ctrl_c(flag: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        let Ok(rt) = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        else {
            return;
        };
        if rt.block_on(tokio::signal::ctrl_c()).is_ok() {
            flag.store(true, Ordering::Relaxed);
        }
    });
}
