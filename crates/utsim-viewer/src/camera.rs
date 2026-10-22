//! Orbit / pan / zoom camera.
//!
//! Left drag: pan. Right drag: orbit. Wheel: zoom. A left click without dragging
//! selects the block under the cursor (see `scene::pick`).

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

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

pub fn control(
    mut cam: ResMut<OrbitCam>,
    lock: Res<CameraLock>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    mut contexts: EguiContexts,
    mut transform: Single<&mut Transform, With<Camera3d>>,
) {
    let over_ui = contexts
        .ctx_mut()
        .is_ok_and(|ctx| ctx.egui_wants_pointer_input() || ctx.is_pointer_over_egui());
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
