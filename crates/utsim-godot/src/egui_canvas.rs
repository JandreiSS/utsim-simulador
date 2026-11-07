//! `EguiCanvas`: what any Godot node that shows an egui interface needs, taken out of
//! `UtsimUi` so that other nodes (the UTSim Ensino's mission mode) do not copy it: reading
//! Godot's input events, presenting an egui frame (platform commands, cursor, textures
//! uploaded to Godot, the meshes drawn as clipped canvas items) and freeing what it made.
//!
//! A node keeps one `EguiCanvas` and its own `egui::Context`: it turns `_input` events into
//! egui events with [`read_input`] and [`crate::egui_input::to_egui`], runs its frame, then
//! calls [`EguiCanvas::present`] with the output and its canvas item; `free_all` when it
//! leaves the tree. Only the core Godot classes are generated for the bridge, so the input
//! events, `Image` and `DisplayServer` are reached through dynamic calls.

use std::collections::HashMap;

use egui::TextureId;
use godot::classes::input::CursorShape;
use godot::classes::rendering_server::CanvasItemTextureFilter;
use godot::classes::{
    ClassDb, Engine, Input as GdInput, InputEvent, Object, Os, RenderingServer, Viewport,
};
use godot::prelude::*;

use crate::egui_input::{Input, Mods};
use crate::egui_mesh::{self, Change, Textures};

/// Godot's `Image.FORMAT_RGBA8`.
const FORMAT_RGBA8: i64 = 5;

/// The Godot side of an egui interface (see the module).
#[derive(Default)]
pub struct EguiCanvas {
    textures: Textures,
    gpu: HashMap<TextureId, Rid>,
    items: Vec<Rid>,
    cursor: Option<egui::CursorIcon>,
}

impl EguiCanvas {
    pub fn new() -> EguiCanvas {
        EguiCanvas::default()
    }

    /// Presents an egui frame: runs the platform commands (clipboard, links, cursor),
    /// uploads the texture changes, tessellates and draws the shapes under `parent` for a
    /// screen of `size` pixels. `extra` resolves textures the node owns itself (user
    /// textures, a viewport), before the egui ones.
    pub fn present(
        &mut self,
        ctx: &egui::Context,
        mut output: egui::FullOutput,
        parent: Rid,
        size: [f32; 2],
        extra: impl Fn(TextureId) -> Option<Rid>,
    ) {
        let mut textures = std::mem::take(&mut output.textures_delta);
        self.platform(&output.platform_output);
        for (id, deltas) in &textures.set {
            for d in deltas {
                match self.textures.apply(*id, d) {
                    Change::Created => self.upload(*id, true),
                    Change::Updated => self.upload(*id, false),
                    Change::Ignored => {}
                }
            }
        }
        let prims = ctx.tessellate(output.shapes, output.pixels_per_point);
        self.paint(parent, &prims, output.pixels_per_point, size, extra);
        for id in &textures.free {
            self.textures.free(*id);
            if let Some(rid) = self.gpu.remove(id) {
                RenderingServer::singleton().free_rid(rid);
            }
        }
        // Applied above (egui checks that every delta was handled).
        textures.clear();
    }

    fn platform(&mut self, out: &egui::PlatformOutput) {
        for c in &out.commands {
            match c {
                egui::OutputCommand::CopyText(text) => clipboard_set(text),
                egui::OutputCommand::OpenUrl(u) => {
                    Os::singleton().shell_open(u.url.as_str());
                }
                egui::OutputCommand::CopyImage(_) => {}
            }
        }
        if self.cursor != Some(out.cursor_icon) {
            self.cursor = Some(out.cursor_icon);
            GdInput::singleton()
                .set_default_cursor_shape_ex()
                .shape(cursor_shape(out.cursor_icon))
                .done();
        }
    }

    /// Sends the CPU copy of texture `id` to Godot (a new texture when `create`).
    fn upload(&mut self, id: TextureId, create: bool) {
        let Some(img) = self.textures.images.get(&id) else {
            return;
        };
        let mut image: Gd<Object> = match ClassDb::singleton().instantiate("Image").try_to() {
            Ok(i) => i,
            Err(_) => return,
        };
        let bytes = PackedByteArray::from(img.rgba.as_slice());
        image.call(
            "set_data",
            &[
                (img.size[0] as i64).to_variant(),
                (img.size[1] as i64).to_variant(),
                false.to_variant(),
                FORMAT_RGBA8.to_variant(),
                bytes.to_variant(),
            ],
        );
        let mut rs = RenderingServer::singleton();
        match self.gpu.get(&id).copied() {
            Some(rid) if !create => {
                rs.call(
                    "texture_2d_update",
                    &[rid.to_variant(), image.to_variant(), 0i64.to_variant()],
                );
            }
            old => {
                if let Some(rid) = old {
                    rs.free_rid(rid);
                }
                let rid = rs
                    .call("texture_2d_create", &[image.to_variant()])
                    .try_to::<Rid>()
                    .unwrap_or(Rid::Invalid);
                self.gpu.insert(id, rid);
            }
        }
    }

