//! The 3D scene: blocks, connections, items, ground grid and picking.

use std::collections::{HashMap, HashSet};
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use utsim_viewer::editing::NetworkView;
use utsim_viewer::geometry::{self, Geom};
use utsim_viewer::handling::{BEAM, BELT_THICKNESS, BELT_WIDTH};
use utsim_viewer::style::{self, Shape};
use utsim_viewer::view;

use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use utsim_viewer::visual::{self, Carry, Look};

use crate::dock::{Dock, pointer_over_ui};
use crate::editor::{Editor, Mode};
use crate::models::{self, Library, VisualMode};
use crate::playback::{Data, FluidSelection, Playback, Selection};

#[derive(Component)]
pub struct BlockVisual;

#[derive(Component)]
pub struct ItemVisual;

use utsim_scene::compose::ITEM_SIZE;

/// Visuals of the blocks currently shown (rebuilt by [`sync_blocks`]).
#[derive(Resource, Default)]
pub struct BlockMaterials {
    handles: Vec<Handle<StandardMaterial>>,
    last: Vec<Option<Option<utsim_core::State>>>,
    kinds: Vec<String>,
    /// Geometry the entities were built for (a change rebuilds them).
    geoms: Vec<Geom>,
    entities: Vec<Entity>,
    /// Height of each entity's origin above the block base (half the box for the simple
    /// shapes, 0 for the realistic models, whose origin is on the floor).
    offsets: Vec<f32>,
    /// Realistic look of each block, when drawn with one.
    looks: Vec<Option<Look>>,
    /// How each simple shape is turned (`visual.yaw` of a fixed object; a change rebuilds).
    turns: Vec<f32>,
}

#[derive(Resource)]
pub struct ItemPool {
    entities: HashMap<usize, Entity>,
    mesh: Handle<Mesh>,
    materials: HashMap<usize, Handle<StandardMaterial>>,
    /// Realistic items (pallets and boxes) instead of cubes.
    scenes: Option<models::ItemScenes>,
    /// The scene moves pallet loads.
    pallets: bool,
    /// Edge of a load carried on forks (the rack's load size, so it does not change size
    /// when it is stored).
    fork_size: f32,
}

fn color(rgb: [f32; 3]) -> Color {
    Color::srgb(rgb[0], rgb[1], rgb[2])
}

