//! `utsim experiment`: runs scenarios and prints a comparison table.

use std::path::Path;

use utsim_core::stats::Summary;
use utsim_model::ModelError;
use utsim_model::analysis::delta_cell;
use utsim_model::experiment::{Experiment, ExperimentResult};

pub fn run(file: &Path, json: Option<&Path>) -> Result<(), ModelError> {
    let experiment = Experiment::load(file)?;
    let started = std::time::Instant::now();
    let result = experiment.run()?;
    print_table(&result);
    println!(
        "\n{} cenário(s) em {:.2?}",
        result.scenarios.len(),
        started.elapsed()
    );
    if let Some(path) = json {
        let text = serde_json::to_string_pretty(&result).expect("result serializes");
        std::fs::write(path, text)?;
        println!("resultado gravado em {}", path.display());
    }
    Ok(())
}

fn cell(s: Option<&Summary>) -> String {
    match s {
        None => "-".to_string(),
        Some(s) if s.half_width.is_nan() => format!("{:.4}", s.mean),
        Some(s) => format!("{:.4} ± {:.4}", s.mean, s.half_width),
    }
}

fn print_table(result: &ExperimentResult) {
    let measures = result.metric_names();
    let rows: Vec<Vec<String>> = measures
        .iter()
        .map(|m| {
            result
                .scenarios
                .iter()
                .map(|s| cell(s.summary.get(m)))
                .collect()
        })
        .collect();
    print_grid(
        &format!(
            "{}: média ± meia-amplitude do IC de 95% entre replicações",
            result.name
        ),
        result,
        &measures,
        &rows,
    );
    if result.scenarios.len() < 2 {
        return;
    }
    // Paired differences to the first scenario (same seeds); `≈`: not significant.
    let deltas: Vec<Vec<String>> = measures
        .iter()
        .map(|m| {
            (0..result.scenarios.len())
                .map(|i| match result.difference_to_first(i, m) {
                    Some(d) => delta_cell(&d),
                    None if i == 0 => "base".to_string(),
                    None => "-".to_string(),
                })
                .collect()
        })
        .collect();
    println!();
    print_grid(
        &format!(
            "Diferença pareada para '{}' (Δ ± IC de 95%; ≈ = sem diferença significativa)",
            result.scenarios[0].name
        ),
        result,
        &measures,
        &deltas,
    );
}

fn print_grid(title: &str, result: &ExperimentResult, measures: &[String], rows: &[Vec<String>]) {
    let first = measures
        .iter()
        .map(String::len)
        .max()
        .unwrap_or(0)
        .max("medida".len());
    let widths: Vec<usize> = result
        .scenarios
        .iter()
        .enumerate()
        .map(|(i, s)| {
            rows.iter()
                .map(|r| r[i].chars().count())
                .max()
                .unwrap_or(0)
                .max(s.name.chars().count())
        })
        .collect();

    println!("{title}\n");
    print!("{:<first$}", "medida");
    for (s, w) in result.scenarios.iter().zip(&widths) {
        print!("  {:>w$}", s.name);
    }
    println!();
    for (m, row) in measures.iter().zip(rows) {
        print!("{m:<first$}");
        for (c, w) in row.iter().zip(&widths) {
            print!("  {c:>w$}");
        }
        println!();
    }
}
