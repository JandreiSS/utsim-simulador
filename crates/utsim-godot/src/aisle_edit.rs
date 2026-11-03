//! Circulation aisles, obstacles and access points in the Godot interface, with no Godot
//! types: the rows the scene and the overlay draw (floor marks, aisles, crossings,
//! obstacles, access points) and the edits the "Corredor" tool makes, each one a
//! `utsim_edit::Command` (or a `Batch`, one undo step).
//!
//! Everything here is in model plan coordinates (metres, Z up); the class converts to
//! Godot. The shapes of the floor marking come from `Model::floor_marks()`, so the scene
//! draws exactly what `utsim corredores --svg` draws.

use std::collections::BTreeMap;

use utsim_edit::Command;
use utsim_model::{Aisle, AisleKind, Model, Pattern, Profile};

use crate::document::{Editor, snap_to};

/// Width a new aisle of each kind starts with (m): the minimums of the Brasil profile for
/// a standard forklift (`L` = 1,2 m), two-way.
pub fn default_width(kind: AisleKind) -> f64 {
    match kind {
        AisleKind::Pedestrian => 1.2,
        AisleKind::Forklift => 3.6,
        AisleKind::Mixed => 4.8,
    }
}

pub fn kind_name(kind: AisleKind) -> &'static str {
    match kind {
        AisleKind::Pedestrian => "pedestrian",
        AisleKind::Forklift => "forklift",
        AisleKind::Mixed => "mixed",
    }
}

pub fn kind_from(name: &str) -> Result<AisleKind, String> {
    match name {
        "pedestrian" => Ok(AisleKind::Pedestrian),
        "forklift" => Ok(AisleKind::Forklift),
        "mixed" => Ok(AisleKind::Mixed),
        _ => Err(format!("tipo de corredor desconhecido: {name}")),
    }
}

/// Name of the profile of an aisle (`""`: the model's).
fn profile_name(p: &Option<Profile>) -> &'static str {
    match p {
        None => "",
        Some(Profile::Brasil) => "Brasil",
        Some(Profile::Branco) => "Branco",
        Some(Profile::Custom(_)) => "Custom",
    }
}

/// One aisle as the panel and the overlay show it.
#[derive(Clone, Debug, PartialEq)]
pub struct AisleRow {
    pub name: String,
    pub kind: &'static str,
    pub kind_label: &'static str,
    pub points: Vec<[f64; 2]>,
    pub width: f64,
    pub two_way: bool,
    pub secondary: bool,
    /// `""` (the model's), `Brasil`, `Branco` or `Custom`.
    pub profile: &'static str,
    pub length: f64,
}

pub fn aisles(model: &Model) -> Vec<AisleRow> {
    model
        .aisles
        .iter()
        .map(|a| AisleRow {
            name: a.name.clone(),
            kind: kind_name(a.kind),
            kind_label: a.kind.label(),
            points: a.points.clone(),
            width: a.width,
            two_way: a.two_way,
            secondary: a.secondary,
            profile: profile_name(&a.profile),
            length: a.length(),
        })
        .collect()
}

/// One polygon of the floor marking.
#[derive(Clone, Debug, PartialEq)]
pub struct MarkRow {
    /// The aisle (or crossing) it belongs to.
    pub source: String,
    /// `continuous`, `zebra` or `arrow`.
    pub pattern: &'static str,
    /// `#RRGGBB`.
    pub color: String,
    pub polygon: Vec<[f64; 2]>,
}

pub fn marks(model: &Model) -> Vec<MarkRow> {
    if model.aisles.is_empty() {
        return Vec::new();
    }
    model
        .floor_marks()
        .into_iter()
        .map(|m| MarkRow {
            source: m.source,
            pattern: match m.pattern {
                Pattern::Continuous => "continuous",
                Pattern::Zebra => "zebra",
                Pattern::Arrow => "arrow",
            },
            color: m.color,
            polygon: m.polygon,
        })
        .collect()
}

