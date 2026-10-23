use std::io::{BufWriter, Write};
use std::path::Path;

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
