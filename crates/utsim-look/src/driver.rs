//! The driver of a forklift: a seated, animated mannequin (the skeletal person of the
//! catalogue, [`crate::visual::PERSON`]) placed at the seat of the vehicle. Presentation
//! only: the simulation never reads it (the driver as a resource of the model is a later
//! step, see `docs/plano_analises_e_dinamica.md`, section 3).
//!
//! Two pure rules, shared by the interfaces:
//! - [`driver_clip`]: which clip to play from the motion of the vehicle (`Driving_Loop`
//!   while it moves or handles a load, `Sitting_Idle_Loop` otherwise);
//! - [`driver_mount`]: where the mannequin goes in the frame of the vehicle's root (its
//!   origin, yaw and scale) so that the hip lands on the seat, plus the eye position for
//!   the "from the resource" camera.
//!
//! The seat of each asset is a number here (library) or in the catalogue entry
//! (`motorista`, see [`crate::catalogue::Motorista`]), in the frame of the placed
//! vehicle: `x` forward, `y` up, `z` to the right, metres, origin on the floor under the
//! centre of the footprint, real size.

use utsim_assets::Spec;
use utsim_scene::{MobileRole, Motion};

use crate::catalogue;
use crate::visual::{Clip, Look, PERSON, Source};

/// Clips of the seated driver, in the file of [`PERSON`] (the Universal Animation
/// Library). They come after [`Clip::ALL`] in the clip list the Godot bridge hands out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DriverClip {
    /// Hands on the wheel (`Driving_Loop`).
    Drive,
    /// Seated at rest (`Sitting_Idle_Loop`).
    Sit,
}

impl DriverClip {
    pub const ALL: [DriverClip; 2] = [DriverClip::Drive, DriverClip::Sit];

    /// Name of the glTF animation (always in the [`PERSON`] file).
    pub fn source(self) -> &'static str {
        match self {
            DriverClip::Drive => "Driving_Loop",
            DriverClip::Sit => "Sitting_Idle_Loop",
        }
    }

    /// Position among the driver clips.
    pub fn index(self) -> usize {
        DriverClip::ALL.iter().position(|c| *c == self).unwrap_or(0)
    }

    /// Index in the complete clip list (the person clips, then these).
    pub fn global_index(self) -> usize {
        Clip::ALL.len() + self.index()
    }
}

/// Which clip the driver plays: driving while the vehicle moves or loads or unloads
/// (hands on the controls, the forks move with the pose's lift), seated at rest while it
/// is idle, blocked or stopped (also working, which a vehicle does not do).
pub fn driver_clip(motion: Motion) -> DriverClip {
    match motion {
        Motion::Moving { .. } | Motion::Loading { .. } | Motion::Unloading { .. } => {
            DriverClip::Drive
        }
        _ => DriverClip::Sit,
    }
}

/// Hip joint (`pelvis` bone) of the mannequin in the `Driving_Loop` and `Sitting_Idle_Loop`
/// clips, relative to its root, file units (`x` lateral, `y` up, `z` forward). Measured
/// from the clips: the two differ by 2 cm.
pub const MANNEQUIN_HIP: [f32; 3] = [0.003, 0.54, -0.32];
/// Eyes above the hip joint, seated, real metres; and ahead of it.
pub const EYE_ABOVE_HIP: f32 = 0.70;
pub const EYE_AHEAD_OF_HIP: f32 = 0.06;

/// Seat of the library forklift (`utsim_assets::vehicles::empilhadeira`): the hip joint
/// of a driver sitting on the cushion (x -0.62 to -0.18, top at 1.06 m), far enough
/// forward that the hands reach the steering wheel (centre x 0.22, 1.39 m high).
pub const FORKLIFT_HIP: [f32; 3] = [-0.34, 1.07, 0.0];

/// Where to put the mannequin inside the root of a vehicle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DriverMount {
    /// Position of the mannequin's root (its origin, at the feet in the rest pose) in the
    /// vehicle root's frame, metres.
    pub position: [f32; 3],
    /// Rotation about the vertical, radians (the mannequin faces `+z` in its file).
    pub yaw: f32,
    /// Uniform scale of the mannequin's scene.
    pub scale: f32,
    /// Eyes (the camera of the driver's view), in the vehicle root's frame.
    pub eye: [f32; 3],
    /// The hip joint on the seat, in the vehicle root's frame.
    pub hip: [f32; 3],
}

/// Rotation about the vertical by `yaw`: the convention of the engines (`+z` turns
/// towards `+x`).
fn turn(v: [f32; 3], yaw: f32) -> [f32; 3] {
    let (s, c) = yaw.sin_cos();
    [v[0] * c + v[2] * s, v[1], -v[0] * s + v[2] * c]
}

