//! The live results of one object during the playback of Simular (pure: no Godot types,
//! unit tested). The panel of `ui_ao_vivo.rs` shows them in a corner of the view.
//!
//! Built once, when the first panel opens ([`crate::playback::Playback::vivo`], [`AoVivo::new`]): one pass over the trace of
//! the replay gives, per block, its content as steps (with the area under them), the
//! instants of its entries and exits (with the cumulative stay), its states (with the
//! cumulative time in each, only for the states it visits), its travels, and where the
//! items it sent arrived (a scrap sink or a rework station, also through queues and
//! carriers). [`AoVivo::leitura`] answers at any playback time `t` with binary
//! searches only, so the panel follows the clock (also paused, or dragged backwards)
//! without scanning the trace again.
//!
//! The statistics window is the report's: from the end of the warm-up (`stats_from`) to
//! `t`, or from 0 while `t` is still in the warm-up. At the end of the replication the
//! numbers are the ones of its `RunReport` (the replay is replication 0 of the run; the
//! "Resultados" window shows the mean over all replications): [`AoVivo::confere`] checks
//! it, and the tests run it on example models.
//!
//! What the trace does not have stays out or comes from the report: the fatigue of a
//! person (only its value at the end of the replication) and the busy servers of a
//! processor with capacity above 1 (the panel shows the time processing).

use crate::texto::{fx, tx, vx};
use std::collections::{BTreeMap, HashMap};

use utsim_core::{RunReport, State, TraceEvent};
use utsim_model::replay::Replay;
use utsim_model::{Model, SinkRole};
use utsim_scene::MobileRole;

use crate::results::STATES;
use crate::ui_results::num;

/// Points of a curve of the panel (from 0 to `t`).
pub const PONTOS: usize = 200;
/// Seconds of real time between two recomputations while the clock plays.
pub const INTERVALO: f64 = 0.2;

/// What kind of object the panel shows (its sections).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classe {
    Processo,
    Fila,
    Saida,
    Chegada,
    Veiculo,
    Pessoa,
    Outro,
}

impl Classe {
    fn de(kind: &str, role: Option<MobileRole>) -> Classe {
        match kind {
            "Processor" | "Combiner" | "Separator" => Classe::Processo,
            "Queue" | "Rack" | "Conveyor" => Classe::Fila,
            "Sink" => Classe::Saida,
            "Source" => Classe::Chegada,
            "Transporter" if role == Some(MobileRole::Person) => Classe::Pessoa,
            "Transporter" | "Crane" => Classe::Veiculo,
            _ => Classe::Outro,
        }
    }
}

/// The content of a block over time, as steps.
#[derive(Clone, Debug, Default)]
struct Degraus {
    t: Vec<f64>,
    v: Vec<u32>,
    /// The largest value reached at that instant (several changes at one instant).
    pico: Vec<u32>,
    /// Area under the steps from 0 to `t[i]`.
    area: Vec<f64>,
    /// Running maximum of `pico` from 0, and from the start of the window (`w0` on).
    max0: Vec<u32>,
    maxw: Vec<u32>,
    w0: usize,
    base_w: u32,
}

impl Degraus {
    fn push(&mut self, t: f64, v: u32) {
        if self.t.last() == Some(&t) {
            let k = self.t.len() - 1;
            self.v[k] = v;
            self.pico[k] = self.pico[k].max(v);
            return;
        }
        let a = match (self.t.last(), self.v.last(), self.area.last()) {
            (Some(&lt), Some(&lv), Some(&la)) => la + lv as f64 * (t - lt),
            _ => 0.0,
        };
        self.t.push(t);
        self.v.push(v);
        self.pico.push(v);
        self.area.push(a);
    }

    /// The running maxima, for a window starting at `s`.
    fn fechar(&mut self, s: f64) {
        let mut m = 0;
        self.max0 = self
            .pico
            .iter()
            .map(|&p| {
                m = m.max(p);
                m
            })
            .collect();
        self.w0 = self.t.partition_point(|&x| x < s);
        self.base_w = self.w0.checked_sub(1).map_or(0, |i| self.v[i]);
        let mut m = self.base_w;
        self.maxw = (0..self.t.len())
            .map(|i| {
                if i >= self.w0 {
                    m = m.max(self.pico[i]);
                }
                m
            })
            .collect();
    }

    fn idx(&self, x: f64) -> Option<usize> {
        self.t.partition_point(|&a| a <= x).checked_sub(1)
    }

    fn valor(&self, x: f64) -> u32 {
        self.idx(x).map_or(0, |i| self.v[i])
    }

    fn area_ate(&self, x: f64) -> f64 {
        match self.idx(x) {
            Some(i) => self.area[i] + self.v[i] as f64 * (x - self.t[i]),
            None => 0.0,
        }
    }

    /// Time-weighted mean over `[s, x]` (the value at `x` when the window is empty).
    fn media(&self, s: f64, x: f64) -> f64 {
        if x - s <= 0.0 {
            self.valor(x) as f64
        } else {
            (self.area_ate(x) - self.area_ate(s)) / (x - s)
        }
    }

    /// Largest value over the window: from 0 (`desde_zero`) or from the report's start.
    fn maximo(&self, x: f64, desde_zero: bool) -> u32 {
        match self.idx(x) {
            None => 0,
            Some(i) if desde_zero => self.max0[i],
            Some(i) if i < self.w0 => self.base_w,
            Some(i) => self.maxw[i],
        }
    }
}

/// Instants of one kind of event, with a cumulative value (a stay, a time in system).
#[derive(Clone, Debug, Default)]
struct Contagem {
    t: Vec<f64>,
    soma: Vec<f64>,
}

impl Contagem {
    fn push(&mut self, t: f64, x: f64) {
        let s = self.soma.last().copied().unwrap_or(0.0) + x;
        self.t.push(t);
        self.soma.push(s);
    }

    fn faixa(&self, s: f64, x: f64) -> (usize, usize) {
        let a = self.t.partition_point(|&u| u < s);
        let b = self.t.partition_point(|&u| u <= x);
        (a, b.max(a))
    }

    /// Events in `[s, x]`.
    fn n(&self, s: f64, x: f64) -> u64 {
        let (a, b) = self.faixa(s, x);
        (b - a) as u64
    }

    /// Mean value of the events in `[s, x]`; `None` without events.
    fn media(&self, s: f64, x: f64) -> Option<f64> {
        let (a, b) = self.faixa(s, x);
        if b == a {
            return None;
        }
        let antes = a.checked_sub(1).map_or(0.0, |k| self.soma[k]);
        Some((self.soma[b - 1] - antes) / (b - a) as f64)
    }
}

/// The states of a block, with the cumulative time in each.
#[derive(Clone, Debug, Default)]
struct Estados {
    t: Vec<f64>,
    /// Index into `visitados`.
    s: Vec<usize>,
    visitados: Vec<State>,
    /// `cum[i * k + j]`: time in state `j` from 0 up to `t[i]` (`k` states visited).
    cum: Vec<f64>,
}

impl Estados {
    fn push(&mut self, t: f64, st: State) {
        let j = match self.visitados.iter().position(|&v| v == st) {
            Some(j) => j,
            None => {
                self.visitados.push(st);
                self.visitados.len() - 1
            }
        };
        self.t.push(t);
        self.s.push(j);
    }

