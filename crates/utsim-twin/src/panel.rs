//! The twin on the dashboard: routes served by `utsim-dash` next to its own (through
//! [`utsim_dash::server::Extra`]):
//!
//! | route | content |
//! |---|---|
//! | `/gemeo` | page "Gêmeo (sombra)": connections, alerts, KPIs real × simulated, objects |
//! | `/gemeo/previsao` | page "Previsão": band in time, traffic lights, limits, scenarios |
//! | `/gemeo.js`, `/gemeo.css`, `/previsao.js` | scripts and style (the dashboard's policy forbids inline scripts) |
//! | `/api/gemeo` | the [`TwinReport`](crate::twin::TwinReport) plus the health of each connection |
//! | `/api/gemeo/estado` | the same with the observed state as an initial state (slower) |
//! | `/api/gemeo/estado.ron` | the observed state as an initial state file |
//! | `/api/gemeo/previsao` | the forecast status and the latest forecast |
//! | `/api/gemeo/previsoes` | every forecast kept (at most [`MAX_FORECASTS`](crate::twin::MAX_FORECASTS)) |
//! | `POST /api/gemeo/previsao` | asks for a forecast now (header `X-UTSim` required) |

use std::sync::Arc;

use utsim_dash::server::Extra;

use crate::live::Shared;

pub const PAGE: &str = include_str!("../web/gemeo.html");
pub const SCRIPT: &str = include_str!("../web/gemeo.js");
pub const STYLE: &str = include_str!("../web/gemeo.css");
pub const FORECAST_PAGE: &str = include_str!("../web/previsao.html");
pub const FORECAST_SCRIPT: &str = include_str!("../web/previsao.js");

pub struct TwinPanel {
    pub shared: Arc<Shared>,
}

const JSON: &str = "application/json; charset=utf-8";

impl Extra for TwinPanel {
    fn get(&self, path: &str) -> Option<(&'static str, Vec<u8>)> {
        Some(match path {
            "/gemeo" | "/gemeo/" => ("text/html; charset=utf-8", PAGE.as_bytes().to_vec()),
            "/gemeo/previsao" | "/gemeo/previsao/" => (
                "text/html; charset=utf-8",
                FORECAST_PAGE.as_bytes().to_vec(),
            ),
            "/gemeo.js" => ("text/javascript; charset=utf-8", SCRIPT.as_bytes().to_vec()),
            "/previsao.js" => (
                "text/javascript; charset=utf-8",
                FORECAST_SCRIPT.as_bytes().to_vec(),
            ),
            "/gemeo.css" => ("text/css; charset=utf-8", STYLE.as_bytes().to_vec()),
            "/api/gemeo" => (JSON, self.shared.json(false).to_string().into_bytes()),
            "/api/gemeo/estado" => (JSON, self.shared.json(true).to_string().into_bytes()),
            "/api/gemeo/estado.ron" => {
                let (state, _) = self.shared.twin.lock().ok()?.initial_state();
                ("text/plain; charset=utf-8", state.to_ron().into_bytes())
            }
            "/api/gemeo/previsao" => {
                let twin = self.shared.twin.lock().ok()?;
                let mut v = serde_json::to_value(twin.forecast_status()).unwrap_or_default();
                v["tempo"] = serde_json::json!(twin.now());
                v["modelo"] = serde_json::json!(twin.model().name);
                (JSON, v.to_string().into_bytes())
            }
            "/api/gemeo/previsoes" => {
                let twin = self.shared.twin.lock().ok()?;
                (
                    JSON,
                    serde_json::to_vec(twin.forecasts()).unwrap_or_default(),
                )
            }
            _ => return None,
        })
    }

    fn post(&self, path: &str) -> Option<(&'static str, Vec<u8>)> {
        if path != "/api/gemeo/previsao" {
            return None;
        }
        let mut twin = self.shared.twin.lock().ok()?;
        let ok = twin.request_forecast();
        let body = if ok {
            serde_json::json!({"ok": true, "instante": twin.now()})
        } else {
            serde_json::json!({"ok": false, "erro": "previsão desligada (use --previsao)"})
        };
        Some((JSON, body.to_string().into_bytes()))
    }
}
