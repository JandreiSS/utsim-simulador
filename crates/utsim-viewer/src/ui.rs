//! egui control panel, block statistics and in-scene labels (Portuguese UI).

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use utsim_core::TallySummary;
use utsim_fluid::FluidKind;
use utsim_viewer::editing::NetworkView;
use utsim_viewer::{fluids, style, view};

use crate::dock::Dock;
use crate::editor::Editor;
use crate::playback::{Data, FluidSelection, Playback, SPEEDS, Selection};

fn fmt_tally(t: &TallySummary) -> String {
    if t.count == 0 {
        "sem observações".to_string()
    } else {
        format!(
            "n={}  média={:.4}  dp={:.4}  mín={:.4}  máx={:.4}",
            t.count, t.mean, t.std_dev, t.min, t.max
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
        format!("recebidos: {received}")
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
                        ui.horizontal(|ui| crate::models::toggle(ui, &mut visual, &mut lighting));
                    }
                    sim_controls(
                        ui,
                        &data,
                        &mut pb,
                        &mut selection,
                        &mut fluid_sel,
                        &mut editor,
                    )
                });
        });
    dock.free = ui.available_rect_before_wrap();

    let (cam, cam_tf) = *camera;
    let labels: Vec<Label> = data
        .replay
        .blocks
        .iter()
        .enumerate()
        .filter(|(i, _)| !data.is_hidden(*i))
        .map(|(i, block)| {
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
) {
    let tl = &data.timeline;
    ui.heading("Simulação");
    ui.label(format!(
        "{} - replicação {}",
        data.replay.model, data.replay.replication
    ));
    ui.label(format!(
        "{} blocos, {} itens, {} eventos de rastro",
        tl.block_count(),
        tl.item_count(),
        tl.record_count()
    ));
    ui.separator();

    ui.horizontal_wrapped(|ui| {
        let label = if pb.playing { "Pausar" } else { "Reproduzir" };
        if ui.button(label).clicked() {
            if pb.t >= data.end {
                pb.t = 0.0;
            }
            pb.playing = !pb.playing;
        }
        if ui.button("< Evento").clicked() {
            pb.playing = false;
            let t = pb.t;
            pb.t = tl.prev_event_time(t).unwrap_or(0.0);
        }
        if ui.button("Evento >").clicked() {
            pb.playing = false;
            let t = pb.t;
            pb.t = tl.next_event_time(t).unwrap_or(data.end).min(data.end);
        }
        if ui.button("Início").clicked() {
            pb.t = 0.0;
        }
        if ui.button("Enquadrar (F)").clicked() {
            editor.frame_request = true;
        }
    });
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            pb.slower();
        }
        ui.label(format!("velocidade x{}", SPEEDS[pb.speed_idx]));
        if ui.button("+").clicked() {
            pb.faster();
        }
    });
    ui.weak("unidades de tempo simulado por segundo");
    let mut t = pb.t;
    let slider = egui::Slider::new(&mut t, 0.0..=data.end.max(f64::MIN_POSITIVE))
        .text("tempo")
        .smart_aim(false);
    if ui.add(slider).changed() {
        pb.t = t;
        pb.playing = false;
    }
    ui.label(format!("tempo simulado: {:.4} de {:.4}", pb.t, data.end));
    ui.weak("Espaço: reproduzir/pausar | +/-: velocidade | setas: evento anterior/próximo | F: enquadrar");
    ui.weak("Mouse: esquerdo arrasta (move) e clica (seleciona), direito gira, roda aproxima");
    ui.separator();

    ui.strong("Blocos");
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
        ui.strong("Fluidos");
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
            ui.label("Clique em um bloco para ver suas estatísticas.");
        }
        Some(sel) => block_stats(ui, data, sel, pb.t),
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
                text: format!("capacidade {capacity}"),
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
                    row(ui, "nível agora", fmt_num(sample.level));
                    row(ui, "capacidade", fmt_num(*capacity));
                    row(
                        ui,
                        "ocupação",
                        fluids::percent_text(sample.level, *capacity),
                    );
                    row(
                        ui,
                        "taxa agora",
                        format!(
                            "{} por unidade de tempo ({})",
                            fmt_num(sample.rate),
                            fluids::rate_label(sample.rate)
                        ),
                    );
                    row(ui, "nível inicial", fmt_num(*initial));
                    if let Some(r) = max_in {
                        row(ui, "entrada máxima", fmt_num(*r));
                    }
                    if let Some(r) = max_out {
                        row(ui, "saída máxima", fmt_num(*r));
                    }
                }
                Some(FluidKind::Source { rate }) => {
                    row(ui, "vazão agora", fmt_num(sample.rate));
                    row(ui, "vazão da fonte", fmt_num(*rate));
                }
                Some(FluidKind::Pipe { max_rate }) => {
                    row(ui, "vazão agora", fmt_num(sample.rate));
                    row(ui, "vazão máxima", fmt_num(*max_rate));
                }
                Some(FluidKind::Sink { max_rate }) => {
                    row(ui, "vazão agora", fmt_num(sample.rate));
                    if let Some(r) = max_rate {
                        row(ui, "vazão máxima", fmt_num(*r));
                    }
                }
                Some(FluidKind::FromItems { object, volume }) => {
                    row(ui, "objeto", object.clone());
                    row(ui, "volume por item", fmt_num(*volume));
                }
                Some(FluidKind::ToItems {
                    object,
                    volume,
                    max_rate,
                }) => {
                    row(ui, "objeto", object.clone());
                    row(ui, "volume por item", fmt_num(*volume));
                    row(ui, "acumulado para o próximo item", fmt_num(sample.level));
                    row(ui, "vazão agora", fmt_num(sample.rate));
                    if let Some(r) = max_rate {
                        row(ui, "vazão máxima", fmt_num(*r));
                    }
                }
                None => {}
            }
        });
    ui.weak("Nível e taxa interpolados a partir do rastro da rede de fluidos.");
}

fn block_stats(ui: &mut egui::Ui, data: &Data, sel: usize, t: f64) {
    let block = &data.replay.blocks[sel];
    let tl = &data.timeline;
    ui.strong(format!("{} ({})", block.name, block.kind));
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
                ui.label("recebidos até agora");
                ui.label(tl.block_received(sel, t).to_string());
            } else {
                ui.label("estado agora");
                ui.label(style::state_label(state));
            }
            ui.end_row();
            ui.label("conteúdo agora");
            ui.label(tl.block_content(sel, t).to_string());
            ui.end_row();
            let Some(r) = report else {
                return;
            };
            ui.label("entradas");
            ui.label(r.input.to_string());
            ui.end_row();
            ui.label("saídas");
            ui.label(r.output.to_string());
            ui.end_row();
            ui.label("conteúdo médio");
            ui.label(format!("{:.4}", r.content_mean));
            ui.end_row();
            ui.label("conteúdo máximo");
            ui.label(format!("{}", r.content_max));
            ui.end_row();
            ui.label("tempo de permanência");
            ui.label(fmt_tally(&r.staytime));
            ui.end_row();
            for (name, frac) in &r.states {
                ui.label(format!("fração em {name}"));
                ui.label(format!("{:.2}%", frac * 100.0));
                ui.end_row();
            }
            for (name, tally) in &r.extra {
                ui.label(name);
                ui.label(fmt_tally(tally));
                ui.end_row();
            }
        });
    ui.weak("Estatísticas finais da replicação (janela após o aquecimento).");
}
