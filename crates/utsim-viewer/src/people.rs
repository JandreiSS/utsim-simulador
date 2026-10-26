//! Skeletal people: the CC0 mannequin of the Universal Animation Library (catalogue
//! entries `pessoa_operador` and `pessoa_operador_carga`, see `assets/terceiros`).
//!
//! The mannequin of the first file is spawned like any glTF scene ([`crate::models`]);
//! the clips come from both files. They share the skeleton and the bone names, and
//! Bevy identifies an animation target by the path of names from the animation root, so
//! a clip of the second file drives the bones of the first one as is (checked when the
//! graph is built: every target of every clip used must exist in the spawned skeleton).
//!
//! One [`AnimationGraph`] is shared by every person: a clip node per
//! [`visual::Clip`], plus the two masked nodes of `IdleCarry` (the legs of the idle
//! clip under the arms of the carrying clip, since the library has no standing-with-a-
//! box clip). Every node stays paused: each frame [`drive`] sets its weight (a
//! crossfade of [`visual::FADE`] in real time) and its time, which comes from the
//! scene ([`visual::clip_time`]: distance walked, simulated clock, load progress), so
//! jumping to `t` gives the pose of `t`.
//!
//! The mannequin is untextured: its materials are tinted to work clothes ([`clothes`])
//! and a helmet and a high-visibility vest are attached to the head and the middle
//! spine bone (`spine_02`: the chest mesh follows it more closely than `spine_03`, which
//! pitches 15 degrees further in most clips).
//! The carried item is held between the hands ([`Figure::hold`], read after the
//! transforms are propagated). When the files are missing (the browser build) or fail
//! to load, people fall back to the procedural `operador_simples`.

use std::sync::OnceLock;

use bevy::asset::LoadState;
use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::gltf::Gltf;
use bevy::math::Affine3A;
use bevy::prelude::*;
use utsim_scene::Motion;
use utsim_viewer::terceiros;
use utsim_viewer::visual::{self, Clip, Source};

use crate::editor::{Editor, Mode};
use crate::models::{Library, Realistic, Tint, VisualMode};
use crate::playback::{Data, Playback};

/// Mask group of the legs and the pelvis.
const LOWER: u32 = 0;
/// Mask group of the spine, arms and head.
const UPPER: u32 = 1;

/// Work clothes of the mannequin (sRGB), close to the procedural operator's.
const SUIT: [f32; 3] = [0.231, 0.431, 0.647];
const JOINTS: [f32; 3] = [0.17, 0.24, 0.31];
const HELMET: [f32; 3] = [1.0, 0.82, 0.0];
const VEST: [f32; 3] = [0.96, 0.49, 0.0];
const STRIPE: [f32; 3] = [0.87, 0.89, 0.91];

/// Animation state of one skeletal person.
#[derive(Default)]
pub struct Figure {
    weights: [f32; 7],
    /// The entity with the `AnimationPlayer` (the skeleton's root), once bound.
    player: Option<Entity>,
    hands: Option<(Entity, Entity)>,
    /// Midpoint of the hands in the person's root frame (metres, `+X` ahead, `+Y` up),
    /// from the last propagated frame.
    pub hold: Option<Vec3>,
}

/// The shared clips and graph.
#[derive(Resource, Default)]
pub struct People {
    gltf: Option<[Handle<Gltf>; 2]>,
    graph: Option<Handle<AnimationGraph>>,
    /// Graph nodes of each clip (two for `IdleCarry`: legs, then arms).
    nodes: Vec<Vec<AnimationNodeIndex>>,
    durations: [f32; 7],
    /// The files are missing or did not load: people use the procedural operator.
    pub failed: bool,
    props: Option<Props>,
}

struct Props {
    helmet: Handle<Mesh>,
    brim: Handle<Mesh>,
    crown: Handle<Mesh>,
    vest: Handle<Mesh>,
    stripe: Handle<Mesh>,
    yellow: Handle<StandardMaterial>,
    orange: Handle<StandardMaterial>,
    grey: Handle<StandardMaterial>,
}

fn entry_source(name: &str) -> Option<Source> {
    terceiros::by_name(name).map(|e| Source::File(terceiros::file_of(e)))
}

/// Whether both files of the mannequin are on disk (never in the browser).
fn files_present() -> bool {
    static PRESENT: OnceLock<bool> = OnceLock::new();
    *PRESENT.get_or_init(|| {
        !cfg!(target_arch = "wasm32")
            && [visual::PERSON, visual::PERSON_CLIPS]
                .iter()
                .all(|n| terceiros::by_name(n).is_some_and(|e| terceiros::file_of(e).is_file()))
    })
}

