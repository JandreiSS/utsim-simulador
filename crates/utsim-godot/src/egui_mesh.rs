//! egui's paint output to what Godot's `RenderingServer` draws (pure: no Godot types,
//! unit tested).
//!
//! egui gives triangle meshes in points, with premultiplied sRGB colours, each with a clip
//! rectangle and a texture (the font atlas or an image). [`convert`] turns one of them into
//! the arrays of `canvas_item_add_triangle_array`: pixels, straight-alpha colours (the 2D
//! canvas blends `src_alpha, 1 - src_alpha`) and the clip in pixels. [`Textures`] keeps a
//! CPU copy of every egui texture, so the partial updates of the font atlas become whole
//! images Godot can upload.

use std::collections::HashMap;

use egui::epaint::{ClippedPrimitive, ImageData, ImageDelta, Primitive};
use egui::{Color32, TextureId};

/// One mesh, ready for `canvas_item_add_triangle_array`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshArrays {
    pub points: Vec<[f32; 2]>,
    /// Straight-alpha RGBA in 0..1, sRGB as Godot's 2D canvas expects.
    pub colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<i32>,
    pub texture: Option<TextureId>,
    /// Clip rectangle in pixels: `[x, y, width, height]`.
    pub clip: [f32; 4],
}

/// A premultiplied egui colour as straight alpha (what a straight-alpha blend needs; a
/// texel times a vertex colour gives the same result either way).
pub fn unmultiply(c: Color32) -> [f32; 4] {
    let a = c.a();
    if a == 0 {
        return [0.0, 0.0, 0.0, 0.0];
    }
    let k = 255.0 / a as f32;
    [
        (c.r() as f32 * k / 255.0).min(1.0),
        (c.g() as f32 * k / 255.0).min(1.0),
        (c.b() as f32 * k / 255.0).min(1.0),
        a as f32 / 255.0,
    ]
}

/// The clip rectangle of a primitive in whole pixels, inside the screen (`size` in
/// pixels); `None` when nothing of it is visible.
pub fn clip_pixels(rect: egui::Rect, ppp: f32, size: [f32; 2]) -> Option<[f32; 4]> {
    let x0 = (rect.min.x * ppp).floor().clamp(0.0, size[0]);
    let y0 = (rect.min.y * ppp).floor().clamp(0.0, size[1]);
    let x1 = (rect.max.x * ppp).ceil().clamp(0.0, size[0]);
    let y1 = (rect.max.y * ppp).ceil().clamp(0.0, size[1]);
    if x1 <= x0 || y1 <= y0 {
        return None;
    }
    Some([x0, y0, x1 - x0, y1 - y0])
}

/// The arrays of one primitive, or `None` for an empty mesh, a clip outside the screen or
/// a paint callback (not used by this interface).
pub fn convert(prim: &ClippedPrimitive, ppp: f32, size: [f32; 2]) -> Option<MeshArrays> {
    let Primitive::Mesh(mesh) = &prim.primitive else {
        return None;
    };
    if mesh.indices.is_empty() || mesh.vertices.is_empty() {
        return None;
    }
    let clip = clip_pixels(prim.clip_rect, ppp, size)?;
    let n = mesh.vertices.len();
    let mut out = MeshArrays {
        points: Vec::with_capacity(n),
        colors: Vec::with_capacity(n),
        uvs: Vec::with_capacity(n),
        indices: mesh.indices.iter().map(|&i| i as i32).collect(),
        texture: Some(mesh.texture_id),
        clip,
    };
    for v in &mesh.vertices {
        out.points.push([v.pos.x * ppp, v.pos.y * ppp]);
        out.colors.push(unmultiply(v.color));
        out.uvs.push([v.uv.x, v.uv.y]);
    }
    Some(out)
}

/// An egui texture kept on the CPU: straight-alpha RGBA8, row by row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CpuImage {
    pub size: [usize; 2],
    pub rgba: Vec<u8>,
    /// Linear filtering (the font atlas and smooth images) or nearest.
    pub linear: bool,
}

/// Straight-alpha RGBA8 bytes of an egui image.
pub fn image_bytes(image: &ImageData) -> Vec<u8> {
    let ImageData::Color(c) = image;
    let mut bytes = Vec::with_capacity(c.pixels.len() * 4);
    for p in &c.pixels {
        bytes.extend_from_slice(&p.to_srgba_unmultiplied());
    }
    bytes
}

/// Every egui texture, as whole images.
#[derive(Default, Debug)]
pub struct Textures {
    pub images: HashMap<TextureId, CpuImage>,
}

/// What a texture change means for the GPU copy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    /// A new texture, or one whose size changed: create it again.
    Created,
    /// Same size: update it in place.
    Updated,
    /// A patch of a texture that does not exist (ignored).
    Ignored,
}

impl Textures {
    /// Applies one delta; returns what to do with the GPU copy.
    pub fn apply(&mut self, id: TextureId, delta: &ImageDelta) -> Change {
        let [w, h] = delta.image.size();
        let bytes = image_bytes(&delta.image);
        let linear = matches!(delta.options.magnification, egui::TextureFilter::Linear);
        match delta.pos {
            None => {
                let created = self.images.get(&id).is_none_or(|old| old.size != [w, h]);
                self.images.insert(
                    id,
                    CpuImage {
                        size: [w, h],
                        rgba: bytes,
                        linear,
                    },
                );
                if created {
                    Change::Created
                } else {
                    Change::Updated
                }
            }
            Some([x, y]) => {
                let Some(img) = self.images.get_mut(&id) else {
                    return Change::Ignored;
                };
                let [iw, ih] = img.size;
                for row in 0..h {
                    let ty = y + row;
                    if ty >= ih {
                        break;
                    }
                    let cols = w.min(iw.saturating_sub(x));
                    let src = row * w * 4;
                    let dst = (ty * iw + x) * 4;
                    img.rgba[dst..dst + cols * 4].copy_from_slice(&bytes[src..src + cols * 4]);
                }
                Change::Updated
            }
        }
    }

