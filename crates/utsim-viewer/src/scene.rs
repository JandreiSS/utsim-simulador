//! The 3D scene: blocks, connections, items, ground grid and picking.

use std::collections::{HashMap, HashSet};
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use utsim_viewer::style::{self, Shape};
use utsim_viewer::timeline::ItemPos;

use crate::playback::{Data, Playback, Selection};

#[derive(Component)]
pub struct BlockVisual;

#[derive(Component)]
pub struct ItemVisual;

const ITEM_SIZE: f32 = 0.3;

#[derive(Resource)]
pub struct BlockMaterials {
    handles: Vec<Handle<StandardMaterial>>,
    last: Vec<Option<Option<utsim_core::State>>>,
}

#[derive(Resource)]
pub struct ItemPool {
    entities: HashMap<usize, Entity>,
    mesh: Handle<Mesh>,
    materials: HashMap<usize, Handle<StandardMaterial>>,
}

fn color(rgb: [f32; 3]) -> Color {
    Color::srgb(rgb[0], rgb[1], rgb[2])
}

fn block_base(data: &Data, b: usize) -> Vec3 {
    Vec3::from(data.positions[b])
}

pub fn setup(
    mut commands: Commands,
    data: Res<Data>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(ClearColor(Color::srgb(0.10, 0.11, 0.14)));
    commands.spawn((
        DirectionalLight {
            illuminance: 9000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(8.0, 14.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        AmbientLight {
            brightness: 350.0,
            ..default()
        },
    ));

    let mut handles = Vec::new();
    for (i, block) in data.replay.blocks.iter().enumerate() {
        let shape = style::shape_of(&block.kind);
        let (half, _) = style::extents(shape);
        let mesh = match shape {
            Shape::LongBox => meshes.add(Cuboid::new(half[0] * 2.0, half[1] * 2.0, half[2] * 2.0)),
            Shape::Box => meshes.add(Cuboid::new(half[0] * 2.0, half[1] * 2.0, half[2] * 2.0)),
            Shape::Cone => meshes.add(Cone::new(0.7, 1.0)),
            Shape::Funnel => meshes.add(Cone::new(1.0, 1.0)),
            Shape::Frustum => meshes.add(ConicalFrustum {
                radius_top: 0.45,
                radius_bottom: 1.0,
                height: 1.0,
            }),
            Shape::Cylinder => meshes.add(Cylinder::new(0.85, 1.0)),
        };
        let material = materials.add(StandardMaterial {
            base_color: color(style::state_color(None)),
            perceptual_roughness: 0.6,
            ..default()
        });
        handles.push(material.clone());
        // Cones point along +Y; the Source arrow points along the flow (+X), the
        // Combiner funnel points down so its flat top carries the items.
        let rotation = match shape {
            Shape::Cone => Quat::from_rotation_z(-FRAC_PI_2),
            Shape::Funnel => Quat::from_rotation_x(PI),
            _ => Quat::IDENTITY,
        };
        let base = block_base(&data, i);
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform {
                translation: base + Vec3::Y * half[1],
                rotation,
                scale: Vec3::ONE,
            },
            BlockVisual,
        ));
    }
    let n = handles.len();
    commands.insert_resource(BlockMaterials {
        handles,
        last: vec![None; n],
    });
    commands.insert_resource(ItemPool {
        entities: HashMap::new(),
        mesh: meshes.add(Cuboid::new(ITEM_SIZE, ITEM_SIZE, ITEM_SIZE)),
        materials: HashMap::new(),
    });
}

pub fn update_block_colors(
    data: Res<Data>,
    pb: Res<Playback>,
    mut mats: ResMut<BlockMaterials>,
    mut assets: ResMut<Assets<StandardMaterial>>,
) {
    for b in 0..mats.handles.len() {
        let state = data.timeline.block_state(b, pb.t);
        if mats.last[b] == Some(state) {
            continue;
        }
        mats.last[b] = Some(state);
        if let Some(mut m) = assets.get_mut(&mats.handles[b]) {
            m.base_color = color(style::state_color(state));
        }
    }
}

