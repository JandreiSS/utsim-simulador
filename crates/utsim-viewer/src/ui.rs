//! egui control panel, block statistics and in-scene labels (texts in `editor.ftl`).

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use utsim_core::TallySummary;
use utsim_fluid::FluidKind;
use utsim_viewer::editing::NetworkView;
use utsim_viewer::texto::{fx, tx, vx};
use utsim_viewer::{fluids, style, view};

use crate::dock::Dock;
use crate::editor::Editor;
use crate::playback::{Data, FluidSelection, Playback, SPEEDS, Selection};

fn fmt_tally(t: &TallySummary) -> String {
    if t.count == 0 {
        tx("ui-sem_observacoes").to_string()
    } else {
        fx(
            "ui-resumo",
            &[
                ("n", vx(&t.count)),
                ("media", vx(&format!("{:.4}", t.mean))),
                ("dp", vx(&format!("{:.4}", t.std_dev))),
                ("min", vx(&format!("{:.4}", t.min))),
                ("max", vx(&format!("{:.4}", t.max))),
            ],
        )
    }
}

/// A block name drawn above the block.
pub struct Label {
    pub text: String,
    /// Bevy-space base position of the block.
    pub position: [f32; 3],
    /// Height above the base where the block (and the items on it) ends.
    pub height: f32,
    pub selected: bool,
}

/// Block names above the blocks (shared by both modes), decluttered in screen space:
/// a label that would cover another is pushed up and tied to its block by a thin line.
/// `free` is the part of the window not covered by panels; labels are clipped to it.
pub fn draw_labels(
    ctx: &egui::Context,
    cam: &Camera,
    cam_tf: &GlobalTransform,
    labels: &[Label],
    free: egui::Rect,
) {
    let painter = ctx
        .layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("rotulos"),
        ))
        .with_clip_rect(free);
    let font = egui::FontId::proportional(13.0);
    let pad = egui::vec2(3.0, 1.0);
    let mut anchors: Vec<egui::Pos2> = Vec::new();
    let mut galleys = Vec::new();
    let mut boxes: Vec<view::LabelBox> = Vec::new();
    for label in labels {
        let world = Vec3::from(label.position) + Vec3::Y * (label.height + 0.5);
        let Ok(p) = cam.world_to_viewport(cam_tf, world) else {
            continue;
        };
        let anchor = egui::pos2(p.x, p.y);
        let fg = if label.selected {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(205)
        };
        let galley = painter.layout_no_wrap(label.text.clone(), font.clone(), fg);
        let size = galley.size() + pad * 2.0;
        boxes.push(view::LabelBox {
            anchor: [anchor.x, anchor.y],
            size: [size.x, size.y],
            first: label.selected,
        });
        anchors.push(anchor);
        galleys.push(galley);
    }
    let placed = view::declutter(&boxes, 2.0);
    for ((place, anchor), galley) in placed.iter().zip(&anchors).zip(galleys) {
        let rect = egui::Rect::from_min_size(
            egui::pos2(place.min[0], place.min[1]),
            egui::vec2(place.size[0], place.size[1]),
        );
        if place.displaced {
            let line = egui::Stroke::new(1.0, egui::Color32::from_white_alpha(110));
            painter.line_segment([*anchor, rect.center_bottom()], line);
            painter.circle_filled(*anchor, 2.0, egui::Color32::from_white_alpha(160));
        }
        painter.rect_filled(rect, 3.0, egui::Color32::from_black_alpha(170));
        painter.galley(rect.min + pad, galley, egui::Color32::WHITE);
    }
}

/// What the "state" column of a block shows: the state name, or for sinks (which never
/// change state) how many items they received so far.
fn state_text(kind: &str, state: Option<utsim_core::State>, received: usize) -> String {
    if kind == "Sink" {
        fx("ui-recebidos", &[("n", vx(&received))])
    } else {
        style::state_label(state).to_string()
    }
}

