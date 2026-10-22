//! Pure geometry of the 3D view (no window, no GPU): camera framing, label
//! decluttering in screen space, connection clipping against block footprints and
//! dashed segments. Vectors are plain `[f32; 3]` in Bevy space (Y up).

pub type V3 = [f32; 3];

/// Height above the ground at which connection arrows are drawn.
pub const ARROW_HEIGHT: f32 = 0.12;

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: V3, b: V3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Axis-aligned bounds of the blocks (base position plus footprint and height) and of
/// extra points (path network nodes). `None` when there is nothing to frame.
pub fn scene_bounds(positions: &[V3], halves: &[V3], points: &[V3]) -> Option<(V3, V3)> {
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    let mut any = false;
    let mut add = |p: V3| {
        any = true;
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    };
    for (p, h) in positions.iter().zip(halves) {
        add([p[0] - h[0], p[1], p[2] - h[2]]);
        add([p[0] + h[0], p[1] + 2.0 * h[1], p[2] + h[2]]);
    }
    for p in points {
        add(*p);
    }
    any.then_some((min, max))
}

/// Camera placement that fits the box `min..max` in the view.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Framing {
    pub target: V3,
    pub dist: f32,
}

/// Orbit camera (target, yaw, pitch, distance) looking at the box center so that all
/// eight corners are inside a perspective view of vertical field of view `fov_y` and
/// the given `aspect` (width / height). `margin` is a fraction of extra room.
pub fn frame_box(
    min: V3,
    max: V3,
    yaw: f32,
    pitch: f32,
    aspect: f32,
    fov_y: f32,
    margin: f32,
) -> Framing {
    let center = [
        (min[0] + max[0]) / 2.0,
        (min[1] + max[1]) / 2.0,
        (min[2] + max[2]) / 2.0,
    ];
    // Camera axes for the orbit used by the viewer: back = Ry(yaw) * Rx(-pitch) * Z.
    let back = [
        pitch.cos() * yaw.sin(),
        pitch.sin(),
        pitch.cos() * yaw.cos(),
    ];
    let right = [yaw.cos(), 0.0, -yaw.sin()];
    let up = cross(back, right);
    let tan_v = (fov_y / 2.0).tan();
    let tan_h = tan_v * aspect.max(0.05);
    let mut dist = 0.0f32;
    for i in 0..8 {
        let corner = [
            if i & 1 == 0 { min[0] } else { max[0] },
            if i & 2 == 0 { min[1] } else { max[1] },
            if i & 4 == 0 { min[2] } else { max[2] },
        ];
        let d = sub(corner, center);
        let (x, y, z) = (dot(d, right), dot(d, up), dot(d, back));
        dist = dist.max(z + x.abs() / tan_h).max(z + y.abs() / tan_v);
    }
    Framing {
        target: center,
        dist: (dist * (1.0 + margin)).max(6.0),
    }
}

/// A label to place: `anchor` is where its bottom center wants to be, in screen pixels.
#[derive(Clone, Copy, Debug)]
pub struct LabelBox {
    pub anchor: [f32; 2],
    pub size: [f32; 2],
    /// Placed before the others (it keeps its spot and the rest move around it).
    pub first: bool,
}

/// Where a label ended up: top-left corner and whether it was pushed away from its anchor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placed {
    pub min: [f32; 2],
    pub size: [f32; 2],
    pub displaced: bool,
}

fn overlaps(a: ([f32; 2], [f32; 2]), b: ([f32; 2], [f32; 2])) -> bool {
    a.0[0] < b.0[0] + b.1[0]
        && b.0[0] < a.0[0] + a.1[0]
        && a.0[1] < b.0[1] + b.1[1]
        && b.0[1] < a.0[1] + a.1[1]
}

/// Greedy label decluttering: each label goes above its anchor; when it would overlap
/// one already placed it is pushed up by its height (plus `gap`) until it is free.
/// The result is indexed like `labels`.
pub fn declutter(labels: &[LabelBox], gap: f32) -> Vec<Placed> {
    let mut order: Vec<usize> = (0..labels.len()).collect();
    order.sort_by_key(|&i| !labels[i].first);
    let mut placed: Vec<Option<Placed>> = vec![None; labels.len()];
    let mut taken: Vec<([f32; 2], [f32; 2])> = Vec::new();
    for i in order {
        let l = labels[i];
        let home = [l.anchor[0] - l.size[0] / 2.0, l.anchor[1] - l.size[1]];
        let mut min = home;
        let mut steps = 0;
        while taken.iter().any(|t| overlaps((min, l.size), *t)) && steps <= labels.len() {
            min[1] -= l.size[1] + gap;
            steps += 1;
        }
        taken.push((min, l.size));
        placed[i] = Some(Placed {
            min,
            size: l.size,
            displaced: steps > 0,
        });
    }
    placed.into_iter().flatten().collect()
}

