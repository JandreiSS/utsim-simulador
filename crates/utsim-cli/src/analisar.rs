//! `utsim analisar`: output analyses of a model (warm-up, precision, batch means,
//! Little's law, shifting bottleneck). Without any analysis flag, runs all of them
//! except batch means (which needs a long single run).

use std::path::PathBuf;

use utsim_core::stats::Summary;
use utsim_model::Model;
use utsim_model::analysis::{self, WarmupSeries};
use utsim_stats::little::DEFAULT_TOLERANCE;

pub struct Args {
    pub model: PathBuf,
    pub warmup: bool,
    pub precision: Option<String>,
    pub batches: bool,
    pub little: bool,
    pub bottleneck: bool,
    pub level: String,
    pub measure: Option<String>,
    pub metrics: Vec<String>,
    pub window: usize,
    pub reps: Option<u32>,
    pub stop: Option<f64>,
    pub json: Option<PathBuf>,
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let level = utsim_stats::parse_level(&args.level).ok_or_else(|| {
        format!(
            "nível de confiança inválido: \"{}\" (use, por exemplo, 95% ou 0.9)",
            args.level
        )
    })?;
    let target = args
        .precision
        .as_deref()
        .map(|t| {
            analysis::parse_target(t).ok_or_else(|| {
                format!("alvo de precisão inválido: \"{t}\" (use 2% ou um valor absoluto como 0.5)")
            })
        })
        .transpose()?;
    let measure = args
        .measure
        .as_deref()
        .map(|m| {
            WarmupSeries::parse(m).ok_or_else(|| {
                format!("medida inválida: \"{m}\" (use objeto.medida, como saida.time_in_system)")
            })
        })
        .transpose()?;
    let mut model = Model::load(&args.model)?;
    if let Some(v) = args.reps {
        model.run.replications = v;
    }
    if let Some(v) = args.stop {
        model.run.stop_time = v;
    }
    model.validate()?;
    for warning in model.warnings() {
        eprintln!("aviso: {warning}");
    }
    let all = !(args.warmup || target.is_some() || args.batches || args.little || args.bottleneck);
    // 95, 99.5...: rounded so binary noise (0.995 · 100) does not show.
    let pct = (100.0 * level * 1e6).round() / 1e6;
    println!(
        "{}: {} replicação(ões), confiança {pct}%",
        model.name, model.run.replications
    );
    let mut json = serde_json::Map::new();
    json.insert("model".into(), model.name.clone().into());
    json.insert("level".into(), level.into());

    if args.warmup || all {
        let series = measure.clone().unwrap_or(WarmupSeries::Wip);
        let reports = analysis::run_for_warmup(&model, &series)?;
        let w = analysis::warmup(&reports, &series, args.window);
        print_warmup(&model, &series, &w);
        json.insert("aquecimento".into(), serde_json::to_value(&w)?);
    }
    if let Some(target) = target.or(all.then_some(utsim_stats::precision::Target::Relative(0.02))) {
        let reports = model.run_replications()?;
        let metrics = if args.metrics.is_empty() {
            analysis::default_precision_metrics(&reports)
        } else {
            args.metrics.clone()
        };
        let rows = analysis::precision(&reports, &metrics, target, level);
        print_precision(&rows, target);
        json.insert("precisao".into(), serde_json::to_value(&rows)?);
    }
    if args.batches {
        let (object, name) = match &measure {
            Some(WarmupSeries::Measure { object, measure }) => (object.clone(), measure.clone()),
            _ => default_batch_measure(&model)
                .ok_or("o modelo não tem sumidouro; indique a medida com --medida objeto.medida")?,
        };
        let mut one = model.clone();
        one.run.replications = 1;
        let reports = one.run_replications_timed(&[(object.clone(), name.clone())])?;
        let b = analysis::batch_means(&reports[0], &object, &name, level);
        print_batches(&format!("{object}.{name}"), b.as_ref());
        json.insert("lotes".into(), serde_json::to_value(&b)?);
    }
    if args.little || all {
        let (reports, transit) = analysis::run_for_little(&model)?;
        let r = analysis::little(&reports, transit.as_deref(), level, DEFAULT_TOLERANCE);
        print_little(&r);
        json.insert("little".into(), serde_json::to_value(&r)?);
    }
    if args.bottleneck || all {
        let traced = analysis::run_traced(&model)?;
        let shares = analysis::shifting_bottleneck(&traced, level);
        print_bottleneck(&shares);
        json.insert("gargalo_movel".into(), serde_json::to_value(&shares)?);
    }
    if let Some(path) = args.json {
        let text = serde_json::to_string_pretty(&serde_json::Value::Object(json))?;
        std::fs::write(&path, text)?;
        println!("\nresultado gravado em {}", path.display());
    }
    Ok(())
}

/// Time in system of the first sink: the usual measure for batch means.
fn default_batch_measure(model: &Model) -> Option<(String, String)> {
    model
        .objects
        .iter()
        .find(|o| matches!(o.kind, utsim_model::ObjectKind::Sink))
        .map(|o| (o.name.clone(), "time_in_system".to_string()))
}

fn fmt(s: &Summary) -> String {
    analysis::mean_hw(s)
}

fn fmt_pct(s: &Summary) -> String {
    if s.half_width.is_nan() {
        format!("{:.2}%", 100.0 * s.mean)
    } else {
        format!("{:.2}% ± {:.2}", 100.0 * s.mean, 100.0 * s.half_width)
    }
}

