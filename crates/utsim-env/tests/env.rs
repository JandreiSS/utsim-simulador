//! Environment contract: determinism, reward accounting on a model computed by hand,
//! and errors. The learning tests are in `learning.rs`.

use utsim_env::{Env, EnvConfig, EnvError, RewardTerm};
use utsim_model::Model;

/// Items arrive at t = 1, 2, 3, 4, 5 (the run ends at 5.5). Option 0 sends the item to
/// "a" (it stays 0.5), option 1 to "b" (it stays 2.0). Both have room for everyone,
/// so the content of each block over time is known exactly.
const HAND: &str = r#"Model(
    name: "conta a mao",
    run: (stop_time: 5.5, seed: 1),
    objects: [
        (name: "a", kind: Processor(capacity: 10, process: Const(0.5))),
        (name: "b", kind: Processor(capacity: 10, process: Const(2.0))),
        (name: "saida", kind: Sink),
    ],
    connections: [("a", "saida"), ("b", "saida")],
    flows: [(
        name: "chegadas",
        activities: [
            (name: "chega", kind: Source(interarrival: Const(1.0)), next: ["escolhe"]),
            (name: "escolhe", kind: Decide(rule: Agent(observation: ["now"])), next: ["vai_a", "vai_b"]),
            (name: "vai_a", kind: CreateItem(into: "a"), next: ["fim"]),
            (name: "vai_b", kind: CreateItem(into: "b"), next: ["fim"]),
            (name: "fim", kind: Sink),
        ],
    )],
)"#;