    fn fechar(&mut self) {
        let k = self.visitados.len();
        let mut acc = vec![0.0; k];
        self.cum = Vec::with_capacity(self.t.len() * k);
        for i in 0..self.t.len() {
            if i > 0 {
                acc[self.s[i - 1]] += self.t[i] - self.t[i - 1];
            }
            self.cum.extend_from_slice(&acc);
        }
    }

    fn acumulado(&self, x: f64, j: usize) -> f64 {
        let k = self.visitados.len();
        match self.t.partition_point(|&a| a <= x).checked_sub(1) {
            Some(i) => self.cum[i * k + j] + if self.s[i] == j { x - self.t[i] } else { 0.0 },
            None => 0.0,
        }
    }

    fn atual(&self, x: f64) -> Option<State> {
        let i = self.t.partition_point(|&a| a <= x).checked_sub(1)?;
        Some(self.visitados[self.s[i]])
    }

    /// Fraction of `[s, x]` in each state visited (as `StateTracker::fractions`).
    fn fracoes(&self, s: f64, x: f64) -> BTreeMap<State, f64> {
        let span = x - s;
        self.visitados
            .iter()
            .enumerate()
            .filter_map(|(j, &st)| {
                let tempo = self.acumulado(x, j) - self.acumulado(s, j);
                let f = if span > 0.0 { tempo / span } else { tempo };
                (f > 0.0 || self.atual(x) == Some(st)).then_some((st, f.max(0.0)))
            })
            .collect()
    }
}

/// Everything one block keeps.
#[derive(Clone, Debug, Default)]
struct Serie {
    conteudo: Degraus,
    entradas: Contagem,
    /// Exits, with the stay of each.
    saidas: Contagem,
    estados: Estados,
    /// Items sent to a scrap sink / to a rework station.
    refugo: Contagem,
    retrabalho: Contagem,
    viagens: Contagem,
    /// Entries into the resting state.
    descansos: Contagem,
    /// Arrivals at a sink, with the time in system of each.
    sistema: Contagem,
}

/// One block as the panel knows it.
#[derive(Clone, Debug)]
struct Bloco {
    nome: String,
    kind: String,
    classe: Classe,
    papel: Option<MobileRole>,
    /// The buffers (queues, racks, conveyors) feeding it.
    antes: Vec<usize>,
    /// The scrap or customer role of a sink.
    papel_saida: Option<SinkRole>,
}

/// The live numbers of every block of one replication.
#[derive(Clone, Debug)]
pub struct AoVivo {
    pub replicacao: u32,
    pub inicio_estatisticas: f64,
    pub fim: f64,
    blocos: Vec<Bloco>,
    series: Vec<Serie>,
    /// The model has a sink with `sink_role: Scrap` (the scrap line counts); it has a
    /// rework station or an inspection (the rework line shows, and the scrap line asks
    /// to mark the scrap sink when there is none).
    pub tem_refugo: bool,
    pub tem_retrabalho: bool,
    report: RunReport,
}

/// The raw numbers of one object at one instant (the tests compare them with the report).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Numeros {
    /// Start of the window the means and counts use.
    pub inicio: f64,
    pub t: f64,
    pub entradas: u64,
    pub saidas: u64,
    pub conteudo: u32,
    pub conteudo_medio: f64,
    pub conteudo_max: u32,
    /// Mean stay of the items that left in the window.
    pub permanencia: Option<f64>,
    /// Items in the buffers feeding the object (`None`: it has none).
    pub fila: Option<u32>,
    /// Mean wait in those buffers of the items that left them in the window.
    pub espera: Option<f64>,
    pub refugo: u64,
    pub retrabalho: u64,
    pub viagens: u64,
    pub descansos: u64,
    /// Mean time in system of the items a sink received in the window.
    pub tempo_sistema: Option<f64>,
    pub estados: BTreeMap<State, f64>,
}

impl Numeros {
    pub fn fracao(&self, s: State) -> f64 {
        self.estados.get(&s).copied().unwrap_or(0.0)
    }

    fn soma(&self, list: &[State]) -> f64 {
        list.iter().map(|&s| self.fracao(s)).sum()
    }

    /// Mobile resource busy (the report's `BUSY_TRAVEL_STATES`).
    pub fn utilizacao(&self) -> f64 {
        self.soma(&[
            State::TravelEmpty,
            State::TravelLoaded,
            State::Loading,
            State::Unloading,
            State::Utilize,
        ])
    }
}

