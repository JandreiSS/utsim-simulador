//! The simulated distribution of each KPI: the model runs `R` replications, sampled
//! every `window / samples_per_window` like a sensor would read it, and the same
//! [`Accumulator`] cuts the windows. Every window of every replication after the warm-up
//! is one draw of "what the KPI looks like in a window, according to the model".

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utsim_core::Step;
use utsim_model::Model;

use crate::config::{Measure, StateClass, TwinConfig};
use crate::map::{Event, EventKind};
use crate::shadow::Accumulator;

/// The draws of one KPI.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiSample {
    pub mean: f64,
    pub sd: f64,
    /// Sorted draws.
    pub values: Vec<f64>,
}

impl KpiSample {
    pub fn new(mut values: Vec<f64>) -> KpiSample {
        values.sort_by(f64::total_cmp);
        let n = values.len() as f64;
        let mean = values.iter().sum::<f64>() / n.max(1.0);
        let var = if values.len() > 1 {
            values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
        } else {
            0.0
        };
        KpiSample {
            mean,
            sd: var.sqrt(),
            values,
        }
    }

    /// Two-sided rank p-value of `y` among the draws: `2·min(#{x ≥ y} + 1, #{x ≤ y} + 1)
    /// / (n + 1)`, at most 1. No normality assumed.
    pub fn p_value(&self, y: f64) -> f64 {
        let n = self.values.len();
        let below = self.values.partition_point(|x| *x < y);
        let at_or_below = self.values.partition_point(|x| *x <= y);
        let ge = n - below;
        let le = at_or_below;
        (2.0 * (ge.min(le) + 1) as f64 / (n + 1) as f64).min(1.0)
    }

