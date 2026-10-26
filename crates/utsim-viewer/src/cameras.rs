//! Pure rules of the camera modes (no window, no GPU): which modes exist, the maths of
//! following a vehicle, of the driver's seat and of walking through the plant with
//! simple collision, and the blend between two poses.
//!
//! The Bevy side (`camera_modes.rs` of the `utsim-view` binary) feeds these functions
//! with the input of the frame and writes the result to the camera. Vectors are plain
//! `[f32; 3]` in Bevy space (Y up, metres), like [`crate::view`]. A model's heading
//! `yaw` is the rotation about Y that takes local `+X` (the front of a vehicle) to the
//! horizontal direction `(cos yaw, 0, -sin yaw)`.

use utsim_scene::MobileRole;

use crate::view::V3;

/// Eye height of a standing person, in metres. The walking camera stays at it: real
/// scale and eye height are what a headset needs later (docs/arquitetura.md, 4p).
pub const EYE_HEIGHT: f32 = 1.6;

/// Radius of the walker's body against the footprints of the blocks.
pub const WALK_RADIUS: f32 = 0.3;

/// Walking speed (m/s) a first-person walk starts with, and its range.
pub const WALK_SPEED: f32 = 3.0;
pub const WALK_SPEED_MIN: f32 = 0.5;
pub const WALK_SPEED_MAX: f32 = 30.0;

/// Running multiplies the walking speed by this.
pub const RUN_FACTOR: f32 = 2.5;

/// Seconds a change of mode takes to glide from the old pose to the new one.
pub const BLEND_SECONDS: f32 = 0.7;

/// The camera modes of the viewer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CameraKind {
    /// Orbit, pan and zoom around a point (the default).
    #[default]
    Orbit,
    /// Follows a mobile resource from behind and above.
    Follow,
    /// Walks through the plant at eye height.
    FirstPerson,
    /// Rides along a vehicle at its driver position.
    Ride,
}

impl CameraKind {
    pub const ALL: [CameraKind; 4] = [
        CameraKind::Orbit,
        CameraKind::Follow,
        CameraKind::FirstPerson,
        CameraKind::Ride,
    ];

    /// Name shown in the toolbar menu and used by scripts.
    pub fn label(self) -> &'static str {
        match self {
            CameraKind::Orbit => "Órbita",
            CameraKind::Follow => "Seguir",
            CameraKind::FirstPerson => "Primeira pessoa",
            CameraKind::Ride => "Do recurso",
        }
    }

    /// Key that selects the mode (also in the shortcuts window).
    pub fn key(self) -> &'static str {
        match self {
            CameraKind::Orbit => "1",
            CameraKind::Follow => "2",
            CameraKind::FirstPerson => "3",
            CameraKind::Ride => "4",
        }
    }

    /// The mode follows or rides a mobile resource.
    pub fn needs_resource(self) -> bool {
        matches!(self, CameraKind::Follow | CameraKind::Ride)
    }

    /// One line on how to drive the mode.
    pub fn hint(self) -> &'static str {
        match self {
            CameraKind::Orbit => "esquerdo arrasta (desloca), direito gira, roda aproxima",
            CameraKind::Follow => {
                "direito gira em volta do recurso, roda muda a distância, Esc sai"
            }
            CameraKind::FirstPerson => {
                "WASD anda, Shift corre, direito (ou L) olha, roda muda a velocidade, Esc sai"
            }
            CameraKind::Ride => "direito olha em volta, Esc sai",
        }
    }

    /// Parses a label (any case, with or without accents) or a key.
    pub fn parse(text: &str) -> Option<CameraKind> {
        let t: String = text
            .trim()
            .to_lowercase()
            .chars()
            .map(|c| match c {
                'ó' | 'ô' | 'õ' => 'o',
                'á' | 'â' | 'ã' => 'a',
                'é' | 'ê' => 'e',
                'í' => 'i',
                'ú' => 'u',
                ' ' | '_' | '-' => ' ',
                c => c,
            })
            .collect();
        match t.as_str() {
            "orbita" | "orbit" | "1" => Some(CameraKind::Orbit),
            "seguir" | "follow" | "2" => Some(CameraKind::Follow),
            "primeira pessoa" | "primeirapessoa" | "firstperson" | "first person" | "3" => {
                Some(CameraKind::FirstPerson)
            }
            "do recurso" | "dorecurso" | "recurso" | "ride" | "4" => Some(CameraKind::Ride),
            _ => None,
        }
    }
}

