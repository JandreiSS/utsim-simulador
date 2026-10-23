//! `utsim-view`: 3D model editor (`Editar`, RF-60/RF-61) and replay viewer (`Simular`,
//! RF-50/RF-51). The simulation is never driven from the 3D view (RNF-13): a run
//! produces a `Replay` that the viewer only reads.

mod camera;
mod dock;
mod editor;
mod fluid_scene;
mod playback;
mod scene;
#[cfg(not(target_arch = "wasm32"))]
mod script;
mod ui;
#[cfg(target_arch = "wasm32")]
mod web;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use editor::Editor;
use playback::Data;

// Everything that touches files, threads, the command line or the process exit code is
// native-only: in the browser (`wasm32`) the entry point lives in `web.rs`.
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};
#[cfg(not(target_arch = "wasm32"))]
use std::process::ExitCode;

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
#[cfg(not(target_arch = "wasm32"))]
use utsim_edit::Document;
#[cfg(not(target_arch = "wasm32"))]
use utsim_model::replay::Replay;
#[cfg(not(target_arch = "wasm32"))]
use utsim_viewer::editing;
#[cfg(not(target_arch = "wasm32"))]
use utsim_viewer::layout;
#[cfg(not(target_arch = "wasm32"))]
use utsim_viewer::source;
#[cfg(not(target_arch = "wasm32"))]
use utsim_viewer::timeline::TransitSetting;

#[cfg(not(target_arch = "wasm32"))]
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
    /// Roteiro .ron de passos com entrada sintética (ver docs/testes.md); exige --capturas.
    /// Um modelo .ron existente é copiado para a pasta de capturas e a cópia é a editada.
    #[arg(long, requires = "capturas")]
    roteiro: Option<PathBuf>,
    /// Pasta onde o modo roteirizado grava capturas de tela (.png) e log.txt
    #[arg(long)]
    capturas: Option<PathBuf>,
}

#[cfg(not(target_arch = "wasm32"))]
fn is_json(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("json"))
}

#[cfg(not(target_arch = "wasm32"))]
/// What to open: an editable document or a recorded replay.
enum Start {
    Edit(Box<Editor>),
    Replay(Box<Replay>),
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
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
                summary(&Data::new(replay, transit, Some(&model)));
            }
        }
        Start::Replay(replay) => {
            println!("modo: simulação (somente leitura)");
            summary(&Data::from_replay((**replay).clone(), transit));
        }
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
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
        "transportadores: {}",
        data.replay
            .blocks
            .iter()
            .filter(|b| b.kind == "Transporter")
            .count()
    );
    println!("viagens: {}", tl.travel_count());
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

/// Adds the UTSim plugins, resources and systems to an app that already has the Bevy
/// default plugins (shared by the native binary and the browser entry point).
fn configure(app: &mut App, editor: Editor, data: Data) -> &mut App {
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

    app.add_plugins(EguiPlugin::default())
        .insert_resource(camera::OrbitCam::framing(min, max))
        .insert_resource(camera::CameraLock::default())
        .insert_resource(camera::FrameRequest(8))
        .insert_resource(dock::Dock::default())
        .insert_resource(playback::Playback::new(data.end))
        .insert_resource(playback::Selection::default())
        .insert_resource(playback::FluidSelection::default())
        .insert_resource(fluid_scene::FluidScene::default())
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
                camera::frame_scene,
                camera::control,
                scene::pick.run_if(editor::in_sim),
                scene::sync_blocks,
                (
                    scene::update_carriers,
                    scene::update_block_colors,
                    scene::update_items,
                )
                    .chain()
                    .run_if(editor::in_sim),
                scene::draw_sim.run_if(editor::in_sim),
                fluid_scene::sync_fluids,
                fluid_scene::update_fluids.run_if(editor::in_sim),
                fluid_scene::draw_fluids.run_if(editor::in_sim),
                editor::draw_edit.run_if(editor::in_edit),
                editor::title,
            )
                .chain(),
        )
        .add_systems(
            EguiPrimaryContextPass,
            (
                dock::begin,
                editor::toolbar,
                editor::flows_panel.run_if(editor::flows_open),
                editor::edit_panels.run_if(editor::in_edit),
                ui::panel.run_if(editor::in_sim),
                dock::apply_viewport,
            )
                .chain(),
        );
    app
}

#[cfg(target_arch = "wasm32")]
fn main() {
    web::main();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> ExitCode {
    let mut cli = Cli::parse();
    let steps = match scripted_steps(&mut cli) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("erro: {e}");
            return ExitCode::FAILURE;
        }
    };
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
    let mut window = Window {
        title: "UTSim".to_string(),
        ..default()
    };
    if steps.is_some() {
        // Predictable captures: fixed size, one pixel per logical pixel.
        window.resolution =
            bevy::window::WindowResolution::new(1280, 800).with_scale_factor_override(1.0);
    }
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }));
    if let (Some(steps), Some(dir)) = (steps, cli.capturas.clone()) {
        app.add_plugins(script::ScriptPlugin { steps, dir });
    }
    let exit = configure(&mut app, editor, data).run();
    match exit {
        AppExit::Success => ExitCode::SUCCESS,
        AppExit::Error(_) => ExitCode::FAILURE,
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Parses the script, if any, and redirects an existing `.ron` input to a copy in the
/// captures directory, so scripts never modify the original.
fn scripted_steps(cli: &mut Cli) -> Result<Option<Vec<utsim_viewer::roteiro::Passo>>, String> {
    let Some(script) = &cli.roteiro else {
        return Ok(None);
    };
    let text = std::fs::read_to_string(script).map_err(|e| format!("{}: {e}", script.display()))?;
    let steps = utsim_viewer::roteiro::parse(&text)?;
    let dir = cli.capturas.clone().ok_or("--roteiro exige --capturas")?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    if let Some(input) = cli.input.clone()
        && !is_json(&input)
        && input.exists()
    {
        let name = input.file_name().ok_or("entrada sem nome de arquivo")?;
        let copy = dir.join(name);
        std::fs::copy(&input, &copy).map_err(|e| format!("copiando {}: {e}", input.display()))?;
        cli.input = Some(copy);
    }
    Ok(Some(steps))
}