/// Arrow for a connection between two blocks, clipped to the blocks' footprints so the
/// tail starts at the source surface and the head ends at the target surface, drawn at
/// the fixed `height`. `pa`/`pb` are base positions, `ha`/`hb` half extents. `None`
/// when the blocks touch or overlap (nothing to draw).
pub fn connection_segment(pa: V3, ha: V3, pb: V3, hb: V3, height: f32) -> Option<(V3, V3)> {
    let d = [pb[0] - pa[0], pb[2] - pa[2]];
    if d[0].abs() < 1e-5 && d[1].abs() < 1e-5 {
        return None;
    }
    // Parameter at which the ray from a box center leaves the box (footprint only).
    let leave = |h: V3| {
        let tx = if d[0].abs() > 1e-6 {
            h[0] / d[0].abs()
        } else {
            f32::INFINITY
        };
        let tz = if d[1].abs() > 1e-6 {
            h[2] / d[1].abs()
        } else {
            f32::INFINITY
        };
        tx.min(tz)
    };
    let (t0, t1) = (leave(ha), 1.0 - leave(hb));
    let len = (d[0] * d[0] + d[1] * d[1]).sqrt();
    if (t1 - t0) * len < 0.1 {
        return None;
    }
    let at = |t: f32| [pa[0] + d[0] * t, height, pa[2] + d[1] * t];
    Some((at(t0), at(t1)))
}

