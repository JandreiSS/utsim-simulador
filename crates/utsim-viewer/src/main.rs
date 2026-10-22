//! `utsim-view`: 3D model editor (`Editar`, RF-60/RF-61) and replay viewer (`Simular`,
//! RF-50/RF-51). The simulation is never driven from the 3D view (RNF-13): a run
//! produces a `Replay` that the viewer only reads.

mod camera;
mod editor;
mod playback;
mod scene;
mod ui;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use clap::Parser;
use editor::Editor;
use playback::Data;
use utsim_edit::Document;
use utsim_model::replay::Replay;
use utsim_viewer::editing;
use utsim_viewer::layout;
use utsim_viewer::source;
use utsim_viewer::timeline::TransitSetting;

#[derive(Parser)]
#[command(
    name = "utsim-view",
    version,
    about = "Editor e visualizador 3D do UTSim: edita um modelo .ron e anima a simulação (ou abre um replay .json, somente leitura)"
)]
struct Cli {
    /// Modelo .ron (abre em Editar; é criado no primeiro salvamento se não existir)
    /// ou replay .json (de `utsim replay`; abre em Simular, somente leitura)
    input: Option<PathBuf>,
    /// Começa com um modelo vazio
    #[arg(long, conflicts_with = "input")]
    novo: bool,
    /// Replicação a usar quando a entrada é um replay .json (sem efeito em modelos)
    #[arg(long, default_value_t = 0)]
    rep: u32,
    /// Tempo de parada das simulações rápidas (padrão: o menor entre run.stop_time e 2000)
    #[arg(long)]
    stop: Option<f64>,
    /// Duração visual das transferências, em unidades de tempo (padrão: automática)
    #[arg(long)]
    transit: Option<f64>,
    /// Carrega, valida, monta a linha do tempo e o layout, imprime um resumo e sai (sem janela)
    #[arg(long)]
    headless_check: bool,
}

fn is_json(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("json"))
}

/// What to open: an editable document or a recorded replay.
enum Start {
    Edit(Box<Editor>),
    Replay(Box<Replay>),
}

fn open(cli: &Cli, transit: TransitSetting) -> Result<Start, String> {
    let edit = |doc: Document, pending: Option<PathBuf>| {
        Ok(Start::Edit(Box::new(
            Editor::new(doc, pending, cli.stop).with_transit(transit),
        )))
    };
    match &cli.input {
        Some(path) if is_json(path) => {
            source::load(path, cli.rep, cli.stop).map(|r| Start::Replay(Box::new(r)))
        }
        Some(path) if path.exists() => {
            let doc = Document::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
            edit(doc, None)
        }
        Some(path) => {
            if path.extension().and_then(|e| e.to_str()) != Some("ron") {
                return Err(format!(
                    "extensão não reconhecida em {} (use .ron ou .json)",
                    path.display()
                ));
            }
            let name = path
                .file_stem()
                .map_or("novo".to_string(), |s| s.to_string_lossy().to_string());
            edit(Document::new(&name), Some(path.clone()))
        }
        None if cli.novo => edit(Document::new("novo"), None),
        None => Err("informe um arquivo .ron/.json ou use --novo".into()),
    }
}

fn headless_check(start: &Start, transit: TransitSetting) -> Result<(), String> {
    match start {
        Start::Edit(ed) => {
            println!("modo: edição ({})", ed.file_name());
            println!("objetos: {}", ed.doc.model().objects.len());
            println!("ligações: {}", ed.doc.model().connections.len());
            println!("problemas: {}", ed.issues.len());
            for issue in &ed.issues {
                let object = issue.object.as_deref().unwrap_or("modelo");
                println!("  - {object}: {}", issue.message);
            }
            println!(
                "layout: {}",
                if ed.doc.is_dirty() {
                    "automático aplicado (ainda não salvo)"
                } else {
                    "do modelo"
                }
            );
            if ed.issues.is_empty() && !ed.doc.model().objects.is_empty() {
                let stop: f64 = ed.stop_text.parse().map_err(|_| "tempo inválido")?;
                let model = editing::run_model(ed.doc.model(), stop);
                let replay = model.replay(0).map_err(|e| e.to_string())?;
                summary(&Data::from_replay(replay, transit));
            }
        }
        Start::Replay(replay) => {
            println!("modo: simulação (somente leitura)");
            summary(&Data::from_replay((**replay).clone(), transit));
        }
    }
    Ok(())
}

fn summary(data: &Data) {
    let tl = &data.timeline;
    println!(
        "modelo: {} (replicação {})",
        data.replay.model, data.replay.replication
    );
    println!("blocos: {}", tl.block_count());
    println!("itens: {}", tl.item_count());
    println!("eventos de rastro: {}", tl.record_count());
    println!(
        "intervalo de tempo: 0 a {} (último evento em {})",
        data.end,
        tl.last_event_time()
    );
    let raw: Vec<[f64; 3]> = data.replay.blocks.iter().map(|b| b.position).collect();
    println!(
        "layout: {} ({} posições)",
        if layout::all_at_origin(&raw) {
            "automático"
        } else {
            "do modelo"
        },
        data.positions.len()
    );
    println!("transferência visual padrão: {:.6}", tl.transit_default());
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let transit = cli
        .transit
        .map_or(TransitSetting::Auto, TransitSetting::Fixed);
    let start = match open(&cli, transit) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("erro: {e}");
            return ExitCode::FAILURE;
        }
    };
    if cli.headless_check {
        return match headless_check(&start, transit) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("erro: {e}");
                ExitCode::FAILURE
            }
        };
    }

    let (editor, data) = match start {
        Start::Edit(ed) => (*ed, Data::from_replay(editing::empty_replay(), transit)),
        Start::Replay(replay) => (
            Editor::read_only().with_transit(transit),
            Data::from_replay(*replay, transit),
        ),
    };
    let positions: &[[f32; 3]] = if editor.editable {
        &editor.view.positions
    } else {
        &data.positions
    };
    let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for p in positions {
        min = min.min(Vec3::from(*p));
        max = max.max(Vec3::from(*p));
    }
    if positions.is_empty() {
        (min, max) = (Vec3::ZERO, Vec3::ZERO);
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "UTSim".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_resource(camera::OrbitCam::framing(min, max))
        .insert_resource(camera::CameraLock::default())
        .insert_resource(playback::Playback::new(data.end))
        .insert_resource(playback::Selection::default())
        .insert_resource(data)
        .insert_resource(editor)
        .add_systems(Startup, scene::setup)
        .add_systems(
            Update,
            (
                editor::refresh,
                editor::poll_job,
                (editor::keys, editor::mouse)
                    .chain()
                    .run_if(editor::in_edit),
                playback::keyboard.run_if(editor::in_sim),
                playback::advance_clock.run_if(editor::in_sim),
                camera::control,
                scene::pick.run_if(editor::in_sim),
                scene::sync_blocks,
                (scene::update_block_colors, scene::update_items)
                    .chain()
                    .run_if(editor::in_sim),
                scene::draw_sim.run_if(editor::in_sim),
                editor::draw_edit.run_if(editor::in_edit),
                editor::title,
            )
                .chain(),
        )
        .add_systems(
            EguiPrimaryContextPass,
            (
                editor::toolbar,
                editor::edit_panels.run_if(editor::in_edit),
                ui::panel.run_if(editor::in_sim),
            )
                .chain(),
        )
        .run();
    ExitCode::SUCCESS
}
