//! Binary glTF 2.0 (`.glb`) writer for a [`Model`].
//!
//! The JSON chunk is written by hand in a fixed key order and every float is quantized
//! ([`quantize`]), so the same model always gives the same bytes on every platform.
//!
//! Conversion from asset coordinates (Z up) to glTF (Y up): `(x, y, z) -> (x, z, -y)`, a
//! proper rotation, so winding and handedness are kept. Assets face `+x` (the UTSim yaw
//! convention), which is glTF `+X` too; engines that expect glTF's `+Z` forward must turn
//! the asset by -90 degrees around the vertical.

use crate::builder::{Geometry, Model, V3, quantize};

const ARRAY_BUFFER: u32 = 34962;
const ELEMENT_ARRAY_BUFFER: u32 = 34963;
const FLOAT: u32 = 5126;
const UNSIGNED_SHORT: u32 = 5123;
const UNSIGNED_INT: u32 = 5125;

/// Asset coordinates to glTF coordinates.
pub fn to_gltf(p: V3) -> V3 {
    [quantize(p[0]), quantize(p[2]), quantize(-p[1])]
}

/// sRGB component to linear (glTF stores linear factors).
fn linear(c: f32) -> f32 {
    let l = if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    };
    quantize(l)
}

/// Minimal JSON text builder (fixed order, no escaping surprises).
struct Json(String);

impl Json {
    fn raw(&mut self, s: &str) {
        self.0.push_str(s);
    }

    fn string(&mut self, s: &str) {
        self.0.push('"');
        for c in s.chars() {
            match c {
                '"' => self.0.push_str("\\\""),
                '\\' => self.0.push_str("\\\\"),
                c if (c as u32) < 0x20 => self.0.push_str(&format!("\\u{:04x}", c as u32)),
                c => self.0.push(c),
            }
        }
        self.0.push('"');
    }

    fn num(&mut self, x: f32) {
        // `Display` of f64 is the shortest exact decimal and never uses an exponent.
        let v = f64::from(x);
        self.0.push_str(&format!("{v}"));
    }

    fn nums(&mut self, xs: &[f32]) {
        self.raw("[");
        for (k, &x) in xs.iter().enumerate() {
            if k > 0 {
                self.raw(",");
            }
            self.num(x);
        }
        self.raw("]");
    }

    fn ints(&mut self, xs: &[usize]) {
        self.raw("[");
        for (k, x) in xs.iter().enumerate() {
            if k > 0 {
                self.raw(",");
            }
            self.raw(&x.to_string());
        }
        self.raw("]");
    }
}

struct Accessor {
    view: usize,
    component: u32,
    count: usize,
    kind: &'static str,
    bounds: Option<(V3, V3)>,
}

struct View {
    offset: usize,
    length: usize,
    target: u32,
}

#[derive(Default)]
struct Bin {
    data: Vec<u8>,
    views: Vec<View>,
    accessors: Vec<Accessor>,
}

impl Bin {
    fn view(&mut self, bytes: &[u8], target: u32) -> usize {
        while self.data.len() % 4 != 0 {
            self.data.push(0);
        }
        self.views.push(View {
            offset: self.data.len(),
            length: bytes.len(),
            target,
        });
        self.data.extend_from_slice(bytes);
        self.views.len() - 1
    }

    fn vec3(&mut self, values: &[V3], bounds: bool) -> usize {
        let mut bytes = Vec::with_capacity(values.len() * 12);
        let (mut min, mut max) = ([f32::MAX; 3], [f32::MIN; 3]);
        for v in values {
            for k in 0..3 {
                bytes.extend_from_slice(&v[k].to_le_bytes());
                min[k] = min[k].min(v[k]);
                max[k] = max[k].max(v[k]);
            }
        }
        let view = self.view(&bytes, ARRAY_BUFFER);
        self.accessors.push(Accessor {
            view,
            component: FLOAT,
            count: values.len(),
            kind: "VEC3",
            bounds: bounds.then_some((min, max)),
        });
        self.accessors.len() - 1
    }

    fn indices(&mut self, indices: &[u32], vertices: usize) -> usize {
        let (bytes, component) = if vertices <= u16::MAX as usize {
            let b: Vec<u8> = indices
                .iter()
                .flat_map(|&i| (i as u16).to_le_bytes())
                .collect();
            (b, UNSIGNED_SHORT)
        } else {
            let b: Vec<u8> = indices.iter().flat_map(|&i| i.to_le_bytes()).collect();
            (b, UNSIGNED_INT)
        };
        let view = self.view(&bytes, ELEMENT_ARRAY_BUFFER);
        self.accessors.push(Accessor {
            view,
            component,
            count: indices.len(),
            kind: "SCALAR",
            bounds: None,
        });
        self.accessors.len() - 1
    }
}

/// `(position, normal, indices)` accessors of one primitive, relative to `pivot`.
fn primitive(bin: &mut Bin, g: &Geometry, pivot: V3) -> (usize, usize, usize) {
    let positions: Vec<V3> = g
        .positions
        .iter()
        .map(|p| to_gltf([p[0] - pivot[0], p[1] - pivot[1], p[2] - pivot[2]]))
        .collect();
    let normals: Vec<V3> = g.normals.iter().map(|n| to_gltf(*n)).collect();
    let pos = bin.vec3(&positions, true);
    let nor = bin.vec3(&normals, false);
    let idx = bin.indices(&g.indices, positions.len());
    (pos, nor, idx)
}

