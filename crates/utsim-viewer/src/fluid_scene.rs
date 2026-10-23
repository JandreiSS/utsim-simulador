//! Fluid networks (phase 7) in the 3D scene: tanks that fill and drain, small markers
//! for the other nodes, pipes as arrows and dashed lines to the discrete objects that
//! the converters read from or write to. Playback only (`Simular`).

use bevy::prelude::*;
use utsim_viewer::fluids::{self, FluidNodeView, NodeShape};
use utsim_viewer::view;

use crate::editor::{Editor, Mode};
use crate::playback::{Data, FluidSelection, Playback};
use crate::scene::block_base_at;

/// Entities of the fluid network currently shown.
#[derive(Resource, Default)]
pub struct FluidScene {
    entities: Vec<Entity>,
    /// `(liquid entity, node index)` of every tank.
    liquids: Vec<(Entity, usize)>,
}

const SHELL: Color = Color::srgba(0.75, 0.85, 0.95, 0.22);
const LIQUID: Color = Color::srgb(0.12, 0.45, 0.95);
const LINK_ON: Color = Color::srgb(0.25, 0.85, 0.95);
const LINK_OFF: Color = Color::srgb(0.30, 0.42, 0.52);
const CONVERTER: Color = Color::srgb(0.75, 0.45, 0.95);

fn marker_color(shape: NodeShape) -> Color {
    match shape {
        NodeShape::Source => Color::srgb(0.25, 0.75, 0.35),
        NodeShape::Sink => Color::srgb(0.80, 0.32, 0.32),
        NodeShape::Pipe => Color::srgb(0.95, 0.68, 0.20),
        NodeShape::Converter => CONVERTER,
        NodeShape::Tank => LIQUID,
    }
}

/// Builds the entities of the network when the replay or the mode changes (and removes
/// them in `Editar`, where fluid nodes are not shown).
pub fn sync_fluids(
    mut commands: Commands,
    editor: Res<Editor>,
    data: Res<Data>,
    mut scene: ResMut<FluidScene>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut last_mode: Local<Option<Mode>>,
) {
    if *last_mode == Some(editor.mode) && !(editor.mode == Mode::Sim && data.is_changed()) {
        return;
    }
    *last_mode = Some(editor.mode);
    for e in scene.entities.drain(..) {
        commands.entity(e).despawn();
    }
    scene.liquids.clear();
    if editor.mode != Mode::Sim {
        return;
    }
    for (i, node) in data.fluid.nodes.iter().enumerate() {
        let base = Vec3::from(node.pos);
        match node.shape {
            NodeShape::Tank => {
                let (r, h) = fluids::tank_dims(node.capacity);
                let root = commands
                    .spawn((Transform::from_translation(base), Visibility::default()))
                    .id();
                let shell = commands
                    .spawn((
                        Mesh3d(meshes.add(Cylinder::new(r, h))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: SHELL,
                            alpha_mode: AlphaMode::Blend,
                            perceptual_roughness: 0.2,
                            cull_mode: None,
                            ..default()
                        })),
                        Transform::from_xyz(0.0, h / 2.0, 0.0),
                    ))
                    .id();
                // The liquid is a unit-height cylinder scaled along Y by `update_fluids`.
                let liquid = commands
                    .spawn((
                        Mesh3d(meshes.add(Cylinder::new(r * 0.93, 1.0))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: LIQUID,
                            perceptual_roughness: 0.25,
                            ..default()
                        })),
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ))
                    .id();
                commands.entity(root).add_children(&[shell, liquid]);
                scene.entities.push(root);
                scene.liquids.push((liquid, i));
            }
            shape => {
                let mesh = match shape {
                    NodeShape::Source => meshes.add(Cone::new(0.35, 0.7)),
                    NodeShape::Sink => meshes.add(Cylinder::new(0.38, 0.5)),
                    // A short horizontal pipe section.
                    NodeShape::Pipe => meshes.add(Cylinder::new(0.2, 0.8)),
                    _ => meshes.add(Cuboid::new(0.6, 0.6, 0.6)),
                };
                let rotation = if shape == NodeShape::Pipe {
                    Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
                } else {
                    Quat::IDENTITY
                };
                let id = commands
                    .spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: marker_color(shape),
                            perceptual_roughness: 0.5,
                            ..default()
                        })),
                        Transform {
                            translation: base + Vec3::Y * 0.35,
                            rotation,
                            scale: Vec3::ONE,
                        },
                    ))
                    .id();
                scene.entities.push(id);
            }
        }
    }
}