/// The seat of the vehicle drawn with `look`, in the placed frame, and the extra yaw of
/// the mannequin: `None` for the assets without a seat.
fn seat_of(look: &Look) -> Option<([f32; 3], f32, f32, f32, f32)> {
    // (hip, mannequin yaw relative to the vehicle's front, user scale, user yaw, the
    // driver's size relative to the catalogue's mannequin)
    match &look.source {
        Source::Library(Spec::Empilhadeira) => Some((FORKLIFT_HIP, 0.0, look.scale, look.yaw, 1.0)),
        Source::Catalogue { name, .. } => {
            let entry = catalogue::by_name(name)?;
            let m = entry.motorista.as_ref()?;
            let user_scale = look.scale / entry.escala_real;
            let user_yaw = look.yaw - entry.giro_y_graus.to_radians();
            Some((
                [m.quadril_m.0, m.quadril_m.1, m.quadril_m.2],
                m.giro_graus.to_radians(),
                user_scale,
                user_yaw,
                m.escala,
            ))
        }
        _ => None,
    }
}

/// The mount of the driver of a forklift drawn with `look`, or `None` (other roles, or an
/// asset without a seat: the AGV, the pallet jack, a file the catalogue does not know).
pub fn driver_mount(role: Option<MobileRole>, look: &Look) -> Option<DriverMount> {
    if role != Some(MobileRole::Forklift) || look.skeletal {
        return None;
    }
    let (seat, turn_to, user_scale, user_yaw, size) = seat_of(look)?;
    let person = catalogue::by_name(PERSON);
    let (person_scale, person_turn) =
        person.map_or((0.96, 90.0), |p| (p.escala_real, p.giro_y_graus));
    let scale = person_scale * user_scale * size;
    // The mannequin's file front (+z) goes to the vehicle's front (+x) by the catalogue
    // turn; the vehicle's own turn and the entry's adjustment come on top.
    let yaw = person_turn.to_radians() + turn_to + user_yaw;
    let hip = turn(
        [
            seat[0] * user_scale,
            seat[1] * user_scale,
            seat[2] * user_scale,
        ],
        user_yaw,
    );
    // Root of the mannequin = hip on the seat minus the hip's offset from the root.
    let offset = turn(
        [
            MANNEQUIN_HIP[0] * scale,
            MANNEQUIN_HIP[1] * scale,
            MANNEQUIN_HIP[2] * scale,
        ],
        yaw,
    );
    let position = [hip[0] - offset[0], hip[1] - offset[1], hip[2] - offset[2]];
    let ahead = turn([EYE_AHEAD_OF_HIP * user_scale, 0.0, 0.0], user_yaw);
    let eye = [
        hip[0] + ahead[0],
        hip[1] + EYE_ABOVE_HIP * user_scale * size,
        hip[2] + ahead[2],
    ];
    Some(DriverMount {
        position,
        yaw,
        scale,
        eye,
        hip,
    })
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;
    use crate::visual::{Context, look_of};
    use std::f32::consts::FRAC_PI_2;
    use utsim_model::VisualDef;
    use utsim_scene::geometry::Geom;

    fn geom() -> Geom {
        Geom::new("Transporter", None, None)
    }

    fn look(role: MobileRole, asset: Option<&str>, scale: f64, yaw: f64) -> Look {
        let visual = VisualDef {
            asset: asset.map(str::to_string),
            scale: Some(scale),
            yaw: Some(yaw),
            ..VisualDef::default()
        };
        look_of(
            "Transporter",
            Some(role),
            Some(&visual),
            &geom(),
            Context {
                pallets: true,
                skeletal_people: true,
            },
            None,
        )
        .expect("look")
    }

    /// World position of the mannequin's hip joint for a mount.
    fn hip_of(m: &DriverMount) -> [f32; 3] {
        let o = turn(
            [
                MANNEQUIN_HIP[0] * m.scale,
                MANNEQUIN_HIP[1] * m.scale,
                MANNEQUIN_HIP[2] * m.scale,
            ],
            m.yaw,
        );
        [
            m.position[0] + o[0],
            m.position[1] + o[1],
            m.position[2] + o[2],
        ]
    }

    #[test]
    fn the_clip_follows_the_motion_of_the_vehicle() {
        assert_eq!(
            driver_clip(Motion::Moving { speed: 2.0 }),
            DriverClip::Drive
        );
        assert_eq!(
            driver_clip(Motion::Loading { progress: 0.3 }),
            DriverClip::Drive
        );
        assert_eq!(
            driver_clip(Motion::Unloading { progress: 0.9 }),
            DriverClip::Drive
        );
        for m in [
            Motion::Idle,
            Motion::Blocked,
            Motion::Stopped,
            Motion::Working,
        ] {
            assert_eq!(driver_clip(m), DriverClip::Sit, "{m:?}");
        }
        assert_eq!(DriverClip::Drive.source(), "Driving_Loop");
        assert_eq!(DriverClip::Sit.source(), "Sitting_Idle_Loop");
        // They come after the seven person clips.
        assert_eq!(DriverClip::Drive.global_index(), Clip::ALL.len());
        assert_eq!(DriverClip::Sit.global_index(), Clip::ALL.len() + 1);
    }

    #[test]
    fn every_driver_clip_exists_in_the_person_file() {
        let p = catalogue::by_name(PERSON).expect("pessoa_operador");
        for c in DriverClip::ALL {
            assert!(p.clipes.iter().any(|n| n == c.source()), "{}", c.source());
        }
    }

    #[test]
    fn the_hip_lands_on_the_seat_of_the_library_forklift() {
        let l = look(MobileRole::Forklift, Some("empilhadeira"), 1.0, 0.0);
        let m = driver_mount(Some(MobileRole::Forklift), &l).expect("mount");
        let hip = hip_of(&m);
        for k in 0..3 {
            assert!((hip[k] - FORKLIFT_HIP[k]).abs() < 1e-4, "{hip:?}");
            assert!((m.hip[k] - FORKLIFT_HIP[k]).abs() < 1e-4);
        }
        // Facing the front of the vehicle (+x): the mannequin's +z is turned a quarter.
        assert!((m.yaw - FRAC_PI_2).abs() < 1e-5);
        assert!((m.scale - 0.96).abs() < 1e-5);
        // The seat is in the cushion's range (x -0.62 to -0.18, top at 1.06 m).
        assert!(m.hip[0] > -0.62 && m.hip[0] < -0.18 && m.hip[1] > 1.06 && m.hip[1] < 1.2);
        // The eyes are above the hip, inside the overhead guard (2.12 m).
        assert!(m.eye[1] > m.hip[1] + 0.5 && m.eye[1] < 2.12, "{:?}", m.eye);
        assert!(m.eye[0] > m.hip[0]);
    }

    #[test]
    fn the_user_scale_and_yaw_carry_the_driver_with_the_vehicle() {
        let base = driver_mount(
            Some(MobileRole::Forklift),
            &look(MobileRole::Forklift, Some("empilhadeira"), 1.0, 0.0),
        )
        .unwrap();
        let big = driver_mount(
            Some(MobileRole::Forklift),
            &look(MobileRole::Forklift, Some("empilhadeira"), 2.0, 0.0),
        )
        .unwrap();
        assert!((big.scale - 2.0 * base.scale).abs() < 1e-5);
        for k in 0..3 {
            assert!((big.hip[k] - 2.0 * base.hip[k]).abs() < 1e-4);
        }
        // A quarter turn of the vehicle moves the seat from -x to +z... of the root.
        let q = driver_mount(
            Some(MobileRole::Forklift),
            &look(MobileRole::Forklift, Some("empilhadeira"), 1.0, 90.0),
        )
        .unwrap();
        let (s, c) = FRAC_PI_2.sin_cos();
        assert!((q.hip[0] - (base.hip[0] * c + base.hip[2] * s)).abs() < 1e-4);
        assert!((q.hip[2] - (-base.hip[0] * s + base.hip[2] * c)).abs() < 1e-4);
        assert!((q.yaw - base.yaw - FRAC_PI_2).abs() < 1e-5);
        // The mannequin's hip is still on the seat.
        let hip = hip_of(&q);
        for k in 0..3 {
            assert!((hip[k] - q.hip[k]).abs() < 1e-4);
        }
    }

    #[test]
    fn the_catalogue_forklift_takes_its_seat_from_the_catalogue() {
        let l = look(
            MobileRole::Forklift,
            Some("empilhadeira_kolosstudios"),
            1.0,
            0.0,
        );
        let m = driver_mount(Some(MobileRole::Forklift), &l).expect("mount");
        let entry = catalogue::by_name("empilhadeira_kolosstudios").unwrap();
        let q = entry
            .motorista
            .as_ref()
            .expect("motorista no catálogo")
            .quadril_m;
        // A smaller driver for a smaller cab.
        assert!((m.scale - 0.96 * entry.motorista.as_ref().unwrap().escala).abs() < 1e-5);
        let hip = hip_of(&m);
        assert!((hip[0] - q.0).abs() < 1e-4 && (hip[1] - q.1).abs() < 1e-4);
        assert!((hip[2] - q.2).abs() < 1e-4);
        // Inside the footprint of the vehicle (x lengthwise, z across).
        let size = entry.real_size();
        assert!(hip[0].abs() < size[0] / 2.0 && hip[2].abs() < size[2] / 2.0);
        assert!(hip[1] > 0.3 && hip[1] < size[1]);
    }

    #[test]
    fn only_forklifts_with_a_seat_get_a_driver() {
        let jack = look(MobileRole::Vehicle, Some("paleteira"), 1.0, 0.0);
        assert!(driver_mount(Some(MobileRole::Vehicle), &jack).is_none());
        let agv = look(MobileRole::Agv, Some("agv"), 1.0, 0.0);
        assert!(driver_mount(Some(MobileRole::Agv), &agv).is_none());
        let person = look(MobileRole::Person, None, 1.0, 0.0);
        assert!(driver_mount(Some(MobileRole::Person), &person).is_none());
        // A forklift role with a catalogue asset that has no seat entry.
        let other = look(MobileRole::Forklift, Some("caixote_quaternius"), 1.0, 0.0);
        assert!(driver_mount(Some(MobileRole::Forklift), &other).is_none());
        // A role that is not a forklift never gets one, whatever the asset.
        let fork = look(MobileRole::Forklift, Some("empilhadeira"), 1.0, 0.0);
        assert!(driver_mount(Some(MobileRole::Person), &fork).is_none());
        assert!(driver_mount(None, &fork).is_none());
    }
}