/// One line of a section: widget name (after `PainelVivo_`), label, value, help.
#[derive(Clone, Debug, PartialEq)]
pub struct LinhaVivo {
    pub chave: &'static str,
    pub rotulo: String,
    pub valor: String,
    pub ajuda: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Secao {
    pub titulo: &'static str,
    pub linhas: Vec<LinhaVivo>,
}

/// What the panel draws for one object at one instant.
#[derive(Clone, Debug, PartialEq)]
pub struct Leitura {
    pub nome: String,
    /// The kind in Portuguese (`processo`, `fila`, `pessoa`...).
    pub tipo: String,
    pub classe: Classe,
    /// The line under the title (time, window, replication).
    pub nota: String,
    pub secoes: Vec<Secao>,
    /// Curves from 0 to `t`: name, `[t, value]` points, colour index.
    pub curvas: Vec<(String, Vec<[f64; 2]>, usize)>,
    /// State fractions of the window, in the order of the states, summing to about 1.
    pub estados: Vec<(State, f64)>,
    pub numeros: Numeros,
}

fn pct(x: f64) -> String {
    format!("{} %", num(x * 100.0))
}

fn opt(x: Option<f64>) -> String {
    x.map_or("-".into(), num)
}

fn linha(chave: &'static str, rotulo: &str, valor: String, ajuda: &str) -> LinhaVivo {
    LinhaVivo {
        chave,
        rotulo: rotulo.into(),
        valor,
        ajuda: ajuda.into(),
    }
}

/// How many records of the trace the engine ran before the end of the warm-up. The
/// engine ends the warm-up at `s0` before the events of that instant scheduled after it,
/// but after the ones the blocks scheduled while starting (a first arrival exactly at
/// `s0`, `Simulation::start`). The trace has no mark of it, so the boundary among the
/// records at `s0` is the first one that leaves every block with the inputs its report
/// counts; the first record at `s0` when none does. 0 without a warm-up.
fn antes_do_aquecimento(replay: &Replay, s0: f64) -> usize {
    let recs = &replay.records;
    let a = recs.partition_point(|r| r.t < s0);
    let b = recs.partition_point(|r| r.t <= s0);
    if s0 <= 0.0 || a == b {
        return a;
    }
    let n = replay.blocks.len();
    let want: Vec<Option<u64>> = replay
        .blocks
        .iter()
        .map(|rb| replay.report.block(&rb.name).map(|x| x.input))
        .collect();
    let mut have = vec![0u64; n];
    for r in &recs[a..] {
        if let TraceEvent::Moved { to, .. } = r.event
            && to < n
        {
            have[to] += 1;
        }
    }
    let ok = |have: &[u64]| {
        have.iter()
            .zip(&want)
            .all(|(h, w)| w.is_none_or(|w| *h == w))
    };
    for (k, r) in recs.iter().enumerate().take(b).skip(a) {
        if ok(&have) {
            return k;
        }
        if let TraceEvent::Moved { to, .. } = r.event
            && to < n
        {
            have[to] -= 1;
        }
    }
    if ok(&have) { b } else { a }
}

impl AoVivo {
    /// Reads the trace of `replay` once. `model`: the model that produced it (the scrap
    /// sinks, the rework stations, the people); without it (a replay file) those stay out.
    /// `role(b)`: the mobile role of replay block `b`, as the scene draws it.
    pub fn new(
        replay: &Replay,
        model: Option<&Model>,
        role: impl Fn(usize) -> Option<MobileRole>,
    ) -> AoVivo {
        let n = replay.blocks.len();
        let obj = |name: &str| model.and_then(|m| m.objects.iter().find(|o| o.name == name));
        let mut blocos: Vec<Bloco> = replay
            .blocks
            .iter()
            .enumerate()
            .map(|(b, rb)| {
                let papel = if rb.kind == "Transporter" {
                    role(b)
                } else {
                    None
                };
                Bloco {
                    nome: rb.name.clone(),
                    kind: rb.kind.clone(),
                    classe: Classe::de(&rb.kind, papel),
                    papel,
                    antes: Vec::new(),
                    papel_saida: obj(&rb.name).and_then(|o| o.sink_role),
                }
            })
            .collect();
        for &(a, b) in &replay.connections {
            if a < n && b < n && blocos[a].classe == Classe::Fila && !blocos[b].antes.contains(&a) {
                blocos[b].antes.push(a);
            }
        }
        let refugo: Vec<bool> = blocos
            .iter()
            .map(|x| x.papel_saida == Some(SinkRole::Scrap))
            .collect();
        let retrabalho: Vec<bool> = blocos
            .iter()
            .map(|x| obj(&x.nome).is_some_and(|o| o.rework.is_some()))
            .collect();
        // Carriers and buffers (queues, racks, conveyors) only pass items on: what a
        // station sent is credited to it when the item reaches its next station.
        let passa: Vec<bool> = blocos
            .iter()
            .map(|x| x.kind == "Transporter" || x.classe == Classe::Fila)
            .collect();
        let saida: Vec<bool> = blocos.iter().map(|x| x.kind == "Sink").collect();
        let s0 = replay.report.stats_from;
        let antes = antes_do_aquecimento(replay, s0);
        let mut series = vec![Serie::default(); n];
        let mut cur = vec![0u32; n];
        // Where each item is, since when, when it was created, and the last station it
        // left (the destination is known when it reaches the next station).
        let mut onde: HashMap<u64, usize> = HashMap::new();
        let mut desde: HashMap<u64, f64> = HashMap::new();
        let mut criado: HashMap<u64, f64> = HashMap::new();
        let mut origem: HashMap<u64, usize> = HashMap::new();
        let mut ultimo: Vec<Option<State>> = vec![None; n];
        // The records of the instant `s0` that the engine ran before the end of the
        // warm-up go just before it (one float below): out of the window, as in the report.
        let antes_s0 = if s0 > 0.0 {
            f64::from_bits(s0.to_bits() - 1)
        } else {
            s0
        };
        for (k, r) in replay.records.iter().enumerate() {
            let t = if k < antes { antes_s0.min(r.t) } else { r.t };
            match &r.event {
                TraceEvent::Created { item, block } if *block < n => {
                    onde.insert(*item, *block);
                    desde.insert(*item, t);
                    criado.insert(*item, t);
                    cur[*block] += 1;
                    series[*block].conteudo.push(t, cur[*block]);
                }
                TraceEvent::Moved { item, to, .. } if *to < n => {
                    if let Some(f) = onde.remove(item) {
                        cur[f] = cur[f].saturating_sub(1);
                        series[f].conteudo.push(t, cur[f]);
                        let stay = t - desde.get(item).copied().unwrap_or(t);
                        series[f].saidas.push(t, stay);
                        if !passa[f] {
                            origem.insert(*item, f);
                        }
                    }
                    if !passa[*to]
                        && let Some(o) = origem.remove(item)
                    {
                        if refugo[*to] {
                            series[o].refugo.push(t, 1.0);
                        }
                        if retrabalho[*to] {
                            series[o].retrabalho.push(t, 1.0);
                        }
                    }
                    onde.insert(*item, *to);
                    desde.insert(*item, t);
                    cur[*to] += 1;
                    series[*to].conteudo.push(t, cur[*to]);
                    series[*to].entradas.push(t, 1.0);
                    if saida[*to] {
                        let c = criado.get(item).copied().unwrap_or(t);
                        series[*to].sistema.push(t, t - c);
                    }
                }
                TraceEvent::Transit { item, to, .. } if *to < n => {
                    // The destination counts when the item arrives (the `Moved` at the end).
                    if let Some(f) = onde.remove(item) {
                        cur[f] = cur[f].saturating_sub(1);
                        series[f].conteudo.push(t, cur[f]);
                        let stay = t - desde.get(item).copied().unwrap_or(t);
                        series[f].saidas.push(t, stay);
                        if !passa[f] {
                            origem.insert(*item, f);
                        }
                    }
                }
                TraceEvent::Packed { item, .. } => {
                    if let Some(f) = onde.remove(item) {
                        cur[f] = cur[f].saturating_sub(1);
                        series[f].conteudo.push(t, cur[f]);
                    }
                }
                TraceEvent::Unpacked { item, block } if *block < n => {
                    onde.insert(*item, *block);
                    desde.insert(*item, t);
                    cur[*block] += 1;
                    series[*block].conteudo.push(t, cur[*block]);
                }
                TraceEvent::Destroyed { item } => {
                    if let Some(f) = onde.remove(item) {
                        cur[f] = cur[f].saturating_sub(1);
                        series[f].conteudo.push(t, cur[f]);
                    }
                    desde.remove(item);
                    criado.remove(item);
                    origem.remove(item);
                }
                TraceEvent::State { block, state } if *block < n => {
                    if *state == State::Resting && ultimo[*block] != Some(State::Resting) {
                        series[*block].descansos.push(t, 1.0);
                    }
                    ultimo[*block] = Some(*state);
                    series[*block].estados.push(t, *state);
                }
                TraceEvent::Travel { carrier, .. } if *carrier < n => {
                    series[*carrier].viagens.push(t, 1.0);
                }
                _ => {}
            }
        }
        for s in &mut series {
            s.conteudo.fechar(s0);
            s.estados.fechar();
        }
        AoVivo {
            replicacao: replay.replication,
            inicio_estatisticas: s0,
            fim: if replay.report.end_time > 0.0 {
                replay.report.end_time
            } else {
                replay.stop_time
            },
            tem_refugo: refugo.iter().any(|&x| x),
            tem_retrabalho: retrabalho.iter().any(|&x| x)
                || model.is_some_and(|m| m.objects.iter().any(|o| o.inspection.is_some())),
            blocos,
            series,
            report: replay.report.clone(),
        }
    }

    fn indice(&self, nome: &str) -> Option<usize> {
        self.blocos.iter().position(|b| b.nome == nome)
    }

