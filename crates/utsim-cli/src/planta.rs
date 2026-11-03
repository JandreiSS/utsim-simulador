use std::path::Path;

use utsim_cad::{
    Calibration, ReadOptions, RoleMap, SvgOptions, Transform, Units, build_obstacles, read_dxf,
    to_svg,
};

fn numeros(s: &str, n: usize, nome: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let v: Vec<f64> = s
        .split(',')
        .map(|x| x.trim().parse::<f64>())
        .collect::<Result<_, _>>()
        .map_err(|_| format!("{nome}: números separados por vírgula, com ponto decimal"))?;
    if v.len() != n {
        return Err(format!("{nome}: esperava {n} números, recebi {}", v.len()).into());
    }
    Ok(v)
}

/// `utsim planta`: lê um DXF 2D, lista camadas, calibra e grava um SVG.
pub fn run(
    file: &Path,
    camadas: bool,
    svg: Option<&Path>,
    calibrar: Option<&str>,
    origem: Option<&str>,
    unidades: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read(file)?;
    let text = String::from_utf8_lossy(&bytes);
    let mut opts = ReadOptions::default();
    if let Some(u) = unidades {
        opts.units = Some(match u.to_lowercase().as_str() {
            "mm" => Units::Millimeters,
            "cm" => Units::Centimeters,
            "dm" => Units::Decimeters,
            "m" => Units::Meters,
            "km" => Units::Kilometers,
            "pol" | "in" => Units::Inches,
            "pe" | "ft" => Units::Feet,
            _ => {
                return Err(format!(
                    "unidade '{u}' desconhecida: use mm, cm, dm, m, km, pol ou pe"
                )
                .into());
            }
        });
    }
    // com calibração, as coordenadas são lidas sem conversão (são as do desenho)
    if calibrar.is_some() {
        opts.units = Some(Units::Drawing);
    }
    let mut plan = read_dxf(&text, &opts)?;
    println!("{}", file.display());
    println!("hash: {}", utsim_cad::fnv1a_hash(&bytes));
    if let Some(c) = calibrar {
        let v = numeros(c, 5, "--calibrar x1,y1,x2,y2,distancia")?;
        let cal = Calibration::new([v[0], v[1]], [v[2], v[3]], v[4])?;
        plan = plan.transformed(&cal.transform());
        println!(
            "calibração: escala {} m por unidade do desenho",
            cal.scale()
        );
    } else if plan.units_assumed {
        println!(
            "unidade: o arquivo não informa; assumido milímetros (use --unidades ou --calibrar)"
        );
    } else {
        println!("unidade: {} m por unidade do desenho", plan.meters_per_unit);
    }
    if let Some(o) = origem {
        let v = numeros(o, 2, "--origem x,y")?;
        plan = plan.transformed(&Transform::origin_at([v[0], v[1]]));
    }
    match plan.bbox() {
        Some(b) => println!(
            "caixa: ({:.3}, {:.3}) a ({:.3}, {:.3}) m, {:.3} x {:.3} m",
            b.min[0],
            b.min[1],
            b.max[0],
            b.max[1],
            b.width(),
            b.height()
        ),
        None => println!("caixa: planta vazia"),
    }
    let roles = RoleMap::suggest(&plan);
    println!(
        "{} camadas, {} segmentos",
        plan.layers.len(),
        plan.segment_count()
    );
    if camadas {
        println!(
            "{:<28} {:>8} {:>9} {:>7}  papel sugerido",
            "camada", "entid.", "segm.", "rót."
        );
        for s in plan.summaries() {
            println!(
                "{:<28} {:>8} {:>9} {:>7}  {}",
                s.name,
                s.entities,
                s.segments,
                s.labels,
                roles.role_of(&s.name).name()
            );
        }
    }
    let obs = build_obstacles(&plan, &roles, 0.15);
    println!(
        "obstáculos (paredes e colunas sugeridas): {} polígonos, {} aberturas",
        obs.polygons.len(),
        obs.openings.len()
    );
    if let Some(path) = svg {
        let o = SvgOptions {
            roles: Some(roles),
            ..SvgOptions::default()
        };
        std::fs::write(path, to_svg(&plan, &o))?;
        println!("SVG gravado em {}", path.display());
    }
    Ok(())
}
