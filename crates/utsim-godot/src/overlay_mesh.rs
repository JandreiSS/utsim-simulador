//! What the 3D editor draws over the scene, as triangles with colours (step 7, part 2, of
//! `docs/plano_interface_rust.md`): the selection corners, the problem and critical marks,
//! the connection arrows, the nodes, the × of the selected link, the preview arrow and the
//! risk zones. The script copies the arrays into an `ArrayMesh` and places the labels; the
//! geometry is tested here.
//!
//! Colours are sRGB `[r, g, b, a]` (the material reads vertex colours as sRGB).

use crate::editor3d::{Editor3d, Mark, PreviewKind, Side, V3};

pub type Rgba = [f32; 4];

pub const LINK: Rgba = [0.62, 0.72, 0.88, 1.0];
pub const LINK_LOGICAL: Rgba = [0.50, 0.55, 0.62, 1.0];
pub const LINK_SELECTED: Rgba = [1.0, 0.85, 0.30, 1.0];
pub const CONNECTING: Rgba = [0.45, 0.95, 0.55, 1.0];
pub const PORT_OUT: Rgba = [0.45, 0.95, 0.55, 1.0];
pub const PORT_IN: Rgba = [0.55, 0.80, 1.0, 1.0];
pub const PORT_SERVICE: Rgba = [1.0, 0.58, 0.12, 1.0];
pub const PORT_HOVER: Rgba = [1.0, 1.0, 1.0, 1.0];
pub const PORT_RING: Rgba = [0.06, 0.07, 0.09, 0.9];
pub const REMOVE: Rgba = [0.92, 0.26, 0.22, 1.0];
pub const PRIMARY: Rgba = [1.0, 1.0, 1.0, 1.0];
pub const OTHERS: Rgba = [0.35, 0.65, 1.0, 1.0];
pub const PROBLEM: Rgba = [1.0, 0.35, 0.30, 1.0];
pub const CRITICAL: Rgba = [1.0, 0.30, 0.22, 1.0];

/// Triangles: three vertices each, one colour per vertex.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tris {
    pub verts: Vec<V3>,
    pub colors: Vec<Rgba>,
}

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn mul(a: V3, k: f64) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}
fn len(a: V3) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

impl Tris {
    pub fn tri(&mut self, a: V3, b: V3, c: V3, color: Rgba) {
        self.verts.extend([a, b, c]);
        self.colors.extend([color; 3]);
    }

    /// A flat band from `a` to `b` on the floor plane.
    pub fn ribbon(&mut self, a: V3, b: V3, width: f64, color: Rgba) {
        let mut d = sub(b, a);
        d[1] = 0.0;
        let l = len(d);
        if l < 1e-5 {
            return;
        }
        let side = mul([-d[2] / l, 0.0, d[0] / l], width * 0.5);
        self.tri(sub(a, side), add(a, side), add(b, side), color);
        self.tri(sub(a, side), add(b, side), sub(b, side), color);
    }

    /// An arrow from `a` to `b`: a band and a head.
    pub fn arrow(&mut self, a: V3, b: V3, color: Rgba, width: f64) {
        let d = sub(b, a);
        let l = len(d);
        if l < 1e-3 {
            return;
        }
        let d = mul(d, 1.0 / l);
        let head = 0.45f64.min(l * 0.4);
        let base = sub(b, mul(d, head));
        self.ribbon(a, base, width, color);
        let side = mul([-d[2], 0.0, d[0]], head * 0.45);
        self.tri(sub(base, side), add(base, side), b, color);
    }

    /// A filled disc on the floor plane.
    pub fn disc(&mut self, c: V3, r: f64, color: Rgba) {
        let n = 16;
        for k in 0..n {
            let a0 = std::f64::consts::TAU * k as f64 / n as f64;
            let a1 = std::f64::consts::TAU * (k + 1) as f64 / n as f64;
            self.tri(
                c,
                add(c, [a0.cos() * r, 0.0, a0.sin() * r]),
                add(c, [a1.cos() * r, 0.0, a1.sin() * r]),
                color,
            );
        }
    }

    /// The outline of a box's footprint.
    pub fn rect(&mut self, lo: V3, hi: V3, color: Rgba, width: f64) {
        let y = 0.03;
        let p = [
            [lo[0], y, lo[2]],
            [hi[0], y, lo[2]],
            [hi[0], y, hi[2]],
            [lo[0], y, hi[2]],
        ];
        for k in 0..4 {
            let (a, b) = (p[k], p[(k + 1) % 4]);
            let d = mul(sub(b, a), width * 0.5 / len(sub(b, a)).max(1e-9));
            self.ribbon(sub(a, d), add(b, d), width, color);
        }
    }

