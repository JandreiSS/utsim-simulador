//! Gym-like reinforcement-learning environment over a UTSim model (phase 7, RF-104).
//! / Ambiente de aprendizado por reforço no estilo Gym sobre um modelo do UTSim.
//!
//! # Português
//! Um modelo com decisões externas (no fluxo, `Decide` com regra `Agent`) pausa a cada
//! escolha e espera uma resposta. [`Env`] transforma isso no laço clássico de RL:
//!
//! ```text
//! let mut env = Env::new(modelo, config)?;
//! let mut obs = env.reset(semente)?;          // roda até a primeira decisão
//! loop {
//!     let acao = politica(&obs.values);       // 0..obs.options
//!     let passo = env.step(acao)?;            // responde e roda até a próxima decisão
//!     if passo.done { break; }
//!     obs = passo.observation;
//! }
//! ```
//!
//! - **Observação** ([`Observation`]): os números que o próprio modelo informa na decisão
//!   (`observation` do `Decide`), seguidos do conteúdo de blocos escolhidos
//!   (`EnvConfig::observe_blocks`, ex.: tamanho das filas) e de métricas ao vivo
//!   (`EnvConfig::observe_metrics`). As expressões do fluxo só enxergam rótulos e `now`;
//!   por isso o estado físico do modelo (filas, máquinas) vem do ambiente.
//! - **Recompensa:** Σ peso·Δmétrica entre duas decisões consecutivas, menos o custo fixo
//!   por decisão. Veja [`RewardTerm`]. Um termo "ponderado no tempo" transforma uma média
//!   no tempo (`fila.content.mean`) na sua integral, e então Δ é a área acumulada entre
//!   duas decisões. Com peso −1 sobre o conteúdo de todos os blocos, a recompensa total
//!   do episódio é −(soma dos tempos no sistema) (lei de Little).
//! - **Fim de episódio:** quando o relógio chega ao tempo de parada (`episode_length`
//!   substitui `run.stop_time`). O aquecimento é sempre zerado no ambiente.
//! - **Custo de leitura das métricas:** cada passo chama `Simulation::report()`, que
//!   percorre todos os blocos (alocando nomes). Só acontece se houver termos de
//!   recompensa ou `observe_metrics`; sem eles o passo custa apenas a simulação.
//!
//! # English
//! A model with external decisions (a flow `Decide` with an `Agent` rule) pauses at each
//! choice. [`Env`] wraps that into the classic RL loop: `reset(seed)` runs to the first
//! decision, `step(action)` answers it and runs to the next one or the end of the
//! episode. The reward is Σ weight·Δmetric between consecutive decisions (metrics are
//! read from the live report) minus an optional per-decision cost. Observations are the
//! decision's own vector plus the contents of chosen blocks and optional live metrics.
//!
//! The crate also ships a small tabular Q-learning baseline in [`qlearning`].

pub mod qlearning;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utsim_core::{BlockId, RunReport, Simulation, Step};
use utsim_model::{Model, ModelError};

/// Environment errors. / Erros do ambiente.
#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error(transparent)]
    Model(#[from] ModelError),
    #[error(transparent)]
    Sim(#[from] utsim_core::SimError),
    #[error("configuração do ambiente inválida: {0}")]
    Config(String),
    #[error("chame reset() antes de step()")]
    NotStarted,
    #[error("o episódio terminou: chame reset()")]
    Done,
}

/// One term of the reward: `weight · Δ value`, where `value` is a metric of the live
/// report (`bloco.metrica`, the keys of `RunReport::metrics`).
/// / Um termo da recompensa: `peso · Δ valor`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardTerm {
    /// Metric key, e.g. `"saida.input"`, `"fila.content.mean"`.
    pub metric: String,
    /// Multiplier of the metric's increase since the previous decision.
    pub weight: f64,
    /// Multiply the metric by the elapsed time first. Use it on time averages
    /// (`*.content.mean`, `*.state.*`) so the term is their integral (area).
    #[serde(default)]
    pub time_weighted: bool,
}

impl RewardTerm {
    pub fn new(metric: impl Into<String>, weight: f64) -> Self {
        RewardTerm {
            metric: metric.into(),
            weight,
            time_weighted: false,
        }
    }

    /// Same as [`RewardTerm::new`], integrating a time average over the elapsed time.
    pub fn integral(metric: impl Into<String>, weight: f64) -> Self {
        RewardTerm {
            time_weighted: true,
            ..RewardTerm::new(metric, weight)
        }
    }
}