#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn panel(
    mut contexts: EguiContexts,
    data: Res<Data>,
    mut pb: ResMut<Playback>,
    mut selection: ResMut<Selection>,
    mut fluid_sel: ResMut<FluidSelection>,
    mut dock: ResMut<Dock>,
    mut editor: ResMut<Editor>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut visual: ResMut<crate::models::VisualMode>,
    mut lighting: ResMut<crate::scene::Lighting>,
    mut cameras: ResMut<crate::camera_modes::CameraModes>,
    mut lod: ResMut<crate::lod::Lod>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let tl = &data.timeline;

    let mut ui = dock.ui(ctx, "esquerda");
    egui::Panel::left("painel_simulacao")
        .default_size(350.0)
        .size_range(260.0..=560.0)
        .show(&mut ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Without the toolbar (a recorded replay), the look switch lives here.
                    if !editor.editable {
                        ui.horizontal(|ui| {
                            crate::models::toggle(ui, &mut visual, &mut lighting);
                            ui.separator();
                            crate::camera_modes::selector(ui, &mut cameras);
                        });
                    }
                    sim_controls(
                        ui,
                        &data,
                        &mut pb,
                        &mut selection,
                        &mut fluid_sel,
                        &mut editor,
                        &mut cameras,
                    )
                });
        });
    dock.free = ui.available_rect_before_wrap();

    let (cam, cam_tf) = *camera;
    // Big models: only the nearest labels are laid out (see `lod.rs`).
    let shown: Vec<usize> = (0..data.replay.blocks.len())
        .filter(|i| !data.is_hidden(*i))
        .collect();
    let mask = crate::lod::label_mask(
        &lod,
        cam_tf.translation(),
        shown
            .iter()
            .map(|i| crate::scene::block_base_at(&data, *i, pb.t)),
        |k| selection.0 == Some(shown[k]),
    );
    let labels: Vec<Label> = shown
        .iter()
        .zip(&mask)
        .filter(|(_, keep)| **keep)
        .map(|(&i, _)| {
            let block = &data.replay.blocks[i];
            let geom = &data.geoms[i];
            let content = tl.block_content(i, pb.t);
            let hidden = geom.hidden_items(content);
            let received = (block.kind == "Sink").then(|| tl.block_received(i, pb.t));
            Label {
                text: style::label_text(&block.name, content, hidden, received),
                position: crate::scene::block_base_at(&data, i, pb.t).to_array(),
                height: geom.label_height(content),
                selected: selection.0 == Some(i),
            }
        })
        .collect();
    lod.stats.labels_drawn = labels.len();
    lod.stats.labels_total = shown.len();
    let mut labels = labels;
    labels.extend(fluid_labels(&data, pb.t, fluid_sel.0));
    labels.extend(capacity_labels(&data.network));
    draw_labels(ctx, cam, cam_tf, &labels, dock.free);
    Ok(())
}

fn sim_controls(
    ui: &mut egui::Ui,
    data: &Data,
    pb: &mut Playback,
    selection: &mut Selection,
    fluid_sel: &mut FluidSelection,
    editor: &mut Editor,
    cameras: &mut crate::camera_modes::CameraModes,
) {
    let tl = &data.timeline;
    ui.heading(tx("ui-simulacao"));
    ui.label(fx(
        "ui-modelo_replicacao",
        &[
            ("modelo", vx(&data.replay.model)),
            ("n", vx(&data.replay.replication)),
        ],
    ));
    ui.label(fx(
        "ui-contagem",
        &[
            ("blocos", vx(&tl.block_count())),
            ("itens", vx(&tl.item_count())),
            ("eventos", vx(&tl.record_count())),
        ],
    ));
    ui.separator();

    ui.horizontal_wrapped(|ui| {
        let label = if pb.playing {
            tx("ui-pausar")
        } else {
            tx("ui-reproduzir")
        };
        if ui.button(label).clicked() {
            if pb.t >= data.end {
                pb.t = 0.0;
            }
            pb.playing = !pb.playing;
        }
        if ui.button(tx("ui-evento_anterior")).clicked() {
            pb.playing = false;
            let t = pb.t;
            pb.t = tl.prev_event_time(t).unwrap_or(0.0);
        }
        if ui.button(tx("ui-evento_seguinte")).clicked() {
            pb.playing = false;
            let t = pb.t;
            pb.t = tl.next_event_time(t).unwrap_or(data.end).min(data.end);
        }
        if ui.button(tx("ui-inicio")).clicked() {
            pb.t = 0.0;
        }
        if ui.button(tx("editor-enquadrar")).clicked() {
            editor.frame_request = true;
        }
    });
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            pb.slower();
        }
        ui.label(fx("ui-velocidade", &[("v", vx(&SPEEDS[pb.speed_idx]))]));
        if ui.button("+").clicked() {
            pb.faster();
        }
    });
    ui.weak(tx("ui-unidades_por_segundo"));
    let mut t = pb.t;
    let slider = egui::Slider::new(&mut t, 0.0..=data.end.max(f64::MIN_POSITIVE))
        .text(tx("ui-tempo"))
        .smart_aim(false);
    if ui.add(slider).changed() {
        pb.t = t;
        pb.playing = false;
    }
    ui.label(fx(
        "ui-tempo_simulado",
        &[
            ("t", vx(&format!("{:.4}", pb.t))),
            ("fim", vx(&format!("{:.4}", data.end))),
        ],
    ));
    ui.weak(tx("ui-ajuda_teclas"));
    ui.weak(tx("ui-ajuda_mouse"));
    ui.weak(tx("ui-ajuda_camera"));
    ui.separator();

    ui.strong(tx("ui-blocos"));
    egui::ScrollArea::vertical()
        .id_salt("lista_blocos")
        .max_height(170.0)
        .show(ui, |ui| {
            for (i, block) in data.replay.blocks.iter().enumerate() {
                if data.is_hidden(i) {
                    continue;
                }
                let state = tl.block_state(i, pb.t);
                let text = format!(
                    "{} [{}]  {}  ({})",
                    block.name,
                    block.kind,
                    state_text(&block.kind, state, tl.block_received(i, pb.t)),
                    tl.block_content(i, pb.t)
                );
                let on = selection.0 == Some(i);
                if ui.selectable_label(on, text).clicked() {
                    selection.0 = if on { None } else { Some(i) };
                    fluid_sel.0 = None;
                }
            }
        });
    ui.separator();

    if !data.fluid.is_empty() {
        ui.strong(tx("camada-fluidos"));
        egui::ScrollArea::vertical()
            .id_salt("lista_fluidos")
            .max_height(130.0)
            .show(ui, |ui| {
                for (i, node) in data.fluid.nodes.iter().enumerate() {
                    let sample = data.fluid_sample(i, pb.t);
                    let text = if node.shape == fluids::NodeShape::Tank {
                        format!(
                            "{} [{}]  {} de {}  ({})",
                            node.name,
                            node.kind_label,
                            fmt_num(sample.level),
                            fmt_num(node.capacity),
                            fluids::percent_text(sample.level, node.capacity)
                        )
                    } else {
                        format!("{} [{}]", node.name, node.kind_label)
                    };
                    let on = fluid_sel.0 == Some(i);
                    if ui.selectable_label(on, text).clicked() {
                        fluid_sel.0 = if on { None } else { Some(i) };
                        selection.0 = None;
                    }
                }
            });
        ui.separator();
    }

    if let Some(node) = fluid_sel.0.filter(|i| *i < data.fluid.nodes.len()) {
        fluid_stats(ui, data, node, pb.t);
        return;
    }
    match selection.0.filter(|s| *s < data.replay.blocks.len()) {
        None => {
            ui.label(tx("ui-clique_em_um_bloco"));
        }
        Some(sel) => block_stats(ui, data, sel, pb.t, cameras),
    }
}

