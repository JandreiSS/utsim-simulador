//! Level of detail and the frame-time overlay.
//!
//! On a big model most of the plant is far from the camera and cannot show detail: the
//! articulated glTF models, the pallets with their boxes and the label of every block
//! cost the same as the ones in front of the camera. [`apply`] looks at the distance of
//! each entity to the camera every frame and switches the far ones to a cheaper
//! representation (rules and thresholds in [`utsim_viewer::lod_rules`]):
//!
//! 1. a realistic block (a [`Realistic`] root) becomes a plain box with its footprint;
//! 2. a realistic item (a pallet with eight boxes) becomes one cube;
//! 3. farther still, items are not drawn;
//! 4. labels: only the nearest ones are drawn (see [`label_mask`]), at most
//!    `label_max`, and none beyond `label_hide_m`.
//!
//! Nothing here touches how the models are spawned: it finds them by the components the
//! scene already puts on them ([`Realistic`], [`ItemVisual`]) and only flips visibility
//! and adds one proxy child per entity, created the first time it is needed. `F3` shows
//! the frame times and the counts per level, `F4` switches the LOD off and on (to
//! compare), and the overlay has sliders for the thresholds.

use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use utsim_scene::MobileRole;
use utsim_scene::compose::ITEM_SIZE;
use utsim_viewer::lod_rules::{BlockDetail, ItemDetail, LodSettings};

use crate::dock::Dock;
use crate::editor::{Editor, Mode};
use crate::models::Realistic;
use crate::playback::Data;
use crate::scene::ItemVisual;

/// Counts per level, refreshed every frame, shown by the overlay.
#[derive(Default, Clone, Copy)]
pub struct LodStats {
    pub blocks_model: usize,
    pub blocks_box: usize,
    pub items_model: usize,
    pub items_box: usize,
    pub items_hidden: usize,
    pub labels_drawn: usize,
    pub labels_total: usize,
    /// Entities of the world and how many of them have a mesh (counted while the
    /// overlay is open).
    pub entities: usize,
    pub meshes: usize,
}

/// The LOD settings, the frame-time history and the overlay switch.
#[derive(Resource)]
pub struct Lod {
    pub settings: LodSettings,
    pub stats: LodStats,
    /// The F3 overlay is open.
    pub overlay: bool,
    /// The LOD is acting (on, and the model is heavy enough).
    pub active: bool,
    /// Last frame times (seconds), newest last.
    frames: VecDeque<f32>,
}

/// Frames the overlay averages over.
const HISTORY: usize = 120;

impl Default for Lod {
    fn default() -> Self {
        Lod {
            settings: LodSettings::default(),
            stats: LodStats::default(),
            overlay: false,
            active: false,
            frames: VecDeque::with_capacity(HISTORY),
        }
    }
}

impl Lod {
    /// The settings as they apply now: all off while the model is too light for the LOD.
    pub fn effective(&self) -> LodSettings {
        LodSettings {
            enabled: self.active,
            ..self.settings
        }
    }

    /// Mean frame time over the history, in milliseconds, and the slowest frame.
    pub fn frame_ms(&self) -> Option<(f32, f32)> {
        if self.frames.is_empty() {
            return None;
        }
        let sum: f32 = self.frames.iter().sum();
        let max = self.frames.iter().copied().fold(0.0, f32::max);
        Some((sum / self.frames.len() as f32 * 1000.0, max * 1000.0))
    }
}

/// Shared meshes and materials of the proxies (one cube mesh, a material per colour).
#[derive(Resource, Default)]
pub struct LodAssets {
    cube: Option<Handle<Mesh>>,
    materials: HashMap<[u8; 3], Handle<StandardMaterial>>,
}

impl LodAssets {
    fn cube(&mut self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        self.cube
            .get_or_insert_with(|| meshes.add(Cuboid::new(1.0, 1.0, 1.0)))
            .clone()
    }

    fn material(
        &mut self,
        rgb: [f32; 3],
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        let key = rgb.map(|c| (c.clamp(0.0, 1.0) * 255.0) as u8);
        self.materials
            .entry(key)
            .or_insert_with(|| {
                materials.add(StandardMaterial {
                    base_color: Color::srgb(rgb[0], rgb[1], rgb[2]),
                    perceptual_roughness: 0.7,
                    ..default()
                })
            })
            .clone()
    }
}

/// The cheap stand-in of a block or an item (a child of it).
#[derive(Component)]
pub struct LodProxy;

/// LOD state of a realistic block root.
#[derive(Component)]
pub struct LodBlock {
    detail: BlockDetail,
    proxy: Entity,
}

/// LOD state of an item root.
#[derive(Component)]
pub struct LodItem {
    detail: ItemDetail,
    proxy: Option<Entity>,
}

/// Cardboard colour of the item cube.
const ITEM_RGB: [f32; 3] = [0.72, 0.53, 0.35];
/// Colour of a far block without a colour of its own.
const BLOCK_RGB: [f32; 3] = [0.55, 0.58, 0.64];

