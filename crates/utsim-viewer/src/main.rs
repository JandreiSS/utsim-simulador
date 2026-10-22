//! `utsim-view`: read-only 3D viewer that animates a recorded replay (RF-50, RF-51).
//! The simulation is never driven from here (RNF-13): the viewer only reads a `Replay`.

mod camera;
mod playback;
mod scene;
mod ui;

use std::path::PathBuf;
use std::process::ExitCode;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use clap::Parser;
use utsim_viewer::layout;
use utsim_viewer::source;
use utsim_viewer::timeline::{Timeline, TransitSetting};

#[derive(Parser)]
#[command(
    name = "utsim-view",
    version,
    about = "Visualizador 3D do UTSim: anima um replay gravado (somente leitura)"
)]
struct Cli {
    /// Modelo .ron (simulado na hora) ou replay .json (de `utsim replay`)
    input: PathBuf,
    /// Replicação a gravar quando a entrada é um modelo
    #[arg(long, default_value_t = 0)]
    rep: u32,
    /// Tempo de parada (sobrescreve run.stop_time); evita replays gigantes
    #[arg(long)]
    stop: Option<f64>,
    /// Duração visual das transferências, em unidades de tempo (padrão: automática)
    #[arg(long)]
    transit: Option<f64>,
    /// Carrega, monta a linha do tempo e o layout, imprime um resumo e sai (sem janela)
    #[arg(long)]
    headless_check: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let replay = match source::load(&cli.input, cli.rep, cli.stop) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("erro: {e}");
            return ExitCode::FAILURE;
        }
    };
    let setting = cli
        .transit
        .map_or(TransitSetting::Auto, TransitSetting::Fixed);
    let timeline = Timeline::build(&replay, setting);
    let raw: Vec<[f64; 3]> = replay.blocks.iter().map(|b| b.position).collect();
    let automatic = layout::all_at_origin(&raw);
    let positions = layout::place(&raw, &replay.connections);
    let end = if replay.stop_time.is_finite() {
        replay.stop_time
    } else {
        timeline.last_event_time()
    }
    .max(timeline.last_event_time());

    if cli.headless_check {
        println!(
            "modelo: {} (replicação {})",
            replay.model, replay.replication
        );
        println!("blocos: {}", timeline.block_count());
        println!("itens: {}", timeline.item_count());
        println!("eventos de rastro: {}", timeline.record_count());
        println!(
            "intervalo de tempo: 0 a {end} (último evento em {})",
            timeline.last_event_time()
        );
        println!(
            "layout: {} ({} posições)",
            if automatic {
                "automático"
            } else {
                "do modelo"
            },
            positions.len()
        );
        println!(
            "transferência visual padrão: {:.6}",
            timeline.transit_default()
        );
        return ExitCode::SUCCESS;
    }

    let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for p in &positions {
        min = min.min(Vec3::from(*p));
        max = max.max(Vec3::from(*p));
    }
    if positions.is_empty() {
        (min, max) = (Vec3::ZERO, Vec3::ZERO);
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("UTSim - {}", replay.model),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_resource(camera::OrbitCam::framing(min, max))
        .insert_resource(playback::Playback::new(end))
        .insert_resource(playback::Selection::default())
        .insert_resource(playback::Data {
            replay,
            timeline,
            positions,
            end,
        })
        .add_systems(Startup, scene::setup)
        .add_systems(
            Update,
            (
                playback::keyboard,
                playback::advance_clock,
                camera::control,
                scene::pick,
                (
                    scene::update_block_colors,
                    scene::update_items,
                    scene::draw_gizmos,
                ),
            )
                .chain(),
        )
        .add_systems(EguiPrimaryContextPass, ui::panel)
        .run();
    ExitCode::SUCCESS
}