fn print_warmup(model: &Model, series: &WarmupSeries, w: &analysis::WarmupAnalysis) {
    println!("\n== Aquecimento ({}) ==", series.label());
    println!(
        "rodado sem aquecimento, {} pontos na média das replicações",
        w.mean.len()
    );
    let Some(m) = &w.mser else {
        println!("série curta demais para o MSER-5 (são precisos 20 pontos)");
        return;
    };
    match w.suggested {
        Some(t) => println!(
            "MSER-5: cortar os primeiros {} pontos; aquecimento sugerido: {t:.2} (atual: {})",
            m.cut, model.run.warmup
        ),
        None => println!("MSER-5: sem sugestão"),
    }
    if m.too_short {
        println!(
            "aviso: o mínimo do MSER-5 fica depois da metade da série; a rodada é curta \
             demais (ou o sistema não estabiliza): aumente o tempo de parada"
        );
    }
    // Welch: about 20 evenly spaced points of the smoothed curve, to choose by eye.
    if !w.welch.is_empty() {
        println!(
            "Welch (média móvel, janela {}), instante e valor:",
            w.window
        );
        let step = (w.welch.len() / 20).max(1);
        for i in (0..w.welch.len()).step_by(step) {
            println!("  {:>12.2} {:>12.4}", w.times[i], w.welch[i]);
        }
    }
}

fn print_precision(
    rows: &[(String, utsim_stats::precision::Precision)],
    target: utsim_stats::precision::Target,
) {
    let alvo = match target {
        utsim_stats::precision::Target::Relative(r) => format!("{:.2}% da média", 100.0 * r),
        utsim_stats::precision::Target::Absolute(h) => format!("{h}"),
    };
    println!("\n== Precisão (semiamplitude alvo: {alvo}) ==");
    println!(
        "{:<36} {:>24} {:>12} {:>12} {:>8}",
        "métrica", "atual", "alvo", "necessárias", "faltam"
    );
    let mut most = 0;
    for (k, p) in rows {
        let needed = p.needed.map_or("-".to_string(), |n| n.to_string());
        let more = p.more.map_or("-".to_string(), |n| n.to_string());
        most = most.max(p.needed.unwrap_or(0));
        println!(
            "{k:<36} {:>24} {:>12.4} {:>12} {:>8}",
            fmt(&p.current),
            p.target,
            needed,
            more
        );
    }
    if let Some((_, p)) = rows.first() {
        if most > p.current.n {
            println!(
                "rode com -n {most} para todas atingirem o alvo; as replicações já feitas se repetem \
                 iguais (a semente é por replicação)"
            );
        } else if most > 0 {
            println!("todas as métricas já atingem o alvo");
        }
    }
}

fn print_batches(name: &str, b: Option<&utsim_stats::precision::BatchMeans>) {
    println!("\n== Médias em lotes ({name}, replicação 0) ==");
    let Some(b) = b else {
        println!("observações insuficientes (são precisas pelo menos 10)");
        return;
    };
    println!(
        "{} lotes de {} observações, autocorrelação lag 1 dos lotes {:.3}",
        b.batches, b.batch_size, b.lag1
    );
    println!("média {}", fmt(&b.summary));
    if let Some(w) = &b.warning {
        println!("aviso: {w}");
    }
}

fn print_little(r: &analysis::LittleReport) {
    println!("\n== Lei de Little (L = λ·W) ==");
    println!(
        "{:<20} {:>22} {:>22} {:>22} {:>10} {:>6}",
        "objeto", "L", "λ", "W", "diferença", ""
    );
    let row = |name: &str, c: &utsim_stats::little::LittleCheck| {
        println!(
            "{name:<20} {:>22} {:>22} {:>22} {:>9.2}% {:>6}",
            fmt(&c.l),
            fmt(&c.lambda),
            fmt(&c.w),
            100.0 * c.relative,
            if c.holds { "ok" } else { "AVISO" }
        );
    };
    for o in &r.objects {
        row(&o.name, &o.check);
    }
    if let Some(sys) = &r.system {
        row("(sistema)", sys);
    }
    if let Some(t) = &r.transit {
        println!(
            "itens em trânsito (transfer), somados ao L do sistema: {}",
            fmt(t)
        );
    }
    let bad = r
        .objects
        .iter()
        .map(|o| &o.check)
        .chain(r.system.as_ref())
        .any(|c| !c.holds);
    if bad {
        println!(
            "aviso: L e λ·W não fecham onde está marcado. Costuma indicar aquecimento curto, \
             itens presos no fim da rodada ou rodada curta demais"
        );
    }
}

fn print_bottleneck(shares: &[analysis::BottleneckShare]) {
    println!("\n== Gargalo móvel (período ativo) ==");
    if shares.iter().all(|s| s.total() == 0.0) {
        println!("nenhum recurso ficou ativo");
        return;
    }
    println!(
        "{:<20} {:>20} {:>20}",
        "recurso", "gargalo único", "compartilhado"
    );
    for s in shares {
        println!(
            "{:<20} {:>20} {:>20}",
            s.name,
            fmt_pct(&s.sole),
            fmt_pct(&s.shifting)
        );
    }
    if let Some(top) = shares.first() {
        println!(
            "gargalo principal: {} ({:.2}% do tempo)",
            top.name,
            100.0 * top.total()
        );
    }
}