pub fn update_items(
    mut commands: Commands,
    data: Res<Data>,
    pb: Res<Playback>,
    mut pool: ResMut<ItemPool>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<&mut Transform, With<ItemVisual>>,
) {
    let t = pb.t;
    let tl = &data.timeline;
    let mut live = Vec::new();
    tl.live_items(t, &mut live);

    // Resting items per block (ordered by arrival), transiting items, container fill.
    let mut resting: HashMap<usize, Vec<(f64, usize)>> = HashMap::new();
    let mut placed: Vec<(usize, Vec3, f32)> = Vec::new();
    let mut fill: HashMap<usize, usize> = HashMap::new();
    let mut transit: Vec<(usize, Vec3)> = Vec::new();
    for &i in &live {
        let (pos, since) = tl.item_pos_since(i, t);
        match pos {
            ItemPos::At(b) if b < data.positions.len() => {
                resting.entry(b).or_default().push((since, i));
            }
            ItemPos::Transit { from, to, progress }
                if from < data.positions.len() && to < data.positions.len() =>
            {
                let s = progress as f32;
                let s = s * s * (3.0 - 2.0 * s);
                let a = block_base(&data, from) + Vec3::Y * 1.2;
                let b = block_base(&data, to) + Vec3::Y * 1.2;
                let arc = Vec3::Y * (PI * s).sin() * 0.7;
                transit.push((i, a.lerp(b, s) + arc));
            }
            ItemPos::Inside(c) => *fill.entry(c).or_default() += 1,
            _ => {}
        }
    }
    for (b, list) in &mut resting {
        list.sort_by(|x, y| x.0.total_cmp(&y.0).then(x.1.cmp(&y.1)));
        let shape = style::shape_of(&data.replay.blocks[*b].kind);
        let base = block_base(&data, *b);
        for (slot, &(_, i)) in list.iter().enumerate() {
            let off = Vec3::from(style::slot_offset(shape, slot));
            placed.push((i, base + off, 1.0));
        }
    }
    for (i, p) in transit {
        placed.push((i, p, 1.0));
    }
    for entry in &mut placed {
        if let Some(&n) = fill.get(&entry.0) {
            // A container that holds items is drawn slightly larger.
            entry.2 = 1.0 + 0.18 * n.min(5) as f32 + 0.2;
        }
    }

    let mut visible: HashSet<usize> = HashSet::with_capacity(placed.len());
    for (i, pos, scale) in placed {
        visible.insert(i);
        let transform = Transform::from_translation(pos).with_scale(Vec3::splat(scale));
        if let Some(&e) = pool.entities.get(&i) {
            if let Ok(mut tr) = transforms.get_mut(e) {
                *tr = transform;
            }
        } else {
            let origin = tl.item_origin(i).unwrap_or(0);
            let mat = pool
                .materials
                .entry(origin)
                .or_insert_with(|| {
                    materials.add(StandardMaterial {
                        base_color: color(style::origin_color(origin)),
                        perceptual_roughness: 0.4,
                        ..default()
                    })
                })
                .clone();
            let e = commands
                .spawn((
                    Mesh3d(pool.mesh.clone()),
                    MeshMaterial3d(mat),
                    transform,
                    ItemVisual,
                ))
                .id();
            pool.entities.insert(i, e);
        }
    }
    pool.entities.retain(|i, e| {
        let keep = visible.contains(i);
        if !keep {
            commands.entity(*e).despawn();
        }
        keep
    });
}