    /// Draws the meshes, one canvas item each, clipped.
    fn paint(
        &mut self,
        parent: Rid,
        prims: &[egui::ClippedPrimitive],
        ppp: f32,
        size: [f32; 2],
        extra: impl Fn(TextureId) -> Option<Rid>,
    ) {
        let mut rs = RenderingServer::singleton();
        let mut used = 0;
        for prim in prims {
            let Some(m) = egui_mesh::convert(prim, ppp, size) else {
                continue;
            };
            if used == self.items.len() {
                let item = rs.canvas_item_create();
                rs.canvas_item_set_parent(item, parent);
                rs.canvas_item_set_default_texture_filter(item, CanvasItemTextureFilter::LINEAR);
                rs.canvas_item_set_clip(item, true);
                self.items.push(item);
            }
            let item = self.items[used];
            rs.canvas_item_clear(item);
            rs.canvas_item_set_draw_index(item, used as i32);
            let [x, y, w, h] = m.clip;
            rs.canvas_item_set_custom_rect_ex(item, true)
                .rect(Rect2::new(Vector2::new(x, y), Vector2::new(w, h)))
                .done();
            let indices = PackedInt32Array::from(m.indices.as_slice());
            let points: PackedVector2Array =
                m.points.iter().map(|p| Vector2::new(p[0], p[1])).collect();
            let colors: PackedColorArray = m
                .colors
                .iter()
                .map(|c| Color::from_rgba(c[0], c[1], c[2], c[3]))
                .collect();
            let uvs: PackedVector2Array = m.uvs.iter().map(|p| Vector2::new(p[0], p[1])).collect();
            let texture = m
                .texture
                .and_then(|t| extra(t).or_else(|| self.gpu.get(&t).copied()))
                .unwrap_or(Rid::Invalid);
            rs.canvas_item_add_triangle_array_ex(item, &indices, &points, &colors)
                .uvs(&uvs)
                .texture(texture)
                .done();
            used += 1;
        }
        for item in &self.items[used..] {
            rs.canvas_item_clear(*item);
        }
    }

    /// Frees the canvas items and the textures (when the node leaves).
    pub fn free_all(&mut self) {
        let mut rs = RenderingServer::singleton();
        for item in self.items.drain(..) {
            rs.free_rid(item);
        }
        for (_, rid) in self.gpu.drain() {
            rs.free_rid(rid);
        }
    }
}

/// The size of `vp`'s visible area (pixels) and its content scale factor.
pub fn screen_of(vp: &Gd<Viewport>) -> ([f32; 2], f32) {
    let size = vp.get_visible_rect().size;
    let content = vp
        .get_window()
        .map(|w| w.get_content_scale_factor())
        .unwrap_or(1.0);
    ([size.x, size.y], if content > 0.0 { content } else { 1.0 })
}

/// The plain data of a Godot input event (dynamic reads: the event classes are not
/// generated for the bridge).
pub fn read_input(ev: &Gd<InputEvent>) -> Option<Input> {
    let class = ev.get_class().to_string();
    let b = |k: &str| ev.get(k).try_to::<bool>().unwrap_or(false);
    let mods = || Mods {
        shift: b("shift_pressed"),
        ctrl: b("ctrl_pressed"),
        alt: b("alt_pressed"),
        meta: b("meta_pressed"),
    };
    let pos = || {
        let p = ev.get("position").try_to::<Vector2>().unwrap_or_default();
        [p.x, p.y]
    };
    match class.as_str() {
        "InputEventMouseMotion" => Some(Input::Motion {
            pos: pos(),
            mods: mods(),
        }),
        "InputEventMouseButton" => Some(Input::Button {
            pos: pos(),
            button: ev.get("button_index").try_to::<i64>().unwrap_or(0),
            pressed: b("pressed"),
            factor: ev.get("factor").try_to::<f32>().unwrap_or(1.0),
            mods: mods(),
        }),
        "InputEventKey" => Some(Input::Key {
            keycode: ev.get("keycode").try_to::<i64>().unwrap_or(0),
            unicode: ev.get("unicode").try_to::<i64>().unwrap_or(0).max(0) as u32,
            pressed: b("pressed"),
            echo: b("echo"),
            mods: mods(),
        }),
        _ => None,
    }
}

fn display_server() -> Option<Gd<Object>> {
    Engine::singleton().get_singleton("DisplayServer")
}

pub fn clipboard_get() -> String {
    display_server()
        .map(|mut d| d.call("clipboard_get", &[]).to_string())
        .unwrap_or_default()
}

pub fn clipboard_set(text: &str) {
    if let Some(mut d) = display_server() {
        d.call("clipboard_set", &[GString::from(text).to_variant()]);
    }
}

fn cursor_shape(c: egui::CursorIcon) -> CursorShape {
    use egui::CursorIcon as C;
    match c {
        C::PointingHand => CursorShape::POINTING_HAND,
        C::Text | C::VerticalText => CursorShape::IBEAM,
        C::ResizeHorizontal | C::ResizeColumn | C::ResizeEast | C::ResizeWest => CursorShape::HSIZE,
        C::ResizeVertical | C::ResizeRow | C::ResizeNorth | C::ResizeSouth => CursorShape::VSIZE,
        C::ResizeNeSw | C::ResizeNorthEast | C::ResizeSouthWest => CursorShape::BDIAGSIZE,
        C::ResizeNwSe | C::ResizeNorthWest | C::ResizeSouthEast => CursorShape::FDIAGSIZE,
        C::Grab | C::Grabbing | C::Move | C::AllScroll => CursorShape::MOVE,
        C::NotAllowed | C::NoDrop => CursorShape::FORBIDDEN,
        C::Wait => CursorShape::WAIT,
        C::Progress => CursorShape::BUSY,
        C::Help => CursorShape::HELP,
        C::Crosshair | C::Cell => CursorShape::CROSS,
        _ => CursorShape::ARROW,
    }
}
