//! `utsim estatistica`: descriptive statistics and hypothesis tests on the columns of a
//! CSV file (crate `utsim-stats`; the CSV is read as in `utsim qualidade`, with
//! `utsim_quality::data`).

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use serde::Serialize;
use utsim_quality::data;
use utsim_stats::descriptive::{self, BinRule, QuantileMethod};
use utsim_stats::hypothesis::{self, Alternative, ProportionMethod};
use utsim_stats::nonparametric::{self, RankMethod};

/// The `estatistica` command line, parsed on its own (as `qualidade`, to keep the main
/// parser small).
#[derive(Parser)]
#[command(
    name = "utsim estatistica",
    about = "Estatística descritiva e testes de hipótese sobre as colunas de um CSV"
)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

/// Parses the arguments after `utsim estatistica` and runs the command.
pub fn run_args(args: Vec<String>) -> Result<(), String> {
    let parsed = Args::try_parse_from(std::iter::once("utsim estatistica".to_string()).chain(args))
        .unwrap_or_else(|e| e.exit());
    run(parsed.cmd)
}

#[derive(clap::Args)]
struct Common {
    /// Nível de confiança dos intervalos
    #[arg(long, default_value = "95%")]
    confianca: String,
    /// Grava o resultado em JSON
    #[arg(long)]
    json: Option<PathBuf>,
}

#[derive(clap::Args)]
struct Alt {
    /// Hipótese alternativa: diferente (padrão), menor ou maior
    #[arg(long, default_value = "diferente")]
    alternativa: String,
}

#[derive(Subcommand)]
enum Cmd {
    /// Resumo de uma coluna: média, desvio, quartis, forma, moda, intervalos, histograma,
    /// boxplot e gráfico de sequência
    Descritiva {
        arquivo: PathBuf,
        /// Coluna (nome do cabeçalho ou número a partir de 1; padrão: a primeira)
        #[arg(long)]
        coluna: Option<String>,
        /// Quartis: weibull ((n+1)p, padrão) ou linear ((n−1)p+1)
        #[arg(long, default_value = "weibull")]
        quartis: String,
        /// Classes do histograma: fd (Freedman-Diaconis, padrão), sturges ou um número
        #[arg(long, default_value = "fd")]
        classes: String,
        #[command(flatten)]
        comum: Common,
    },
    /// Teste t: uma amostra (--mu), duas amostras (--coluna2; Welch, ou --combinada) ou
    /// pareado (--pareado)
    TesteT {
        arquivo: PathBuf,
        #[arg(long)]
        coluna: Option<String>,
        /// Segunda coluna (duas amostras ou pareado)
        #[arg(long)]
        coluna2: Option<String>,
        /// Média sob H₀ (uma amostra) ou diferença sob H₀ (duas amostras, pareado)
        #[arg(long, default_value_t = 0.0)]
        mu: f64,
        /// Amostras pareadas linha a linha
        #[arg(long)]
        pareado: bool,
        /// Variância combinada (variâncias iguais) em vez de Welch
        #[arg(long)]
        combinada: bool,
        #[command(flatten)]
        alt: Alt,
        #[command(flatten)]
        comum: Common,
    },
    /// ANOVA de um fator com Tukey: cada coluna numérica é um grupo
    Anova {
        arquivo: PathBuf,
        #[command(flatten)]
        comum: Common,
    },
    /// Qui-quadrado de independência (tabela de contagens) ou de aderência (--aderencia)
    QuiQuadrado {
        arquivo: PathBuf,
        /// Aderência das contagens da primeira coluna numérica às proporções dadas
        #[arg(long)]
        aderencia: bool,
        /// Proporções esperadas da aderência, ex. 9,3,3,1 (padrão: iguais)
        #[arg(long)]
        proporcoes: Option<String>,
        /// Parâmetros estimados dos mesmos dados (tiram graus de liberdade)
        #[arg(long, default_value_t = 0)]
        estimados: usize,
        #[command(flatten)]
        comum: Common,
    },
    /// Uma proporção (--x, --n, --p0) ou duas (--x2, --n2); Fisher exato com --fisher
    Proporcao {
        #[arg(long)]
        x: u64,
        #[arg(long)]
        n: u64,
        /// Proporção sob H₀ (uma proporção)
        #[arg(long, default_value_t = 0.5)]
        p0: f64,
        #[arg(long, requires = "n2")]
        x2: Option<u64>,
        #[arg(long, requires = "x2")]
        n2: Option<u64>,
        /// exato (binomial, padrão) ou normal
        #[arg(long, default_value = "exato")]
        metodo: String,
        /// Duas proporções: também o teste exato de Fisher
        #[arg(long)]
        fisher: bool,
        #[command(flatten)]
        alt: Alt,
        #[command(flatten)]
        comum: Common,
    },
    /// Mann-Whitney (duas amostras independentes)
    MannWhitney {
        arquivo: PathBuf,
        #[arg(long)]
        coluna: Option<String>,
        #[arg(long)]
        coluna2: Option<String>,
        /// Sempre a aproximação normal
        #[arg(long)]
        normal: bool,
        #[command(flatten)]
        alt: Alt,
        #[command(flatten)]
        comum: Common,
    },
    /// Wilcoxon dos postos sinalizados: uma amostra (--mu) ou pareado (--coluna2)
    Wilcoxon {
        arquivo: PathBuf,
        #[arg(long)]
        coluna: Option<String>,
        #[arg(long)]
        coluna2: Option<String>,
        #[arg(long, default_value_t = 0.0)]
        mu: f64,
        #[arg(long)]
        normal: bool,
        #[command(flatten)]
        alt: Alt,
        #[command(flatten)]
        comum: Common,
    },
}