/// Connections, ground grid and selection outline (immediate-mode gizmos).
pub fn draw_gizmos(data: Res<Data>, selection: Res<Selection>, mut gizmos: Gizmos) {
    let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for p in &data.positions {
        min = min.min(Vec3::from(*p));
        max = max.max(Vec3::from(*p));
    }
    // Grid: about 40 cells across the scene, spacing rounded to 1, 2 or 5 x 10^k.
    let span = (max - min).max_element().max(10.0) + 16.0;
    let raw = span / 40.0;
    let mag = 10f32.powf(raw.log10().floor());
    let step = [1.0, 2.0, 5.0, 10.0]
        .iter()
        .map(|m| m * mag)
        .find(|s| *s >= raw)
        .unwrap_or(raw);
    let center = (min + max) / 2.0;
    let half = (span / step / 2.0).ceil() as i32;
    let cx = (center.x / step).round() * step;
    let cz = (center.z / step).round() * step;
    let grid = Color::srgb(0.22, 0.24, 0.28);
    let len = half as f32 * step;
    for k in -half..=half {
        let o = k as f32 * step;
        gizmos.line(
            Vec3::new(cx + o, 0.0, cz - len),
            Vec3::new(cx + o, 0.0, cz + len),
            grid,
        );
        gizmos.line(
            Vec3::new(cx - len, 0.0, cz + o),
            Vec3::new(cx + len, 0.0, cz + o),
            grid,
        );
    }

    let edge = Color::srgb(0.75, 0.80, 0.90);
    for &(a, b) in &data.replay.connections {
        if a >= data.positions.len() || b >= data.positions.len() || a == b {
            continue;
        }
        let pa = block_base(&data, a) + Vec3::Y * 0.4;
        let pb = block_base(&data, b) + Vec3::Y * 0.4;
        let dir = (pb - pa).normalize_or_zero();
        let reach = |blk: usize| {
            let (h, _) = style::extents(style::shape_of(&data.replay.blocks[blk].kind));
            h[0].max(h[2]) + 0.1
        };
        let (s, e) = (pa + dir * reach(a), pb - dir * reach(b));
        if (e - s).dot(dir) > 0.1 {
            gizmos.arrow(s, e, edge).with_tip_length(0.35);
        }
    }

    if let Some(sel) = selection.0.filter(|s| *s < data.positions.len()) {
        let (h, _) = style::extents(style::shape_of(&data.replay.blocks[sel].kind));
        let center = block_base(&data, sel) + Vec3::Y * h[1];
        gizmos.cube(
            Transform::from_translation(center).with_scale(Vec3::new(
                h[0] * 2.0 + 0.3,
                h[1] * 2.0 + 0.3,
                h[2] * 2.0 + 0.3,
            )),
            Color::srgb(1.0, 1.0, 1.0),
        );
    }
}

/// Click (without dragging) on a block selects it; click on empty ground clears.
pub fn pick(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    data: Res<Data>,
    mut selection: ResMut<Selection>,
    mut press: Local<Option<Vec2>>,
    mut contexts: EguiContexts,
) {
    let over_ui = contexts
        .ctx_mut()
        .is_ok_and(|ctx| ctx.egui_wants_pointer_input() || ctx.is_pointer_over_egui());
    let cursor = window.cursor_position();
    if buttons.just_pressed(MouseButton::Left) {
        *press = if over_ui { None } else { cursor };
    }
    if !buttons.just_released(MouseButton::Left) {
        return;
    }
    let (Some(start), Some(cursor)) = (press.take(), cursor) else {
        return;
    };
    if start.distance(cursor) > 4.0 || over_ui {
        return;
    }
    let (camera, transform) = *camera;
    let Ok(ray) = camera.viewport_to_world(transform, cursor) else {
        return;
    };
    let mut best: Option<(f32, usize)> = None;
    for (i, block) in data.replay.blocks.iter().enumerate() {
        let (h, _) = style::extents(style::shape_of(&block.kind));
        let center = block_base(&data, i) + Vec3::Y * h[1];
        if let Some(d) = ray_box(
            ray.origin,
            *ray.direction,
            center - Vec3::from(h),
            center + Vec3::from(h),
        ) && best.is_none_or(|(bd, _)| d < bd)
        {
            best = Some((d, i));
        }
    }
    selection.0 = best.map(|(_, i)| i);
}

/// Slab test: distance along the ray to the box, if hit.
fn ray_box(origin: Vec3, dir: Vec3, min: Vec3, max: Vec3) -> Option<f32> {
    let inv = Vec3::ONE / dir;
    let t1 = (min - origin) * inv;
    let t2 = (max - origin) * inv;
    let near = t1.min(t2).max_element();
    let far = t1.max(t2).min_element();
    (far >= near.max(0.0)).then_some(near.max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_hits_and_misses_a_box() {
        let (min, max) = (Vec3::splat(-1.0), Vec3::splat(1.0));
        let d = ray_box(Vec3::new(0.0, 0.0, 5.0), Vec3::NEG_Z, min, max).unwrap();
        assert!((d - 4.0).abs() < 1e-5);
        assert!(ray_box(Vec3::new(3.0, 0.0, 5.0), Vec3::NEG_Z, min, max).is_none());
        assert!(ray_box(Vec3::new(0.0, 0.0, 5.0), Vec3::Z, min, max).is_none());
    }
}
