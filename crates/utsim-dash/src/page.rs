//! The page: HTML, CSS and JavaScript embedded in the binary, served as they are by the
//! server or inlined, with the data, in a single frozen file for the static export.

use std::fs;
use std::io;
use std::path::Path;

use crate::data::Package;
use crate::export;

pub const CSS: &str = include_str!("../web/app.css");
pub const CHARTS_JS: &str = include_str!("../web/charts.js");
pub const APP_JS: &str = include_str!("../web/app.js");
const INDEX: &str = include_str!("../web/index.html");

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn fill(title: &str, head: &str, scripts: &str) -> String {
    INDEX
        .replace("{{TITLE}}", &escape_html(title))
        .replace("{{HEAD}}", head)
        .replace("{{SCRIPTS}}", scripts)
}

/// The page the server serves: the style and the scripts come as separate files.
pub fn live_html() -> String {
    fill(
        "Painel UTSim",
        "<link rel=\"stylesheet\" href=\"/app.css\">",
        "<script src=\"/charts.js\"></script>\n<script src=\"/app.js\"></script>",
    )
}

/// JSON safe to put inside a `<script>` element.
fn script_json(value: &impl serde::Serialize) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "null".to_string())
        .replace("</", "<\\/")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

/// The frozen page: one self-contained file with the data embedded, which opens without
/// a server and without internet.
pub fn static_html(package: &Package) -> String {
    let head = format!("<style>\n{CSS}\n</style>");
    let scripts = format!(
        "<script>window.UTSIM_DADOS = {};</script>\n<script>\n{CHARTS_JS}\n</script>\n<script>\n{APP_JS}\n</script>",
        script_json(&package.routes)
    );
    fill(
        &format!("{} · Painel UTSim", package.title),
        &head,
        &scripts,
    )
}

/// Writes the static export into `dir`: `index.html` plus the CSV, JSON and (when the
/// build has it) `.xlsx` files that the page's download buttons point to. Returns the
/// names of the files written.
pub fn export_folder(dir: &Path, package: &Package) -> io::Result<Vec<String>> {
    fs::create_dir_all(dir)?;
    let mut written = Vec::new();
    let mut put = |name: &str, bytes: &[u8]| -> io::Result<()> {
        fs::write(dir.join(name), bytes)?;
        written.push(name.to_string());
        Ok(())
    };
    put("index.html", static_html(package).as_bytes())?;
    put("kpis.csv", export::kpis_csv(package).as_bytes())?;
    put("objetos.csv", export::objects_csv(package).as_bytes())?;
    put(
        "replicacoes.csv",
        export::replications_csv(package).as_bytes(),
    )?;
    put("painel.json", export::json(package).as_bytes())?;
    if let Ok(bytes) = export::xlsx(package) {
        put("painel.xlsx", &bytes)?;
    }
    Ok(written)
}