/// Position and orientation of the camera. `yaw` turns about Y (0 looks along `-Z`),
/// `pitch` looks up when positive: the rotation is `Quat::from_euler(YXZ, yaw, pitch, 0)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub position: V3,
    pub yaw: f32,
    pub pitch: f32,
}

/// Horizontal direction of a camera `yaw`: where `-Z` goes.
pub fn look_dir(yaw: f32) -> [f32; 2] {
    [-yaw.sin(), -yaw.cos()]
}

/// Horizontal direction a model with heading `yaw` is facing (its local `+X`).
pub fn heading_dir(yaw: f32) -> [f32; 2] {
    [yaw.cos(), -yaw.sin()]
}

/// Camera `yaw` that looks along the heading of a model.
pub fn camera_yaw_of_heading(heading: f32) -> f32 {
    heading - std::f32::consts::FRAC_PI_2
}

/// Wraps an angle to `-PI..=PI`.
pub fn wrap_angle(a: f32) -> f32 {
    let tau = std::f32::consts::TAU;
    let r = a.rem_euclid(tau);
    if r > std::f32::consts::PI { r - tau } else { r }
}

/// Fraction of the way to a target after `dt` seconds of exponential smoothing with
/// the given `rate` (per second): frame-rate independent.
pub fn smoothing(rate: f32, dt: f32) -> f32 {
    1.0 - (-rate * dt).exp()
}

/// Moves `current` towards `target` by exponential smoothing.
pub fn smooth(current: f32, target: f32, rate: f32, dt: f32) -> f32 {
    current + (target - current) * smoothing(rate, dt)
}

/// [`smooth`] for an angle, along the shortest way round.
pub fn smooth_angle(current: f32, target: f32, rate: f32, dt: f32) -> f32 {
    current + wrap_angle(target - current) * smoothing(rate, dt)
}

/// Height above a vehicle's base that the follow camera looks at.
pub const FOLLOW_FOCUS_HEIGHT: f32 = 1.0;

/// Follow camera: sits `dist` metres from the focus, behind the heading `heading`
/// (turned by `yaw_offset`) and raised by `pitch`. Returns `(eye, focus)`.
pub fn follow_eye(target: V3, heading: f32, yaw_offset: f32, pitch: f32, dist: f32) -> (V3, V3) {
    let focus = [target[0], target[1] + FOLLOW_FOCUS_HEIGHT, target[2]];
    let d = heading_dir(heading + yaw_offset);
    let (c, s) = (pitch.cos(), pitch.sin());
    let eye = [
        focus[0] - d[0] * dist * c,
        focus[1] + dist * s,
        focus[2] - d[1] * dist * c,
    ];
    (eye, focus)
}

/// Driver position relative to the vehicle's base (`x` forward, `y` up, `z` to the
/// right), and the downward tilt of the default view, by role.
pub fn driver_seat(role: Option<MobileRole>) -> (V3, f32) {
    match role {
        // Operator's seat under the overhead guard (the library model's seat cushion is
        // at x -0.62..-0.18, 1.06 m high), looking over the hood at the forks.
        Some(MobileRole::Forklift) => ([-0.40, 1.72, 0.0], -0.12),
        // A pedestrian: eyes at 1.6 m, a little ahead of the body centre.
        Some(MobileRole::Person) => ([0.12, EYE_HEIGHT, 0.0], -0.05),
        // An AGV has no driver: a point above the front of the deck.
        Some(MobileRole::Agv) => ([0.35, 1.1, 0.0], -0.22),
        Some(MobileRole::Vehicle) | None => ([0.0, 1.0, 0.0], -0.15),
    }
}

/// Eye of the ride-along camera for a vehicle at `position` with heading `heading`.
pub fn ride_eye(position: V3, heading: f32, role: Option<MobileRole>) -> V3 {
    let (seat, _) = driver_seat(role);
    let f = heading_dir(heading);
    // Right of the heading (looking along it, Y up).
    let right = [-f[1], f[0]];
    [
        position[0] + f[0] * seat[0] + right[0] * seat[2],
        position[1] + seat[1],
        position[2] + f[1] * seat[0] + right[1] * seat[2],
    ]
}

