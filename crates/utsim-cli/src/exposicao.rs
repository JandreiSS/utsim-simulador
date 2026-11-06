//! `utsim exposicao`: time of people inside the risk zones, entries without the required
//! protective equipment and simulated near-conflicts between people and vehicles
//! (`utsim_exposicao`).

use std::path::PathBuf;

use clap::Parser;
use utsim_exposicao::{Options, analyze_all, interval, relatorio};
use utsim_model::Model;

#[derive(Parser)]
#[command(
    name = "utsim exposicao",
    about = "Exposição a riscos na simulação: tempo em zona, entradas sem EPI e quase-conflitos simulados"
)]
struct Args {
    /// Modelo (.ron) com `zones` e `ppe`
    model: PathBuf,
    /// Número de replicações (sobrescreve run.replications)
    #[arg(short = 'n', long)]
    reps: Option<u32>,
    /// Distância de quase-conflito entre pessoa e veículo (m)
    #[arg(long, default_value_t = utsim_exposicao::NEAR_MISS_DISTANCE)]
    limiar: f64,
    /// Passo de amostragem da proximidade e do mapa de calor (unidade de tempo do modelo)
    #[arg(long, default_value_t = 0.1)]
    passo: f64,
    /// Lado da célula do mapa de calor (m)
    #[arg(long, default_value_t = 1.0)]
    celula: f64,
    /// Grava o relatório em HTML (com o aviso legal)
    #[arg(long)]
    html: Option<PathBuf>,
}

pub fn run_args(args: Vec<String>) -> Result<(), String> {
    let a = Args::try_parse_from(std::iter::once("utsim exposicao".to_string()).chain(args))
        .unwrap_or_else(|e| e.exit());
    let mut model = Model::load(&a.model).map_err(|e| e.to_string())?;
    if let Some(n) = a.reps {
        model.run.replications = n;
    }
    let opts = Options {
        near_miss: a.limiar,
        step: a.passo,
        cell: a.celula,
    };
    let reps = analyze_all(&model, opts).map_err(|e| e.to_string())?;
    println!("Exposição (origem: Simulado), {} replicações", reps.len());
    println!("{}", relatorio::AVISO_LEGAL);
    if let Some(e) = reps.first() {
        for z in &e.zones {
            let t: Vec<f64> = reps
                .iter()
                .flat_map(|r| {
                    r.zones
                        .iter()
                        .filter(|x| x.zone == z.zone && x.person == z.person)
                })
                .map(|x| x.time)
                .collect();
            let i = interval(&t);
            println!(
                "  {} / {}: tempo {:.1} ± {:.1}, entradas sem EPI (rep. 1) {}{}",
                z.zone,
                z.person,
                i.mean,
                i.half_width,
                z.entries_without_ppe,
                if z.missing_ppe.is_empty() {
                    String::new()
                } else {
                    format!(" (falta {})", z.missing_ppe.join(", "))
                }
            );
        }
        for w in &e.warnings {
            println!("  aviso: {w}");
        }
    }
    let n: Vec<f64> = reps.iter().map(|e| e.near_miss_count() as f64).collect();
    let i = interval(&n);
    println!(
        "Quase-conflitos simulados por replicação (< {} m): {:.1} ± {:.1}",
        a.limiar, i.mean, i.half_width
    );
    println!("{}", relatorio::AVISO_SIMULADO);
    if let Some(p) = a.html {
        std::fs::write(&p, relatorio::html(&model, &reps)).map_err(|e| e.to_string())?;
        println!("Relatório: {}", p.display());
    }
    Ok(())
}
