//! The dashboard configuration file (`<model>.painel.ron`): targets, which cards to show
//! and in what order, the traffic-light thresholds. Without a file everything has a
//! default, so the dashboard assembles itself.
//!
//! ```ron
//! Dashboard(
//!     title: Some("Linha 1"),
//!     time_unit: Seconds,
//!     cards: ["vazao", "lead_time_p90", "wip", "utilizacao_gargalo"],
//!     targets: {"vazao": AtLeast(120.0), "lead_time_p90": AtMost(900.0)},
//!     thresholds: (green: 0.9, yellow: 0.5),
//! )
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// What the time unit of the model means (throughput is also shown per hour).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeUnit {
    #[default]
    Seconds,
    Minutes,
    Hours,
    /// Abstract unit (queueing-theory models): no per-hour figures.
    Unit,
}

impl TimeUnit {
    /// How many time units make one hour (1 for the abstract unit).
    pub fn per_hour(self) -> f64 {
        match self {
            TimeUnit::Seconds => 3600.0,
            TimeUnit::Minutes => 60.0,
            TimeUnit::Hours | TimeUnit::Unit => 1.0,
        }
    }

    /// Short label of the unit.
    pub fn label(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "s",
            TimeUnit::Minutes => "min",
            TimeUnit::Hours => "h",
            TimeUnit::Unit => "u.t.",
        }
    }

    /// Label of a throughput in this unit.
    pub fn throughput_label(self) -> &'static str {
        match self {
            TimeUnit::Unit => "itens/u.t.",
            _ => "itens/h",
        }
    }
}

/// A goal for a KPI.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Target {
    /// The KPI must be at least this value (throughput, service level).
    AtLeast(f64),
    /// The KPI must be at most this value (lead time, losses).
    AtMost(f64),
}

impl Target {
    pub fn value(self) -> f64 {
        match self {
            Target::AtLeast(v) | Target::AtMost(v) => v,
        }
    }

    /// Whether `x` meets the goal.
    pub fn met(self, x: f64) -> bool {
        match self {
            Target::AtLeast(v) => x >= v,
            Target::AtMost(v) => x <= v,
        }
    }
}

/// Traffic-light limits on the probability of meeting the goal.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Thresholds {
    /// Green when the probability is at least this (default 90%).
    pub green: f64,
    /// Yellow when it is at least this (default 50%); red below.
    pub yellow: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Thresholds {
            green: 0.9,
            yellow: 0.5,
        }
    }
}

/// OEE of one station: the ideal cycle and, optionally, the sinks that receive scrap.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OeeSpec {
    /// Object whose OEE is shown.
    pub object: String,
    /// Ideal cycle time per item, in model time units.
    pub ideal_cycle: f64,
    /// Sinks whose arrivals are scrap; without any, quality counts as 100%.
    #[serde(default)]
    pub scrap_sinks: Vec<String>,
}

/// The dashboard configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Dashboard {
    /// Title of the page (default: the model name).
    pub title: Option<String>,
    pub time_unit: TimeUnit,
    /// Confidence level of the intervals (default 95%).
    pub level: f64,
    pub thresholds: Thresholds,
    /// KPI ids of the manager view, in order (default: the standard cards).
    pub cards: Vec<String>,
    /// Goals by KPI id.
    pub targets: BTreeMap<String, Target>,
    /// Service-level limit: the fraction of items with lead time at most this (in model
    /// time units) becomes the KPI `nivel_servico`.
    pub lead_time_limit: Option<f64>,
    pub oee: Vec<OeeSpec>,
}

impl Default for Dashboard {
    fn default() -> Self {
        Dashboard {
            title: None,
            time_unit: TimeUnit::default(),
            level: utsim_stats::DEFAULT_LEVEL,
            thresholds: Thresholds::default(),
            cards: Vec::new(),
            targets: BTreeMap::new(),
            lead_time_limit: None,
            oee: Vec::new(),
        }
    }
}

/// Cards of the manager view when the file lists none.
pub const DEFAULT_CARDS: &[&str] = &[
    "vazao",
    "lead_time",
    "lead_time_p90",
    "wip",
    "utilizacao_gargalo",
    "otd",
    "perdas",
];

