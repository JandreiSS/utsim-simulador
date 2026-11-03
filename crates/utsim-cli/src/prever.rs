//! `utsim prever`: the forecast without a twin. From a state file (or the empty system),
//! `R` replications run to the horizon; the KPIs come with the precision of the mean, the
//! range of what may happen (P5 to P95), the probability of each goal and the "what if"
//! scenarios, compared with common random numbers.

use std::path::{Path, PathBuf};

use utsim_dash::Dashboard;
use utsim_dash::config::{Target, TimeUnit};
use utsim_model::experiment::{Experiment, Scenario};
use utsim_model::{InitialState, Model};
use utsim_twin::forecast::{self, Forecast, ForecastSettings, Horizon, Request};

#[derive(clap::Args)]
pub struct Args {
    /// Modelo (.ron)
    model: PathBuf,
    /// Estado inicial (.ron) de onde a previsão parte (padrão: o sistema vazio)
    #[arg(long)]
    estado: Option<PathBuf>,
    /// Horizonte: 4h, 30min, 90s ou um número (unidades do modelo); padrão: o do --config, ou 4h
    #[arg(long)]
    horizonte: Option<String>,
    /// Número de replicações (padrão 40)
    #[arg(short = 'n', long)]
    reps: Option<u32>,
    /// Cenários "e se": lista RON [(name: "...", set: {"objeto.campo": "valor"})] ou um experimento
    #[arg(long, value_name = "ARQUIVO")]
    cenarios: Option<PathBuf>,
    /// Configuração da previsão, Forecast(...): metas, limites, objetos acompanhados, cenários
    #[arg(long, value_name = "ARQUIVO")]
    config: Option<PathBuf>,
    /// Unidade de tempo do modelo: s, min, h ou u (padrão: a do <modelo>.painel.ron, ou s)
    #[arg(long)]
    unidade: Option<String>,
    /// Meta de um indicador, KPI>=valor ou KPI<=valor (repetível), ex.: producao>=450
    #[arg(long = "meta", value_name = "META")]
    metas: Vec<String>,
    /// Semente (padrão: a do modelo, a mesma de `utsim run --estado`)
    #[arg(long)]
    seed: Option<u64>,
    /// Grava a previsão completa em JSON
    #[arg(long)]
    json: Option<PathBuf>,
}

/// `s`, `min`, `h`, `u` (or the names of the dashboard file).
pub fn parse_unit(text: &str) -> Result<TimeUnit, String> {
    Ok(match text.trim().to_lowercase().as_str() {
        "s" | "seg" | "segundos" | "seconds" => TimeUnit::Seconds,
        "min" | "minutos" | "minutes" => TimeUnit::Minutes,
        "h" | "horas" | "hours" => TimeUnit::Hours,
        "u" | "ut" | "unit" | "unidade" => TimeUnit::Unit,
        _ => return Err(format!("unidade '{text}' desconhecida: use s, min, h ou u")),
    })
}

/// `producao>=450`, `lead_time<=30`.
fn parse_target(text: &str) -> Result<(String, Target), String> {
    let bad = || format!("meta '{text}' inválida: use KPI>=valor ou KPI<=valor");
    let (id, v, at_least) = if let Some((a, b)) = text.split_once(">=") {
        (a, b, true)
    } else if let Some((a, b)) = text.split_once("<=") {
        (a, b, false)
    } else {
        return Err(bad());
    };
    let v: f64 = v.trim().replace(',', ".").parse().map_err(|_| bad())?;
    if id.trim().is_empty() || !v.is_finite() {
        return Err(bad());
    }
    let t = if at_least {
        Target::AtLeast(v)
    } else {
        Target::AtMost(v)
    };
    Ok((id.trim().to_string(), t))
}

/// The scenarios of a file: a RON list of scenarios, or the scenarios of an experiment
/// that change something.
fn load_scenarios(path: &Path) -> Result<Vec<Scenario>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let options =
        ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
    match options.from_str::<Vec<Scenario>>(&text) {
        Ok(v) => Ok(v),
        Err(list_err) => {
            let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
            match Experiment::parse(&text, dir) {
                Ok(exp) => Ok(exp
                    .scenarios
                    .into_iter()
                    .filter(|s| !s.set.is_empty())
                    .collect()),
                Err(_) => Err(format!(
                    "{}: não é uma lista de cenários nem um experimento ({list_err})",
                    path.display()
                )),
            }
        }
    }
}

/// The settings of a forecast: the given ones (or the defaults), the time unit from them,
/// else the dashboard file's; the dashboard's goals under the forecast's own.
pub fn settings_with_dashboard(
    base: Option<ForecastSettings>,
    dash: &Dashboard,
) -> (ForecastSettings, TimeUnit) {
    let mut s = base.unwrap_or_default();
    let unit = s.time_unit.unwrap_or(dash.time_unit);
    let mut targets = dash.targets.clone();
    targets.extend(std::mem::take(&mut s.targets));
    s.targets = targets;
    (s, unit)
}

