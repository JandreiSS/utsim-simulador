use std::path::Path;

use utsim_model::{Model, ModelError};

/// `utsim replay`: records one replication with its layout for the 3D viewer.
pub fn run(model: &Path, replication: u32, out: &Path) -> Result<(), ModelError> {
    let replay = Model::load(model)?.replay(replication)?;
    let json = serde_json::to_string(&replay).expect("replay serializes");
    std::fs::write(out, json)?;
    println!(
        "{}: replicação {replication}, {} eventos de rastro gravados em {}",
        replay.model,
        replay.records.len(),
        out.display()
    );
    Ok(())
}