/// A crossing between a pedestrian and a vehicle aisle.
#[derive(Clone, Debug, PartialEq)]
pub struct CrossingRow {
    pub name: String,
    pub position: [f64; 2],
    /// Half the length of the conflict zone along the path (m).
    pub radius: f64,
}

pub fn crossings(model: &Model) -> Vec<CrossingRow> {
    if model.aisles.is_empty() {
        return Vec::new();
    }
    model
        .aisle_net()
        .crossings
        .iter()
        .map(|c| CrossingRow {
            name: c.name.clone(),
            position: c.position,
            radius: c.radius,
        })
        .collect()
}

/// One obstacle: the area of an object or a wall/column of the model.
#[derive(Clone, Debug, PartialEq)]
pub struct ObstacleRow {
    pub name: String,
    /// `objeto` (the area of a fixed object) or `parede` (`Model::obstacles`).
    pub kind: &'static str,
    pub polygon: Vec<[f64; 2]>,
    /// Clearance the resources that serve it keep from it (m).
    pub clearance: f64,
}

/// The clearance each fixed object is served with: the largest among the resources that
/// serve it, else the largest of all resources, else a person's.
fn clearances(model: &Model) -> (BTreeMap<String, f64>, f64) {
    let mut by_object: BTreeMap<String, f64> = BTreeMap::new();
    let mut widest: Option<f64> = None;
    for t in model
        .objects
        .iter()
        .filter(|o| utsim_model::is_mobile(&o.kind))
    {
        let c = model.clearance_of(t);
        widest = Some(widest.map_or(c, |w| w.max(c)));
        for served in model.served_by(&t.name) {
            let e = by_object.entry(served.name.clone()).or_insert(c);
            *e = e.max(c);
        }
    }
    (by_object, widest.unwrap_or(0.6))
}

pub fn obstacles(model: &Model) -> Vec<ObstacleRow> {
    let (by_object, widest) = clearances(model);
    let mut out: Vec<ObstacleRow> = model
        .nav_bodies()
        .iter()
        .map(|b| ObstacleRow {
            name: b.name.clone(),
            kind: "objeto",
            polygon: b.polygon(),
            clearance: by_object.get(&b.name).copied().unwrap_or(widest),
        })
        .collect();
    out.extend(model.obstacles.iter().map(|o| ObstacleRow {
        name: o.name.clone(),
        kind: "parede",
        polygon: o.polygon.clone(),
        clearance: widest,
    }));
    out
}

/// The access point of a fixed object.
#[derive(Clone, Debug, PartialEq)]
pub struct AccessRow {
    pub object: String,
    /// On the floor, model plan coordinates.
    pub point: [f64; 2],
    /// Set in the model (`access`), not the default on the `-y` side.
    pub custom: bool,
    /// Where the object is, to draw the line from it to its access point.
    pub center: [f64; 2],
}

pub fn access_points(model: &Model) -> Vec<AccessRow> {
    let (by_object, widest) = clearances(model);
    model
        .nav_bodies()
        .iter()
        .map(|b| AccessRow {
            object: b.name.clone(),
            point: b.access_point(by_object.get(&b.name).copied().unwrap_or(widest)),
            custom: b.access.is_some(),
            center: b.center,
        })
        .collect()
}

fn dist2(a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)
}

