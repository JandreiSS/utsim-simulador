//! `utsim-assets`: writes the procedural `.glb` models of the appearance library.
//!
//! ```text
//! utsim-assets [pasta]                      # every default model (default: assets/modelos)
//! utsim-assets --listar                     # names, nodes, triangles and sizes
//! utsim-assets --porta-paletes 6,4,1.2,1.5 [pasta]   # baias, níveis, largura, altura
//! utsim-assets --esteira 8 [pasta]          # comprimento
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

use utsim_assets::{ConveyorSpec, RackSpec, Spec};

const USAGE: &str = "uso: utsim-assets [pasta] | --listar | --porta-paletes BAIAS,NIVEIS,LARGURA,ALTURA [pasta] | --esteira COMPRIMENTO [pasta]";

fn numbers(text: &str) -> Result<Vec<f32>, String> {
    text.split(',')
        .map(|s| {
            s.trim()
                .parse::<f32>()
                .ok()
                .filter(|v| v.is_finite() && *v > 0.0)
                .ok_or_else(|| format!("número inválido: \"{s}\""))
        })
        .collect()
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut specs = Spec::defaults();
    let mut dir = PathBuf::from("assets/modelos");
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" | "--ajuda" => {
                println!("{USAGE}");
                return Ok(());
            }
            "--listar" => {
                for s in Spec::defaults() {
                    let m = s.model();
                    let (min, max) = m.bounds();
                    let nodes: Vec<&str> =
                        m.nodes.iter().skip(1).map(|n| n.name.as_str()).collect();
                    println!(
                        "{:<17} {:>5} triângulos  {:.2} x {:.2} x {:.2} m  nós: {}",
                        s.name(),
                        m.triangles(),
                        max[0] - min[0],
                        max[1] - min[1],
                        max[2] - min[2],
                        nodes.join(", ")
                    );
                }
                return Ok(());
            }
            "--porta-paletes" => {
                let v = numbers(&args.next().ok_or(USAGE)?)?;
                let [bays, levels, bay_width, level_height] = v[..] else {
                    return Err("--porta-paletes espera BAIAS,NIVEIS,LARGURA,ALTURA".into());
                };
                specs = vec![Spec::PortaPaletes(RackSpec {
                    bays: bays as usize,
                    levels: levels as usize,
                    bay_width,
                    level_height,
                    ..RackSpec::default()
                })];
            }
            "--esteira" => {
                let v = numbers(&args.next().ok_or(USAGE)?)?;
                let [length] = v[..] else {
                    return Err("--esteira espera COMPRIMENTO".into());
                };
                specs = vec![Spec::Esteira(ConveyorSpec {
                    length,
                    ..ConveyorSpec::default()
                })];
            }
            other if other.starts_with('-') => {
                return Err(format!("opção desconhecida: {other}\n{USAGE}"));
            }
            other => dir = PathBuf::from(other),
        }
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for s in specs {
        let path = dir.join(format!("{}.glb", s.key()));
        let bytes = s.glb();
        std::fs::write(&path, &bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        println!("{} ({} bytes)", path.display(), bytes.len());
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("erro: {e}");
            ExitCode::FAILURE
        }
    }
}
