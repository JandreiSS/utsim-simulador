//! The twin in shadow mode: observations in, observed state, window KPIs, comparison with
//! the model and alerts out. No network and no clock here: the live loop
//! ([`crate::live`]) and the replay ([`replay`]) feed the same [`Twin`], so a replay of a
//! recording gives the same report as the live session.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utsim_model::{InitialState, Model};

use crate::config::{Counter, Measure, TwinConfig};
use crate::drift::{Alert, DriftMonitor, KpiStatus, Limits};
use crate::map::{Mapper, RuleStats, Skip};
use crate::obs::{Observation, Quality, Value};
use crate::record::{Anonymizer, Line, Recorder, Recording};
use crate::shadow::{self, Accumulator, ObjectState, WindowKpis};
use crate::simdist::{SimDistribution, mapped_measures};

/// One mapping rule as the panel shows it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleView {
    pub fonte: String,
    pub tag: String,
    pub objeto: String,
    pub medida: Measure,
    #[serde(flatten)]
    pub stats: RuleStats,
}

/// Everything the twin knows, as JSON for the panel and for the replay comparison.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TwinReport {
    pub modelo: String,
    pub modo: String,
    /// Twin time reached.
    pub tempo: f64,
    pub janela: f64,
    pub janelas: Vec<WindowKpis>,
    pub kpis: BTreeMap<String, KpiStatus>,
    pub alertas: Vec<Alert>,
    pub limites: Limits,
    /// KPI tests done (KPIs × windows).
    pub testes: usize,
    pub objetos: BTreeMap<String, ObjectState>,
    pub regras: Vec<RuleView>,
    pub recebidas: u64,
    pub nao_mapeadas: u64,
    /// Unmapped tags seen (at most 200), with their counts.
    pub tags_nao_mapeados: BTreeMap<String, u64>,
    pub replicacoes_simuladas: u32,
    pub janelas_por_replicacao: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado_inicial: Option<InitialState>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notas_estado: Vec<String>,
}

pub struct Twin {
    model: Model,
    cfg: TwinConfig,
    mapper: Mapper,
    acc: Accumulator,
    drift: DriftMonitor,
    tested: usize,
    anonymizer: Option<Anonymizer>,
    recorder: Option<Recorder>,
    record_errors: u64,
    received: u64,
    unmapped: u64,
    unmapped_tags: BTreeMap<String, u64>,
}

impl Twin {
    pub fn new(model: Model, cfg: TwinConfig, sim: SimDistribution) -> Twin {
        let measures = mapped_measures(&cfg);
        let mut acc = Accumulator::new(cfg.shadow.window, cfg.shadow.warmup, &measures, None);
        for r in &cfg.tags {
            if r.counter == Counter::Pulse
                && matches!(r.measure, Measure::Inputs | Measure::Outputs)
            {
                acc.baseline(&r.object, r.measure, cfg.shadow.warmup);
            }
        }
        let anonymizer = cfg
            .record
            .anonymize_people
            .then(|| Anonymizer::new(&cfg.record.people));
        Twin {
            mapper: Mapper::new(&cfg.tags),
            drift: DriftMonitor::new(&cfg.shadow, sim),
            acc,
            model,
            cfg,
            tested: 0,
            anonymizer,
            recorder: None,
            record_errors: 0,
            received: 0,
            unmapped: 0,
            unmapped_tags: BTreeMap::new(),
        }
    }

    /// Records every observation from now on.
    pub fn with_recorder(mut self, recorder: Recorder) -> Twin {
        self.recorder = Some(recorder);
        self
    }

    pub fn recorder(&self) -> Option<&Recorder> {
        self.recorder.as_ref()
    }

