//! The `.gemeo.ron` file: connections to the plant, the mapping sensor → object, the
//! shadow settings and the recording settings. It lives next to the model, not inside
//! it: addresses and topics change per installation, the model stays portable.
//!
//! ```text
//! Twin(
//!     model: "linha.ron",
//!     clock: Tag(source: "clp", tag: "UTSim/linha/TempoSimulado"),
//!     sources: [
//!         (name: "clp", kind: OpcUa(endpoint: "opc.tcp://127.0.0.1:4840/",
//!                                   namespace: "urn:utsim:emulacao")),
//!         (name: "uns", kind: Mqtt(host: "127.0.0.1", topics: ["fabrica/#"])),
//!     ],
//!     tags: [
//!         (source: "clp", tag: "UTSim/linha/maquina/Saidas", object: "maquina", measure: Outputs),
//!         (source: "clp", tag: "UTSim/linha/maquina/Estado", object: "maquina", measure: State),
//!         (source: "uns", tag: "fabrica/fila/qtd", object: "fila", measure: Content, deadband: 0.5),
//!     ],
//!     shadow: (window: 600.0, replications: 20),
//!     record: (keep_days: 30, anonymize_people: true),
//! )
//! ```
//!
//! Field names are in English, with Portuguese aliases (`modelo`, `relogio`, `conexoes`,
//! `mapeamento`, `sombra`, `gravacao`, ...), as in the model and the state files.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use utsim_model::{Model, ObjectKind};

/// Contents of a `.gemeo.ron` file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Twin", deny_unknown_fields)]
pub struct TwinConfig {
    /// Model file, relative to the `.gemeo.ron` file.
    #[serde(alias = "modelo")]
    pub model: String,
    /// Hash of the model text when the mapping was written ([`model_hash`]); a warning
    /// when the model changed since.
    #[serde(
        default,
        alias = "hash_modelo",
        skip_serializing_if = "Option::is_none"
    )]
    pub model_hash: Option<String>,
    #[serde(default, alias = "relogio")]
    pub clock: ClockSpec,
    #[serde(default, alias = "conexoes", alias = "fontes")]
    pub sources: Vec<SourceDef>,
    #[serde(default, alias = "mapeamento")]
    pub tags: Vec<TagMap>,
    #[serde(default, alias = "sombra")]
    pub shadow: ShadowSettings,
    #[serde(default, alias = "gravacao")]
    pub record: RecordSettings,
}

/// Where the twin's time comes from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClockSpec {
    /// The local clock: one model time unit lasts `unit_seconds` seconds.
    #[serde(alias = "Local")]
    Wall {
        #[serde(default = "one", alias = "unidade_segundos")]
        unit_seconds: f64,
    },
    /// A tag carries the time (a PLC counter, the `TempoSimulado` of an emulation): every
    /// observation gets the last value read from it.
    #[serde(alias = "Etiqueta")]
    Tag {
        #[serde(alias = "conexao", alias = "fonte")]
        source: String,
        tag: String,
    },
}

impl Default for ClockSpec {
    fn default() -> Self {
        ClockSpec::Wall { unit_seconds: 1.0 }
    }
}

fn one() -> f64 {
    1.0
}

/// One connection to the plant. All of them only read.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceDef {
    #[serde(alias = "nome")]
    pub name: String,
    #[serde(alias = "tipo")]
    pub kind: SourceKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SourceKind {
    /// OPC UA client: subscribes to the mapped nodes (monitored items).
    OpcUa {
        endpoint: String,
        /// Namespace URI of tags written without `ns=`; absent: namespace 0 is not
        /// assumed, every tag must then be a full node id (`ns=2;s=M1.Estado`).
        #[serde(default)]
        namespace: Option<String>,
        /// Publishing and sampling interval, milliseconds.
        #[serde(default = "default_interval", alias = "intervalo_ms")]
        interval_ms: u64,
        #[serde(default, alias = "seguranca")]
        security: OpcSecurity,
        /// Accept the server certificate without it being in the trusted folder.
        #[serde(default, alias = "confiar_servidor")]
        trust_server: bool,
    },
    /// MQTT 3.1.1 client: subscribes to the topics; the payload is a number, `true` or
    /// `false`, a text or a JSON document.
    Mqtt {
        host: String,
        #[serde(default = "default_mqtt_port", alias = "porta")]
        port: u16,
        #[serde(alias = "topicos")]
        topics: Vec<String>,
        #[serde(default)]
        client_id: Option<String>,
        /// Environment variable with `user:password` (never in the file).
        #[serde(default, alias = "credenciais_env")]
        credentials_env: Option<String>,
    },
    /// Sparkplug B over MQTT: births, data and deaths of the edge nodes of one group.
    Sparkplug {
        host: String,
        #[serde(default = "default_mqtt_port", alias = "porta")]
        port: u16,
        #[serde(alias = "grupo")]
        group: String,
        #[serde(default)]
        client_id: Option<String>,
        #[serde(default, alias = "credenciais_env")]
        credentials_env: Option<String>,
    },
}