/// Footprint of a block on the ground (axis-aligned, metres) the walker cannot enter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obstacle {
    pub min: [f32; 2],
    pub max: [f32; 2],
}

impl Obstacle {
    /// The footprint of a block with base `position` and half extents `half`.
    pub fn of_block(position: V3, half: V3) -> Self {
        Obstacle {
            min: [position[0] - half[0], position[2] - half[2]],
            max: [position[0] + half[0], position[2] + half[2]],
        }
    }

    /// Whether a disc of radius `r` at `(x, z)` overlaps the footprint.
    pub fn hits(&self, x: f32, z: f32, r: f32) -> bool {
        x > self.min[0] - r && x < self.max[0] + r && z > self.min[1] - r && z < self.max[1] + r
    }
}

/// Blocks flatter than this (floor marks, painted areas) can be walked over.
pub const WALKABLE_HEIGHT: f32 = 0.2;

fn blocked(obstacles: &[Obstacle], x: f32, z: f32, from: (f32, f32)) -> bool {
    obstacles
        .iter()
        // Already inside (placed there, or the model changed): never trap the walker.
        .filter(|o| !o.hits(from.0, from.1, WALK_RADIUS))
        .any(|o| o.hits(x, z, WALK_RADIUS))
}

/// One walking step: `forward` and `strafe` in `-1..=1` (forward along the look
/// direction, strafe to the right), `speed` in m/s. The walker slides along a footprint
/// instead of stopping at it. The result stays at [`EYE_HEIGHT`].
pub fn walk_step(
    position: V3,
    yaw: f32,
    forward: f32,
    strafe: f32,
    speed: f32,
    dt: f32,
    obstacles: &[Obstacle],
) -> V3 {
    let f = look_dir(yaw);
    let right = [-f[1], f[0]];
    let mut dx = f[0] * forward + right[0] * strafe;
    let mut dz = f[1] * forward + right[1] * strafe;
    let len = (dx * dx + dz * dz).sqrt();
    if len > 1.0 {
        dx /= len;
        dz /= len;
    }
    let step = speed * dt;
    let from = (position[0], position[2]);
    let (mut x, mut z) = from;
    // Axes one at a time, so a wall slides instead of sticking.
    if !blocked(obstacles, x + dx * step, z, from) {
        x += dx * step;
    }
    if !blocked(obstacles, x, z + dz * step, from) {
        z += dz * step;
    }
    [x, EYE_HEIGHT, z]
}

