//! The PERT/CPM window of the selection (G3), with no Godot types: derives the activity
//! network of the selected objects (or "from A to B") with `utsim-pert`, computes it in
//! the mode asked and shapes everything the window draws as JSON: the activity-on-node
//! diagram (the coordinates of `utsim_pert::report::diagram`), the Gantt bars, the
//! histogram of the total time, the table and the summary lines.
//!
//! The window opens in the simulated mode (the critical path observed in the simulation,
//! queues and resources included: the owner's choice); the classic PERT and the Monte
//! Carlo over the network are one click away. The CPM dates on the mean durations are
//! always there, as reference, like in the reports of `utsim pert`.

use serde_json::{Value, json};
use utsim_model::Model;
use utsim_pert::monte_carlo::{self, McOptions, McResult};
use utsim_pert::observed::{self, ObservedOptions, ObservedResult};
use utsim_pert::pert::{self, Moments, PertResult};
use utsim_pert::report::{self, Analysis};
use utsim_pert::{ActivityNetwork, DeriveOptions, Mode, Selection, derive};

use crate::results::{histogram, num, summary};

/// Bins of the histogram of the classic PERT (its normal approximation).
const NORMAL_BINS: usize = 24;

/// What the window asks for.
#[derive(Clone, Debug, PartialEq)]
pub struct PertRequest {
    pub selection: Selection,
    pub mode: Mode,
    pub deadline: Option<f64>,
    /// Draws of the Monte Carlo.
    pub samples: usize,
    pub moments: Moments,
    /// Replications of the simulated mode (`None`: the model's).
    pub replications: Option<u32>,
}

impl PertRequest {
    /// The selection of the editor: `between` uses the first name as the start and the
    /// second as the end ("de A até B"); otherwise every name is in the part.
    pub fn of(names: &[String], between: bool, mode: Mode) -> Result<PertRequest, String> {
        let selection = if between {
            match names {
                [from, to] => Selection::Between {
                    from: from.clone(),
                    to: to.clone(),
                },
                _ => return Err("selecione dois objetos: o início e o fim do trecho".into()),
            }
        } else if names.is_empty() {
            return Err("selecione os objetos da parte do processo".into());
        } else {
            Selection::Objects(names.to_vec())
        };
        Ok(PertRequest {
            selection,
            mode,
            deadline: None,
            samples: 10_000,
            moments: Moments::Exact,
            replications: None,
        })
    }
}

pub fn mode_name(mode: Mode) -> &'static str {
    match mode {
        Mode::Simulated => "simulado",
        Mode::Classic => "classico",
        Mode::MonteCarlo => "montecarlo",
    }
}

/// A computed PERT/CPM.
pub struct PertOutput {
    pub title: String,
    pub request: PertRequest,
    pub network: ActivityNetwork,
    pub classic: PertResult,
    pub monte_carlo: Option<McResult>,
    pub observed: Option<ObservedResult>,
}

pub fn run(model: &Model, req: &PertRequest) -> Result<PertOutput, String> {
    model.validate().map_err(|e| e.to_string())?;
    let network =
        derive(model, &req.selection, &DeriveOptions::default()).map_err(|e| e.to_string())?;
    let classic = pert::analyze(&network, req.moments, req.deadline);
    let monte_carlo = (req.mode == Mode::MonteCarlo).then(|| {
        monte_carlo::run(
            &network,
            &McOptions {
                samples: req.samples.max(100),
                seed: model.run.seed,
                deadline: req.deadline,
                ..McOptions::default()
            },
        )
    });
    let observed = match req.mode {
        Mode::Simulated => Some(
            observed::run(
                model,
                &network,
                &ObservedOptions {
                    replications: req.replications,
                    deadline: req.deadline,
                    ..ObservedOptions::default()
                },
            )
            .map_err(|e| e.to_string())?,
        ),
        _ => None,
    };
    Ok(PertOutput {
        title: model.name.clone(),
        request: req.clone(),
        network,
        classic,
        monte_carlo,
        observed,
    })
}

