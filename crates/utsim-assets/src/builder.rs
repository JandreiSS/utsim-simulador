//! Scene graph and mesh building in *asset coordinates* (metres, Z up, `+x` forward,
//! `+y` to the left, origin on the floor at the centre of the footprint).
//!
//! Every shape is given in asset coordinates; the writer ([`crate::glb`]) makes vertices
//! relative to their node's pivot and node translations relative to the parent pivot,
//! and converts to glTF's Y-up frame. So an author only places pivots at the joints and
//! draws the pieces where they are.

/// A point or direction in asset coordinates.
pub type V3 = [f32; 3];

/// Flat-colour PBR material. Colours are sRGB (as picked); the writer stores them linear.
#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub name: String,
    /// sRGB colour and alpha (alpha < 1 is written with `alphaMode: BLEND`).
    pub color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    /// sRGB emissive colour (black for none).
    pub emissive: [f32; 3],
}

/// Triangles of one material in one node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Geometry {
    pub positions: Vec<V3>,
    pub normals: Vec<V3>,
    pub indices: Vec<u32>,
}

impl Geometry {
    fn vertex(&mut self, p: V3, n: V3) -> u32 {
        self.positions.push(p);
        self.normals.push(n);
        (self.positions.len() - 1) as u32
    }

    fn tri(&mut self, a: u32, b: u32, c: u32) {
        self.indices.extend([a, b, c]);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Part {
    pub material: usize,
    pub geometry: Geometry,
}

/// A named node. `pivot` is the joint, in asset coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub parent: Option<usize>,
    pub pivot: V3,
    pub parts: Vec<Part>,
}

/// A whole asset: node 0 is the root, named after the asset, with its pivot at the origin.
#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub name: String,
    pub materials: Vec<Material>,
    pub nodes: Vec<Node>,
}

/// Rounds to a grid of 2^-14 m (0.06 mm), so the written bytes do not depend on the last
/// bit of the platform's `sin`/`cos`/`powf` (the committed files must match everywhere).
pub fn quantize(x: f32) -> f32 {
    let q = (x * 16384.0).round() / 16384.0;
    if q == 0.0 { 0.0 } else { q }
}

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn mul(a: V3, k: f32) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn norm(a: V3) -> V3 {
    let l = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    if l < 1e-12 {
        [0.0, 0.0, 1.0]
    } else {
        mul(a, 1.0 / l)
    }
}

