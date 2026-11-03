//! `utsim qualidade`: control charts, capability and measurement system analysis on a CSV
//! file (crate `utsim-quality`; the CSV layouts are in `utsim_quality::data`).

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use utsim_quality::arl::{self, Scheme};
use utsim_quality::capability::{self, Options, Spec, WithinMethod};
use utsim_quality::charts::{self, ChartData, ChartKind, Limits};
use utsim_quality::memory::{Cusum, Ewma};
use utsim_quality::{RuleSet, attribute, data, msa, report};

/// The `qualidade` command line, parsed on its own: as one more subcommand of the main
/// parser, its arguments made the debug build overflow the 1 MB main-thread stack of
/// Windows.
#[derive(Parser)]
#[command(
    name = "utsim qualidade",
    about = "Controle estatístico de processo, capacidade e análise do sistema de medição"
)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

/// Parses the arguments after `utsim qualidade` and runs the command (help and parse
/// errors print and exit, as clap does).
pub fn run_args(args: Vec<String>) -> Result<(), String> {
    let parsed = Args::try_parse_from(std::iter::once("utsim qualidade".to_string()).chain(args))
        .unwrap_or_else(|e| e.exit());
    run(parsed.cmd)
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Carta de controle (I-MR, X̄-R, X̄-S, p, np, c, u, EWMA ou CUSUM)
    Carta {
        arquivo: PathBuf,
        /// imr, xbarr, xbars, p, np, c, u, ewma ou cusum
        #[arg(long, default_value = "imr")]
        tipo: String,
        /// Corta uma coluna de valores em subgrupos consecutivos deste tamanho
        #[arg(long)]
        subgrupo: Option<usize>,
        /// Regras: 1 (só fora dos limites), we (Western Electric), nelson, ou lista como 1,2,5 ou we1,we4
        #[arg(long, default_value = "1")]
        regras: String,
        /// Fase I: subgrupos com causa atribuível fora da estimativa (a partir de 1), ex. 3,7
        #[arg(long)]
        excluir: Option<String>,
        /// Fase I revista: retira os pontos fora dos limites e reestima até não sobrar nenhum
        #[arg(long)]
        revisar: bool,
        /// Fase II: limites congelados de um estudo anterior (JSON de --salvar-limites)
        #[arg(long, conflicts_with_all = ["excluir", "revisar"])]
        limites: Option<PathBuf>,
        /// Grava os limites estimados em JSON (para a fase II)
        #[arg(long)]
        salvar_limites: Option<PathBuf>,
        /// Média conhecida (padrão dado; EWMA e CUSUM: alvo)
        #[arg(long)]
        media: Option<f64>,
        /// σ conhecido do processo (padrão dado; EWMA e CUSUM: σ do valor individual)
        #[arg(long)]
        sigma: Option<f64>,
        /// EWMA: peso λ
        #[arg(long, default_value_t = 0.2)]
        lambda: f64,
        /// EWMA: largura L dos limites em σ
        #[arg(long, default_value_t = 3.0)]
        l: f64,
        /// CUSUM: folga k em σ
        #[arg(long, default_value_t = 0.5)]
        k: f64,
        /// CUSUM: intervalo de decisão h em σ
        #[arg(long, default_value_t = 5.0)]
        h: f64,
        /// Grava o relatório com os gráficos em HTML
        #[arg(long)]
        html: Option<PathBuf>,
        /// Grava a carta em JSON
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Capacidade do processo (Cp, Cpk, Pp, Ppk, Cpm, PPM, nível sigma)
    Capacidade {
        arquivo: PathBuf,
        /// Limite inferior de especificação
        #[arg(long)]
        lie: Option<f64>,
        /// Limite superior de especificação
        #[arg(long)]
        lse: Option<f64>,
        /// Alvo (padrão: meio da especificação)
        #[arg(long)]
        alvo: Option<f64>,
        /// Corta uma coluna de valores em subgrupos consecutivos deste tamanho
        #[arg(long)]
        subgrupo: Option<usize>,
        /// σ dentro: mr (amplitude móvel), rbar (R̄/d2) ou sbar (S̄/c4)
        #[arg(long)]
        sigma_dentro: Option<String>,
        /// normal, boxcox, percentis ou todos
        #[arg(long, default_value = "normal")]
        metodo: String,
        /// Box-Cox: arredonda λ para um valor usual dentro do IC
        #[arg(long)]
        arredondar: bool,
        /// Deslocamento somado ao Z no nível sigma (convenção: 1,5; 0 desliga)
        #[arg(long, default_value_t = 1.5)]
        deslocamento: f64,
        /// Nível de confiança dos intervalos
        #[arg(long, default_value = "95%")]
        confianca: String,
        #[arg(long)]
        html: Option<PathBuf>,
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Análise do sistema de medição (Gage R&R, atributos, viés e linearidade)
    Msa {
        arquivo: PathBuf,
        /// grr (peça, operador, valor), atributos (item, avaliador, resposta[, padrão]),
        /// vies ou linearidade (referência, valor)
        #[arg(long, default_value = "grr")]
        tipo: String,
        /// Tolerância LSE − LIE (para a %tolerância)
        #[arg(long)]
        tolerancia: Option<f64>,
        #[arg(long)]
        lie: Option<f64>,
        #[arg(long)]
        lse: Option<f64>,
        /// Multiplicador de σ da variação do estudo (6 ou 5,15)
        #[arg(long, default_value_t = 6.0)]
        k: f64,
        /// Linearidade: variação do processo (6σ) para a linearidade absoluta
        #[arg(long)]
        variacao_processo: Option<f64>,
        #[arg(long)]
        html: Option<PathBuf>,
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Comprimento médio de corrida (ARL) de um esquema, por simulação
    Arl {
        /// shewhart, we, nelson, ewma ou cusum
        #[arg(long, default_value = "shewhart")]
        tipo: String,
        /// Deslocamento da média em σ (0: alarmes falsos)
        #[arg(long, default_value_t = 0.0)]
        deslocamento: f64,
        #[arg(long, default_value_t = 0.2)]
        lambda: f64,
        #[arg(long, default_value_t = 3.0)]
        l: f64,
        #[arg(long, default_value_t = 0.5)]
        k: f64,
        #[arg(long, default_value_t = 5.0)]
        h: f64,
        /// Corridas simuladas
        #[arg(long, default_value_t = 10_000)]
        corridas: usize,
        #[arg(long, default_value_t = 1)]
        semente: u64,
    },
}

pub fn run(cmd: Cmd) -> Result<(), String> {
    match cmd {
        Cmd::Carta {
            arquivo,
            tipo,
            subgrupo,
            regras,
            excluir,
            revisar,
            limites,
            salvar_limites,
            media,
            sigma,
            lambda,
            l,
            k,
            h,
            html,
            json,
        } => {
            let table = read_table(&arquivo)?;
            let title = file_name(&arquivo);
            let rules = RuleSet::parse(&regras)?;
            let t = tipo.to_lowercase();
            if t == "ewma" || t == "cusum" {
                return memory_chart(
                    &title, &table, subgrupo, &t, media, sigma, lambda, l, k, h, html,
                );
            }
            let kind = ChartKind::parse(&tipo).ok_or_else(|| {
                format!("--tipo '{tipo}' desconhecido: use imr, xbarr, xbars, p, np, c, u, ewma ou cusum")
            })?;
            let chart_data = chart_data(kind, &table, subgrupo)?;
            let excluded = parse_indices(excluir.as_deref(), chart_data.len())?;
            let chart = if let Some(path) = &limites {
                let text = std::fs::read_to_string(path)
                    .map_err(|e| format!("não foi possível ler '{}': {e}", path.display()))?;
                let lim: Limits = serde_json::from_str(&text)
                    .map_err(|e| format!("limites inválidos em '{}': {e}", path.display()))?;
                charts::apply(&lim, &chart_data, &rules, &[])?
            } else if let (Some(mu), Some(s)) = (media, sigma) {
                let lim = standard_limits(kind, &chart_data, mu, s)?;
                charts::apply(&lim, &chart_data, &rules, &excluded)?
            } else if revisar {
                let (chart, removed) = charts::phase1_revised(kind, &chart_data, &rules, 10)?;
                if !removed.is_empty() {
                    eprintln!(
                        "aviso: limites revistos sem os subgrupos {} (confirme a causa atribuível de cada um)",
                        removed
                            .iter()
                            .map(|i| (i + 1).to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                chart
            } else {
                charts::phase1(kind, &chart_data, &excluded, &rules)?
            };
            println!("{}", report::chart_text(&title, &chart));
            if let Some(path) = &salvar_limites {
                let text =
                    serde_json::to_string_pretty(&chart.limits).map_err(|e| e.to_string())?;
                write(path, &text)?;
                println!("limites gravados em {}", path.display());
            }
            if let Some(path) = &html {
                write(path, &report::chart_html(&title, &chart))?;
                println!("relatório HTML gravado em {}", path.display());
            }
            if let Some(path) = &json {
                let text = serde_json::to_string_pretty(&chart).map_err(|e| e.to_string())?;
                write(path, &text)?;
                println!("carta JSON gravada em {}", path.display());
            }
            Ok(())
        }
        Cmd::Capacidade {
            arquivo,
            lie,
            lse,
            alvo,
            subgrupo,
            sigma_dentro,
            metodo,
            arredondar,
            deslocamento,
            confianca,
            html,
            json,
        } => {
            let table = read_table(&arquivo)?;
            let title = file_name(&arquivo);
            let (groups, notes) = data::subgroups(&table, subgrupo)?;
            for n in notes {
                eprintln!("aviso: {n}");
            }
            let level = utsim_stats::parse_level(&confianca)
                .ok_or_else(|| format!("--confianca '{confianca}' inválida (use 95% ou 0,95)"))?;
            let within = match sigma_dentro.as_deref() {
                None => None,
                Some("mr") => Some(WithinMethod::MovingRange),
                Some("rbar") => Some(WithinMethod::RBar),
                Some("sbar") => Some(WithinMethod::SBar),
                Some(o) => {
                    return Err(format!(
                        "--sigma-dentro '{o}' desconhecido: use mr, rbar ou sbar"
                    ));
                }
            };
            let spec = Spec {
                lsl: lie,
                usl: lse,
                target: alvo,
            };
            let options = Options {
                within,
                level,
                sigma_shift: deslocamento,
            };
            let (normal, boxcox, percent) = match metodo.as_str() {
                "normal" => (true, false, false),
                "boxcox" | "box-cox" => (true, true, false),
                "percentis" | "percentil" => (true, false, true),
                "todos" => (true, true, true),
                o => {
                    return Err(format!(
                        "--metodo '{o}' desconhecido: use normal, boxcox, percentis ou todos"
                    ));
                }
            };
            let values: Vec<f64> = groups.iter().flatten().copied().collect();
            let cap = capability::capability(&groups, &spec, &options)?;
            if normal {
                println!("{}", report::capability_text(&title, &cap));
            }
            let bc = if boxcox {
                let r = capability::box_cox_capability(&groups, &spec, &options, arredondar)?;
                println!("{}", report::box_cox_text(&title, &r));
                Some(r)
            } else {
                None
            };
            let pc = if percent {
                let (source, p) = percentile(&values, &spec)?;
                println!("{}", report::percentile_text(&source, &p));
                Some((source, p))
            } else {
                None
            };
            if let Some(path) = &html {
                let page = report::capability_html(
                    &title,
                    &values,
                    &cap,
                    bc.as_ref(),
                    pc.as_ref().map(|(s, p)| (s.as_str(), p)),
                );
                write(path, &page)?;
                println!("relatório HTML gravado em {}", path.display());
            }
            if let Some(path) = &json {
                let v = serde_json::json!({
                    "normal": cap,
                    "box_cox": bc,
                    "percentis": pc.as_ref().map(|(s, p)| serde_json::json!({"distribuicao": s, "indices": p})),
                });
                write(
                    path,
                    &serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?,
                )?;
                println!("resultado JSON gravado em {}", path.display());
            }
            Ok(())
        }
        Cmd::Msa {
            arquivo,
            tipo,
            tolerancia,
            lie,
            lse,
            k,
            variacao_processo,
            html,
            json,
        } => {
            let table = read_table(&arquivo)?;
            let title = file_name(&arquivo);
            let tolerance = tolerancia.or(match (lie, lse) {
                (Some(a), Some(b)) => Some(b - a),
                _ => None,
            });
            let (text, page, value) = match tipo.as_str() {
                "grr" | "rr" => {
                    let ms = data::gage(&table)?;
                    let d = msa::GageData::from_measurements(&ms)?;
                    let options = msa::GageOptions {
                        k,
                        tolerance,
                        ..msa::GageOptions::default()
                    };
                    let studies = vec![
                        msa::gage_anova(&d, &options)?,
                        msa::gage_average_range(&d, &options)?,
                    ];
                    (
                        report::gage_text(&title, &studies),
                        report::gage_html(&title, &studies),
                        serde_json::to_value(&studies).map_err(|e| e.to_string())?,
                    )
                }
                "atributos" | "atributo" => {
                    let r = data::ratings(&table)?;
                    let s = attribute::attribute_agreement(&r, 0.95)?;
                    (
                        report::attribute_text(&title, &s),
                        report::attribute_html(&title, &s),
                        serde_json::to_value(&s).map_err(|e| e.to_string())?,
                    )
                }
                "vies" | "viés" | "linearidade" => {
                    let pairs = data::reference_pairs(&table)?;
                    let mut refs: Vec<f64> = pairs.iter().map(|p| p.0).collect();
                    refs.sort_by(|a, b| a.total_cmp(b));
                    refs.dedup();
                    if refs.len() == 1 {
                        let vals: Vec<f64> = pairs.iter().map(|p| p.1).collect();
                        let b = msa::bias(refs[0], &vals, 0.95)?;
                        (
                            report::bias_text(&title, None, Some(&b)),
                            report::bias_html(&title, &pairs, None, Some(&b)),
                            serde_json::to_value(&b).map_err(|e| e.to_string())?,
                        )
                    } else {
                        let l = msa::linearity(&pairs, variacao_processo, 0.95)?;
                        (
                            report::bias_text(&title, Some(&l), None),
                            report::bias_html(&title, &pairs, Some(&l), None),
                            serde_json::to_value(&l).map_err(|e| e.to_string())?,
                        )
                    }
                }
                o => {
                    return Err(format!(
                        "--tipo '{o}' desconhecido: use grr, atributos, vies ou linearidade"
                    ));
                }
            };
            println!("{text}");
            if let Some(path) = &html {
                write(path, &page)?;
                println!("relatório HTML gravado em {}", path.display());
            }
            if let Some(path) = &json {
                write(
                    path,
                    &serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?,
                )?;
                println!("resultado JSON gravado em {}", path.display());
            }
            Ok(())
        }
        Cmd::Arl {
            tipo,
            deslocamento,
            lambda,
            l,
            k,
            h,
            corridas,
            semente,
        } => {
            let scheme = match tipo.as_str() {
                "shewhart" | "1" => Scheme::Shewhart(RuleSet::basic()),
                "we" => Scheme::Shewhart(RuleSet::western_electric()),
                "nelson" => Scheme::Shewhart(RuleSet::nelson()),
                "ewma" => Scheme::Ewma {
                    lambda,
                    l,
                    asymptotic: false,
                },
                "cusum" => Scheme::Cusum { k, h },
                o => {
                    return Err(format!(
                        "--tipo '{o}' desconhecido: use shewhart, we, nelson, ewma ou cusum"
                    ));
                }
            };
            if corridas < 2 {
                return Err("--corridas deve ser ao menos 2".into());
            }
            let est = arl::simulate(&scheme, deslocamento, corridas, semente, 1_000_000);
            println!(
                "ARL com deslocamento de {}σ: {} ± {} (erro padrão; {} corridas, desvio do comprimento {})",
                report::num(deslocamento),
                report::num(est.arl),
                report::num(est.std_error),
                est.runs,
                report::num(est.sdrl)
            );
            match &scheme {
                Scheme::Shewhart(r) if r.0.len() == 1 => println!(
                    "fórmula fechada: {}",
                    report::num(arl::shewhart(3.0, deslocamento))
                ),
                Scheme::Cusum { k, h } => println!(
                    "aproximação de Siegmund: {}",
                    report::num(arl::cusum_siegmund(*k, *h, deslocamento))
                ),
                _ => {}
            }
            if est.censored > 0 {
                eprintln!("aviso: {} corridas sem sinal até o limite", est.censored);
            }
            Ok(())
        }
    }
}

fn read_table(path: &Path) -> Result<data::Table, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("não foi possível ler '{}': {e}", path.display()))?;
    let (text, latin1) = data::decode(&bytes);
    if latin1 {
        eprintln!(
            "aviso: '{}' não está em UTF-8; lido como Latin-1 (Windows)",
            path.display()
        );
    }
    let t = data::parse(&text);
    if t.rows.is_empty() {
        return Err(format!("'{}' não tem linhas de dados", path.display()));
    }
    Ok(t)
}

fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into(),
    )
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text)
        .map_err(|e| format!("não foi possível gravar '{}': {e}", path.display()))
}

fn chart_data(
    kind: ChartKind,
    table: &data::Table,
    size: Option<usize>,
) -> Result<ChartData, String> {
    if kind.is_attribute() {
        let (counts, sizes) = data::attributes(table)?;
        Ok(ChartData::Attributes { counts, sizes })
    } else {
        let size = size.or((kind == ChartKind::IMr).then_some(1));
        let (groups, notes) = data::subgroups(table, size)?;
        for n in notes {
            eprintln!("aviso: {n}");
        }
        Ok(ChartData::Variables(groups))
    }
}

/// Indices given from 1 (`3,7`) as 0-based.
fn parse_indices(text: Option<&str>, len: usize) -> Result<Vec<usize>, String> {
    let Some(t) = text else { return Ok(vec![]) };
    let mut out = Vec::new();
    for part in t.split([',', ' ']).filter(|s| !s.is_empty()) {
        let i: usize = part
            .parse()
            .map_err(|_| format!("--excluir: '{part}' não é um número de subgrupo"))?;
        if i == 0 || i > len {
            return Err(format!("--excluir: subgrupo {i} fora de 1 a {len}"));
        }
        out.push(i - 1);
    }
    Ok(out)
}

/// Limits from a known mean and σ (standards given).
fn standard_limits(
    kind: ChartKind,
    d: &ChartData,
    mean: f64,
    sigma: f64,
) -> Result<Limits, String> {
    match kind {
        ChartKind::IMr | ChartKind::XbarR | ChartKind::XbarS => {
            let n = match d {
                ChartData::Variables(g) => g.first().map_or(1, Vec::len),
                _ => 1,
            };
            Ok(Limits::Variables {
                kind,
                n: if kind == ChartKind::IMr { 1 } else { n },
                mean,
                sigma,
            })
        }
        _ => Err(
            "--media e --sigma valem para cartas de variáveis; para atributos use --limites".into(),
        ),
    }
}

/// EWMA or CUSUM on individual values (or subgroup means, with σ/√n).
#[allow(clippy::too_many_arguments)]
fn memory_chart(
    title: &str,
    table: &data::Table,
    size: Option<usize>,
    which: &str,
    mean: Option<f64>,
    sigma: Option<f64>,
    lambda: f64,
    l: f64,
    k: f64,
    h: f64,
    html: Option<PathBuf>,
) -> Result<(), String> {
    let (groups, notes) = data::subgroups(table, size)?;
    for n in notes {
        eprintln!("aviso: {n}");
    }
    let n = groups[0].len();
    let x: Vec<f64> = groups
        .iter()
        .map(|g| g.iter().sum::<f64>() / g.len() as f64)
        .collect();
    let mut notes = Vec::new();
    let (mu, s) = match (mean, sigma) {
        (Some(m), Some(s)) => (m, s / (n as f64).sqrt()),
        _ => {
            let kind = if n == 1 {
                ChartKind::IMr
            } else {
                ChartKind::XbarR
            };
            let lim = charts::estimate_limits(kind, &ChartData::Variables(groups.clone()), &[])?;
            let Limits::Variables {
                mean: m, sigma: s, ..
            } = lim
            else {
                unreachable!()
            };
            notes.push(format!(
                "alvo e σ estimados dos dados (fase I): média {}, σ {} ({})",
                report::num(m),
                report::num(s),
                if n == 1 { "MR̄/d2" } else { "R̄/d2" }
            ));
            (mean.unwrap_or(m), sigma.unwrap_or(s) / (n as f64).sqrt())
        }
    };
    if n > 1 {
        notes.push(format!("médias de subgrupos de {n}: σ da média = σ/√{n}"));
    }
    let (panels, line) = if which == "ewma" {
        let e = Ewma {
            lambda,
            l,
            target: mu,
            sigma: s,
            asymptotic: false,
        };
        let r = e.run(&x)?;
        let line = format!(
            "EWMA (λ = {}, L = {}): alvo {}, σ {}; sinais nos pontos: {}",
            report::num(lambda),
            report::num(l),
            report::num(mu),
            report::num(s),
            list(&r.signals)
        );
        (vec![r.panel(mu)], line)
    } else {
        let c = Cusum {
            k,
            h,
            target: mu,
            sigma: s,
        };
        let r = c.run(&x)?;
        let mut line = format!(
            "CUSUM tabular (k = {}, h = {}): alvo {}, σ {}, H = {}; sinais para cima: {}; para baixo: {}",
            report::num(k),
            report::num(h),
            report::num(mu),
            report::num(s),
            report::num(r.h),
            list(&r.signals_up),
            list(&r.signals_down)
        );
        if let Some((i, m)) = r.shift_estimates.first() {
            line.push_str(&format!(
                "\nprimeiro sinal para cima no ponto {}: nova média estimada {}",
                i + 1,
                report::num(*m)
            ));
        }
        (r.panels(), line)
    };
    println!("{title}: {line}");
    for n in &notes {
        println!("nota: {n}");
    }
    if let Some(path) = &html {
        let page = report::panels_html(title, &line, &panels, &notes);
        write(path, &page)?;
        println!("relatório HTML gravado em {}", path.display());
    }
    Ok(())
}

fn list(v: &[usize]) -> String {
    if v.is_empty() {
        "nenhum".into()
    } else {
        v.iter()
            .map(|i| (i + 1).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Percentile method: the best distribution fitted by `utsim-fit`, or the empirical
/// quantiles when no family is accepted and there are enough values.
fn percentile(
    values: &[f64],
    spec: &Spec,
) -> Result<(String, capability::PercentileIndices), String> {
    let mut config = utsim_fit::report::FitConfig::default();
    config.gof.bootstrap = 100;
    let fit = utsim_fit::report::analyze("capacidade", values, &config);
    if let Some(best) = fit.ranking.best {
        let c = &fit.ranking.candidates[best];
        if let Some(f) = &c.fitted {
            let q = |p: f64| f.model.quantile(p);
            let r = capability::percentile_capability(&q, spec)?;
            let params = c
                .params
                .iter()
                .map(|(n, v)| format!("{n} = {}", report::num(*v)))
                .collect::<Vec<_>>()
                .join(", ");
            return Ok((format!("{} ajustada ({params})", c.family), r));
        }
    }
    let r = capability::percentile_capability_empirical(values, spec)?;
    Ok(("empírica (nenhuma família aceita)".into(), r))
}