impl PertOutput {
    pub fn analysis(&self) -> Analysis<'_> {
        Analysis {
            title: &self.title,
            selection: self.request.selection.describe(),
            mode: self.request.mode,
            network: &self.network,
            classic: &self.classic,
            monte_carlo: self.monte_carlo.as_ref(),
            observed: self.observed.as_ref(),
        }
    }

    /// Criticality of each activity in the mode (1 or 0 from the CPM in the classic).
    pub fn criticality(&self) -> Vec<f64> {
        let net = &self.network;
        match (&self.monte_carlo, &self.observed) {
            (Some(mc), _) => mc.criticality.clone(),
            (_, Some(obs)) => net
                .activities
                .iter()
                .map(|a| {
                    if a.is_milestone() {
                        1.0
                    } else {
                        obs.activities
                            .iter()
                            .find(|o| o.name == a.name)
                            .map_or(0.0, |o| o.pooled_criticality)
                    }
                })
                .collect(),
            _ => self
                .classic
                .schedule
                .critical
                .iter()
                .map(|&c| if c { 1.0 } else { 0.0 })
                .collect(),
        }
    }

    /// Objects of the model whose activity is critical in at least half of the cases
    /// (outlined in red in the 3D view).
    pub fn critical_objects(&self) -> Vec<String> {
        let crit = self.criticality();
        self.network
            .activities
            .iter()
            .zip(&crit)
            .filter(|(a, c)| **c >= 0.5 && !a.is_milestone())
            .filter_map(|(a, _)| a.object.clone())
            .collect()
    }

    /// The histogram of the total time: of the mode (Monte Carlo, observed units) or, in
    /// the classic PERT, its normal approximation `N(μ, σ²)` as 1000 expected counts.
    fn total_histogram(&self) -> Value {
        if let Some(h) = self.monte_carlo.as_ref().and_then(|m| m.histogram.as_ref()) {
            return histogram(h);
        }
        if let Some(h) = self.observed.as_ref().and_then(|o| o.histogram.as_ref()) {
            return histogram(h);
        }
        let (mu, sd) = (self.classic.mean, self.classic.sd());
        if !(sd > 0.0 && mu.is_finite()) {
            return Value::Null;
        }
        let (lo, hi) = (mu - 3.5 * sd, mu + 3.5 * sd);
        let width = (hi - lo) / NORMAL_BINS as f64;
        let cdf = |x: f64| utsim_pert::math::norm_cdf((x - mu) / sd);
        let counts: Vec<u64> = (0..NORMAL_BINS)
            .map(|k| {
                let a = lo + k as f64 * width;
                (1000.0 * (cdf(a + width) - cdf(a))).round() as u64
            })
            .collect();
        json!({"min": lo, "max": hi, "bin_width": width, "counts": counts, "normal": true})
    }

    /// The total time: `{mean, half_width, sd, percentiles, deadline}`.
    fn total(&self) -> Value {
        let c = &self.classic;
        let deadline = |d: f64, p: f64, low: Value, high: Value| json!({"deadline": d, "probability": num(p), "low": low, "high": high});
        if let Some(mc) = &self.monte_carlo {
            return json!({
                "mean": num(mc.total.mean),
                "half_width": num(mc.total.half_width),
                "sd": num(mc.total.std_dev),
                "percentiles": mc.percentiles,
                "deadline": mc.deadline.map(|d| deadline(d.deadline, d.probability, num(d.low), num(d.high))),
                "units": mc.samples,
            });
        }
        if let Some(o) = &self.observed {
            return json!({
                "mean": num(o.total.mean),
                "half_width": num(o.total.half_width),
                "sd": num(o.pooled.std_dev),
                "percentiles": o.percentiles,
                "deadline": o.deadline.as_ref().map(|d| deadline(
                    d.deadline,
                    d.fraction.mean,
                    num(d.fraction.mean - d.fraction.half_width),
                    num(d.fraction.mean + d.fraction.half_width),
                )),
                "units": o.units,
            });
        }
        json!({
            "mean": num(c.mean),
            "half_width": Value::Null,
            "sd": num(c.sd()),
            "percentiles": [],
            "deadline": c.deadline.zip(c.probability).map(|(d, p)| deadline(d, p, Value::Null, Value::Null)),
            "units": Value::Null,
        })
    }

    /// Critical paths of the mode with their index, as activity names (no milestones).
    fn paths(&self) -> Value {
        let net = &self.network;
        if let Some(mc) = &self.monte_carlo {
            return mc
                .paths
                .iter()
                .take(10)
                .map(|p| json!({"names": report::path_names(net, &p.path), "index": num(p.index)}))
                .collect();
        }
        if let Some(o) = &self.observed {
            return o
                .paths
                .iter()
                .take(10)
                .map(|(names, f)| json!({"names": names, "index": num(*f)}))
                .collect();
        }
        self.classic
            .schedule
            .critical_paths
            .iter()
            .take(10)
            .map(|p| json!({"names": report::path_names(net, p), "index": 1.0}))
            .collect()
    }

    pub fn to_json(&self) -> Value {
        let s = &self.classic.schedule;
        let crit = self.criticality();
        let d = report::diagram(&self.network, s, &crit);
        let boxes: Vec<Value> = d
            .boxes
            .iter()
            .map(|b| {
                json!({
                    "name": b.name, "x": b.x, "y": b.y, "width": b.width, "height": b.height,
                    "milestone": b.milestone, "critical": b.critical,
                    "criticality": num(b.criticality),
                    "es": num(b.es), "ef": num(b.ef), "ls": num(b.ls), "lf": num(b.lf),
                    "total_float": num(b.total_float),
                })
            })
            .collect();
        let arrows: Vec<Value> = d
            .arrows
            .iter()
            .map(|a| json!({"points": a.points, "critical": a.critical, "weight": num(a.weight)}))
            .collect();
        let activities: Vec<Value> = self
            .network
            .activities
            .iter()
            .enumerate()
            .filter(|(_, a)| !a.is_milestone())
            .map(|(i, a)| {
                json!({
                    "name": a.name,
                    "label": a.label,
                    "object": a.object,
                    "mean": num(self.classic.means[i]),
                    "sd": num(self.classic.variances[i].sqrt()),
                    "es": num(s.es[i]), "ef": num(s.ef[i]), "ls": num(s.ls[i]), "lf": num(s.lf[i]),
                    "total_float": num(s.total_float[i]),
                    "free_float": num(s.free_float[i]),
                    "critical": s.critical[i],
                    "criticality": num(crit[i]),
                    "note": a.note,
                })
            })
            .collect();
        let mut warnings = self.network.warnings.clone();
        warnings.extend(self.classic.warnings.iter().cloned());
        if let Some(o) = &self.observed {
            warnings.extend(o.warnings.iter().cloned());
        }
        json!({
            "title": self.title,
            "selection": self.request.selection.describe(),
            "mode": mode_name(self.request.mode),
            "mode_label": self.request.mode.label(),
            "project": num(s.project),
            "diagram": {"width": d.width, "height": d.height, "boxes": boxes, "arrows": arrows},
            "activities": activities,
            "total": self.total(),
            "histogram": self.total_histogram(),
            "paths": self.paths(),
            "critical_objects": self.critical_objects(),
            "warnings": warnings,
            "assumptions": self.network.assumptions,
            "observed_mean_waits": self.observed.as_ref().map(|o| o.activities.iter()
                .map(|a| json!({"name": a.name, "wait": num(a.wait), "work": num(a.work), "blocked": num(a.blocked),
                    "criticality": summary(&a.criticality)}))
                .collect::<Vec<_>>()),
        })
    }

    pub fn text(&self) -> String {
        report::to_text(&self.analysis())
    }

    pub fn html(&self) -> String {
        report::to_html(&self.analysis())
    }

    pub fn write_csv(&self, dir: &std::path::Path) -> std::io::Result<Vec<String>> {
        report::write_csv(&self.analysis(), dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::tests::example;

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_three_modes_on_a_line() {
        let m = example("mm1.ron");
        let mut req =
            PertRequest::of(&names(&["chegada", "saida"]), true, Mode::Simulated).unwrap();
        req.replications = Some(2);
        req.deadline = Some(10.0);
        let sim = run(&m, &req).unwrap().to_json();
        assert_eq!(sim["mode"], "simulado");
        assert!(
            sim["total"]["units"].as_u64().unwrap() > 0,
            "{}",
            sim["total"]
        );
        assert!(sim["histogram"]["counts"].is_array());
        assert!(sim["total"]["deadline"]["probability"].is_number());
        req.mode = Mode::Classic;
        let classic = run(&m, &req).unwrap();
        let cj = classic.to_json();
        // fila (0 in the classic network) and maquina, Exp(0.8).
        assert!(
            (cj["project"].as_f64().unwrap() - 0.8).abs() < 1e-9,
            "{}",
            cj["project"]
        );
        assert_eq!(cj["histogram"]["normal"], true);
        assert!(classic.critical_objects().contains(&"maquina".to_string()));
        assert!(classic.text().contains("PERT"));
        assert!(classic.html().contains("<svg"));
        req.mode = Mode::MonteCarlo;
        req.samples = 2000;
        let mc = run(&m, &req).unwrap().to_json();
        assert_eq!(mc["total"]["units"], 2000);
        let mean = mc["total"]["mean"].as_f64().unwrap();
        assert!((mean - 0.8).abs() < 0.1, "{mean}");
        // Diagram: one box per activity (milestones included), arrows between them.
        let boxes = mc["diagram"]["boxes"].as_array().unwrap();
        assert!(boxes.len() >= 3);
        assert!(!mc["diagram"]["arrows"].as_array().unwrap().is_empty());
    }

    #[test]
    fn the_selection_must_be_given() {
        assert!(PertRequest::of(&[], false, Mode::Classic).is_err());
        assert!(PertRequest::of(&names(&["a"]), true, Mode::Classic).is_err());
        let m = example("mm1.ron");
        let req = PertRequest::of(&names(&["nao_existe"]), false, Mode::Classic).unwrap();
        assert!(run(&m, &req).is_err());
    }
}
