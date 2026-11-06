//! Risk zones of the model (`Model.zones`, `docs/formato_qualidade_seguranca.md`) as the 3D
//! editor changes them: each change is one command of the document (one undo step). The
//! polygons are in the model plan (metres, `y` forward); the bridge converts from Godot.

use utsim_edit::Command;
use utsim_model::Model;
use utsim_model::quality::{PPE_NAMES, ZoneDef, ZoneKind};

/// The kinds of zone in panel order: the RON name and the Portuguese label.
pub const KINDS: [(ZoneKind, &str, &str); 6] = [
    (ZoneKind::Vehicle, "Vehicle", "Tráfego de veículos"),
    (ZoneKind::Machine, "Machine", "Alcance de máquina"),
    (ZoneKind::SuspendedLoad, "SuspendedLoad", "Carga suspensa"),
    (ZoneKind::Noise, "Noise", "Ruído"),
    (ZoneKind::Heat, "Heat", "Calor"),
    (ZoneKind::Other, "Other", "Outro"),
];

pub fn kind_key(k: ZoneKind) -> &'static str {
    KINDS.iter().find(|x| x.0 == k).map_or("Other", |x| x.1)
}

pub fn kind_of(key: &str) -> Option<ZoneKind> {
    KINDS.iter().find(|x| x.1 == key).map(|x| x.0)
}

/// The protective equipment the panel offers: the known names, then any other the model
/// already uses.
pub fn ppe_choices(model: &Model) -> Vec<String> {
    let mut out: Vec<String> = PPE_NAMES.iter().map(|s| s.to_string()).collect();
    for z in &model.zones {
        for p in &z.ppe {
            if !out.contains(p) {
                out.push(p.clone());
            }
        }
    }
    out
}

fn zone<'a>(model: &'a Model, name: &str) -> Result<&'a ZoneDef, String> {
    model
        .zones
        .iter()
        .find(|z| z.name == name)
        .ok_or_else(|| format!("a zona '{name}' não existe"))
}

/// A new zone along `polygon`: the first free name `zona_N`, traffic of vehicles, level 3.
pub fn add(model: &Model, polygon: Vec<[f64; 2]>) -> Result<(String, Command), String> {
    if polygon.len() < 3 {
        return Err("uma zona precisa de pelo menos 3 pontos".into());
    }
    let name = (1..)
        .map(|k| format!("zona_{k}"))
        .find(|n| !model.zones.iter().any(|z| &z.name == n))
        .expect("a free name");
    let z = ZoneDef {
        name: name.clone(),
        kind: ZoneKind::Vehicle,
        polygon,
        level: Some(3),
        ppe: Vec::new(),
    };
    Ok((name, Command::AddZone(z)))
}

/// Zone `name` with a new polygon.
pub fn set_polygon(model: &Model, name: &str, polygon: Vec<[f64; 2]>) -> Result<Command, String> {
    let mut z = zone(model, name)?.clone();
    z.polygon = polygon;
    Ok(Command::SetZone {
        name: name.to_owned(),
        zone: z,
    })
}

/// Zone `name` with one field changed: `name`, `kind` (RON name), `level` (1 to 5; 0 = none)
/// or `ppe` (`item` toggled).
pub fn set_field(model: &Model, name: &str, field: &str, value: &str) -> Result<Command, String> {
    let mut z = zone(model, name)?.clone();
    match field {
        "name" => {
            let v = value.trim();
            if v.is_empty() {
                return Err("a zona precisa de um nome".into());
            }
            z.name = v.to_owned();
        }
        "kind" => {
            z.kind = kind_of(value).ok_or_else(|| format!("tipo de zona desconhecido: {value}"))?
        }
        "level" => {
            let l: f64 = value
                .trim()
                .replace(',', ".")
                .parse()
                .map_err(|_| "nível: um número de 1 a 5")?;
            z.level = (l >= 1.0).then(|| l.round().clamp(1.0, 5.0) as u8);
        }
        "ppe" => {
            let v = value.trim().to_owned();
            if let Some(k) = z.ppe.iter().position(|p| *p == v) {
                z.ppe.remove(k);
            } else if !v.is_empty() {
                z.ppe.push(v);
            }
        }
        _ => return Err(format!("campo de zona desconhecido: {field}")),
    }
    Ok(Command::SetZone {
        name: name.to_owned(),
        zone: z,
    })
}

