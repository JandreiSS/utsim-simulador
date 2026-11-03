//! Realistic models in the 3D scene ("Visual: simples / realista").
//!
//! Each block drawn realistically is a root entity (the [`BlockVisual`] the scene moves)
//! with one child holding a glTF scene of the asset library (`utsim-assets`) or a user
//! `.glb` (`visual.asset`). The library `.glb` bytes are generated in memory by
//! `utsim-assets` (the same bytes as the files committed in `assets/modelos/`) and
//! registered in Bevy's `embedded://` asset source, so the desktop and the browser build
//! load them the same way, with no file access or HTTP. User files are read from disk
//! (desktop only).
//!
//! Once a scene is spawned, its named nodes are bound ([`Rig`]) and driven every frame
//! from the scene pose (`utsim_scene`): the forklift's `carro` at the `lift` height,
//! wheels by the distance travelled, the `luz` status light, the operator's limbs. The
//! numbers come from the pure rules of `utsim_viewer::visual`. The skeletal mannequin of
//! the people is animated by clips instead (`crate::people`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bevy::asset::LoadState;
use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::gltf::{GltfAssetLabel, GltfMaterialName};
use bevy::prelude::*;
use utsim_scene::{MobileRole, Motion};
use utsim_viewer::visual::{self, Context, Look, Source};

use crate::editor::{Editor, Mode};
use crate::playback::{Data, Playback};
use crate::scene::BlockVisual;

/// Which look the scene uses. Starts realistic; falls back to the simple shapes when a
/// library asset fails to load.
#[derive(Resource)]
pub struct VisualMode {
    pub realistic: bool,
    /// False once a library asset failed to load (the toggle is then disabled).
    pub available: bool,
}

impl Default for VisualMode {
    fn default() -> Self {
        VisualMode {
            realistic: true,
            available: true,
        }
    }
}

impl VisualMode {
    pub fn active(&self) -> bool {
        self.realistic && self.available
    }
}

/// glTF scenes loaded so far, by asset key.
#[derive(Resource, Default)]
pub struct Library {
    scenes: HashMap<String, Handle<WorldAsset>>,
    /// Embedded path of every registered `.glb`, by asset key.
    paths: HashMap<String, String>,
    /// Tinted copies of materials, by (original, colour).
    tints: HashMap<(AssetId<StandardMaterial>, [u8; 3]), Handle<StandardMaterial>>,
}

/// A 1 x 1 light grey PNG, served in place of a texture a `.glb` names but that is not
/// on disk.
const GREY_PNG: [u8; 69] = [
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
    0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x78, 0xda, 0x63, 0xd8, 0xb2, 0xe3, 0x00,
    0x00, 0x04, 0x50, 0x02, 0x2d, 0x22, 0xa9, 0x17, 0x28, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e,
    0x44, 0xae, 0x42, 0x60, 0x82,
];