fn default_interval() -> u64 {
    100
}

fn default_mqtt_port() -> u16 {
    1883
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpcSecurity {
    /// No signature nor encryption (a warning is printed).
    #[default]
    #[serde(alias = "Nenhuma")]
    None,
    /// Basic256Sha256, signed and encrypted.
    #[serde(alias = "AssinadoCifrado")]
    SignAndEncrypt,
}

/// What a tag measures on its object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Measure {
    /// Count of items that entered.
    #[serde(alias = "Entradas")]
    Inputs,
    /// Count of items that left.
    #[serde(alias = "Saidas")]
    Outputs,
    /// State code (running, stopped, broken down).
    #[serde(alias = "Estado")]
    State,
    /// Items in the object (queue length, WIP).
    #[serde(alias = "Conteudo")]
    Content,
    /// Position of a mobile resource.
    #[serde(alias = "Posicao")]
    Position,
}

impl Measure {
    pub fn label(self) -> &'static str {
        match self {
            Measure::Inputs => "entradas",
            Measure::Outputs => "saídas",
            Measure::State => "estado",
            Measure::Content => "conteúdo",
            Measure::Position => "posição",
        }
    }
}

/// How a counter tag reports.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Counter {
    /// A running total (the increments are counted; a decrease is a reset).
    #[default]
    Total,
    /// Each reading is an increment (a pulse `true` counts 1).
    #[serde(alias = "Pulso")]
    Pulse,
}

/// The three state classes the shadow tracks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StateClass {
    #[serde(alias = "Rodando")]
    Running,
    #[serde(alias = "Parado")]
    Stopped,
    #[serde(alias = "Quebrado")]
    Down,
}

impl StateClass {
    pub fn label(self) -> &'static str {
        match self {
            StateClass::Running => "rodando",
            StateClass::Stopped => "parado",
            StateClass::Down => "quebrado",
        }
    }

    pub fn index(self) -> usize {
        match self {
            StateClass::Running => 0,
            StateClass::Stopped => 1,
            StateClass::Down => 2,
        }
    }

    /// The class of a UTSim state name (`processing`, `down`...) or of the Portuguese
    /// words `rodando`, `parado`, `quebrado`.
    pub fn of_name(name: &str) -> Option<StateClass> {
        Some(match name.trim().to_ascii_lowercase().as_str() {
            "processing" | "setup" | "utilize" | "travel_empty" | "travel_loaded" | "loading"
            | "unloading" | "generating" | "releasing" | "collecting" | "rodando" | "running" => {
                StateClass::Running
            }
            "idle" | "empty" | "blocked" | "waiting_operator" | "waiting_transport"
            | "off_shift" | "stopped" | "parado" => StateClass::Stopped,
            "down" | "failed" | "quebrado" => StateClass::Down,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// One mapping rule: a tag of a source → a measure of a model object, with its
/// conversion and filters. The value used is `raw · scale · unit + offset`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TagMap {
    #[serde(alias = "conexao", alias = "fonte")]
    pub source: String,
    /// Node id (OPC UA), topic (MQTT) or `group/edge[/device]/metric` (Sparkplug).
    pub tag: String,
    #[serde(alias = "objeto")]
    pub object: String,
    #[serde(alias = "medida")]
    pub measure: Measure,
    #[serde(default = "one", alias = "escala")]
    pub scale: f64,
    #[serde(default, alias = "deslocamento")]
    pub offset: f64,
    /// Unit of the raw value: `m`, `cm`, `mm` for positions; `un` (or nothing) for counts.
    #[serde(default, alias = "unidade", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, alias = "contador")]
    pub counter: Counter,
    /// The counter wraps around at this value (e.g. 65536).
    #[serde(default, alias = "volta", skip_serializing_if = "Option::is_none")]
    pub rollover: Option<f64>,
    /// Changes smaller than this are ignored (content and position).
    #[serde(
        default,
        alias = "banda_morta",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadband: Option<f64>,
    /// Values outside `[min, max]` are discarded and counted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// A new state only counts after lasting this long (model time units).
    #[serde(
        default,
        alias = "atraso_estado",
        skip_serializing_if = "Option::is_none"
    )]
    pub debounce: Option<f64>,
    /// State codes of this tag → class, on top of the UTSim state names.
    #[serde(default, alias = "estados", skip_serializing_if = "BTreeMap::is_empty")]
    pub states: BTreeMap<String, StateClass>,
    /// JSON pointer of the value inside a JSON payload (`/valor`, `/pos/x`).
    #[serde(default, alias = "campo", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// For positions read one axis per tag.
    #[serde(default, alias = "eixo", skip_serializing_if = "Option::is_none")]
    pub axis: Option<Axis>,
    /// The tag identifies a person: anonymised in the recording.
    #[serde(default, alias = "pessoa")]
    pub person: bool,
}