impl Dashboard {
    pub fn parse(text: &str) -> Result<Dashboard, String> {
        let cfg: Dashboard =
            ron::from_str(text).map_err(|e| format!("arquivo do painel inválido: {e}"))?;
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Dashboard, String> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("não foi possível ler {}: {e}", path.display()))?;
        Dashboard::parse(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// `<model>.painel.ron` next to the model (`linha.ron` -> `linha.painel.ron`).
    pub fn default_path(model: &Path) -> PathBuf {
        model.with_extension("painel.ron")
    }

    /// Loads the file next to the model when it exists; the default otherwise.
    pub fn load_next_to(model: &Path) -> Result<Dashboard, String> {
        let path = Dashboard::default_path(model);
        if path.is_file() {
            Dashboard::load(path)
        } else {
            Ok(Dashboard::default())
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(self.level > 0.0 && self.level < 1.0) {
            return Err("level deve estar entre 0 e 1".to_string());
        }
        let t = self.thresholds;
        if !(0.0..=1.0).contains(&t.yellow) || !(0.0..=1.0).contains(&t.green) || t.yellow > t.green
        {
            return Err("thresholds: use 0 <= yellow <= green <= 1".to_string());
        }
        if let Some(o) = self
            .oee
            .iter()
            .find(|o| o.ideal_cycle.is_nan() || o.ideal_cycle <= 0.0)
        {
            return Err(format!(
                "oee: o ciclo ideal de {} deve ser positivo",
                o.object
            ));
        }
        if self.lead_time_limit.is_some_and(|l| l.is_nan() || l <= 0.0) {
            return Err("lead_time_limit deve ser positivo".to_string());
        }
        Ok(())
    }

    /// The cards to show: the configured ones or [`DEFAULT_CARDS`].
    pub fn card_ids(&self) -> Vec<String> {
        if self.cards.is_empty() {
            DEFAULT_CARDS.iter().map(|s| s.to_string()).collect()
        } else {
            self.cards.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_file_is_the_default() {
        assert_eq!(
            Dashboard::parse("Dashboard()").unwrap(),
            Dashboard::default()
        );
        assert_eq!(Dashboard::default().card_ids()[0], "vazao");
    }

    #[test]
    fn the_documented_example_parses() {
        let cfg = Dashboard::parse(
            r#"Dashboard(
                title: Some("Linha 1"),
                time_unit: Minutes,
                cards: ["vazao", "wip"],
                targets: {"vazao": AtLeast(120.0), "lead_time_p90": AtMost(900.0)},
                thresholds: (green: 0.95, yellow: 0.6),
                oee: [(object: "M2", ideal_cycle: 40.0, scrap_sinks: ["refugo"])],
            )"#,
        )
        .unwrap();
        assert_eq!(cfg.title.as_deref(), Some("Linha 1"));
        assert_eq!(cfg.time_unit.per_hour(), 60.0);
        assert_eq!(cfg.card_ids(), ["vazao", "wip"]);
        assert!(cfg.targets["vazao"].met(120.0) && !cfg.targets["vazao"].met(119.9));
        assert!(cfg.targets["lead_time_p90"].met(900.0));
        assert_eq!(cfg.thresholds.green, 0.95);
        assert_eq!(cfg.oee[0].scrap_sinks, ["refugo"]);
    }

    #[test]
    fn typos_and_bad_values_are_errors() {
        assert!(Dashboard::parse("Dashboard(titulo: None)").is_err());
        assert!(Dashboard::parse("Dashboard(thresholds: (green: 0.4, yellow: 0.8))").is_err());
        assert!(Dashboard::parse("Dashboard(level: 1.5)").is_err());
        assert!(Dashboard::parse("Dashboard(oee: [(object: \"a\", ideal_cycle: 0.0)])").is_err());
    }

    #[test]
    fn the_file_sits_next_to_the_model() {
        assert_eq!(
            Dashboard::default_path(Path::new("x/linha.ron")),
            Path::new("x/linha.painel.ron")
        );
    }
}