/// Prints a forecast: the summary, the KPIs, the limits and the scenarios.
pub fn print(f: &Forecast) {
    println!("{}", f.resumo);
    println!(
        "\n{:<34} {:>16} {:>18} {:>10} {:>24} semáforo",
        "indicador", "média ± precisão", "faixa P5 a P95", "meta", "P(meta) (Wilson)"
    );
    for k in &f.kpis {
        let meta = k.meta.as_ref().map_or("—".to_string(), |m| {
            format!(
                "{}{}",
                if m.sentido == "min" { "≥ " } else { "≤ " },
                m.valor
            )
        });
        let prob = k.prob.as_ref().map_or("—".to_string(), |p| {
            format!(
                "{:.0} % ({:.0} a {:.0} %)",
                100.0 * p.p,
                100.0 * p.lo,
                100.0 * p.hi
            )
        });
        let light = serde_json::to_value(k.situacao).unwrap_or_default();
        println!(
            "{:<34} {:>16} {:>18} {:>10} {:>24} {}",
            format!("{} ({})", k.nome, k.unidade),
            k.texto,
            k.faixa,
            meta,
            prob,
            light.as_str().unwrap_or("")
        );
    }
    if !f.eventos.is_empty() {
        println!("\nLimites:");
        for e in &f.eventos {
            println!("  {}", e.texto);
        }
    }
    if !f.cenarios.is_empty() {
        println!(
            "\nE se (diferença pareada para o como está, mesmas sementes; ≈ = não significativa):"
        );
        for c in &f.cenarios {
            println!("  {}", c.nome);
            if let Some(e) = &c.erro {
                println!("    erro: {e}");
                continue;
            }
            for k in &c.kpis {
                let efeito = match k.melhora {
                    Some(true) => "melhora",
                    Some(false) => "piora",
                    None => "",
                };
                println!(
                    "    {:<32} {:>16}  Δ {:<18} {efeito}",
                    k.nome, k.texto, k.texto_diferenca
                );
            }
        }
    }
    for a in f.avisos.iter().chain(&f.notas_estado) {
        eprintln!("aviso: {a}");
    }
}

pub fn run(a: Args) -> Result<(), String> {
    let model = Model::load(&a.model).map_err(|e| format!("{}: {e}", a.model.display()))?;
    model.validate().map_err(|e| e.to_string())?;
    let dash = match Dashboard::load_next_to(&a.model) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("aviso: {e}; usando o painel padrão");
            Dashboard::default()
        }
    };
    let base = match &a.config {
        Some(p) => Some(ForecastSettings::parse(
            &std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?,
        )?),
        None => None,
    };
    let (mut settings, mut unit) = settings_with_dashboard(base, &dash);
    if let Some(u) = &a.unidade {
        unit = parse_unit(u)?;
        settings.time_unit = Some(unit);
    }
    if let Some(h) = &a.horizonte {
        settings.horizon = Horizon::parse(h)?;
    }
    if let Some(n) = a.reps {
        settings.replications = n;
    }
    if let Some(p) = &a.cenarios {
        settings.scenarios.extend(load_scenarios(p)?);
    }
    for m in &a.metas {
        let (id, t) = parse_target(m)?;
        settings.targets.insert(id, t);
    }
    let mut errors = Vec::new();
    for w in settings.validate(&model, &mut errors) {
        eprintln!("aviso: {w}");
    }
    if !errors.is_empty() {
        return Err(format!("previsão inválida:\n  - {}", errors.join("\n  - ")));
    }
    let state = match &a.estado {
        Some(p) => InitialState::load(p).map_err(|e| format!("{}: {e}", p.display()))?,
        None => InitialState::default(),
    };
    let seed = a.seed.or(settings.seed).unwrap_or(model.run.seed);
    eprintln!(
        "Previsão de \"{}\" a partir de {} (relógio {}): {} replicações, semente {seed}",
        model.name,
        a.estado
            .as_ref()
            .map_or("o sistema vazio".to_string(), |p| p.display().to_string()),
        state.clock,
        settings.replications
    );
    let started = std::time::Instant::now();
    let f = forecast::forecast(
        &Request {
            model: &model,
            settings: &settings,
            unit,
            instant: 0.0,
            seed,
            on_request: true,
        },
        &state,
    )?;
    print(&f);
    eprintln!("\nprevisão em {:.2?}", started.elapsed());
    if let Some(path) = &a.json {
        let text = serde_json::to_string_pretty(&f).map_err(|e| e.to_string())?;
        std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        println!("previsão gravada em {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets_and_units_parse() {
        assert_eq!(
            parse_target("producao>=450").unwrap(),
            ("producao".to_string(), Target::AtLeast(450.0))
        );
        assert_eq!(
            parse_target("lead_time <= 2,5").unwrap(),
            ("lead_time".to_string(), Target::AtMost(2.5))
        );
        assert!(parse_target("producao=450").is_err());
        assert_eq!(parse_unit("min").unwrap(), TimeUnit::Minutes);
        assert!(parse_unit("dia").is_err());
    }
}