    /// Whether the panel has something to say about `nome`.
    pub fn conhece(&self, nome: &str) -> bool {
        self.indice(nome)
            .is_some_and(|i| !matches!(self.blocos[i].kind.as_str(), "Flow" | "Fluid"))
    }

    /// The window of the statistics at `t`: from the end of the warm-up, or from 0 while
    /// `t` is still in it.
    pub fn inicio(&self, t: f64) -> f64 {
        if t >= self.inicio_estatisticas {
            self.inicio_estatisticas
        } else {
            0.0
        }
    }

    /// The numbers of `nome` at `t`.
    pub fn numeros(&self, nome: &str, t: f64) -> Option<Numeros> {
        let i = self.indice(nome)?;
        Some(self.numeros_de(i, t))
    }

    fn numeros_de(&self, i: usize, t: f64) -> Numeros {
        let s = self.inicio(t);
        let desde_zero = s <= 0.0 && self.inicio_estatisticas > 0.0;
        let x = &self.series[i];
        let b = &self.blocos[i];
        let (fila, espera) = if b.antes.is_empty() {
            (None, None)
        } else {
            let fila = b
                .antes
                .iter()
                .map(|&q| self.series[q].conteudo.valor(t))
                .sum();
            // Pooled over the buffers: total stay over total exits.
            let (mut soma, mut n) = (0.0, 0u64);
            for &q in &b.antes {
                let c = &self.series[q].saidas;
                if let Some(m) = c.media(s, t) {
                    let k = c.n(s, t);
                    soma += m * k as f64;
                    n += k;
                }
            }
            (Some(fila), (n > 0).then(|| soma / n as f64))
        };
        Numeros {
            inicio: s,
            t,
            entradas: x.entradas.n(s, t),
            saidas: x.saidas.n(s, t),
            conteudo: x.conteudo.valor(t),
            conteudo_medio: x.conteudo.media(s, t),
            conteudo_max: x.conteudo.maximo(t, desde_zero),
            permanencia: x.saidas.media(s, t),
            fila,
            espera,
            refugo: x.refugo.n(s, t),
            retrabalho: x.retrabalho.n(s, t),
            viagens: x.viagens.n(s, t),
            descansos: x.descansos.n(s, t),
            tempo_sistema: x.sistema.media(s, t),
            estados: x.estados.fracoes(s, t),
        }
    }

    /// A curve of `f(bucket start, bucket end)` over `[0, t]` in [`PONTOS`] buckets, the
    /// last point at `t` itself.
    fn curva(t: f64, mut f: impl FnMut(f64, f64) -> f64) -> Vec<[f64; 2]> {
        if t <= 0.0 {
            return vec![[0.0, f(0.0, 0.0)]];
        }
        let n = PONTOS;
        let mut out = Vec::with_capacity(n + 1);
        out.push([0.0, f(0.0, 0.0)]);
        for k in 1..=n {
            let a = t * (k - 1) as f64 / n as f64;
            let b = t * k as f64 / n as f64;
            out.push([b, f(a, b)]);
        }
        out
    }

