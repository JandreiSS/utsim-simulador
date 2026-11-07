//! The floor grid (step 7, part 3, of `docs/plano_interface_rust.md`): its extent and step
//! in Editar (around the objects, the snap step, coarser when it would have too many
//! lines) and in Simular (about 40 cells across the scene, 1, 2 or 5 × 10^k metres), and
//! its lines. Godot coordinates (`y` up).

pub type V3 = [f64; 3];

/// Editar: at least 40 m around the origin and 10 m around every object, the step of the
/// snap doubled until there are at most 400 lines each way.
pub fn edit_extent(positions: &[V3], snap: f64) -> (V3, V3, f64) {
    let (mut lo, mut hi): (V3, V3) = ([-20.0, 0.0, -20.0], [20.0, 0.0, 20.0]);
    for p in positions {
        lo = [lo[0].min(p[0] - 10.0), 0.0, lo[2].min(p[2] - 10.0)];
        hi = [hi[0].max(p[0] + 10.0), 0.0, hi[2].max(p[2] + 10.0)];
    }
    let mut step = if snap > 0.0 { snap } else { 1.0 };
    while (hi[0] - lo[0]) / step > 400.0 || (hi[2] - lo[2]) / step > 400.0 {
        step *= 2.0;
    }
    (lo, hi, step)
}

/// Simular: a square around the scene's bounds with about 40 cells, the step rounded up to
/// 1, 2 or 5 × 10^k.
pub fn sim_extent(lo: V3, hi: V3) -> (V3, V3, f64) {
    let span = (hi[0] - lo[0]).max(hi[2] - lo[2]).max(10.0) + 16.0;
    let raw = span / 40.0;
    let mag = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 5.0, 10.0]
        .into_iter()
        .map(|m| m * mag)
        .find(|s| *s >= raw)
        .unwrap_or(raw);
    let c = [(lo[0] + hi[0]) / 2.0, 0.0, (lo[2] + hi[2]) / 2.0];
    (
        [c[0] - span / 2.0, 0.0, c[2] - span / 2.0],
        [c[0] + span / 2.0, 0.0, c[2] + span / 2.0],
        step,
    )
}

/// The grid lines (pairs of points) from `lo` to `hi` every `step`, on whole multiples of
/// the step, just above the floor.
pub fn lines(lo: V3, hi: V3, step: f64) -> Vec<V3> {
    if step <= 0.0 {
        return Vec::new();
    }
    let (x0, x1) = ((lo[0] / step).floor() * step, (hi[0] / step).ceil() * step);
    let (z0, z1) = ((lo[2] / step).floor() * step, (hi[2] / step).ceil() * step);
    let y = 0.002;
    let mut out = Vec::new();
    let nx = ((x1 - x0) / step).round() as i64;
    for k in 0..=nx {
        let x = x0 + k as f64 * step;
        out.extend([[x, y, z0], [x, y, z1]]);
    }
    let nz = ((z1 - z0) / step).round() as i64;
    for k in 0..=nz {
        let z = z0 + k as f64 * step;
        out.extend([[x0, y, z], [x1, y, z]]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_editar_grid_covers_the_objects_and_stays_under_400_lines() {
        let (lo, hi, step) = edit_extent(&[[100.0, 0.0, -5.0]], 1.0);
        assert_eq!((lo[0], hi[0], lo[2], hi[2]), (-20.0, 110.0, -20.0, 20.0));
        assert_eq!(step, 1.0);
        let (_, _, step) = edit_extent(&[[1000.0, 0.0, 0.0]], 1.0);
        assert_eq!(step, 4.0, "1030 m wide: 1, 2, then 4 m");
        assert_eq!(edit_extent(&[], 0.0).2, 1.0);
    }

    #[test]
    fn the_simular_grid_has_about_forty_cells_of_a_round_step() {
        let (lo, hi, step) = sim_extent([0.0; 3], [100.0, 5.0, 40.0]);
        assert_eq!(step, 5.0, "116 m / 40 = 2.9 → 5");
        assert!((hi[0] - lo[0] - 116.0).abs() < 1e-9);
        assert_eq!(sim_extent([0.0; 3], [1.0, 0.0, 1.0]).2, 1.0);
    }

    #[test]
    fn lines_fall_on_multiples_of_the_step_and_come_in_pairs() {
        let l = lines([-1.5, 0.0, -1.0], [1.2, 0.0, 1.0], 1.0);
        assert!(l.len().is_multiple_of(2));
        // x from -2 to 2 (5 lines) and z from -1 to 1 (3 lines).
        assert_eq!(l.len(), 2 * (5 + 3));
        assert!(
            l.iter()
                .all(|p| (p[0].fract() == 0.0 || p[0] == -2.0 || p[0] == 2.0) && p[1] > 0.0)
        );
        assert!(lines([0.0; 3], [1.0; 3], 0.0).is_empty());
    }
}