fn hand_config() -> EnvConfig {
    EnvConfig {
        // Reward = -(area under the number of items in "a" and in "b").
        reward: vec![
            RewardTerm::integral("a.content.mean", -1.0),
            RewardTerm::integral("b.content.mean", -1.0),
        ],
        observe_blocks: vec!["a".into(), "b".into(), "a+b".into()],
        ..EnvConfig::default()
    }
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn reward_is_the_hand_computed_area_between_decisions() {
    let model = Model::parse(HAND).unwrap();
    let mut env = Env::new(model, hand_config()).unwrap();
    let obs = env.reset(1).unwrap();
    assert_eq!(obs.options, 2);
    assert!(close(obs.time, 1.0));
    // [model observation (now), content a, content b, content a+b]
    assert_eq!(obs.values, vec![1.0, 0.0, 0.0, 0.0]);

    // Choices a, b, a, b, a.
    let actions = [0, 1, 0, 1, 0];
    // Area between consecutive decisions:
    // [1,2]: a 0.5 | [2,3]: b 1 | [3,4]: a 0.5 + b 1 | [4,5]: b 1 | [5,5.5]: a 0.5 + b 0.5
    let expected = [-0.5, -1.0, -1.5, -1.0, -1.0];
    let times = [2.0, 3.0, 4.0, 5.0];
    let mut total = 0.0;
    for (k, (&a, &want)) in actions.iter().zip(&expected).enumerate() {
        let r = env.step(a).unwrap();
        assert!(close(r.reward, want), "passo {k}: {} != {want}", r.reward);
        total += r.reward;
        assert_eq!(r.info.decisions, k as u64 + 1);
        if k < 4 {
            assert!(!r.done);
            assert!(close(r.observation.time, times[k]));
            assert_eq!(r.observation.options, 2);
            assert_eq!(r.info.block, "chegadas");
            assert!(r.info.metrics.is_none());
        } else {
            assert!(r.done);
            assert_eq!(r.observation.options, 0);
            assert!(close(r.info.time, 5.5));
            // Final metrics: 3 items went through a and 2 through b; the one in b
            // since t = 4 would leave at 6.0, after the end of the run.
            let m = r.info.metrics.expect("métricas finais");
            assert_eq!(m["saida.input"], 4.0);
            assert_eq!(m["a.input"], 3.0);
            assert_eq!(m["b.input"], 2.0);
        }
    }
    // Total = -(sum of the time in the system) = -(3·0.5 + 2.0 + 2.0 clipped at 1.5).
    assert!(close(total, -5.0), "{total}");
    assert!(matches!(env.step(0), Err(EnvError::Done)));
}

#[test]
fn observations_follow_the_live_state() {
    let model = Model::parse(HAND).unwrap();
    let mut env = Env::new(model, hand_config()).unwrap();
    env.reset(1).unwrap();
    // Item 1 to "b" (stays until t = 3): at the decision of t = 2 it is inside b.
    let r = env.step(1).unwrap();
    assert_eq!(r.observation.values, vec![2.0, 0.0, 1.0, 1.0]);
    // Item 2 to "b" as well: at t = 3 item 1 has just left (3.0 = 1 + 2), item 2 is in.
    let r = env.step(1).unwrap();
    assert_eq!(r.observation.values[0], 3.0);
    assert_eq!(r.observation.values[2], 1.0);
}

#[test]
fn decision_cost_is_charged_at_every_step() {
    let model = Model::parse(HAND).unwrap();
    let config = EnvConfig {
        decision_cost: 0.25,
        ..hand_config()
    };
    let mut env = Env::new(model, config).unwrap();
    env.reset(1).unwrap();
    let r = env.step(0).unwrap();
    assert!(close(r.reward, -0.5 - 0.25));
}

#[test]
fn episode_length_replaces_the_stop_time() {
    let model = Model::parse(HAND).unwrap();
    let config = EnvConfig {
        episode_length: Some(2.5),
        ..hand_config()
    };
    let mut env = Env::new(model, config).unwrap();
    env.reset(1).unwrap();
    let r = env.step(0).unwrap(); // decision at t = 2
    assert!(!r.done);
    let r = env.step(0).unwrap(); // next arrival (t = 3) is past 2.5
    assert!(r.done);
    // [2, 2.5]: the item chosen at t = 2 spends 0.5 in "a".
    assert!(close(r.reward, -0.5), "{}", r.reward);
}

#[test]
fn counts_without_reward_terms_need_no_report() {
    let model = Model::parse(HAND).unwrap();
    let mut env = Env::new(model, EnvConfig::default()).unwrap();
    env.reset(1).unwrap();
    for _ in 0..4 {
        let r = env.step(0).unwrap();
        assert_eq!(r.reward, 0.0);
    }
}

#[test]
fn misuse_and_bad_configuration_are_reported() {
    let model = Model::parse(HAND).unwrap();
    let mut env = Env::new(model.clone(), hand_config()).unwrap();
    assert!(matches!(env.step(0), Err(EnvError::NotStarted)));
    env.reset(1).unwrap();
    assert!(env.step(7).is_err(), "opção fora do intervalo");
    let bad = EnvConfig {
        observe_blocks: vec!["nao_existe".into()],
        ..EnvConfig::default()
    };
    assert!(matches!(
        Env::new(model.clone(), bad),
        Err(EnvError::Config(_))
    ));
    let bad = EnvConfig {
        reward: vec![RewardTerm::new("nao_existe.input", 1.0)],
        ..EnvConfig::default()
    };
    assert!(matches!(Env::new(model, bad), Err(EnvError::Config(_))));
}

/// Same seed, same policy -> same trajectory, to the last bit.
#[test]
fn same_seed_gives_the_same_trajectory() {
    let text = include_str!("../../../examples/roteamento_rl.ron");
    let config = EnvConfig {
        reward: [
            "fila_rapida",
            "maquina_rapida",
            "fila_lenta",
            "maquina_lenta",
        ]
        .iter()
        .map(|b| RewardTerm::integral(format!("{b}.content.mean"), -1.0))
        .collect(),
        observe_blocks: vec![
            "fila_rapida+maquina_rapida".into(),
            "fila_lenta+maquina_lenta".into(),
        ],
        episode_length: Some(50.0),
        ..EnvConfig::default()
    };
    let play = |seed: u64| {
        let mut env = Env::new(Model::parse(text).unwrap(), config.clone()).unwrap();
        let mut obs = env.reset(seed).unwrap();
        let mut trace = Vec::new();
        let mut k = 0usize;
        while obs.options > 0 {
            // A fixed, state-dependent policy.
            let a = usize::from(obs.values[1] > obs.values[2] || k % 7 == 0);
            let r = env.step(a).unwrap();
            trace.push((obs.values.clone(), r.reward));
            obs = r.observation;
            k += 1;
            if r.done {
                break;
            }
        }
        trace
    };
    let a = play(5);
    assert!(a.len() > 50);
    assert_eq!(a, play(5));
    assert_ne!(a, play(6));
}

/// Three tokens decide in the same instant: the kernel queues the requests and the flow
/// parks the tokens in FIFO order, so answers 0, 1, 0 give two items to "a" and one to "b".
#[test]
fn parked_tokens_are_answered_in_order() {
    let text = HAND.replace(
        "Source(interarrival: Const(1.0))",
        "Schedule(times: [(1.0, 3)])",
    );
    let model = Model::parse(&text).unwrap();
    // run() cannot answer: it refuses the model.
    assert!(model.build(0).run().is_err());

    let mut sim = model.build(0);
    let mut answers = [0, 1, 0].into_iter();
    let mut asked = 0;
    loop {
        match sim.run_until(f64::INFINITY).unwrap() {
            utsim_core::Step::Decision(d) => {
                assert_eq!((d.options, d.time), (2, 1.0));
                assert_eq!(d.observation, vec![1.0]);
                sim.decide(answers.next().unwrap()).unwrap();
                asked += 1;
            }
            utsim_core::Step::Finished => break,
            utsim_core::Step::Paused(_) => unreachable!(),
        }
    }
    assert_eq!(asked, 3);
    let r = sim.finish();
    assert_eq!(r.block("a").unwrap().input, 2);
    assert_eq!(r.block("b").unwrap().input, 1);
}