    /// What the panel shows of `nome` at `t`; `None` for an unknown name.
    pub fn leitura(&self, nome: &str, t: f64) -> Option<Leitura> {
        let i = self.indice(nome)?;
        let t = t.clamp(0.0, self.fim.max(0.0));
        let b = &self.blocos[i];
        let x = &self.series[i];
        let n = self.numeros_de(i, t);
        let tipo = match (b.classe, b.papel) {
            (Classe::Pessoa, _) => tx("godot-ao-vivo-pessoa").to_string(),
            _ if b.kind == "Crane" => tx("godot-ao-vivo-ponte_rolante").into(),
            (_, Some(MobileRole::Forklift)) => tx("godot-ao-vivo-empilhadeira").into(),
            (_, Some(MobileRole::Agv)) => "AGV".into(),
            (Classe::Veiculo, _) => tx("godot-ao-vivo-veiculo").into(),
            _ => crate::results::kind_label(&b.kind.to_lowercase()).to_string(),
        };
        let janela = if self.inicio_estatisticas > 0.0 && t < self.inicio_estatisticas {
            fx(
                "godot-ao-vivo-aquecimento_ate_numeros_desde_0",
                &[("inicio_estatisticas", vx(&num(self.inicio_estatisticas)))],
            )
        } else if self.inicio_estatisticas > 0.0 {
            fx(
                "godot-ao-vivo-desde_o_fim_do_aquecimento",
                &[("inicio_estatisticas", vx(&num(self.inicio_estatisticas)))],
            )
        } else {
            tx("godot-ao-vivo-desde_o_inicio").into()
        };
        let nota = fx(
            "godot-ao-vivo-t_de_replicacao",
            &[
                ("t", vx(&num(t))),
                ("fim", vx(&num(self.fim))),
                ("replicacao", vx(&self.replicacao)),
                ("janela", vx(&janela)),
            ],
        );
        let f = |s: State| n.fracao(s);
        let ocupado = f(State::Processing);
        let bloqueado = f(State::Blocked) + f(State::WaitingTransport);
        let parado = f(State::Down) + f(State::WaitingRepair) + f(State::Maintenance);
        let ocioso = f(State::Idle) + f(State::Empty);
        let mut secoes = Vec::new();
        let mut curvas = Vec::new();
        let conteudo = |nome: &str, cor: usize| {
            (
                nome.to_string(),
                Self::curva(t, |a, z| {
                    if z > a {
                        x.conteudo.media(a, z)
                    } else {
                        x.conteudo.valor(z) as f64
                    }
                }),
                cor,
            )
        };
        match b.classe {
            Classe::Processo => {
                let mut agora = vec![linha(
                    "EmProcesso",
                    tx("vivo-em-processamento-agora"),
                    n.conteudo.to_string(),
                    tx("vivo-itens-no-objeto-neste-instante"),
                )];
                if let Some(q) = n.fila {
                    agora.push(linha(
                        "Fila",
                        tx("vivo-itens-em-espera-fila"),
                        q.to_string(),
                        tx("vivo-itens-nas-filas-que-alimentam"),
                    ));
                }
                secoes.push(Secao {
                    titulo: tx("godot-ao-vivo-agora"),
                    linhas: agora,
                });
                let mut acum = vec![
                    linha(
                        "Entradas",
                        tx("vivo-entradas"),
                        n.entradas.to_string(),
                        tx("vivo-itens-que-entraram-na-janela"),
                    ),
                    linha(
                        "Processados",
                        tx("vivo-ja-processados"),
                        n.saidas.to_string(),
                        tx("vivo-itens-que-sairam-do-objeto"),
                    ),
                ];
                if self.tem_refugo || self.tem_retrabalho {
                    acum.push(if self.tem_refugo {
                        linha(
                            "Refugo",
                            tx("vivo-rejeitados-para-descarte"),
                            n.refugo.to_string(),
                            tx("vivo-itens-que-chegaram-a-uma"),
                        )
                    } else {
                        linha(
                            "Refugo",
                            tx("vivo-rejeitados-para-descarte"),
                            tx("godot-ao-vivo-marque_a_saida_de_refugo").into(),
                            tx("vivo-nenhuma-saida-tem-sink-role"),
                        )
                    });
                    acum.push(linha(
                        "Retrabalho",
                        tx("vivo-rejeitados-para-retrabalho"),
                        n.retrabalho.to_string(),
                        tx("vivo-itens-que-chegaram-a-um"),
                    ));
                }
                secoes.push(Secao {
                    titulo: tx("godot-ao-vivo-acumulado"),
                    linhas: acum,
                });
                let mut tempos = vec![linha(
                    "Ciclo",
                    tx("vivo-ciclo-medio-permanencia"),
                    opt(n.permanencia),
                    tx("vivo-tempo-medio-no-objeto-dos"),
                )];
                if n.fila.is_some() {
                    tempos.push(linha(
                        "Espera",
                        tx("vivo-espera-media-na-fila"),
                        opt(n.espera),
                        tx("vivo-tempo-medio-nas-filas-que"),
                    ));
                }
                tempos.extend([
                    linha(
                        "Ocupado",
                        tx("vivo-ocupado-processando"),
                        pct(ocupado),
                        tx("vivo-fracao-da-janela-no-estado"),
                    ),
                    linha(
                        "Ocioso",
                        tx("vivo-ocioso"),
                        pct(ocioso),
                        tx("vivo-fracao-da-janela-sem-trabalho"),
                    ),
                    linha(
                        "Bloqueado",
                        tx("vivo-bloqueado"),
                        pct(bloqueado),
                        tx("vivo-pronto-sem-poder-sair-bloqueado"),
                    ),
                    linha(
                        "Parado",
                        tx("vivo-parado-quebra"),
                        pct(parado),
                        tx("vivo-quebrado-aguardando-reparo-ou-em"),
                    ),
                    linha(
                        "Setup",
                        tx("vivo-setup"),
                        pct(f(State::Setup)),
                        tx("vivo-fracao-em-setup"),
                    ),
                ]);
                secoes.push(Secao {
                    titulo: tx("godot-ao-vivo-tempos"),
                    linhas: tempos,
                });
                curvas.push(conteudo(tx("godot-ao-vivo-em_processo_wip"), 0));
                if !b.antes.is_empty() {
                    let antes = b.antes.clone();
                    curvas.push((
                        tx("godot-ao-vivo-fila").to_string(),
                        Self::curva(t, |a, z| {
                            antes
                                .iter()
                                .map(|&q| {
                                    let c = &self.series[q].conteudo;
                                    if z > a {
                                        c.media(a, z)
                                    } else {
                                        c.valor(z) as f64
                                    }
                                })
                                .sum()
                        }),
                        1,
                    ));
                }
            }
            Classe::Fila | Classe::Outro | Classe::Chegada => {
                let mut linhas = vec![
                    linha(
                        "Conteudo",
                        tx("vivo-conteudo-agora"),
                        n.conteudo.to_string(),
                        tx("vivo-itens-no-objeto-neste-instante-2"),
                    ),
                    linha(
                        "ConteudoMedio",
                        tx("vivo-conteudo-medio"),
                        num(n.conteudo_medio),
                        tx("vivo-media-no-tempo-do-conteudo"),
                    ),
                    linha(
                        "ConteudoMax",
                        tx("vivo-conteudo-maximo"),
                        n.conteudo_max.to_string(),
                        tx("vivo-maior-conteudo-da-janela"),
                    ),
                ];
                if b.classe != Classe::Chegada {
                    linhas.push(linha(
                        "Espera",
                        tx("vivo-espera-media"),
                        opt(n.permanencia),
                        tx("vivo-tempo-medio-no-objeto-dos"),
                    ));
                    linhas.push(linha(
                        "Entradas",
                        tx("vivo-entradas"),
                        n.entradas.to_string(),
                        tx("vivo-itens-que-entraram-na-janela"),
                    ));
                }
                linhas.push(linha(
                    "Saidas",
                    tx("vivo-saidas"),
                    n.saidas.to_string(),
                    tx("vivo-itens-que-sairam-na-janela"),
                ));
                if b.classe == Classe::Chegada {
                    linhas.push(linha(
                        "Bloqueado",
                        tx("vivo-bloqueado"),
                        pct(bloqueado),
                        tx("vivo-fracao-da-janela-sem-poder"),
                    ));
                }
                secoes.push(Secao {
                    titulo: tx("godot-ao-vivo-agora_e_na_janela"),
                    linhas,
                });
                curvas.push(conteudo(tx("godot-ao-vivo-conteudo"), 0));
            }
            Classe::Saida => {
                let span = t - n.inicio;
                let vazao = if span > 0.0 {
                    n.entradas as f64 / span
                } else {
                    0.0
                };
                let mut linhas = vec![
                    linha(
                        "Total",
                        tx("vivo-recebidos-total"),
                        n.entradas.to_string(),
                        tx("vivo-itens-que-chegaram-a-saida"),
                    ),
                    linha(
                        "Vazao",
                        tx("vivo-vazao-por-unidade-de-tempo"),
                        num(vazao),
                        tx("vivo-recebidos-divididos-pela-duracao-da"),
                    ),
                    linha(
                        "TempoSistema",
                        tx("vivo-tempo-medio-no-sistema"),
                        opt(n.tempo_sistema),
                        tx("vivo-da-criacao-do-item-a"),
                    ),
                ];
                if let Some(p) = b.papel_saida {
                    linhas.push(linha(
                        "Papel",
                        tx("vivo-papel-na-qualidade"),
                        match p {
                            SinkRole::Scrap => "refugo".into(),
                            SinkRole::Customer => "cliente".into(),
                        },
                        tx("vivo-o-papel-da-saida-nos"),
                    ));
                }
                secoes.push(Secao {
                    titulo: tx("godot-ao-vivo-saida"),
                    linhas,
                });
                let ent = &x.entradas;
                curvas.push((
                    tx("godot-ao-vivo-recebidos_acumulado").to_string(),
                    Self::curva(t, |_, z| ent.n(0.0, z) as f64),
                    0,
                ));
            }
            Classe::Veiculo | Classe::Pessoa => {
                let mut linhas = vec![
                    linha(
                        "Utilizacao",
                        tx("vivo-utilizacao"),
                        pct(n.utilizacao()),
                        tx("vivo-andando-carregando-descarregando-ou-trabalhando"),
                    ),
                    linha(
                        "Viagens",
                        tx("vivo-viagens"),
                        n.viagens.to_string(),
                        tx("vivo-deslocamentos-comecados-na-janela"),
                    ),
                    linha(
                        "Transportados",
                        tx("vivo-itens-transportados"),
                        n.saidas.to_string(),
                        tx("vivo-itens-entregues-na-janela"),
                    ),
                    linha(
                        "Carregando",
                        tx("vivo-carregando-agora"),
                        n.conteudo.to_string(),
                        tx("vivo-itens-consigo-neste-instante"),
                    ),
                    linha(
                        "Carregado",
                        tx("vivo-viajando-carregado"),
                        pct(f(State::TravelLoaded)),
                        tx("vivo-fracao-da-janela-andando-com"),
                    ),
                    linha(
                        "Vazio",
                        tx("vivo-viajando-vazio"),
                        pct(f(State::TravelEmpty)),
                        tx("vivo-fracao-da-janela-andando-sem"),
                    ),
                ];
                if b.classe == Classe::Pessoa {
                    linhas.push(linha(
                        "NoPosto",
                        tx("vivo-trabalhando-num-posto"),
                        pct(f(State::Utilize)),
                        tx("vivo-fracao-da-janela-operando-um"),
                    ));
                    linhas.push(linha(
                        "Descanso",
                        tx("vivo-descanso-fadiga"),
                        pct(f(State::Resting)),
                        tx("vivo-fracao-da-janela-em-pausa"),
                    ));
                    linhas.push(linha(
                        "Pausas",
                        tx("vivo-pausas"),
                        n.descansos.to_string(),
                        tx("vivo-vezes-que-entrou-em-descanso"),
                    ));
                    let c = self.report.block(&b.nome).map(|r| &r.custom);
                    if let Some(m) = c.and_then(|c| c.get("person.fatigue_mean")) {
                        let mx = c
                            .and_then(|c| c.get("person.fatigue_max"))
                            .copied()
                            .unwrap_or(f64::NAN);
                        linhas.push(linha(
                            "Fadiga",
                            tx("vivo-fadiga-media-maxima-fim-da"),
                            format!("{} / {}", num(*m), num(mx)),
                            tx("vivo-a-fadiga-nao-esta-no"),
                        ));
                    }
                } else {
                    if b.kind == "Crane" {
                        linhas.push(linha(
                            "EsperaOperador",
                            tx("vivo-esperando-o-operador"),
                            pct(f(State::WaitingOperator)),
                            tx("vivo-fracao-da-janela-parada-na"),
                        ));
                    }
                    linhas.push(linha(
                        "Parado",
                        tx("vivo-parado-quebra"),
                        pct(parado),
                        tx("vivo-quebrado-aguardando-reparo-ou-em"),
                    ));
                }
                secoes.push(Secao {
                    titulo: if b.classe == Classe::Pessoa {
                        tx("godot-ao-vivo-pessoa-2")
                    } else if b.kind == "Crane" {
                        tx("godot-ao-vivo-ponte_rolante-2")
                    } else {
                        tx("godot-ao-vivo-veiculo-2")
                    },
                    linhas,
                });
                curvas.push(conteudo(tx("godot-ao-vivo-itens_consigo"), 0));
            }
        }
        let estados = STATES
            .iter()
            .filter_map(|s| n.estados.get(s).map(|f| (*s, *f)))
            .filter(|(_, f)| *f > 0.0)
            .collect();
        Some(Leitura {
            nome: b.nome.clone(),
            tipo,
            classe: b.classe,
            nota,
            secoes,
            curvas,
            estados,
            numeros: n,
        })
    }

