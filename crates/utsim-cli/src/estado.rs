//! `utsim run --estado` and `--salvar-estado`: start from a given state and save the state
//! at an instant, so runs can be chained.

use std::path::{Path, PathBuf};

use utsim_model::{InitialState, Model};

/// Reads the initial state, applies `--warmup` to it and checks it against the model,
/// printing the warnings. The model's own warm-up gives way to the state's (0 by
/// default), so a short `--stop` from a state is not refused because of it.
pub fn load(
    model: &mut Model,
    path: &Path,
    warmup: Option<f64>,
) -> Result<InitialState, Box<dyn std::error::Error>> {
    let mut state = InitialState::load(path)?;
    if warmup.is_some() {
        state.warmup = warmup;
    }
    model.run.warmup = state.warmup.unwrap_or(0.0);
    model.validate()?;
    for warning in model.check_state(&state)? {
        eprintln!("aviso: {warning}");
    }
    println!(
        "partindo do estado de {} ({} objeto(s), relógio {}, aquecimento {})",
        path.display(),
        state.objects.len(),
        state.clock,
        state.warmup.unwrap_or(0.0)
    );
    Ok(state)
}

/// `--salvar-estado T ARQUIVO`: runs replication 0 up to `T` and writes its state.
pub fn save(
    model: &Model,
    start: Option<&InitialState>,
    args: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let [t, path] = args else {
        return Err("--salvar-estado precisa de dois valores: o instante e o arquivo".into());
    };
    let t: f64 = t
        .parse()
        .map_err(|_| format!("--salvar-estado: '{t}' não é um instante (número)"))?;
    let path = PathBuf::from(path);
    let (state, notes) = model.state_at(t, start, 0)?;
    for note in notes {
        eprintln!("aviso: {note}");
    }
    state.save(&path)?;
    println!(
        "estado da replicação 0 em t={t} gravado em {} (relógio {})",
        path.display(),
        state.clock
    );
    Ok(())
}