/// Smooth easing `0..=1` of a blend that has run for `elapsed` of `duration` seconds.
pub fn blend_factor(elapsed: f32, duration: f32) -> f32 {
    let t = (elapsed / duration.max(1e-3)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Blend of two poses (`k` in `0..=1`; the angles take the shortest way).
pub fn blend_pose(a: Pose, b: Pose, k: f32) -> Pose {
    let l = |x: f32, y: f32| x + (y - x) * k;
    Pose {
        position: [
            l(a.position[0], b.position[0]),
            l(a.position[1], b.position[1]),
            l(a.position[2], b.position[2]),
        ],
        yaw: a.yaw + wrap_angle(b.yaw - a.yaw) * k,
        pitch: l(a.pitch, b.pitch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn labels_and_keys_round_trip() {
        for kind in CameraKind::ALL {
            assert_eq!(CameraKind::parse(kind.label()), Some(kind));
            assert_eq!(CameraKind::parse(kind.key()), Some(kind));
        }
        assert_eq!(CameraKind::parse("orbita"), Some(CameraKind::Orbit));
        assert_eq!(
            CameraKind::parse("Primeira_pessoa"),
            Some(CameraKind::FirstPerson)
        );
        assert_eq!(CameraKind::parse("voo"), None);
    }

    #[test]
    fn heading_and_camera_yaw_agree() {
        for heading in [0.0f32, 0.7, 2.0, -1.3, 3.0] {
            let h = heading_dir(heading);
            let c = look_dir(camera_yaw_of_heading(heading));
            assert!(near(h[0], c[0]) && near(h[1], c[1]), "{heading}");
        }
    }

    #[test]
    fn follow_camera_sits_behind_and_above() {
        // Heading +X: behind is -X.
        let (eye, focus) = follow_eye([10.0, 0.0, 5.0], 0.0, 0.0, 0.5, 8.0);
        assert!(eye[0] < focus[0] - 6.0);
        assert!(near(eye[2], 5.0));
        assert!(eye[1] > focus[1] + 3.0);
        let d = ((eye[0] - focus[0]).powi(2) + (eye[1] - focus[1]).powi(2)).sqrt();
        assert!(near(d, 8.0));
    }

    #[test]
    fn ride_eye_follows_the_heading() {
        let p = ride_eye([0.0, 0.0, 0.0], 0.0, Some(MobileRole::Forklift));
        assert!(near(p[0], -0.40) && near(p[1], 1.72));
        // Turned a quarter (+Y of the model is -Z in Bevy): the seat moves with it.
        let q = ride_eye(
            [0.0, 0.0, 0.0],
            std::f32::consts::FRAC_PI_2,
            Some(MobileRole::Forklift),
        );
        assert!(near(q[0], 0.0) && near(q[2], 0.40));
    }

    #[test]
    fn walking_forward_goes_along_the_look_direction() {
        // yaw 0 looks along -Z.
        let p = walk_step([0.0, 9.0, 0.0], 0.0, 1.0, 0.0, 2.0, 1.0, &[]);
        assert!(near(p[0], 0.0) && near(p[2], -2.0) && near(p[1], EYE_HEIGHT));
        // Strafing right at yaw 0 goes along +X.
        let q = walk_step([0.0, 0.0, 0.0], 0.0, 0.0, 1.0, 2.0, 1.0, &[]);
        assert!(near(q[0], 2.0) && near(q[2], 0.0));
        // Diagonals are not faster.
        let d = walk_step([0.0, 0.0, 0.0], 0.0, 1.0, 1.0, 1.0, 1.0, &[]);
        assert!(near((d[0] * d[0] + d[2] * d[2]).sqrt(), 1.0));
    }

    #[test]
    fn a_footprint_blocks_and_the_walker_slides() {
        // A wall across the way, x in -5..5, z in -3..-2.
        let wall = Obstacle {
            min: [-5.0, -3.0],
            max: [5.0, -2.0],
        };
        // Walking diagonally into it: the z move is refused, the x move slides.
        let p = walk_step([0.0, 0.0, -1.5], 0.0, 1.0, 1.0, 1.0, 0.5, &[wall]);
        assert!(p[0] > 0.0);
        assert!(p[2] >= -1.5 - 1e-4, "{p:?}");
        // Straight into it: stops short of the radius.
        let mut q = [0.0, 0.0, 0.0];
        for _ in 0..200 {
            q = walk_step(q, 0.0, 1.0, 0.0, 3.0, 0.05, &[wall]);
        }
        assert!(q[2] > -2.0 + WALK_RADIUS - 0.2, "{q:?}");
    }

    #[test]
    fn a_walker_inside_a_footprint_can_leave() {
        let block = Obstacle::of_block([0.0, 0.0, 0.0], [2.0, 1.0, 2.0]);
        let p = walk_step([0.0, 0.0, 0.0], 0.0, 1.0, 0.0, 1.0, 1.0, &[block]);
        assert!(p[2] < 0.0, "stuck: {p:?}");
    }

    #[test]
    fn angle_smoothing_takes_the_short_way() {
        let a = smooth_angle(3.0, -3.0, 1000.0, 1.0);
        // From 3.0 to -3.0 the short way crosses PI (about 0.28 rad), not zero.
        assert!(a.abs() > 3.0 || wrap_angle(a + 3.0).abs() < 0.01, "{a}");
        assert!(near(wrap_angle(7.0), 7.0 - std::f32::consts::TAU));
    }

    #[test]
    fn blend_runs_from_zero_to_one() {
        assert_eq!(blend_factor(0.0, 1.0), 0.0);
        assert_eq!(blend_factor(1.0, 1.0), 1.0);
        assert!(blend_factor(0.5, 1.0) > 0.4 && blend_factor(0.5, 1.0) < 0.6);
        let a = Pose {
            position: [0.0; 3],
            yaw: 3.0,
            pitch: 0.0,
        };
        let b = Pose {
            position: [2.0, 0.0, 0.0],
            yaw: -3.0,
            pitch: 0.2,
        };
        let m = blend_pose(a, b, 0.5);
        assert!(near(m.position[0], 1.0));
        assert!(m.yaw.abs() > 3.0 - 1.0);
    }
}
