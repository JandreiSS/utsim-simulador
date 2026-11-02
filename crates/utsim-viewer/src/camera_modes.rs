//! Camera modes: orbit, follow, first person and ride-along.
//!
//! Every mode is a [`CameraController`]: it takes the input of the frame ([`CamInput`])
//! and what the scene knows ([`CamWorld`]: the tracked vehicle, the footprints to walk
//! around) and returns the pose it wants for the camera. [`drive`] picks the active
//! controller, glides between the old and the new pose when the mode changes and writes
//! the result to the camera. Each controller keeps its own state, so going back to a mode
//! finds it as it was left.
//!
//! This is also the seam for virtual reality: an XR camera is one more controller (its
//! pose comes from the headset instead of the mouse), added as a variant of
//! [`CameraKind`] and a field of [`CameraModes`]; nothing else in the viewer reads the
//! camera other than through its `Transform`. The walking mode already works at real
//! scale and eye height ([`cameras::EYE_HEIGHT`]). The pure maths is in
//! [`utsim_viewer::cameras`], the part that needs no window and is unit-tested.

use bevy::ecs::system::SystemParam;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_egui::{EguiContexts, egui};
use utsim_scene::MobileRole;
use utsim_viewer::cameras::{self, CameraKind, Obstacle, Pose};

use crate::camera::{CameraLock, OrbitCam};
use crate::dock::{Dock, pointer_over_ui};
use crate::editor::{Editor, Mode};
use crate::models::Realistic;
use crate::playback::{Data, Playback, Selection};

/// The vehicle the follow and ride cameras track, as it is drawn now.
#[derive(Clone, Copy, Debug)]
pub struct Tracked {
    /// Base of the vehicle on the floor.
    pub position: Vec3,
    /// Heading (see [`cameras::heading_dir`]).
    pub heading: f32,
    pub role: Option<MobileRole>,
}

/// Input of one frame, already filtered (nothing arrives while a panel has the pointer).
#[derive(Clone, Copy, Debug, Default)]
pub struct CamInput {
    /// Seconds since the last frame.
    pub dt: f32,
    /// The pointer is over a panel or window.
    pub over_ui: bool,
    /// Mouse movement this frame (pixels).
    pub motion: Vec2,
    /// Mouse wheel, in lines (positive: away from the user).
    pub scroll: f32,
    /// The point of the ground (Bevy space) under the cursor, when the cursor is over the
    /// 3D view and the ray meets the ground.
    pub cursor_ground: Option<Vec3>,
    /// Right button held, or the pointer is locked: the mouse looks around.
    pub look: bool,
    /// Left button held.
    pub drag: bool,
    /// A block is being dragged by the editor: the left button is not for the camera.
    pub locked: bool,
    /// Walking input, `-1..=1`: forward (W/S) and to the right (D/A).
    pub forward: f32,
    pub strafe: f32,
    /// Shift is held.
    pub run: bool,
}

/// What the scene tells a controller.
pub struct CamWorld<'a> {
    pub tracked: Option<Tracked>,
    pub obstacles: &'a [Obstacle],
    /// The point the orbit camera looks at (where a walk starts).
    pub orbit_target: Vec3,
}

/// A way of driving the camera. See the module docs.
pub trait CameraController {
    fn kind(&self) -> CameraKind;

    /// The mode just became active; `current` is the pose the camera has now.
    fn enter(&mut self, _current: &Transform, _world: &CamWorld) {}

    /// Advances the mode with this frame's input and returns the camera pose, or `None`
    /// to leave the camera where it is (nothing to look at).
    fn update(&mut self, input: &CamInput, world: &CamWorld) -> Option<Transform>;
}

fn pose_of(t: &Transform) -> Pose {
    let (yaw, pitch, _) = t.rotation.to_euler(EulerRot::YXZ);
    Pose {
        position: t.translation.to_array(),
        yaw,
        pitch,
    }
}

fn transform_of(p: Pose) -> Transform {
    Transform {
        translation: Vec3::from(p.position),
        rotation: Quat::from_euler(EulerRot::YXZ, p.yaw, p.pitch, 0.0),
        scale: Vec3::ONE,
    }
}

