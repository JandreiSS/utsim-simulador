//! `utsim fit`: fits candidate distributions to a data file and prints the report.

use std::path::{Path, PathBuf};

use utsim_fit::auto::{self, AutoConfig, Choice, Criterion};
use utsim_fit::data::{self, DataOptions};
use utsim_fit::report::{FitConfig, analyze};

pub struct Args {
    pub file: PathBuf,
    pub column: Option<String>,
    pub timestamps: bool,
    pub alpha: f64,
    pub bootstrap: usize,
    pub seed: u64,
    pub html: Option<PathBuf>,
    pub json: Option<PathBuf>,
    /// Report of the automatic choice (same page as `utsim dados`), plus a CSV beside it.
    pub report: Option<PathBuf>,
    pub criterion: String,
}

fn criterion(text: &str) -> Result<Criterion, String> {
    match text.to_lowercase().as_str() {
        "aic" => Ok(Criterion::Aic),
        "bic" => Ok(Criterion::Bic),
        "ks" => Ok(Criterion::Ks),
        "ad" => Ok(Criterion::Ad),
        other => Err(format!(
            "--criterio deve ser aic, bic, ks ou ad (recebido '{other}')"
        )),
    }
}

pub fn run(args: Args) -> Result<(), String> {
    let crit = criterion(&args.criterion)?;
    if !(args.alpha > 0.0 && args.alpha < 1.0) {
        return Err(format!(
            "--alpha deve estar entre 0 e 1 (recebido {})",
            args.alpha
        ));
    }
    let options = DataOptions {
        column: args.column.clone(),
        timestamps: args.timestamps,
    };
    let loaded = data::load(&args.file, &options)?;
    for w in &loaded.warnings {
        eprintln!("aviso: {w}");
    }
    for n in &loaded.notes {
        eprintln!("coluna: {n}");
    }
    let mut config = FitConfig {
        alpha: args.alpha,
        ..FitConfig::default()
    };
    config.gof.bootstrap = args.bootstrap;
    config.gof.seed = args.seed;

    let name = args.file.file_name().map_or_else(
        || args.file.display().to_string(),
        |n| n.to_string_lossy().into(),
    );
    let what = if args.timestamps {
        format!("intervalos da {}", loaded.column)
    } else {
        loaded.column.clone()
    };
    let dataset = format!("{name} ({what})");
    let report = analyze(&dataset, &loaded.values, &config);
    println!("{}", report.to_text());

    if let Some(path) = &args.html {
        write(path, &report.to_html())?;
        println!("relatório HTML gravado em {}", path.display());
    }
    if let Some(path) = &args.json {
        write(path, &report.to_json())?;
        println!("relatório JSON gravado em {}", path.display());
    }
    if let Some(path) = &args.report {
        let auto_config = AutoConfig {
            choose: Choice::Best,
            criterion: crit,
            fit: config,
        };
        let fit = auto::auto_fit(&dataset, &loaded.values, &auto_config)?;
        write(path, &fit.to_html("utsim fit"))?;
        let csv = path.with_extension("csv");
        write(&csv, &fit.to_csv())?;
        println!(
            "escolha automática ({}): {}
relatório gravado em {} (candidatas em {})",
            crit.label(),
            fit.ron(),
            path.display(),
            csv.display()
        );
    }
    Ok(())
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text)
        .map_err(|e| format!("não foi possível gravar '{}': {e}", path.display()))
}
