//! The 3D scene: blocks, connections, items, ground grid and picking.

use std::collections::{HashMap, HashSet};
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use utsim_viewer::editing::NetworkView;
use utsim_viewer::geometry::{self, Geom};
use utsim_viewer::handling::{BEAM, BELT_THICKNESS, BELT_WIDTH};
use utsim_viewer::layout;
use utsim_viewer::style::{self, Shape};
use utsim_viewer::timeline::ItemPos;
use utsim_viewer::view;

use crate::dock::{Dock, pointer_over_ui};
use crate::editor::{Editor, Mode};
use crate::playback::{Data, Playback, Selection};

#[derive(Component)]
pub struct BlockVisual;

#[derive(Component)]
pub struct ItemVisual;

const ITEM_SIZE: f32 = 0.3;

/// Visuals of the blocks currently shown (rebuilt by [`sync_blocks`]).
#[derive(Resource, Default)]
pub struct BlockMaterials {
    handles: Vec<Handle<StandardMaterial>>,
    last: Vec<Option<Option<utsim_core::State>>>,
    kinds: Vec<String>,
    /// Geometry the entities were built for (a change rebuilds them).
    geoms: Vec<Geom>,
    entities: Vec<Entity>,
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

/// Base position of block `b` at time `t`: its layout position, or, for a mobile
/// resource that has started travelling, where the trace says it is.
pub fn block_base_at(data: &Data, b: usize, t: f64) -> Vec3 {
    data.timeline
        .carrier_pos(b, t)
        .map_or_else(|| block_base(data, b), |p| Vec3::from(layout::to_bevy(p)))
}

fn block_mesh(shape: Shape, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
    let (half, _) = style::extents(shape);
    match shape {
        Shape::LongBox | Shape::Box => {
            meshes.add(Cuboid::new(half[0] * 2.0, half[1] * 2.0, half[2] * 2.0))
        }
        Shape::Cone => meshes.add(Cone::new(0.7, 1.0)),
        Shape::Funnel => meshes.add(Cone::new(1.0, 1.0)),
        Shape::Frustum => meshes.add(ConicalFrustum {
            radius_top: 0.45,
            radius_bottom: 1.0,
            height: 1.0,
        }),
        Shape::Cylinder => meshes.add(Cylinder::new(0.85, 1.0)),
        Shape::Vehicle => meshes.add(Cuboid::new(half[0] * 2.0, half[1] * 2.0, half[2] * 2.0)),
        Shape::Flag => meshes.add(Cone::new(0.25, half[1] * 2.0)),
        // Belts and racks are built by `spawn_block` from their real size.
        Shape::Belt | Shape::Rack => {
            meshes.add(Cuboid::new(half[0] * 2.0, half[1] * 2.0, half[2] * 2.0))
        }
    }
}

/// Spawns the visual of one block at `translation` (its base plus half its height).
fn spawn_block(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    geom: &Geom,
    material: Handle<StandardMaterial>,
    translation: Vec3,
) -> Entity {
    let steel = materials.add(StandardMaterial {
        base_color: Color::srgb(0.27, 0.29, 0.34),
        perceptual_roughness: 0.7,
        ..default()
    });
    match (geom.shape, geom.belt_params(), geom.rack_params()) {
        (Shape::Belt, Some(p), _) => {
            let length = p.length as f32;
            let id = commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(length, BELT_THICKNESS, BELT_WIDTH))),
                    MeshMaterial3d(material),
                    Transform {
                        translation,
                        rotation: Quat::from_rotation_y(geom.yaw),
                        scale: Vec3::ONE,
                    },
                    BlockVisual,
                ))
                .id();
            let rail = meshes.add(Cuboid::new(length, 0.14, 0.06));
            let roller = meshes.add(Cuboid::new(0.05, 0.012, BELT_WIDTH - 0.14));
            // Cross lines every item length (at most ~80 of them) show the belt scale.
            let step = p.item_length.max(p.length / 80.0) as f32;
            let count = (length / step).floor() as usize;
            commands.entity(id).with_children(|parent| {
                for side in [-1.0f32, 1.0] {
                    parent.spawn((
                        Mesh3d(rail.clone()),
                        MeshMaterial3d(steel.clone()),
                        Transform::from_xyz(
                            0.0,
                            BELT_THICKNESS / 2.0 + 0.07,
                            side * (BELT_WIDTH / 2.0 - 0.03),
                        ),
                    ));
                }
                for k in 0..=count {
                    parent.spawn((
                        Mesh3d(roller.clone()),
                        MeshMaterial3d(steel.clone()),
                        Transform::from_xyz(
                            -length / 2.0 + k as f32 * step,
                            BELT_THICKNESS / 2.0 + 0.006,
                            0.0,
                        ),
                    ));
                }
            });
            id
        }
        (Shape::Rack, _, Some(p)) => {
            let (uprights, beams) = geometry::rack_frame(&p);
            let (total_h, half_h) = (p.total_height(), geom.half[1]);
            let post = meshes.add(Cuboid::new(BEAM, total_h, BEAM));
            let beam = meshes.add(Cuboid::new(p.total_width(), BEAM, BEAM * 1.6));
            // The beams are the "deck" of each level: the items rest across both.
            let lane = p.item_size() * 0.4;
            let id = commands
                .spawn((
                    Transform::from_translation(translation),
                    Visibility::default(),
                    BlockVisual,
                ))
                .id();
            commands.entity(id).with_children(|parent| {
                for (x, z) in uprights {
                    parent.spawn((
                        Mesh3d(post.clone()),
                        MeshMaterial3d(steel.clone()),
                        Transform::from_xyz(x, 0.0, z),
                    ));
                }
                for y in beams {
                    for z in [-lane, lane] {
                        parent.spawn((
                            Mesh3d(beam.clone()),
                            MeshMaterial3d(material.clone()),
                            Transform::from_xyz(0.0, y - half_h, z),
                        ));
                    }
                }
            });
            id
        }
        _ => {
            let shape = geom.shape;
            let mast = (shape == Shape::Vehicle)
                .then(|| (meshes.add(Cuboid::new(0.12, 0.9, 0.12)), material.clone()));
            let id = commands
                .spawn((
                    Mesh3d(block_mesh(shape, meshes)),
                    MeshMaterial3d(material),
                    Transform {
                        translation,
                        rotation: block_rotation(shape),
                        scale: Vec3::ONE,
                    },
                    BlockVisual,
                ))
                .id();
            if let Some((mesh, material)) = mast {
                // The mast marks a transporter (a lift mast at the rear of the body).
                commands.entity(id).with_children(|parent| {
                    parent.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(material),
                        Transform::from_xyz(-0.3, 0.55, 0.0),
                    ));
                });
            }
            id
        }
    }
}

