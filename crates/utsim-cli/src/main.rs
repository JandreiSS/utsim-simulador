mod cena;
mod emulate;
mod experiment;
mod fit;
mod gerar;
mod optimize;
mod replay;

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
    /// Grava o rastro completo de uma replicação para o visualizador 3D
    Replay {
        model: PathBuf,
        /// Arquivo JSON de saída
        #[arg(short, long, default_value = "replay.json")]
        out: PathBuf,
        #[arg(long, default_value_t = 0)]
        rep: u32,
    },
    /// Grava a cena 3D de uma replicação (protocolo utsim-scene/1, JSON em linhas)
    Cena {
        model: PathBuf,
        #[arg(long, default_value_t = 0)]
        rep: u32,
        /// Quadros por unidade de tempo simulado
        #[arg(long, default_value_t = 10.0)]
        fps: f64,
        /// Último instante gravado (padrão: o fim da rodada)
        #[arg(long)]
        ate: Option<f64>,
        /// Arquivo de saída (padrão: a saída padrão)
        #[arg(long)]
        saida: Option<PathBuf>,
    },
    /// Gera um modelo sintético de qualquer tamanho (testes de desempenho e de escala)
    GerarModelo {
        /// Número de objetos do modelo (no mínimo 4)
        #[arg(long, default_value_t = 100)]
        objetos: usize,
        /// Forma do modelo: linha, rede ou armazem
        #[arg(long, default_value = "linha")]
        tipo: String,
        /// Itens que a fonte cria (exatamente)
        #[arg(long, default_value_t = 1000)]
        itens: u64,
        /// Arquivo de saída (padrão: a saída padrão)
        #[arg(long)]
        saida: Option<PathBuf>,
    },
    /// Compara cenários de um arquivo de experimento
    Experiment {
        file: PathBuf,
        /// Grava o resultado completo em JSON
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Otimiza parâmetros do modelo por simulação (busca com comparação pareada)
    Otimizar {
        problema: PathBuf,
        /// Grava o relatório completo, com gráficos, em um HTML
        #[arg(long)]
        html: Option<PathBuf>,
        /// Grava o resultado completo em JSON
        #[arg(long)]
        json: Option<PathBuf>,
        /// Replicações por ponto (sobrescreve o arquivo do problema)
        #[arg(long)]
        reps: Option<u32>,
        /// Máximo de pontos avaliados (sobrescreve o arquivo do problema)
        #[arg(long)]
        orcamento: Option<usize>,
    },
    /// Emula o modelo em tempo real, com servidor OPC UA (comissionamento virtual)
    Emular {
        model: PathBuf,
        /// Porta TCP do servidor OPC UA (0 escolhe uma livre)
        #[arg(long, default_value_t = 4840)]
        porta: u16,
        /// Fator de velocidade: 1.0 = tempo real, 10.0 = dez vezes mais rápido
        #[arg(long, default_value_t = 1.0)]
        velocidade: f64,
        /// Tempo simulado em que a emulação termina (padrão: run.stop_time)
        #[arg(long)]
        ate: Option<f64>,
        /// Grava os comandos recebidos (tempo, variável, valor) neste arquivo RON
        #[arg(long)]
        gravar: Option<PathBuf>,
        /// Reaplica os comandos de um arquivo RON, sem OPC UA e sem esperar o relógio
        #[arg(long)]
        reproduzir: Option<PathBuf>,
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
        Cmd::Otimizar {
            problema,
            html,
            json,
            reps,
            orcamento,
        } => optimize::run(optimize::Args {
            file: problema,
            html,
            json,
            reps,
            budget: orcamento,
        })?,
        Cmd::Emular {
            model,
            porta,
            velocidade,
            ate,
            gravar,
            reproduzir,
        } => emulate::run(emulate::Args {
            model,
            port: porta,
            speed: velocidade,
            until: ate,
            record: gravar,
            replay: reproduzir,
        })?,
        Cmd::Check { model } => {
            let m = Model::load(&model)?;
            m.validate()?;
            print_warnings(&m);
            println!(
                "{}: ok ({} objetos, {} conexões)",
                m.name,
                m.objects.len(),
                m.connections.len()
            );
        }
        Cmd::Experiment { file, json } => experiment::run(&file, json.as_deref())?,
        Cmd::GerarModelo {
            objetos,
            tipo,
            itens,
            saida,
        } => gerar::run(&tipo, objetos, itens, saida.as_deref())?,
        Cmd::Replay { model, out, rep } => replay::run(&model, rep, &out)?,
        Cmd::Cena {
            model,
            rep,
            fps,
            ate,
            saida,
        } => cena::run(&model, rep, fps, ate, saida.as_deref())?,
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
            m.validate()?;
            print_warnings(&m);
            let started = std::time::Instant::now();
            let reports = m.run_replications()?;
            let elapsed = started.elapsed();
            print_summary(&m, &reports);
            let events: u64 = reports.iter().map(|r| r.events).sum();
            println!("\n{events} eventos em {:.2?}", elapsed);
            if let Some(dir) = csv {
                let files = utsim_model::csv::export(&dir, &reports)?;
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

/// Non-fatal model warnings, on stderr so they never mix with results.
fn print_warnings(model: &Model) {
    for warning in model.warnings() {
        eprintln!("aviso: {warning}");
    }
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
        // Metrics computed by the block itself (e.g. tank levels of a fluid network).
        let custom: Vec<String> = b
            .custom
            .keys()
            .map(|k| format!("{k} {}", cell(format!("{n}.{k}"))))
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
        // One per line: a fluid network may have several tanks.
        for m in custom {
            println!("{n:<14} {m}");
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