/// Shadow mode settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShadowSettings {
    /// Length of the KPI windows (model time units).
    #[serde(default = "default_window", alias = "janela")]
    pub window: f64,
    /// Replications of the model that give the simulated distribution of each KPI.
    #[serde(default = "default_reps", alias = "replicacoes")]
    pub replications: u32,
    /// Real time ignored at the start (the first window starts here).
    #[serde(default, alias = "aquecimento")]
    pub warmup: f64,
    /// Warm-up of the simulated runs (absent: the model's `run.warmup`).
    #[serde(
        default,
        alias = "aquecimento_simulado",
        skip_serializing_if = "Option::is_none"
    )]
    pub sim_warmup: Option<f64>,
    /// False discovery rate of the window tests (Benjamini-Hochberg).
    #[serde(default = "default_fdr", alias = "taxa_falsas_descobertas")]
    pub fdr: f64,
    /// In-control average run length wanted for the whole set of KPIs (CUSUM and EWMA
    /// limits widen with the number of KPIs).
    #[serde(default = "default_arl0")]
    pub arl0: f64,
    #[serde(default = "default_lambda", alias = "lambda_ewma")]
    pub ewma_lambda: f64,
    #[serde(default = "default_k", alias = "k_cusum")]
    pub cusum_k: f64,
    /// Samples per window taken from the simulated runs.
    #[serde(default = "default_steps", alias = "amostras_por_janela")]
    pub samples_per_window: u32,
}

fn default_window() -> f64 {
    3600.0
}
fn default_reps() -> u32 {
    20
}
fn default_fdr() -> f64 {
    0.10
}
fn default_arl0() -> f64 {
    370.0
}
fn default_lambda() -> f64 {
    0.2
}
fn default_k() -> f64 {
    0.5
}
fn default_steps() -> u32 {
    60
}

impl Default for ShadowSettings {
    fn default() -> Self {
        ShadowSettings {
            window: default_window(),
            replications: default_reps(),
            warmup: 0.0,
            sim_warmup: None,
            fdr: default_fdr(),
            arl0: default_arl0(),
            ewma_lambda: default_lambda(),
            cusum_k: default_k(),
            samples_per_window: default_steps(),
        }
    }
}

/// Recording settings (used with `--gravar`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordSettings {
    /// Replace identifiers of people by stable nicknames (per recording).
    #[serde(default = "yes", alias = "anonimizar_pessoas")]
    pub anonymize_people: bool,
    /// Recordings older than this are deleted when a new one starts (and once a day).
    #[serde(default = "default_keep", alias = "retencao_dias")]
    pub keep_days: u32,
    /// Tags containing any of these texts are treated as people even when not mapped.
    #[serde(default, alias = "pessoas")]
    pub people: Vec<String>,
}

