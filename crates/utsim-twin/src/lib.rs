//! Digital twin of a UTSim model: shadow mode (only observes) and forecast.
//!
//! ```text
//!  OPC UA (client) · MQTT · Sparkplug B        read-only connectors, one thread each
//!         |  bounded channel
//!         v
//!  [live]  twin time (local clock or a clock tag), order (instant, source, seq)
//!         v
//!  [Twin]  record (JSONL, people anonymised) -> mapping -> observed state + window KPIs
//!         |                                                   |
//!         |                     simulated distribution (R replications of the model)
//!         v                                                   v
//!  InitialState of the plant                 rank p-values + BH, CUSUM/EWMA -> alerts
//!         |  snapshot (every N or on request)
//!         v
//!  [forecast]  R replications to the horizon: bands, P(goal), crossings, "what if"
//!         \__________________________ panel (/gemeo on utsim-dash) ________________/
//! ```
//!
//! - [`config`]: the `.gemeo.ron` file and its validation against the model;
//! - [`obs`], [`map`]: observations and the mapping sensor → object (scale, unit,
//!   offset, counters, state codes, dead band, debounce, range, JSON fields);
//! - [`shadow`]: observed state, KPIs per window, the observed state as an
//!   [`utsim_model::InitialState`] (the start of the forecast);
//! - [`simdist`], [`drift`]: the model's distribution of each KPI and the deviation tests;
//! - [`record`], [`twin::replay`]: recording and replay without network, same results;
//! - [`connect`]: OPC UA client, MQTT (JSON) and Sparkplug B ([`sparkplug`]);
//! - [`forecast`]: the forecast ahead from the observed state (or a state file): R
//!   replications to the horizon, bands, probability of the goals, limit crossings and
//!   "what if" scenarios with common random numbers;
//! - [`live`], [`panel`]: the live loop (with the forecast worker) and the dashboard routes.
//!
//! Nothing here writes to the plant. See the manual page "Gêmeo digital (modo sombra)".

pub mod config;
pub mod connect;
pub mod drift;
pub mod forecast;
pub mod live;
pub mod map;
pub mod obs;
pub mod panel;
pub mod record;
pub mod shadow;
pub mod simdist;
#[cfg(feature = "mqtt")]
pub mod sparkplug;
pub mod twin;

pub use config::{ConfigError, TwinConfig, model_hash};
pub use twin::{Twin, TwinReport, replay};

use std::path::Path;

use utsim_model::Model;

/// A configuration, its model and the warnings of the validation.
pub struct Prepared {
    pub model: Model,
    pub model_text: String,
    pub config: TwinConfig,
    pub config_text: String,
    pub warnings: Vec<String>,
}

/// Loads and validates a `.gemeo.ron` text with its model (`model_path`, or the one the
/// file names, relative to `config_dir`).
pub fn prepare_text(
    config_text: &str,
    config_dir: Option<&Path>,
    model_path: Option<&Path>,
) -> Result<Prepared, String> {
    let config = TwinConfig::parse(config_text).map_err(|e| e.to_string())?;
    let path = match model_path {
        Some(p) => p.to_path_buf(),
        None => match config_dir {
            Some(d) => config.model_path(&d.join("x")),
            None => config.model.clone().into(),
        },
    };
    let model_text =
        std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let model = Model::load(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    model.validate().map_err(|e| e.to_string())?;
    let warnings = config
        .validate(&model, Some(&model_text))
        .map_err(|e| e.to_string())?;
    Ok(Prepared {
        model,
        model_text,
        config,
        config_text: config_text.to_string(),
        warnings,
    })
}
