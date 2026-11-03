//! Deviation between the real plant and the model, window by window.
//!
//! For every KPI of a closed window:
//! - the real value `y` is ranked among the simulated draws (two-sided p-value, no
//!   normality assumed) and the p-values of the window go through Benjamini-Hochberg at
//!   the false discovery rate `fdr`: a rejection is a warning (`aviso`);
//! - the standardised residual `z = (y − mean)/sd` (real − simulated) feeds an EWMA and a
//!   tabular CUSUM of `utsim-quality`, which accumulate small persistent shifts: a signal
//!   is a deviation (`desvio`).
//!
//! Correction for the number `m` of KPIs watched at once: the CUSUM limit `h` is chosen
//! so that each KPI's in-control ARL (Siegmund) is `m · arl0`, and the EWMA width is
//! `L = Φ⁻¹(1 − 0.0027/(2m))` (Bonferroni on the 3σ false alarm rate), so the whole set
//! keeps about one false alarm every `arl0` windows.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utsim_quality::memory::{Cusum, Ewma};

use crate::config::ShadowSettings;
use crate::shadow::WindowKpis;
use crate::simdist::SimDistribution;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Level {
    #[default]
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "aviso")]
    Warning,
    #[serde(rename = "desvio")]
    Deviation,
}

/// An alert, raised when a KPI's level goes up.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub janela: usize,
    /// Twin time at the end of the window.
    pub t: f64,
    pub kpi: String,
    pub objeto: String,
    pub nivel: Level,
    /// `BH`, `CUSUM` or `EWMA`.
    pub teste: String,
    /// `acima` or `abaixo` of the model.
    pub sentido: String,
    pub real: f64,
    pub simulado_media: f64,
    pub simulado_dp: f64,
    pub p: f64,
    pub z: f64,
    pub mensagem: String,
}

/// Where one KPI stands.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct KpiStatus {
    pub nivel: Level,
    pub n_janelas: usize,
    pub ultimo_real: Option<f64>,
    pub ultimo_p: Option<f64>,
    pub ultimo_z: Option<f64>,
    pub simulado_media: f64,
    pub simulado_dp: f64,
    pub n_simulado: usize,
    pub cusum_alta: f64,
    pub cusum_baixa: f64,
    pub ewma: f64,
    /// Residuals real − simulated (standardised), one per window.
    pub z: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    /// KPIs watched.
    pub m: usize,
    pub cusum_k: f64,
    pub cusum_h: f64,
    pub ewma_lambda: f64,
    pub ewma_l: f64,
    pub fdr: f64,
    pub arl0: f64,
}

/// The CUSUM `h` whose two-sided in-control ARL (Siegmund) is `target`.
pub fn cusum_h_for(k: f64, target: f64) -> f64 {
    let f = |h: f64| utsim_quality::arl::cusum_siegmund(k, h, 0.0) - target;
    utsim_stats::special::brent(f, 0.05, 60.0, 1e-9).unwrap_or(5.0)
}

/// Benjamini-Hochberg: which of the p-values are rejected at false discovery rate `q`.
pub fn benjamini_hochberg(p: &[f64], q: f64) -> Vec<bool> {
    let m = p.len();
    let mut order: Vec<usize> = (0..m).collect();
    order.sort_by(|&a, &b| p[a].total_cmp(&p[b]).then(a.cmp(&b)));
    let mut cut = 0;
    for (rank, &i) in order.iter().enumerate() {
        if p[i] <= q * (rank + 1) as f64 / m as f64 {
            cut = rank + 1;
        }
    }
    let mut out = vec![false; m];
    for &i in &order[..cut] {
        out[i] = true;
    }
    out
}

pub struct DriftMonitor {
    limits: Limits,
    sim: SimDistribution,
    kpis: BTreeMap<String, KpiStatus>,
    alerts: Vec<Alert>,
    /// Tests done (KPI × window).
    tests: usize,
}

impl DriftMonitor {
    pub fn new(settings: &ShadowSettings, sim: SimDistribution) -> DriftMonitor {
        let m = sim.kpis.len().max(1);
        let alpha = 0.0027 / m as f64;
        let limits = Limits {
            m,
            cusum_k: settings.cusum_k,
            cusum_h: cusum_h_for(settings.cusum_k, settings.arl0 * m as f64),
            ewma_lambda: settings.ewma_lambda,
            ewma_l: utsim_stats::special::norm_ppf(1.0 - alpha / 2.0),
            fdr: settings.fdr,
            arl0: settings.arl0,
        };
        let kpis = sim
            .kpis
            .iter()
            .map(|(k, s)| {
                (
                    k.clone(),
                    KpiStatus {
                        simulado_media: s.mean,
                        simulado_dp: s.sd,
                        n_simulado: s.values.len(),
                        ..KpiStatus::default()
                    },
                )
            })
            .collect();
        DriftMonitor {
            limits,
            sim,
            kpis,
            alerts: Vec::new(),
            tests: 0,
        }
    }

    pub fn limits(&self) -> &Limits {
        &self.limits
    }

