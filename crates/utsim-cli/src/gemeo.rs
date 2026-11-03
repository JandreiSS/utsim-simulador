//! `utsim gemeo`: the digital twin in shadow mode (only observes the plant), with
//! recording and, with `--previsao`, the forecast ahead from the observed state; and
//! `utsim gemeo reproduzir`, the replay of a recording without network.

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use utsim_dash::{Dashboard, Server, Study};
use utsim_twin::live::{self, LiveOptions, Shared};
use utsim_twin::panel::TwinPanel;
use utsim_twin::record::{Recorder, Recording, SessionHeader};
use utsim_twin::{Prepared, Twin, TwinReport, connect, model_hash, simdist};

#[derive(clap::Args)]
#[command(args_conflicts_with_subcommands = true, subcommand_negates_reqs = true)]
pub struct Args {
    #[command(subcommand)]
    action: Option<Action>,
    /// Modelo (.ron) que o gêmeo acompanha
    #[arg(required = true)]
    model: Option<PathBuf>,
    /// Arquivo do gêmeo: conexões e mapeamento (padrão: <modelo>.gemeo.ron)
    #[arg(long)]
    config: Option<PathBuf>,
    /// Grava todas as observações nesta pasta (uma subpasta por sessão)
    #[arg(long, value_name = "PASTA")]
    gravar: Option<PathBuf>,
    /// Serve o painel em 127.0.0.1 nesta porta, com a página /gemeo
    #[arg(long, value_name = "PORTA")]
    painel: Option<u16>,
    /// Tempo do gêmeo em que a sessão termina (padrão: até Ctrl+C)
    #[arg(long)]
    ate: Option<f64>,
    /// Grava o relatório final (KPIs, alertas, estado) em JSON
    #[arg(long)]
    json: Option<PathBuf>,
    /// Grava o estado observado no fim como estado inicial (.ron, para `utsim run --estado`)
    #[arg(long, value_name = "ARQUIVO")]
    estado: Option<PathBuf>,
    /// Liga a previsão: a cada intervalo (seção forecast do .gemeo.ron) e sob pedido no
    /// painel, R replicações a partir do estado observado até o horizonte
    #[arg(long)]
    previsao: bool,
}

#[derive(clap::Subcommand)]
enum Action {
    /// Reexecuta uma gravação sem rede e sem relógio (mesmos resultados da sessão)
    Reproduzir {
        /// Pasta da sessão (sessao-...) ou pasta de gravações (usa a mais recente)
        gravacao: PathBuf,
        /// Modelo (padrão: o da gravação)
        #[arg(long)]
        modelo: Option<PathBuf>,
        /// Arquivo do gêmeo (padrão: o texto guardado na gravação)
        #[arg(long)]
        config: Option<PathBuf>,
        /// Grava o relatório (KPIs, alertas, estado) em JSON
        #[arg(long)]
        json: Option<PathBuf>,
        /// Grava o estado observado no fim como estado inicial (.ron)
        #[arg(long, value_name = "ARQUIVO")]
        estado: Option<PathBuf>,
        /// Refaz as previsões da sessão (nos mesmos instantes e com as mesmas sementes);
        /// sem previsões gravadas, faz uma no fim
        #[arg(long)]
        previsao: bool,
    },
}

fn default_config(model: &Path) -> PathBuf {
    let stem = model
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    model.with_file_name(format!("{stem}.gemeo.ron"))
}

fn simulate(p: &Prepared) -> Result<simdist::SimDistribution, String> {
    eprintln!(
        "Distribuição simulada: {} replicações de \"{}\", janelas de {}",
        p.config.shadow.replications, p.model.name, p.config.shadow.window
    );
    simdist::simulate(&p.model, &p.config)
}