/// Snap target for a point of an aisle: the nearest vertex of another aisle (or of the
/// same one, except `skip`) within `radius`, else the nearest point of a segment of any
/// aisle within `radius`. `None`: nothing near (the caller snaps to the grid).
pub fn snap_to_aisles(
    model: &Model,
    p: [f64; 2],
    radius: f64,
    skip: Option<(&str, usize)>,
) -> Option<[f64; 2]> {
    let mut best: Option<(f64, [f64; 2])> = None;
    for a in &model.aisles {
        for (i, v) in a.points.iter().enumerate() {
            if skip == Some((a.name.as_str(), i)) {
                continue;
            }
            let d = dist2(*v, p);
            if d <= radius * radius && best.is_none_or(|(b, _)| d < b) {
                best = Some((d, *v));
            }
        }
    }
    if best.is_some() {
        return best.map(|(_, v)| v);
    }
    for a in &model.aisles {
        for (i, w) in a.points.windows(2).enumerate() {
            // The segments touching the point being moved would pull it onto itself.
            if skip.is_some_and(|(n, k)| n == a.name && (i == k || i + 1 == k)) {
                continue;
            }
            let d = [w[1][0] - w[0][0], w[1][1] - w[0][1]];
            let len2 = d[0] * d[0] + d[1] * d[1];
            if len2 <= 0.0 {
                continue;
            }
            let t = (((p[0] - w[0][0]) * d[0] + (p[1] - w[0][1]) * d[1]) / len2).clamp(0.0, 1.0);
            let q = [w[0][0] + d[0] * t, w[0][1] + d[1] * t];
            let dd = dist2(q, p);
            if dd <= radius * radius && best.is_none_or(|(b, _)| dd < b) {
                best = Some((dd, q));
            }
        }
    }
    best.map(|(_, q)| q)
}

/// The aisle (and index of its nearest vertex) within `radius` of `p`.
pub fn vertex_near(model: &Model, p: [f64; 2], radius: f64) -> Option<(String, usize)> {
    let mut best: Option<(f64, String, usize)> = None;
    for a in &model.aisles {
        for (i, v) in a.points.iter().enumerate() {
            let d = dist2(*v, p);
            if d <= radius * radius && best.as_ref().is_none_or(|(b, _, _)| d < *b) {
                best = Some((d, a.name.clone(), i));
            }
        }
    }
    best.map(|(_, n, i)| (n, i))
}

fn rounded(p: [f64; 2]) -> [f64; 2] {
    // Millimetres: the saved text stays short and the snapped values stay exact.
    [
        (p[0] * 1000.0).round() / 1000.0 + 0.0,
        (p[1] * 1000.0).round() / 1000.0 + 0.0,
    ]
}

impl Editor {
    fn aisle(&self, name: &str) -> Result<Aisle, String> {
        self.model()
            .aisles
            .iter()
            .find(|a| a.name == name)
            .cloned()
            .ok_or_else(|| format!("corredor \"{name}\" não existe"))
    }

    /// Adds an aisle along `points` (two at least; consecutive duplicates are dropped).
    /// Width `0` takes the default of the kind. Returns its name.
    pub fn add_aisle(
        &mut self,
        points: &[[f64; 2]],
        kind: &str,
        width: f64,
        two_way: bool,
    ) -> Result<String, String> {
        let kind = kind_from(kind)?;
        let mut line: Vec<[f64; 2]> = Vec::new();
        for p in points {
            let p = rounded(*p);
            if line.last() != Some(&p) {
                line.push(p);
            }
        }
        if line.len() < 2 {
            return Err("o corredor precisa de ao menos 2 pontos diferentes".into());
        }
        let name = self.doc.unique_aisle_name();
        self.apply(Command::AddAisle(Box::new(Aisle {
            name: name.clone(),
            kind,
            points: line,
            width: if width > 0.0 {
                width
            } else {
                default_width(kind)
            },
            two_way,
            secondary: false,
            profile: None,
        })))?;
        Ok(name)
    }

    fn replace_aisle(&mut self, name: &str, edit: impl FnOnce(&mut Aisle)) -> Result<(), String> {
        let mut aisle = self.aisle(name)?;
        edit(&mut aisle);
        self.apply(Command::SetAisle {
            name: name.to_string(),
            aisle: Box::new(aisle),
        })
    }

    /// Moves vertex `index` of the aisle to `p`.
    pub fn move_aisle_point(
        &mut self,
        name: &str,
        index: usize,
        p: [f64; 2],
    ) -> Result<(), String> {
        if index >= self.aisle(name)?.points.len() {
            return Err("vértice inexistente".into());
        }
        self.replace_aisle(name, |a| a.points[index] = rounded(p))
    }

