//! `utsim corredores modelo.ron [--svg arquivo]`: resumo dos corredores, da rede gerada e
//! dos cruzamentos, avisos e a planta em SVG.

use std::path::Path;

use utsim_model::Model;

pub fn run(path: &Path, svg: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    let m = Model::load(path)?;
    m.validate()?;
    if m.aisles.is_empty() {
        println!("{}: o modelo não tem corredores (`aisles`)", m.name);
    } else {
        for line in m.aisle_lines() {
            println!("{line}");
        }
    }
    for line in m.movement_lines() {
        println!("movimentação: {line}");
    }
    for w in m.warnings() {
        eprintln!("aviso: {w}");
    }
    if let Some(out) = svg {
        std::fs::write(out, m.aisles_svg())?;
        println!("planta gravada em {}", out.display());
    }
    Ok(())
}