/// Base position of block `b` at time `t` (Bevy space): its layout position, or, for a
/// mobile resource that has started travelling, where the trace says it is. Composed by
/// the engine-independent scene ([`utsim_scene::Scene::base_at`]).
pub fn block_base_at(data: &Data, b: usize, t: f64) -> Vec3 {
    Vec3::from(data.base_at(b, t))
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

/// Spawns the visual of one block at `translation` (its base plus half its height), turned
/// by `turn` about the vertical (a fixed object's `visual.yaw`; a belt follows its `geom.yaw`).
fn spawn_block(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    geom: &Geom,
    material: Handle<StandardMaterial>,
    translation: Vec3,
    turn: f32,
) -> Entity {
    let steel = materials.add(StandardMaterial {
        base_color: Color::srgb(0.27, 0.29, 0.34),
        perceptual_roughness: 0.7,
        ..default()
    });
    match (geom.shape, geom.belt_params(), geom.rack_params()) {
        // A belt drawn by nodes: one slab per piece of its route, along it (the route is
        // the top of the belt, relative to the base), and a leg to the floor under each
        // point of the route that stands above it.
        (Shape::Belt, Some(_), _) if geom.route.as_ref().is_some_and(|l| l.len() >= 2) => {
            let line = geom.route.clone().unwrap_or_default();
            let down = Vec3::new(0.0, geom.half[1], 0.0);
            let id = commands
                .spawn((
                    Transform::from_translation(translation),
                    Visibility::default(),
                    BlockVisual,
                ))
                .id();
            let leg = meshes.add(Cuboid::new(0.05, 1.0, 0.05));
            commands.entity(id).with_children(|parent| {
                for w in line.windows(2) {
                    let (a, b) = (Vec3::from(w[0]), Vec3::from(w[1]));
                    let d = b - a;
                    let len = d.length();
                    if len < 1e-4 {
                        continue;
                    }
                    let mid = (a + b) / 2.0 - down - Vec3::new(0.0, BELT_THICKNESS / 2.0, 0.0);
                    parent.spawn((
                        Mesh3d(meshes.add(Cuboid::new(len + 0.01, BELT_THICKNESS, BELT_WIDTH))),
                        MeshMaterial3d(material.clone()),
                        Transform {
                            translation: mid,
                            rotation: Quat::from_rotation_arc(Vec3::X, d / len),
                            scale: Vec3::ONE,
                        },
                    ));
                }
                // The floor is `translation.y` below the parent.
                let floor = -translation.y;
                for (k, p) in line.iter().enumerate() {
                    let top = p[1] - geom.half[1] - BELT_THICKNESS;
                    let h = top - floor;
                    let corner = k == 0 || k + 1 == line.len();
                    // A leg at the ends and about every 2 m of the sampled line.
                    if h < 0.15 || !(corner || k % 8 == 0) {
                        continue;
                    }
                    parent.spawn((
                        Mesh3d(leg.clone()),
                        MeshMaterial3d(steel.clone()),
                        Transform {
                            translation: Vec3::new(p[0], floor + h / 2.0, p[2]),
                            rotation: Quat::IDENTITY,
                            scale: Vec3::new(1.0, h, 1.0),
                        },
                    ));
                }
            });
            id
        }
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
                    Transform::from_translation(translation)
                        .with_rotation(Quat::from_rotation_y(turn)),
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
                        rotation: Quat::from_rotation_y(turn) * block_rotation(shape),
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
    visual: Res<VisualMode>,
    mut library: ResMut<Library>,
    registry: Res<EmbeddedAssetRegistry>,
    server: Res<AssetServer>,
    people: Res<crate::people::People>,
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
    // The realistic looks follow the document (visual, roles) as well as the geometry.
    let looks = if visual.active() {
        models::looks(&editor, &data, people.available())
    } else {
        Vec::new()
    };
    let wanted = |i: usize| looks.get(i).cloned().flatten();
    // How each simple shape is turned: a fixed object's `visual.yaw` (a belt already
    // follows its `geom.yaw`, the mobiles their heading).
    let turns: Vec<f32> = match editor.mode {
        Mode::Edit => {
            let model = editor.doc.model();
            (0..geoms.len())
                .map(|i| {
                    let kind = editor.view.kinds.get(i).copied().unwrap_or("");
                    if kind == "Transporter" || geoms[i].shape == Shape::Belt {
                        return 0.0;
                    }
                    model
                        .objects
                        .iter()
                        .find(|o| Some(&o.name) == editor.view.names.get(i))
                        .and_then(|o| o.visual.as_ref())
                        .and_then(|v| v.yaw)
                        .filter(|y| y.is_finite())
                        .map_or(0.0, |y| (y as f32).to_radians())
                })
                .collect()
        }
        Mode::Sim => (0..geoms.len())
            .map(|i| {
                if geoms[i].shape == Shape::Belt {
                    0.0
                } else {
                    data.turn(i)
                }
            })
            .collect(),
    };
    let rebuild = *last_mode != Some(editor.mode)
        || (editor.mode == Mode::Sim && data.is_changed())
        || visual.is_changed()
        || mats
            .kinds
            .iter()
            .map(String::as_str)
            .ne(kinds.iter().copied())
        || mats.geoms.as_slice() != geoms
        || mats.turns != turns
        || (0..geoms.len()).any(|i| mats.looks.get(i).cloned().flatten() != wanted(i));
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
        mats.offsets.clear();
        mats.looks.clear();
        mats.kinds = kinds.iter().map(|k| k.to_string()).collect();
        mats.geoms = geoms.to_vec();
        mats.turns = turns.clone();
        for (i, geom) in geoms.iter().enumerate() {
            let material = materials.add(StandardMaterial {
                base_color: color(style::state_color(None)),
                perceptual_roughness: 0.6,
                ..default()
            });
            mats.handles.push(material.clone());
            mats.last.push(None);
            let base = Vec3::from(positions[i]);
            let look = wanted(i);
            let realistic = look.as_ref().and_then(|look| {
                models::spawn_block(
                    &mut commands,
                    &mut library,
                    &registry,
                    &server,
                    i,
                    look,
                    base,
                )
            });
            let id = match realistic {
                Some(id) => {
                    mats.offsets.push(0.0);
                    mats.looks.push(look);
                    id
                }
                None => {
                    mats.offsets.push(geom.half[1]);
                    mats.looks.push(None);
                    spawn_block(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        geom,
                        material,
                        base + Vec3::Y * geom.half[1],
                        turns[i],
                    )
                }
            };
            if kinds[i] == "Flow" || kinds[i] == "Fluid" {
                // Token flows live in the Fluxos panel, not in the 3D scene.
                commands.entity(id).insert(Visibility::Hidden);
            }
            mats.entities.push(id);
        }
        // Items: realistic loads when the blocks are realistic.
        pool.pallets = looks.iter().flatten().any(|l| {
            matches!(
                l.source,
                visual::Source::Library(
                    utsim_assets::Spec::PortaPaletes(_)
                        | utsim_assets::Spec::Empilhadeira
                        | utsim_assets::Spec::Paleteira
                )
            )
        });
        pool.scenes = if visual.active() {
            models::ItemScenes::get(&mut library, &registry, &server)
        } else {
            None
        };
        pool.fork_size = geoms
            .iter()
            .filter_map(|g| g.rack_params())
            .map(|p| p.item_size())
            .fold(None, |a: Option<f32>, s| Some(a.map_or(s, |a| a.max(s))))
            .unwrap_or(0.8);
        return;
    }
    for (i, &e) in mats.entities.iter().enumerate() {
        if let Ok(mut tr) = transforms.get_mut(e) {
            tr.translation = Vec3::from(positions[i]) + Vec3::Y * mats.offsets[i];
        }
    }
}

/// The sun of the scene (its shadows follow [`Lighting`]).
#[derive(Component)]
pub struct Sun;

/// The floor of the realistic look: receives the shadows.
#[derive(Component)]
pub struct Ground;

/// Lighting choices of the realistic look ("Sombras" next to the "Visual" switch).
#[derive(Resource)]
pub struct Lighting {
    /// Shadows of the sun on the floor and on the models. Off by default in the
    /// browser (WebGL2), where the shadow passes cost the most.
    pub shadows: bool,
}

impl Default for Lighting {
    fn default() -> Self {
        Lighting {
            shadows: !cfg!(target_arch = "wasm32"),
        }
    }
}

/// The "Sombras" switch.
pub fn shadows_toggle(ui: &mut bevy_egui::egui::Ui, lighting: &mut ResMut<Lighting>) {
    let mut on = lighting.shadows;
    if ui
        .checkbox(&mut on, "Sombras")
        .on_hover_text("Sombras do sol sobre o piso e os modelos (pode pesar em máquinas fracas)")
        .changed()
    {
        lighting.shadows = on;
    }
}

pub fn setup(
    mut commands: Commands,
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
        Sun,
    ));
    commands.spawn((
        Camera3d::default(),
        AmbientLight {
            brightness: 350.0,
            ..default()
        },
    ));
    // A unit square scaled to the scene by `update_lighting`.
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.31, 0.32),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.004, 0.0),
        Visibility::Hidden,
        Ground,
    ));

    commands.insert_resource(BlockMaterials::default());
    commands.insert_resource(ItemPool {
        entities: HashMap::new(),
        mesh: meshes.add(Cuboid::new(ITEM_SIZE, ITEM_SIZE, ITEM_SIZE)),
        materials: HashMap::new(),
        scenes: None,
        pallets: false,
        fork_size: 0.8,
    });
}

