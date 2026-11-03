//! `utsim fluxograma`: process flowchart and process chart of a model.

use std::path::PathBuf;

use utsim_diagram::{Direction, Options, Symbol};
use utsim_model::Model;
use utsim_model::analysis::object_stats;

pub struct Args {
    pub model: PathBuf,
    pub svg: Option<PathBuf>,
    pub html: Option<PathBuf>,
    pub mermaid: Option<PathBuf>,
    pub csv: Option<PathBuf>,
    pub with_results: bool,
    pub horizontal: bool,
    pub no_times: bool,
    pub no_distances: bool,
    pub symbols: Vec<String>,
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let model = Model::load(&args.model)?;
    model.validate()?;
    let mut options = Options {
        direction: if args.horizontal {
            Direction::Horizontal
        } else {
            Direction::Vertical
        },
        show_times: !args.no_times,
        show_distances: !args.no_distances,
        ..Options::default()
    };
    for spec in &args.symbols {
        let (name, symbol) = spec
            .split_once('=')
            .ok_or_else(|| format!("--simbolo espera objeto=simbolo, recebi '{spec}'"))?;
        let name = name.trim();
        if !model.objects.iter().any(|o| o.name == name) {
            return Err(format!("--simbolo: não há objeto '{name}' no modelo").into());
        }
        let symbol = Symbol::from_id(symbol).ok_or_else(|| {
            let known: Vec<&str> = Symbol::ALL.iter().map(|s| s.id()).collect();
            format!(
                "--simbolo: símbolo '{}' desconhecido (use {})",
                symbol.trim(),
                known.join(", ")
            )
        })?;
        options.overrides.insert(name.to_string(), symbol);
    }
    if args.with_results {
        let reports = model.run_replications()?;
        options.stats = Some(utsim_diagram::stats_from_run(&object_stats(
            &model, &reports,
        )));
    }
    let graph = utsim_diagram::flowchart(&model, &options);
    let mut wrote = false;
    let mut write = |path: &Option<PathBuf>, text: String| -> std::io::Result<()> {
        if let Some(p) = path {
            std::fs::write(p, text)?;
            println!("gravado em {}", p.display());
            wrote = true;
        }
        Ok(())
    };
    write(&args.svg, utsim_diagram::svg(&graph))?;
    write(&args.html, utsim_diagram::html(&graph))?;
    write(&args.mermaid, utsim_diagram::mermaid(&graph))?;
    write(&args.csv, utsim_diagram::carta_csv(&graph))?;
    if !wrote {
        // Nothing asked for: print the Mermaid text, ready to pipe or paste.
        print!("{}", utsim_diagram::mermaid(&graph));
    }
    Ok(())
}
