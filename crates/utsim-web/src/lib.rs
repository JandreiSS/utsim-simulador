//! UTSim in the browser. The functions here are thin `wasm-bindgen` wrappers: all the
//! logic lives in [`api`], plain Rust that is unit-tested natively.

pub mod api;

use wasm_bindgen::prelude::*;

/// Runs the model `replications` times (0 = the value written in the file).
/// Returns JSON: `{ok, name, replications, stop_time, events, blocks, summary}` or
/// `{ok:false, errors:[...]}`.
#[wasm_bindgen]
pub fn run_model(ron_text: &str, replications: u32) -> String {
    api::run_model(ron_text, replications)
}

/// JSON list of problems found in the model (empty list = valid).
#[wasm_bindgen]
pub fn validate_model(ron_text: &str) -> String {
    api::validate_model(ron_text)
}

/// Fits distributions to the CSV text and returns the HTML report.
/// `options_json`: `{column, timestamps, alpha, bootstrap, seed}` (all optional).
#[wasm_bindgen]
pub fn fit_data(csv_text: &str, options_json: &str) -> Result<String, JsError> {
    api::fit_data(csv_text, options_json).map_err(|e| JsError::new(&e))
}

/// Records one replication (trace + layout) as JSON, for a future 3D view.
/// `stop` > 0 overrides `run.stop_time`.
#[wasm_bindgen]
pub fn replay(ron_text: &str, stop: f64) -> String {
    api::replay(ron_text, stop)
}

/// JSON `[{name, text}]` with every example model, embedded at build time.
#[wasm_bindgen]
pub fn examples() -> String {
    api::examples()
}
