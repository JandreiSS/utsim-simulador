use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use utsim_emu::{
    CommandLog, Emulator, OpcUaFrontend, RunOptions, SystemClock, replay as replay_log, run_paced,
};
use utsim_model::Model;

pub struct Args {
    pub model: PathBuf,
    pub port: u16,
    pub speed: f64,
    pub until: Option<f64>,
    pub record: Option<PathBuf>,
    pub replay: Option<PathBuf>,
}

/// `utsim emular`: real-time emulation over OPC UA, or replay of a command log.
pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let model = Model::load(&args.model)?;
    model.validate()?;
    let mut emu = Emulator::new(model.build(0))?;

    if let Some(path) = &args.replay {
        let log = CommandLog::load(path)?;
        println!(
            "{}: reproduzindo {} comando(s) de {}",
            model.name,
            log.commands.len(),
            path.display()
        );
        let report = replay_log(&mut emu, &log, args.until)?;
        crate::print_summary(&model, &[report]);
        return Ok(());
    }

    let names = emu.block_names().to_vec();
    let mut frontend = OpcUaFrontend::start(&model.name, &names, args.port, args.speed)?;
    let cancel = Arc::new(AtomicBool::new(false));
    frontend.cancel_on_ctrl_c(cancel.clone());
    println!("Servidor OPC UA em {}", frontend.endpoint());
    println!(
        "Modelo \"{}\": velocidade {}x, endereços em UTSim/{} (Ctrl-C encerra)",
        model.name, args.speed, model.name
    );
    let opts = RunOptions {
        speed: args.speed,
        until: args.until,
        cancel: Some(cancel),
        ..Default::default()
    };
    let outcome = run_paced(&mut emu, &SystemClock::new(), &mut frontend, &opts)?;
    println!("\nEmulação encerrada em t={}\n", outcome.report.end_time);
    crate::print_summary(&model, std::slice::from_ref(&outcome.report));
    if let Some(path) = &args.record {
        outcome.log.save(path)?;
        println!(
            "\n{} comando(s) gravado(s) em {}",
            outcome.log.commands.len(),
            path.display()
        );
    }
    Ok(())
}
