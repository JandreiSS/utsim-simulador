//! Coordinate conversion between the scene contract and Godot.
//!
//! The scene (`utsim-scene`) gives positions in the model's plan coordinates: metres,
//! right-handed, **Z up**, yaw counter-clockwise seen from above with `0` facing `+x`.
//! Godot is right-handed and **Y up**, like Bevy, so the mapping is the same as the
//! Bevy viewer's: `(x, y, z)` becomes `(x, z, -y)`, and a yaw is a rotation of the same
//! angle about `+Y` (a heading `(cos, sin, 0)` becomes `(cos, 0, -sin)`, which is
//! `Basis(Vector3.UP, yaw) * Vector3.RIGHT`).
//!
//! The scene's internal *layout space* (`utsim_scene::layout::to_bevy`) is that same
//! Y-up space, so values the bridge computes there (the realistic placement of carried
//! loads, ported from the Bevy viewer) are already Godot coordinates: see
//! [`layout_to_godot`].

/// Model point `(x, y, z)`, Z up, to Godot `(x, z, -y)`, Y up.
pub fn to_godot(p: [f32; 3]) -> [f32; 3] {
    [p[0], p[2], -p[1]]
}

/// Godot point back to model coordinates (the exact inverse of [`to_godot`]).
pub fn from_godot(p: [f32; 3]) -> [f32; 3] {
    [p[0], -p[2], p[1]]
}

/// A point of the scene's layout space in Godot coordinates: the identity, since both are
/// the model point mapped by `(x, z, -y)`.
pub fn layout_to_godot(p: [f32; 3]) -> [f32; 3] {
    p
}

/// Model yaw (radians, counter-clockwise from above) as the rotation about Godot's `+Y`.
pub fn yaw_to_godot(yaw: f32) -> f32 {
    yaw
}

/// Size `(length x, width y, height z)` of a model footprint as Godot extents
/// `(x, height, width)`. Sizes have no sign, so the `-y` of [`to_godot`] does not apply.
pub fn size_to_godot(size: [f32; 3]) -> [f32; 3] {
    [size[0], size[2], size[1]]
}

/// Godot `Vector3` direction of a heading `yaw`, as Godot computes it for a node whose
/// rotation is `yaw` about `+Y`: `Basis(UP, yaw) * RIGHT`.
pub fn heading(yaw: f32) -> [f32; 3] {
    [yaw.cos(), 0.0, -yaw.sin()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;
    use utsim_scene::layout;

    #[test]
    fn model_points_map_to_x_z_minus_y() {
        assert_eq!(to_godot([1.0, 2.0, 3.0]), [1.0, 3.0, -2.0]);
        assert_eq!(from_godot(to_godot([1.0, 2.0, 3.0])), [1.0, 2.0, 3.0]);
        // The floor stays the floor; up is +Y.
        assert_eq!(to_godot([5.0, -4.0, 0.0])[1], 0.0);
        assert_eq!(to_godot([0.0, 0.0, 1.6]), [0.0, 1.6, 0.0]);
    }

    #[test]
    fn layout_space_is_godot_space() {
        for p in [[1.0, 2.0, 3.0], [-7.5, 0.25, 12.0], [0.0, 0.0, 0.0]] {
            let model = layout::to_model(p);
            assert_eq!(to_godot(model), layout_to_godot(p));
            let back = layout::to_bevy([p[0] as f64, p[1] as f64, p[2] as f64]);
            assert_eq!(to_godot([p[0], p[1], p[2]]), back);
        }
    }

    #[test]
    fn a_heading_turns_the_same_way_in_both_spaces() {
        for yaw in [0.0, 0.3, FRAC_PI_2, 2.5, -1.0] {
            // Model heading (cos, sin, 0) mapped to Godot equals the rotated +X.
            let model = [yaw.cos(), yaw.sin(), 0.0];
            let g = to_godot(model);
            let h = heading(yaw_to_godot(yaw));
            for k in 0..3 {
                assert!((g[k] - h[k]).abs() < 1e-6, "{yaw}: {g:?} vs {h:?}");
            }
        }
        // Facing +y in the plan (yaw 90 degrees) is facing -Z in Godot.
        let h = heading(FRAC_PI_2);
        assert!((h[2] + 1.0).abs() < 1e-6);
    }

    #[test]
    fn sizes_swap_width_and_height() {
        assert_eq!(size_to_godot([4.0, 1.0, 2.0]), [4.0, 2.0, 1.0]);
    }
}