/// Box (centre above the floor, size) that stands for a realistic block.
fn proxy_box(role: Option<MobileRole>, half: [f32; 3]) -> (Vec3, Vec3) {
    match role {
        Some(MobileRole::Forklift) => (Vec3::new(0.7, 1.05, 0.0), Vec3::new(3.4, 2.1, 1.2)),
        Some(MobileRole::Person) => (Vec3::new(0.0, 0.9, 0.0), Vec3::new(0.5, 1.8, 0.4)),
        Some(MobileRole::Agv) => (Vec3::new(0.0, 0.3, 0.0), Vec3::new(1.2, 0.6, 0.8)),
        _ => (Vec3::Y * half[1], Vec3::from(half) * 2.0),
    }
}

/// Half extents and role of block `b` as shown now.
fn block_info(editor: &Editor, data: &Data, b: usize) -> Option<([f32; 3], Option<MobileRole>)> {
    match editor.mode {
        Mode::Edit => {
            let g = editor.view.geoms.get(b)?;
            let role = (editor.view.kinds.get(b) == Some(&"Transporter")).then(|| {
                utsim_scene::compose::role_of(&editor.view.names[b], Some(editor.doc.model()))
            });
            Some((g.half, role))
        }
        Mode::Sim => Some((data.geoms.get(b)?.half, data.role(b))),
    }
}

/// What the LOD reads of an item: its root, where it is, its state and whether it is a
/// plain cube already.
type ItemQuery = (Entity, &'static Transform, Option<&'static LodItem>, Has<Mesh3d>);