// ---------------------------------------------------------------------------------
// Orbit

impl CameraController for OrbitCam {
    fn kind(&self) -> CameraKind {
        CameraKind::Orbit
    }

    fn update(&mut self, input: &CamInput, _world: &CamWorld) -> Option<Transform> {
        if !input.over_ui {
            let d = input.motion;
            if input.look {
                self.yaw -= d.x * 0.006;
                self.pitch = (self.pitch + d.y * 0.006).clamp(0.05, 1.55);
            } else if input.drag && d != Vec2::ZERO && !input.locked {
                // Pan on the ground plane, relative to where the camera looks.
                let rot = Quat::from_rotation_y(self.yaw);
                let right = rot * Vec3::X;
                let forward = rot * Vec3::NEG_Z;
                let k = self.dist * 0.0014;
                self.target += (-right * d.x + forward * d.y) * k;
            }
            if input.scroll != 0.0 {
                self.zoom(input.scroll, input.cursor_ground);
            }
        }
        Some(self.transform())
    }
}

impl OrbitCam {
    /// Zooms by `scroll` lines (positive: closer). With a ground point under the cursor
    /// the view scales about that point, so it stays under the cursor; without one it
    /// zooms towards the centre of the view.
    pub fn zoom(&mut self, scroll: f32, toward: Option<Vec3>) {
        let dist = (self.dist * (1.0 - 0.1 * scroll)).clamp(1.5, 5000.0);
        if let Some(p) = toward {
            let k = dist / self.dist;
            self.target = p + (self.target - p) * k;
        }
        self.dist = dist;
    }
}

// ---------------------------------------------------------------------------------
// Follow

/// Follows a vehicle from behind and above.
pub struct FollowCam {
    /// Distance from the vehicle (metres); the wheel and the HUD slider change it.
    pub dist: f32,
    /// Elevation of the camera over the vehicle (radians).
    pub pitch: f32,
    /// Turn around the vehicle relative to "straight behind" (radians).
    pub yaw_offset: f32,
    smoothed: Vec3,
    heading: f32,
}

impl Default for FollowCam {
    fn default() -> Self {
        FollowCam {
            dist: 9.0,
            pitch: 0.5,
            yaw_offset: 0.0,
            smoothed: Vec3::ZERO,
            heading: 0.0,
        }
    }
}

impl CameraController for FollowCam {
    fn kind(&self) -> CameraKind {
        CameraKind::Follow
    }

    fn enter(&mut self, _current: &Transform, world: &CamWorld) {
        if let Some(t) = world.tracked {
            self.smoothed = t.position;
            self.heading = t.heading;
        }
    }

    fn update(&mut self, input: &CamInput, world: &CamWorld) -> Option<Transform> {
        let t = world.tracked?;
        if !input.over_ui {
            if input.look {
                self.yaw_offset -= input.motion.x * 0.006;
                self.pitch = (self.pitch + input.motion.y * 0.006).clamp(0.05, 1.45);
            }
            if input.scroll != 0.0 {
                self.dist = (self.dist * (1.0 - 0.1 * input.scroll)).clamp(2.0, 80.0);
            }
        }
        // The camera trails the vehicle: position quickly, heading slowly, so a turn
        // swings the view round instead of snapping it.
        let k = cameras::smoothing(7.0, input.dt);
        self.smoothed += (t.position - self.smoothed) * k;
        self.heading = cameras::smooth_angle(self.heading, t.heading, 2.5, input.dt);
        let (eye, focus) = cameras::follow_eye(
            self.smoothed.to_array(),
            self.heading,
            self.yaw_offset,
            self.pitch,
            self.dist,
        );
        Some(Transform::from_translation(Vec3::from(eye)).looking_at(Vec3::from(focus), Vec3::Y))
    }
}

// ---------------------------------------------------------------------------------
// First person

/// Walks through the plant at eye height.
pub struct WalkCam {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    /// Walking speed (m/s); the wheel changes it.
    pub speed: f32,
    started: bool,
}

