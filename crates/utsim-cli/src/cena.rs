use std::io::{BufWriter, Write};
use std::path::Path;

use utsim_emu::{Emulator, NullFrontend, RunOptions, SceneFrontend, SystemClock, run_paced};
use utsim_model::Model;
use utsim_scene::{JsonLines, Scene, SceneOptions, SceneSink};

/// `utsim cena`: writes the scene stream of one replication (header, then one frame
/// every `1 / fps` of simulated time up to `until`, or the end of the run).
pub fn run(
    model: &Path,
    replication: u32,
    fps: f64,
    until: Option<f64>,
    out: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !(fps.is_finite() && fps > 0.0) {
        return Err("--fps deve ser positivo".into());
    }
    let model = Model::load(model)?;
    let replay = model.replay(replication)?;
    let scene = Scene::build(
        &replay,
        &SceneOptions {
            model: Some(&model),
            ..SceneOptions::default()
        },
    );
    let end = until.unwrap_or(scene.end).min(scene.end).max(0.0);
    let writer: Box<dyn Write> = match out {
        Some(path) => Box::new(std::fs::File::create(path)?),
        None => Box::new(std::io::stdout().lock()),
    };
    let mut sink = JsonLines(BufWriter::new(writer));
    sink.header(&scene.header())?;
    // Frame k at k / fps (computed from k, so no drift accumulates).
    let mut frames = 0u64;
    loop {
        let t = frames as f64 / fps;
        if t > end + 1e-9 {
            break;
        }
        sink.frame(&scene.frame(t))?;
        frames += 1;
    }
    sink.0.flush()?;
    if let Some(path) = out {
        eprintln!(
            "{}: replicação {replication}, {frames} quadros de 0 a {end} gravados em {}",
            replay.model,
            path.display()
        );
    }
    Ok(())
}

/// `utsim cena --servir`: runs the model paced against the wall clock, without OPC UA,
/// and serves the live scene on `addr:port` (TCP and WebSocket). It is the emulation
/// loop with only the scene attached, so clients can also send commands (stop a block,
/// change the speed). For developing a scene client without a PLC.
pub fn serve(
    model: &Path,
    replication: u32,
    port: u16,
    addr: &str,
    fps: f64,
    speed: f64,
    until: Option<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !(speed.is_finite() && speed > 0.0) {
        return Err("--velocidade deve ser positiva".into());
    }
    let model = Model::load(model)?;
    model.validate()?;
    let mut emu = Emulator::new_traced(model.build(replication))?;
    let names = emu.block_names().to_vec();
    let mut frontend = SceneFrontend::start(NullFrontend, (addr, port), &model, &names, fps)?;
    println!(
        "Cena 3D ao vivo de \"{}\" em {} (TCP e WebSocket, protocolo utsim-scene/1)",
        model.name,
        frontend.live().local_addr()
    );
    println!(
        "Velocidade {speed}x; a rodada termina em t={}",
        until.unwrap_or(model.run.stop_time)
    );
    crate::emulate::warn_if_open(addr);
    let opts = RunOptions {
        speed,
        until,
        ..Default::default()
    };
    let outcome = run_paced(&mut emu, &SystemClock::new(), &mut frontend, &opts)?;
    println!("Rodada encerrada em t={}", outcome.report.end_time);
    Ok(())
}