    pub fn config(&self) -> &TwinConfig {
        &self.cfg
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn now(&self) -> f64 {
        self.acc.now().max(0.0)
    }

    /// Builds the observation of a reading that arrived at twin time `t`: finds its rule
    /// and anonymises it when it is a person.
    #[allow(clippy::too_many_arguments)]
    pub fn stamp(
        &self,
        source: &str,
        tag: &str,
        value: Value,
        quality: Quality,
        seq: u64,
        t: f64,
        wall_ms: u64,
    ) -> Observation {
        let rule = self.mapper.rule_for(source, tag);
        let mut obs = Observation {
            t,
            wall_ms,
            source: source.to_string(),
            seq,
            tag: tag.to_string(),
            value,
            quality,
            rule,
        };
        if let Some(a) = &self.anonymizer {
            let person = rule.is_some_and(|r| self.cfg.tags[r].person) || a.matches(tag);
            if person {
                a.scrub(&mut obs);
            }
        }
        obs
    }

    /// Processes an observation (already stamped; in time order).
    pub fn ingest(&mut self, obs: Observation) {
        if let Some(rec) = &mut self.recorder {
            if rec.write(&Line::Obs(obs.clone()), obs.wall_ms).is_err() {
                self.record_errors += 1;
            }
        }
        self.received += 1;
        match obs.rule {
            Some(r) if r < self.cfg.tags.len() => match self.mapper.apply(r, &obs) {
                Ok(e) => self.acc.apply(&e),
                Err(Skip::Baseline) => {
                    let rule = &self.cfg.tags[r];
                    self.acc.advance(obs.t);
                    self.acc.baseline(&rule.object, rule.measure, obs.t);
                }
                Err(_) => self.acc.advance(obs.t),
            },
            _ => {
                self.unmapped += 1;
                let key = format!("{}: {}", obs.source, obs.tag);
                if self.unmapped_tags.len() < 200 || self.unmapped_tags.contains_key(&key) {
                    *self.unmapped_tags.entry(key).or_default() += 1;
                }
                self.acc.advance(obs.t);
            }
        }
        self.test_new_windows();
    }

    /// The twin's clock reached `t` (no observation): closes the windows that ended.
    pub fn advance(&mut self, t: f64) {
        self.acc.advance(t);
        self.test_new_windows();
    }

    /// Ends the session at `t` (written to the recording, so the replay ends there too).
    pub fn finish(&mut self, t: f64) {
        let t = t.max(self.acc.now());
        self.advance(t);
        if let Some(rec) = &mut self.recorder {
            let _ = rec.write(&Line::End { t }, crate::obs::now_ms());
            let _ = rec.flush();
        }
    }

    pub fn flush(&mut self) {
        if let Some(rec) = &mut self.recorder {
            let _ = rec.flush();
        }
    }

    fn test_new_windows(&mut self) {
        let new: Vec<WindowKpis> = self.acc.closed_since(self.tested).to_vec();
        for w in &new {
            self.drift.on_window(w);
        }
        self.tested += new.len();
    }

    pub fn alerts(&self) -> &[Alert] {
        self.drift.alerts()
    }

    pub fn windows(&self) -> &[WindowKpis] {
        self.acc.closed()
    }

    /// The observed state as an initial state file (for the forecast), with notes.
    pub fn initial_state(&self) -> (InitialState, Vec<String>) {
        shadow::initial_state(&self.model, &self.acc.objects(), self.now())
    }

    pub fn record_errors(&self) -> u64 {
        self.record_errors
    }

    /// The report. With `with_state`, the observed state as an initial state too (it
    /// validates against the model: slower).
    pub fn report(&self, with_state: bool) -> TwinReport {
        let (estado_inicial, notas_estado) = if with_state {
            let (s, n) = self.initial_state();
            (Some(s), n)
        } else {
            (None, Vec::new())
        };
        let sim = self.drift.sim();
        TwinReport {
            modelo: self.model.name.clone(),
            modo: "sombra".into(),
            tempo: self.now(),
            janela: self.cfg.shadow.window,
            janelas: self.acc.closed().to_vec(),
            kpis: self.drift.kpis().clone(),
            alertas: self.drift.alerts().to_vec(),
            limites: self.drift.limits().clone(),
            testes: self.drift.tests(),
            objetos: self.acc.objects(),
            regras: self
                .cfg
                .tags
                .iter()
                .zip(&self.mapper.stats)
                .map(|(r, s)| RuleView {
                    fonte: r.source.clone(),
                    tag: if r.person && self.anonymizer.is_some() {
                        "(pessoa: anonimizado)".into()
                    } else {
                        r.tag.clone()
                    },
                    objeto: r.object.clone(),
                    medida: r.measure,
                    stats: s.clone(),
                })
                .collect(),
            recebidas: self.received,
            nao_mapeadas: self.unmapped,
            tags_nao_mapeados: self.unmapped_tags.clone(),
            replicacoes_simuladas: sim.replications,
            janelas_por_replicacao: sim.windows_per_replication,
            estado_inicial,
            notas_estado,
        }
    }
}

/// Replays a recording without network and without clock: the same observations, in the
/// same order, at the same twin times.
pub fn replay(twin: &mut Twin, recording: &Recording) {
    let mut end = None;
    for line in &recording.lines {
        match line {
            Line::Obs(o) => twin.ingest(o.clone()),
            Line::End { t } => end = Some(*t),
        }
    }
    if let Some(t) = end {
        twin.advance(t);
    }
}