fn summary(r: &TwinReport) {
    println!(
        "Gêmeo (sombra) de \"{}\": tempo {:.1}, {} janela(s) de {}, {} leituras ({} sem regra), {} testes",
        r.modelo,
        r.tempo,
        r.janelas.len(),
        r.janela,
        r.recebidas,
        r.nao_mapeadas,
        r.testes
    );
    println!(
        "  limites: {} KPIs, BH a {:.0} %, CUSUM k = {}, h = {:.2}, EWMA λ = {}, L = {:.2}",
        r.limites.m,
        100.0 * r.limites.fdr,
        r.limites.cusum_k,
        r.limites.cusum_h,
        r.limites.ewma_lambda,
        r.limites.ewma_l
    );
    for (k, s) in &r.kpis {
        let level = serde_json::to_value(s.nivel).unwrap_or_default();
        println!(
            "  {k:<28} {:<7} real {:>10}  modelo {:>10.3} ± {:.3}",
            level.as_str().unwrap_or(""),
            s.ultimo_real
                .map(|v| format!("{v:.3}"))
                .unwrap_or_else(|| "—".into()),
            s.simulado_media,
            s.simulado_dp
        );
    }
    if r.alertas.is_empty() {
        println!("  nenhum alerta");
    }
    for a in &r.alertas {
        let level = serde_json::to_value(a.nivel).unwrap_or_default();
        println!("  [{}] {}", level.as_str().unwrap_or(""), a.mensagem);
    }
}

/// Turns the forecast on with the `forecast` section of the file (or the defaults), the
/// time unit and the goals of the model's dashboard file.
fn enable_forecast(twin: &mut Twin, model_path: &Path) {
    let dash = Dashboard::load_next_to(model_path).unwrap_or_default();
    let (settings, unit) =
        crate::prever::settings_with_dashboard(twin.config().forecast.clone(), &dash);
    eprintln!(
        "Previsão ligada: {}, horizonte {:?}, {} replicações",
        match settings.interval(unit) {
            Some(e) => format!("a cada {e} unidades do modelo"),
            None => "só sob pedido".to_string(),
        },
        settings.horizon,
        settings.replications
    );
    twin.enable_forecast(settings, unit);
}

fn outputs(twin: &Twin, json: Option<&Path>, estado: Option<&Path>) -> Result<(), String> {
    let report = twin.report(json.is_some());
    summary(&report);
    if let Some(last) = twin.forecasts().last() {
        println!(
            "\n{} previsão(ões); a última (t = {:.1}):",
            twin.forecasts().len(),
            last.instante
        );
        crate::prever::print(last);
    }
    for e in twin.forecast_errors() {
        eprintln!("aviso: previsão: {e}");
    }
    if let Some(path) = json {
        let mut value = serde_json::to_value(&report).map_err(|e| e.to_string())?;
        if !twin.forecasts().is_empty() {
            value["previsoes"] = serde_json::to_value(twin.forecasts()).unwrap_or_default();
        }
        let text = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
        std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        eprintln!("Relatório em {}", path.display());
    }
    if let Some(path) = estado {
        let (state, notes) = twin.initial_state();
        for n in notes {
            eprintln!("aviso: {n}");
        }
        state
            .save(path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        eprintln!("Estado observado em {}", path.display());
    }
    Ok(())
}

pub fn run(a: Args) -> Result<(), String> {
    if let Some(Action::Reproduzir {
        gravacao,
        modelo,
        config,
        json,
        estado,
        previsao,
    }) = a.action
    {
        return reproduce(&gravacao, modelo, config, json, estado, previsao);
    }
    let model_path = a.model.expect("clap exige o modelo");
    let config_path = a.config.unwrap_or_else(|| default_config(&model_path));
    let config_text = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("{}: {e}", config_path.display()))?;
    let p = utsim_twin::prepare_text(&config_text, config_path.parent(), Some(&model_path))?;
    for w in &p.warnings {
        eprintln!("aviso: {w}");
    }
    let sim = simulate(&p)?;
    let mut twin = Twin::new(p.model.clone(), p.config.clone(), sim);
    if a.previsao {
        enable_forecast(&mut twin, &model_path);
    }
    if let Some(dir) = &a.gravar {
        let header = SessionHeader {
            formato: 1,
            inicio_ms: utsim_twin::obs::now_ms(),
            config: p.config_text.clone(),
            modelo: std::fs::canonicalize(&model_path)
                .unwrap_or(model_path.clone())
                .display()
                .to_string(),
            hash_modelo: model_hash(&p.model_text),
            anonimizado: p.config.record.anonymize_people,
            retencao_dias: p.config.record.keep_days,
        };
        let rec = Recorder::start(dir, &header).map_err(|e| format!("{}: {e}", dir.display()))?;
        eprintln!(
            "Gravando em {} (retenção de {} dias; pessoas {})",
            rec.session().display(),
            header.retencao_dias,
            if header.anonimizado {
                "anonimizadas"
            } else {
                "NÃO anonimizadas"
            }
        );
        twin = twin.with_recorder(rec);
    }
    let cancel = Arc::new(AtomicBool::new(false));
    live::cancel_on_ctrl_c(cancel.clone());
    let started = live::start(connect::build(&p.config)?, &cancel)?;
    let shared = Arc::new(Shared {
        health: started.health.clone(),
        twin: Mutex::new(twin),
    });
    let _server = match a.painel {
        Some(port) => {
            let (cfg, origin) = match Dashboard::load_next_to(&model_path) {
                Ok(c) => (c, "painel do modelo".to_string()),
                Err(_) => (Dashboard::default(), "padrão automático".to_string()),
            };
            let study = Study::start(p.model.clone(), cfg, None, &origin)?;
            let extra: Arc<dyn utsim_dash::server::Extra> = Arc::new(TwinPanel {
                shared: shared.clone(),
            });
            let server = Server::start_with(study, port, Some(extra))
                .map_err(|e| format!("não foi possível escutar na porta {port}: {e}"))?;
            println!(
                "Painel do gêmeo em {}gemeo (só este computador)",
                server.url()
            );
            if a.previsao {
                println!("Previsão em {}gemeo/previsao", server.url());
            }
            Some(server)
        }
        None => None,
    };
    println!(
        "Gêmeo em modo sombra (só leitura): {} conexão(ões); Ctrl+C encerra",
        shared.health.len()
    );
    let end = live::run(
        &shared,
        started,
        &LiveOptions {
            tick: Duration::from_millis(100),
            until: a.ate,
            max_wall: None,
            cancel,
        },
    );
    eprintln!("Sessão encerrada no tempo {end:.1}");
    let twin = shared.twin.lock().map_err(|_| "gêmeo indisponível")?;
    if twin.record_errors() > 0 {
        eprintln!(
            "aviso: {} observação(ões) não gravadas (erro de escrita)",
            twin.record_errors()
        );
    }
    outputs(&twin, a.json.as_deref(), a.estado.as_deref())
}

