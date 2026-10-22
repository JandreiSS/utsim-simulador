//! Python bindings for the UTSim RL environment. / Ligações Python do ambiente de RL.
//!
//! ```python
//! import utsim
//! env = utsim.Env("examples/roteamento_rl.ron",
//!                 reward=[("fila_rapida.content.mean", -1.0, True)],
//!                 observe_blocks=["fila_rapida+maquina_rapida"], episode_length=100.0)
//! obs, info = env.reset(seed=1)            # obs: list[float]; info["options"] actions
//! obs, reward, done, info = env.step(0)
//! ```
//!
//! Everything crosses the boundary as plain Python lists/floats/dicts (no numpy).
//! `reward` is a list of `(metric, weight, time_weighted)` tuples.

use std::collections::HashMap;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use utsim_env::{Env, EnvConfig, Info, Observation, RewardTerm};
use utsim_model::Model;

fn err(e: impl std::fmt::Display) -> PyErr {
    PyRuntimeError::new_err(e.to_string())
}

fn info_dict<'py>(
    py: Python<'py>,
    obs: &Observation,
    info: Option<&Info>,
) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("options", obs.options)?;
    d.set_item("time", obs.time)?;
    if let Some(i) = info {
        d.set_item("decisions", i.decisions)?;
        d.set_item("block", &i.block)?;
        if let Some(m) = &i.metrics {
            let m: HashMap<&String, f64> = m.iter().map(|(k, v)| (k, *v)).collect();
            d.set_item("metrics", m)?;
        }
    }
    Ok(d)
}

/// RL environment over a UTSim model file. / Ambiente de RL sobre um arquivo de modelo.
#[pyclass(name = "Env", unsendable)]
struct PyEnv {
    inner: Env,
}

#[pymethods]
impl PyEnv {
    #[new]
    #[pyo3(signature = (model, reward=Vec::new(), decision_cost=0.0, episode_length=None,
                        observe_blocks=Vec::new(), observe_metrics=Vec::new()))]
    fn new(
        model: &str,
        reward: Vec<(String, f64, bool)>,
        decision_cost: f64,
        episode_length: Option<f64>,
        observe_blocks: Vec<String>,
        observe_metrics: Vec<String>,
    ) -> PyResult<Self> {
        let model = Model::load(model).map_err(err)?;
        let config = EnvConfig {
            reward: reward
                .into_iter()
                .map(|(metric, weight, time_weighted)| RewardTerm {
                    metric,
                    weight,
                    time_weighted,
                })
                .collect(),
            decision_cost,
            episode_length,
            observe_blocks,
            observe_metrics,
        };
        Ok(PyEnv {
            inner: Env::new(model, config).map_err(err)?,
        })
    }

    /// Starts an episode; returns `(observation, info)`.
    #[pyo3(signature = (seed=0))]
    fn reset<'py>(
        &mut self,
        py: Python<'py>,
        seed: u64,
    ) -> PyResult<(Vec<f64>, Bound<'py, PyDict>)> {
        let obs = self.inner.reset(seed).map_err(err)?;
        let info = info_dict(py, &obs, None)?;
        Ok((obs.values, info))
    }

    /// Answers the decision; returns `(observation, reward, done, info)`.
    fn step<'py>(
        &mut self,
        py: Python<'py>,
        action: usize,
    ) -> PyResult<(Vec<f64>, f64, bool, Bound<'py, PyDict>)> {
        let r = self.inner.step(action).map_err(err)?;
        let info = info_dict(py, &r.observation, Some(&r.info))?;
        Ok((r.observation.values, r.reward, r.done, info))
    }
}

#[pymodule]
fn utsim(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEnv>()?;
    Ok(())
}
