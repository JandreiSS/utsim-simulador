//! Synthetic factory for the twin's tests: a UTSim model run by `utsim-emu` plays the
//! real plant, publishing what its OPC UA server would publish (content, state, counters
//! of each block) every `dt`, only the values that changed.

#![allow(dead_code)]

pub mod pb;

use utsim_emu::Emulator;
use utsim_model::Model;
use utsim_twin::obs::Value;

/// The model of the tests: arrivals every 1.0 on average, one machine (cycle `cycle`
/// on average), ρ = cycle.
pub fn model_text(cycle: f64, stop: f64) -> String {
    format!(
        r#"Model(
    name: "linha",
    run: (stop_time: {stop:.1}, warmup: 400.0, seed: 4),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0))),
        (name: "fila", kind: Queue()),
        (name: "maquina", kind: Processor(process: Exp(mean: {cycle}))),
        (name: "saida", kind: Sink),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#
    )
}

pub fn model(cycle: f64, stop: f64) -> Model {
    Model::parse(&model_text(cycle, stop)).unwrap()
}

/// The `.gemeo.ron` of the tests (OPC UA addresses of `utsim emular`).
pub fn config_text(endpoint: &str, window: f64, reps: u32) -> String {
    format!(
        r#"Twin(
    model: "linha.ron",
    clock: Tag(source: "clp", tag: "UTSim/linha/TempoSimulado"),
    sources: [
        (name: "clp", kind: OpcUa(endpoint: "{endpoint}", namespace: "urn:utsim:emulacao", interval_ms: 20)),
    ],
    tags: [
        (source: "clp", tag: "UTSim/linha/maquina/Saidas", object: "maquina", measure: Outputs),
        (source: "clp", tag: "UTSim/linha/maquina/Estado", object: "maquina", measure: State),
        (source: "clp", tag: "UTSim/linha/fila/Conteudo", object: "fila", measure: Content),
    ],
    shadow: (window: {window:.1}, replications: {reps}, warmup: 400.0, samples_per_window: 20),
)"#
    )
}

/// What the plant publishes: `(time, tag, value)`, the clock tag first at each step.
pub fn plant(model: &Model, replication: u32, until: f64, dt: f64) -> Vec<(f64, String, Value)> {
    let mut emu = Emulator::new(model.build(replication)).unwrap();
    let mut out = Vec::new();
    let mut last: Vec<(String, Value)> = Vec::new();
    let mut t = 0.0;
    while t <= until {
        emu.advance(t).unwrap();
        let s = emu.snapshot(1.0);
        out.push((t, "UTSim/linha/TempoSimulado".to_string(), Value::Num(t)));
        for b in &s.blocks {
            for (var, v) in [
                ("Conteudo", Value::Num(f64::from(b.content))),
                ("Estado", Value::Text(b.state.clone())),
                ("Saidas", Value::Num(b.outputs as f64)),
            ] {
                let tag = format!("UTSim/linha/{}/{var}", b.name);
                match last.iter_mut().find(|(k, _)| *k == tag) {
                    Some((_, old)) if *old == v => continue,
                    Some((_, old)) => *old = v.clone(),
                    None => last.push((tag.clone(), v.clone())),
                }
                out.push((t, tag, v));
            }
        }
        t += dt;
    }
    out
}