fn yes() -> bool {
    true
}
fn default_keep() -> u32 {
    30
}

impl Default for RecordSettings {
    fn default() -> Self {
        RecordSettings {
            anonymize_people: true,
            keep_days: default_keep(),
            people: Vec::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("{0}: {1}")]
    Io(String, std::io::Error),
    #[error("erro de sintaxe no arquivo do gêmeo: {0}")]
    Syntax(String),
    #[error("arquivo do gêmeo inválido:\n  - {}", .0.join("\n  - "))]
    Invalid(Vec<String>),
}

/// FNV-1a 64-bit hash of a model text (`fnv64:<16 hex>`): detects that the model
/// changed after the mapping was written. Not a security hash.
pub fn model_hash(text: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in text.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("fnv64:{h:016x}")
}

/// Length units accepted for positions, in metres.
fn length_unit(u: &str) -> Option<f64> {
    Some(match u {
        "m" => 1.0,
        "cm" => 0.01,
        "mm" => 0.001,
        "km" => 1000.0,
        _ => return None,
    })
}

impl TagMap {
    /// Factor of the unit (1 when absent).
    pub fn unit_factor(&self) -> f64 {
        match (&self.measure, self.unit.as_deref()) {
            (Measure::Position, Some(u)) => length_unit(u).unwrap_or(1.0),
            _ => 1.0,
        }
    }
}

impl TwinConfig {
    pub fn parse(text: &str) -> Result<TwinConfig, ConfigError> {
        ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(text)
            .map_err(|e| ConfigError::Syntax(e.to_string()))
    }