    pub fn free(&mut self, id: TextureId) {
        self.images.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::epaint::{ColorImage, Mesh, Vertex};
    use egui::{Pos2, Rect, TextureOptions};
    use std::sync::Arc;

    #[test]
    fn colours_lose_the_premultiplication() {
        assert_eq!(unmultiply(Color32::TRANSPARENT), [0.0; 4]);
        assert_eq!(unmultiply(Color32::WHITE), [1.0; 4]);
        // Half-transparent white: premultiplied (128, 128, 128, 128).
        let c = unmultiply(Color32::from_rgba_premultiplied(128, 128, 128, 128));
        assert!((c[0] - 1.0).abs() < 1e-6 && (c[3] - 128.0 / 255.0).abs() < 1e-6);
        let red = unmultiply(Color32::from_rgba_unmultiplied(200, 0, 0, 100));
        assert!((red[0] - 200.0 / 255.0).abs() < 0.02, "{red:?}");
    }

    #[test]
    fn clips_are_whole_pixels_inside_the_screen() {
        let r = Rect::from_min_max(Pos2::new(-5.0, 10.2), Pos2::new(50.5, 2000.0));
        assert_eq!(
            clip_pixels(r, 2.0, [800.0, 600.0]),
            Some([0.0, 20.0, 101.0, 580.0])
        );
        let outside = Rect::from_min_max(Pos2::new(900.0, 0.0), Pos2::new(950.0, 10.0));
        assert_eq!(clip_pixels(outside, 1.0, [800.0, 600.0]), None);
    }

    #[test]
    fn a_mesh_becomes_pixel_arrays() {
        let mut mesh = Mesh::default();
        for (p, uv) in [
            ((0.0, 0.0), (0.0, 0.0)),
            ((10.0, 0.0), (1.0, 0.0)),
            ((0.0, 5.0), (0.0, 1.0)),
        ] {
            mesh.vertices.push(Vertex {
                pos: Pos2::new(p.0, p.1),
                uv: Pos2::new(uv.0, uv.1),
                color: Color32::WHITE,
            });
        }
        mesh.indices = vec![0, 1, 2];
        let prim = ClippedPrimitive {
            clip_rect: Rect::from_min_max(Pos2::ZERO, Pos2::new(100.0, 100.0)),
            primitive: Primitive::Mesh(mesh),
        };
        let a = convert(&prim, 1.5, [1600.0, 900.0]).expect("malha");
        assert_eq!(a.points[1], [15.0, 0.0]);
        assert_eq!(a.points[2], [0.0, 7.5]);
        assert_eq!(a.indices, vec![0, 1, 2]);
        assert_eq!(a.uvs[1], [1.0, 0.0]);
        assert_eq!(a.clip, [0.0, 0.0, 150.0, 150.0]);
        assert_eq!(a.texture, Some(TextureId::default()));
        let empty = ClippedPrimitive {
            clip_rect: prim.clip_rect,
            primitive: Primitive::Mesh(Mesh::default()),
        };
        assert!(convert(&empty, 1.0, [100.0, 100.0]).is_none());
    }

    fn image(w: usize, h: usize, c: Color32) -> ImageData {
        ImageData::Color(Arc::new(ColorImage::new([w, h], vec![c; w * h])))
    }

    #[test]
    fn patches_update_the_cpu_copy() {
        let id = TextureId::Managed(0);
        let mut t = Textures::default();
        let full = ImageDelta::full(image(4, 4, Color32::BLACK), TextureOptions::LINEAR);
        assert_eq!(t.apply(id, &full), Change::Created);
        assert!(t.images[&id].linear);
        let patch =
            ImageDelta::partial([1, 2], image(2, 1, Color32::WHITE), TextureOptions::LINEAR);
        assert_eq!(t.apply(id, &patch), Change::Updated);
        let img = &t.images[&id];
        let at = |x: usize, y: usize| img.rgba[(y * 4 + x) * 4];
        assert_eq!((at(1, 2), at(2, 2), at(3, 2), at(1, 1)), (255, 255, 0, 0));
        // The same size again updates; a new size creates.
        assert_eq!(t.apply(id, &full), Change::Updated);
        let bigger = ImageDelta::full(image(8, 8, Color32::BLACK), TextureOptions::NEAREST);
        assert_eq!(t.apply(id, &bigger), Change::Created);
        assert!(!t.images[&id].linear);
        // A patch that runs past the edge is cut, and one of an unknown texture ignored.
        let edge = ImageDelta::partial([7, 7], image(3, 3, Color32::WHITE), TextureOptions::LINEAR);
        assert_eq!(t.apply(id, &edge), Change::Updated);
        assert_eq!(t.apply(TextureId::Managed(9), &edge), Change::Ignored);
        t.free(id);
        assert!(t.images.is_empty());
    }
}