/// The parts of the sun that [`update_lighting`] changes.
type SunParts = (
    &'static mut DirectionalLight,
    &'static mut bevy::light::CascadeShadowConfig,
    &'static mut Transform,
);

/// Ground and sun of the realistic look: the floor covers the scene with a margin, the
/// shadow cascades reach from the camera to the far side of the scene, and the ambient
/// light is softer with shadows on (the shaded side is lit by the sky only).
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn update_lighting(
    editor: Res<Editor>,
    data: Res<Data>,
    visual: Res<VisualMode>,
    lighting: Res<Lighting>,
    mut ground: Single<(&mut Transform, &mut Visibility), With<Ground>>,
    mut sun: Single<SunParts, (With<Sun>, Without<Ground>)>,
    camera: Single<(&GlobalTransform, &mut AmbientLight), With<Camera3d>>,
) {
    let positions: &[[f32; 3]] = match editor.mode {
        Mode::Edit => &editor.view.positions,
        Mode::Sim => &data.positions,
    };
    let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for p in positions
        .iter()
        .chain(data.fluid.nodes.iter().map(|n| &n.pos))
    {
        min = min.min(Vec3::from(*p));
        max = max.max(Vec3::from(*p));
    }
    if min.x > max.x {
        (min, max) = (Vec3::ZERO, Vec3::ZERO);
    }
    let center = (min + max) / 2.0;
    let span = (max - min).with_y(0.0).length() + 30.0;
    let realistic = visual.active();
    let (gt, gv) = &mut *ground;
    gt.translation = Vec3::new(center.x, -0.004, center.z);
    gt.scale = Vec3::new(max.x - min.x + 30.0, 1.0, max.z - min.z + 30.0);
    **gv = if realistic {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    let on = realistic && lighting.shadows;
    let (light, cascades, tf) = &mut *sun;
    if light.shadow_maps_enabled != on {
        light.shadow_maps_enabled = on;
    }
    // The sun keeps its direction; its position only matters to the gizmos.
    tf.translation = center + Vec3::new(8.0, 14.0, 6.0);
    **tf = tf.looking_at(center, Vec3::Y);
    let (cam, mut ambient) = camera.into_inner();
    let target = if on { 520.0 } else { 350.0 };
    if ambient.brightness != target {
        ambient.brightness = target;
    }
    if on {
        let reach = cam.translation().distance(center) + span / 2.0;
        let config = bevy::light::CascadeShadowConfigBuilder {
            num_cascades: if cfg!(target_arch = "wasm32") { 2 } else { 3 },
            minimum_distance: 0.1,
            maximum_distance: reach.max(20.0),
            first_cascade_far_bound: (reach / 6.0).max(8.0),
            overlap_proportion: 0.2,
        }
        .build();
        **cascades = config;
    }
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
            let offset = mats
                .offsets
                .get(i)
                .copied()
                .unwrap_or(data.geoms[i].half[1]);
            tr.translation = block_base_at(&data, i, pb.t) + Vec3::Y * offset;
        }
    }
}

