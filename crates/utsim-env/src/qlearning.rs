//! Tabular Q-learning baseline. / Linha de base: Q-learning tabular.
//!
//! # Português
//! Pequeno e sem dependências de aprendizado de máquina, serve de referência para
//! comparar políticas (aleatória, heurística, aprendida) e para testar o ambiente.
//! A observação é discretizada por faixas configuráveis ([`Dim`]): cada dimensão escolhe
//! um índice da observação e os limites entre faixas; o estado é o vetor de faixas.
//! A tabela Q é um `HashMap` (só guarda os estados visitados), iniciada em zero. Com
//! recompensas negativas isso é otimismo diante da incerteza, que já explora bastante.
//! A taxa de aprendizado de cada par (estado, ação) decai com o número de visitas até um
//! piso. A exploração é ε-gulosa com ε decaindo linearmente ao longo dos episódios.
//!
//! # English
//! A tiny tabular Q-learner. The observation is discretized by configurable bins
//! ([`Dim`]); the Q-table is a `HashMap` initialised at zero (optimistic when rewards
//! are negative); exploration is ε-greedy with ε decaying linearly over the episodes.
//! [`evaluate`] measures any policy over a range of seeds with a 95% confidence
//! interval, so policies can be compared with common random numbers.

use std::collections::HashMap;

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use utsim_core::stats::Summary;

use crate::{Env, EnvError, Observation};

/// One discretized dimension: observation entry `index`, split at `edges` (ascending).
/// The bin is the number of edges less than or equal to the value, so `edges = [1, 2]`
/// gives bins `<1`, `[1,2)` and `>=2`. / Uma dimensão discretizada.
#[derive(Clone, Debug, PartialEq)]
pub struct Dim {
    pub index: usize,
    pub edges: Vec<f64>,
}

impl Dim {
    /// Integer values `0, 1, ..., cap` get their own bin; larger ones share the last.
    pub fn counts(index: usize, cap: usize) -> Dim {
        Dim {
            index,
            edges: (1..=cap).map(|k| k as f64).collect(),
        }
    }

    /// `bins` equal-width bins between `lo` and `hi` (values outside go to the ends).
    pub fn uniform(index: usize, lo: f64, hi: f64, bins: usize) -> Dim {
        let bins = bins.max(1);
        Dim {
            index,
            edges: (1..bins)
                .map(|k| lo + (hi - lo) * k as f64 / bins as f64)
                .collect(),
        }
    }

    fn bin(&self, obs: &[f64]) -> u64 {
        let v = obs.get(self.index).copied().unwrap_or(0.0);
        self.edges.iter().filter(|&&e| v >= e).count() as u64
    }
}

/// Learning parameters. / Parâmetros de aprendizado.
#[derive(Clone, Debug, PartialEq)]
pub struct QConfig {
    /// Learning-rate floor. The rate of a (state, action) pair starts at 1 and decays
    /// as `1/visits^0.7` (stochastic-approximation schedule) until it reaches `alpha`.
    pub alpha: f64,
    /// Discount per decision.
    pub gamma: f64,
    /// Exploration probability at the first and at the last training episode.
    pub epsilon_start: f64,
    pub epsilon_end: f64,
    /// Seed of the agent's own random numbers (exploration, tie-breaking).
    pub seed: u64,
}

impl Default for QConfig {
    fn default() -> Self {
        QConfig {
            alpha: 0.1,
            gamma: 0.95,
            epsilon_start: 0.3,
            epsilon_end: 0.02,
            seed: 1,
        }
    }
}

/// Tabular Q-learning agent. / Agente de Q-learning tabular.
pub struct QAgent {
    dims: Vec<Dim>,
    options: usize,
    config: QConfig,
    table: HashMap<u64, Vec<f64>>,
    visits: HashMap<u64, Vec<u32>>,
    rng: Pcg64Mcg,
}

impl QAgent {
    pub fn new(dims: Vec<Dim>, options: usize, config: QConfig) -> QAgent {
        let rng = Pcg64Mcg::seed_from_u64(config.seed);
        QAgent {
            dims,
            options,
            config,
            table: HashMap::new(),
            visits: HashMap::new(),
            rng,
        }
    }