impl Default for WalkCam {
    fn default() -> Self {
        WalkCam {
            position: Vec3::new(0.0, cameras::EYE_HEIGHT, 0.0),
            yaw: 0.0,
            pitch: -0.05,
            speed: cameras::WALK_SPEED,
            started: false,
        }
    }
}

impl CameraController for WalkCam {
    fn kind(&self) -> CameraKind {
        CameraKind::FirstPerson
    }

    fn enter(&mut self, current: &Transform, world: &CamWorld) {
        if !self.started {
            // The first walk starts on the ground half way between the orbit camera and the
            // point it looks at (the edge of the plant, on the side the view came from),
            // facing the same way.
            self.started = true;
            let mid = (current.translation + world.orbit_target) / 2.0;
            self.position = Vec3::new(mid.x, cameras::EYE_HEIGHT, mid.z);
            self.yaw = pose_of(current).yaw;
            self.pitch = -0.05;
        }
    }

    fn update(&mut self, input: &CamInput, world: &CamWorld) -> Option<Transform> {
        if input.look {
            self.yaw -= input.motion.x * 0.0025;
            self.pitch = (self.pitch - input.motion.y * 0.0025).clamp(-1.5, 1.5);
        }
        if !input.over_ui && input.scroll != 0.0 {
            self.speed = (self.speed * 1.15f32.powf(input.scroll))
                .clamp(cameras::WALK_SPEED_MIN, cameras::WALK_SPEED_MAX);
        }
        let speed = self.speed * if input.run { cameras::RUN_FACTOR } else { 1.0 };
        self.position = Vec3::from(cameras::walk_step(
            self.position.to_array(),
            self.yaw,
            input.forward,
            input.strafe,
            speed,
            input.dt,
            world.obstacles,
        ));
        Some(transform_of(Pose {
            position: self.position.to_array(),
            yaw: self.yaw,
            pitch: self.pitch,
        }))
    }
}

// ---------------------------------------------------------------------------------
// Ride-along

/// Rides along a vehicle at its driver position.
#[derive(Default)]
pub struct RideCam {
    /// Look around, relative to the heading (radians).
    pub look_yaw: f32,
    pub look_pitch: f32,
    heading: f32,
}

impl CameraController for RideCam {
    fn kind(&self) -> CameraKind {
        CameraKind::Ride
    }

    fn enter(&mut self, _current: &Transform, world: &CamWorld) {
        if let Some(t) = world.tracked {
            self.heading = t.heading;
        }
    }

    fn update(&mut self, input: &CamInput, world: &CamWorld) -> Option<Transform> {
        let t = world.tracked?;
        if input.look && !input.over_ui {
            self.look_yaw = (self.look_yaw - input.motion.x * 0.004).clamp(-2.6, 2.6);
            self.look_pitch = (self.look_pitch - input.motion.y * 0.004).clamp(-1.0, 1.2);
        }
        self.heading = cameras::smooth_angle(self.heading, t.heading, 10.0, input.dt);
        let eye = cameras::ride_eye(t.position.to_array(), t.heading, t.role);
        let (_, tilt) = cameras::driver_seat(t.role);
        Some(transform_of(Pose {
            position: eye,
            yaw: cameras::camera_yaw_of_heading(self.heading) + self.look_yaw,
            pitch: tilt + self.look_pitch,
        }))
    }
}

// ---------------------------------------------------------------------------------
// The resource and the system

/// A change of mode in progress: the camera glides from `from`.
struct Transition {
    from: Pose,
    elapsed: f32,
}

/// The active mode and the state of every mode.
#[derive(Resource)]
pub struct CameraModes {
    pub kind: CameraKind,
    pub follow: FollowCam,
    pub walk: WalkCam,
    pub ride: RideCam,
    /// The block the follow and ride cameras track.
    pub target: Option<usize>,
    /// Hold the pointer inside the window and look with it (first person, desktop only).
    pub pointer_lock: bool,
    /// A short message for the HUD (why a mode could not start).
    pub notice: Option<String>,
    notice_left: f32,
    request: Option<CameraKind>,
    request_target: Option<usize>,
    transition: Option<Transition>,
    last_selection: Option<usize>,
}