/// Where a drawn item goes in the realistic mode: on the forks or in the hands of its
/// carrier, on an AGV's deck, on the floor of a queue area; elsewhere where the scene
/// puts it. Returns the centre and the cube scale.
fn realistic_placement(
    data: &Data,
    t: f64,
    mats: &BlockMaterials,
    fork_size: f32,
    p: &utsim_scene::ItemPlacement,
    stacked: &mut HashMap<usize, usize>,
    holds: &HashMap<usize, Vec3>,
) -> (Vec3, f32) {
    let center = Vec3::from(p.center);
    let Some(utsim_scene::EntityId::Block(b)) = p.parent else {
        return (center, p.scale);
    };
    let b = b as usize;
    let Some(look) = mats.looks.get(b).and_then(Option::as_ref) else {
        return (center, p.scale);
    };
    if mats.kinds.get(b).is_some_and(|k| k == "Queue") {
        return (center - Vec3::Y * visual::queue_drop(), p.scale);
    }
    let Some(pose) = models::drawn_pose(data, b, t, look) else {
        return (center, p.scale);
    };
    let scale = match look.carry {
        Carry::Forks { .. } => fork_size / ITEM_SIZE,
        _ => p.scale,
    };
    // A skeletal person holds it between the hands.
    if let Some(&hold) = holds.get(&b) {
        let index = stacked.entry(b).or_default();
        let size = scale * ITEM_SIZE;
        let local = crate::people::held_center(hold, size) + Vec3::Y * (*index as f32 * size);
        *index += 1;
        let root = Transform::from_translation(Vec3::from(pose.position))
            .with_rotation(Quat::from_rotation_y(pose.yaw));
        return (root.transform_point(local), scale);
    }
    let index = stacked.entry(b).or_default();
    let spot = visual::carried_center(
        look.carry,
        pose.position,
        pose.yaw,
        pose.lift,
        *index,
        scale * ITEM_SIZE,
    );
    *index += 1;
    match spot {
        Some(c) => (Vec3::from(c), scale),
        None => (center, p.scale),
    }
}

