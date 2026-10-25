use std::path::Path;

use utsim_model::synthetic::{Shape, generate};

/// `utsim gerar-modelo`: writes a synthetic model of any size (benchmarks, scale tests).
pub fn run(
    shape: &str,
    objects: usize,
    items: u64,
    out: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let shape = Shape::parse(shape)
        .ok_or_else(|| format!("tipo '{shape}' desconhecido: use linha, rede ou armazem"))?;
    let model = generate(shape, objects, items)?;
    model.validate()?;
    let config = ron::ser::PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(false)
        .enumerate_arrays(false);
    let body = ron::ser::to_string_pretty(&model, config)?;
    match out {
        Some(path) => {
            std::fs::write(path, body)?;
            eprintln!(
                "{}: {} objetos, {} conexões, {items} itens gravados em {}",
                model.name,
                model.objects.len(),
                model.connections.len(),
                path.display()
            );
        }
        None => println!("{body}"),
    }
    Ok(())
}
