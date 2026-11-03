//! `utsim fluxograma`: process flowchart and process chart of a model.

use std::path::PathBuf;

use utsim_diagram::{Direction, Options, Symbol, TipoDiagrama};
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
    pub kind: Option<String>,
    pub replication: u32,
    pub from: Option<f64>,
    pub to: Option<f64>,
    pub compare: Option<PathBuf>,
    pub lanes: Vec<String>,
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
    options.replication = args.replication;
    options.window = match (args.from, args.to) {
        (Some(a), Some(b)) => Some((a, b)),
        (None, None) => None,
        _ => return Err("--de e --ate vão juntos".into()),
    };
    for spec in &args.lanes {
        let (name, lane) = spec
            .split_once('=')
            .ok_or_else(|| format!("--raia espera objeto=nome, recebi '{spec}'"))?;
        options
            .areas
            .insert(name.trim().to_string(), lane.trim().to_string());
    }
    if let Some(p) = &args.compare {
        let other = Model::load(p)?;
        other.validate()?;
        options.compare = Some(Box::new(other));
    }
    if let Some(kind) = &args.kind {
        return run_kind(&args, &model, &options, kind);
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

/// `--tipo`: one kind of diagram, or all of them.
fn run_kind(
    args: &Args,
    model: &Model,
    options: &Options,
    kind: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if kind.trim().eq_ignore_ascii_case("todos") {
        let index = args.html.as_ref().ok_or(
            "--tipo todos precisa de --html (o arquivo do índice; os outros ficam ao lado)",
        )?;
        let dir = index.parent().map(PathBuf::from).unwrap_or_default();
        let stem = index.file_stem().map_or("fluxograma".to_string(), |s| {
            s.to_string_lossy().to_string()
        });
        let mut items = Vec::new();
        for t in TipoDiagrama::ALL {
            match utsim_diagram::gerar(t, model, options) {
                Ok(out) => {
                    let base = format!("{stem}-{}", t.id());
                    std::fs::write(dir.join(format!("{base}.html")), &out.html)?;
                    if let Some(s) = &out.svg {
                        std::fs::write(dir.join(format!("{base}.svg")), s)?;
                    }
                    if let Some(s) = &out.csv {
                        std::fs::write(dir.join(format!("{base}.csv")), s)?;
                    }
                    if let Some(s) = &out.mermaid {
                        std::fs::write(dir.join(format!("{base}.mmd")), s)?;
                    }
                    println!("{}: {base}.html", t.nome());
                    items.push((t, Ok(format!("{base}.html"))));
                }
                Err(e) => {
                    eprintln!("aviso: {}: {e}", t.nome());
                    items.push((t, Err(e)));
                }
            }
        }
        std::fs::write(index, utsim_diagram::indice(model, &items))?;
        println!("gravado em {}", index.display());
        return Ok(());
    }
    let tipo = TipoDiagrama::from_id(kind).ok_or_else(|| {
        let ids: Vec<&str> = TipoDiagrama::ALL.iter().map(|t| t.id()).collect();
        format!(
            "tipo '{kind}' desconhecido (use {} ou todos)",
            ids.join(", ")
        )
    })?;
    let out = utsim_diagram::gerar(tipo, model, options)?;
    let mut wrote = false;
    let mut put =
        |what: &str, path: &Option<PathBuf>, text: Option<&String>| -> std::io::Result<()> {
            if let Some(p) = path {
                match text {
                    Some(t) => {
                        std::fs::write(p, t)?;
                        println!("gravado em {}", p.display());
                        wrote = true;
                    }
                    None => eprintln!("aviso: o tipo {} não tem saída {what}", tipo.id()),
                }
            }
            Ok(())
        };
    put("SVG", &args.svg, out.svg.as_ref())?;
    put("HTML", &args.html, Some(&out.html))?;
    put("Mermaid", &args.mermaid, out.mermaid.as_ref())?;
    put("CSV", &args.csv, out.csv.as_ref())?;
    if !wrote {
        match out.mermaid {
            Some(m) => print!("{m}"),
            None => print!("{}", out.html),
        }
    }
    Ok(())
}