#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn update_items(
    mut commands: Commands,
    data: Res<Data>,
    pb: Res<Playback>,
    mats: Res<BlockMaterials>,
    mut pool: ResMut<ItemPool>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<&mut Transform, With<ItemVisual>>,
    figures: Query<&models::Realistic>,
) {
    let tl = &data.timeline;
    let realistic = pool.scenes.is_some();
    let holds: HashMap<usize, Vec3> = figures
        .iter()
        .filter_map(|r| Some((r.block, r.person.as_ref()?.hold?)))
        .collect();
    // Where every item is comes from the scene; the viewer only draws it (the realistic
    // models only move carried loads onto the forks, the hands or the deck).
    let mut stacked = HashMap::new();
    let placed: Vec<(usize, Vec3, f32, f32)> = data
        .items_at(pb.t)
        .into_iter()
        .filter(|p| p.drawn)
        .map(|p| {
            let (pos, scale) = if realistic {
                realistic_placement(&data, pb.t, &mats, pool.fork_size, &p, &mut stacked, &holds)
            } else {
                (Vec3::from(p.center), p.scale)
            };
            (p.item, pos, scale, p.yaw)
        })
        .collect();

    let mut visible: HashSet<usize> = HashSet::with_capacity(placed.len());
    for (i, pos, scale, yaw) in placed {
        visible.insert(i);
        let mut transform = Transform::from_translation(pos).with_scale(Vec3::splat(scale));
        if realistic {
            transform.rotation = Quat::from_rotation_y(yaw);
        }
        if let Some(&e) = pool.entities.get(&i) {
            if let Ok(mut tr) = transforms.get_mut(e) {
                *tr = transform;
            }
        } else if let Some(scenes) = &pool.scenes {
            let origin = tl.item_origin(i).unwrap_or(0);
            let e = commands
                .spawn((transform, Visibility::default(), ItemVisual))
                .id();
            models::spawn_item_children(
                &mut commands,
                e,
                scenes,
                pool.pallets,
                style::origin_color(origin),
            );
            pool.entities.insert(i, e);
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
    let mut positions: Vec<[f32; 3]> = (0..data.positions.len())
        .map(|b| block_base_at(&data, b, pb.t).to_array())
        .collect();
    // Fluid nodes only widen the ground grid (they come after every block, so the
    // block indices of the connections and outlines are unchanged).
    positions.extend(data.fluid.nodes.iter().map(|n| n.pos));
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
    // Controlled edges (AGV capacity) are red and drawn as a double line.
    let controlled = Color::srgb(1.0, 0.30, 0.25);
    let lift = Vec3::Y * 0.03;
    for seg in &network.segments {
        let (a, b) = (Vec3::from(seg.a) + lift, Vec3::from(seg.b) + lift);
        let color = if seg.capacity.is_some() {
            controlled
        } else {
            color
        };
        if seg.two_way {
            gizmos.line(a, b, color);
        } else {
            gizmos.arrow(a, b, color).with_tip_length(0.4);
        }
        if seg.capacity.is_some() {
            let side = (b - a).cross(Vec3::Y).normalize_or_zero() * 0.12;
            gizmos.line(a + side, b + side, color);
            gizmos.line(a - side, b - side, color);
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
    mut fluid_sel: ResMut<FluidSelection>,
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
    for i in (0..data.replay.blocks.len()).filter(|&i| !data.is_hidden(i)) {
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
    let mut best_fluid: Option<(f32, usize)> = None;
    for (i, node) in data.fluid.nodes.iter().enumerate() {
        let h = utsim_viewer::fluids::node_half(node);
        let center = Vec3::from(node.pos) + Vec3::Y * h[1];
        if let Some(d) = ray_box(
            ray.origin,
            *ray.direction,
            center - Vec3::from(h),
            center + Vec3::from(h),
        ) && best_fluid.is_none_or(|(bd, _)| d < bd)
        {
            best_fluid = Some((d, i));
        }
    }
    // The closest hit wins; a click selects either a block or a fluid node.
    let fluid_wins = match (best, best_fluid) {
        (Some((d, _)), Some((fd, _))) => fd < d,
        (None, Some(_)) => true,
        _ => false,
    };
    if fluid_wins {
        selection.0 = None;
        fluid_sel.0 = best_fluid.map(|(_, i)| i);
    } else {
        selection.0 = best.map(|(_, i)| i);
        fluid_sel.0 = None;
    }
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