/// A number with up to 3 decimals and no trailing zeros ("20", "2.5", "0.125").
fn fmt_num(v: f64) -> String {
    let s = format!("{v:.3}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// Labels of the fluid nodes: name, and for tanks the level against the capacity.
fn fluid_labels(data: &Data, t: f64, selected: Option<usize>) -> Vec<Label> {
    data.fluid
        .nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let text = if node.shape == fluids::NodeShape::Tank {
                let level = data.fluid_sample(i, t).level;
                format!(
                    "{}  {}/{}",
                    node.name,
                    fmt_num(level),
                    fmt_num(node.capacity)
                )
            } else {
                node.name.clone()
            };
            Label {
                text,
                position: node.pos,
                height: fluids::node_half(node)[1] * 2.0,
                selected: selected == Some(i),
            }
        })
        .collect()
}

/// A "cap. N" label at the middle of every controlled edge of the path network.
pub fn capacity_labels(network: &NetworkView) -> Vec<Label> {
    network
        .segments
        .iter()
        .filter_map(|seg| {
            let capacity = seg.capacity?;
            let mid = [
                (seg.a[0] + seg.b[0]) / 2.0,
                (seg.a[1] + seg.b[1]) / 2.0,
                (seg.a[2] + seg.b[2]) / 2.0,
            ];
            Some(Label {
                text: fx("ui-capacidade_rotulo", &[("n", vx(&capacity))]),
                position: mid,
                height: -0.2,
                selected: false,
            })
        })
        .collect()
}

