//! The rows of [`crate::aisle_edit`] as Godot values, shared by `UtsimDocument` (Editar)
//! and `UtsimPlayback` (the scene, in both modes). Points on the floor go from model plan
//! coordinates `(x, y)` to Godot `(x, 0, -y)`.

use godot::prelude::*;
use utsim_model::Model;

use crate::aisle_edit;
use crate::put;

/// A floor point of the model plan as a Godot point (on the floor).
pub fn point(p: [f64; 2]) -> Vector3 {
    Vector3::new(p[0] as f32, 0.0, -(p[1] as f32))
}

/// A Godot point as a point of the model plan.
pub fn plan(v: Vector3) -> [f64; 2] {
    [f64::from(v.x), -f64::from(v.z)]
}

fn polygon(points: &[[f64; 2]]) -> PackedVector3Array {
    points.iter().map(|p| point(*p)).collect()
}

/// `#RRGGBB` (or `#RGB`) as a colour; white when it does not read.
fn colour(text: &str) -> Color {
    let hex = text.trim_start_matches('#');
    let value = |s: &str| u8::from_str_radix(s, 16).ok();
    let (r, g, b) = match hex.len() {
        6 => (value(&hex[0..2]), value(&hex[2..4]), value(&hex[4..6])),
        3 => (
            value(&hex[0..1]).map(|v| v * 17),
            value(&hex[1..2]).map(|v| v * 17),
            value(&hex[2..3]).map(|v| v * 17),
        ),
        _ => (None, None, None),
    };
    match (r, g, b) {
        (Some(r), Some(g), Some(b)) => Color::from_rgb(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
        ),
        _ => Color::WHITE,
    }
}

/// The polygons of the floor marking: `{source, pattern ("continuous" | "zebra" |
/// "arrow"), color, polygon}`.
pub fn marks(model: Option<&Model>) -> VarArray {
    let mut out = VarArray::new();
    for m in model.map(aisle_edit::marks).unwrap_or_default() {
        let mut d = VarDictionary::new();
        put(&mut d, "source", GString::from(m.source.as_str()));
        put(&mut d, "pattern", GString::from(m.pattern));
        put(&mut d, "color", colour(&m.color));
        put(&mut d, "polygon", polygon(&m.polygon));
        out.push(&d.to_variant());
    }
    out
}

/// `{name, position, radius}` of each pedestrian x vehicle crossing.
pub fn crossings(model: Option<&Model>) -> VarArray {
    let mut out = VarArray::new();
    for c in model.map(aisle_edit::crossings).unwrap_or_default() {
        let mut d = VarDictionary::new();
        put(&mut d, "name", GString::from(c.name.as_str()));
        put(&mut d, "position", point(c.position));
        put(&mut d, "radius", c.radius);
        out.push(&d.to_variant());
    }
    out
}

/// `{name, kind, kind_label, points, model_points, width, two_way, secondary, profile,
/// length}`.
pub fn aisles(model: &Model) -> VarArray {
    let mut out = VarArray::new();
    for a in aisle_edit::aisles(model) {
        let mut d = VarDictionary::new();
        put(&mut d, "name", GString::from(a.name.as_str()));
        put(&mut d, "kind", GString::from(a.kind));
        put(&mut d, "kind_label", GString::from(a.kind_label));
        put(&mut d, "points", polygon(&a.points));
        let model_points: PackedVector2Array = a
            .points
            .iter()
            .map(|p| Vector2::new(p[0] as f32, p[1] as f32))
            .collect();
        put(&mut d, "model_points", model_points);
        put(&mut d, "width", a.width);
        put(&mut d, "two_way", a.two_way);
        put(&mut d, "secondary", a.secondary);
        put(&mut d, "profile", GString::from(a.profile));
        put(&mut d, "length", a.length);
        out.push(&d.to_variant());
    }
    out
}

/// `{name, kind ("objeto" | "parede"), polygon, clearance}`.
pub fn obstacles(model: &Model) -> VarArray {
    let mut out = VarArray::new();
    for o in aisle_edit::obstacles(model) {
        let mut d = VarDictionary::new();
        put(&mut d, "name", GString::from(o.name.as_str()));
        put(&mut d, "kind", GString::from(o.kind));
        put(&mut d, "polygon", polygon(&o.polygon));
        put(&mut d, "clearance", o.clearance);
        out.push(&d.to_variant());
    }
    out
}

/// `{object, point, center, custom}`.
pub fn access_points(model: &Model) -> VarArray {
    let mut out = VarArray::new();
    for a in aisle_edit::access_points(model) {
        let mut d = VarDictionary::new();
        put(&mut d, "object", GString::from(a.object.as_str()));
        put(&mut d, "point", point(a.point));
        put(&mut d, "center", point(a.center));
        put(&mut d, "custom", a.custom);
        out.push(&d.to_variant());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colours_read_in_long_and_short_form() {
        assert_eq!(colour("#FF0000"), Color::from_rgb(1.0, 0.0, 0.0));
        assert_eq!(colour("#0f0"), Color::from_rgb(0.0, 1.0, 0.0));
        assert_eq!(colour("amarelo"), Color::WHITE);
    }

    #[test]
    fn plan_and_godot_points_are_inverse() {
        let p = [3.5, -2.0];
        assert_eq!(plan(point(p)), p);
        let v = point(p);
        assert_eq!((v.x, v.y, v.z), (3.5, 0.0, 2.0));
    }
}
