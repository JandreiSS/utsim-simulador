//! Paced runner and command replay, with a fake clock and no network.

use std::collections::VecDeque;
use std::time::Duration;

use utsim_core::Step;
use utsim_emu::{
    Clock, CommandLog, Emulator, FakeClock, Frontend, Input, RunOptions, Snapshot, Value, replay,
    run_paced,
};
use utsim_model::Model;

const MM1: &str = r#"Model(
    name: "mm1",
    run: (stop_time: 2000.0, seed: 4),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0))),
        (name: "fila", kind: Queue()),
        (name: "maquina", kind: Processor(process: Exp(mean: 0.8))),
        (name: "saida", kind: Sink),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#;

fn emulator() -> Emulator {
    Emulator::new(Model::parse(MM1).unwrap().build(0)).unwrap()
}

fn stop(block: &str, on: bool) -> Input {
    Input {
        variable: format!("{block}.Parar"),
        value: Value::Bool(on),
    }
}

#[test]
fn replay_equals_direct_stop_and_resume() {
    // Directly through the API.
    let m = Model::parse(MM1).unwrap();
    let mut sim = m.build(0);
    sim.run_until(500.0).unwrap();
    let maquina = sim.block_id("maquina").unwrap();
    sim.stop_block(maquina);
    sim.run_until(1500.0).unwrap();
    sim.resume_block(maquina);
    sim.run_until(2000.0).unwrap();
    let direct = sim.finish().metrics();

    // Through a command log (also through a RON round trip).
    let mut log = CommandLog::default();
    log.record(500.0, &stop("maquina", true));
    log.record(1500.0, &stop("maquina", false));
    let text = ron::ser::to_string(&log).unwrap();
    let log = CommandLog::parse(&text).unwrap();
    let mut emu = emulator();
    let replayed = replay(&mut emu, &log, None).unwrap().metrics();

    assert_eq!(replayed, direct);
    // The stop really happened.
    assert!((replayed["maquina.state.stopped"] - 0.5).abs() < 1e-9);
}

#[test]
fn pacing_follows_the_speed_factor() {
    // Frontend that records the simulated time at each publication.
    struct Probe {
        times: Vec<f64>,
    }
    impl Frontend for Probe {
        fn poll(&mut self) -> Vec<Input> {
            Vec::new()
        }
        fn publish(&mut self, s: &Snapshot) {
            self.times.push(s.time);
        }
    }
    let clock = FakeClock::new();
    let mut emu = emulator();
    let mut probe = Probe { times: Vec::new() };
    let opts = RunOptions {
        speed: 10.0,
        tick: Duration::from_millis(50),
        until: Some(50.0),
        cancel: None,
    };
    run_paced(&mut emu, &clock, &mut probe, &opts).unwrap();
    // 50 simulated units at 10x take 5 s of (fake) wall time, in 50 ms ticks.
    assert_eq!(clock.now(), Duration::from_millis(5000));
    assert_eq!(probe.times.len(), 101);
    for (i, t) in probe.times.iter().enumerate() {
        assert!((t - 0.5 * i as f64).abs() < 1e-9, "tick {i}: {t}");
    }
}

#[test]
fn live_commands_are_logged_and_reproducible() {
    // Scripted frontend: sends commands on given ticks.
    struct Script {
        tick: usize,
        plan: VecDeque<(usize, Input)>,
    }
    impl Frontend for Script {
        fn poll(&mut self) -> Vec<Input> {
            self.tick += 1;
            let mut out = Vec::new();
            while self.plan.front().is_some_and(|(t, _)| *t <= self.tick) {
                out.push(self.plan.pop_front().unwrap().1);
            }
            out
        }
        fn publish(&mut self, _s: &Snapshot) {}
    }
    let clock = FakeClock::new();
    let mut emu = emulator();
    let mut script = Script {
        tick: 0,
        plan: VecDeque::from([
            (21, stop("maquina", true)),
            (
                41,
                Input {
                    variable: "Velocidade".into(),
                    value: Value::Num(100.0),
                },
            ),
            (301, stop("maquina", false)),
        ]),
    };
    let opts = RunOptions {
        speed: 50.0,
        tick: Duration::from_millis(50),
        until: None,
        cancel: None,
    };
    let live = run_paced(&mut emu, &clock, &mut script, &opts).unwrap();
    assert_eq!(live.log.commands.len(), 3);
    assert!(live.log.commands[0].time > 0.0);

    let mut emu2 = emulator();
    let again = replay(&mut emu2, &live.log, None).unwrap();
    assert_eq!(again.metrics(), live.report.metrics());
    assert!(live.report.metrics()["maquina.state.stopped"] > 0.0);
}

#[test]
fn unknown_block_is_an_error_on_apply() {
    let mut emu = emulator();
    assert!(emu.apply(&stop("nao_existe", true)).is_err());
    assert!(
        !emu.apply(&Input {
            variable: "Velocidade".into(),
            value: Value::Num(2.0)
        })
        .unwrap()
    );
}

#[test]
fn snapshot_shows_stopped_state() {
    let mut emu = emulator();
    emu.advance(100.0).unwrap();
    emu.apply(&stop("maquina", true)).unwrap();
    assert!(matches!(emu.advance(200.0).unwrap(), Step::Paused(_)));
    let snap = emu.snapshot(1.0);
    let m = snap.blocks.iter().find(|b| b.name == "maquina").unwrap();
    assert_eq!(m.state, "stopped");
    assert!(m.stopped);
    let q = snap.blocks.iter().find(|b| b.name == "fila").unwrap();
    assert!(q.content > 10, "a fila acumula: {}", q.content);
}
