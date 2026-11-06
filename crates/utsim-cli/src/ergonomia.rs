//! `utsim ergonomia`: the project's own ergonomics grade per element and per station, on
//! the time study of a model (`utsim_seguranca_core::ergonomia`).

use std::path::PathBuf;

use clap::Parser;
use utsim_diagram::{rastro, tempos};
use utsim_model::Model;
use utsim_seguranca_core::ergonomia::{self, AVISO_NR17, Metodo};

#[derive(Parser)]
#[command(
    name = "utsim ergonomia",
    about = "Nota de ergonomia própria por elemento e por posto, sobre o estudo de tempos do modelo"
)]
struct Args {
    /// Modelo (.ron)
    model: PathBuf,
    /// Marcações por elemento em CSV: ator;elemento;tronco;bracos;pescoco;pernas;carga_kg;pega;vibracao
    #[arg(long)]
    marcacoes: Option<PathBuf>,
    /// Modelo depois da mudança de método: compara nota e tempo padrão
    #[arg(long)]
    comparar: Option<PathBuf>,
    /// Marcações do modelo depois da mudança (padrão: as mesmas)
    #[arg(long)]
    marcacoes_depois: Option<PathBuf>,
    /// Método da empresa em CSV (pesos e faixas)
    #[arg(long)]
    metodo: Option<PathBuf>,
    /// Unidade de tempo do modelo: s, min ou h
    #[arg(long, default_value = "s")]
    unidade: String,
    /// Grava o relatório em HTML
    #[arg(long)]
    html: Option<PathBuf>,
}

fn ler(p: &PathBuf) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}

fn estudo(p: &PathBuf, unidade: &str) -> Result<(Model, tempos::Estudo), String> {
    let model = Model::load(p).map_err(|e| e.to_string())?;
    let r = rastro::capturar(&model, 0)?;
    let opcoes = tempos::Opcoes {
        unidade: unidade.to_string(),
        ..tempos::Opcoes::default()
    };
    let e = tempos::estudar(&model, &r, &opcoes)?;
    Ok((model, e))
}

pub fn run_args(args: Vec<String>) -> Result<(), String> {
    let a = Args::try_parse_from(std::iter::once("utsim ergonomia".to_string()).chain(args))
        .unwrap_or_else(|e| e.exit());
    let metodo = match &a.metodo {
        Some(p) => Metodo::de_csv(&ler(p)?)?,
        None => Metodo::default(),
    };
    let marcas = match &a.marcacoes {
        Some(p) => ergonomia::marcacoes_csv(&ler(p)?)?,
        None => Vec::new(),
    };
    let (model, antes) = estudo(&a.model, &a.unidade)?;
    let postos = ergonomia::avaliar(&antes, &marcas, &metodo);
    println!(
        "Ergonomia de {} ({}), origem: Simulado",
        model.name, metodo.nome
    );
    println!("{AVISO_NR17}");
    for p in &postos {
        println!(
            "  {}: nota {:.2} ({}), média ponderada {:.2}",
            p.ator, p.nota, p.faixa, p.media_ponderada
        );
        for e in &p.elementos {
            println!(
                "    {:<28} {:>8.1}/h  fração {:.2}  nota {:.2} {}{}",
                e.elemento,
                e.por_hora,
                e.fracao,
                e.nota,
                e.faixa,
                if e.marcado { "" } else { " (não marcado)" }
            );
        }
    }
    let mut comparacao = Vec::new();
    if let Some(p) = &a.comparar {
        let marcas_d = match &a.marcacoes_depois {
            Some(p) => ergonomia::marcacoes_csv(&ler(p)?)?,
            None => marcas.clone(),
        };
        let (_, depois) = estudo(p, &a.unidade)?;
        let d = ergonomia::avaliar(&depois, &marcas_d, &metodo);
        comparacao = ergonomia::comparar(&postos, &d);
        println!("Antes e depois:");
        for c in &comparacao {
            println!(
                "  {}: nota {:.2} -> {:.2}, tempo padrão {:?} -> {:?}",
                c.ator, c.nota_antes, c.nota_depois, c.padrao_antes, c.padrao_depois
            );
        }
    }
    if let Some(p) = &a.html {
        std::fs::write(
            p,
            ergonomia::html(&model.name, &metodo, &postos, &comparacao),
        )
        .map_err(|e| e.to_string())?;
        println!("Relatório: {}", p.display());
    }
    Ok(())
}
