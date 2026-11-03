use std::path::{Path, PathBuf};
use std::time::Duration;

use utsim_dash::analyses::ScenarioSet;
use utsim_dash::{Dashboard, Server, Study, page};
use utsim_model::Model;
use utsim_model::experiment::Experiment;

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
    let (cfg, origin, cfg_path) = match &a.config {
        Some(path) => (
            Dashboard::load(path)?,
            path.display().to_string(),
            Some(path.clone()),
        ),
        None => {
            let next = Dashboard::default_path(&a.model);
            if next.is_file() {
                (
                    Dashboard::load(&next)?,
                    next.display().to_string(),
                    Some(next),
                )
            } else {
                (Dashboard::default(), "padrão automático".to_string(), None)
            }
        }
    };
    eprintln!("Configuração do painel: {origin}");
    let scenarios = match &cfg.scenarios {
        Some(file) => {
            // Relative to the dashboard file, or to the model without one.
            let base = cfg_path.as_deref().unwrap_or(&a.model);
            let path = base.parent().unwrap_or(Path::new(".")).join(file);
            let exp = Experiment::load(&path)
                .map_err(|e| format!("cenários do painel ({}): {e}", path.display()))?;
            eprintln!(
                "Cenários: {} ({} cenários de {})",
                exp.name,
                exp.scenarios.len(),
                path.display()
            );
            Some(ScenarioSet {
                name: exp.name,
                scenarios: exp.scenarios,
                measures: exp.measures,
            })
        }
        None => None,
    };

    if let Some(dir) = &a.export {
        let study = Study::run_with(model, cfg, a.reps, &origin, scenarios)?;
        let package = study.package();
        let files = page::export_folder(dir, &package)?;
        eprintln!(
            "Painel exportado em {} ({})",
            dir.display(),
            files.join(", ")
        );
        return Ok(());
    }

    let study = Study::start_with(model, cfg, a.reps, &origin, scenarios)?;
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
    let mut stage = "";
    while !study.progress().concluido {
        let p = study.progress();
        if p.feitas != shown {
            shown = p.feitas;
            eprintln!("replicações: {} de {}", p.feitas, p.total);
        }
        if p.etapa != stage {
            stage = p.etapa;
            if !stage.is_empty() {
                eprintln!("análises: {stage}");
            }
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