    pub fn alerts(&self) -> &[Alert] {
        &self.alerts
    }

    pub fn kpis(&self) -> &BTreeMap<String, KpiStatus> {
        &self.kpis
    }

    pub fn tests(&self) -> usize {
        self.tests
    }

    pub fn sim(&self) -> &SimDistribution {
        &self.sim
    }

    /// Tests a closed window.
    pub fn on_window(&mut self, w: &WindowKpis) {
        let present: Vec<(&String, f64)> = w
            .values
            .iter()
            .filter(|(k, _)| self.sim.kpis.contains_key(*k))
            .map(|(k, v)| (k, *v))
            .collect();
        if present.is_empty() {
            return;
        }
        let ps: Vec<f64> = present
            .iter()
            .map(|(k, y)| self.sim.kpis[*k].p_value(*y))
            .collect();
        let rejected = benjamini_hochberg(&ps, self.limits.fdr);
        self.tests += present.len();
        let ewma = Ewma {
            lambda: self.limits.ewma_lambda,
            l: self.limits.ewma_l,
            target: 0.0,
            sigma: 1.0,
            asymptotic: false,
        };
        let cusum = Cusum {
            k: self.limits.cusum_k,
            h: self.limits.cusum_h,
            target: 0.0,
            sigma: 1.0,
        };
        for (i, (key, y)) in present.iter().enumerate() {
            let sample = &self.sim.kpis[*key];
            let z = sample.z(*y);
            let st = self.kpis.get_mut(*key).expect("kpi known");
            st.z.push(z);
            st.n_janelas += 1;
            st.ultimo_real = Some(*y);
            st.ultimo_p = Some(ps[i]);
            st.ultimo_z = Some(z);
            let last = st.z.len() - 1;
            let er = ewma.run(&st.z).expect("valid EWMA");
            let cr = cusum.run(&st.z).expect("valid CUSUM");
            st.ewma = er.z[last];
            st.cusum_alta = cr.upper[last];
            st.cusum_baixa = cr.lower[last];
            let ewma_signal = er.signals.last() == Some(&last);
            let cusum_up = cr.signals_up.last() == Some(&last);
            let cusum_down = cr.signals_down.last() == Some(&last);
            let previous = st.nivel;
            let (level, test, up) = if cusum_up || cusum_down {
                (Level::Deviation, "CUSUM", cusum_up)
            } else if ewma_signal {
                (Level::Deviation, "EWMA", st.ewma > 0.0)
            } else if previous == Level::Deviation
                && (st.cusum_alta > self.limits.cusum_h / 2.0
                    || st.cusum_baixa > self.limits.cusum_h / 2.0)
            {
                // Hysteresis: stays until the sums fall to half the limit.
                (Level::Deviation, "CUSUM", st.cusum_alta >= st.cusum_baixa)
            } else if rejected[i] {
                (Level::Warning, "BH", z > 0.0)
            } else {
                (Level::Ok, "", false)
            };
            st.nivel = level;
            if level > previous {
                let (object, name) = key.split_once('.').unwrap_or((key.as_str(), ""));
                let sentido = if up { "acima" } else { "abaixo" };
                let mensagem = format!(
                    "{} de {object} {sentido} do modelo na janela {} (real {:.3}, modelo {:.3} ± {:.3}; {test}, p = {:.3}, z = {:+.2})",
                    kpi_label(name),
                    w.index + 1,
                    y,
                    sample.mean,
                    sample.sd,
                    ps[i],
                    z
                );
                self.alerts.push(Alert {
                    janela: w.index,
                    t: w.end,
                    kpi: key.to_string(),
                    objeto: object.to_string(),
                    nivel: level,
                    teste: test.to_string(),
                    sentido: sentido.to_string(),
                    real: *y,
                    simulado_media: sample.mean,
                    simulado_dp: sample.sd,
                    p: ps[i],
                    z,
                    mensagem,
                });
            }
        }
    }
}

pub fn kpi_label(name: &str) -> &str {
    match name {
        "vazao" => "vazão",
        "chegadas" => "chegadas",
        "wip" => "conteúdo médio (WIP)",
        "utilizacao" => "utilização",
        "quebra" => "tempo quebrado",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bh_by_hand() {
        // m = 4, q = 0.1: thresholds 0.025, 0.05, 0.075, 0.1.
        let r = benjamini_hochberg(&[0.04, 0.01, 0.5, 0.07], 0.1);
        assert_eq!(r, vec![true, true, false, true]);
        assert_eq!(benjamini_hochberg(&[0.3, 0.2], 0.1), vec![false, false]);
    }

    #[test]
    fn cusum_limit_grows_with_the_number_of_kpis() {
        let h1 = cusum_h_for(0.5, 370.0);
        let h10 = cusum_h_for(0.5, 3700.0);
        assert!((utsim_quality::arl::cusum_siegmund(0.5, h1, 0.0) - 370.0).abs() < 1e-3);
        assert!(h1 > 4.0 && h1 < 5.5, "{h1}");
        assert!(h10 > h1);
    }
}