    /// Adds a vertex after `index` (a bend in the middle of the aisle).
    pub fn insert_aisle_point(
        &mut self,
        name: &str,
        index: usize,
        p: [f64; 2],
    ) -> Result<(), String> {
        if index >= self.aisle(name)?.points.len() {
            return Err("vértice inexistente".into());
        }
        self.replace_aisle(name, |a| a.points.insert(index + 1, rounded(p)))
    }

    /// Removes a vertex; an aisle keeps at least two.
    pub fn remove_aisle_point(&mut self, name: &str, index: usize) -> Result<(), String> {
        let a = self.aisle(name)?;
        if a.points.len() <= 2 {
            return Err("o corredor precisa de ao menos 2 pontos: exclua o corredor".into());
        }
        if index >= a.points.len() {
            return Err("vértice inexistente".into());
        }
        self.replace_aisle(name, |a| {
            a.points.remove(index);
        })
    }

    pub fn remove_aisle(&mut self, name: &str) -> Result<(), String> {
        self.apply(Command::RemoveAisle {
            name: name.to_string(),
        })
    }

    pub fn rename_aisle(&mut self, from: &str, to: &str) -> Result<(), String> {
        let to = to.trim().to_string();
        if to.is_empty() {
            return Err("o nome do corredor não pode ficar vazio".into());
        }
        self.replace_aisle(from, |a| a.name = to)
    }

    pub fn set_aisle_kind(&mut self, name: &str, kind: &str) -> Result<(), String> {
        let kind = kind_from(kind)?;
        self.replace_aisle(name, |a| a.kind = kind)
    }

    pub fn set_aisle_width(&mut self, name: &str, width: f64) -> Result<(), String> {
        self.replace_aisle(name, |a| a.width = width)
    }

    pub fn set_aisle_two_way(&mut self, name: &str, two_way: bool) -> Result<(), String> {
        self.replace_aisle(name, |a| a.two_way = two_way)
    }

    pub fn set_aisle_secondary(&mut self, name: &str, secondary: bool) -> Result<(), String> {
        self.replace_aisle(name, |a| a.secondary = secondary)
    }

    /// `""`: the profile of the model; `Brasil` or `Branco`.
    pub fn set_aisle_profile(&mut self, name: &str, profile: &str) -> Result<(), String> {
        let profile = match profile {
            "" => None,
            "Brasil" => Some(Profile::Brasil),
            "Branco" => Some(Profile::Branco),
            _ => return Err(format!("perfil desconhecido: {profile}")),
        };
        self.replace_aisle(name, |a| a.profile = profile)
    }

    /// Turns a one-way aisle around.
    pub fn reverse_aisle(&mut self, name: &str) -> Result<(), String> {
        self.replace_aisle(name, |a| a.points.reverse())
    }

    /// The access point of `object` as the floor point `p` (snapped to `step`, 0: no
    /// grid), stored in the object's own coordinates. The point must be a distance from
    /// the object's area: that is for the warnings to say.
    pub fn set_access_at(&mut self, object: &str, p: [f64; 2], step: f64) -> Result<(), String> {
        let body = self
            .model()
            .nav_bodies()
            .into_iter()
            .find(|b| b.name == object)
            .ok_or_else(|| {
                format!("\"{object}\" não tem área ocupada (só objetos fixos têm ponto de acesso)")
            })?;
        let (dx, dy) = (p[0] - body.center[0], p[1] - body.center[1]);
        let (s, c) = (-body.yaw).sin_cos();
        let local = [dx * c - dy * s, dx * s + dy * c];
        let local = [snap_to(local[0], step), snap_to(local[1], step)];
        self.apply(Command::SetAccess {
            name: object.to_string(),
            access: Some(rounded(local)),
        })
    }