    /// L marks at the four corners of a footprint.
    pub fn corners(&mut self, lo: V3, hi: V3, color: Rgba, width: f64) {
        let y = 0.03;
        let arm = ((hi[0] - lo[0]).min(hi[2] - lo[2]) * 0.3).clamp(0.25, 0.8);
        for (x, z, sx, sz) in [
            (lo[0], lo[2], 1.0, 1.0),
            (hi[0], lo[2], -1.0, 1.0),
            (hi[0], hi[2], -1.0, -1.0),
            (lo[0], hi[2], 1.0, -1.0),
        ] {
            let p = [x, y, z];
            self.ribbon(p, add(p, [sx * arm, 0.0, 0.0]), width, color);
            self.ribbon(p, add(p, [0.0, 0.0, sz * arm]), width, color);
        }
    }

    /// A node: a disc with a dark rim and a stub to the block's side (`edge_x`); bigger
    /// and white-rimmed when lit.
    pub fn node(&mut self, at: V3, color: Rgba, lit: bool, edge_x: f64) {
        let mut stub = color;
        stub[3] = 0.8;
        self.ribbon([edge_x, at[1], at[2]], at, 0.04, stub);
        let r = if lit { 0.17 } else { 0.12 };
        self.disc(at, r + 0.035, if lit { PORT_HOVER } else { PORT_RING });
        self.disc(at, r, if lit { lighten(color, 0.25) } else { color });
    }

    /// A polygon on the floor at height `y`, triangulated (concave allowed).
    pub fn polygon(&mut self, pts: &[V3], y: f64, color: Rgba) {
        let flat: Vec<[f64; 2]> = pts.iter().map(|p| [p[0], p[2]]).collect();
        for [a, b, c] in triangulate(&flat) {
            let at = |k: usize| [pts[k][0], y, pts[k][2]];
            self.tri(at(a), at(b), at(c), color);
        }
    }
}

fn lighten(c: Rgba, k: f32) -> Rgba {
    [
        c[0] + (1.0 - c[0]) * k,
        c[1] + (1.0 - c[1]) * k,
        c[2] + (1.0 - c[2]) * k,
        c[3],
    ]
}

/// Ear clipping of a simple polygon (either winding): triangles as vertex indices.
pub fn triangulate(p: &[[f64; 2]]) -> Vec<[usize; 3]> {
    let n = p.len();
    if n < 3 {
        return Vec::new();
    }
    let area: f64 = (0..n)
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % n]);
            a[0] * b[1] - b[0] * a[1]
        })
        .sum();
    let ccw = area > 0.0;
    let cross = |a: [f64; 2], b: [f64; 2], c: [f64; 2]| {
        (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    };
    let inside = |q: [f64; 2], a: [f64; 2], b: [f64; 2], c: [f64; 2]| {
        let (d1, d2, d3) = (cross(a, b, q), cross(b, c, q), cross(c, a, q));
        let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
        let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
        !(neg && pos)
    };
    let mut idx: Vec<usize> = (0..n).collect();
    let mut out = Vec::new();
    let mut guard = 0;
    while idx.len() > 3 && guard < n * n {
        guard += 1;
        let m = idx.len();
        let mut cut = false;
        for k in 0..m {
            let (ia, ib, ic) = (idx[(k + m - 1) % m], idx[k], idx[(k + 1) % m]);
            let (a, b, c) = (p[ia], p[ib], p[ic]);
            let turn = cross(a, b, c);
            if (ccw && turn <= 0.0) || (!ccw && turn >= 0.0) {
                continue;
            }
            if idx
                .iter()
                .filter(|&&j| j != ia && j != ib && j != ic)
                .any(|&j| inside(p[j], a, b, c))
            {
                continue;
            }
            out.push([ia, ib, ic]);
            idx.remove(k);
            cut = true;
            break;
        }
        if !cut {
            break;
        }
    }
    if idx.len() == 3 {
        out.push([idx[0], idx[1], idx[2]]);
    }
    out
}

/// A label of the overlay: where, what and in which colour.
#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    pub at: V3,
    pub text: String,
    pub color: Rgba,
}

/// Everything the editor draws: opaque marks (`lines`, drawn over everything), the
/// translucent zone fill (`fill`), the labels and where the × of the selected link goes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Drawing {
    pub lines: Tris,
    pub fill: Tris,
    pub port_labels: Vec<Label>,
    pub zone_labels: Vec<Label>,
    pub remove: Option<V3>,
}