/// The model as a `.glb` file.
pub fn write(model: &Model) -> Vec<u8> {
    let mut bin = Bin::default();
    let mut j = Json(String::new());

    // Meshes (one per node with geometry, one primitive per material).
    let mut mesh_of: Vec<Option<usize>> = vec![None; model.nodes.len()];
    let mut meshes = Json(String::new());
    let mut mesh_count = 0;
    for (k, node) in model.nodes.iter().enumerate() {
        let parts: Vec<_> = node
            .parts
            .iter()
            .filter(|p| !p.geometry.indices.is_empty())
            .collect();
        if parts.is_empty() {
            continue;
        }
        if mesh_count > 0 {
            meshes.raw(",");
        }
        meshes.raw("{\"name\":");
        meshes.string(&node.name);
        meshes.raw(",\"primitives\":[");
        for (n, part) in parts.iter().enumerate() {
            let (pos, nor, idx) = primitive(&mut bin, &part.geometry, node.pivot);
            if n > 0 {
                meshes.raw(",");
            }
            meshes.raw(&format!(
                "{{\"attributes\":{{\"NORMAL\":{nor},\"POSITION\":{pos}}},\"indices\":{idx},\"material\":{},\"mode\":4}}",
                part.material
            ));
        }
        meshes.raw("]}");
        mesh_of[k] = Some(mesh_count);
        mesh_count += 1;
    }
    while bin.data.len() % 4 != 0 {
        bin.data.push(0);
    }

    j.raw("{\"asset\":{\"generator\":\"utsim-assets\",\"version\":\"2.0\"}");
    j.raw(",\"scene\":0,\"scenes\":[{\"name\":");
    j.string(&model.name);
    j.raw(",\"nodes\":[0]}]");

    j.raw(",\"nodes\":[");
    for (k, node) in model.nodes.iter().enumerate() {
        if k > 0 {
            j.raw(",");
        }
        j.raw("{\"name\":");
        j.string(&node.name);
        let children: Vec<usize> = model
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.parent == Some(k))
            .map(|(c, _)| c)
            .collect();
        if !children.is_empty() {
            j.raw(",\"children\":");
            j.ints(&children);
        }
        if let Some(m) = mesh_of[k] {
            j.raw(&format!(",\"mesh\":{m}"));
        }
        let parent = node.parent.map_or([0.0; 3], |p| model.nodes[p].pivot);
        let t = to_gltf([
            node.pivot[0] - parent[0],
            node.pivot[1] - parent[1],
            node.pivot[2] - parent[2],
        ]);
        if t != [0.0; 3] {
            j.raw(",\"translation\":");
            j.nums(&t);
        }
        j.raw("}");
    }
    j.raw("]");

    if mesh_count > 0 {
        j.raw(",\"meshes\":[");
        j.raw(&meshes.0);
        j.raw("]");
    }

    j.raw(",\"materials\":[");
    for (k, m) in model.materials.iter().enumerate() {
        if k > 0 {
            j.raw(",");
        }
        j.raw("{\"name\":");
        j.string(&m.name);
        let blend = m.color[3] < 1.0;
        if blend {
            j.raw(",\"alphaMode\":\"BLEND\",\"doubleSided\":true");
        }
        if m.emissive != [0.0; 3] {
            j.raw(",\"emissiveFactor\":");
            j.nums(&m.emissive.map(linear));
        }
        j.raw(",\"pbrMetallicRoughness\":{\"baseColorFactor\":");
        j.nums(&[
            linear(m.color[0]),
            linear(m.color[1]),
            linear(m.color[2]),
            quantize(m.color[3]),
        ]);
        j.raw(",\"metallicFactor\":");
        j.num(quantize(m.metallic));
        j.raw(",\"roughnessFactor\":");
        j.num(quantize(m.roughness));
        j.raw("}}");
    }
    j.raw("]");

    j.raw(",\"accessors\":[");
    for (k, a) in bin.accessors.iter().enumerate() {
        if k > 0 {
            j.raw(",");
        }
        j.raw(&format!(
            "{{\"bufferView\":{},\"componentType\":{},\"count\":{},\"type\":\"{}\"",
            a.view, a.component, a.count, a.kind
        ));
        if let Some((min, max)) = a.bounds {
            j.raw(",\"max\":");
            j.nums(&max);
            j.raw(",\"min\":");
            j.nums(&min);
        }
        j.raw("}");
    }
    j.raw("]");

    j.raw(",\"bufferViews\":[");
    for (k, v) in bin.views.iter().enumerate() {
        if k > 0 {
            j.raw(",");
        }
        j.raw(&format!(
            "{{\"buffer\":0,\"byteLength\":{},\"byteOffset\":{},\"target\":{}}}",
            v.length, v.offset, v.target
        ));
    }
    j.raw("]");
    j.raw(&format!(
        ",\"buffers\":[{{\"byteLength\":{}}}]}}",
        bin.data.len()
    ));

    let mut json = j.0.into_bytes();
    while json.len() % 4 != 0 {
        json.push(b' ');
    }
    let total = 12 + 8 + json.len() + 8 + bin.data.len();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&(total as u32).to_le_bytes());
    out.extend_from_slice(&(json.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json);
    out.extend_from_slice(&(bin.data.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\0");
    out.extend_from_slice(&bin.data);
    out
}
