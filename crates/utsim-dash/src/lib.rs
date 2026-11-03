//! Web dashboard for managers and engineers (`utsim painel`).
//!
//! The program serves a page on `127.0.0.1` and a read-only JSON API; the page draws the
//! numbers (KPIs with confidence intervals and P5-P95 ranges, traffic lights by the
//! probability of meeting the goal, distributions, series, Gantt charts) with its own SVG
//! charts, with no external library and no network. All calculation is in Rust:
//!
//! - [`config`]: the optional `<model>.painel.ron` file (goals, cards, thresholds);
//! - [`kpi`]: the KPIs and their formulas, intervals and traffic lights;
//! - [`data`]: one JSON per API route (also embedded in the static export);
//! - [`study`]: runs the replications in the background, publishing partial results;
//! - [`server`]: the HTTP server on `std::net`;
//! - [`export`] and [`page`]: CSV, JSON, `.xlsx` and the frozen HTML page.
//!
//! See `docs/manual_painel.md` for the user's guide.

pub mod config;
pub mod data;
pub mod export;
pub mod kpi;
pub mod num;
pub mod page;
pub mod server;
pub mod study;

pub use config::Dashboard;
pub use data::Package;
pub use server::Server;
pub use study::Study;