/// Environment settings. / Configuração do ambiente.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvConfig {
    /// Reward = Σ weight·Δmetric − `decision_cost`.
    #[serde(default)]
    pub reward: Vec<RewardTerm>,
    /// Constant cost subtracted from the reward at every decision.
    #[serde(default)]
    pub decision_cost: f64,
    /// Episode length (replaces `run.stop_time`); `None` keeps the model's.
    #[serde(default)]
    pub episode_length: Option<f64>,
    /// Blocks whose current content (items inside) is appended to the observation, in
    /// order. `"a+b"` appends the sum of the contents of `a` and `b` (a line made of a
    /// queue and a machine, say).
    #[serde(default)]
    pub observe_blocks: Vec<String>,
    /// Live metrics (`bloco.metrica`) appended after the block contents.
    #[serde(default)]
    pub observe_metrics: Vec<String>,
}

impl Default for EnvConfig {
    fn default() -> Self {
        EnvConfig {
            reward: Vec::new(),
            decision_cost: 0.0,
            episode_length: None,
            observe_blocks: Vec::new(),
            observe_metrics: Vec::new(),
        }
    }
}

/// What the agent sees at a decision. / O que o agente vê numa decisão.
#[derive(Clone, Debug, PartialEq)]
pub struct Observation {
    /// Model's observation, then block contents, then live metrics.
    pub values: Vec<f64>,
    /// Number of actions available (`0..options`); 0 once the episode is over.
    pub options: usize,
    /// Simulation time of the decision.
    pub time: f64,
}

/// Extra information returned by [`Env::step`]. / Informação extra de [`Env::step`].
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    /// Simulation time now.
    pub time: f64,
    /// Decisions answered in this episode so far.
    pub decisions: u64,
    /// Name of the block that asked (empty when the episode is over).
    pub block: String,
    /// Final report metrics, only when `done`.
    pub metrics: Option<BTreeMap<String, f64>>,
}

/// Result of [`Env::step`]: the Gym tuple `(observation, reward, done, info)`.
#[derive(Clone, Debug, PartialEq)]
pub struct StepResult {
    pub observation: Observation,
    pub reward: f64,
    pub done: bool,
    pub info: Info,
}

/// Where each observed entry comes from. / De onde vem cada entrada observada.
struct Plan {
    /// Blocks summed into each `observe_blocks` entry.
    blocks: Vec<Vec<BlockId>>,
}

/// The environment. / O ambiente.
pub struct Env {
    model: Model,
    config: EnvConfig,
    plan: Plan,
    sim: Option<Simulation>,
    /// Metric values (per reward term) at the previous decision.
    last: Vec<f64>,
    decisions: u64,
    done: bool,
    /// Length of the model part of the last observation (for the final zeros).
    model_len: usize,
}

impl Env {
    /// Builds the environment: validates the model and the configuration.
    /// / Valida o modelo e a configuração.
    pub fn new(mut model: Model, config: EnvConfig) -> Result<Env, EnvError> {
        model.run.warmup = 0.0;
        if let Some(l) = config.episode_length {
            if !(l.is_finite() && l > 0.0) {
                return Err(EnvError::Config("episode_length deve ser > 0".into()));
            }
            model.run.stop_time = l;
        }
        model.validate()?;
        // Block names resolved once, against a probe build.
        let probe = model.build(0);
        let mut blocks = Vec::new();
        for entry in &config.observe_blocks {
            let mut ids = Vec::new();
            for name in entry.split('+') {
                let name = name.trim();
                let id = probe.block_id(name).ok_or_else(|| {
                    EnvError::Config(format!("observe_blocks: bloco '{name}' não existe"))
                })?;
                ids.push(id);
            }
            blocks.push(ids);
        }
        for key in config
            .reward
            .iter()
            .map(|t| &t.metric)
            .chain(&config.observe_metrics)
        {
            // Only the block part can be checked up front: some metrics (states,
            // recorded tallies) appear in the report once they happen.
            let block = key.split('.').next().unwrap_or("");
            if probe.block_id(block).is_none() || !key.contains('.') {
                return Err(EnvError::Config(format!(
                    "métrica '{key}': use 'bloco.métrica' com um bloco que existe"
                )));
            }
        }
        Ok(Env {
            model,
            config,
            plan: Plan { blocks },
            sim: None,
            last: Vec::new(),
            decisions: 0,
            done: true,
            model_len: 0,
        })
    }