    /// The differences between the numbers of `nome` at the end of the replication and
    /// its `RunReport` (empty: they agree). The "Resultados" numbers are the mean of the
    /// replications; this is the one replication played.
    pub fn confere(&self, nome: &str) -> Vec<String> {
        let Some(r) = self.report.block(nome) else {
            return vec![format!("{nome}: não está no relatório")];
        };
        let Some(n) = self.numeros(nome, self.report.end_time) else {
            return vec![format!("{nome}: não está na reprodução")];
        };
        let mut erros = Vec::new();
        let perto = |a: f64, b: f64| (a - b).abs() <= 1e-7 * (1.0 + b.abs());
        let mut igual = |o: &str, a: f64, b: f64| {
            if !perto(a, b) {
                erros.push(format!("{nome}.{o}: painel {a}, relatório {b}"));
            }
        };
        igual("input", n.entradas as f64, r.input as f64);
        igual("output", n.saidas as f64, r.output as f64);
        igual("content_now", n.conteudo as f64, r.content_now as f64);
        igual("content_mean", n.conteudo_medio, r.content_mean);
        igual("content_max", n.conteudo_max as f64, r.content_max);
        igual("staytime.count", n.saidas as f64, r.staytime.count as f64);
        if r.staytime.count > 0 {
            igual(
                "staytime.mean",
                n.permanencia.unwrap_or(f64::NAN),
                r.staytime.mean,
            );
        }
        for s in STATES {
            igual(
                &format!("state.{}", s.name()),
                n.fracao(s),
                r.states.get(s.name()).copied().unwrap_or(0.0),
            );
        }
        if let Some(ts) = r.extra.get("time_in_system").filter(|x| x.count > 0) {
            igual(
                "time_in_system.mean",
                n.tempo_sistema.unwrap_or(f64::NAN),
                ts.mean,
            );
        }
        if r.kind == "transporter"
            && let Some(u) = r.utilization()
        {
            igual("utilization", n.utilizacao(), u.work);
        }
        if let Some(rest) = r.custom.get("person.rest_time") {
            let span = self.report.end_time - n.inicio;
            igual("person.rest_time", n.fracao(State::Resting) * span, *rest);
        }
        erros
    }

    /// Every drawn object's differences (see [`AoVivo::confere`]).
    pub fn confere_todos(&self) -> Vec<String> {
        self.blocos
            .iter()
            .filter(|b| !matches!(b.kind.as_str(), "Flow" | "Fluid"))
            .flat_map(|b| self.confere(&b.nome))
            .collect()
    }
}

/// Whether the panel numbers must be computed again: the time moved and, while the clock
/// plays, at most every [`INTERVALO`] seconds of real time (`agora`, `antes`: real time
/// now and at the last computation).
pub fn recalcular(t: f64, t_antes: f64, tocando: bool, agora: f64, antes: f64) -> bool {
    if t == t_antes {
        return false;
    }
    !tocando || agora - antes >= INTERVALO
}

/// One object as a click sees it: the box of the scene's geometry (`tamanho`, in the
/// object's own axes) around `centro`, turned by `yaw` about the vertical (Godot's
/// `rotation.y`), and whether it is drawn (`oculto`: hidden for the camera's view).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Caixa {
    pub centro: [f32; 3],
    pub tamanho: [f32; 3],
    pub yaw: f32,
    pub oculto: bool,
}

