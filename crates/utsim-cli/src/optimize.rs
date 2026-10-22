//! `utsim otimizar`: runs a simulation-based optimization and prints/writes the report.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use utsim_core::stats::Summary;
use utsim_model::{Model, ModelError};
use utsim_opt::{
    Algorithm, Evaluation, OptProblem, OptResult, Point, Sense, compare, evaluate,
    paired_difference, values,
};

pub struct Args {
    pub file: PathBuf,
    pub html: Option<PathBuf>,
    pub json: Option<PathBuf>,
    pub reps: Option<u32>,
    pub budget: Option<usize>,
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let (mut problem, dir) = OptProblem::load(&args.file)?;
    if let Some(r) = args.reps {
        problem.replications = r;
    }
    if let Some(b) = args.budget {
        problem.budget = b;
    }
    let model_path = dir.join(&problem.model);
    let base = Model::load(&model_path).map_err(|e| {
        format!(
            "não foi possível carregar o modelo '{}': {e}",
            model_path.display()
        )
    })?;
    let errors = problem.validate(&base);
    if !errors.is_empty() {
        return Err(Box::new(ModelError::Invalid(errors)));
    }

    let started = std::time::Instant::now();
    let result = search(&problem, &base)?;
    let elapsed = started.elapsed();

    let rows = build_rows(&problem, &result);
    println!("{}", text_report(&problem, &result, &rows));
    println!(
        "{} ponto(s) avaliado(s) em {:.2?}",
        result.evaluations.len(),
        elapsed
    );
    if let Some(path) = &args.html {
        write(path, &html_report(&problem, &result, &rows))?;
        println!("relatório HTML gravado em {}", path.display());
    }
    if let Some(path) = &args.json {
        let text = serde_json::to_string_pretty(&result).expect("result serializes");
        write(path, &text)?;
        println!("resultado gravado em {}", path.display());
    }
    Ok(())
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text)
        .map_err(|e| format!("não foi possível gravar '{}': {e}", path.display()))
}

/// Runs the search, reporting progress on stderr. The exhaustive search is driven here
/// point by point so progress is live; other algorithms go through `utsim_opt::optimize`
/// and their progress is replayed from the evaluation order.
fn search(problem: &OptProblem, base: &Model) -> Result<OptResult, ModelError> {
    if !matches!(problem.algorithm, Algorithm::Grid) {
        let result = utsim_opt::optimize(problem, base)?;
        let mut best: Option<f64> = None;
        for (i, e) in result.evaluations.iter().enumerate() {
            best = update_best(problem.objective.sense, best, e);
            eprintln!("{}", progress_line(i + 1, result.evaluations.len(), best));
        }
        return Ok(result);
    }
    let domains: Vec<(String, Vec<String>)> = problem
        .variables
        .iter()
        .map(|v| (v.key.clone(), values(&v.domain)))
        .collect();
    let total: usize = domains.iter().map(|(_, vs)| vs.len()).product();
    let count = total.min(problem.budget);
    if count < total {
        eprintln!(
            "aviso: orçamento de {count} ponto(s) menor que o espaço de {total}; a busca será parcial"
        );
    }
    let mut evaluations = Vec::with_capacity(count);
    let mut best = None;
    for index in 0..count {
        let mut rest = index;
        let mut point = Point::new();
        for (key, vs) in domains.iter().rev() {
            point.insert(key.clone(), vs[rest % vs.len()].clone());
            rest /= vs.len();
        }
        let e = evaluate(base, problem, &point)?;
        best = update_best(problem.objective.sense, best, &e);
        eprintln!("{}", progress_line(index + 1, count, best));
        evaluations.push(e);
    }
    Ok(compare(problem, evaluations))
}

fn update_best(sense: Sense, best: Option<f64>, e: &Evaluation) -> Option<f64> {
    if !e.feasible {
        return best;
    }
    let m = e.objective.mean;
    match best {
        Some(b) if !is_better(sense, m, b) => Some(b),
        _ => Some(m),
    }
}