/// Splits `start..end` into dashes of length `dash` separated by `gap`.
pub fn dashes(start: V3, end: V3, dash: f32, gap: f32) -> Vec<(V3, V3)> {
    let d = sub(end, start);
    let len = dot(d, d).sqrt();
    if len < 1e-5 || dash <= 0.0 {
        return Vec::new();
    }
    let dir = [d[0] / len, d[1] / len, d[2] / len];
    let at = |s: f32| {
        [
            start[0] + dir[0] * s,
            start[1] + dir[1] * s,
            start[2] + dir[2] * s,
        ]
    };
    let mut out = Vec::new();
    let mut s = 0.0;
    while s < len {
        out.push((at(s), at((s + dash).min(len))));
        s += dash + gap.max(0.0);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style;

    #[test]
    fn bounds_include_block_extents_and_extra_points() {
        let half = |kind: &str| style::extents(style::shape_of(kind)).0;
        let (min, max) = scene_bounds(
            &[[0.0; 3], [10.0, 0.0, 0.0]],
            &[half("Processor"), half("Sink")],
            &[],
        )
        .unwrap();
        assert!(min[0] < 0.0 && max[0] > 10.0);
        assert!(max[1] > 0.0);
        let (min, max) = scene_bounds(&[[0.0; 3]], &[half("Sink")], &[[0.0, 0.0, -30.0]]).unwrap();
        assert_eq!(min[2], -30.0);
        assert!(max[2] > 0.0);
        assert!(scene_bounds(&[], &[], &[]).is_none());
    }

    /// Projects a point with the same orbit convention and checks it is inside the view.
    fn inside(f: Framing, yaw: f32, pitch: f32, aspect: f32, fov: f32, p: V3) -> bool {
        let back = [
            pitch.cos() * yaw.sin(),
            pitch.sin(),
            pitch.cos() * yaw.cos(),
        ];
        let right = [yaw.cos(), 0.0, -yaw.sin()];
        let up = cross(back, right);
        let eye = [
            f.target[0] + back[0] * f.dist,
            f.target[1] + back[1] * f.dist,
            f.target[2] + back[2] * f.dist,
        ];
        let d = sub(p, eye);
        let depth = -dot(d, back);
        let tv = (fov / 2.0).tan();
        depth > 0.0
            && (dot(d, right) / depth).abs() <= tv * aspect + 1e-4
            && (dot(d, up) / depth).abs() <= tv + 1e-4
    }

    #[test]
    fn framing_fits_every_corner_for_several_aspects() {
        let (min, max) = ([-2.0, 0.0, -25.0], [42.0, 1.0, 2.0]);
        for aspect in [0.6, 1.0, 1.6, 2.4] {
            let f = frame_box(
                min,
                max,
                0.0,
                0.75,
                aspect,
                std::f32::consts::FRAC_PI_4,
                0.05,
            );
            for i in 0..8 {
                let c = [
                    if i & 1 == 0 { min[0] } else { max[0] },
                    if i & 2 == 0 { min[1] } else { max[1] },
                    if i & 4 == 0 { min[2] } else { max[2] },
                ];
                assert!(
                    inside(f, 0.0, 0.75, aspect, std::f32::consts::FRAC_PI_4, c),
                    "aspect {aspect} corner {c:?}"
                );
            }
        }
    }

    #[test]
    fn narrower_view_needs_a_larger_distance() {
        let (min, max) = ([0.0, 0.0, -20.0], [40.0, 1.0, 0.0]);
        let wide = frame_box(min, max, 0.0, 0.75, 2.0, 0.78, 0.05);
        let narrow = frame_box(min, max, 0.0, 0.75, 0.8, 0.78, 0.05);
        assert!(narrow.dist > wide.dist);
        assert_eq!(wide.target, [20.0, 0.5, -10.0]);
    }

    #[test]
    fn stacked_labels_are_pushed_up_without_overlapping() {
        let l = |first| LabelBox {
            anchor: [100.0, 100.0],
            size: [60.0, 14.0],
            first,
        };
        let placed = declutter(&[l(false), l(false), l(false), l(true)], 2.0);
        // The `first` one keeps the home spot.
        assert!(!placed[3].displaced);
        assert_eq!(placed[3].min, [70.0, 86.0]);
        for i in 0..4 {
            for j in (i + 1)..4 {
                assert!(
                    !overlaps(
                        (placed[i].min, placed[i].size),
                        (placed[j].min, placed[j].size)
                    ),
                    "{i} vs {j}"
                );
            }
        }
        assert!(placed[0].displaced && placed[1].displaced && placed[2].displaced);
        assert!(placed[0].min[1] < placed[3].min[1]);
    }

    #[test]
    fn separated_labels_stay_at_home() {
        let a = LabelBox {
            anchor: [50.0, 50.0],
            size: [40.0, 14.0],
            first: false,
        };
        let b = LabelBox {
            anchor: [300.0, 50.0],
            ..a
        };
        let placed = declutter(&[a, b], 2.0);
        assert!(placed.iter().all(|p| !p.displaced));
        assert_eq!(placed[1].min, [280.0, 36.0]);
    }

    #[test]
    fn connection_is_clipped_to_both_footprints() {
        let h = [1.0, 0.5, 1.0];
        let (s, e) = connection_segment([0.0; 3], h, [10.0, 0.0, 0.0], h, 0.12).unwrap();
        assert!((s[0] - 1.0).abs() < 1e-5 && (e[0] - 9.0).abs() < 1e-5);
        assert_eq!((s[1], e[1]), (0.12, 0.12));
        // Diagonal: the tail leaves through the box side, never inside the box.
        let (s, e) = connection_segment([0.0; 3], h, [10.0, 0.0, 10.0], h, 0.12).unwrap();
        assert!((s[0] - 1.0).abs() < 1e-5 && (s[2] - 1.0).abs() < 1e-5);
        assert!((e[0] - 9.0).abs() < 1e-5 && (e[2] - 9.0).abs() < 1e-5);
        // Different sizes: long queue (half 2.0 on X) to a processor.
        let (s, e) =
            connection_segment([0.0; 3], [2.0, 0.15, 0.7], [10.0, 0.0, 0.0], h, 0.12).unwrap();
        assert!((s[0] - 2.0).abs() < 1e-5 && (e[0] - 9.0).abs() < 1e-5);
    }

    #[test]
    fn overlapping_or_touching_blocks_get_no_arrow() {
        let h = [1.0, 0.5, 1.0];
        assert!(connection_segment([0.0; 3], h, [0.0; 3], h, 0.1).is_none());
        assert!(connection_segment([0.0; 3], h, [1.5, 0.0, 0.0], h, 0.1).is_none());
        assert!(connection_segment([0.0; 3], h, [2.05, 0.0, 0.0], h, 0.1).is_none());
    }

    #[test]
    fn dashes_cover_the_segment_with_gaps() {
        let d = dashes([0.0; 3], [1.0, 0.0, 0.0], 0.2, 0.1);
        assert_eq!(d.len(), 4);
        assert_eq!(d[0], ([0.0; 3], [0.2, 0.0, 0.0]));
        assert!((d[3].1[0] - 1.0).abs() < 1e-5);
        assert!(d[1].0[0] > d[0].1[0]);
        assert!(dashes([0.0; 3], [0.0; 3], 0.2, 0.1).is_empty());
    }
}