impl Default for CameraModes {
    fn default() -> Self {
        CameraModes {
            kind: CameraKind::Orbit,
            follow: FollowCam::default(),
            walk: WalkCam::default(),
            ride: RideCam::default(),
            target: None,
            pointer_lock: false,
            notice: None,
            notice_left: 0.0,
            request: None,
            request_target: None,
            transition: None,
            last_selection: None,
        }
    }
}

impl CameraModes {
    /// Asks for a mode; [`drive`] switches at the next frame (and finds the resource to
    /// follow when it needs one).
    pub fn request(&mut self, kind: CameraKind) {
        self.request = Some(kind);
    }

    /// Follows block `block` (a mobile resource) from behind.
    pub fn follow_block(&mut self, block: usize) {
        self.request_target = Some(block);
        self.request = Some(CameraKind::Follow);
    }

    /// Rides along block `block` at its driver position.
    pub fn ride_block(&mut self, block: usize) {
        self.request_target = Some(block);
        self.request = Some(CameraKind::Ride);
    }

    fn notify(&mut self, text: &str) {
        self.notice = Some(text.to_string());
        self.notice_left = 4.0;
    }

    /// Whether the editor may use the mouse (placing, dragging): not while walking.
    pub fn editing_allowed(&self) -> bool {
        self.kind != CameraKind::FirstPerson
    }
}

/// Run condition: the left button belongs to the editor.
pub fn editing_allowed(modes: Res<CameraModes>) -> bool {
    modes.editing_allowed()
}

/// The vehicle `block` as it is drawn now: the realistic model's own transform when it
/// has one (it includes the approach to a rack), else the scene pose.
fn drawn_vehicle(
    data: &Data,
    t: f64,
    block: usize,
    realistic: &Query<(&Realistic, &Transform), Without<Camera3d>>,
) -> Option<Tracked> {
    let pose = data.mobile_at(block, t)?;
    let mut tracked = Tracked {
        position: Vec3::from(pose.position),
        heading: pose.yaw,
        role: data.role(block),
    };
    if let Some((_, tf)) = realistic.iter().find(|(r, _)| r.block == block) {
        tracked.position = tf.translation;
        tracked.heading = tf.rotation.to_euler(EulerRot::YXZ).0;
    }
    Some(tracked)
}

/// Footprints the walker avoids, from the blocks shown now.
fn obstacles(editor: &Editor, data: &Data) -> Vec<Obstacle> {
    let mut out = Vec::new();
    let mut add = |position: [f32; 3], half: [f32; 3]| {
        // Flat marks on the floor (areas, painted lines) are walked over.
        if half[1] * 2.0 >= cameras::WALKABLE_HEIGHT {
            out.push(Obstacle::of_block(position, half));
        }
    };
    match editor.mode {
        Mode::Edit => {
            let v = &editor.view;
            for i in 0..v.positions.len() {
                if !matches!(v.kinds[i], "Transporter" | "Dispatcher" | "Flow" | "Fluid") {
                    add(v.positions[i], v.geoms[i].half);
                }
            }
        }
        Mode::Sim => {
            for i in 0..data.positions.len() {
                let kind = data.replay.blocks[i].kind.as_str();
                if !data.is_hidden(i)
                    && !data.timeline.is_mobile(i)
                    && !matches!(kind, "Flow" | "Fluid")
                {
                    add(data.positions[i], data.geoms[i].half);
                }
            }
        }
    }
    out
}

/// Mouse, keyboard and clock of the frame.
#[derive(SystemParam)]
pub struct Inputs<'w> {
    buttons: Res<'w, ButtonInput<MouseButton>>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    motion: Res<'w, AccumulatedMouseMotion>,
    scroll: Res<'w, AccumulatedMouseScroll>,
    time: Res<'w, Time<Real>>,
}