fn progress_line(done: usize, total: usize, best: Option<f64>) -> String {
    match best {
        Some(b) => format!("[{done}/{total}] melhor viável até agora: {b:.4}"),
        None => format!("[{done}/{total}] nenhum ponto viável até agora"),
    }
}

fn is_better(sense: Sense, a: f64, b: f64) -> bool {
    match sense {
        Sense::Minimize => a < b,
        Sense::Maximize => a > b,
    }
}

/// One line of the table of evaluated points.
#[derive(Debug, Clone)]
pub struct Row {
    /// 1-based position in evaluation order.
    pub order: usize,
    pub point: Point,
    pub objective: Summary,
    pub feasible: bool,
    pub is_best: bool,
    /// Statistically equivalent to the best (paired test).
    pub equivalent: bool,
    /// Paired difference against the best point (none for the best itself).
    pub delta: Option<Summary>,
}

/// Rows sorted by objective (according to the sense), feasible points first.
pub fn build_rows(problem: &OptProblem, result: &OptResult) -> Vec<Row> {
    let sense = problem.objective.sense;
    let best = result.best.map(|b| &result.evaluations[b]);
    let mut rows: Vec<Row> = result
        .evaluations
        .iter()
        .enumerate()
        .map(|(i, e)| Row {
            order: i + 1,
            point: e.point.clone(),
            objective: e.objective,
            feasible: e.feasible,
            is_best: result.best == Some(i),
            equivalent: result.equivalent.contains(&i),
            delta: match best {
                Some(b) if result.best != Some(i) => Some(paired_difference(e, b)),
                _ => None,
            },
        })
        .collect();
    rows.sort_by(|a, b| {
        b.feasible.cmp(&a.feasible).then_with(|| {
            let o = a
                .objective
                .mean
                .partial_cmp(&b.objective.mean)
                .unwrap_or(std::cmp::Ordering::Equal);
            if sense == Sense::Maximize {
                o.reverse()
            } else {
                o
            }
        })
    });
    rows
}

fn mean_hw(s: &Summary) -> String {
    if s.half_width.is_nan() {
        format!("{:.4}", s.mean)
    } else {
        format!("{:.4} ± {:.4}", s.mean, s.half_width)
    }
}

/// Difference cell: `Δ ± hw`, with `≈` when the interval contains zero.
pub fn delta_cell(d: &Summary) -> String {
    let insignificant = d.half_width.is_nan() || d.mean.abs() <= d.half_width;
    let text = if d.half_width.is_nan() {
        format!("{:+.4}", d.mean)
    } else {
        format!("{:+.4} ± {:.4}", d.mean, d.half_width)
    };
    if insignificant {
        format!("{text} ≈")
    } else {
        text
    }
}

/// Delta cell of a non-best row, flagging statistical equivalents.
fn row_mark(r: &Row) -> String {
    let text = r.delta.as_ref().map(delta_cell).unwrap_or_default();
    if r.equivalent {
        format!("{text} equivalente")
    } else {
        text
    }
}