/// Two unit vectors `v`, `w` with `v x w = u` (a right-handed frame around axis `u`).
fn frame(u: V3) -> (V3, V3) {
    let reference = if u[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let v = norm(cross(reference, u));
    let w = cross(u, v);
    (v, w)
}

/// `#rrggbb` as sRGB components.
pub fn hex(rgb: u32) -> [f32; 3] {
    [
        ((rgb >> 16) & 0xff) as f32 / 255.0,
        ((rgb >> 8) & 0xff) as f32 / 255.0,
        (rgb & 0xff) as f32 / 255.0,
    ]
}

impl Model {
    pub fn new(name: &str) -> Self {
        Model {
            name: name.to_string(),
            materials: Vec::new(),
            nodes: vec![Node {
                name: name.to_string(),
                parent: None,
                pivot: [0.0; 3],
                parts: Vec::new(),
            }],
        }
    }

    /// Opaque material from an sRGB hex colour.
    pub fn material(&mut self, name: &str, rgb: u32, metallic: f32, roughness: f32) -> usize {
        let c = hex(rgb);
        self.push_material(Material {
            name: name.to_string(),
            color: [c[0], c[1], c[2], 1.0],
            metallic,
            roughness,
            emissive: [0.0; 3],
        })
    }

    /// Glowing material (lamps, status lights).
    pub fn emissive(&mut self, name: &str, rgb: u32) -> usize {
        let c = hex(rgb);
        self.push_material(Material {
            name: name.to_string(),
            color: [c[0], c[1], c[2], 1.0],
            metallic: 0.0,
            roughness: 0.4,
            emissive: c,
        })
    }

    /// See-through material (glass, tank shells).
    pub fn translucent(&mut self, name: &str, rgb: u32, alpha: f32) -> usize {
        let c = hex(rgb);
        self.push_material(Material {
            name: name.to_string(),
            color: [c[0], c[1], c[2], alpha],
            metallic: 0.1,
            roughness: 0.15,
            emissive: [0.0; 3],
        })
    }

    fn push_material(&mut self, m: Material) -> usize {
        self.materials.push(m);
        self.materials.len() - 1
    }

    /// A child node of `parent` with its joint at `pivot` (asset coordinates).
    pub fn node(&mut self, name: &str, parent: usize, pivot: V3) -> usize {
        self.nodes.push(Node {
            name: name.to_string(),
            parent: Some(parent),
            pivot,
            parts: Vec::new(),
        });
        self.nodes.len() - 1
    }

    /// Index of the node called `name`.
    pub fn find(&self, name: &str) -> Option<usize> {
        self.nodes.iter().position(|n| n.name == name)
    }

    fn geom(&mut self, node: usize, material: usize) -> &mut Geometry {
        let parts = &mut self.nodes[node].parts;
        let k = match parts.iter().position(|p| p.material == material) {
            Some(k) => k,
            None => {
                parts.push(Part {
                    material,
                    geometry: Geometry::default(),
                });
                parts.len() - 1
            }
        };
        &mut parts[k].geometry
    }

    /// Axis-aligned box between two corners.
    pub fn cuboid(&mut self, node: usize, mat: usize, min: V3, max: V3) {
        let center = mul(add(min, max), 0.5);
        let half = mul(sub(max, min), 0.5);
        self.obox(
            node,
            mat,
            center,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            half,
        );
    }

    /// Oriented box: `axes` is a right-handed orthonormal basis, `half` the half sizes
    /// along it. Flat shaded (24 vertices, 12 triangles).
    pub fn obox(&mut self, node: usize, mat: usize, center: V3, axes: [V3; 3], half: V3) {
        let g = self.geom(node, mat);
        for k in 0..3 {
            let (i, j) = ((k + 1) % 3, (k + 2) % 3);
            for s in [1.0f32, -1.0] {
                let n = mul(axes[k], s);
                let c = add(center, mul(axes[k], s * half[k]));
                let corner = |a: f32, b: f32| {
                    add(c, add(mul(axes[i], a * half[i]), mul(axes[j], b * half[j])))
                };
                let quad = [
                    corner(-1.0, -1.0),
                    corner(1.0, -1.0),
                    corner(1.0, 1.0),
                    corner(-1.0, 1.0),
                ];
                let ids: Vec<u32> = quad.iter().map(|&p| g.vertex(p, n)).collect();
                if s > 0.0 {
                    g.tri(ids[0], ids[1], ids[2]);
                    g.tri(ids[0], ids[2], ids[3]);
                } else {
                    g.tri(ids[0], ids[2], ids[1]);
                    g.tri(ids[0], ids[3], ids[2]);
                }
            }
        }
    }

    /// Box from `a` to `b` with a `width` x `height` cross-section (`height` is measured
    /// in the vertical plane through the bar when possible).
    pub fn bar(&mut self, node: usize, mat: usize, a: V3, b: V3, width: f32, height: f32) {
        let d = sub(b, a);
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let u = norm(d);
        let (v, w) = frame(u);
        self.obox(
            node,
            mat,
            mul(add(a, b), 0.5),
            [u, v, w],
            [len / 2.0, width / 2.0, height / 2.0],
        );
    }

    /// Truncated cone from `a` (radius `ra`) to `b` (radius `rb`), with caps. Smooth
    /// sides, flat caps.
    #[allow(clippy::too_many_arguments)] // node, material and the shape's own parameters
    pub fn frustum(&mut self, node: usize, mat: usize, a: V3, b: V3, ra: f32, rb: f32, seg: u32) {
        let u = norm(sub(b, a));
        let (v, w) = frame(u);
        let len = {
            let d = sub(b, a);
            (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
        };
        // Side normals lean by the cone slope.
        let slope = (ra - rb) / len.max(1e-6);
        let g = self.geom(node, mat);
        let dir = |k: u32| {
            let t = std::f32::consts::TAU * k as f32 / seg as f32;
            add(mul(v, t.cos()), mul(w, t.sin()))
        };
        let base = g.positions.len() as u32;
        for k in 0..=seg {
            let r = dir(k);
            let n = norm(add(r, mul(u, slope)));
            g.vertex(add(a, mul(r, ra)), n);
            g.vertex(add(b, mul(r, rb)), n);
        }
        for k in 0..seg {
            let (a0, b0, a1, b1) = (
                base + 2 * k,
                base + 2 * k + 1,
                base + 2 * k + 2,
                base + 2 * k + 3,
            );
            g.tri(a0, a1, b1);
            g.tri(a0, b1, b0);
        }
        for (center, radius, sign) in [(b, rb, 1.0f32), (a, ra, -1.0)] {
            if radius <= 0.0 {
                continue;
            }
            let n = mul(u, sign);
            let c = g.vertex(center, n);
            let ring = g.positions.len() as u32;
            for k in 0..seg {
                g.vertex(add(center, mul(dir(k), radius)), n);
            }
            for k in 0..seg {
                let (p, q) = (ring + k, ring + (k + 1) % seg);
                if sign > 0.0 {
                    g.tri(c, p, q);
                } else {
                    g.tri(c, q, p);
                }
            }
        }
    }

    /// Cylinder from `a` to `b`.
    pub fn cylinder(&mut self, node: usize, mat: usize, a: V3, b: V3, r: f32, seg: u32) {
        self.frustum(node, mat, a, b, r, r, seg);
    }

    /// Sphere (z is the polar axis), smooth shaded. `top_only` draws the upper half
    /// with a flat bottom (a dome or a helmet).
    #[allow(clippy::too_many_arguments)] // node, material and the shape's own parameters
    pub fn sphere(
        &mut self,
        node: usize,
        mat: usize,
        center: V3,
        r: f32,
        seg: u32,
        rings: u32,
        top_only: bool,
    ) {
        let g = self.geom(node, mat);
        let lat0 = if top_only {
            0.0
        } else {
            -std::f32::consts::FRAC_PI_2
        };
        let lat_step = (std::f32::consts::FRAC_PI_2 - lat0) / rings as f32;
        let base = g.positions.len() as u32;
        for i in 0..=rings {
            let lat = lat0 + lat_step * i as f32;
            for j in 0..=seg {
                let lon = std::f32::consts::TAU * j as f32 / seg as f32;
                let n = [lat.cos() * lon.cos(), lat.cos() * lon.sin(), lat.sin()];
                g.vertex(add(center, mul(n, r)), n);
            }
        }
        let row = seg + 1;
        for i in 0..rings {
            for j in 0..seg {
                let p00 = base + i * row + j;
                let p01 = p00 + 1;
                let p10 = p00 + row;
                let p11 = p10 + 1;
                let south_pole = !top_only && i == 0;
                let north_pole = i + 1 == rings;
                if !south_pole {
                    g.tri(p00, p01, p11);
                }
                if !north_pole {
                    g.tri(p00, p11, p10);
                }
            }
        }
        if top_only {
            let n = [0.0, 0.0, -1.0];
            let c = g.vertex(center, n);
            let ring = g.positions.len() as u32;
            for j in 0..seg {
                let lon = std::f32::consts::TAU * j as f32 / seg as f32;
                g.vertex(add(center, [r * lon.cos(), r * lon.sin(), 0.0]), n);
            }
            for j in 0..seg {
                g.tri(c, ring + (j + 1) % seg, ring + j);
            }
        }
    }

    /// World-space pivot of a node (asset coordinates).
    pub fn pivot(&self, node: usize) -> V3 {
        self.nodes[node].pivot
    }

    /// Axis-aligned bounds of all geometry, asset coordinates, in the rest pose.
    pub fn bounds(&self) -> (V3, V3) {
        let (mut min, mut max) = ([f32::MAX; 3], [f32::MIN; 3]);
        for n in &self.nodes {
            for p in &n.parts {
                for v in &p.geometry.positions {
                    for k in 0..3 {
                        min[k] = min[k].min(v[k]);
                        max[k] = max[k].max(v[k]);
                    }
                }
            }
        }
        (min, max)
    }

    /// Number of triangles.
    pub fn triangles(&self) -> usize {
        self.nodes
            .iter()
            .flat_map(|n| &n.parts)
            .map(|p| p.geometry.indices.len() / 3)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cuboid_has_outward_counter_clockwise_faces() {
        let mut m = Model::new("t");
        let mat = m.material("a", 0xffffff, 0.0, 0.5);
        m.cuboid(0, mat, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]);
        let g = &m.nodes[0].parts[0].geometry;
        assert_eq!(g.indices.len(), 36);
        for t in g.indices.chunks(3) {
            let [a, b, c] = [0, 1, 2].map(|k| g.positions[t[k] as usize]);
            let n = cross(sub(b, a), sub(c, a));
            let stored = g.normals[t[0] as usize];
            let dot = n[0] * stored[0] + n[1] * stored[1] + n[2] * stored[2];
            assert!(dot > 0.0, "triangle {t:?} winds inward");
        }
    }

    #[test]
    fn cylinder_sides_and_caps_face_outward() {
        let mut m = Model::new("t");
        let mat = m.material("a", 0xffffff, 0.0, 0.5);
        m.cylinder(0, mat, [0.0, -1.0, 0.0], [0.0, 1.0, 0.0], 0.5, 12);
        let g = &m.nodes[0].parts[0].geometry;
        for t in g.indices.chunks(3) {
            let [a, b, c] = [0, 1, 2].map(|k| g.positions[t[k] as usize]);
            let n = cross(sub(b, a), sub(c, a));
            let centroid = mul(add(add(a, b), c), 1.0 / 3.0);
            let dot = n[0] * centroid[0] + n[1] * centroid[1] + n[2] * centroid[2];
            assert!(dot > 0.0, "triangle {t:?} winds inward");
        }
    }

    #[test]
    fn quantize_is_a_fixed_grid() {
        assert_eq!(quantize(0.1), 1638.0 / 16384.0);
        assert_eq!(quantize(-0.0), 0.0);
        assert_eq!(quantize(2.0), 2.0);
    }
}
