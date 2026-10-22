//! `utsim experiment`: runs scenarios and prints a comparison table.

use std::path::Path;

use utsim_core::stats::Summary;
use utsim_model::ModelError;
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
    let measures: Vec<String> = if result.measures.is_empty() {
        let mut all: Vec<String> = result
            .scenarios
            .iter()
            .flat_map(|s| s.summary.keys().cloned())
            .collect();
        all.sort();
        all.dedup();
        all
    } else {
        result.measures.clone()
    };
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

    println!(
        "{}: média ± meia-amplitude do IC de 95% entre replicações\n",
        result.name
    );
    print!("{:<first$}", "medida");
    for (s, w) in result.scenarios.iter().zip(&widths) {
        print!("  {:>w$}", s.name);
    }
    println!();
    for (m, row) in measures.iter().zip(&rows) {
        print!("{m:<first$}");
        for (c, w) in row.iter().zip(&widths) {
            print!("  {c:>w$}");
        }
        println!();
    }
}