    /// Starts a new episode with the given random seed and runs to the first decision.
    /// / Inicia um episódio e roda até a primeira decisão. If the model never asks for
    /// a decision, the returned observation has `options == 0`.
    pub fn reset(&mut self, seed: u64) -> Result<Observation, EnvError> {
        let mut model = self.model.clone();
        model.run.seed = seed;
        let mut sim = model.build(0);
        self.decisions = 0;
        self.done = false;
        let step = sim.run_until(f64::INFINITY)?;
        self.sim = Some(sim);
        let obs = match step {
            Step::Decision(d) => self.observe(&d.observation, d.options, d.time),
            _ => {
                self.done = true;
                self.observe(&[], 0, self.sim_ref().time())
            }
        };
        let report = self.needs_report().then(|| self.sim_ref().report());
        self.last = self.reward_values(report.as_ref());
        Ok(obs)
    }

    /// Answers the pending decision with `action` (`0..options`) and runs to the next
    /// decision or the end of the episode. / Responde a decisão e roda até a próxima.
    pub fn step(&mut self, action: usize) -> Result<StepResult, EnvError> {
        if self.sim.is_none() {
            return Err(EnvError::NotStarted);
        }
        if self.done {
            return Err(EnvError::Done);
        }
        let sim = self.sim.as_mut().expect("checked");
        sim.decide(action)?;
        self.decisions += 1;
        let step = sim.run_until(f64::INFINITY)?;
        let (observation, block, report) = match step {
            Step::Decision(d) => {
                let block = self.sim_ref().block_status()[d.block.0].0.clone();
                let report = self.needs_report().then(|| self.sim_ref().report());
                (
                    self.observe(&d.observation, d.options, d.time),
                    block,
                    report,
                )
            }
            _ => {
                self.done = true;
                let sim = self.sim.as_mut().expect("checked");
                let report = sim.finish();
                let time = sim.time();
                (self.observe(&[], 0, time), String::new(), Some(report))
            }
        };
        let now = self.reward_values(report.as_ref());
        let reward = now
            .iter()
            .zip(&self.last)
            .zip(&self.config.reward)
            .map(|((n, l), t)| t.weight * (n - l))
            .sum::<f64>()
            - self.config.decision_cost;
        self.last = now;
        let info = Info {
            time: self.sim_ref().time(),
            decisions: self.decisions,
            block,
            metrics: self.done.then(|| report.map(|r| r.metrics())).flatten(),
        };
        Ok(StepResult {
            observation,
            reward,
            done: self.done,
            info,
        })
    }

    /// Whether the episode is over. / Se o episódio terminou.
    pub fn done(&self) -> bool {
        self.done
    }

    /// Live view of the running simulation (e.g. for custom observations).
    pub fn simulation(&self) -> Option<&Simulation> {
        self.sim.as_ref()
    }

    /// The configuration in use. / A configuração em uso.
    pub fn config(&self) -> &EnvConfig {
        &self.config
    }

    fn sim_ref(&self) -> &Simulation {
        self.sim.as_ref().expect("reset() called")
    }

    fn needs_report(&self) -> bool {
        !self.config.reward.is_empty() || !self.config.observe_metrics.is_empty()
    }

    /// Current value of every reward term (metric, integrated when time-weighted).
    fn reward_values(&self, report: Option<&RunReport>) -> Vec<f64> {
        let Some(report) = report else {
            return Vec::new();
        };
        let metrics = report.metrics();
        let elapsed = report.end_time - report.stats_from;
        self.config
            .reward
            .iter()
            .map(|t| {
                let v = metrics.get(&t.metric).copied().unwrap_or(0.0);
                if t.time_weighted { v * elapsed } else { v }
            })
            .collect()
    }

    fn observe(&mut self, model_obs: &[f64], options: usize, time: f64) -> Observation {
        if options > 0 {
            self.model_len = model_obs.len();
        }
        let mut values = if options > 0 {
            model_obs.to_vec()
        } else {
            vec![0.0; self.model_len]
        };
        let sim = self.sim_ref();
        let status = sim.block_status();
        for ids in &self.plan.blocks {
            values.push(ids.iter().map(|b| status[b.0].2 as f64).sum());
        }
        if !self.config.observe_metrics.is_empty() {
            let metrics = sim.report().metrics();
            for key in &self.config.observe_metrics {
                values.push(metrics.get(key).copied().unwrap_or(0.0));
            }
        }
        Observation {
            values,
            options,
            time,
        }
    }
}