    /// Discretized state of an observation (mixed-radix code of the bins).
    pub fn state(&self, obs: &[f64]) -> u64 {
        self.dims
            .iter()
            .fold(0, |acc, d| acc * (d.edges.len() as u64 + 1) + d.bin(obs))
    }

    /// Number of distinct states visited so far.
    pub fn states_seen(&self) -> usize {
        self.table.len()
    }

    /// Best action according to the table (ties broken by the lowest index, so the
    /// untrained greedy policy is deterministic).
    pub fn greedy(&self, obs: &[f64]) -> usize {
        match self.table.get(&self.state(obs)) {
            Some(q) => argmax(q),
            None => 0,
        }
    }

    fn q_row(&mut self, state: u64) -> &mut Vec<f64> {
        let n = self.options;
        self.table.entry(state).or_insert_with(|| vec![0.0; n])
    }

    /// Trains for `episodes` episodes; episode `k` uses env seed `seed_base + k`.
    /// Returns the (exploratory) return of each episode. / Devolve o retorno de cada
    /// episódio de treino.
    pub fn train(
        &mut self,
        env: &mut Env,
        episodes: usize,
        seed_base: u64,
    ) -> Result<Vec<f64>, EnvError> {
        let mut returns = Vec::with_capacity(episodes);
        for ep in 0..episodes {
            let frac = if episodes > 1 {
                ep as f64 / (episodes - 1) as f64
            } else {
                1.0
            };
            let eps = self.config.epsilon_start
                + (self.config.epsilon_end - self.config.epsilon_start) * frac;
            let mut obs = env.reset(seed_base + ep as u64)?;
            let mut total = 0.0;
            while obs.options > 0 {
                let s = self.state(&obs.values);
                let a = if self.rng.random::<f64>() < eps {
                    self.rng.random_range(0..obs.options)
                } else {
                    self.greedy(&obs.values)
                };
                let r = env.step(a)?;
                total += r.reward;
                let target = if r.done {
                    r.reward
                } else {
                    let s2 = self.state(&r.observation.values);
                    let best = self
                        .table
                        .get(&s2)
                        .map(|q| q.iter().copied().fold(f64::NEG_INFINITY, f64::max))
                        .unwrap_or(0.0);
                    r.reward + self.config.gamma * best
                };
                let n = self
                    .visits
                    .entry(s)
                    .or_insert_with(|| vec![0; self.options]);
                n[a] += 1;
                let alpha = self.config.alpha.max((n[a] as f64).powf(-0.7));
                let q = &mut self.q_row(s)[a];
                *q += alpha * (target - *q);
                obs = r.observation;
                if r.done {
                    break;
                }
            }
            returns.push(total);
        }
        Ok(returns)
    }
}

fn argmax(q: &[f64]) -> usize {
    let mut best = 0;
    for (i, &v) in q.iter().enumerate() {
        if v > q[best] {
            best = i;
        }
    }
    best
}

/// Plays one episode per seed with `policy` and returns the total reward of each
/// episode, in seed order. / Joga um episódio por semente e devolve o retorno de cada um.
pub fn episode_returns<F>(
    env: &mut Env,
    seeds: std::ops::Range<u64>,
    mut policy: F,
) -> Result<Vec<f64>, EnvError>
where
    F: FnMut(&Observation) -> usize,
{
    let mut out = Vec::new();
    for seed in seeds {
        let mut obs = env.reset(seed)?;
        let mut total = 0.0;
        while obs.options > 0 {
            let r = env.step(policy(&obs))?;
            total += r.reward;
            obs = r.observation;
            if r.done {
                break;
            }
        }
        out.push(total);
    }
    Ok(out)
}

/// Mean episode return and 95% confidence half-width over `seeds`.
pub fn evaluate<F>(
    env: &mut Env,
    seeds: std::ops::Range<u64>,
    policy: F,
) -> Result<Summary, EnvError>
where
    F: FnMut(&Observation) -> usize,
{
    Ok(Summary::from_samples(&episode_returns(env, seeds, policy)?))
}
