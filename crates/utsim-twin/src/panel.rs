//! The twin on the dashboard: routes served by `utsim-dash` next to its own (through
//! [`utsim_dash::server::Extra`]):
//!
//! | route | content |
//! |---|---|
//! | `/gemeo` | page "Gêmeo (sombra)": connections, alerts, KPIs real × simulated, objects |
//! | `/gemeo.js`, `/gemeo.css` | its script and style (the dashboard's policy forbids inline scripts) |
//! | `/api/gemeo` | the [`TwinReport`](crate::twin::TwinReport) plus the health of each connection |
//! | `/api/gemeo/estado` | the same with the observed state as an initial state (slower) |
//! | `/api/gemeo/estado.ron` | the observed state as an initial state file |

use std::sync::Arc;

use utsim_dash::server::Extra;

use crate::live::Shared;

pub const PAGE: &str = include_str!("../web/gemeo.html");
pub const SCRIPT: &str = include_str!("../web/gemeo.js");
pub const STYLE: &str = include_str!("../web/gemeo.css");

pub struct TwinPanel {
    pub shared: Arc<Shared>,
}

impl Extra for TwinPanel {
    fn get(&self, path: &str) -> Option<(&'static str, Vec<u8>)> {
        let json = "application/json; charset=utf-8";
        Some(match path {
            "/gemeo" | "/gemeo/" => ("text/html; charset=utf-8", PAGE.as_bytes().to_vec()),
            "/gemeo.js" => ("text/javascript; charset=utf-8", SCRIPT.as_bytes().to_vec()),
            "/gemeo.css" => ("text/css; charset=utf-8", STYLE.as_bytes().to_vec()),
            "/api/gemeo" => (json, self.shared.json(false).to_string().into_bytes()),
            "/api/gemeo/estado" => (json, self.shared.json(true).to_string().into_bytes()),
            "/api/gemeo/estado.ron" => {
                let (state, _) = self.shared.twin.lock().ok()?.initial_state();
                ("text/plain; charset=utf-8", state.to_ron().into_bytes())
            }
            _ => return None,
        })
    }
}