/// Cones point along +Y; the Source arrow points along the flow (+X), the Combiner
/// funnel points down so its flat top carries the items.
fn block_rotation(shape: Shape) -> Quat {
    match shape {
        Shape::Cone => Quat::from_rotation_z(-FRAC_PI_2),
        Shape::Funnel => Quat::from_rotation_x(PI),
        _ => Quat::IDENTITY,
    }
}

/// Keeps the block entities in line with what is shown: the document in `Editar`,
/// the replay in `Simular`. Entities are rebuilt when the list of kinds, the mode
/// or the replay changes; otherwise only the positions are updated.
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn sync_blocks(
    mut commands: Commands,
    editor: Res<Editor>,
    data: Res<Data>,
    mut mats: ResMut<BlockMaterials>,
    mut pool: ResMut<ItemPool>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<&mut Transform, With<BlockVisual>>,
    mut last_mode: Local<Option<Mode>>,
) {
    let (kinds, positions, geoms): (Vec<&str>, &[[f32; 3]], &[Geom]) = match editor.mode {
        Mode::Edit => (
            editor.view.kinds.clone(),
            &editor.view.positions,
            &editor.view.geoms,
        ),
        Mode::Sim => (
            data.replay.blocks.iter().map(|b| b.kind.as_str()).collect(),
            &data.positions,
            &data.geoms,
        ),
    };
    let rebuild = *last_mode != Some(editor.mode)
        || (editor.mode == Mode::Sim && data.is_changed())
        || mats
            .kinds
            .iter()
            .map(String::as_str)
            .ne(kinds.iter().copied())
        || mats.geoms.as_slice() != geoms;
    if rebuild {
        *last_mode = Some(editor.mode);
        for e in mats.entities.drain(..) {
            commands.entity(e).despawn();
        }
        for (_, e) in pool.entities.drain() {
            commands.entity(e).despawn();
        }
        mats.handles.clear();
        mats.last.clear();
        mats.kinds = kinds.iter().map(|k| k.to_string()).collect();
        mats.geoms = geoms.to_vec();
        for (i, geom) in geoms.iter().enumerate() {
            let material = materials.add(StandardMaterial {
                base_color: color(style::state_color(None)),
                perceptual_roughness: 0.6,
                ..default()
            });
            mats.handles.push(material.clone());
            mats.last.push(None);
            let translation = Vec3::from(positions[i]) + Vec3::Y * geom.half[1];
            let id = spawn_block(
                &mut commands,
                &mut meshes,
                &mut materials,
                geom,
                material,
                translation,
            );
            if kinds[i] == "Flow" {
                // Token flows live in the Fluxos panel, not in the 3D scene.
                commands.entity(id).insert(Visibility::Hidden);
            }
            mats.entities.push(id);
        }
        return;
    }
    for (i, &e) in mats.entities.iter().enumerate() {
        if let Ok(mut tr) = transforms.get_mut(e) {
            tr.translation = Vec3::from(positions[i]) + Vec3::Y * geoms[i].half[1];
        }
    }
}

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
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

    commands.insert_resource(BlockMaterials::default());
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