    /// Standardised residual `(y − mean)/sd`, bounded to ±10 (a constant simulated KPI
    /// gives ±10 for any difference).
    pub fn z(&self, y: f64) -> f64 {
        let d = y - self.mean;
        if self.sd > 0.0 {
            (d / self.sd).clamp(-10.0, 10.0)
        } else if d.abs() <= 1e-12 * (1.0 + self.mean.abs()) {
            0.0
        } else {
            10.0f64.copysign(d)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimDistribution {
    pub replications: u32,
    pub windows_per_replication: usize,
    pub window: f64,
    pub kpis: BTreeMap<String, KpiSample>,
}

/// `(object, measure)` pairs mapped in the configuration (positions left out).
pub fn mapped_measures(cfg: &TwinConfig) -> Vec<(String, Measure)> {
    let mut v: Vec<(String, Measure)> = cfg
        .tags
        .iter()
        .filter(|r| r.measure != Measure::Position)
        .map(|r| (r.object.clone(), r.measure))
        .collect();
    v.sort();
    v.dedup();
    v
}

/// Runs one replication and returns the KPIs of its windows.
fn one_replication(
    model: &Model,
    rep: u32,
    measures: &[(String, Measure)],
    window: f64,
    origin: f64,
    windows: usize,
    steps: u32,
) -> Result<Vec<BTreeMap<String, f64>>, String> {
    let mut sim = model.build(rep);
    sim.begin().map_err(|e| e.to_string())?;
    let ids: Vec<(usize, &String, Measure)> = {
        let status = sim.block_status();
        measures
            .iter()
            .map(|(o, m)| {
                status
                    .iter()
                    .position(|(n, _, _)| n == o)
                    .map(|i| (i, o, *m))
                    .ok_or_else(|| format!("o objeto '{o}' não existe no modelo"))
            })
            .collect::<Result<_, _>>()?
    };
    let mut acc = Accumulator::new(window, origin, measures, Some(origin));
    let mut last_count: BTreeMap<(usize, Measure), u64> = BTreeMap::new();
    let mut last_content: BTreeMap<usize, usize> = BTreeMap::new();
    let mut last_state: BTreeMap<usize, StateClass> = BTreeMap::new();
    let dt = window / f64::from(steps);
    let total = windows * steps as usize;
    for k in 0..=total {
        let t = origin + k as f64 * dt;
        if let Step::Decision(_) = sim.run_until(t).map_err(|e| e.to_string())? {
            return Err("o modelo pede decisões externas (agente); o gêmeo não suporta".into());
        }
        let status = sim.block_status();
        let report = sim.report();
        for &(i, name, m) in &ids {
            let (_, state, content) = &status[i];
            let kind = match m {
                Measure::Inputs | Measure::Outputs => {
                    let b = report.block(name);
                    let v = b
                        .map(|b| {
                            if m == Measure::Inputs {
                                b.input
                            } else {
                                b.output
                            }
                        })
                        .unwrap_or(0);
                    let prev = last_count.insert((i, m), v);
                    let delta = match prev {
                        None => continue,
                        Some(p) if v >= p => v - p,
                        // Statistics reset at the warm-up.
                        Some(_) => v,
                    };
                    if delta == 0 {
                        continue;
                    }
                    EventKind::Count {
                        measure: m,
                        delta: delta as f64,
                    }
                }
                Measure::Content => {
                    if last_content.insert(i, *content) == Some(*content) {
                        continue;
                    }
                    EventKind::Content(*content as f64)
                }
                Measure::State => {
                    let Some(class) = state.and_then(|s| StateClass::of_name(s.name())) else {
                        continue;
                    };
                    if last_state.insert(i, class) == Some(class) {
                        continue;
                    }
                    EventKind::State {
                        class,
                        debounce: 0.0,
                    }
                }
                Measure::Position => continue,
            };
            acc.apply(&Event {
                t,
                object: name.clone(),
                kind,
            });
        }
        acc.advance(t);
    }
    Ok(acc.closed().iter().map(|w| w.values.clone()).collect())
}

/// Simulates the distribution of the mapped KPIs. Replications run in parallel threads;
/// the result does not depend on the number of threads.
pub fn simulate(model: &Model, cfg: &TwinConfig) -> Result<SimDistribution, String> {
    let s = &cfg.shadow;
    let origin = s.sim_warmup.unwrap_or(model.run.warmup);
    let windows = ((model.run.stop_time - origin) / s.window).floor() as usize;
    if windows == 0 {
        return Err(format!(
            "a rodada do modelo não cabe numa janela de {} (de {origin} a {})",
            s.window, model.run.stop_time
        ));
    }
    let measures = mapped_measures(cfg);
    let reps: Vec<u32> = (0..s.replications).collect();
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .clamp(1, 8);
    let chunk = reps.len().div_ceil(threads).max(1);
    let results: Vec<Result<Vec<BTreeMap<String, f64>>, String>> = std::thread::scope(|sc| {
        let handles: Vec<_> = reps
            .chunks(chunk)
            .map(|part| {
                let measures = &measures;
                sc.spawn(move || {
                    part.iter()
                        .map(|&r| {
                            one_replication(
                                model,
                                r,
                                measures,
                                s.window,
                                origin,
                                windows,
                                s.samples_per_window,
                            )
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("replication thread"))
            .collect()
    });
    let mut draws: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for r in results {
        for w in r? {
            for (k, v) in w {
                draws.entry(k).or_default().push(v);
            }
        }
    }
    Ok(SimDistribution {
        replications: s.replications,
        windows_per_replication: windows,
        window: s.window,
        kpis: draws
            .into_iter()
            .map(|(k, v)| (k, KpiSample::new(v)))
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_p_values() {
        let s = KpiSample::new((1..=19).map(f64::from).collect());
        // Median: p = 1.
        assert_eq!(s.p_value(10.0), 1.0);
        // Above every draw: 2·1/20.
        assert!((s.p_value(100.0) - 0.1).abs() < 1e-12);
        assert!((s.p_value(-5.0) - 0.1).abs() < 1e-12);
        let c = KpiSample::new(vec![0.0; 9]);
        assert_eq!(c.p_value(0.0), 1.0);
        assert!((c.p_value(0.3) - 0.2).abs() < 1e-12);
        assert_eq!(c.z(0.0), 0.0);
        assert_eq!(c.z(0.3), 10.0);
    }
}