fn fluid_stats(ui: &mut egui::Ui, data: &Data, node: usize, t: f64) {
    let view = &data.fluid.nodes[node];
    let sample = data.fluid_sample(node, t);
    ui.strong(format!("{} ({})", view.name, view.kind_label));
    let kind = data
        .replay
        .fluids
        .as_ref()
        .and_then(|d| d.nodes.get(node))
        .map(|n| &n.kind);
    egui::Grid::new("estatisticas_fluido")
        .striped(true)
        .show(ui, |ui| {
            let row = |ui: &mut egui::Ui, name: &str, value: String| {
                ui.label(name);
                ui.label(value);
                ui.end_row();
            };
            match kind {
                Some(FluidKind::Tank {
                    capacity,
                    initial,
                    max_in,
                    max_out,
                }) => {
                    row(ui, tx("ui-nivel_agora"), fmt_num(sample.level));
                    row(ui, tx("ui-capacidade"), fmt_num(*capacity));
                    row(
                        ui,
                        tx("ui-ocupacao"),
                        fluids::percent_text(sample.level, *capacity),
                    );
                    row(
                        ui,
                        tx("ui-taxa_agora"),
                        fx(
                            "ui-por_unidade_de_tempo",
                            &[
                                ("taxa", vx(&fmt_num(sample.rate))),
                                ("sentido", vx(&fluids::rate_label(sample.rate))),
                            ],
                        ),
                    );
                    row(ui, tx("ui-nivel_inicial"), fmt_num(*initial));
                    if let Some(r) = max_in {
                        row(ui, tx("ui-entrada_maxima"), fmt_num(*r));
                    }
                    if let Some(r) = max_out {
                        row(ui, tx("ui-saida_maxima"), fmt_num(*r));
                    }
                }
                Some(FluidKind::Source { rate }) => {
                    row(ui, tx("ui-vazao_agora"), fmt_num(sample.rate));
                    row(ui, tx("ui-vazao_da_fonte"), fmt_num(*rate));
                }
                Some(FluidKind::Pipe { max_rate }) => {
                    row(ui, tx("ui-vazao_agora"), fmt_num(sample.rate));
                    row(ui, tx("ui-vazao_maxima"), fmt_num(*max_rate));
                }
                Some(FluidKind::Sink { max_rate }) => {
                    row(ui, tx("ui-vazao_agora"), fmt_num(sample.rate));
                    if let Some(r) = max_rate {
                        row(ui, tx("ui-vazao_maxima"), fmt_num(*r));
                    }
                }
                Some(FluidKind::FromItems { object, volume }) => {
                    row(ui, tx("ui-objeto"), object.clone());
                    row(ui, tx("ui-volume_por_item"), fmt_num(*volume));
                }
                Some(FluidKind::ToItems {
                    object,
                    volume,
                    max_rate,
                }) => {
                    row(ui, tx("ui-objeto"), object.clone());
                    row(ui, tx("ui-volume_por_item"), fmt_num(*volume));
                    row(ui, tx("ui-acumulado"), fmt_num(sample.level));
                    row(ui, tx("ui-vazao_agora"), fmt_num(sample.rate));
                    if let Some(r) = max_rate {
                        row(ui, tx("ui-vazao_maxima"), fmt_num(*r));
                    }
                }
                None => {}
            }
        });
    ui.weak(tx("ui-nivel_interpolado"));
}

fn block_stats(
    ui: &mut egui::Ui,
    data: &Data,
    sel: usize,
    t: f64,
    cameras: &mut crate::camera_modes::CameraModes,
) {
    let block = &data.replay.blocks[sel];
    let tl = &data.timeline;
    ui.strong(format!("{} ({})", block.name, block.kind));
    if tl.is_mobile(sel) {
        ui.horizontal(|ui| {
            if ui
                .button(tx("ui-seguir_recurso"))
                .on_hover_text(tx("ui-seguir_recurso_dica"))
                .clicked()
            {
                cameras.follow_block(sel);
            }
            if ui
                .button(tx("ui-ver_do_recurso"))
                .on_hover_text(tx("ui-ver_do_recurso_dica"))
                .clicked()
            {
                cameras.ride_block(sel);
            }
        });
    }
    let report = data
        .replay
        .report
        .blocks
        .get(sel)
        .filter(|r| r.name == block.name)
        .or_else(|| data.replay.report.block(&block.name));
    egui::Grid::new("estatisticas")
        .striped(true)
        .show(ui, |ui| {
            let state = tl.block_state(sel, t);
            if block.kind == "Sink" {
                ui.label(tx("ui-recebidos_ate_agora"));
                ui.label(tl.block_received(sel, t).to_string());
            } else {
                ui.label(tx("ui-estado_agora"));
                ui.label(style::state_label(state));
            }
            ui.end_row();
            ui.label(tx("ui-conteudo_agora"));
            ui.label(tl.block_content(sel, t).to_string());
            ui.end_row();
            let Some(r) = report else {
                return;
            };
            ui.label(tx("ui-entradas"));
            ui.label(r.input.to_string());
            ui.end_row();
            ui.label(tx("ui-saidas"));
            ui.label(r.output.to_string());
            ui.end_row();
            ui.label(tx("ui-conteudo_medio"));
            ui.label(format!("{:.4}", r.content_mean));
            ui.end_row();
            ui.label(tx("ui-conteudo_maximo"));
            ui.label(format!("{}", r.content_max));
            ui.end_row();
            ui.label(tx("ui-tempo_de_permanencia"));
            ui.label(fmt_tally(&r.staytime));
            ui.end_row();
            for (name, frac) in &r.states {
                ui.label(fx("ui-fracao_em", &[("estado", vx(name))]));
                ui.label(format!("{:.2}%", frac * 100.0));
                ui.end_row();
            }
            for (name, tally) in &r.extra {
                ui.label(name);
                ui.label(fmt_tally(tally));
                ui.end_row();
            }
        });
    ui.weak(tx("ui-estatisticas_finais"));
}