/// What the cameras look at: the document, the replay, the clock and the models.
#[derive(SystemParam)]
pub struct Scenery<'w, 's> {
    editor: Res<'w, Editor>,
    data: Res<'w, Data>,
    pb: Res<'w, Playback>,
    selection: Res<'w, Selection>,
    realistic: Query<'w, 's, (&'static Realistic, &'static Transform), Without<Camera3d>>,
}

/// The camera system: reads the input, advances the active controller and writes the
/// camera. Runs after the realistic models are animated, so a followed vehicle is where
/// it is drawn.
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn drive(
    mut modes: ResMut<CameraModes>,
    mut orbit: ResMut<OrbitCam>,
    lock: Res<CameraLock>,
    input: Inputs,
    dock: Res<Dock>,
    mut contexts: EguiContexts,
    scenery: Scenery,
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut transform: Single<&mut Transform, With<Camera3d>>,
    view: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let Inputs {
        buttons,
        keys,
        motion,
        scroll,
        time,
    } = input;
    let Scenery {
        editor,
        data,
        pb,
        selection,
        realistic,
    } = scenery;
    let (over_ui, typing) = contexts.ctx_mut().map_or((false, false), |ctx| {
        (pointer_over_ui(ctx, &dock), ctx.egui_wants_keyboard_input())
    });
    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let alt = keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);
    let sim = editor.mode == Mode::Sim;
    let dt = time.delta_secs().min(0.1);
    if modes.notice.is_some() {
        modes.notice_left -= dt;
        if modes.notice_left <= 0.0 {
            modes.notice = None;
        }
    }

    // Keys: 1-4 pick a mode, L holds the pointer, Esc steps back.
    if !typing && !ctrl && !alt {
        let digits = [
            (KeyCode::Digit1, CameraKind::Orbit),
            (KeyCode::Digit2, CameraKind::Follow),
            (KeyCode::Digit3, CameraKind::FirstPerson),
            (KeyCode::Digit4, CameraKind::Ride),
        ];
        for (key, kind) in digits {
            if keys.just_pressed(key) {
                modes.request(kind);
            }
        }
        if keys.just_pressed(KeyCode::KeyL) && modes.kind == CameraKind::FirstPerson {
            modes.pointer_lock = !modes.pointer_lock;
        }
        if keys.just_pressed(KeyCode::Escape) && modes.kind != CameraKind::Orbit {
            if modes.pointer_lock {
                modes.pointer_lock = false;
            } else {
                modes.request(CameraKind::Orbit);
            }
        }
    }

    // Following or riding needs a replay and a vehicle; leaving `Simular` ends them.
    if modes.kind.needs_resource() && !sim {
        modes.kind = CameraKind::Orbit;
        modes.transition = None;
    }

    // A pick of another vehicle retargets the follow and ride cameras.
    if selection.0 != modes.last_selection {
        modes.last_selection = selection.0;
        if let Some(b) = selection.0
            && modes.kind.needs_resource()
            && data.timeline.is_mobile(b)
        {
            modes.target = Some(b);
        }
    }

    if let Some(kind) = modes.request.take() {
        let resolved = if kind.needs_resource() {
            if !sim {
                modes.notify("Seguir e Do recurso só funcionam em Simular");
                None
            } else {
                let mobile = |b: &usize| data.timeline.is_mobile(*b) && !data.is_hidden(*b);
                let target = modes
                    .request_target
                    .filter(mobile)
                    .or(selection.0.filter(mobile))
                    .or(modes.target.filter(mobile))
                    .or((0..data.replay.blocks.len()).find(mobile));
                if target.is_none() {
                    modes.notify("O modelo não tem recursos móveis para seguir");
                }
                target
            }
        } else {
            Some(0)
        };
        modes.request_target = None;
        if let Some(target) = resolved
            && (kind != modes.kind || kind.needs_resource() && Some(target) != modes.target)
        {
            if kind.needs_resource() {
                modes.target = Some(target);
            }
            modes.notice = None;
            modes.transition = Some(Transition {
                from: pose_of(&transform),
                elapsed: 0.0,
            });
            modes.kind = kind;
            let tracked = modes
                .target
                .filter(|_| kind.needs_resource())
                .and_then(|b| drawn_vehicle(&data, pb.t, b, &realistic));
            let world = CamWorld {
                tracked,
                obstacles: &[],
                orbit_target: orbit.target,
            };
            let current = **transform;
            let m = &mut *modes;
            match kind {
                CameraKind::Orbit => {}
                CameraKind::Follow => m.follow.enter(&current, &world),
                CameraKind::FirstPerson => m.walk.enter(&current, &world),
                CameraKind::Ride => m.ride.enter(&current, &world),
            }
        }
    }

    // Pointer lock: only while walking, and never in the browser.
    let want_lock = cfg!(not(target_arch = "wasm32"))
        && modes.kind == CameraKind::FirstPerson
        && modes.pointer_lock;
    let (grab, visible) = if want_lock {
        (CursorGrabMode::Locked, false)
    } else {
        (CursorGrabMode::None, true)
    };
    if cursor.grab_mode != grab || cursor.visible != visible {
        cursor.grab_mode = grab;
        cursor.visible = visible;
    }

    // Input of the frame.
    let lines = match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / 40.0,
    };
    let walking = modes.kind == CameraKind::FirstPerson && !typing && !ctrl;
    let axis = |plus: KeyCode, minus: KeyCode| {
        (keys.pressed(plus) as i32 - keys.pressed(minus) as i32) as f32
    };
    let cam_input = CamInput {
        dt,
        over_ui: over_ui && !want_lock,
        motion: motion.delta,
        scroll: lines,
        cursor_ground: window
            .cursor_position()
            .and_then(|c| view.0.viewport_to_world(view.1, c).ok())
            .and_then(ground_point),
        look: buttons.pressed(MouseButton::Right) || want_lock,
        drag: buttons.pressed(MouseButton::Left),
        locked: lock.0,
        forward: if walking {
            axis(KeyCode::KeyW, KeyCode::KeyS)
        } else {
            0.0
        },
        strafe: if walking {
            axis(KeyCode::KeyD, KeyCode::KeyA)
        } else {
            0.0
        },
        run: keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]),
    };

    let tracked = modes
        .target
        .filter(|_| modes.kind.needs_resource())
        .and_then(|b| drawn_vehicle(&data, pb.t, b, &realistic));
    let walls = if modes.kind == CameraKind::FirstPerson {
        obstacles(&editor, &data)
    } else {
        Vec::new()
    };
    let world = CamWorld {
        tracked,
        obstacles: &walls,
        orbit_target: orbit.target,
    };
    let modes_kind = modes.kind;
    let controller: &mut dyn CameraController = match modes.kind {
        CameraKind::Orbit => &mut *orbit,
        CameraKind::Follow => &mut modes.follow,
        CameraKind::FirstPerson => &mut modes.walk,
        CameraKind::Ride => &mut modes.ride,
    };
    debug_assert_eq!(controller.kind(), modes_kind);
    let Some(wanted) = controller.update(&cam_input, &world) else {
        return;
    };

    // Glide from the old pose after a change of mode.
    let mut pose = pose_of(&wanted);
    if let Some(tr) = &mut modes.transition {
        tr.elapsed += dt;
        let k = cameras::blend_factor(tr.elapsed, cameras::BLEND_SECONDS);
        pose = cameras::blend_pose(tr.from, pose, k);
        if tr.elapsed >= cameras::BLEND_SECONDS {
            modes.transition = None;
        }
    }
    **transform = if modes.transition.is_some() {
        transform_of(pose)
    } else {
        wanted
    };
}