/// Sets the height of every tank's liquid from its level at the current time.
pub fn update_fluids(
    data: Res<Data>,
    pb: Res<Playback>,
    scene: Res<FluidScene>,
    mut transforms: Query<(&mut Transform, &mut Visibility)>,
) {
    for &(liquid, i) in &scene.liquids {
        let Some(node) = data.fluid.nodes.get(i) else {
            continue;
        };
        let (_, h) = fluids::tank_dims(node.capacity);
        let fill = fluids::fill_fraction(data.fluid_sample(i, pb.t).level, node.capacity);
        let height = fill * (h - 0.06);
        if let Ok((mut tr, mut vis)) = transforms.get_mut(liquid) {
            tr.scale.y = height.max(1e-3);
            tr.translation.y = 0.03 + height / 2.0;
            *vis = if fill > 1e-4 {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// Where a link attaches to a node, pushed to the node's edge in the direction of `toward`.
fn attach(node: &FluidNodeView, toward: Vec3) -> Vec3 {
    let center = Vec3::from(node.pos) + Vec3::Y * fluids::port_height(node);
    let dir = Vec3::new(toward.x - center.x, 0.0, toward.z - center.z).normalize_or_zero();
    let reach = match node.shape {
        NodeShape::Tank => fluids::tank_dims(node.capacity).0,
        _ => 0.4,
    };
    center + dir * reach
}

/// Links, converter lines and the selection outline.
pub fn draw_fluids(
    data: Res<Data>,
    pb: Res<Playback>,
    selection: Res<FluidSelection>,
    mut gizmos: Gizmos,
) {
    let nodes = &data.fluid.nodes;
    const EPS: f64 = 1e-9;
    for &(a, b) in &data.fluid.links {
        let (na, nb) = (&nodes[a], &nodes[b]);
        let (ca, cb) = (
            Vec3::from(na.pos) + Vec3::Y * fluids::port_height(na),
            Vec3::from(nb.pos) + Vec3::Y * fluids::port_height(nb),
        );
        // A link carries flow when the node that is not a tank moves fluid; between
        // tanks, when either one is filling or draining.
        let rate = |i: usize| data.fluid_sample(i, pb.t).rate.abs();
        let pass: Vec<usize> = [a, b]
            .into_iter()
            .filter(|&i| nodes[i].shape != NodeShape::Tank)
            .collect();
        let active = if pass.is_empty() {
            rate(a) > EPS || rate(b) > EPS
        } else {
            pass.iter().any(|&i| rate(i) > EPS)
        };
        let color = if active { LINK_ON } else { LINK_OFF };
        let (s, e) = (attach(na, cb), attach(nb, ca));
        gizmos.arrow(s, e, color).with_tip_length(0.4);
        // Two extra parallel lines make the pipe read thicker than a connection.
        let side = (e - s).cross(Vec3::Y).normalize_or_zero() * 0.06;
        gizmos.line(s + side, e + side, color);
        gizmos.line(s - side, e - side, color);
    }

    for conv in &data.fluid.converters {
        let node = &nodes[conv.node];
        let h = data.geoms[conv.block].half[1];
        let object = block_base_at(&data, conv.block, pb.t) + Vec3::Y * h;
        let own = Vec3::from(node.pos) + Vec3::Y * 0.35;
        let (from, to) = if conv.into_fluid {
            (object, own)
        } else {
            (own, object)
        };
        for (p, q) in view::dashes(from.to_array(), to.to_array(), 0.3, 0.2) {
            gizmos.line(Vec3::from(p), Vec3::from(q), CONVERTER);
        }
        let dir = (to - from).normalize_or_zero();
        gizmos
            .arrow(to - dir * 0.6, to, CONVERTER)
            .with_tip_length(0.3);
    }

    if let Some(node) = selection.0.and_then(|i| nodes.get(i)) {
        let half = fluids::node_half(node);
        gizmos.cube(
            Transform {
                translation: Vec3::from(node.pos) + Vec3::Y * half[1],
                rotation: Quat::IDENTITY,
                scale: Vec3::from(half) * 2.0 + Vec3::splat(0.3),
            },
            Color::WHITE,
        );
    }
}