    /// Back to the default access point (the `-y` side).
    pub fn reset_access(&mut self, object: &str) -> Result<(), String> {
        self.apply(Command::SetAccess {
            name: object.to_string(),
            access: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utsim_edit::from_palette;

    fn crossing_editor() -> Editor {
        let mut e = Editor::new("cruz");
        e.add_aisle(&[[-10.0, 0.0], [10.0, 0.0]], "forklift", 0.0, true)
            .unwrap();
        e.add_aisle(&[[0.0, -10.0], [0.0, 10.0]], "pedestrian", 0.0, true)
            .unwrap();
        e
    }

    #[test]
    fn aisles_are_drawn_edited_and_undone_with_default_widths() {
        let mut e = crossing_editor();
        let rows = aisles(e.model());
        assert_eq!(rows.len(), 2);
        assert_eq!(
            (rows[0].name.as_str(), rows[0].kind, rows[0].width),
            ("corredor_1", "forklift", 3.6)
        );
        assert_eq!((rows[1].kind, rows[1].width), ("pedestrian", 1.2));
        assert!((rows[0].length - 20.0).abs() < 1e-9);
        e.set_aisle_two_way("corredor_1", false).unwrap();
        e.set_aisle_width("corredor_1", 2.4).unwrap();
        e.set_aisle_kind("corredor_1", "mixed").unwrap();
        e.set_aisle_secondary("corredor_2", true).unwrap();
        e.set_aisle_profile("corredor_2", "Branco").unwrap();
        let a = &aisles(e.model())[0];
        assert!(!a.two_way && a.width == 2.4 && a.kind == "mixed");
        assert_eq!(aisles(e.model())[1].profile, "Branco");
        e.move_aisle_point("corredor_1", 1, [12.34567, 1.0])
            .unwrap();
        assert_eq!(e.model().aisles[0].points[1], [12.346, 1.0]);
        e.insert_aisle_point("corredor_1", 0, [0.0, 4.0]).unwrap();
        assert_eq!(e.model().aisles[0].points.len(), 3);
        e.remove_aisle_point("corredor_1", 1).unwrap();
        assert!(e.remove_aisle_point("corredor_2", 0).is_err(), "keeps two");
        e.reverse_aisle("corredor_1").unwrap();
        assert_eq!(e.model().aisles[0].points[0], [12.346, 1.0]);
        e.rename_aisle("corredor_2", "passeio").unwrap();
        assert!(e.rename_aisle("passeio", "corredor_1").is_err());
        assert!(e.rename_aisle("passeio", " ").is_err());
        e.remove_aisle("passeio").unwrap();
        assert_eq!(aisles(e.model()).len(), 1);
        // Undo walks every step back to the empty model.
        let mut steps = 0;
        while e.doc.undo() {
            steps += 1;
        }
        assert_eq!(steps, 13);
        assert!(e.model().aisles.is_empty());
        assert!(
            e.add_aisle(&[[1.0, 1.0], [1.0, 1.0]], "forklift", 0.0, true)
                .is_err()
        );
        assert!(
            e.add_aisle(&[[0.0, 0.0], [1.0, 0.0]], "tanque", 0.0, true)
                .is_err()
        );
    }

    #[test]
    fn a_crossing_gets_a_zebra_and_the_marks_are_coloured_by_the_profile() {
        let mut e = crossing_editor();
        let rows = marks(e.model());
        let count = |p: &str| rows.iter().filter(|m| m.pattern == p).count();
        assert!(count("continuous") >= 4, "two lines per aisle");
        assert!(count("zebra") >= 4, "stripes on the crossing");
        assert!(
            rows.iter()
                .all(|m| m.polygon.len() >= 3 && m.color.starts_with('#'))
        );
        let yellow = rows
            .iter()
            .find(|m| m.pattern == "continuous")
            .unwrap()
            .color
            .to_uppercase();
        assert_eq!(yellow, "#F2C200");
        let c = crossings(e.model());
        assert_eq!(c.len(), 1);
        assert!(c[0].position[0].abs() < 1e-6 && c[0].position[1].abs() < 1e-6);
        // The White profile changes the colour of the lines.
        e.apply(Command::SetFloorMarking(Some(Box::new(
            utsim_model::FloorMarking {
                profile: Profile::Branco,
                ..utsim_model::FloorMarking::default()
            },
        ))))
        .unwrap();
        let white = marks(e.model())
            .into_iter()
            .find(|m| m.pattern == "continuous")
            .unwrap()
            .color;
        assert_ne!(white.to_uppercase(), yellow);
        assert!(Editor::new("vazio").model().aisles.is_empty());
        assert!(marks(Editor::new("vazio").model()).is_empty());
    }

    #[test]
    fn snapping_prefers_vertices_then_segments_and_skips_the_moving_point() {
        let e = crossing_editor();
        let m = e.model();
        assert_eq!(snap_to_aisles(m, [9.8, 0.2], 0.5, None), Some([10.0, 0.0]));
        let on_line = snap_to_aisles(m, [4.0, 0.3], 0.5, None).unwrap();
        assert!((on_line[0] - 4.0).abs() < 1e-9 && on_line[1].abs() < 1e-9);
        assert_eq!(snap_to_aisles(m, [4.0, 3.0], 0.5, None), None);
        // The vertex being moved does not snap to itself.
        assert_eq!(
            snap_to_aisles(m, [9.8, 0.2], 0.5, Some(("corredor_1", 1))),
            None
        );
        assert_eq!(
            vertex_near(m, [0.1, 9.9], 0.5),
            Some(("corredor_2".to_string(), 1))
        );
        assert_eq!(vertex_near(m, [5.0, 5.0], 0.5), None);
    }

    #[test]
    fn obstacles_and_access_points_follow_the_objects() {
        let mut e = Editor::new("acesso");
        let fila = from_palette("Queue", "fila", [10.0, 0.0, 0.0]).unwrap();
        e.apply(Command::Add(Box::new(fila))).unwrap();
        let emp = from_palette("Transporter", "emp", [0.0, 5.0, 0.0]).unwrap();
        e.apply(Command::Add(Box::new(emp))).unwrap();
        let rows = obstacles(e.model());
        assert_eq!(rows.len(), 1, "a mobile resource is not an obstacle");
        assert_eq!((rows[0].name.as_str(), rows[0].kind), ("fila", "objeto"));
        assert_eq!(rows[0].polygon.len(), 4);
        let a = &access_points(e.model())[0];
        assert!(!a.custom);
        assert!(
            (a.point[0] - 10.0).abs() < 1e-9 && a.point[1] < 0.0,
            "-y side"
        );
        // Dragging the point to the right side stores it in the object's coordinates.
        e.set_access_at("fila", [13.0, 0.04], 0.05).unwrap();
        let a = &access_points(e.model())[0];
        assert!(a.custom);
        assert_eq!(e.model().objects[0].access, Some([3.0, 0.05]));
        assert!(e.set_access_at("emp", [0.0, 0.0], 0.0).is_err());
        e.reset_access("fila").unwrap();
        assert_eq!(e.model().objects[0].access, None);
        e.doc.undo();
        assert_eq!(e.model().objects[0].access, Some([3.0, 0.05]));
        // A wall of the model is an obstacle too.
        let mut e = Editor::new("parede");
        e.apply(Command::Add(Box::new(
            from_palette("Queue", "q", [0.0; 3]).unwrap(),
        )))
        .unwrap();
        assert_eq!(obstacles(e.model()).len(), 1);
    }

    #[test]
    fn aisles_survive_saving_and_reopening() {
        let mut e = crossing_editor();
        e.set_aisle_two_way("corredor_1", false).unwrap();
        let text = e.doc.to_text();
        assert!(text.contains("aisles:"), "{text}");
        let model = Model::parse(&text).unwrap();
        assert_eq!(model.aisles.len(), 2);
        assert!(!model.aisles[0].two_way);
    }
}