fn reproduce(
    path: &Path,
    model: Option<PathBuf>,
    config: Option<PathBuf>,
    json: Option<PathBuf>,
    estado: Option<PathBuf>,
    previsao: bool,
) -> Result<(), String> {
    let rec = Recording::open(path)?;
    let (config_text, config_dir) = match &config {
        Some(c) => (
            std::fs::read_to_string(c).map_err(|e| format!("{}: {e}", c.display()))?,
            c.parent().map(Path::to_path_buf),
        ),
        None => (rec.header.config.clone(), None),
    };
    if config.is_some() && config_text != rec.header.config {
        eprintln!(
            "aviso: o arquivo do gêmeo difere do gravado; as regras são achadas pela posição"
        );
    }
    let model_path = model.unwrap_or_else(|| PathBuf::from(&rec.header.modelo));
    let p = utsim_twin::prepare_text(&config_text, config_dir.as_deref(), Some(&model_path))?;
    if model_hash(&p.model_text) != rec.header.hash_modelo {
        eprintln!(
            "aviso: o modelo {} mudou desde a gravação; os resultados podem diferir",
            model_path.display()
        );
    }
    eprintln!(
        "Reproduzindo {} ({} linhas), sem rede",
        rec.session.display(),
        rec.lines.len()
    );
    let sim = simulate(&p)?;
    let mut twin = Twin::new(p.model, p.config, sim);
    if previsao {
        enable_forecast(&mut twin, &model_path);
    }
    utsim_twin::replay(&mut twin, &rec);
    if previsao {
        let recorded = rec
            .lines
            .iter()
            .any(|l| matches!(l, utsim_twin::record::Line::Forecast { .. }));
        if !recorded {
            eprintln!("a gravação não tem previsões; prevendo a partir do fim da sessão");
            twin.request_forecast();
        }
        twin.run_forecasts();
    }
    outputs(&twin, json.as_deref(), estado.as_deref())
}
