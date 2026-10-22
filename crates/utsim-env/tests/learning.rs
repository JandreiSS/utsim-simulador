//! Learning on `examples/roteamento_rl.ron`: random policy vs tabular Q-learning vs the
//! "join the shortest expected delay" heuristic, all on the same evaluation seeds
//! (common random numbers).
//!
//! Reward = -(area under the total number of items in the system) between decisions;
//! summed over an episode it is -(sum of the times in the system) (Little's law), so a
//! better policy has a *higher* (less negative) return.

use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use utsim_core::stats::Summary;
use utsim_env::qlearning::{Dim, QAgent, QConfig, episode_returns};
use utsim_env::{Env, EnvConfig, RewardTerm};
use utsim_model::Model;

const MODEL: &str = include_str!("../../../examples/roteamento_rl.ron");

/// Observation: `[now, items in the fast line, items in the slow line]`.
fn make_env(episode_length: f64) -> Env {
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
        episode_length: Some(episode_length),
        ..EnvConfig::default()
    };
    Env::new(Model::parse(MODEL).unwrap(), config).unwrap()
}

/// Expected time to finish if the item joins each line now (exponential services:
/// fast line = 1 machine of mean 0.8; slow line = 2 machines of mean 2.0 each, so a
/// departure every 1.0 on average once both are busy).
fn shortest_expected_delay(obs: &[f64]) -> usize {
    let (nf, ns) = (obs[1], obs[2]);
    let fast = (nf + 1.0) * 0.8;
    let slow = if ns < 2.0 { 2.0 } else { ns + 1.0 };
    usize::from(slow < fast)
}

fn random_policy(seed: u64) -> impl FnMut(&utsim_env::Observation) -> usize {
    let mut rng = Pcg64Mcg::seed_from_u64(seed);
    move |_| rng.random_range(0..2)
}

fn describe(name: &str, s: &Summary) {
    println!(
        "{name:>12}: retorno médio {:>9.1} ± {:.1} (IC 95%, n = {})",
        s.mean, s.half_width, s.n
    );
}

fn q_agent() -> QAgent {
    // Line sizes 0..=10 each get their own bin (index 1 = fast, 2 = slow); larger
    // sizes share the last bin.
    QAgent::new(
        vec![Dim::counts(1, 10), Dim::counts(2, 10)],
        2,
        QConfig {
            alpha: 0.02,
            gamma: 0.98,
            ..QConfig::default()
        },
    )
}

/// Paired difference (a - b) over the same seeds: mean and 95% half-width.
fn paired(a: &[f64], b: &[f64]) -> Summary {
    let d: Vec<f64> = a.iter().zip(b).map(|(x, y)| x - y).collect();
    Summary::from_samples(&d)
}

#[test]
fn q_learning_beats_random_and_approaches_the_heuristic() {
    const EPISODES: usize = 1000;
    let mut env = make_env(100.0);
    let t0 = Instant::now();
    let mut agent = q_agent();
    let train = agent.train(&mut env, EPISODES, 1_000).unwrap();
    println!(
        "treino: {EPISODES} episódios em {:.1?}; {} estados visitados",
        t0.elapsed(),
        agent.states_seen()
    );
    let head = Summary::from_samples(&train[..50]);
    let tail = Summary::from_samples(&train[EPISODES - 50..]);
    println!(
        "retorno de treino (com exploração): primeiros 50 = {:.0}, últimos 50 = {:.0}",
        head.mean, tail.mean
    );

    let seeds = 10_000..10_040u64; // never used in training; same for every policy
    let random = episode_returns(&mut env, seeds.clone(), random_policy(7)).unwrap();
    let heuristic =
        episode_returns(&mut env, seeds.clone(), shortest_expected_delay_policy).unwrap();
    let learned = episode_returns(&mut env, seeds, |o| agent.greedy(&o.values)).unwrap();
    describe("aleatória", &Summary::from_samples(&random));
    describe("heurística", &Summary::from_samples(&heuristic));
    describe("Q-learning", &Summary::from_samples(&learned));
    let vs_random = paired(&learned, &random);
    let vs_heur = paired(&learned, &heuristic);
    println!(
        "Q - aleatória: {:.1} ± {:.1}; Q - heurística: {:.1} ± {:.1}",
        vs_random.mean, vs_random.half_width, vs_heur.mean, vs_heur.half_width
    );

    // Significantly better than random: the paired 95% interval is above zero.
    assert!(vs_random.mean - vs_random.half_width > 0.0, "{vs_random:?}");
    // Close to the heuristic: at most 10% worse (returns are negative costs).
    let (q, h) = (
        Summary::from_samples(&learned).mean,
        Summary::from_samples(&heuristic).mean,
    );
    assert!(
        q >= 1.10 * h,
        "Q-learning {q:.0} longe da heurística {h:.0}"
    );
}

fn shortest_expected_delay_policy(o: &utsim_env::Observation) -> usize {
    shortest_expected_delay(&o.values)
}

/// Cost of reading metrics mid-run: steps per second with and without reward terms
/// (each reward term set makes every step call `Simulation::report()`).
#[test]
fn report_cost_per_step() {
    let time = |mut env: Env| {
        let t0 = Instant::now();
        let mut n = 0u32;
        for seed in 0..20 {
            let mut obs = env.reset(seed).unwrap();
            while obs.options > 0 {
                let r = env.step(shortest_expected_delay(&obs.values)).unwrap();
                n += 1;
                obs = r.observation;
                if r.done {
                    break;
                }
            }
        }
        (n, t0.elapsed() / n)
    };
    let (n, with) = time(make_env(100.0));
    let mut config = make_env(100.0).config().clone();
    config.reward.clear();
    let (_, without) = time(Env::new(Model::parse(MODEL).unwrap(), config).unwrap());
    println!("{n} passos: {with:.1?}/passo com métricas, {without:.1?}/passo sem");
}