fn point_text(point: &Point) -> String {
    point
        .iter()
        .map(|(k, v)| format!("{k} = {v}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Scenario line ready to paste into an experiment file.
pub fn scenario_line(name: &str, point: &Point) -> String {
    let set = point
        .iter()
        .map(|(k, v)| format!("{}: {}", ron_string(k), ron_string(v)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("(name: {}, set: {{ {set} }})", ron_string(name))
}

fn ron_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn bound_text(problem: &OptProblem, index: usize) -> String {
    let c = &problem.constraints[index];
    match (c.min, c.max) {
        (Some(lo), Some(hi)) => format!("{lo} ≤ {} ≤ {hi}", c.metric),
        (Some(lo), None) => format!("{} ≥ {lo}", c.metric),
        (None, Some(hi)) => format!("{} ≤ {hi}", c.metric),
        (None, None) => c.metric.clone(),
    }
}

/// Constraint lines of one evaluation: `(ok, text)`.
fn constraint_lines(problem: &OptProblem, e: &Evaluation) -> Vec<(bool, String)> {
    e.constraints
        .iter()
        .enumerate()
        .map(|(i, (_, s, ok))| {
            (
                *ok,
                format!(
                    "{} (média observada: {})",
                    bound_text(problem, i),
                    mean_hw(s)
                ),
            )
        })
        .collect()
}

fn sense_text(sense: Sense) -> &'static str {
    match sense {
        Sense::Minimize => "minimizar",
        Sense::Maximize => "maximizar",
    }
}

pub fn text_report(problem: &OptProblem, result: &OptResult, rows: &[Row]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Otimização: {}", problem.name);
    let _ = writeln!(
        out,
        "Objetivo: {}; {} replicação(ões) por ponto; {} ponto(s) avaliado(s)\n",
        sense_text(problem.objective.sense),
        problem.replications,
        result.evaluations.len()
    );
    match result.best {
        Some(b) => {
            let e = &result.evaluations[b];
            let _ = writeln!(out, "Melhor ponto");
            for (k, v) in &e.point {
                let _ = writeln!(out, "  {k} = {v}");
            }
            let _ = writeln!(out, "  objetivo: {}", mean_hw(&e.objective));
            if !e.constraints.is_empty() {
                let _ = writeln!(out, "\nRestrições do melhor ponto");
                for (ok, line) in constraint_lines(problem, e) {
                    let _ = writeln!(out, "  {} {line}", if ok { "✓" } else { "✗" });
                }
            }
            if !result.equivalent.is_empty() {
                let _ = writeln!(out, "\nPontos equivalentes ao melhor (95%, teste pareado)");
                for &i in &result.equivalent {
                    let ev = &result.evaluations[i];
                    let _ = writeln!(
                        out,
                        "  {}  (objetivo {})",
                        point_text(&ev.point),
                        mean_hw(&ev.objective)
                    );
                }
            }
            let _ = writeln!(out, "\nCenário para colar em um arquivo de experimento");
            let _ = writeln!(out, "  {},", scenario_line("otimo", &e.point));
        }
        None => {
            let _ = writeln!(out, "Nenhum ponto avaliado atende às restrições.");
        }
    }

    let _ = writeln!(
        out,
        "\nPontos avaliados (viáveis primeiro, melhor objetivo no topo)"
    );
    let _ = writeln!(
        out,
        "  {:>3}  {:<4} {:<40} {:<22} viável / Δ vs melhor (≈ = sem diferença)",
        "#", "ord.", "ponto", "objetivo"
    );
    for (rank, r) in rows.iter().enumerate() {
        let mark = if r.is_best {
            "melhor".to_string()
        } else {
            row_mark(r)
        };
        let flag = if r.feasible { "✓" } else { "✗" };
        let _ = writeln!(
            out,
            "  {:>3}  {:<4} {:<40} {:<22} {} {}",
            rank + 1,
            r.order,
            point_text(&r.point),
            mean_hw(&r.objective),
            flag,
            mark
        );
    }

    let _ = writeln!(out, "\nExplicação");
    for line in &result.explanation {
        let _ = writeln!(out, "  {line}");
    }
    out
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const CSS: &str = r#"
:root{--bg:#fff;--fg:#1c1f24;--muted:#5d6672;--line:#d8dde3;--accent:#2563eb;--good:#15803d;--bad:#b91c1c;--card:#f5f7fa}
@media(prefers-color-scheme:dark){:root{--bg:#14171b;--fg:#e6e9ee;--muted:#9aa4b0;--line:#2d343c;--accent:#6ea0ff;--good:#4ade80;--bad:#f87171;--card:#1d2228}}
body{font:15px/1.5 system-ui,sans-serif;background:var(--bg);color:var(--fg);max-width:1000px;margin:2rem auto;padding:0 1rem}
h1{font-size:1.5rem}h2{font-size:1.15rem;margin-top:2rem}
table{border-collapse:collapse;width:100%;font-size:.9rem}
th,td{padding:.35rem .6rem;border-bottom:1px solid var(--line);text-align:left}
th{color:var(--muted);font-weight:600}
tr.best td{background:var(--card);font-weight:600}
code,pre{font-family:ui-monospace,monospace;font-size:.85rem}
pre{background:var(--card);padding:.75rem;border-radius:6px;overflow-x:auto}
.ok{color:var(--good)}.no{color:var(--bad)}.muted{color:var(--muted)}
svg{max-width:100%;height:auto}
svg text{fill:var(--fg);font-size:11px}
svg .axis{stroke:var(--muted);stroke-width:1}
svg .grid{stroke:var(--line);stroke-width:1}
svg .pt{fill:var(--accent)}svg .ptx{fill:none;stroke:var(--muted);stroke-width:1.5}
svg .bestpt{fill:var(--good)}
svg .whisker{stroke:var(--muted);stroke-width:1}
svg .line{fill:none;stroke:var(--accent);stroke-width:1.5}
svg .bar{fill:var(--accent);opacity:.8}svg .barbest{fill:var(--good);opacity:.9}
"#;

pub fn html_report(problem: &OptProblem, result: &OptResult, rows: &[Row]) -> String {
    let mut h = String::new();
    let _ = write!(
        h,
        "<!doctype html><html lang=\"pt-BR\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
         <title>Otimização: {}</title><style>{CSS}</style></head><body>",
        esc(&problem.name)
    );
    let _ = write!(h, "<h1>Otimização: {}</h1>", esc(&problem.name));
    let _ = write!(
        h,
        "<p class=\"muted\">Objetivo: {}. {} replicação(ões) por ponto, {} ponto(s) avaliado(s), \
         mesmas sementes em todos os pontos.</p>",
        sense_text(problem.objective.sense),
        problem.replications,
        result.evaluations.len()
    );

    h.push_str("<h2>Melhor ponto</h2>");
    match result.best {
        Some(b) => {
            let e = &result.evaluations[b];
            h.push_str("<ul>");
            for (k, v) in &e.point {
                let _ = write!(h, "<li><code>{} = {}</code></li>", esc(k), esc(v));
            }
            let _ = write!(
                h,
                "<li>objetivo: <b>{}</b></li></ul>",
                esc(&mean_hw(&e.objective))
            );
            if !e.constraints.is_empty() {
                h.push_str("<h2>Restrições do melhor ponto</h2><ul>");
                for (ok, line) in constraint_lines(problem, e) {
                    let (class, mark) = if ok { ("ok", "✓") } else { ("no", "✗") };
                    let _ = write!(
                        h,
                        "<li><span class=\"{class}\">{mark}</span> {}</li>",
                        esc(&line)
                    );
                }
                h.push_str("</ul>");
            }
            if !result.equivalent.is_empty() {
                h.push_str(
                    "<h2>Pontos equivalentes ao melhor</h2><p class=\"muted\">Sem diferença \
                     significativa (95%, teste pareado).</p><ul>",
                );
                for &i in &result.equivalent {
                    let ev = &result.evaluations[i];
                    let _ = write!(
                        h,
                        "<li><code>{}</code> (objetivo {})</li>",
                        esc(&point_text(&ev.point)),
                        esc(&mean_hw(&ev.objective))
                    );
                }
                h.push_str("</ul>");
            }
            let _ = write!(
                h,
                "<h2>Cenário para um arquivo de experimento</h2><pre>{},</pre>",
                esc(&scenario_line("otimo", &e.point))
            );
        }
        None => h.push_str("<p class=\"no\">Nenhum ponto avaliado atende às restrições.</p>"),
    }

    h.push_str("<h2>Objetivo por ordem de avaliação</h2>");
    h.push_str(&order_chart(problem, result));
    h.push_str("<p class=\"muted\">Linha: melhor viável até então. Círculo vazio: ponto inviável. Barras: intervalo de 95%.</p>");
    h.push_str("<h2>Os 10 melhores pontos (intervalo de 95%)</h2>");
    h.push_str(&top_chart(rows));

    h.push_str(
        "<h2>Pontos avaliados</h2><table><tr><th>#</th><th>Ord.</th><th>Ponto</th>\
         <th>Objetivo</th><th>Viável</th><th>Δ vs melhor</th></tr>",
    );
    for (rank, r) in rows.iter().enumerate() {
        let delta = if r.is_best {
            "melhor".to_string()
        } else {
            row_mark(r)
        };
        let (class, mark) = if r.feasible {
            ("ok", "✓")
        } else {
            ("no", "✗")
        };
        let _ = write!(
            h,
            "<tr{}><td>{}</td><td>{}</td><td><code>{}</code></td><td>{}</td>\
             <td class=\"{class}\">{mark}</td><td>{}</td></tr>",
            if r.is_best { " class=\"best\"" } else { "" },
            rank + 1,
            r.order,
            esc(&point_text(&r.point)),
            esc(&mean_hw(&r.objective)),
            esc(&delta)
        );
    }
    h.push_str(
        "</table><p class=\"muted\">≈ indica que o intervalo de 95% da diferença pareada contém zero.</p>",
    );

    h.push_str("<h2>Explicação</h2><ul>");
    for line in &result.explanation {
        let _ = write!(h, "<li>{}</li>", esc(line));
    }
    h.push_str("</ul></body></html>");
    h
}

/// Value range with a little padding, tolerant of empty/degenerate input.
fn padded_range(lo: f64, hi: f64) -> (f64, f64) {
    if !lo.is_finite() || !hi.is_finite() {
        return (0.0, 1.0);
    }
    let span = if hi > lo {
        hi - lo
    } else {
        lo.abs().max(1.0) * 0.1
    };
    (lo - span * 0.05, hi + span * 0.05)
}

fn bounds(items: impl Iterator<Item = (f64, f64)>) -> (f64, f64) {
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for (m, hw) in items {
        let hw = if hw.is_nan() { 0.0 } else { hw };
        lo = lo.min(m - hw);
        hi = hi.max(m + hw);
    }
    padded_range(lo, hi)
}

/// Scatter of the objective in evaluation order, with the running best of the feasible
/// points as a line. Hollow marks are infeasible points.
fn order_chart(problem: &OptProblem, result: &OptResult) -> String {
    let (w, hgt, left, right, top, bottom) = (900.0, 320.0, 60.0, 15.0, 15.0, 35.0);
    let n = result.evaluations.len().max(1);
    let (lo, hi) = bounds(
        result
            .evaluations
            .iter()
            .map(|e| (e.objective.mean, e.objective.half_width)),
    );
    let x = |i: usize| {
        if n == 1 {
            left + (w - left - right) / 2.0
        } else {
            left + (w - left - right) * i as f64 / (n - 1) as f64
        }
    };
    let y = |v: f64| top + (hgt - top - bottom) * (1.0 - (v - lo) / (hi - lo));
    let mut s = format!(
        "<svg viewBox=\"0 0 {w} {hgt}\" role=\"img\" aria-label=\"Objetivo por ordem de avaliação\">"
    );
    for k in 0..=4 {
        let v = lo + (hi - lo) * k as f64 / 4.0;
        let yy = y(v);
        let _ = write!(
            s,
            "<line class=\"grid\" x1=\"{left}\" x2=\"{}\" y1=\"{yy:.1}\" y2=\"{yy:.1}\"/>\
             <text x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{v:.3}</text>",
            w - right,
            left - 6.0,
            yy + 4.0
        );
    }
    let _ = write!(
        s,
        "<line class=\"axis\" x1=\"{left}\" x2=\"{}\" y1=\"{by}\" y2=\"{by}\"/>\
         <text x=\"{}\" y=\"{}\" text-anchor=\"middle\">ordem de avaliação</text>",
        w - right,
        (w + left) / 2.0,
        hgt - 6.0,
        by = hgt - bottom
    );
    // Running best of the feasible points.
    let mut best: Option<f64> = None;
    let mut path = String::new();
    for (i, e) in result.evaluations.iter().enumerate() {
        best = update_best(problem.objective.sense, best, e);
        if let Some(b) = best {
            let _ = write!(
                path,
                "{}{:.1},{:.1} ",
                if path.is_empty() { "M" } else { "L" },
                x(i),
                y(b)
            );
        }
    }
    if !path.is_empty() {
        let _ = write!(s, "<path class=\"line\" d=\"{path}\"/>");
    }
    for (i, e) in result.evaluations.iter().enumerate() {
        let (cx, cy) = (x(i), y(e.objective.mean));
        if !e.objective.half_width.is_nan() {
            let _ = write!(
                s,
                "<line class=\"whisker\" x1=\"{cx:.1}\" x2=\"{cx:.1}\" y1=\"{:.1}\" y2=\"{:.1}\"/>",
                y(e.objective.mean - e.objective.half_width),
                y(e.objective.mean + e.objective.half_width)
            );
        }
        let class = if result.best == Some(i) {
            "bestpt"
        } else if e.feasible {
            "pt"
        } else {
            "ptx"
        };
        let r = if result.best == Some(i) { 5 } else { 3 };
        let _ = write!(
            s,
            "<circle class=\"{class}\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r}\"><title>{}</title></circle>",
            esc(&format!("#{}: {}", i + 1, point_text(&e.point)))
        );
    }
    s.push_str("</svg>");
    s
}

/// Horizontal bars of the first 10 rows with their 95% intervals.
fn top_chart(rows: &[Row]) -> String {
    let top: Vec<&Row> = rows.iter().take(10).collect();
    if top.is_empty() {
        return "<p class=\"muted\">Sem pontos.</p>".into();
    }
    let (w, row_h, left, right) = (900.0, 26.0, 330.0, 20.0);
    let hgt = row_h * top.len() as f64 + 30.0;
    let (lo, hi) = bounds(
        top.iter()
            .map(|r| (r.objective.mean, r.objective.half_width)),
    );
    // Bars start at the lower end of the axis, so differences stay visible.
    let x = |v: f64| left + (w - left - right) * (v - lo) / (hi - lo);
    let mut s = format!(
        "<svg viewBox=\"0 0 {w} {hgt}\" role=\"img\" aria-label=\"Os 10 melhores pontos\">"
    );
    for (i, r) in top.iter().enumerate() {
        let yy = 8.0 + row_h * i as f64;
        let mut label = point_text(&r.point);
        if label.chars().count() > 48 {
            label = label.chars().take(47).collect::<String>() + "…";
        }
        let class = if r.is_best { "barbest" } else { "bar" };
        let _ = write!(
            s,
            "<text x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{}{}</text>\
             <rect class=\"{class}\" x=\"{left}\" y=\"{yy:.1}\" width=\"{:.1}\" height=\"{}\"/>",
            left - 8.0,
            yy + 13.0,
            if r.feasible { "" } else { "✗ " },
            esc(&label),
            (x(r.objective.mean) - left).max(1.0),
            row_h - 8.0
        );
        if !r.objective.half_width.is_nan() {
            let (a, b) = (
                x(r.objective.mean - r.objective.half_width),
                x(r.objective.mean + r.objective.half_width),
            );
            let m = yy + (row_h - 8.0) / 2.0;
            let _ = write!(
                s,
                "<line class=\"whisker\" x1=\"{a:.1}\" x2=\"{b:.1}\" y1=\"{m:.1}\" y2=\"{m:.1}\"/>\
                 <line class=\"whisker\" x1=\"{a:.1}\" x2=\"{a:.1}\" y1=\"{:.1}\" y2=\"{:.1}\"/>\
                 <line class=\"whisker\" x1=\"{b:.1}\" x2=\"{b:.1}\" y1=\"{:.1}\" y2=\"{:.1}\"/>",
                m - 4.0,
                m + 4.0,
                m - 4.0,
                m + 4.0
            );
        }
    }
    let _ = write!(
        s,
        "<text x=\"{left}\" y=\"{}\">{lo:.3}</text><text x=\"{}\" y=\"{}\" text-anchor=\"end\">{hi:.3}</text></svg>",
        hgt - 4.0,
        w - right,
        hgt - 4.0
    );
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summary(mean: f64, hw: f64) -> Summary {
        Summary {
            n: 5,
            mean,
            std_dev: 1.0,
            half_width: hw,
        }
    }

    fn eval(cap: &str, reps: &[f64], feasible: bool) -> Evaluation {
        Evaluation {
            point: Point::from([("m.capacity".to_string(), cap.to_string())]),
            per_replication: reps.to_vec(),
            objective: Summary::from_samples(reps),
            constraints: vec![],
            feasible,
        }
    }

    fn problem(sense: &str) -> OptProblem {
        OptProblem::parse(&format!(
            r#"OptProblem(name: "t", model: "m.ron",
                variables: [(key: "m.capacity", domain: Integer(min: 1, max: 3))],
                objective: (sense: {sense}, terms: [("x", 1.0)]))"#
        ))
        .unwrap()
    }

    #[test]
    fn rows_put_feasible_first_and_mark_equivalents() {
        let p = problem("Minimize");
        let evals = vec![
            eval("1", &[1.0, 1.1, 0.9, 1.0, 1.0], false), // lowest raw value but infeasible
            eval("2", &[5.0, 5.2, 4.8, 5.1, 4.9], true),
            eval("3", &[5.1, 5.1, 4.9, 5.0, 5.0], true),
            eval("4", &[9.0, 9.1, 8.9, 9.2, 8.8], true),
        ];
        let result = compare(&p, evals);
        let rows = build_rows(&p, &result);
        assert_eq!(rows.len(), 4);
        assert!(rows[..3].iter().all(|r| r.feasible));
        assert!(!rows[3].feasible);
        assert!(rows[0].is_best);
        assert!(rows[0].delta.is_none());
        assert!(rows[1].objective.mean <= rows[2].objective.mean);
        let near = rows.iter().find(|r| r.order == 3).unwrap();
        assert!(near.equivalent);
        assert!(delta_cell(near.delta.as_ref().unwrap()).ends_with('≈'));
        let far = rows.iter().find(|r| r.order == 4).unwrap();
        assert!(!far.equivalent);
        assert!(!delta_cell(far.delta.as_ref().unwrap()).contains('≈'));
    }

    #[test]
    fn maximize_sorts_descending() {
        let p = problem("Maximize");
        let evals = vec![
            eval("1", &[1.0, 1.1, 0.9], true),
            eval("2", &[9.0, 9.1, 8.9], true),
        ];
        let result = compare(&p, evals);
        let rows = build_rows(&p, &result);
        assert_eq!(rows[0].order, 2);
        assert!(rows[0].is_best);
    }

    #[test]
    fn formatting_helpers() {
        assert_eq!(mean_hw(&summary(1.5, f64::NAN)), "1.5000");
        assert_eq!(mean_hw(&summary(1.5, 0.25)), "1.5000 ± 0.2500");
        assert!(delta_cell(&summary(0.1, 0.5)).ends_with('≈'));
        assert!(!delta_cell(&summary(2.0, 0.5)).contains('≈'));
        let point = Point::from([
            ("a.b".to_string(), "Exp(mean: 1.0)".to_string()),
            ("c.d".to_string(), "3".to_string()),
        ]);
        assert_eq!(
            scenario_line("otimo", &point),
            r#"(name: "otimo", set: { "a.b": "Exp(mean: 1.0)", "c.d": "3" })"#
        );
    }

    #[test]
    fn reports_render_with_and_without_best() {
        let p = problem("Minimize");
        let result = compare(&p, vec![eval("1", &[1.0, 2.0, 3.0], false)]);
        let rows = build_rows(&p, &result);
        let text = text_report(&p, &result, &rows);
        assert!(text.contains("Nenhum ponto avaliado atende"));
        let html = html_report(&p, &result, &rows);
        assert!(html.contains("<svg") && !html.contains("http://") && !html.contains("https://"));

        let result = compare(&p, vec![eval("1", &[1.0, 2.0, 3.0], true)]);
        let rows = build_rows(&p, &result);
        let text = text_report(&p, &result, &rows);
        assert!(text.contains("set: {"));
    }
}