fn level(c: &Common) -> Result<f64, String> {
    utsim_stats::parse_level(&c.confianca)
        .ok_or_else(|| format!("nível de confiança inválido: '{}'", c.confianca))
}

fn alternative(a: &Alt) -> Result<Alternative, String> {
    match a.alternativa.trim().to_lowercase().as_str() {
        "diferente" | "bilateral" | "≠" => Ok(Alternative::TwoSided),
        "menor" | "<" => Ok(Alternative::Less),
        "maior" | ">" => Ok(Alternative::Greater),
        o => Err(format!(
            "alternativa desconhecida: '{o}' (use diferente, menor ou maior)"
        )),
    }
}

fn save_json(c: &Common, value: &impl Serialize) -> Result<(), String> {
    if let Some(path) = &c.json {
        let text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
        std::fs::write(path, text)
            .map_err(|e| format!("não foi possível gravar '{}': {e}", path.display()))?;
    }
    Ok(())
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

/// Index of a column given by header name or 1-based number; `None` picks the first
/// column with a number in it.
fn column_index(t: &data::Table, name: Option<&str>) -> Result<usize, String> {
    match name {
        Some(n) => {
            if let Some(i) = t.column(&[n]) {
                return Ok(i);
            }
            match n.trim().parse::<usize>() {
                Ok(k) if k >= 1 => Ok(k - 1),
                _ => Err(format!("coluna '{n}' não encontrada")),
            }
        }
        None => numeric_columns(t)
            .first()
            .copied()
            .ok_or_else(|| "nenhuma coluna numérica".to_string()),
    }
}

fn cell(t: &data::Table, row: &[String], j: usize) -> Option<f64> {
    row.get(j).and_then(|c| data::number(c, t.decimal_comma))
}

/// The numbers of column `j` (empty or non-numeric cells skipped).
fn column(t: &data::Table, j: usize) -> Vec<f64> {
    t.rows.iter().filter_map(|r| cell(t, r, j)).collect()
}

/// Columns with at least one number.
fn numeric_columns(t: &data::Table) -> Vec<usize> {
    let width = t.rows.iter().map(|r| r.len()).max().unwrap_or(0);
    (0..width)
        .filter(|&j| t.rows.iter().any(|r| cell(t, r, j).is_some()))
        .collect()
}

fn column_name(t: &data::Table, j: usize) -> String {
    t.header
        .as_ref()
        .and_then(|h| h.get(j))
        .cloned()
        .unwrap_or_else(|| format!("coluna {}", j + 1))
}

/// Two columns of a file; the second is required.
fn two_columns(
    t: &data::Table,
    a: Option<&str>,
    b: Option<&str>,
) -> Result<(usize, usize), String> {
    let i = column_index(t, a)?;
    let j = match b {
        Some(_) => column_index(t, b)?,
        None => *numeric_columns(t)
            .iter()
            .find(|&&k| k != i)
            .ok_or("é preciso uma segunda coluna (--coluna2)")?,
    };
    Ok((i, j))
}

/// Rows where both columns have a number.
fn pairs(t: &data::Table, i: usize, j: usize) -> (Vec<f64>, Vec<f64>) {
    t.rows
        .iter()
        .filter_map(|r| Some((cell(t, r, i)?, cell(t, r, j)?)))
        .unzip()
}

fn p(v: f64) -> String {
    if v.is_nan() {
        "—".into()
    } else if v < 1e-4 {
        format!("{v:.2e}")
    } else {
        format!("{v:.4}")
    }
}

fn g(v: f64) -> String {
    if v.is_nan() {
        "—".into()
    } else if v.is_infinite() {
        if v > 0.0 {
            "+∞".into()
        } else {
            "−∞".into()
        }
    } else {
        format!("{v:.6}")
    }
}

fn alt_text(a: Alternative) -> &'static str {
    match a {
        Alternative::TwoSided => "≠",
        Alternative::Less => "<",
        Alternative::Greater => ">",
    }
}