/// The editor's drawing. `critical`: the objects on the PERT/CPM critical path.
pub fn draw(e: &Editor3d, critical: &[String]) -> Drawing {
    let o = e.overlay();
    let mut d = Drawing::default();
    let t = &mut d.lines;
    let grow =
        |lo: V3, hi: V3, g: f64| ([lo[0] - g, lo[1], lo[2] - g], [hi[0] + g, hi[1], hi[2] + g]);
    for (lo, hi, m) in &o.corners {
        match m {
            Mark::Primary | Mark::Other => {
                let (a, b) = grow(*lo, *hi, 0.1);
                t.corners(
                    a,
                    b,
                    if *m == Mark::Primary { PRIMARY } else { OTHERS },
                    0.09,
                );
            }
            Mark::Problem => {
                let (a, b) = grow(*lo, *hi, 0.04);
                t.corners(a, b, PROBLEM, 0.04);
            }
        }
    }
    for b in e.blocks.iter().filter(|b| critical.contains(&b.name)) {
        let (lo, hi) = b.pick_box();
        let (a, c) = grow(lo, hi, 0.3);
        t.rect(a, c, CRITICAL, 0.1);
    }
    // Connections; the port number on the lines of an object with several outputs.
    let segs = e.segments();
    for s in &segs {
        let selected = e.selected_link.as_ref() == Some(&(s.from.clone(), s.to.clone()));
        let color = if selected {
            LINK_SELECTED
        } else if s.logical {
            LINK_LOGICAL
        } else {
            LINK
        };
        t.arrow(s.a, s.b, color, if selected { 0.09 } else { 0.06 });
        if segs.iter().filter(|x| x.from == s.from).count() > 1 {
            let at = add(add(s.a, mul(sub(s.b, s.a), 0.25)), [0.0, 0.25, 0.0]);
            d.port_labels.push(Label {
                at,
                text: s.port.to_string(),
                color,
            });
        }
    }
    for n in &o.nodes {
        let color = match n.side {
            Side::Out => PORT_OUT,
            Side::In => PORT_IN,
            Side::Service => PORT_SERVICE,
        };
        t.node(n.at, color, n.lit, n.edge);
    }
    if let Some(at) = o.remove {
        if e.selected_team.is_none()
            && let Some((a, b)) = &e.selected_link
            && let Some(s) = segs.iter().find(|s| &s.from == a && &s.to == b)
        {
            let m = add(s.a, mul(sub(s.b, s.a), 0.5));
            if len(sub(m, at)) > 0.05 {
                let mut c = REMOVE;
                c[3] = 0.8;
                t.ribbon(m, at, 0.03, c);
            }
        }
        let lit = e.camera.near(at, e.mouse, crate::editor3d::PORT_PIXELS);
        t.disc(at, if lit { 0.2 } else { 0.17 }, PORT_HOVER);
        t.disc(at, if lit { 0.17 } else { 0.14 }, REMOVE);
        d.remove = Some(add(at, [0.0, 0.02, 0.0]));
    }
    if let Some((a, b, PreviewKind::Flow)) = o.preview {
        t.arrow(a, b, CONNECTING, 0.07);
    }
    // Risk zones.
    for (z, sel) in &o.zones {
        if z.points.len() < 3 {
            continue;
        }
        let c = crate::zone_edit::level_color(z.level);
        let col = [c[0], c[1], c[2], 1.0];
        d.fill.polygon(
            &z.points,
            0.02,
            [c[0], c[1], c[2], if *sel { 0.34 } else { 0.22 }],
        );
        let n = z.points.len();
        for k in 0..n {
            let a = add(z.points[k], [0.0, 0.035, 0.0]);
            let b = add(z.points[(k + 1) % n], [0.0, 0.035, 0.0]);
            d.lines.ribbon(a, b, if *sel { 0.1 } else { 0.06 }, col);
        }
        if *sel && e.tool == "zona" {
            for v in &z.points {
                d.lines.disc(add(*v, [0.0, 0.04, 0.0]), 0.14, PORT_HOVER);
                d.lines.disc(add(*v, [0.0, 0.045, 0.0]), 0.1, col);
            }
        }
        let centre = mul(
            z.points.iter().fold([0.0; 3], |a, v| add(a, *v)),
            1.0 / n as f64,
        );
        d.zone_labels.push(Label {
            at: add(centre, [0.0, 0.3, 0.0]),
            text: z.name.clone(),
            color: lighten(col, 0.3),
        });
    }
    let dr = &o.zone_drawing;
    for k in 0..dr.len().saturating_sub(1) {
        d.lines.ribbon(
            add(dr[k], [0.0, 0.04, 0.0]),
            add(dr[k + 1], [0.0, 0.04, 0.0]),
            0.07,
            CONNECTING,
        );
        d.lines
            .disc(add(dr[k], [0.0, 0.045, 0.0]), 0.12, CONNECTING);
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor3d::{Block, Camera, Conn, ZoneView};

    fn area(p: &[[f64; 2]], tris: &[[usize; 3]]) -> f64 {
        tris.iter()
            .map(|[a, b, c]| {
                let (a, b, c) = (p[*a], p[*b], p[*c]);
                ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs() / 2.0
            })
            .sum()
    }

    #[test]
    fn polygons_are_triangulated_whole_convex_or_not_and_either_way_round() {
        let square = [[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]];
        let t = triangulate(&square);
        assert_eq!(t.len(), 2);
        assert!((area(&square, &t) - 12.0).abs() < 1e-9);
        let mut back = square;
        back.reverse();
        assert!((area(&back, &triangulate(&back)) - 12.0).abs() < 1e-9);
        // An L (concave): area 3 x 1 + 1 x 2 = 5, in 4 triangles.
        let l = [
            [0.0, 0.0],
            [3.0, 0.0],
            [3.0, 1.0],
            [1.0, 1.0],
            [1.0, 3.0],
            [0.0, 3.0],
        ];
        let t = triangulate(&l);
        assert_eq!(t.len(), 4);
        assert!((area(&l, &t) - 5.0).abs() < 1e-9);
        assert!(triangulate(&l[..2]).is_empty());
    }

    #[test]
    fn primitives_make_whole_triangles_with_one_colour_each() {
        let mut t = Tris::default();
        t.ribbon([0.0; 3], [1.0, 0.0, 0.0], 0.1, LINK);
        t.arrow([0.0; 3], [2.0, 0.0, 0.0], LINK, 0.06);
        t.disc([0.0; 3], 0.2, REMOVE);
        t.corners([0.0; 3], [2.0, 1.0, 2.0], PRIMARY, 0.05);
        t.node([1.0, 0.06, 0.0], PORT_OUT, true, 0.5);
        assert!(t.verts.len().is_multiple_of(3));
        assert_eq!(t.verts.len(), t.colors.len());
        // A zero-length ribbon draws nothing.
        let mut z = Tris::default();
        z.ribbon([1.0; 3], [1.0; 3], 0.1, LINK);
        assert!(z.verts.is_empty());
    }

    #[test]
    fn the_drawing_has_the_nodes_the_arrows_the_selection_and_the_zones() {
        let mut e = Editor3d::new();
        e.camera = Camera::look_at([6.0, 16.0, 10.0], [6.0, 0.0, 0.0], 50.0, [1600.0, 900.0]);
        let b = |n: &str, k: &str, x: f64| Block {
            name: n.into(),
            kind: k.into(),
            pos: [x, 0.0, 0.0],
            size: [1.0; 3],
            skeletal: false,
            problems: n == "saida",
        };
        e.set_scene(
            vec![
                b("a", "Source", 0.0),
                b("p", "Processor", 4.0),
                b("q", "Queue", 4.0),
                b("saida", "Sink", 8.0),
            ],
            vec![
                Conn {
                    from: "a".into(),
                    to: "p".into(),
                    port: 1,
                    logical: false,
                },
                Conn {
                    from: "a".into(),
                    to: "q".into(),
                    port: 2,
                    logical: false,
                },
            ],
            vec![],
        );
        e.selected = vec!["p".into()];
        e.selected_link = Some(("a".into(), "p".into()));
        e.set_zones(vec![ZoneView {
            name: "z".into(),
            points: vec![[0.0, 0.0, 2.0], [3.0, 0.0, 2.0], [3.0, 0.0, 4.0]],
            level: Some(5),
        }]);
        let d = draw(&e, &["q".to_owned()]);
        assert!(!d.lines.verts.is_empty() && d.lines.verts.len().is_multiple_of(3));
        assert_eq!(d.lines.verts.len(), d.lines.colors.len());
        // The selected link is yellow; the × is there; two port labels (a has two outputs).
        assert!(d.lines.colors.contains(&LINK_SELECTED));
        assert!(d.remove.is_some());
        assert_eq!(d.port_labels.len(), 2);
        // The selection is white, the problem red, the critical object outlined.
        assert!(d.lines.colors.contains(&PRIMARY));
        assert!(d.lines.colors.contains(&PROBLEM));
        assert!(d.lines.colors.contains(&CRITICAL));
        // The zone: a translucent triangle and its label.
        assert_eq!(d.fill.verts.len(), 3);
        assert!(d.fill.colors.iter().all(|c| c[3] < 1.0));
        assert_eq!(d.zone_labels[0].text, "z");
    }
}