/// Where the ray meets the ground plane (`y = 0`), if it points down at it.
fn ground_point(ray: Ray3d) -> Option<Vec3> {
    let dir = *ray.direction;
    if dir.y > -1e-4 {
        return None;
    }
    let t = -ray.origin.y / dir.y;
    (t > 0.0).then(|| ray.origin + dir * t)
}

/// The "Câmera" menu of the toolbar.
pub fn selector(ui: &mut egui::Ui, modes: &mut CameraModes) {
    let shown = modes.kind;
    egui::ComboBox::from_id_salt("camera_modo")
        .selected_text(format!("Câmera: {}", shown.label()))
        .show_ui(ui, |ui| {
            for kind in CameraKind::ALL {
                let text = format!("{} ({})", kind.label(), kind.key());
                if ui.selectable_label(kind == shown, text).clicked() {
                    modes.request(kind);
                }
            }
        })
        .response
        .on_hover_text(
            "Órbita, seguir um recurso, andar pela planta ou ver do recurso (teclas 1 a 4)",
        );
}

/// A small panel over the top right corner of the 3D view while a mode other than the
/// orbit is active: what the mode is following, its controls and the way out.
pub fn hud(
    mut contexts: EguiContexts,
    mut modes: ResMut<CameraModes>,
    dock: Res<Dock>,
    data: Res<Data>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let kind = modes.kind;
    if kind == CameraKind::Orbit && modes.notice.is_none() {
        return Ok(());
    }
    let corner = dock.free.right_top() + egui::vec2(-10.0, 10.0);
    egui::Area::new(egui::Id::new("camera_hud"))
        .order(egui::Order::Foreground)
        .pivot(egui::Align2::RIGHT_TOP)
        .fixed_pos(corner)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_max_width(260.0);
                if let Some(notice) = modes.notice.clone() {
                    ui.colored_label(egui::Color32::from_rgb(255, 190, 90), notice);
                }
                if kind == CameraKind::Orbit {
                    return;
                }
                ui.strong(format!("Câmera: {}", kind.label()));
                if kind.needs_resource()
                    && let Some(b) = modes.target
                    && let Some(block) = data.replay.blocks.get(b)
                {
                    ui.label(format!("recurso: {}", block.name));
                }
                match kind {
                    CameraKind::Follow => {
                        ui.add(
                            egui::Slider::new(&mut modes.follow.dist, 2.0..=80.0)
                                .text("distância (m)")
                                .logarithmic(true),
                        );
                    }
                    CameraKind::FirstPerson => {
                        ui.add(
                            egui::Slider::new(
                                &mut modes.walk.speed,
                                cameras::WALK_SPEED_MIN..=cameras::WALK_SPEED_MAX,
                            )
                            .text("velocidade (m/s)")
                            .logarithmic(true),
                        );
                        #[cfg(not(target_arch = "wasm32"))]
                        ui.checkbox(&mut modes.pointer_lock, "Travar o mouse (L)");
                    }
                    _ => {}
                }
                ui.weak(kind.hint());
                if ui.button("Sair (Esc)").clicked() {
                    modes.request(CameraKind::Orbit);
                }
            });
        });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zooming_towards_a_point_keeps_it_on_the_same_line_of_sight() {
        let mut cam = OrbitCam {
            target: Vec3::new(2.0, 0.0, -3.0),
            yaw: 0.4,
            pitch: 0.8,
            dist: 30.0,
        };
        let point = Vec3::new(9.0, 0.0, 4.0);
        let before = cam.transform().translation;
        cam.zoom(3.0, Some(point));
        let after = cam.transform().translation;
        // The camera moved along the line towards the point: same direction from it.
        let (a, b) = ((before - point).normalize(), (after - point).normalize());
        assert!(a.distance(b) < 1e-4, "{a} {b}");
        assert!(after.distance(point) < before.distance(point));
        assert!((cam.dist - 30.0 * 0.7).abs() < 1e-3);
    }

    #[test]
    fn zooming_without_a_point_keeps_the_target() {
        let mut cam = OrbitCam {
            target: Vec3::new(1.0, 0.0, 1.0),
            yaw: 0.0,
            pitch: 0.7,
            dist: 20.0,
        };
        cam.zoom(-2.0, None);
        assert_eq!(cam.target, Vec3::new(1.0, 0.0, 1.0));
        assert!((cam.dist - 24.0).abs() < 1e-3);
    }
}
