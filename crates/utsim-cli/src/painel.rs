use std::path::PathBuf;
use std::time::Duration;

use utsim_dash::{Dashboard, Server, Study, page};
use utsim_model::Model;

pub struct Args {
    pub model: PathBuf,
    pub port: u16,
    pub config: Option<PathBuf>,
    pub reps: Option<u32>,
    pub export: Option<PathBuf>,
    pub open: bool,
}

/// `utsim painel`: runs the replications and serves the dashboard on `127.0.0.1`, or,
/// with `--exportar`, writes the frozen page into a folder and exits.
pub fn run(a: Args) -> Result<(), Box<dyn std::error::Error>> {
    let model = Model::load(&a.model)?;
    let (cfg, origin) = match &a.config {
        Some(path) => (Dashboard::load(path)?, path.display().to_string()),
        None => {
            let next = Dashboard::default_path(&a.model);
            if next.is_file() {
                (Dashboard::load(&next)?, next.display().to_string())
            } else {
                (Dashboard::default(), "padrão automático".to_string())
            }
        }
    };
    eprintln!("Configuração do painel: {origin}");

    if let Some(dir) = &a.export {
        let study = Study::run(model, cfg, a.reps, &origin)?;
        let package = study.package();
        let files = page::export_folder(dir, &package)?;
        eprintln!(
            "Painel exportado em {} ({})",
            dir.display(),
            files.join(", ")
        );
        return Ok(());
    }

    let study = Study::start(model, cfg, a.reps, &origin)?;
    let server = Server::start(study.clone(), a.port)
        .map_err(|e| format!("não foi possível escutar na porta {}: {e}", a.port))?;
    println!(
        "Painel em {} (só este computador; Ctrl+C encerra)",
        server.url()
    );
    if a.open {
        open_browser(&server.url());
    }
    let mut shown = u32::MAX;
    while !study.progress().concluido {
        let p = study.progress();
        if p.feitas != shown {
            shown = p.feitas;
            eprintln!("replicações: {} de {}", p.feitas, p.total);
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    if let Some(e) = study.error() {
        return Err(e.into());
    }
    eprintln!("Estudo concluído; o painel segue no ar.");
    loop {
        std::thread::sleep(Duration::from_secs(3600));
    }
}

/// Opens the system browser (only with `--abrir`).
fn open_browser(url: &str) {
    #[cfg(windows)]
    let result = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn();
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(url).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let result = std::process::Command::new("xdg-open").arg(url).spawn();
    if result.is_err() {
        eprintln!("não foi possível abrir o navegador; abra {url}");
    }
}