impl People {
    /// People get the skeletal mannequin (else the procedural operator).
    pub fn available(&self) -> bool {
        files_present() && !self.failed
    }
}

/// The tint of the mannequin's two materials.
pub fn clothes() -> Tint {
    Tint(vec![("M_Main", SUIT), ("M_Joints", JOINTS)])
}

/// Starts loading the clips once a person is shown, builds the graph once they and a
/// skeleton are there, and gives up (procedural operator) when a file fails.
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn prepare(
    mut people: ResMut<People>,
    mut library: ResMut<Library>,
    registry: Res<EmbeddedAssetRegistry>,
    server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    clips: Res<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    figures: Query<&Realistic>,
    children: Query<&Children>,
    targets: Query<(&Name, &bevy::animation::AnimationTargetId, &ChildOf)>,
    names: Query<&Name>,
) {
    if people.failed || people.graph.is_some() {
        return;
    }
    let Some(first) = figures.iter().find(|r| r.person.is_some()) else {
        return;
    };
    let (Some(a), Some(b)) = (
        entry_source(visual::PERSON),
        entry_source(visual::PERSON_CLIPS),
    ) else {
        people.failed = true;
        return;
    };
    if library.failed(&a, &server) {
        warn!("pessoa: o boneco não carregou; usando o operador simples");
        people.failed = true;
        return;
    }
    if people.gltf.is_none() {
        match (
            library.gltf(&a, &registry, &server),
            library.gltf(&b, &registry, &server),
        ) {
            (Some(ga), Some(gb)) => people.gltf = Some([ga, gb]),
            _ => {
                people.failed = true;
                return;
            }
        }
    }
    let [ga, gb] = people.gltf.clone().expect("set above");
    if [&ga, &gb]
        .iter()
        .any(|h| matches!(server.load_state(h.id()), LoadState::Failed(_)))
    {
        warn!("pessoa: os clipes não carregaram; usando o operador simples");
        people.failed = true;
        return;
    }
    let (Some(ga), Some(gb)) = (gltfs.get(&ga), gltfs.get(&gb)) else {
        return;
    };
    // The bones of a spawned skeleton, by mask group.
    let mut bones = Vec::new();
    for e in children.iter_descendants(first.asset) {
        if let Ok((name, id, parent)) = targets.get(e) {
            bones.push((
                *id,
                group_of(name.as_str(), parent.parent(), &targets, &names),
            ));
        }
    }
    if bones.is_empty() {
        return; // not spawned yet
    }
    let mut handles = Vec::new();
    for clip in Clip::ALL {
        let (name, second) = clip.source();
        let gltf = if second { gb } else { ga };
        let Some(h) = gltf.named_animations.get(name) else {
            warn!("pessoa: clipe {name} não encontrado; usando o operador simples");
            people.failed = true;
            return;
        };
        handles.push(h.clone());
    }
    if handles.iter().any(|h| clips.get(h).is_none()) {
        return;
    }
    // Retargeting by bone name: every target of every clip is a bone of this skeleton.
    let known: std::collections::HashSet<_> = bones.iter().map(|(id, _)| *id).collect();
    for (clip, h) in Clip::ALL.iter().zip(&handles) {
        let curves = clips.get(h).expect("checked").curves();
        let missing = curves.keys().filter(|id| !known.contains(id)).count();
        if missing > 0 {
            warn!(
                "pessoa: {missing} de {} alvos do clipe {} fora do esqueleto",
                curves.len(),
                clip.source().0
            );
        }
    }
    let mut graph = AnimationGraph::new();
    let root = graph.root;
    let mut nodes = Vec::new();
    for (clip, h) in Clip::ALL.iter().zip(&handles) {
        nodes.push(if *clip == Clip::IdleCarry {
            let idle = handles[Clip::Idle.index()].clone();
            vec![
                graph.add_clip_with_mask(idle, 1 << UPPER, 1.0, root),
                graph.add_clip_with_mask(h.clone(), 1 << LOWER, 1.0, root),
            ]
        } else {
            vec![graph.add_clip(h.clone(), 1.0, root)]
        });
        people.durations[clip.index()] = clips.get(h).expect("checked").duration();
    }
    for (id, group) in bones {
        if let Some(g) = group {
            graph.add_target_to_mask_group(id, g);
        }
    }
    people.nodes = nodes;
    people.graph = Some(graphs.add(graph));
    info!("pessoa: clipes do esqueleto prontos");
}

