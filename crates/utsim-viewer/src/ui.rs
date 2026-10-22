//! egui control panel, block statistics and in-scene labels (Portuguese UI).

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use utsim_core::TallySummary;
use utsim_viewer::style;

use crate::playback::{Data, Playback, SPEEDS, Selection};

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
    pub kind: String,
    pub selected: bool,
}

/// Block names above the blocks (shared by both modes).
pub fn draw_labels(ctx: &egui::Context, cam: &Camera, cam_tf: &GlobalTransform, labels: &[Label]) {
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Background,
        egui::Id::new("rotulos"),
    ));
    for label in labels {
        let (half, _) = style::extents(style::shape_of(&label.kind));
        let world = Vec3::from(label.position) + Vec3::Y * (half[1] * 2.0 + 0.9);
        if let Ok(p) = cam.world_to_viewport(cam_tf, world) {
            let fg = if label.selected {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_gray(190)
            };
            painter.text(
                egui::pos2(p.x, p.y),
                egui::Align2::CENTER_BOTTOM,
                &label.text,
                egui::FontId::proportional(13.0),
                fg,
            );
        }
    }
}

pub fn panel(
    mut contexts: EguiContexts,
    data: Res<Data>,
    mut pb: ResMut<Playback>,
    mut selection: ResMut<Selection>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let tl = &data.timeline;

    let (cam, cam_tf) = *camera;
    let labels: Vec<Label> = data
        .replay
        .blocks
        .iter()
        .enumerate()
        .map(|(i, block)| {
            let content = tl.block_content(i, pb.t);
            Label {
                text: if content > 0 {
                    format!("{} ({content})", block.name)
                } else {
                    block.name.clone()
                },
                position: data.positions[i],
                kind: block.kind.clone(),
                selected: selection.0 == Some(i),
            }
        })
        .collect();
    draw_labels(ctx, cam, cam_tf, &labels);

    egui::Window::new("UTSim - visualizador")
        .default_pos(egui::pos2(10.0, 10.0))
        .default_width(330.0)
        .show(ctx, |ui| {
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

            ui.horizontal(|ui| {
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
            });
            ui.horizontal(|ui| {
                if ui.button("-").clicked() {
                    pb.slower();
                }
                ui.label(format!(
                    "velocidade x{} (unidades de tempo por segundo)",
                    SPEEDS[pb.speed_idx]
                ));
                if ui.button("+").clicked() {
                    pb.faster();
                }
            });
            let mut t = pb.t;
            let slider = egui::Slider::new(&mut t, 0.0..=data.end.max(f64::MIN_POSITIVE))
                .text("tempo")
                .smart_aim(false);
            if ui.add(slider).changed() {
                pb.t = t;
                pb.playing = false;
            }
            ui.label(format!("tempo simulado: {:.4} de {:.4}", pb.t, data.end));
            ui.weak("Espaço: reproduzir/pausar | +/-: velocidade | setas: evento anterior/próximo");
            ui.weak(
                "Mouse: esquerdo arrasta (move) e clica (seleciona), direito gira, roda aproxima",
            );
            ui.separator();

            ui.strong("Blocos");
            egui::ScrollArea::vertical()
                .id_salt("lista_blocos")
                .max_height(150.0)
                .show(ui, |ui| {
                    for (i, block) in data.replay.blocks.iter().enumerate() {
                        let state = tl.block_state(i, pb.t);
                        let text = format!(
                            "{} [{}]  {}  ({})",
                            block.name,
                            block.kind,
                            style::state_label(state),
                            tl.block_content(i, pb.t)
                        );
                        let on = selection.0 == Some(i);
                        if ui.selectable_label(on, text).clicked() {
                            selection.0 = if on { None } else { Some(i) };
                        }
                    }
                });
            ui.separator();

            match selection.0.filter(|s| *s < data.replay.blocks.len()) {
                None => {
                    ui.label("Clique em um bloco para ver suas estatísticas.");
                }
                Some(sel) => block_stats(ui, &data, sel, pb.t),
            }
        });
    Ok(())
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
            ui.label("estado agora");
            ui.label(style::state_label(state));
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
