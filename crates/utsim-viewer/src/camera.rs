//! Orbit / pan / zoom camera.
//!
//! Left drag: pan. Right drag: orbit. Wheel: zoom. A left click without dragging
//! selects the block under the cursor (see `scene::pick`). `F` (or the "Enquadrar"
//! button) frames the whole model inside the free area left by the docked panels.

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use utsim_viewer::editing::NetworkView;
use utsim_viewer::view;

use crate::dock::{Dock, pointer_over_ui};
use crate::editor::{Editor, Mode};
use crate::playback::Data;

/// Set by the editor while a left-button gesture belongs to a block (dragging it),
/// so the camera does not pan at the same time.
#[derive(Resource, Default)]
pub struct CameraLock(pub bool);

#[derive(Resource, Clone, Copy)]
pub struct OrbitCam {
    pub target: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub dist: f32,
}

impl OrbitCam {
    /// Frames a scene whose blocks span `min..max` (Bevy space).
    pub fn framing(min: Vec3, max: Vec3) -> Self {
        let target = (min + max) / 2.0;
        let radius = ((max - min).length() / 2.0).max(4.0);
        OrbitCam {
            target,
            yaw: 0.0,
            pitch: 0.75,
            dist: radius * 2.2 + 4.0,
        }
    }

    pub fn transform(&self) -> Transform {
        let rot = Quat::from_euler(EulerRot::YXZ, self.yaw, -self.pitch, 0.0);
        let pos = self.target + rot * Vec3::new(0.0, 0.0, self.dist);
        Transform::from_translation(pos).looking_at(self.target, Vec3::Y)
    }
}

#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn control(
    mut cam: ResMut<OrbitCam>,
    lock: Res<CameraLock>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    dock: Res<Dock>,
    mut contexts: EguiContexts,
    mut transform: Single<&mut Transform, With<Camera3d>>,
) {
    let over_ui = contexts
        .ctx_mut()
        .is_ok_and(|ctx| pointer_over_ui(ctx, &dock));
    if !over_ui {
        let d = motion.delta;
        if buttons.pressed(MouseButton::Right) {
            cam.yaw -= d.x * 0.006;
            cam.pitch = (cam.pitch + d.y * 0.006).clamp(0.05, 1.55);
        } else if buttons.pressed(MouseButton::Left) && d != Vec2::ZERO && !lock.0 {
            // Pan on the ground plane, relative to where the camera looks.
            let rot = Quat::from_rotation_y(cam.yaw);
            let right = rot * Vec3::X;
            let forward = rot * Vec3::NEG_Z;
            let k = cam.dist * 0.0014;
            cam.target += (-right * d.x + forward * d.y) * k;
        }
        let lines = match scroll.unit {
            MouseScrollUnit::Line => scroll.delta.y,
            MouseScrollUnit::Pixel => scroll.delta.y / 40.0,
        };
        if lines != 0.0 {
            cam.dist = (cam.dist * (1.0 - 0.1 * lines)).clamp(1.5, 5000.0);
        }
    }
    **transform = cam.transform();
}

/// Frames still to spend re-fitting the model after a request (the panels need a frame
/// or two to settle on their size before the free area is final).
#[derive(Resource, Default)]
pub struct FrameRequest(pub u32);

/// Fits the whole model (blocks and path network) in the free area: on open, when
/// entering `Simular`, after a new document, and on `F` / "Enquadrar".
pub fn frame_scene(
    mut request: ResMut<FrameRequest>,
    mut editor: ResMut<Editor>,
    data: Res<Data>,
    keys: Res<ButtonInput<KeyCode>>,
    dock: Res<Dock>,
    mut cam: ResMut<OrbitCam>,
    mut contexts: EguiContexts,
) {
    let typing = contexts
        .ctx_mut()
        .is_ok_and(|ctx| ctx.egui_wants_keyboard_input());
    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    if (keys.just_pressed(KeyCode::KeyF) && !typing && !ctrl) || editor.frame_request {
        editor.frame_request = false;
        request.0 = 6;
    }
    if request.0 == 0 || !dock.ready {
        return;
    }
    let size = dock.free.size();
    request.0 -= 1;
    let (min, max) = match editor.mode {
        Mode::Edit => {
            let net = NetworkView::of(editor.doc.model());
            let halves: Vec<[f32; 3]> = editor.view.geoms.iter().map(|g| g.half).collect();
            let mut points = net.nodes;
            points.extend(editor.fluid_view.bounds_points());
            view::scene_bounds(&editor.view.positions, &halves, &points)
        }
        Mode::Sim => {
            let halves: Vec<[f32; 3]> = data.geoms.iter().map(|g| g.half).collect();
            let mut points = data.network.nodes.clone();
            points.extend(data.fluid.bounds_points());
            view::scene_bounds(&data.positions, &halves, &points)
        }
    }
    .unwrap_or(([0.0; 3], [0.0; 3]));
    // An empty or tiny model still gets a ground area to work on (16 x 16 at least).
    let (mut min, mut max) = (min, max);
    for k in [0, 2] {
        let mid = (min[k] + max[k]) / 2.0;
        if max[k] - min[k] < 16.0 {
            (min[k], max[k]) = (mid - 8.0, mid + 8.0);
        }
    }
    let f = view::frame_box(
        min,
        max,
        cam.yaw,
        cam.pitch,
        size.x / size.y.max(1.0),
        crate::dock::FOV_Y,
        0.08,
    );
    cam.target = Vec3::from(f.target);
    cam.dist = f.dist.clamp(1.5, 5000.0);
}
