//! `utsim tempos`: time and method study of man-machine work (cycle chart, element times,
//! machines per operator, standard time, before/after comparison), from the trace of one
//! replication (`utsim_diagram::tempos`).

use std::path::PathBuf;

use clap::Parser;
use utsim_diagram::rastro;
use utsim_diagram::tempos::{self, Opcoes};
use utsim_model::Model;

/// The `tempos` command line, parsed on its own (as `qualidade`, to keep the main parser
/// small).
#[derive(Parser, Clone)]
#[command(
    name = "utsim tempos",
    about = "Estudo de tempos e métodos (homem-máquina): ciclo, elementos, máquinas por \
             operador, tempo padrão e comparação de métodos"
)]
struct Args {
    /// Modelo (.ron)
    model: PathBuf,
    /// Operador (ou máquina) estudado; sem ele, todos os operadores que trabalham em máquinas
    #[arg(long)]
    ator: Option<String>,
    /// Replicação gravada (a primeira é a 0)
    #[arg(long, default_value_t = 0)]
    replica: u32,
    /// Início do intervalo analisado (padrão: fim do aquecimento)
    #[arg(long, requires = "ate")]
    de: Option<f64>,
    /// Fim do intervalo analisado (padrão: fim da rodada)
    #[arg(long, requires = "de")]
    ate: Option<f64>,
    /// Ciclo desenhado, numerado a partir de 1 (padrão: o de duração mediana)
    #[arg(long)]
    ciclo: Option<usize>,
    /// Ritmo (avaliação de desempenho) em %, 100 = normal
    #[arg(long, default_value_t = 100.0)]
    ritmo: f64,
    /// Tolerâncias (pessoais, fadiga e esperas) em % do tempo normal; padrão: a `Allowance`
    /// do perfil do operador, se houver, senão 0
    #[arg(long)]
    tolerancia: Option<f64>,
    /// Custo do operador por hora (com --custo-maquina, escolhe o número de máquinas)
    #[arg(long, requires = "custo_maquina")]
    custo_operador: Option<f64>,
    /// Custo de uma máquina por hora
    #[arg(long, requires = "custo_operador")]
    custo_maquina: Option<f64>,
    /// Unidade de tempo do modelo: s, min ou h
    #[arg(long, default_value = "s")]
    unidade: String,
    /// Segundo modelo (o "depois", método novo) para comparar elemento a elemento
    #[arg(long)]
    comparar: Option<PathBuf>,
    /// Grava o relatório completo (gráfico do ciclo, tabelas, comparação) em HTML
    #[arg(long)]
    html: Option<PathBuf>,
    /// Grava o gráfico homem-máquina do ciclo representativo (primeiro grupo) em SVG
    #[arg(long)]
    svg: Option<PathBuf>,
    /// Grava a tabela de elementos em CSV (com --comparar, também <nome>-comparacao.csv)
    #[arg(long)]
    csv: Option<PathBuf>,
    /// Grava o estudo (e a comparação) em JSON
    #[arg(long)]
    json: Option<PathBuf>,
}

/// Parses the arguments after `utsim tempos` and runs the study.
pub fn run_args(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let a = Args::try_parse_from(std::iter::once("utsim tempos".to_string()).chain(args))
        .unwrap_or_else(|e| e.exit());
    run(a)
}

fn estudo(path: &PathBuf, a: &Args) -> Result<(Model, tempos::Estudo, String), String> {
    let model = Model::load(path).map_err(|e| format!("{}: {e}", path.display()))?;
    model.validate().map_err(|e| e.to_string())?;
    let r = rastro::capturar(&model, a.replica)?;
    let opcoes = Opcoes {
        ator: a.ator.clone(),
        janela: a.de.zip(a.ate),
        ciclo: a.ciclo,
        ritmo: a.ritmo / 100.0,
        tolerancia: a.tolerancia.map(|t| t / 100.0),
        custos: a.custo_operador.zip(a.custo_maquina),
        unidade: a.unidade.clone(),
        ..Opcoes::default()
    };
    let e = tempos::estudar(&model, &r, &opcoes)?;
    let origem = r.origem(&model);
    Ok((model, e, origem))
}

fn run(a: Args) -> Result<(), Box<dyn std::error::Error>> {
    let (_, antes, mut origem) = estudo(&a.model, &a)?;
    let mut depois = None;
    if let Some(p) = &a.comparar {
        // The "after" model may name its operator differently: study all of it.
        let outro = Args {
            ator: a
                .ator
                .clone()
                .filter(|n| Model::load(p).is_ok_and(|m| m.objects.iter().any(|o| o.name == *n))),
            ciclo: None,
            ..a.clone()
        };
        let (_, d, o2) = estudo(p, &outro)?;
        origem = format!("antes: {origem} Depois: {o2}");
        depois = Some(d);
    }
    let comparacao = match &depois {
        Some(d) => Some(tempos::comparar(&antes, d)?),
        None => None,
    };
    print!("{}", tempos::texto(&antes, comparacao.as_ref()));
    let write = |p: &PathBuf, s: &str| -> std::io::Result<()> {
        std::fs::write(p, s)?;
        println!("gravado em {}", p.display());
        Ok(())
    };
    if let Some(p) = &a.html {
        write(p, &tempos::html(&antes, comparacao.as_ref(), &origem))?;
    }
    if let Some(p) = &a.svg {
        match antes
            .grupos
            .first()
            .and_then(|g| tempos::svg(g, &antes.unidade, true))
        {
            Some(s) => write(p, &s)?,
            None => eprintln!("aviso: sem ciclo representativo para desenhar"),
        }
    }
    if let Some(p) = &a.csv {
        write(p, &tempos::csv_elementos(&antes))?;
        if let Some(c) = &comparacao {
            let stem = p
                .file_stem()
                .map_or("tempos".into(), |s| s.to_string_lossy().to_string());
            let q = p.with_file_name(format!("{stem}-comparacao.csv"));
            write(&q, &tempos::csv_comparacao(c))?;
        }
    }
    if let Some(p) = &a.json {
        let v = serde_json::json!({
            "estudo": antes,
            "depois": depois,
            "comparacao": comparacao,
        });
        write(p, &serde_json::to_string_pretty(&v)?)?;
    }
    Ok(())
}
