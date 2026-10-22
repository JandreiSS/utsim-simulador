//! Loading a replay from a model (`.ron`, simulated here) or a recording (`.json`).

use std::path::Path;

use utsim_model::Model;
use utsim_model::replay::Replay;

/// Loads the replay. For a `.ron` model the replication `rep` is simulated (with
/// `stop` overriding `run.stop_time`); for a `.json` recording `stop` just truncates.
pub fn load(path: &Path, rep: u32, stop: Option<f64>) -> Result<Replay, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);
    match ext.as_deref() {
        Some("json") => {
            let text = std::fs::read_to_string(path)
                .map_err(|e| format!("não foi possível ler {}: {e}", path.display()))?;
            let mut replay: Replay = serde_json::from_str(&text)
                .map_err(|e| format!("replay inválido em {}: {e}", path.display()))?;
            if let Some(stop) = stop {
                replay.records.retain(|r| r.t <= stop);
                replay.stop_time = replay.stop_time.min(stop);
            }
            Ok(replay)
        }
        Some("ron") => {
            let mut model = Model::load(path).map_err(|e| format!("{}: {e}", path.display()))?;
            if let Some(stop) = stop {
                model.run.stop_time = stop;
                // The warm-up only affects statistics; keep the model valid.
                if model.run.warmup >= stop {
                    model.run.warmup = 0.0;
                }
            }
            model
                .replay(rep)
                .map_err(|e| format!("{}: {e}", path.display()))
        }
        _ => Err(format!(
            "extensão não reconhecida em {} (use .ron ou .json)",
            path.display()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_a_model_and_honours_stop() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/mm1.ron");
        let rep = load(&path, 0, Some(50.0)).unwrap();
        assert_eq!(rep.stop_time, 50.0);
        assert!(rep.records.iter().all(|r| r.t <= 50.0));
        assert!(!rep.records.is_empty());
    }

    #[test]
    fn json_round_trip_and_truncation() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/mm1.ron");
        let rep = load(&path, 0, Some(50.0)).unwrap();
        let tmp = std::env::temp_dir().join("utsim_viewer_roundtrip.json");
        std::fs::write(&tmp, serde_json::to_string(&rep).unwrap()).unwrap();
        let back = load(&tmp, 0, Some(20.0)).unwrap();
        assert_eq!(back.stop_time, 20.0);
        assert!(back.records.iter().all(|r| r.t <= 20.0));
        assert!(back.records.len() < rep.records.len());
        std::fs::remove_file(tmp).ok();
    }

    #[test]
    fn rejects_unknown_extension() {
        assert!(load(Path::new("x.txt"), 0, None).is_err());
    }
}