/// Mask group of the bone `name` whose parent is `parent`: the legs and the pelvis, or
/// the spine with everything above it; `None` for the root bones.
fn group_of(
    name: &str,
    parent: Entity,
    targets: &Query<(&Name, &bevy::animation::AnimationTargetId, &ChildOf)>,
    names: &Query<&Name>,
) -> Option<u32> {
    let classify = |n: &str| {
        if n.starts_with("spine_") {
            Some(UPPER)
        } else if n == "pelvis" || n.starts_with("thigh_") {
            Some(LOWER)
        } else {
            None
        }
    };
    if let Some(g) = classify(name) {
        return Some(g);
    }
    let mut at = Some(parent);
    while let Some(e) = at {
        if let Some(g) = names.get(e).ok().and_then(|n| classify(n.as_str())) {
            return Some(g);
        }
        at = targets.get(e).ok().map(|(_, _, p)| p.parent());
    }
    None
}

/// Binds every spawned skeleton to the graph and dresses it (helmet and vest).
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn bind(
    mut commands: Commands,
    mut people: ResMut<People>,
    mut figures: Query<&mut Realistic>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
    nodes: Query<(&Name, &Transform, &ChildOf)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(graph) = people.graph.clone() else {
        return;
    };
    if people.props.is_none() {
        let mat = |rgb: [f32; 3], rough: f32| StandardMaterial {
            base_color: Color::srgb(rgb[0], rgb[1], rgb[2]),
            perceptual_roughness: rough,
            ..default()
        };
        people.props = Some(Props {
            helmet: meshes.add(ConicalFrustum {
                radius_top: 0.075,
                radius_bottom: 0.118,
                height: 0.085,
            }),
            brim: meshes.add(Cylinder::new(0.122, 0.012)),
            crown: meshes.add(Sphere::new(0.075)),
            vest: meshes.add(Cuboid::new(0.355, 0.34, 0.27)),
            stripe: meshes.add(Cuboid::new(0.365, 0.035, 0.28)),
            yellow: materials.add(mat(HELMET, 0.4)),
            orange: materials.add(mat(VEST, 0.7)),
            grey: materials.add(mat(STRIPE, 0.3)),
        });
    }
    let props = people.props.as_ref().expect("set above");
    for mut real in &mut figures {
        let asset = real.asset;
        let Some(fig) = real.person.as_mut() else {
            continue;
        };
        if fig.player.is_some() {
            continue;
        }
        let find = |name: &str| {
            children
                .iter_descendants(asset)
                .find(|e| nodes.get(*e).is_ok_and(|(n, _, _)| n.as_str() == name))
        };
        let Some(player) = children
            .iter_descendants(asset)
            .find(|e| players.contains(*e))
        else {
            continue;
        };
        let (Some(hand_r), Some(hand_l), Some(head), Some(chest)) = (
            find("hand_r"),
            find("hand_l"),
            find("Head"),
            find("spine_02"),
        ) else {
            continue;
        };
        if let Ok(mut p) = players.get_mut(player) {
            for list in &people.nodes {
                for &n in list {
                    p.start(n).set_weight(0.0).pause();
                }
            }
        }
        commands
            .entity(player)
            .insert(AnimationGraphHandle(graph.clone()));
        // Props, placed in the rest pose (file units, Y up, facing +Z).
        let rest = |bone: Entity| rest_transform(bone, asset, &nodes);
        let attach = |commands: &mut Commands, bone: Entity, at: Vec3, scale: Vec3, mesh, mat| {
            let local = rest(bone).inverse()
                * Affine3A::from_scale_rotation_translation(scale, Quat::IDENTITY, at);
            commands.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(mat),
                Transform::from_matrix(Mat4::from(local)),
                ChildOf(bone),
            ));
        };
        let (y, o, g) = (
            props.yellow.clone(),
            props.orange.clone(),
            props.grey.clone(),
        );
        attach(
            &mut commands,
            head,
            Vec3::new(0.0, 1.778, 0.008),
            Vec3::ONE,
            props.helmet.clone(),
            y.clone(),
        );
        attach(
            &mut commands,
            head,
            Vec3::new(0.0, 1.818, 0.008),
            Vec3::new(1.0, 0.45, 1.0),
            props.crown.clone(),
            y.clone(),
        );
        attach(
            &mut commands,
            head,
            Vec3::new(0.0, 1.738, 0.03),
            Vec3::ONE,
            props.brim.clone(),
            y,
        );
        attach(
            &mut commands,
            chest,
            Vec3::new(0.0, 1.235, 0.0),
            Vec3::ONE,
            props.vest.clone(),
            o,
        );
        attach(
            &mut commands,
            chest,
            Vec3::new(0.0, 1.17, 0.0),
            Vec3::ONE,
            props.stripe.clone(),
            g.clone(),
        );
        attach(
            &mut commands,
            chest,
            Vec3::new(0.0, 1.30, 0.0),
            Vec3::ONE,
            props.stripe.clone(),
            g,
        );
        fig.player = Some(player);
        fig.hands = Some((hand_l, hand_r));
    }
}