/// Switches the entities to the level their distance asks for.
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn apply(
    mut commands: Commands,
    mut lod: ResMut<Lod>,
    mut assets: ResMut<LodAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    editor: Res<Editor>,
    data: Res<Data>,
    camera: Single<&Transform, With<Camera3d>>,
    blocks: Query<(Entity, &Realistic, &Transform, Option<&LodBlock>), Without<Camera3d>>,
    items: Query<ItemQuery, With<ItemVisual>>,
    children: Query<&Children>,
    proxies: Query<(), With<LodProxy>>,
    mut visibility: Query<&mut Visibility>,
    all: Query<()>,
    meshed: Query<(), With<Mesh3d>>,
) {
    let eye = camera.translation;
    // Light models are drawn in full; the LOD engages with the number of realistic
    // blocks and drawn items.
    let settings = lod
        .settings
        .engaged(blocks.iter().count(), items.iter().count());
    lod.active = settings.enabled;
    let mut stats = LodStats {
        labels_drawn: lod.stats.labels_drawn,
        labels_total: lod.stats.labels_total,
        entities: if lod.overlay { all.iter().count() } else { 0 },
        meshes: if lod.overlay {
            meshed.iter().count()
        } else {
            0
        },
        ..LodStats::default()
    };

    for (entity, real, tf, state) in &blocks {
        let current = state.map_or(BlockDetail::Model, |s| s.detail);
        let wanted = settings.block_detail(eye.distance(tf.translation), current);
        match wanted {
            BlockDetail::Model => stats.blocks_model += 1,
            BlockDetail::Box => stats.blocks_box += 1,
        }
        if wanted == current {
            continue;
        }
        let proxy = match state {
            Some(s) => s.proxy,
            None => {
                let (half, role) =
                    block_info(&editor, &data, real.block).unwrap_or(([0.5; 3], None));
                let (centre, size) = proxy_box(role, half);
                let mat = assets.material(real.look.color.unwrap_or(BLOCK_RGB), &mut materials);
                commands
                    .spawn((
                        Mesh3d(assets.cube(&mut meshes)),
                        MeshMaterial3d(mat),
                        Transform::from_translation(centre).with_scale(size),
                        Visibility::Hidden,
                        LodProxy,
                        ChildOf(entity),
                    ))
                    .id()
            }
        };
        let show_model = wanted == BlockDetail::Model;
        if let Ok(mut v) = visibility.get_mut(real.asset) {
            v.set_if_neq(if show_model {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
        }
        // A fresh proxy has no `Visibility` component to query yet; the command wins.
        commands.entity(proxy).insert(if show_model {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        });
        commands.entity(entity).insert(LodBlock {
            detail: wanted,
            proxy,
        });
    }

    for (entity, tf, state, simple) in &items {
        let current = state.map_or(ItemDetail::Model, |s| s.detail);
        let wanted = settings.item_detail(eye.distance(tf.translation), current);
        match wanted {
            ItemDetail::Model => stats.items_model += 1,
            ItemDetail::Box => stats.items_box += 1,
            ItemDetail::Hidden => stats.items_hidden += 1,
        }
        if wanted == current {
            continue;
        }
        // A simple item is already one cube: only hiding matters.
        let mut proxy = state.and_then(|s| s.proxy);
        if proxy.is_none() && wanted == ItemDetail::Box && !simple {
            proxy = Some(
                commands
                    .spawn((
                        Mesh3d(assets.cube(&mut meshes)),
                        MeshMaterial3d(assets.material(ITEM_RGB, &mut materials)),
                        Transform::from_scale(Vec3::splat(ITEM_SIZE)),
                        Visibility::Hidden,
                        LodProxy,
                        ChildOf(entity),
                    ))
                    .id(),
            );
        }
        if let Ok(mut v) = visibility.get_mut(entity) {
            v.set_if_neq(if wanted == ItemDetail::Hidden {
                Visibility::Hidden
            } else {
                Visibility::Inherited
            });
        }
        // The model's parts hide when the cube stands in for them (or nothing is drawn).
        let show_parts = wanted == ItemDetail::Model;
        if let Ok(kids) = children.get(entity) {
            for kid in kids.iter() {
                if proxies.contains(kid) {
                    continue;
                }
                if let Ok(mut v) = visibility.get_mut(kid) {
                    v.set_if_neq(if show_parts {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    });
                }
            }
        }
        if let Some(p) = proxy {
            commands.entity(p).insert(if wanted == ItemDetail::Box {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
        }
        commands.entity(entity).insert(LodItem {
            detail: wanted,
            proxy,
        });
    }
    lod.stats = stats;
}

/// Which of the labels at `positions` to draw (nearest first, capped), see
/// [`LodSettings::label_mask`]. `always` labels (the selected block) are kept.
pub fn label_mask(
    lod: &Lod,
    eye: Vec3,
    positions: impl Iterator<Item = Vec3>,
    always: impl Fn(usize) -> bool,
) -> Vec<bool> {
    let dists: Vec<f32> = positions.map(|p| eye.distance(p)).collect();
    let mut mask = lod.effective().label_mask(&dists);
    for (i, m) in mask.iter_mut().enumerate() {
        *m |= always(i);
    }
    mask
}

/// Records the frame time and handles F3 (overlay) and F4 (LOD on/off).
pub fn frames(
    mut lod: ResMut<Lod>,
    time: Res<Time<Real>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut contexts: EguiContexts,
) {
    if lod.frames.len() == HISTORY {
        lod.frames.pop_front();
    }
    lod.frames.push_back(time.delta_secs());
    let typing = contexts
        .ctx_mut()
        .is_ok_and(|ctx| ctx.egui_wants_keyboard_input());
    if typing {
        return;
    }
    if keys.just_pressed(KeyCode::F3) {
        lod.overlay = !lod.overlay;
    }
    if keys.just_pressed(KeyCode::F4) {
        lod.settings.enabled = !lod.settings.enabled;
    }
}

/// The F3 overlay: frame time, counts per level and the thresholds.
pub fn overlay(mut contexts: EguiContexts, mut lod: ResMut<Lod>, dock: Res<Dock>) -> Result {
    if !lod.overlay {
        return Ok(());
    }
    let ctx = contexts.ctx_mut()?;
    let corner = dock.free.left_bottom() + egui::vec2(10.0, -10.0);
    let (ms, worst) = lod.frame_ms().unwrap_or((0.0, 0.0));
    let s = lod.stats;
    egui::Area::new(egui::Id::new("desempenho"))
        .order(egui::Order::Foreground)
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(corner)
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_max_width(300.0);
                ui.strong(format!(
                    "{:.1} ms por quadro ({:.0} quadros/s)",
                    ms,
                    1000.0 / ms.max(0.001)
                ));
                ui.label(format!("pior quadro: {worst:.1} ms"));
                ui.checkbox(&mut lod.settings.enabled, "Níveis de detalhe (F4)");
                if lod.settings.enabled && !lod.active {
                    ui.weak("(parado: o modelo é leve, veja \"a partir de\")");
                }
                ui.label(format!(
                    "blocos: {} modelo, {} caixa",
                    s.blocks_model, s.blocks_box
                ));
                ui.label(format!(
                    "itens: {} modelo, {} cubo, {} ocultos",
                    s.items_model, s.items_box, s.items_hidden
                ));
                ui.label(format!("rótulos: {} de {}", s.labels_drawn, s.labels_total));
                ui.label(format!(
                    "entidades: {} ({} com malha)",
                    s.entities, s.meshes
                ));
                ui.collapsing("Distâncias (m)", |ui| {
                    let st = &mut lod.settings;
                    ui.add(
                        egui::Slider::new(&mut st.min_blocks, 0..=1000)
                            .text("a partir de (blocos)"),
                    );
                    ui.add(egui::Slider::new(&mut st.min_items, 0..=5000).text("ou (itens)"));
                    ui.add(
                        egui::Slider::new(&mut st.block_box_m, 5.0..=400.0)
                            .text("bloco vira caixa"),
                    );
                    ui.add(
                        egui::Slider::new(&mut st.item_box_m, 5.0..=400.0).text("item vira cubo"),
                    );
                    ui.add(
                        egui::Slider::new(&mut st.item_hide_m, 10.0..=1000.0).text("itens ocultos"),
                    );
                    ui.add(
                        egui::Slider::new(&mut st.label_hide_m, 10.0..=1000.0)
                            .text("rótulos ocultos"),
                    );
                    ui.add(egui::Slider::new(&mut st.label_max, 10..=1000).text("rótulos (máx.)"));
                });
                ui.weak("F3 fecha");
            });
        });
    Ok(())
}
