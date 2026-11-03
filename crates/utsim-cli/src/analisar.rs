//! `utsim analisar`: output analyses of a model (warm-up, precision, batch means,
//! Little's law, shifting bottleneck). Without any analysis flag, runs all of them
//! except batch means (which needs a long single run). The analyses of the second
//! group (scenario comparison, correlation, heat map, observations) run only when asked
//! for, and asking for one of them alone does not run the first group.

use std::io::Write;
use std::path::PathBuf;

use utsim_core::stats::Summary;
use utsim_model::Model;
use utsim_model::analysis::{self, WarmupSeries};
use utsim_model::experiment::{Experiment, ExperimentResult, run_scenarios};
use utsim_scene::heatmap::{HeatGroup, HeatMeasure, Heatmap};
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
    pub compare: Option<PathBuf>,
    pub correlation: bool,
    pub heatmap: Option<PathBuf>,
    pub cell: f64,
    pub group: String,
    pub observations: Option<PathBuf>,
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
    let group = HeatGroup::parse(&args.group)
        .ok_or_else(|| format!("--por inválido: \"{}\" (use recurso ou classe)", args.group))?;
    if !(args.cell.is_finite() && args.cell > 0.0) {
        return Err("--celula deve ser positiva (em metros)".into());
    }
    let second = args.compare.is_some()
        || args.correlation
        || args.heatmap.is_some()
        || args.observations.is_some();
    let all = !(args.warmup
        || target.is_some()
        || args.batches
        || args.little
        || args.bottleneck
        || second);
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
    if let Some(file) = &args.compare {
        let experiment = Experiment::load(file)?;
        let mut base = model.clone();
        if args.reps.is_none()
            && let Some(r) = experiment.replications
        {
            base.run.replications = r;
        }
        let scenarios = run_scenarios(
            &base,
            &experiment.scenarios,
            &experiment.measures,
            &|_, _| true,
        )?;
        let result = ExperimentResult {
            name: experiment.name.clone(),
            measures: experiment.measures.clone(),
            scenarios,
        };
        let metrics = if args.metrics.is_empty() {
            analysis::default_comparison_metrics(&result)
        } else {
            args.metrics.clone()
        };
        let rows: Vec<_> = metrics
            .iter()
            .map(|m| analysis::compare_scenarios(&result, m, level))
            .collect();
        print_comparison(&result, &rows, level);
        json.insert("cenarios".into(), serde_json::to_value(&rows)?);
    }
    if args.correlation {
        let reports = model.run_replications()?;
        let metrics = if args.metrics.is_empty() {
            analysis::default_correlation_metrics(&reports)
        } else {
            args.metrics.clone()
        };
        let m = analysis::correlation(&reports, &metrics);
        print_correlation(&m);
        json.insert("correlacao".into(), serde_json::to_value(&m)?);
    }
    if let Some(out) = &args.heatmap {
        let replay = model.replay(0)?;
        let map = Heatmap::build(
            &replay,
            Some(&model),
            args.cell,
            group,
            replay.report.stats_from,
            replay.report.end_time,
        );
        std::fs::write(out, map.to_svg(&replay, None, HeatMeasure::Time))?;
        print_heatmap(&map);
        println!("mapa de calor gravado em {}", out.display());
        let layers: Vec<_> = std::iter::once(&map.total)
            .chain(&map.layers)
            .map(|l| {
                serde_json::json!({
                    "camada": l.name,
                    "distancia": l.distance,
                    "tempo_em_movimento": l.moving,
                    "celulas": map.hottest(l, HeatMeasure::Time, 20),
                })
            })
            .collect();
        json.insert(
            "mapa_calor".into(),
            serde_json::json!({"celula": map.cell, "camadas": layers}),
        );
    }
    if let Some(out) = &args.observations {
        let reports = analysis::run_for_observations(&model)?;
        let mut file = std::io::BufWriter::new(std::fs::File::create(out)?);
        let rows = analysis::write_observations(&reports, &mut file)?;
        file.flush()?;
        println!(
            "\n{rows} observações de {} replicação(ões) gravadas em {} (formato longo)",
            reports.len(),
            out.display()
        );
        if model.run.sample_interval.is_none() {
            println!(
                "o modelo não amostra o conteúdo (run.sample_interval); só as permanências e \
                 medidas dos objetos foram gravadas"
            );
        }
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

fn short(metric: &str) -> String {
    metric.strip_suffix(".mean").unwrap_or(metric).to_string()
}

fn p_text(p: f64) -> String {
    if p.is_nan() {
        "-".to_string()
    } else if p < 1e-4 {
        "< 0.0001".to_string()
    } else {
        format!("= {p:.4}")
    }
}

fn print_comparison(result: &ExperimentResult, rows: &[analysis::ScenarioComparison], level: f64) {
    let pct = (100.0 * level * 1e6).round() / 1e6;
    println!(
        "\n== Comparação de cenários ({}: {} cenário(s), mesmas sementes) ==",
        result.name,
        result.scenarios.len()
    );
    if result.scenarios.len() < 2 {
        println!("são precisos ao menos dois cenários");
        return;
    }
    for c in rows {
        println!("\n{}", c.metric);
        for (name, s) in c.scenarios.iter().zip(&c.summaries) {
            println!("  {name:<24} {:>26}", fmt(s));
        }
        match &c.anova {
            Some(a) => println!(
                "  ANOVA de um fator: F({}, {}) = {:.4}, p {} ({})",
                a.df_between,
                a.df_within,
                a.f,
                p_text(a.p_value),
                if a.p_value < 1.0 - level {
                    "os cenários diferem"
                } else {
                    "sem diferença detectada entre os cenários"
                }
            ),
            None => println!("  ANOVA: são precisas ao menos duas replicações por cenário"),
        }
        if let Some(t) = &c.tukey {
            println!(
                "  Tukey HSD ({pct}% simultâneo, q = {:.4}; supõe cenários independentes):",
                t.q_critical
            );
            for p in &t.pairs {
                println!(
                    "    {:<22} - {:<22} {:>+12.4} [{:+.4}; {:+.4}] p {:<10} {}",
                    c.scenarios[p.i],
                    c.scenarios[p.j],
                    p.difference,
                    p.low(),
                    p.high(),
                    p_text(p.p_value),
                    if p.significant { "difere" } else { "≈" }
                );
            }
        }
        if let Some(b) = &c.bonferroni {
            println!(
                "  Pares com Bonferroni (números aleatórios comuns, {} comparação(ões), cada \
                 intervalo a {:.4}%):",
                b.comparisons,
                100.0 * b.level_each
            );
            for p in &b.pairs {
                println!(
                    "    {:<22} - {:<22} {:>+12.4} [{:+.4}; {:+.4}] p {:<10} {}",
                    c.scenarios[p.i],
                    c.scenarios[p.j],
                    p.difference,
                    p.low(),
                    p.high(),
                    p_text(p.p_value),
                    if p.significant { "difere" } else { "≈" }
                );
            }
        }
    }
    println!(
        "\nComo os cenários usam as mesmas sementes, os pares com Bonferroni costumam ser mais \
         precisos que o Tukey (que trata os cenários como independentes); p é o valor-p ajustado."
    );
}

fn print_correlation(m: &utsim_stats::correlation::CorrelationMatrix) {
    println!("\n== Correlação entre métricas ({} replicações) ==", m.n);
    if m.names.len() < 2 || m.n < 3 {
        println!("são precisas ao menos duas métricas que variam e três replicações");
        return;
    }
    let labels: Vec<String> = m.names.iter().map(|n| short(n)).collect();
    println!("Pearson (acima da diagonal) e Spearman (abaixo):");
    print!("{:>4}", "");
    for k in 0..labels.len() {
        print!(" {:>7}", format!("[{}]", k + 1));
    }
    println!();
    for i in 0..labels.len() {
        print!("{:>4}", format!("[{}]", i + 1));
        for j in 0..labels.len() {
            let v = if i == j {
                f64::NAN
            } else if j > i {
                m.pearson[i][j]
            } else {
                m.spearman[i][j]
            };
            if v.is_nan() {
                print!(" {:>7}", if i == j { "·" } else { "-" });
            } else {
                print!(" {v:>7.3}");
            }
        }
        println!("  {}", labels[i]);
    }
    println!("pares mais fortes:");
    for p in m.strongest().iter().take(8) {
        println!(
            "  {} x {}: Pearson {:+.3} (p {}), Spearman {:+.3} (p {})",
            short(&p.a),
            short(&p.b),
            p.pearson,
            p_text(p.p_pearson),
            p.spearman,
            p_text(p.p_spearman)
        );
    }
    println!(
        "correlação não é causa: duas métricas podem andar juntas porque ambas dependem de uma \
         terceira (a chegada de itens, por exemplo)"
    );
}

fn print_heatmap(map: &Heatmap) {
    println!(
        "\n== Mapa de calor de deslocamentos (replicação 0, células de {} m, {} x {}) ==",
        map.cell, map.cols, map.rows
    );
    if map.total.moving <= 0.0 {
        println!("nada se deslocou no intervalo (o modelo não tem transportadores nem transfer)");
        return;
    }
    println!(
        "{:<24} {:>14} {:>16}",
        "camada", "distância (m)", "tempo em mov."
    );
    for l in &map.layers {
        println!("{:<24} {:>14.1} {:>16.2}", l.name, l.distance, l.moving);
    }
    println!("células mais ocupadas (todas as camadas):");
    for c in map.hottest(&map.total, HeatMeasure::Time, 5) {
        println!(
            "  ({:.2}; {:.2}) m: {:.2} de ocupação, {} passagens",
            c.x, c.y, c.time, c.passes
        );
    }
}