    pub fn load(path: impl AsRef<Path>) -> Result<(TwinConfig, String), ConfigError> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::Io(path.display().to_string(), e))?;
        Ok((TwinConfig::parse(&text)?, text))
    }

    /// The model file, resolved against the folder of the `.gemeo.ron` file.
    pub fn model_path(&self, config_path: &Path) -> PathBuf {
        let p = PathBuf::from(&self.model);
        if p.is_absolute() {
            p
        } else {
            config_path.parent().map(|d| d.join(&p)).unwrap_or(p)
        }
    }

    pub fn source(&self, name: &str) -> Option<&SourceDef> {
        self.sources.iter().find(|s| s.name == name)
    }

    /// Checks the file against the model. Errors stop the twin; the warnings are shown.
    pub fn validate(
        &self,
        model: &Model,
        model_text: Option<&str>,
    ) -> Result<Vec<String>, ConfigError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut names = BTreeSet::new();
        for s in &self.sources {
            if !names.insert(s.name.as_str()) {
                errors.push(format!("conexão '{}' declarada duas vezes", s.name));
            }
            match &s.kind {
                SourceKind::OpcUa {
                    endpoint,
                    security,
                    interval_ms,
                    ..
                } => {
                    if !endpoint.starts_with("opc.tcp://") {
                        errors.push(format!(
                            "conexão '{}': o endereço OPC UA começa com opc.tcp:// (está '{endpoint}')",
                            s.name
                        ));
                    }
                    if *interval_ms == 0 {
                        errors.push(format!(
                            "conexão '{}': interval_ms deve ser positivo",
                            s.name
                        ));
                    }
                    if *security == OpcSecurity::None {
                        warnings.push(format!(
                            "conexão '{}': OPC UA sem assinatura nem cifra (security: None); use SignAndEncrypt fora do laboratório",
                            s.name
                        ));
                    }
                }
                SourceKind::Mqtt { topics, .. } => {
                    if topics.is_empty() {
                        errors.push(format!("conexão '{}': nenhum tópico em topics", s.name));
                    }
                    warnings.push(format!(
                        "conexão '{}': MQTT sem TLS nesta versão; use só em rede de confiança",
                        s.name
                    ));
                }
                SourceKind::Sparkplug { group, .. } => {
                    if group.is_empty() || group.contains(['/', '+', '#']) {
                        errors.push(format!(
                            "conexão '{}': grupo Sparkplug inválido ('{group}')",
                            s.name
                        ));
                    }
                    warnings.push(format!(
                        "conexão '{}': MQTT sem TLS nesta versão; use só em rede de confiança",
                        s.name
                    ));
                }
            }
        }
        match &self.clock {
            ClockSpec::Wall { unit_seconds } => {
                if !(unit_seconds.is_finite() && *unit_seconds > 0.0) {
                    errors.push("clock: unit_seconds deve ser positivo".into());
                }
            }
            ClockSpec::Tag { source, .. } => {
                if self.source(source).is_none() {
                    errors.push(format!("clock: conexão '{source}' não existe"));
                }
            }
        }

        let mut seen_tags = BTreeSet::new();
        let mut totals = BTreeSet::new();
        let mut mapped_objects = BTreeSet::new();
        for (i, r) in self.tags.iter().enumerate() {
            let at = format!("tags[{i}] ({} → {})", r.tag, r.object);
            if self.source(&r.source).is_none() {
                errors.push(format!("{at}: conexão '{}' não existe", r.source));
            }
            if !seen_tags.insert((r.source.as_str(), r.tag.as_str())) {
                errors.push(format!(
                    "{at}: o mesmo tag aparece em duas regras (cada leitura vai para uma regra só)"
                ));
            }
            if !(r.scale.is_finite() && r.scale != 0.0) || !r.offset.is_finite() {
                errors.push(format!(
                    "{at}: escala e deslocamento devem ser finitos, escala ≠ 0"
                ));
            }
            match (r.measure, r.unit.as_deref()) {
                (_, None) => {}
                (Measure::Position, Some(u)) if length_unit(u).is_some() => {}
                (Measure::Position, Some(u)) => errors.push(format!(
                    "{at}: unidade '{u}' desconhecida para posição (m, cm, mm, km)"
                )),
                (_, Some("un")) => {}
                (m, Some(u)) => errors.push(format!(
                    "{at}: unidade '{u}' não se aplica a {} (só 'un')",
                    m.label()
                )),
            }
            if let (Some(a), Some(b)) = (r.min, r.max) {
                if a > b {
                    errors.push(format!("{at}: min maior que max"));
                }
            }
            for (name, v) in [
                ("rollover", r.rollover),
                ("deadband", r.deadband),
                ("debounce", r.debounce),
            ] {
                if let Some(v) = v {
                    if !(v.is_finite() && v > 0.0) {
                        errors.push(format!("{at}: {name} deve ser positivo"));
                    }
                }
            }
            if r.rollover.is_some() && !matches!(r.measure, Measure::Inputs | Measure::Outputs) {
                errors.push(format!("{at}: rollover só vale para contadores"));
            }
            if r.debounce.is_some() && r.measure != Measure::State {
                errors.push(format!("{at}: debounce só vale para estado"));
            }
            if r.axis.is_some() && r.measure != Measure::Position {
                errors.push(format!("{at}: axis só vale para posição"));
            }
            if !r.states.is_empty() && r.measure != Measure::State {
                errors.push(format!("{at}: states só vale para estado"));
            }
            if let Some(f) = &r.field {
                if !(f.is_empty() || f.starts_with('/')) {
                    errors.push(format!("{at}: field é um ponteiro JSON e começa com '/'"));
                }
            }
            if matches!(r.measure, Measure::Inputs | Measure::Outputs)
                && !totals.insert((r.object.as_str(), r.measure))
            {
                errors.push(format!(
                    "{at}: duas regras contam {} de '{}' (contagem dupla)",
                    r.measure.label(),
                    r.object
                ));
            }
            let Some(obj) = model.objects.iter().find(|o| o.name == r.object) else {
                errors.push(format!(
                    "{at}: o objeto '{}' não existe no modelo",
                    r.object
                ));
                continue;
            };
            mapped_objects.insert(r.object.as_str());
            match (&obj.kind, r.measure) {
                (ObjectKind::Sink, Measure::Outputs) => errors.push(format!(
                    "{at}: '{}' é uma saída do sistema (Sink) e não tem saídas",
                    r.object
                )),
                (ObjectKind::Source { .. }, Measure::Inputs) => errors.push(format!(
                    "{at}: '{}' é uma fonte (Source) e não tem entradas",
                    r.object
                )),
                (ObjectKind::Sink, Measure::State | Measure::Content) => warnings.push(format!(
                    "{at}: '{}' é um Sink; {} não diz muito sobre ele",
                    r.object,
                    r.measure.label()
                )),
                (ObjectKind::Transporter { .. }, _) => {}
                (_, Measure::Position) => warnings.push(format!(
                    "{at}: '{}' não é um recurso móvel; a posição só aparece no estado observado",
                    r.object
                )),
                _ => {}
            }
        }
        for o in &model.objects {
            if !mapped_objects.contains(o.name.as_str()) {
                warnings.push(format!(
                    "objeto '{}' sem nenhum tag (não observado)",
                    o.name
                ));
            }
        }
        if let ClockSpec::Tag { source, tag } = &self.clock {
            if self
                .tags
                .iter()
                .any(|r| r.source == *source && r.tag == *tag)
            {
                errors.push(format!(
                    "clock: o tag '{tag}' do relógio também está no mapeamento"
                ));
            }
        }
        let s = &self.shadow;
        if !(s.window.is_finite() && s.window > 0.0) {
            errors.push("shadow.window deve ser positivo".into());
        }
        if s.replications < 2 {
            errors.push("shadow.replications deve ser pelo menos 2".into());
        }
        if !(s.warmup.is_finite() && s.warmup >= 0.0) {
            errors.push("shadow.warmup deve ser ≥ 0".into());
        }
        if !(s.fdr > 0.0 && s.fdr < 1.0) {
            errors.push("shadow.fdr deve ficar entre 0 e 1".into());
        }
        if !(s.arl0.is_finite() && s.arl0 > 1.0) {
            errors.push("shadow.arl0 deve ser maior que 1".into());
        }
        if !(s.ewma_lambda > 0.0 && s.ewma_lambda <= 1.0) {
            errors.push("shadow.ewma_lambda deve ficar em (0, 1]".into());
        }
        if !(s.cusum_k.is_finite() && s.cusum_k >= 0.0) {
            errors.push("shadow.cusum_k deve ser ≥ 0".into());
        }
        if s.samples_per_window == 0 {
            errors.push("shadow.samples_per_window deve ser positivo".into());
        }
        let sim_warmup = s.sim_warmup.unwrap_or(model.run.warmup);
        if s.window.is_finite() && model.run.stop_time - sim_warmup < s.window {
            errors.push(format!(
                "a rodada do modelo ({} a {}) não cabe numa janela de {}: aumente run.stop_time ou diminua shadow.window",
                sim_warmup, model.run.stop_time, s.window
            ));
        }
        if self.record.keep_days == 0 {
            errors.push("record.keep_days deve ser pelo menos 1".into());
        }
        if self.tags.is_empty() {
            errors.push("nenhum tag mapeado".into());
        }
        if let (Some(want), Some(text)) = (&self.model_hash, model_text) {
            let got = model_hash(text);
            if *want != got {
                warnings.push(format!(
                    "o modelo mudou depois do mapeamento (model_hash {want}, atual {got}); confira os nomes dos objetos"
                ));
            }
        }
        if errors.is_empty() {
            Ok(warnings)
        } else {
            Err(ConfigError::Invalid(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example of the manual parses (and keeps its defaults).
    #[test]
    fn manual_example_parses() {
        let page = include_str!("../../../docs/manual/src/gemeo-digital.md");
        let start = page.find("```ron\nTwin(").expect("exemplo") + "```ron\n".len();
        let end = start + page[start..].find("```").unwrap();
        let cfg = TwinConfig::parse(&page[start..end]).unwrap();
        assert_eq!(cfg.sources.len(), 3);
        assert_eq!(cfg.tags[0].rollover, Some(65536.0));
        assert_eq!(cfg.tags[1].states["3"], StateClass::Down);
        assert_eq!(cfg.tags[3].unit_factor(), 0.001);
        assert!(cfg.tags[4].person);
        assert_eq!(cfg.shadow.fdr, 0.10);
        assert_eq!(cfg.record.keep_days, 30);
    }

    #[test]
    fn hash_is_stable() {
        assert_eq!(model_hash(""), "fnv64:cbf29ce484222325");
        assert_ne!(model_hash("a"), model_hash("b"));
    }
}
