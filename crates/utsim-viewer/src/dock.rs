//! Docked egui panels and the 3D viewport.
//!
//! The panels (toolbar on top, palette / playback on the left, properties on the right)
//! are real egui panels that take space instead of floating over the scene. `bevy_egui`
//! does not hand out the root `Ui`, so each system makes a child `Ui` over the area that
//! is still free ([`Dock::free`]) and shrinks it after showing its panel. At the end the
//! 3D camera gets the free area as its viewport, so the scene is drawn, picked and
//! framed only where it is visible.

use bevy::camera::SubCameraView;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContexts, egui};

#[derive(Resource)]
pub struct Dock {
    /// Area (egui points) not covered by any panel; the 3D view lives here.
    pub free: egui::Rect,
    /// False until the first frame laid the panels out.
    pub ready: bool,
}

impl Default for Dock {
    fn default() -> Self {
        Dock {
            free: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
            ready: false,
        }
    }
}

impl Dock {
    /// A `Ui` covering what is still free, to show a panel in.
    pub fn ui(&self, ctx: &egui::Context, name: &str) -> egui::Ui {
        egui::Ui::new(
            ctx.clone(),
            egui::Id::new(("dock", name)),
            egui::UiBuilder::new()
                .layer_id(egui::LayerId::background())
                .max_rect(self.free),
        )
    }
}

/// Whether the pointer is over any egui widget, panel or window (so the 3D view must
/// ignore it).
pub fn pointer_over_ui(ctx: &egui::Context, dock: &Dock) -> bool {
    ctx.egui_wants_pointer_input()
        || ctx.is_pointer_over_egui()
        || (dock.ready
            && ctx
                .input(|i| i.pointer.hover_pos())
                .is_some_and(|p| !dock.free.contains(p)))
}

/// Start of every egui pass: the whole window is free again.
pub fn begin(mut dock: ResMut<Dock>, mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;
    dock.free = ctx.content_rect();
    Ok(())
}

/// Vertical field of view the scene is framed with (the free area behaves as if it
/// were the whole view with this field of view).
pub const FOV_Y: f32 = std::f32::consts::FRAC_PI_4;

/// End of the egui pass: shifts and scales the projection so the 3D scene fills the
/// free area. `bevy_egui` draws over the camera's viewport, so the viewport must stay the
/// whole window; instead the frustum is offset (`sub_camera_view`) and its field of view
/// widened, which keeps the pixel scale of a camera that only covered the free area.
pub fn apply_viewport(
    mut dock: ResMut<Dock>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut camera: Single<(&mut Camera, &mut Projection), With<Camera3d>>,
) {
    let r = dock.free;
    if r.width() < 8.0 || r.height() < 8.0 {
        return;
    }
    let s = window.scale_factor();
    let win = window.physical_size();
    let center = Vec2::new((r.min.x + r.max.x) / 2.0, (r.min.y + r.max.y) / 2.0) * s;
    let offset = Vec2::new(win.x as f32 / 2.0, win.y as f32 / 2.0) - center;
    let sub = SubCameraView {
        full_size: win,
        offset,
        size: win,
    };
    let (cam, projection) = &mut *camera;
    if cam.sub_camera_view != Some(sub) {
        cam.sub_camera_view = Some(sub);
    }
    let fov = 2.0 * ((FOV_Y / 2.0).tan() * window.height() / r.height()).atan();
    if let Projection::Perspective(p) = &mut **projection
        && (p.fov - fov).abs() > 1e-6
    {
        p.fov = fov;
    }
    dock.ready = true;
}
