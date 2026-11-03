//! `utsim pert`: PERT/CPM of a part of the model.

use std::path::PathBuf;

use utsim_model::Model;
use utsim_pert::monte_carlo::{self, McOptions};
use utsim_pert::observed::{self, ObservedOptions};
use utsim_pert::pert::{self, Moments};
use utsim_pert::report::{self, Analysis};
use utsim_pert::{DeriveOptions, Mode, Selection, derive};

pub struct Args {
    pub model: PathBuf,
    pub objects: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub deadline: Option<f64>,
    pub mode: String,
    pub moments: String,
    pub samples: usize,
    pub seed: Option<u64>,
    pub reps: Option<u32>,
    pub csv: Option<PathBuf>,
    pub html: Option<PathBuf>,
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let selection = match (&args.objects, &args.from, &args.to) {
        (Some(list), None, None) => Selection::Objects(
            list.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        ),
        (None, Some(a), Some(b)) => Selection::Between {
            from: a.clone(),
            to: b.clone(),
        },
        _ => return Err("informe --objetos a,b,c ou --de A --ate B".into()),
    };
    let mode = Mode::parse(&args.mode).ok_or_else(|| {
        format!(
            "--modo '{}' desconhecido: use simulado, classico ou montecarlo",
            args.mode
        )
    })?;
    let moments = match args.moments.as_str() {
        "exatos" | "exato" => Moments::Exact,
        "classicos" | "classico" | "beta" => Moments::Classic,
        other => {
            return Err(
                format!("--momentos '{other}' desconhecido: use exatos ou classicos").into(),
            );
        }
    };
    let model = Model::load(&args.model)?;
    model.validate()?;
    for warning in model.warnings() {
        eprintln!("aviso: {warning}");
    }
    let net = derive(&model, &selection, &DeriveOptions::default())?;
    let classic = pert::analyze(&net, moments, args.deadline);
    let mc = (mode == Mode::MonteCarlo).then(|| {
        monte_carlo::run(
            &net,
            &McOptions {
                samples: args.samples,
                seed: args.seed.unwrap_or(model.run.seed),
                deadline: args.deadline,
                ..McOptions::default()
            },
        )
    });
    let obs = match mode {
        Mode::Simulated => {
            let mut m = model.clone();
            if let Some(s) = args.seed {
                m.run.seed = s;
            }
            Some(observed::run(
                &m,
                &net,
                &ObservedOptions {
                    replications: args.reps,
                    deadline: args.deadline,
                    ..ObservedOptions::default()
                },
            )?)
        }
        _ => None,
    };
    let analysis = Analysis {
        title: &model.name,
        selection: selection.describe(),
        mode,
        network: &net,
        classic: &classic,
        monte_carlo: mc.as_ref(),
        observed: obs.as_ref(),
    };
    print!("{}", report::to_text(&analysis));
    if let Some(dir) = &args.csv {
        let files = report::write_csv(&analysis, dir)?;
        println!("\nCSV gravado em {}: {}", dir.display(), files.join(", "));
    }
    if let Some(path) = &args.html {
        std::fs::write(path, report::to_html(&analysis))?;
        println!("relatório HTML gravado em {}", path.display());
    }
    Ok(())
}