/// The object a ray from `origem` along `dir` hits first; `None` when it hits none.
/// Hidden objects and the ones whose box holds the eye (the vehicle a driver's camera
/// rides) are skipped. Each side is at least 0.6 m, so a thin object can be clicked.
pub fn escolher(origem: [f32; 3], dir: [f32; 3], caixas: &[Caixa]) -> Option<usize> {
    let mut melhor: Option<(usize, f32)> = None;
    for (i, c) in caixas.iter().enumerate() {
        if c.oculto {
            continue;
        }
        // The ray in the box's own axes (turned back by `yaw`, as Godot turns about +Y).
        let (sn, cs) = c.yaw.sin_cos();
        let rel = [
            origem[0] - c.centro[0],
            origem[1] - c.centro[1],
            origem[2] - c.centro[2],
        ];
        let local = |v: [f32; 3]| [v[0] * cs - v[2] * sn, v[1], v[0] * sn + v[2] * cs];
        let (o, d) = (local(rel), local(dir));
        let half = c.tamanho.map(|x| (x * 0.5).max(0.3));
        let (mut t0, mut t1) = (f32::NEG_INFINITY, f32::INFINITY);
        let mut acerta = true;
        for k in 0..3 {
            if d[k].abs() < 1e-9 {
                if o[k].abs() > half[k] {
                    acerta = false;
                    break;
                }
                continue;
            }
            let a = (-half[k] - o[k]) / d[k];
            let b = (half[k] - o[k]) / d[k];
            t0 = t0.max(a.min(b));
            t1 = t1.min(a.max(b));
        }
        // `t0 <= 0`: the eye is inside the box (or it is behind).
        if !acerta || t0 > t1 || t0 <= 1e-4 {
            continue;
        }
        if melhor.is_none_or(|(_, best)| t0 < best) {
            melhor = Some((i, t0));
        }
    }
    melhor.map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn vivo(name: &str) -> (Model, Replay, AoVivo) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples")
            .join(name);
        let model = Model::load(&path).expect("model");
        let replay = model.replay(0).expect("replay");
        let v = AoVivo::new(&replay, Some(&model), |b| {
            Some(utsim_scene::compose::role_of(
                &replay.blocks[b].name,
                Some(&model),
            ))
        });
        (model, replay, v)
    }

    fn sem_diferencas(name: &str) -> AoVivo {
        let (_, _, v) = vivo(name);
        let erros = v.confere_todos();
        assert!(erros.is_empty(), "{name}: {erros:#?}");
        v
    }

    #[test]
    fn at_the_end_the_quality_model_matches_its_report() {
        let (model, replay, v) = vivo("inspecao_e_retrabalho.ron");
        let erros = v.confere_todos();
        assert!(erros.is_empty(), "{erros:#?}");
        assert!(v.inicio_estatisticas > 0.0, "the example has a warm-up");
        let end = replay.report.end_time;
        let insp = v.numeros("inspecao", end).unwrap();
        let rep = |n: &str| replay.report.block(n).unwrap();
        // Where the rejected items went: every item the scrap sink and the rework
        // station received came from the inspection.
        assert!(insp.refugo > 0 && insp.retrabalho > 0, "{insp:?}");
        assert_eq!(insp.refugo, rep("refugo").input);
        assert_eq!(insp.retrabalho, rep("retrabalho").input);
        // The queue in front of the inspection, and its mean wait.
        assert_eq!(insp.fila, Some(rep("fila").content_now as u32));
        let espera = insp.espera.unwrap();
        assert!((espera - rep("fila").staytime.mean).abs() < 1e-7 * (1.0 + espera));
        // The replication played is replication 0 of the run the results window shows.
        let run = model.run_replications().expect("run");
        assert_eq!(run[0].metrics(), replay.report.metrics());
        // The lines of the panel say the same numbers.
        let l = v.leitura("inspecao", end).unwrap();
        let valor = |k: &str| {
            l.secoes
                .iter()
                .flat_map(|s| &s.linhas)
                .find(|x| x.chave == k)
                .map(|x| x.valor.clone())
                .unwrap()
        };
        assert_eq!(valor("Processados"), rep("inspecao").output.to_string());
        assert_eq!(valor("Refugo"), rep("refugo").input.to_string());
        assert_eq!(l.classe, Classe::Processo);
        assert_eq!(l.curvas.len(), 2, "WIP and queue");
        assert!(l.curvas.iter().all(|c| c.1.len() == PONTOS + 1));
        let total: f64 = l.estados.iter().map(|e| e.1).sum();
        assert!((total - 1.0).abs() < 1e-6, "{total}");
    }

    #[test]
    fn at_the_end_the_vehicles_match_their_report() {
        let v = sem_diferencas("agvs.ron");
        let n = v.numeros("agv_1", v.fim).unwrap();
        assert!(n.viagens > 0 && n.saidas > 0, "{n:?}");
        let l = v.leitura("agv_1", v.fim).unwrap();
        assert_eq!(l.classe, Classe::Veiculo);
    }

    #[test]
    fn at_the_end_the_people_match_their_report() {
        let v = sem_diferencas("pessoas_fadiga.ron");
        let l = v.leitura("ana", v.fim).unwrap();
        assert_eq!(l.classe, Classe::Pessoa);
        assert!(
            l.secoes[0].linhas.iter().any(|x| x.chave == "Fadiga"),
            "the fatigue of the report"
        );
        // Operators of a cell, with a warm-up.
        sem_diferencas("celula_com_operadores.ron");
    }

    fn de_texto(text: &str) -> (Replay, AoVivo) {
        let model = Model::parse(text).expect("model");
        let replay = model.replay(0).expect("replay");
        let v = AoVivo::new(&replay, Some(&model), |_| None);
        (replay, v)
    }

    #[test]
    fn rejected_items_are_credited_through_queues() {
        // A queue between the inspection and the rework station, and one before the scrap
        // sink: the rejected items still count for the inspection, when they arrive.
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/inspecao_e_retrabalho.ron");
        let text = std::fs::read_to_string(path).unwrap();
        let text = text
            .replace(
                r#"("inspecao", "retrabalho"), ("inspecao", "refugo"),"#,
                r#"("inspecao", "fila_retrabalho"), ("inspecao", "fila_refugo"),
        ("fila_retrabalho", "retrabalho"), ("fila_refugo", "refugo"),"#,
            )
            .replace(
                r#"(name: "cliente","#,
                r#"(name: "fila_retrabalho", kind: Queue(), position: (10.0, -2.0, 0.0)),
        (name: "fila_refugo", kind: Queue(), position: (12.0, -2.0, 0.0)),
        (name: "cliente","#,
            );
        assert!(
            text.contains("fila_retrabalho\", \"retrabalho"),
            "the edit applied"
        );
        let (replay, v) = de_texto(&text);
        let erros = v.confere_todos();
        assert!(erros.is_empty(), "{erros:#?}");
        let end = replay.report.end_time;
        let insp = v.numeros("inspecao", end).unwrap();
        let rep = |n: &str| replay.report.block(n).unwrap();
        assert!(insp.refugo > 0 && insp.retrabalho > 0, "{insp:?}");
        assert_eq!(insp.retrabalho, rep("retrabalho").input);
        assert_eq!(insp.refugo, rep("refugo").input);
        // The queues themselves send nothing to scrap or rework.
        let q = v.numeros("fila_retrabalho", end).unwrap();
        assert_eq!((q.refugo, q.retrabalho), (0, 0));
    }

    #[test]
    fn without_a_scrap_sink_the_panel_asks_to_mark_it() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/inspecao_e_retrabalho.ron");
        let text = std::fs::read_to_string(path)
            .unwrap()
            .replace("sink_role: Some(Scrap), ", "");
        let (_, v) = de_texto(&text);
        assert!(!v.tem_refugo && v.tem_retrabalho);
        let l = v.leitura("inspecao", v.fim).unwrap();
        let refugo = l
            .secoes
            .iter()
            .flat_map(|s| &s.linhas)
            .find(|x| x.chave == "Refugo")
            .unwrap();
        assert_eq!(refugo.valor, "marque a saída de refugo");
    }

    #[test]
    fn events_at_the_end_of_the_warm_up_fall_on_the_report_side() {
        // The first arrival is at t = 100 exactly, the end of the warm-up: the source
        // scheduled it while starting, so the engine runs it before ending the warm-up and
        // the report does not count it; the next ones (every 100) it counts.
        let (replay, v) = de_texto(
            r#"Model(name: "borda", run: (stop_time: 1000.0, warmup: 100.0, seed: 1, replications: 1),
            objects: [
                (name: "chegada", kind: Source(interarrival: Const(100.0)), position: (0.0, 0.0, 0.0)),
                (name: "posto", kind: Processor(process: Const(30.0)), position: (2.0, 0.0, 0.0)),
                (name: "saida", kind: Sink, position: (4.0, 0.0, 0.0)),
            ],
            connections: [("chegada", "posto"), ("posto", "saida")])"#,
        );
        assert!(
            replay.records.iter().any(|r| r.t == 100.0),
            "an event at the boundary"
        );
        // Some of them ran before the end of the warm-up.
        let a = replay.records.partition_point(|r| r.t < 100.0);
        assert!(antes_do_aquecimento(&replay, 100.0) > a);
        let erros = v.confere_todos();
        assert!(erros.is_empty(), "{erros:#?}");
        // A warm-up of 130: the end of the first process (at 130) was scheduled after the
        // end of the warm-up, so it runs after it and counts.
        let (_, v) = de_texto(
            r#"Model(name: "borda", run: (stop_time: 1000.0, warmup: 130.0, seed: 1, replications: 1),
            objects: [
                (name: "chegada", kind: Source(interarrival: Const(100.0)), position: (0.0, 0.0, 0.0)),
                (name: "posto", kind: Processor(process: Const(30.0)), position: (2.0, 0.0, 0.0)),
                (name: "saida", kind: Sink, position: (4.0, 0.0, 0.0)),
            ],
            connections: [("chegada", "posto"), ("posto", "saida")])"#,
        );
        let erros = v.confere_todos();
        assert!(erros.is_empty(), "{erros:#?}");
    }

    #[test]
    fn the_crane_panel_shows_the_wait_for_its_operator() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../godot/roteiros/g4_ponte_botoeira.ron");
        let model = Model::load(&path).expect("model");
        let replay = model.replay(0).expect("replay");
        let v = AoVivo::new(&replay, Some(&model), |b| {
            Some(utsim_scene::compose::role_of(
                &replay.blocks[b].name,
                Some(&model),
            ))
        });
        assert!(v.confere_todos().is_empty(), "{:?}", v.confere_todos());
        let l = v.leitura("ponte", replay.stop_time).expect("ponte");
        assert_eq!(l.classe, Classe::Veiculo);
        assert_eq!(l.tipo, "ponte rolante");
        let secao = l
            .secoes
            .iter()
            .find(|s| s.titulo == "Ponte rolante")
            .unwrap();
        let espera = secao
            .linhas
            .iter()
            .find(|x| x.chave == "EsperaOperador")
            .expect("the line of the wait");
        assert_ne!(espera.valor, pct(0.0), "the crane waited for its operator");
        assert!(l.numeros.fracao(State::WaitingOperator) > 0.0);
        // The person at the pendant works (utilize) while running the crane.
        let p = v.leitura("operador", replay.stop_time).expect("operador");
        assert_eq!(p.classe, Classe::Pessoa);
        assert!(p.numeros.fracao(State::Utilize) > 0.0);
    }

    #[test]
    fn other_examples_match_their_reports_too() {
        for m in ["tutorial_01_caixa.ron", "servidores.ron", "manutencao.ron"] {
            sem_diferencas(m);
        }
    }

    #[test]
    fn the_numbers_grow_with_the_time_and_go_back_with_it() {
        let (_, _, v) = vivo("inspecao_e_retrabalho.ron");
        // In the warm-up the window starts at 0.
        let a = v.numeros("torno", 100.0).unwrap();
        assert_eq!(a.inicio, 0.0);
        let b = v.numeros("torno", 2000.0).unwrap();
        let c = v.numeros("torno", 4000.0).unwrap();
        assert_eq!(b.inicio, v.inicio_estatisticas);
        assert!(b.saidas < c.saidas);
        // Asking again for an earlier time gives the same numbers (scrubbing back).
        assert_eq!(v.numeros("torno", 2000.0).unwrap(), b);
        let l = v.leitura("torno", 2000.0).unwrap();
        let last = l.curvas[0].1.last().unwrap();
        assert_eq!(last[0], 2000.0);
        assert!(v.leitura("nao_existe", 1.0).is_none());
    }

    #[test]
    fn the_numbers_are_computed_again_only_when_needed() {
        assert!(!recalcular(5.0, 5.0, false, 1.0, 0.0));
        assert!(recalcular(5.0, 4.0, false, 0.01, 0.0), "paused: at once");
        assert!(!recalcular(5.0, 4.0, true, 0.1, 0.0), "playing: throttled");
        assert!(recalcular(5.0, 4.0, true, 0.25, 0.0));
    }

    #[test]
    fn a_ray_picks_the_nearest_box() {
        let caixa = |centro: [f32; 3], tamanho: [f32; 3], yaw: f32| Caixa {
            centro,
            tamanho,
            yaw,
            oculto: false,
        };
        let mut caixas = vec![
            caixa([0.0, 0.5, -10.0], [1.0; 3], 0.0),
            caixa([0.0, 0.5, -5.0], [1.0; 3], 0.0),
            caixa([5.0, 0.5, -5.0], [1.0; 3], 0.0),
        ];
        let o = [0.0, 0.5, 0.0];
        assert_eq!(escolher(o, [0.0, 0.0, -1.0], &caixas), Some(1));
        assert_eq!(escolher(o, [0.0, 0.0, 1.0], &caixas), None);
        assert_eq!(escolher(o, [0.0, 1.0, 0.0], &caixas), None);
        // Hidden for the view: the one behind it.
        caixas[1].oculto = true;
        assert_eq!(escolher(o, [0.0, 0.0, -1.0], &caixas), Some(0));
        // The eye inside a box (a driver's camera in its vehicle): that box is skipped.
        caixas[1].oculto = false;
        assert_eq!(
            escolher([0.0, 0.5, -5.0], [0.0, 0.0, -1.0], &caixas),
            Some(0)
        );
    }

    #[test]
    fn a_long_object_is_picked_along_its_length_only() {
        // A 10 x 1 m conveyor along x next to a 1 m box at x = 0, z = -2.
        let esteira = |yaw: f32| Caixa {
            centro: [0.0, 0.5, 0.0],
            tamanho: [10.0, 1.0, 1.0],
            yaw,
            oculto: false,
        };
        let vizinho = Caixa {
            centro: [0.0, 0.5, -2.0],
            tamanho: [1.0; 3],
            yaw: 0.0,
            oculto: false,
        };
        // Looking down at z = -2 from above: the neighbour, not the conveyor.
        let down = [0.0, -1.0, 0.0];
        assert_eq!(
            escolher([0.0, 5.0, -2.0], down, &[esteira(0.0), vizinho]),
            Some(1)
        );
        assert_eq!(
            escolher([4.0, 5.0, 0.0], down, &[esteira(0.0), vizinho]),
            Some(0)
        );
        // Turned 90 degrees, it runs along z: now it covers z = -2 and not x = 4.
        let q = std::f32::consts::FRAC_PI_2;
        assert_eq!(escolher([0.0, 5.0, -4.0], down, &[esteira(q)]), Some(0));
        assert_eq!(escolher([4.0, 5.0, 0.0], down, &[esteira(q)]), None);
    }
}