fn run(cmd: Cmd) -> Result<(), String> {
    match cmd {
        Cmd::Descritiva {
            arquivo,
            coluna,
            quartis,
            classes,
            comum,
        } => {
            let lv = level(&comum)?;
            let t = read_table(&arquivo)?;
            let j = column_index(&t, coluna.as_deref())?;
            let xs = column(&t, j);
            let method = match quartis.as_str() {
                "weibull" => QuantileMethod::Weibull,
                "linear" => QuantileMethod::Linear,
                o => return Err(format!("quartis: '{o}' (use weibull ou linear)")),
            };
            let rule = match classes.as_str() {
                "fd" => BinRule::FreedmanDiaconis,
                "sturges" => BinRule::Sturges,
                k => BinRule::Count(
                    k.parse()
                        .map_err(|_| format!("classes: '{k}' (use fd, sturges ou um número)"))?,
                ),
            };
            let d = descriptive::describe(&xs, lv, method).ok_or("a coluna não tem números")?;
            let h = descriptive::histogram(&xs, rule);
            let b = descriptive::boxplot(&xs, method);
            let r = descriptive::run_chart(&xs);
            println!("Descritiva de '{}' (n = {})", column_name(&t, j), d.n);
            println!(
                "  média {}  desvio {}  variância {}",
                g(d.mean),
                g(d.sd),
                g(d.variance)
            );
            println!("  erro padrão {}  CV {}", g(d.se_mean), g(d.cv));
            println!(
                "  mín {}  Q1 {}  mediana {}  Q3 {}  máx {}  (quartis {:?})",
                g(d.min),
                g(d.q1),
                g(d.median),
                g(d.q3),
                g(d.max),
                d.quantile_method
            );
            println!(
                "  assimetria {}  curtose (excesso) {}",
                g(d.skewness),
                g(d.kurtosis)
            );
            if d.modes.is_empty() {
                println!("  moda: nenhuma (todos os valores distintos)");
            } else {
                let m: Vec<String> = d.modes.iter().map(|v| g(*v)).collect();
                println!("  moda {} ({} vezes)", m.join(", "), d.mode_count);
            }
            let ci = |name: &str, c: Option<descriptive::Interval>| {
                if let Some(c) = c {
                    println!(
                        "  IC {:.1}% {name}: {} a {}",
                        c.level * 100.0,
                        g(c.low),
                        g(c.high)
                    );
                }
            };
            ci("da média", d.mean_ci);
            ci("do desvio", d.sd_ci);
            ci("da mediana", d.median_ci);
            if let Some(h) = &h {
                println!(
                    "  histograma ({:?}): {} classes de largura {}",
                    h.rule,
                    h.counts.len(),
                    g(h.width)
                );
            }
            if let Some(b) = &b {
                println!(
                    "  boxplot: bigodes {} a {}, {} atípicos",
                    g(b.lower_whisker),
                    g(b.upper_whisker),
                    b.outliers.len()
                );
            }
            if let Some(r) = &r {
                if let Some(a) = r.about_median {
                    println!(
                        "  sequência: {} corridas em torno da mediana (esperado {:.1}); p agrupamento {}, p mistura {}",
                        a.runs,
                        a.expected,
                        p(a.p_fewer),
                        p(a.p_more)
                    );
                }
                if let Some(u) = r.up_down {
                    println!(
                        "  sequência: {} corridas para cima e para baixo (esperado {:.1}); p tendência {}, p oscilação {}",
                        u.runs,
                        u.expected,
                        p(u.p_fewer),
                        p(u.p_more)
                    );
                }
            }
            save_json(
                &comum,
                &serde_json::json!({"descritiva": d, "histograma": h, "boxplot": b, "sequencia": r}),
            )
        }
        Cmd::TesteT {
            arquivo,
            coluna,
            coluna2,
            mu,
            pareado,
            combinada,
            alt,
            comum,
        } => {
            let lv = level(&comum)?;
            let a = alternative(&alt)?;
            let t = read_table(&arquivo)?;
            let r = if pareado || coluna2.is_some() {
                let (i, j) = two_columns(&t, coluna.as_deref(), coluna2.as_deref())?;
                if pareado {
                    let (x, y) = pairs(&t, i, j);
                    hypothesis::t_paired(&x, &y, mu, lv, a)
                } else {
                    hypothesis::t_two_sample(&column(&t, i), &column(&t, j), mu, combinada, lv, a)
                }
            } else {
                let i = column_index(&t, coluna.as_deref())?;
                hypothesis::t_one_sample(&column(&t, i), mu, lv, a)
            }
            .ok_or("são precisos ao menos dois valores em cada amostra")?;
            let kind = match r.kind {
                hypothesis::TTestKind::OneSample => "uma amostra",
                hypothesis::TTestKind::Welch => "duas amostras (Welch)",
                hypothesis::TTestKind::Pooled => "duas amostras (variância combinada)",
                hypothesis::TTestKind::Paired => "pareado",
            };
            let param = match r.kind {
                hypothesis::TTestKind::OneSample => "μ",
                hypothesis::TTestKind::Paired => "μ_d",
                _ => "μ₁ − μ₂",
            };
            println!("Teste t, {kind}; H₁: {param} {} {}", alt_text(a), g(mu));
            for k in 0..r.sizes.len() {
                println!(
                    "  n {}  média {}  desvio {}",
                    r.sizes[k],
                    g(r.means[k]),
                    g(r.sds[k])
                );
            }
            println!(
                "  estimativa {}  erro padrão {}  t = {}  gl = {}",
                g(r.estimate),
                g(r.se),
                g(r.t),
                g(r.df)
            );
            println!(
                "  p = {} (bilateral {}, menor {}, maior {})",
                p(r.p_value),
                p(r.p.two_sided),
                p(r.p.less),
                p(r.p.greater)
            );
            println!(
                "  IC {:.1}%: {} a {}  d de Cohen {}",
                r.ci.level * 100.0,
                g(r.ci.low),
                g(r.ci.high),
                g(r.effect_size)
            );
            save_json(&comum, &r)
        }
        Cmd::Anova { arquivo, comum } => {
            let lv = level(&comum)?;
            let t = read_table(&arquivo)?;
            let cols = numeric_columns(&t);
            let groups: Vec<Vec<f64>> = cols.iter().map(|&j| column(&t, j)).collect();
            let refs: Vec<&[f64]> = groups.iter().map(|g| g.as_slice()).collect();
            let o = hypothesis::one_way_anova(&refs, lv)
                .ok_or("são precisos ao menos dois grupos com valores e graus de liberdade")?;
            let a = &o.anova;
            println!("ANOVA de um fator ({} grupos, n = {})", a.groups, a.n);
            for (k, &j) in cols.iter().enumerate() {
                println!(
                    "  {}: n {}  média {}",
                    column_name(&t, j),
                    a.sizes[k],
                    g(a.means[k])
                );
            }
            println!(
                "  entre: SQ {} gl {} QM {}",
                g(a.ss_between),
                a.df_between,
                g(a.ms_between)
            );
            println!(
                "  dentro: SQ {} gl {} QM {}",
                g(a.ss_within),
                a.df_within,
                g(a.ms_within)
            );
            println!(
                "  F = {}  p = {}  R² = {}",
                g(a.f),
                p(a.p_value),
                g(o.r_squared)
            );
            println!(
                "  Tukey {:.1}% (q = {}):",
                o.tukey.level * 100.0,
                g(o.tukey.q_critical)
            );
            for pc in &o.tukey.pairs {
                println!(
                    "    {} − {}: {} [{} a {}] p = {}{}",
                    column_name(&t, cols[pc.i]),
                    column_name(&t, cols[pc.j]),
                    g(pc.difference),
                    g(pc.low()),
                    g(pc.high()),
                    p(pc.p_value),
                    if pc.significant { " *" } else { "" }
                );
            }
            save_json(&comum, &o)
        }
        Cmd::QuiQuadrado {
            arquivo,
            aderencia,
            proporcoes,
            estimados,
            comum,
        } => {
            let t = read_table(&arquivo)?;
            let cols = numeric_columns(&t);
            if aderencia {
                let obs = column(&t, *cols.first().ok_or("nenhuma coluna numérica")?);
                let props: Option<Vec<f64>> = proporcoes
                    .as_deref()
                    .map(|s| {
                        s.split(',')
                            .map(|v| v.trim().parse::<f64>())
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .transpose()
                    .map_err(|_| "proporções inválidas (ex.: 9,3,3,1)".to_string())?;
                let r = hypothesis::chi_square_goodness_of_fit(&obs, props.as_deref(), estimados)
                    .ok_or(
                    "aderência impossível: confira categorias, proporções e graus de liberdade",
                )?;
                println!("Qui-quadrado de aderência ({} categorias)", obs.len());
                println!(
                    "  X² = {}  gl = {}  p = {}",
                    g(r.statistic),
                    r.df,
                    p(r.p_value)
                );
                if r.low_expected_warning {
                    println!(
                        "  aviso: {} contagens esperadas abaixo de 5 (mínima {}); a aproximação é duvidosa",
                        r.cells_below_5,
                        g(r.min_expected)
                    );
                }
                save_json(&comum, &r)
            } else {
                let table: Vec<Vec<f64>> = t
                    .rows
                    .iter()
                    .map(|r| {
                        cols.iter()
                            .map(|&j| cell(&t, r, j).unwrap_or(0.0))
                            .collect()
                    })
                    .collect();
                let r = hypothesis::chi_square_independence(&table).ok_or(
                    "tabela inválida: são precisas 2 linhas e 2 colunas com totais positivos",
                )?;
                println!(
                    "Qui-quadrado de independência ({} × {}, N = {})",
                    table.len(),
                    cols.len(),
                    r.total
                );
                println!(
                    "  X² = {}  gl = {}  p = {}  (G² = {}, p = {})",
                    g(r.statistic),
                    r.df,
                    p(r.p_value),
                    g(r.g_statistic),
                    p(r.g_p_value)
                );
                println!("  resíduos ajustados:");
                for row in &r.adjusted_residuals {
                    let cells: Vec<String> = row.iter().map(|v| format!("{v:7.2}")).collect();
                    println!("   {}", cells.join(" "));
                }
                if r.low_expected_warning {
                    println!(
                        "  aviso: {} células com esperado abaixo de 5 (mínimo {}); junte categorias ou use Fisher em 2 × 2",
                        r.cells_below_5,
                        g(r.min_expected)
                    );
                }
                save_json(&comum, &r)
            }
        }
        Cmd::Proporcao {
            x,
            n,
            p0,
            x2,
            n2,
            metodo,
            fisher,
            alt,
            comum,
        } => {
            let lv = level(&comum)?;
            let a = alternative(&alt)?;
            if let (Some(x2), Some(n2)) = (x2, n2) {
                let r = hypothesis::two_proportions(x, n, x2, n2, lv, a)
                    .ok_or("contagens inválidas (x ≤ n, n > 0)")?;
                println!(
                    "Duas proporções: {} e {}; H₁: p₁ − p₂ {} 0",
                    g(r.p_hat[0]),
                    g(r.p_hat[1]),
                    alt_text(a)
                );
                println!(
                    "  diferença {}  z = {}  p = {}",
                    g(r.difference),
                    g(r.z),
                    p(r.p_value)
                );
                println!(
                    "  IC {:.1}%: {} a {}",
                    r.ci.level * 100.0,
                    g(r.ci.low),
                    g(r.ci.high)
                );
                let f = fisher
                    .then(|| hypothesis::fisher_exact([[x, n - x], [x2, n2 - x2]]))
                    .flatten();
                if let Some(f) = &f {
                    println!(
                        "  Fisher exato: p = {} (H₁: razão de chances {} 1), razão de chances {}",
                        p(f.p.get(a)),
                        alt_text(a),
                        g(f.odds_ratio)
                    );
                }
                save_json(&comum, &serde_json::json!({"proporcoes": r, "fisher": f}))
            } else {
                let m = match metodo.as_str() {
                    "exato" => ProportionMethod::Exact,
                    "normal" => ProportionMethod::Normal,
                    o => return Err(format!("método: '{o}' (use exato ou normal)")),
                };
                let r = hypothesis::one_proportion(x, n, p0, m, lv, a)
                    .ok_or("contagens inválidas (x ≤ n, n > 0, 0 < p0 < 1)")?;
                println!(
                    "Uma proporção ({}): p̂ = {}; H₁: p {} {}",
                    metodo,
                    g(r.p_hat),
                    alt_text(a),
                    g(p0)
                );
                println!(
                    "  z = {}  p = {} (bilateral {}, menor {}, maior {})",
                    g(r.z),
                    p(r.p_value),
                    p(r.p.two_sided),
                    p(r.p.less),
                    p(r.p.greater)
                );
                println!(
                    "  IC {:.1}%: {} a {}  (Wilson {} a {})",
                    r.ci.level * 100.0,
                    g(r.ci.low),
                    g(r.ci.high),
                    g(r.wilson.low),
                    g(r.wilson.high)
                );
                save_json(&comum, &r)
            }
        }
        Cmd::MannWhitney {
            arquivo,
            coluna,
            coluna2,
            normal,
            alt,
            comum,
        } => {
            let a = alternative(&alt)?;
            let t = read_table(&arquivo)?;
            let (i, j) = two_columns(&t, coluna.as_deref(), coluna2.as_deref())?;
            let method = if normal {
                RankMethod::Normal
            } else {
                RankMethod::Auto
            };
            let r = nonparametric::mann_whitney(&column(&t, i), &column(&t, j), method, a)
                .ok_or("as duas amostras precisam de valores")?;
            println!(
                "Mann-Whitney: n₁ = {}, n₂ = {}; H₁: posição de '{}' {} posição de '{}'",
                r.n1,
                r.n2,
                column_name(&t, i),
                alt_text(a),
                column_name(&t, j)
            );
            println!(
                "  W = {}  U₁ = {}  U₂ = {}  z = {}",
                g(r.rank_sum),
                g(r.u1),
                g(r.u2),
                g(r.z)
            );
            println!(
                "  p = {} ({}{})  deslocamento (Hodges-Lehmann) {}",
                p(r.p_value),
                if r.exact {
                    "exato"
                } else {
                    "aproximação normal"
                },
                if r.ties { ", com empates" } else { "" },
                g(r.shift)
            );
            save_json(&comum, &r)
        }
        Cmd::Wilcoxon {
            arquivo,
            coluna,
            coluna2,
            mu,
            normal,
            alt,
            comum,
        } => {
            let a = alternative(&alt)?;
            let t = read_table(&arquivo)?;
            let method = if normal {
                RankMethod::Normal
            } else {
                RankMethod::Auto
            };
            let r = if coluna2.is_some() {
                let (i, j) = two_columns(&t, coluna.as_deref(), coluna2.as_deref())?;
                let (x, y) = pairs(&t, i, j);
                nonparametric::wilcoxon_paired(&x, &y, mu, method, a)
            } else {
                let i = column_index(&t, coluna.as_deref())?;
                nonparametric::wilcoxon_signed_rank(&column(&t, i), mu, method, a)
            }
            .ok_or("nenhuma diferença diferente de zero")?;
            println!(
                "Wilcoxon dos postos sinalizados: n = {} ({} zeros retirados); H₁: posição {} {}",
                r.n,
                r.zeros,
                alt_text(a),
                g(mu)
            );
            println!(
                "  W⁺ = {}  W⁻ = {}  z = {}",
                g(r.w_plus),
                g(r.w_minus),
                g(r.z)
            );
            println!(
                "  p = {} ({}{})  pseudomediana {}",
                p(r.p_value),
                if r.exact {
                    "exato"
                } else {
                    "aproximação normal"
                },
                if r.ties { ", com empates" } else { "" },
                g(r.pseudo_median)
            );
            save_json(&comum, &r)
        }
    }
}