/// Rest transform of `bone` in the frame of `asset` (the glTF scene root's parent):
/// the product of the local transforms up the hierarchy.
fn rest_transform(
    bone: Entity,
    asset: Entity,
    nodes: &Query<(&Name, &Transform, &ChildOf)>,
) -> Affine3A {
    let mut m = Affine3A::IDENTITY;
    let mut at = bone;
    while at != asset {
        let Ok((_, tf, parent)) = nodes.get(at) else {
            break;
        };
        m = tf.compute_affine() * m;
        at = parent.parent();
    }
    m
}

/// Plays the clip of each person's motion (see the module docs).
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn drive(
    editor: Res<Editor>,
    data: Res<Data>,
    pb: Res<Playback>,
    time: Res<Time<Real>>,
    mode: Res<VisualMode>,
    people: Res<People>,
    mut figures: Query<&mut Realistic>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if !mode.active() || people.graph.is_none() {
        return;
    }
    let sim = editor.mode == Mode::Sim;
    let dt = time.delta_secs();
    for mut real in &mut figures {
        let b = real.block;
        let scale = real.look.scale;
        let pose = if sim {
            crate::models::drawn_pose(&data, b, pb.t, &real.look)
        } else {
            None
        };
        let Some(fig) = real.person.as_mut() else {
            continue;
        };
        let Some(Ok(mut player)) = fig.player.map(|p| players.get_mut(p)) else {
            continue;
        };
        let (motion, carrying) = match pose {
            Some(p) => (
                p.motion,
                p.lift > 0.0 || data.timeline.block_content(b, pb.t) > 0,
            ),
            None => (Motion::Idle, false),
        };
        let progress = match motion {
            Motion::Loading { progress } | Motion::Unloading { progress } => progress,
            _ => 0.0,
        };
        let clip = visual::person_clip(motion, carrying);
        visual::fade(&mut fig.weights, clip, dt);
        let distance = if sim {
            data.distance_at(b, pb.t) as f32
        } else {
            0.0
        };
        let clock = if sim { pb.t } else { time.elapsed_secs_f64() };
        for c in Clip::ALL {
            let w = fig.weights[c.index()];
            let Some(list) = people.nodes.get(c.index()) else {
                continue;
            };
            for (k, &node) in list.iter().enumerate() {
                // The arms of IdleCarry hold still at the start of the carrying clip.
                let at = match (c, k) {
                    (Clip::IdleCarry, 0) => visual::clip_time(
                        Clip::Idle,
                        people.durations[Clip::Idle.index()],
                        distance,
                        scale,
                        clock,
                        progress,
                    ),
                    (Clip::IdleCarry, _) => 0.0,
                    _ => visual::clip_time(
                        c,
                        people.durations[c.index()],
                        distance,
                        scale,
                        clock,
                        progress,
                    ),
                };
                if let Some(a) = player.animation_mut(node) {
                    a.set_weight(w).seek_to(at);
                }
            }
        }
    }
}

/// Reads where the hands are, once the transforms are propagated (`PostUpdate`).
pub fn track_hands(
    mut figures: Query<(&mut Realistic, &GlobalTransform)>,
    globals: Query<&GlobalTransform, Without<Realistic>>,
) {
    for (mut real, root) in &mut figures {
        let Some(fig) = real.person.as_mut() else {
            continue;
        };
        let Some((l, r)) = fig.hands else {
            continue;
        };
        let (Ok(l), Ok(r)) = (globals.get(l), globals.get(r)) else {
            continue;
        };
        let mid = (l.translation() + r.translation()) / 2.0;
        fig.hold = Some(root.affine().inverse().transform_point3(mid));
    }
}

/// Centre of an item of edge `size` held by a person whose hands' midpoint is `hold`
/// (root frame): between the hands, but never inside the chest.
pub fn held_center(hold: Vec3, size: f32) -> Vec3 {
    Vec3::new(hold.x.max(size / 2.0 + 0.04), hold.y, hold.z)
}