pub fn remove(model: &Model, name: &str) -> Result<Command, String> {
    zone(model, name)?;
    Ok(Command::RemoveZone {
        name: name.to_owned(),
    })
}

/// The colour of a zone by its level (1 lowest), as `[r, g, b]` in 0..1: green to red.
pub fn level_color(level: Option<u8>) -> [f32; 3] {
    match level.unwrap_or(0) {
        1 => [0.30, 0.69, 0.31],
        2 => [0.80, 0.86, 0.22],
        3 => [1.00, 0.76, 0.03],
        4 => [1.00, 0.44, 0.26],
        5 => [0.90, 0.22, 0.21],
        _ => [0.62, 0.66, 0.72],
    }
}

/// Whether `p` is inside the polygon (even-odd rule), both on the same plane axes.
pub fn contains(polygon: &[[f64; 2]], p: [f64; 2]) -> bool {
    let n = polygon.len();
    let mut inside = false;
    let mut j = n.wrapping_sub(1);
    for i in 0..n {
        let (a, b) = (polygon[i], polygon[j]);
        if (a[1] > p[1]) != (b[1] > p[1])
            && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;
    use utsim_edit::Document;

    #[test]
    fn a_zone_is_added_reshaped_retyped_and_removed_one_step_each() {
        let mut d = Document::new("z");
        let square = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]];
        assert!(add(d.model(), square[..2].to_vec()).is_err());
        let (name, c) = add(d.model(), square.clone()).unwrap();
        assert_eq!(name, "zona_1");
        d.apply(c).unwrap();
        let (name2, c) = add(d.model(), square.clone()).unwrap();
        assert_eq!(name2, "zona_2");
        d.apply(c).unwrap();
        let mut moved = square.clone();
        moved[2] = [5.0, 4.0];
        d.apply(set_polygon(d.model(), "zona_1", moved.clone()).unwrap())
            .unwrap();
        assert_eq!(d.model().zones[0].polygon, moved);
        for (f, v) in [
            ("kind", "Noise"),
            ("level", "5"),
            ("ppe", "protetor_auricular"),
            ("ppe", "capacete"),
            ("name", "prensa"),
        ] {
            let name = d.model().zones[0].name.clone();
            d.apply(set_field(d.model(), &name, f, v).unwrap()).unwrap();
        }
        let z = &d.model().zones[0];
        assert_eq!(
            (z.name.as_str(), z.kind, z.level),
            ("prensa", ZoneKind::Noise, Some(5))
        );
        assert_eq!(
            z.ppe,
            vec!["protetor_auricular".to_owned(), "capacete".to_owned()]
        );
        // Toggling an item takes it out; level 0 clears the level.
        d.apply(set_field(d.model(), "prensa", "ppe", "capacete").unwrap())
            .unwrap();
        d.apply(set_field(d.model(), "prensa", "level", "0").unwrap())
            .unwrap();
        assert_eq!(d.model().zones[0].ppe.len(), 1);
        assert_eq!(d.model().zones[0].level, None);
        assert!(set_field(d.model(), "prensa", "kind", "Lava").is_err());
        assert!(
            d.apply(set_field(d.model(), "prensa", "name", "zona_2").unwrap())
                .is_err()
        );
        d.apply(remove(d.model(), "zona_2").unwrap()).unwrap();
        assert_eq!(d.model().zones.len(), 1);
        assert!(d.undo());
        assert_eq!(d.model().zones.len(), 2);
        assert!(ppe_choices(d.model()).contains(&"capacete".to_owned()));
    }

    #[test]
    fn inside_and_outside_a_polygon_and_the_colours_by_level() {
        let l = [[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [2.0, 1.0], [0.0, 4.0]];
        assert!(contains(&l, [1.0, 0.5]));
        assert!(!contains(&l, [2.0, 3.0]), "the notch");
        assert!(!contains(&l, [5.0, 1.0]));
        assert_ne!(level_color(Some(1)), level_color(Some(5)));
        assert_eq!(kind_of(kind_key(ZoneKind::Heat)), Some(ZoneKind::Heat));
    }
}