/// Moves the mobile resources to where the trace says they are at the current time
/// (runs after `sync_blocks`, which resets every block to its layout position).
pub fn update_carriers(
    data: Res<Data>,
    pb: Res<Playback>,
    mats: Res<BlockMaterials>,
    mut transforms: Query<&mut Transform, With<BlockVisual>>,
) {
    for (i, &e) in mats.entities.iter().enumerate() {
        if i >= data.replay.blocks.len() || !data.timeline.is_mobile(i) {
            continue;
        }
        if let Ok(mut tr) = transforms.get_mut(e) {
            tr.translation = block_base_at(&data, i, pb.t) + Vec3::Y * data.geoms[i].half[1];
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
                let a = block_base_at(&data, from, t) + Vec3::Y * 1.2;
                let b = block_base_at(&data, to, t) + Vec3::Y * 1.2;
                let arc = Vec3::Y * (PI * s).sin() * 0.7;
                transit.push((i, a.lerp(b, s) + arc));
            }
            ItemPos::Inside(c) => *fill.entry(c).or_default() += 1,
            _ => {}
        }
    }
    for (b, list) in &mut resting {
        list.sort_by(|x, y| x.0.total_cmp(&y.0).then(x.1.cmp(&y.1)));
        let geom = &data.geoms[*b];
        // Items inside a transporter ride on it (stacked on its body).
        let base = block_base_at(&data, *b, t);
        match geom.shape {
            // Belts and racks put every item where the trace says it is.
            Shape::Belt | Shape::Rack => {
                for &(_, i) in list.iter() {
                    let spot = if geom.shape == Shape::Belt {
                        tl.belt_position(*b, i, t)
                            .and_then(|distance| geom.belt_item(distance))
                    } else {
                        tl.rack_slot(*b, i, t).and_then(|slot| geom.rack_item(slot))
                    };
                    if let Some((offset, size)) = spot {
                        placed.push((i, base + Vec3::from(offset), size / ITEM_SIZE));
                    }
                }
            }
            shape => {
                for (slot, &(_, i)) in list.iter().enumerate() {
                    // The stack is capped; the label shows how many items are not drawn.
                    if slot >= style::visible_slots(shape) {
                        break;
                    }
                    let off = Vec3::from(style::slot_offset(shape, slot));
                    placed.push((i, base + off, 1.0));
                }
            }
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

/// Connections, ground grid and selection outline in `Simular`.
pub fn draw_sim(data: Res<Data>, pb: Res<Playback>, selection: Res<Selection>, mut gizmos: Gizmos) {
    // Mobile resources are outlined where they are now.
    let positions: Vec<[f32; 3]> = (0..data.positions.len())
        .map(|b| block_base_at(&data, b, pb.t).to_array())
        .collect();
    let outlines: Vec<(usize, Color)> = selection
        .0
        .filter(|s| *s < data.positions.len())
        .map(|s| (s, Color::srgb(1.0, 1.0, 1.0)))
        .into_iter()
        .collect();
    draw_world(
        &mut gizmos,
        &positions,
        &data.geoms,
        &data.replay.connections,
        &data.logical,
        &outlines,
        None,
    );
    draw_network(&mut gizmos, &data.network);
}

/// The path network on the ground: nodes as small markers, edges as thin lines
/// (arrows for one-way edges).
pub fn draw_network(gizmos: &mut Gizmos, network: &NetworkView) {
    let color = Color::srgb(0.95, 0.75, 0.25);
    let lift = Vec3::Y * 0.03;
    for seg in &network.segments {
        let (a, b) = (Vec3::from(seg.a) + lift, Vec3::from(seg.b) + lift);
        if seg.two_way {
            gizmos.line(a, b, color);
        } else {
            gizmos.arrow(a, b, color).with_tip_length(0.4);
        }
    }
    for n in &network.nodes {
        gizmos.sphere(
            Isometry3d::from_translation(Vec3::from(*n) + lift),
            0.12,
            color,
        );
    }
}

/// Draws the ground grid (`fixed_step` forces its spacing, else it adapts to the
/// scene), the connections as arrows and an outline around the listed blocks.
pub fn draw_world(
    gizmos: &mut Gizmos,
    positions: &[[f32; 3]],
    geoms: &[Geom],
    connections: &[(usize, usize)],
    logical: &[bool],
    outlines: &[(usize, Color)],
    fixed_step: Option<f32>,
) {
    let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for p in positions {
        min = min.min(Vec3::from(*p));
        max = max.max(Vec3::from(*p));
    }
    if positions.is_empty() {
        (min, max) = (Vec3::ZERO, Vec3::ZERO);
    }
    // Grid: about 40 cells across the scene, spacing rounded to 1, 2 or 5 x 10^k.
    let min_span = if fixed_step.is_some() { 24.0 } else { 10.0 };
    let span = (max - min).max_element().max(min_span) + 16.0;
    let raw = span / 40.0;
    let mag = 10f32.powf(raw.log10().floor());
    let step = fixed_step.unwrap_or_else(|| {
        [1.0, 2.0, 5.0, 10.0]
            .iter()
            .map(|m| m * mag)
            .find(|s| *s >= raw)
            .unwrap_or(raw)
    });
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
    for (n, &(a, b)) in connections.iter().enumerate() {
        let Some((s, e)) = connection_line(positions, geoms, a, b) else {
            continue;
        };
        if logical.get(n).copied().unwrap_or(false) {
            // Logical flow (a vehicle carries the item): dashed and dimmer, with a short head.
            let dim = Color::srgb(0.45, 0.50, 0.60);
            for (p, q) in view::dashes(s.to_array(), e.to_array(), 0.35, 0.25) {
                gizmos.line(Vec3::from(p), Vec3::from(q), dim);
            }
            let dir = (e - s).normalize_or_zero();
            let tail = (e - dir * 0.5).max(s.min(e)).clamp(s.min(e), s.max(e));
            gizmos.arrow(tail, e, dim).with_tip_length(0.3);
        } else {
            gizmos.arrow(s, e, edge).with_tip_length(0.35);
        }
    }

    for &(i, color) in outlines {
        let geom = &geoms[i];
        let center = Vec3::from(positions[i]) + Vec3::Y * geom.half[1];
        // A belt is outlined along its own axis, the rest by their bounding box.
        let (size, rotation) = match geom.belt_params() {
            Some(p) => (
                Vec3::new(p.length as f32, BELT_THICKNESS, BELT_WIDTH),
                Quat::from_rotation_y(geom.yaw),
            ),
            None => (Vec3::from(geom.half) * 2.0, Quat::IDENTITY),
        };
        gizmos.cube(
            Transform {
                translation: center,
                rotation,
                scale: size + Vec3::splat(0.3),
            },
            color,
        );
    }
}

/// Start and end of the arrow for connection `a -> b`, clipped to the block
/// footprints (tail on the source surface, head on the target surface) at a fixed
/// height above the ground; `None` when the blocks overlap or the indices are invalid.
pub fn connection_line(
    positions: &[[f32; 3]],
    geoms: &[Geom],
    a: usize,
    b: usize,
) -> Option<(Vec3, Vec3)> {
    if a >= positions.len() || b >= positions.len() || a == b {
        return None;
    }
    let half = |blk: usize| geoms[blk].half;
    view::connection_segment(
        positions[a],
        half(a),
        positions[b],
        half(b),
        view::ARROW_HEIGHT,
    )
    .map(|(s, e)| (Vec3::from(s), Vec3::from(e)))
}

/// Click (without dragging) on a block selects it; click on empty ground clears.
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn pick(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    data: Res<Data>,
    pb: Res<Playback>,
    mut selection: ResMut<Selection>,
    mut press: Local<Option<Vec2>>,
    dock: Res<Dock>,
    mut contexts: EguiContexts,
) {
    let over_ui = contexts
        .ctx_mut()
        .is_ok_and(|ctx| pointer_over_ui(ctx, &dock));
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
    for i in (0..data.replay.blocks.len()).filter(|&i| !data.is_flow(i)) {
        let h = data.geoms[i].half;
        let center = block_base_at(&data, i, pb.t) + Vec3::Y * h[1];
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
pub fn ray_box(origin: Vec3, dir: Vec3, min: Vec3, max: Vec3) -> Option<f32> {
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