/// File-name-safe key.
fn safe(text: &str) -> String {
    text.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

impl Library {
    /// The glTF scene of `source`, registering its bytes the first time. `None` when a
    /// user file cannot be read (or in the browser, which has no files).
    pub fn scene(
        &mut self,
        source: &Source,
        registry: &EmbeddedAssetRegistry,
        server: &AssetServer,
    ) -> Option<Handle<WorldAsset>> {
        let key = Self::key(source);
        if let Some(h) = self.scenes.get(&key) {
            return Some(h.clone());
        }
        let path = self.register(source, registry)?;
        let handle: Handle<WorldAsset> =
            server.load(GltfAssetLabel::Scene(0).from_asset(format!("embedded://{path}")));
        self.scenes.insert(key, handle.clone());
        Some(handle)
    }

    /// The whole glTF of `source` (its named animations), registering its bytes the
    /// first time; `None` when the file cannot be read.
    pub fn gltf(
        &mut self,
        source: &Source,
        registry: &EmbeddedAssetRegistry,
        server: &AssetServer,
    ) -> Option<Handle<bevy::gltf::Gltf>> {
        let path = self.register(source, registry)?;
        Some(server.load(format!("embedded://{path}")))
    }

    fn key(source: &Source) -> String {
        match source {
            Source::Library(spec) => safe(&spec.key()),
            Source::File(path) | Source::Catalogue { file: path, .. } => {
                format!("arquivo_{}", safe(&path.to_string_lossy()))
            }
        }
    }

    /// Embedded path of the `.glb` of `source`, registering its bytes the first time.
    fn register(&mut self, source: &Source, registry: &EmbeddedAssetRegistry) -> Option<String> {
        let key = Self::key(source);
        if let Some(path) = self.paths.get(&key) {
            return Some(path.clone());
        }
        let path = match source {
            Source::Library(spec) => {
                let path = format!("utsim/modelos/{key}.glb");
                registry.insert_asset(PathBuf::from(&path), Path::new(&path), spec.glb());
                path
            }
            Source::File(file) | Source::Catalogue { file, .. } => {
                // Each folder gets its own embedded folder, so the textures a `.glb`
                // names by relative URI (`Textures/colormap.png`) resolve next to it.
                let bytes = read_file(file)?;
                let folder = file.parent().unwrap_or(Path::new(""));
                let base = format!("utsim/arquivos/{}", safe(&folder.to_string_lossy()));
                for uri in visual::relative_uris(&bytes) {
                    let embedded = format!("{base}/{uri}");
                    let image = read_optional(&folder.join(&uri)).or_else(|| {
                        // A missing texture would keep the meshes from being drawn at
                        // all: a flat grey one keeps the shape visible.
                        warn!(
                            "{}: textura {uri} não encontrada (usando cinza)",
                            file.display()
                        );
                        uri.to_lowercase()
                            .ends_with(".png")
                            .then(|| GREY_PNG.to_vec())
                    });
                    if let Some(image) = image {
                        registry.insert_asset(
                            PathBuf::from(&embedded),
                            Path::new(&embedded),
                            image,
                        );
                    }
                }
                // The extension picks the glTF loader.
                let name = file.file_stem().map_or("modelo.glb".into(), |n| {
                    format!("{}.glb", safe(&n.to_string_lossy()))
                });
                let path = format!("{base}/{name}");
                registry.insert_asset(PathBuf::from(&path), Path::new(&path), bytes);
                path
            }
        };
        self.paths.insert(key, path.clone());
        Some(path)
    }

    /// Load state of the scene of `source`, when it was requested.
    pub fn failed(&self, source: &Source, server: &AssetServer) -> bool {
        self.scenes
            .get(&Self::key(source))
            .is_some_and(|h| matches!(server.load_state(h.id()), LoadState::Failed(_)))
    }

    /// A copy of `original` with base colour `rgb` (shared by every user of that tint).
    pub fn tinted(
        &mut self,
        original: &Handle<StandardMaterial>,
        rgb: [f32; 3],
        materials: &mut Assets<StandardMaterial>,
    ) -> Option<Handle<StandardMaterial>> {
        let key = (
            original.id(),
            rgb.map(|c| (c.clamp(0.0, 1.0) * 255.0) as u8),
        );
        if let Some(h) = self.tints.get(&key) {
            return Some(h.clone());
        }
        let mut m = materials.get(original)?.clone();
        m.base_color = Color::srgb(rgb[0], rgb[1], rgb[2]).with_alpha(m.base_color.alpha());
        let h = materials.add(m);
        self.tints.insert(key, h.clone());
        Some(h)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_file(path: &Path) -> Option<Vec<u8>> {
    match std::fs::read(path) {
        Ok(b) => Some(b),
        Err(e) => {
            warn!("visual.asset {}: {e}", path.display());
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn read_file(_path: &Path) -> Option<Vec<u8>> {
    None
}

/// A file next to a `.glb` (a texture), `None` when it is not there.
#[cfg(not(target_arch = "wasm32"))]
fn read_optional(path: &Path) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}

#[cfg(target_arch = "wasm32")]
fn read_optional(_path: &Path) -> Option<Vec<u8>> {
    None
}

/// Falls back to the simple shapes if a library scene fails to load.
pub fn watch_library(
    library: Res<Library>,
    server: Res<AssetServer>,
    mut mode: ResMut<VisualMode>,
) {
    if !mode.available {
        return;
    }
    for (key, h) in &library.scenes {
        if key.starts_with("arquivo_") {
            continue;
        }
        if let LoadState::Failed(e) = server.load_state(h.id()) {
            warn!("modelo {key} não carregou ({e}); usando a aparência simples");
            mode.available = false;
            return;
        }
    }
}

/// The looks of the blocks shown now (`None`: simple shape), in block order.
pub fn looks(editor: &Editor, data: &Data, skeletal_people: bool) -> Vec<Option<Look>> {
    let dir = editor
        .doc
        .path()
        .map(Path::to_path_buf)
        .or_else(|| editor.pending_path.clone())
        .and_then(|p| p.parent().map(Path::to_path_buf));
    let dir = dir.as_deref();
    match editor.mode {
        Mode::Edit => {
            let model = editor.doc.model();
            let view = &editor.view;
            let role = |i: usize| {
                (view.kinds[i] == "Transporter")
                    .then(|| utsim_scene::compose::role_of(&view.names[i], Some(model)))
            };
            let ctx = Context {
                pallets: (0..view.kinds.len())
                    .any(|i| view.kinds[i] == "Rack" || role(i) == Some(MobileRole::Forklift)),
                skeletal_people,
            };
            (0..view.kinds.len())
                .map(|i| {
                    let visual = model
                        .objects
                        .iter()
                        .find(|o| o.name == view.names[i])
                        .and_then(|o| o.visual.as_ref());
                    visual::look_of(view.kinds[i], role(i), visual, &view.geoms[i], ctx, dir)
                })
                .collect()
        }
        Mode::Sim => {
            let blocks = &data.replay.blocks;
            let ctx = Context {
                pallets: (0..blocks.len()).any(|b| {
                    blocks[b].kind == "Rack" || data.role(b) == Some(MobileRole::Forklift)
                }),
                skeletal_people,
            };
            (0..blocks.len())
                .map(|b| {
                    if data.is_hidden(b) {
                        return None;
                    }
                    visual::look_of(
                        &blocks[b].kind,
                        data.role(b),
                        data.visual(b),
                        &data.geoms[b],
                        ctx,
                        dir,
                    )
                })
                .collect()
        }
    }
}

/// Pose of mobile block `b` as the realistic model draws it: the scene's pose (which
/// already puts a forklift serving a rack cell in the aisle, facing it), except that
/// vehicles and people keep out of the machines and an operator works at the front.
pub fn drawn_pose(data: &Data, b: usize, t: f64, look: &Look) -> Option<utsim_scene::MobilePose> {
    let mut p = data.mobile_at(b, t)?;
    let original = p.position;
    // Machines are solid: a vehicle or person parked at a station's centre (its network
    // node) is drawn at the station's edge, its front (forks, body) facing it.
    let reach = match look.carry {
        visual::Carry::Forks { forward, .. } => forward + 0.6,
        visual::Carry::Hands => 0.35,
        _ => 0.6,
    };
    for s in 0..data.replay.blocks.len() {
        if matches!(
            data.replay.blocks[s].kind.as_str(),
            "Processor" | "Combiner"
        ) {
            p.position = visual::keep_outside(
                p.position,
                p.yaw,
                data.positions[s],
                data.geoms[s].half,
                reach,
            );
        }
    }
    if look.carry == visual::Carry::Hands && p.motion == Motion::Working {
        // The trace puts an operator at the station's centre: stand at its front instead.
        let station = (0..data.replay.blocks.len()).find(|&s| {
            matches!(
                data.replay.blocks[s].kind.as_str(),
                "Processor" | "Combiner"
            ) && (0..3)
                .step_by(2)
                .all(|k| (original[k] - data.positions[s][k]).abs() <= data.geoms[s].half[k])
        });
        if let Some(s) = station {
            (p.position, p.yaw) = visual::station_front(data.positions[s], data.geoms[s].half);
        }
    }
    Some(p)
}

/// The "Visual: simples / realista" switch.
pub fn toggle(
    ui: &mut bevy_egui::egui::Ui,
    mode: &mut ResMut<VisualMode>,
    lighting: &mut ResMut<crate::scene::Lighting>,
) {
    ui.label("Visual:");
    let (on, available) = (mode.realistic, mode.available);
    ui.add_enabled_ui(available, |ui| {
        if ui.selectable_label(!on || !available, "simples").clicked() && on {
            mode.realistic = false;
        }
        if ui
            .selectable_label(on && available, "realista")
            .on_hover_text("Modelos 3D articulados da biblioteca (empilhadeira, AGV, operador...)")
            .clicked()
            && !on
        {
            mode.realistic = true;
        }
        if on && available {
            crate::scene::shadows_toggle(ui, lighting);
        }
    });
}

/// Bound nodes of a spawned asset.
#[derive(Default)]
struct Rig {
    carro: Option<(Entity, Vec3)>,
    /// `(node, radius, rest rotation)`.
    wheels: Vec<(Entity, f32, Quat)>,
    /// Per-instance material of the `luz` node.
    light: Option<Handle<StandardMaterial>>,
    /// tronco, cabeca, braco_e, braco_d, perna_e, perna_d with their rest rotations.
    limbs: Option<[(Entity, Quat); 6]>,
}

/// A block drawn with a library asset or a user file.
#[derive(Component)]
pub struct Realistic {
    pub block: usize,
    pub look: Look,
    /// The child entity holding the glTF scene.
    pub asset: Entity,
    rig: Option<Rig>,
    light_rgb: Option<[f32; 3]>,
    /// Skeletal person: its animation state (`crate::people`).
    pub person: Option<crate::people::Figure>,
}

/// Recolours the materials with these names under an entity once its scene is spawned
/// (items: the box cardboard by the item's origin; the mannequin's work clothes).
#[derive(Component)]
pub struct Tint(pub Vec<(&'static str, [f32; 3])>);

impl Tint {
    pub fn one(material: &'static str, rgb: [f32; 3]) -> Tint {
        Tint(vec![(material, rgb)])
    }
}

/// Spawns the realistic visual of a block at `translation` (its floor base); returns
/// the root entity, or `None` when the asset is not available (keep the simple shape).
pub fn spawn_block(
    commands: &mut Commands,
    library: &mut Library,
    registry: &EmbeddedAssetRegistry,
    server: &AssetServer,
    block: usize,
    look: &Look,
    translation: Vec3,
) -> Option<Entity> {
    let scene = library.scene(&look.source, registry, server)?;
    let root = commands
        .spawn((
            Transform::from_translation(translation),
            Visibility::default(),
            BlockVisual,
        ))
        .id();
    let asset = commands
        .spawn((
            WorldAssetRoot(scene),
            Transform {
                translation: Vec3::from(look.offset),
                rotation: Quat::from_rotation_y(look.yaw),
                scale: Vec3::splat(look.scale),
            },
            ChildOf(root),
        ))
        .id();
    if look.skeletal {
        commands.entity(asset).insert(crate::people::clothes());
    } else if let Some(rgb) = look.color {
        commands.entity(asset).insert(Tint::one("pintura", rgb));
    }
    commands.entity(root).insert(Realistic {
        block,
        look: look.clone(),
        asset,
        rig: None,
        light_rgb: None,
        person: look.skeletal.then(crate::people::Figure::default),
    });
    Some(root)
}

/// Item shapes, as glTF scenes.
pub struct ItemScenes {
    pub pallet: Handle<WorldAsset>,
    pub caixa: Handle<WorldAsset>,
}

impl ItemScenes {
    pub fn get(
        library: &mut Library,
        registry: &EmbeddedAssetRegistry,
        server: &AssetServer,
    ) -> Option<Self> {
        Some(ItemScenes {
            pallet: library.scene(
                &Source::Library(utsim_assets::Spec::Palete),
                registry,
                server,
            )?,
            caixa: library.scene(
                &Source::Library(utsim_assets::Spec::Caixa),
                registry,
                server,
            )?,
        })
    }
}

/// Cardboard tinted 35 % towards the item's origin colour.
fn box_tint(origin_rgb: [f32; 3]) -> [f32; 3] {
    let card = utsim_assets::builder::hex(0xb8875a);
    [0, 1, 2].map(|k| card[k] + (origin_rgb[k] - card[k]) * 0.35)
}

/// Children of an item entity: an [`ITEM_SIZE`](utsim_scene::compose::ITEM_SIZE) cube
/// filled by a pallet with 2 x 2 x 2 boxes, or by one box.
pub fn spawn_item_children(
    commands: &mut Commands,
    parent: Entity,
    scenes: &ItemScenes,
    pallets: bool,
    origin_rgb: [f32; 3],
) {
    let e = utsim_scene::compose::ITEM_SIZE;
    let half = e / 2.0;
    let tint = || Tint::one("papelao", box_tint(origin_rgb));
    if !pallets {
        // Box 0.40 x 0.30 x 0.30 m: the long side fills the cube edge.
        let k = e / 0.40;
        commands.spawn((
            WorldAssetRoot(scenes.caixa.clone()),
            Transform::from_xyz(0.0, -half, 0.0).with_scale(Vec3::splat(k)),
            tint(),
            ChildOf(parent),
        ));
        return;
    }
    // Pallet 1.2 x 0.144 x 1.0 m scaled to the cube edge, boxes on top.
    let k = e / 1.2;
    commands.spawn((
        WorldAssetRoot(scenes.pallet.clone()),
        Transform::from_xyz(0.0, -half, 0.0).with_scale(Vec3::splat(k)),
        ChildOf(parent),
    ));
    let deck = -half + 0.144 * k;
    let (w, d) = (1.2 * k, 1.0 * k);
    let layer = (e - 0.144 * k) / 2.0;
    let scale = Vec3::new(
        (w / 2.0 - 0.004) / 0.40,
        (layer - 0.003) / 0.30,
        (d / 2.0 - 0.004) / 0.30,
    );
    for level in 0..2 {
        for (sx, sz) in [(-1.0f32, -1.0f32), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
            commands.spawn((
                WorldAssetRoot(scenes.caixa.clone()),
                Transform::from_xyz(sx * w / 4.0, deck + level as f32 * layer, sz * d / 4.0)
                    .with_scale(scale),
                tint(),
                ChildOf(parent),
            ));
        }
    }
}

/// Applies [`Tint`]s once the scenes below them are spawned.
pub fn apply_tints(
    mut commands: Commands,
    tints: Query<(Entity, &Tint)>,
    children: Query<&Children>,
    meshes: Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
    mut library: ResMut<Library>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, tint) in &tints {
        let mut spawned = false;
        let mut done = true;
        for e in children.iter_descendants(entity) {
            spawned = true;
            let Ok((mat, name)) = meshes.get(e) else {
                continue;
            };
            let Some(&(_, rgb)) = tint.0.iter().find(|(m, _)| name.0 == *m) else {
                continue;
            };
            match library.tinted(&mat.0, rgb, &mut materials) {
                Some(h) => {
                    commands.entity(e).insert(MeshMaterial3d(h));
                }
                None => done = false,
            }
        }
        if spawned && done {
            commands.entity(entity).remove::<Tint>();
        }
    }
}

/// Finds the named nodes of every newly spawned asset.
pub fn bind_rigs(
    mut commands: Commands,
    mut rigs: Query<&mut Realistic>,
    children: Query<&Children>,
    nodes: Query<(&Name, &Transform)>,
    meshes: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for mut real in &mut rigs {
        if real.rig.is_some() {
            continue;
        }
        let mut found: HashMap<String, (Entity, Transform)> = HashMap::new();
        for e in children.iter_descendants(real.asset) {
            if let Ok((name, tf)) = nodes.get(e) {
                found.entry(name.as_str().to_string()).or_insert((e, *tf));
            }
        }
        if found.is_empty() {
            continue; // not spawned yet
        }
        let mut rig = Rig {
            carro: found.get("carro").map(|(e, tf)| (*e, tf.translation)),
            ..Rig::default()
        };
        let mut wheels: Vec<_> = found
            .iter()
            .filter(|(n, _)| n.starts_with("roda"))
            .map(|(_, (e, tf))| (*e, tf.translation.y, tf.rotation))
            .collect();
        wheels.sort_by_key(|w| w.0);
        rig.wheels = wheels;
        let limb = |n: &str| found.get(n).map(|(e, tf)| (*e, tf.rotation));
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) = (
            limb("tronco"),
            limb("cabeca"),
            limb("braco_e"),
            limb("braco_d"),
            limb("perna_e"),
            limb("perna_d"),
        ) {
            rig.limbs = Some([a, b, c, d, e, f]);
        }
        if let Some((luz, _)) = found.get("luz") {
            // A material of its own, so each instance shows its own state.
            let mut own: Option<Handle<StandardMaterial>> = None;
            let mut pending = false;
            for e in std::iter::once(*luz).chain(children.iter_descendants(*luz)) {
                let Ok(mat) = meshes.get(e) else {
                    continue;
                };
                if own.is_none() {
                    match materials.get(&mat.0).cloned() {
                        Some(m) => own = Some(materials.add(m)),
                        None => {
                            pending = true;
                            break;
                        }
                    }
                }
                if let Some(h) = &own {
                    commands.entity(e).insert(MeshMaterial3d(h.clone()));
                }
            }
            if pending {
                continue;
            }
            rig.light = own;
        }
        real.rig = Some(rig);
    }
}

/// Drives the bound nodes from the scene pose: heading, forks, wheels, lights, limbs.
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn animate(
    editor: Res<Editor>,
    data: Res<Data>,
    pb: Res<Playback>,
    time: Res<Time<Real>>,
    mode: Res<VisualMode>,
    mut roots: Query<(&mut Realistic, &mut Transform)>,
    mut nodes: Query<&mut Transform, Without<Realistic>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !mode.active() {
        return;
    }
    let sim = editor.mode == Mode::Sim;
    let clock = time.elapsed_secs();
    for (mut real, mut root) in &mut roots {
        let b = real.block;
        let scale = real.look.scale;
        let pose = if sim {
            drawn_pose(&data, b, pb.t, &real.look)
        } else {
            None
        };
        let (motion, lift) = pose.map_or((Motion::Idle, 0.0), |p| (p.motion, p.lift));
        if let Some(p) = pose {
            root.translation = Vec3::from(p.position);
            root.rotation = Quat::from_rotation_y(p.yaw);
        }
        // From the scene, so the wheels and the stride depend on the time only.
        let walked = if sim {
            data.distance_at(b, pb.t) as f32 / scale
        } else {
            0.0
        };
        let agv = sim && data.role(b) == Some(MobileRole::Agv);
        let state = if sim {
            data.timeline.block_state(b, pb.t)
        } else {
            None
        };
        let rgb = visual::light_color(agv, motion, state);
        let changed = real.light_rgb != Some(rgb);
        real.light_rgb = Some(rgb);
        let Some(rig) = &real.rig else {
            continue;
        };
        if let Some((carro, rest)) = rig.carro
            && let Ok(mut tf) = nodes.get_mut(carro)
        {
            let height = if pose.is_some() {
                lift
            } else {
                utsim_assets::dims::FORK_REST
            };
            tf.translation.y = rest.y + (height - utsim_assets::dims::FORK_REST) / scale;
        }
        for &(wheel, radius, rest) in &rig.wheels {
            if let Ok(mut tf) = nodes.get_mut(wheel) {
                tf.rotation = rest * Quat::from_rotation_z(visual::wheel_angle(walked, radius));
            }
        }
        if let Some(limbs) = rig.limbs {
            let p = visual::person_pose(motion, walked * scale, lift, clock);
            // A positive angle swings a hanging limb forward; the torso leans forward
            // with a negative one (its parts are above the pivot).
            let angles = [
                -p.torso,
                0.0,
                p.arm_left,
                p.arm_right,
                p.leg_left,
                p.leg_right,
            ];
            for ((entity, rest), angle) in limbs.iter().zip(angles) {
                if let Ok(mut tf) = nodes.get_mut(*entity) {
                    tf.rotation = *rest * Quat::from_rotation_z(angle);
                }
            }
        }
        if changed
            && let Some(h) = &rig.light
            && let Some(mut m) = materials.get_mut(h)
        {
            let c = Color::srgb(rgb[0], rgb[1], rgb[2]);
            m.base_color = c;
            m.emissive = c.to_linear() * 3.0;
        }
    }
}
