mod csv;
mod experiment;
mod fit;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use utsim_core::stats::Summary;
use utsim_core::{RunReport, summarize};
use utsim_model::Model;

#[derive(Parser)]
#[command(
    name = "utsim",
    version,
    about = "UTSim: simulador de eventos discretos"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Executa um modelo e mostra as estatísticas
    Run {
        model: PathBuf,
        /// Número de replicações (sobrescreve run.replications)
        #[arg(short = 'n', long)]
        reps: Option<u32>,
        #[arg(long)]
        seed: Option<u64>,
        /// Tempo de parada (sobrescreve run.stop_time)
        #[arg(long)]
        stop: Option<f64>,
        /// Tempo de aquecimento (sobrescreve run.warmup)
        #[arg(long)]
        warmup: Option<f64>,
        /// Grava os relatórios completos em JSON
        #[arg(long)]
        json: Option<PathBuf>,
        /// Grava resumo, replicações, séries e histogramas em CSV neste diretório
        #[arg(long)]
        csv: Option<PathBuf>,
        /// Intervalo de amostragem das séries temporais (sobrescreve run.sample_interval)
        #[arg(long)]
        sample: Option<f64>,
        /// Número de classes dos histogramas (sobrescreve run.histogram_bins)
        #[arg(long)]
        bins: Option<usize>,
    },
    /// Valida um modelo sem executar
    Check { model: PathBuf },
    /// Compara cenários de um arquivo de experimento
    Experiment {
        file: PathBuf,
        /// Grava o resultado completo em JSON
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Ajusta distribuições a dados reais (CSV/TXT) e justifica a escolha
    Fit {
        file: PathBuf,
        /// Coluna: nome do cabeçalho ou posição (a partir de 1); padrão: primeira numérica
        #[arg(long)]
        coluna: Option<String>,
        /// A coluna traz instantes crescentes; ajusta os intervalos entre eles
        #[arg(long)]
        timestamps: bool,
        /// Nível de significância dos testes
        #[arg(long, default_value_t = 0.05)]
        alpha: f64,
        /// Réplicas do bootstrap paramétrico (p-valores de KS e Anderson-Darling)
        #[arg(long, default_value_t = 300)]
        bootstrap: usize,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Grava o relatório completo, com gráficos, em um HTML
        #[arg(long)]
        html: Option<PathBuf>,
        /// Grava o relatório em JSON
        #[arg(long)]
        json: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("erro: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.cmd {
        Cmd::Fit {
            file,
            coluna,
            timestamps,
            alpha,
            bootstrap,
            seed,
            html,
            json,
        } => fit::run(fit::Args {
            file,
            column: coluna,
            timestamps,
            alpha,
            bootstrap,
            seed,
            html,
            json,
        })?,
        Cmd::Check { model } => {
            let m = Model::load(&model)?;
            m.validate()?;
            println!(
                "{}: ok ({} objetos, {} conexões)",
                m.name,
                m.objects.len(),
                m.connections.len()
            );
        }
        Cmd::Experiment { file, json } => experiment::run(&file, json.as_deref())?,
        Cmd::Run {
            model,
            reps,
            seed,
            stop,
            warmup,
            json,
            csv,
            sample,
            bins,
        } => {
            let mut m = Model::load(&model)?;
            if let Some(v) = reps {
                m.run.replications = v;
            }
            if let Some(v) = seed {
                m.run.seed = v;
            }
            if let Some(v) = stop {
                m.run.stop_time = v;
            }
            if let Some(v) = warmup {
                m.run.warmup = v;
            }
            if let Some(v) = sample {
                m.run.sample_interval = Some(v);
            }
            if let Some(v) = bins {
                m.run.histogram_bins = Some(v);
            }
            let started = std::time::Instant::now();
            let reports = m.run_replications()?;
            let elapsed = started.elapsed();
            print_summary(&m, &reports);
            let events: u64 = reports.iter().map(|r| r.events).sum();
            println!("\n{events} eventos em {:.2?}", elapsed);
            if let Some(dir) = csv {
                let files = csv::export(&dir, &reports)?;
                println!("CSV gravado em {}: {}", dir.display(), files.join(", "));
            }
            if let Some(path) = json {
                let out = serde_json::json!({ "model": m.name, "summary": summarize(&reports), "replications": reports });
                std::fs::write(
                    &path,
                    serde_json::to_string_pretty(&out).expect("report serializes"),
                )?;
                println!("relatório gravado em {}", path.display());
            }
        }
    }
    Ok(())
}

fn print_summary(model: &Model, reports: &[RunReport]) {
    let summary = summarize(reports);
    let first = &reports[0];
    println!(
        "{}: {} replicação(ões), estatísticas de t={} até t={}\n",
        model.name,
        reports.len(),
        first.stats_from,
        first.end_time
    );
    let cell = |key: String| summary.get(&key).map_or_else(|| "-".to_string(), fmt);
    println!(
        "{:<14} {:<10} {:>22} {:>22} {:>22} {:>22}",
        "objeto", "tipo", "entradas", "saídas", "conteúdo médio", "permanência média"
    );
    for b in &first.blocks {
        let n = &b.name;
        println!(
            "{:<14} {:<10} {:>22} {:>22} {:>22} {:>22}",
            n,
            b.kind,
            cell(format!("{n}.input")),
            cell(format!("{n}.output")),
            cell(format!("{n}.content.mean")),
            cell(format!("{n}.staytime.mean")),
        );
    }

    println!();
    for b in &first.blocks {
        let n = &b.name;
        let prefix = format!("{n}.state.");
        let states: Vec<String> = summary
            .iter()
            .filter_map(|(k, s)| k.strip_prefix(&prefix).map(|state| (state, s)))
            .filter(|(_, s)| s.mean > 0.0)
            .map(|(state, s)| format!("{state} {}", fmt_pct(s)))
            .collect();
        let extras: Vec<String> = b
            .extra
            .keys()
            .map(|k| format!("{k} {}", cell(format!("{n}.{k}.mean"))))
            .collect();
        if !states.is_empty() || !extras.is_empty() {
            println!(
                "{n:<14} {}",
                states
                    .into_iter()
                    .chain(extras)
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
        }
    }
}

fn fmt(s: &Summary) -> String {
    if s.half_width.is_nan() {
        format!("{:.4}", s.mean)
    } else {
        format!("{:.4} ± {:.4}", s.mean, s.half_width)
    }
}

fn fmt_pct(s: &Summary) -> String {
    if s.half_width.is_nan() {
        format!("{:.2}%", 100.0 * s.mean)
    } else {
        format!("{:.2}% ± {:.2}", 100.0 * s.mean, 100.0 * s.half_width)
    }
}
