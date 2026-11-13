//! The properties of one object as a schema the Godot panel draws generically, and the
//! edit of one property as a [`Command`] of `utsim-edit` (so it is undoable and validated
//! by the document like any other edit).
//!
//! A schema is a flat list of [`Field`]s. Each field has a `path` inside the object
//! (`process`, `transfer.speed`, `downtime.mtbf.mean`, `visual.color`), a label in
//! Portuguese, a [`FieldType`], the current [`Value`], and, when it applies, choices,
//! a unit, a help text, a minimum and the nesting depth. An optional field (`setup`,
//! `transfer`, `batch`) carries `optional` and `enabled`; switching it on or off is the
//! path plus `.on`. A distribution carries a [`DistView`] with its family, parameters
//! and RON text, so the panel offers "family + parameters" with "editar como texto" as
//! the fallback.
//!
//! [`edit`] turns `(object, path, value)` into the command:
//! - `name` is a `Rename`, `position.x|y|z` a `Move`, `visual.*` a `SetVisual`;
//! - everything else is a `Set` of `<object>.<field>` with the RON text of the whole
//!   field, written from a typed copy of the object (so changing `transfer.speed` keeps
//!   the other parts of `transfer`);
//! - a text value on a field that is not text is the RON of that field (the fallback),
//!   and `<dist>.text` replaces a distribution with the RON given.
//!
//! The field list and the reading of values follow the Bevy editor
//! (`utsim_viewer::properties::spec`), which this module copies because the viewer pulls
//! in Bevy. The copy goes away with the shared crate after the viewer's polish round.

use std::f64::consts::PI;

use utsim_core::{Dist, OffWindow, OutputRule, ShiftEnd, ShiftTable};
use utsim_edit::Command;
use utsim_model::{
    Model, Movement, ObjectDef, ObjectKind, Profile, RunSettings, ShiftSpec, VisualDef, is_mobile,
};

use crate::catalogue;
use crate::looks;
use crate::ron_text as ron;

/// A value going to or coming from the panel (converted from and to a Godot `Variant`).
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<String>),
}

impl Value {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) if f.is_finite() => Some(*f),
            Value::Text(t) => t.trim().replace(',', ".").parse().ok(),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Float(f) if f.is_finite() => Some(f.round() as i64),
            Value::Text(t) => t.trim().parse().ok(),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            Value::Int(i) => Some(*i != 0),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(t) => Some(t),
            _ => None,
        }
    }
}

/// How a field is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    /// One line of text (the name, a label).
    Text,
    /// Several lines of code (output and entry scripts).
    Script,
    Float,
    Int,
    Bool,
    /// One of `choices` (the value is the choice's `value`).
    Choice,
    /// A distribution: see [`Field::dist`].
    Dist,
    /// RON text of a value the panel has no widget for (lists, maps).
    Ron,
    /// Several of `choices` (the value is the list of the chosen ones); toggling one is
    /// the path plus `.<choice>` with a boolean.
    Set,
    /// `#rrggbb`.
    Color,
    /// An asset of the library or of the third-party catalogue (`choices`).
    Asset,
    /// A button (`visual.reset`); the value says whether it is enabled.
    Action,
    /// A title for the fields below it (`batch`, `transfer`, `downtime`), usually optional.
    Group,
    /// Read-only text.
    Info,
}

impl FieldType {
    pub fn name(self) -> &'static str {
        match self {
            FieldType::Text => "text",
            FieldType::Script => "script",
            FieldType::Float => "float",
            FieldType::Int => "int",
            FieldType::Bool => "bool",
            FieldType::Choice => "choice",
            FieldType::Dist => "dist",
            FieldType::Ron => "ron",
            FieldType::Set => "set",
            FieldType::Color => "color",
            FieldType::Asset => "asset",
            FieldType::Action => "action",
            FieldType::Group => "group",
            FieldType::Info => "info",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Choice {
    pub value: String,
    pub label: String,
    /// Tooltip (asset credits, what an option does).
    pub help: String,
    /// Heading the choice is listed under (`Biblioteca do UTSim`, `Terceiros`).
    pub group: String,
}

fn choice(value: &str, label: &str) -> Choice {
    Choice {
        value: value.into(),
        label: label.into(),
        help: String::new(),
        group: String::new(),
    }
}

/// One parameter of a distribution.
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub key: &'static str,
    pub label: &'static str,
    pub value: f64,
    pub integer: bool,
}

/// A distribution as the panel edits it.
#[derive(Clone, Debug, PartialEq)]
pub struct DistView {
    /// Index into [`FAMILIES`].
    pub family: usize,
    /// Parameters with a number each; empty for tables (`Discrete`, `Empirical`), which
    /// are edited as text.
    pub params: Vec<Param>,
    /// RON text of the whole distribution ("editar como texto").
    pub text: String,
    pub mean: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub path: String,
    pub label: String,
    pub ty: FieldType,
    pub value: Value,
    pub choices: Vec<Choice>,
    pub unit: &'static str,
    pub help: String,
    /// Heading the field is listed under (`Objeto`, `Posição`, `Parâmetros`, `Saída`,
    /// `Quebras e turnos`, `Aparência`, `Simulação`).
    pub section: &'static str,
    /// Nesting under a group (0 = top level).
    pub depth: u8,
    /// Has an on/off switch (`path.on`).
    pub optional: bool,
    /// Switched on (always true when not optional).
    pub enabled: bool,
    pub min: Option<f64>,
    pub dist: Option<DistView>,
}

impl Field {
    fn new(section: &'static str, path: &str, label: &str, ty: FieldType, value: Value) -> Field {
        Field {
            path: path.into(),
            label: label.into(),
            ty,
            value,
            choices: Vec::new(),
            unit: "",
            help: String::new(),
            section,
            depth: 0,
            optional: false,
            enabled: true,
            min: None,
            dist: None,
        }
    }

    fn unit(mut self, unit: &'static str) -> Field {
        self.unit = unit;
        self
    }

    fn help(mut self, help: &str) -> Field {
        self.help = help.into();
        self
    }

    fn min(mut self, min: f64) -> Field {
        self.min = Some(min);
        self
    }

    fn depth(mut self, depth: u8) -> Field {
        self.depth = depth;
        self
    }

    fn optional(mut self, enabled: bool) -> Field {
        self.optional = true;
        self.enabled = enabled;
        self
    }

    fn choices(mut self, choices: Vec<Choice>) -> Field {
        self.choices = choices;
        self
    }
}

pub const S_OBJECT: &str = "Objeto";
pub const S_POSITION: &str = "Posição";
pub const S_PARAMS: &str = "Parâmetros";
pub const S_OUTPUT: &str = "Saída";
pub const S_DOWNTIME: &str = "Quebras e turnos";
pub const S_LOOK: &str = "Aparência";
pub const S_RUN: &str = "Simulação";
pub const S_MOVE: &str = "Movimentação";
pub const S_FLOOR: &str = "Piso e corredores";
pub const S_MODEL: &str = "Modelo";
pub const S_PERSON: &str = "Pessoa";

/// Portuguese name of an object kind.
pub fn kind_label(kind: &ObjectKind) -> &'static str {
    match kind {
        ObjectKind::Source { .. } => "Chegada",
        ObjectKind::Queue { .. } => "Espera",
        ObjectKind::Processor { .. } => "Processo",
        ObjectKind::Combiner { .. } => "Montagem",
        ObjectKind::Separator { .. } => "Desmontagem",
        ObjectKind::Sink => "Saída",
        ObjectKind::Transporter { .. } => "Transportador",
        ObjectKind::Dispatcher { .. } => "Despachante",
        ObjectKind::Conveyor { .. } => "Esteira",
        ObjectKind::Rack { .. } => "Porta-paletes",
    }
}

/// Portuguese names of the distribution families, in menu order (the Bevy editor's).
pub const FAMILIES: [&str; 20] = [
    "Constante",
    "Uniforme",
    "Exponencial",
    "Triangular",
    "Normal",
    "Lognormal",
    "Gama",
    "Erlang",
    "Weibull",
    "Beta",
    "Log-logística",
    "Pearson V",
    "Pearson VI",
    "Discreta",
    "Empírica",
    "Poisson",
    "Geométrica",
    "Binomial",
    "Binomial negativa",
    "Deslocada",
];

pub fn family_index(d: &Dist) -> usize {
    match d {
        Dist::Const(_) => 0,
        Dist::Uniform { .. } => 1,
        Dist::Exp { .. } => 2,
        Dist::Triangular { .. } => 3,
        Dist::Normal { .. } => 4,
        Dist::LogNormal { .. } => 5,
        Dist::Gamma { .. } => 6,
        Dist::Erlang { .. } => 7,
        Dist::Weibull { .. } => 8,
        Dist::Beta { .. } => 9,
        Dist::LogLogistic { .. } => 10,
        Dist::PearsonV { .. } => 11,
        Dist::PearsonVI { .. } => 12,
        Dist::Discrete(_) => 13,
        Dist::Empirical(_) => 14,
        Dist::Poisson { .. } => 15,
        Dist::Geometric { .. } => 16,
        Dist::Binomial { .. } => 17,
        Dist::NegBinomial { .. } => 18,
        Dist::Shifted { .. } => 19,
    }
}

/// A distribution of family `index` with sensible parameters and mean `m` (> 0).
pub fn family_default(index: usize, m: f64) -> Dist {
    match index {
        0 => Dist::Const(m),
        1 => Dist::Uniform {
            min: 0.5 * m,
            max: 1.5 * m,
        },
        2 => Dist::Exp { mean: m },
        3 => Dist::Triangular {
            min: 0.5 * m,
            mode: m,
            max: 1.5 * m,
        },
        4 => Dist::Normal {
            mean: m,
            sd: 0.2 * m,
        },
        5 => Dist::LogNormal {
            mean: m,
            sd: 0.5 * m,
        },
        6 => Dist::Gamma {
            shape: 2.0,
            scale: m / 2.0,
        },
        7 => Dist::Erlang { k: 2, mean: m },
        8 => Dist::Weibull {
            shape: 2.0,
            // Mean of Weibull(2, s) is s * gamma(1.5) = s * sqrt(pi) / 2.
            scale: m / (PI.sqrt() / 2.0),
        },
        9 => Dist::Beta {
            alpha: 2.0,
            beta: 2.0,
            min: 0.0,
            max: 2.0 * m,
        },
        10 => {
            // Mean of LogLogistic(3, s) is s * (pi/3) / sin(pi/3).
            let x = PI / 3.0;
            Dist::LogLogistic {
                shape: 3.0,
                scale: m * x.sin() / x,
            }
        }
        11 => Dist::PearsonV {
            shape: 3.0,
            scale: 2.0 * m,
        },
        12 => Dist::PearsonVI {
            shape1: 2.0,
            shape2: 3.0,
            scale: m,
        },
        13 => Dist::Discrete(vec![(0.5 * m, 1.0), (1.5 * m, 1.0)]),
        14 => Dist::Empirical(vec![0.5 * m, m, 1.5 * m]),
        15 => Dist::Poisson { mean: m },
        16 => Dist::Geometric { p: 1.0 / (1.0 + m) },
        17 => {
            let n = (2.0 * m).ceil().clamp(1.0, f64::from(u32::MAX)) as u32;
            Dist::Binomial {
                n,
                p: (m / f64::from(n)).min(1.0),
            }
        }
        18 => Dist::NegBinomial {
            r: 2.0,
            p: 2.0 / (m + 2.0),
        },
        _ => Dist::Shifted {
            offset: 0.0,
            dist: Box::new(Dist::Exp { mean: m }),
        },
    }
}

/// Switches `current` to family `index`, keeping roughly the same mean when possible.
/// Switching to `Deslocada` wraps the current distribution, which keeps it exactly.
pub fn switch_family(current: &Dist, index: usize) -> Dist {
    if index == 19 {
        return Dist::Shifted {
            offset: 0.0,
            dist: Box::new(current.clone()),
        };
    }
    let mean = current.mean();
    let m = if mean.is_finite() && mean > 0.0 {
        mean
    } else {
        1.0
    };
    family_default(index, m)
}

/// A numeric parameter of a distribution, borrowed for editing.
enum ParamRef<'a> {
    F(&'a mut f64),
    U(&'a mut u32),
}

fn params_mut(d: &mut Dist) -> Vec<(&'static str, &'static str, ParamRef<'_>)> {
    use ParamRef::{F, U};
    match d {
        Dist::Const(v) => vec![("value", "valor", F(v))],
        Dist::Uniform { min, max } => vec![("min", "mínimo", F(min)), ("max", "máximo", F(max))],
        Dist::Exp { mean } => vec![("mean", "média", F(mean))],
        Dist::Triangular { min, mode, max } => vec![
            ("min", "mínimo", F(min)),
            ("mode", "moda", F(mode)),
            ("max", "máximo", F(max)),
        ],
        Dist::Normal { mean, sd } | Dist::LogNormal { mean, sd } => {
            vec![("mean", "média", F(mean)), ("sd", "desvio padrão", F(sd))]
        }
        Dist::Gamma { shape, scale }
        | Dist::Weibull { shape, scale }
        | Dist::LogLogistic { shape, scale }
        | Dist::PearsonV { shape, scale } => {
            vec![("shape", "forma", F(shape)), ("scale", "escala", F(scale))]
        }
        Dist::Erlang { k, mean } => vec![("k", "fases (k)", U(k)), ("mean", "média", F(mean))],
        Dist::Beta {
            alpha,
            beta,
            min,
            max,
        } => vec![
            ("alpha", "alfa", F(alpha)),
            ("beta", "beta", F(beta)),
            ("min", "mínimo", F(min)),
            ("max", "máximo", F(max)),
        ],
        Dist::PearsonVI {
            shape1,
            shape2,
            scale,
        } => vec![
            ("shape1", "forma 1", F(shape1)),
            ("shape2", "forma 2", F(shape2)),
            ("scale", "escala", F(scale)),
        ],
        Dist::Poisson { mean } => vec![("mean", "média", F(mean))],
        Dist::Geometric { p } => vec![("p", "probabilidade (p)", F(p))],
        Dist::Binomial { n, p } => vec![
            ("n", "tentativas (n)", U(n)),
            ("p", "probabilidade (p)", F(p)),
        ],
        Dist::NegBinomial { r, p } => {
            vec![
                ("r", "sucessos (r)", F(r)),
                ("p", "probabilidade (p)", F(p)),
            ]
        }
        Dist::Shifted { offset, .. } => vec![("offset", "deslocamento", F(offset))],
        Dist::Discrete(_) | Dist::Empirical(_) => Vec::new(),
    }
}

pub fn dist_view(d: &Dist) -> DistView {
    let mut copy = d.clone();
    let params = params_mut(&mut copy)
        .into_iter()
        .map(|(key, label, r)| match r {
            ParamRef::F(v) => Param {
                key,
                label,
                value: *v,
                integer: false,
            },
            ParamRef::U(v) => Param {
                key,
                label,
                value: f64::from(*v),
                integer: true,
            },
        })
        .collect();
    DistView {
        family: family_index(d),
        params,
        text: ron::dist(d),
        mean: d.mean(),
    }
}

fn parse_dist(text: &str) -> Result<Dist, String> {
    let d: Dist = ron::parse(text).map_err(|e| format!("distribuição inválida: {e}"))?;
    d.validate()?;
    Ok(d)
}

/// Edits `d` at `sub`: `family` (index), a parameter key (number), `inner` (RON of the
/// distribution a `Deslocada` wraps), or `""`/`text` (RON of the whole distribution).
fn edit_dist(d: &mut Dist, sub: &str, v: &Value) -> Result<(), String> {
    match sub {
        "family" => {
            let i = v.as_int().ok_or("família inválida")?;
            let i = usize::try_from(i)
                .ok()
                .filter(|i| *i < FAMILIES.len())
                .ok_or("família inválida")?;
            *d = switch_family(d, i);
            Ok(())
        }
        "" | "text" => {
            let t = v.as_text().ok_or("texto RON esperado")?;
            *d = parse_dist(t)?;
            Ok(())
        }
        "inner" => {
            let Dist::Shifted { dist, .. } = d else {
                return Err("só a distribuição deslocada tem uma distribuição interna".into());
            };
            let t = v.as_text().ok_or("texto RON esperado")?;
            **dist = parse_dist(t)?;
            Ok(())
        }
        key => {
            let x = v.as_f64().ok_or("número esperado")?;
            for (k, _, r) in params_mut(d) {
                if k == key {
                    match r {
                        ParamRef::F(p) => *p = x,
                        ParamRef::U(p) => {
                            if !(0.0..=f64::from(u32::MAX)).contains(&x.round()) {
                                return Err(format!("{key} precisa ser um inteiro >= 0"));
                            }
                            *p = x.round() as u32;
                        }
                    }
                    return Ok(());
                }
            }
            Err(format!("a distribuição não tem o parâmetro \"{key}\""))
        }
    }
}

fn dist_field(section: &'static str, path: &str, label: &str, d: &Dist) -> Field {
    let mut f = Field::new(
        section,
        path,
        label,
        FieldType::Dist,
        Value::Text(ron::dist(d)),
    );
    f.dist = Some(dist_view(d));
    f
}

fn opt_dist_field(section: &'static str, path: &str, label: &str, d: Option<&Dist>) -> Field {
    match d {
        Some(d) => dist_field(section, path, label, d).optional(true),
        None => Field::new(section, path, label, FieldType::Dist, Value::Null).optional(false),
    }
}

fn float(section: &'static str, path: &str, label: &str, v: f64) -> Field {
    Field::new(section, path, label, FieldType::Float, Value::Float(v))
}

fn int(section: &'static str, path: &str, label: &str, v: u64) -> Field {
    Field::new(section, path, label, FieldType::Int, Value::Int(v as i64))
}

fn boolean(section: &'static str, path: &str, label: &str, v: bool) -> Field {
    Field::new(section, path, label, FieldType::Bool, Value::Bool(v))
}

fn pick(
    section: &'static str,
    path: &str,
    label: &str,
    v: &str,
    choices: &[(&str, &str)],
) -> Field {
    Field::new(
        section,
        path,
        label,
        FieldType::Choice,
        Value::Text(v.into()),
    )
    .choices(choices.iter().map(|(v, l)| choice(v, l)).collect())
}

fn group(section: &'static str, path: &str, label: &str, on: bool) -> Field {
    Field::new(section, path, label, FieldType::Group, Value::Bool(on)).optional(on)
}

/// Names of every Transporter or Dispatcher (what `transport` and `operators.team` name).
pub fn mobile_names(model: &Model) -> Vec<String> {
    model
        .objects
        .iter()
        .filter(|o| is_mobile(&o.kind))
        .map(|o| o.name.clone())
        .collect()
}

fn transporter_names(model: &Model) -> Vec<String> {
    model
        .objects
        .iter()
        .filter(|o| matches!(o.kind, ObjectKind::Transporter { .. }))
        .map(|o| o.name.clone())
        .collect()
}

/// Number of outgoing connections of `name` (the ports of its output rule).
pub fn outputs(model: &Model, name: &str) -> usize {
    model.connections.iter().filter(|(a, _)| a == name).count()
}

/// Output rule ids and labels, in menu order.
pub const OUTPUT_RULES: [(&str, &str); 6] = [
    ("FirstAvailable", "Primeira livre"),
    ("RoundRobin", "Rodízio"),
    ("Random", "Aleatória"),
    ("Weighted", "Pesos"),
    ("ByLabel", "Por rótulo"),
    ("Script", "Script"),
];

fn output_id(rule: &OutputRule) -> &'static str {
    match rule {
        OutputRule::FirstAvailable => "FirstAvailable",
        OutputRule::RoundRobin => "RoundRobin",
        OutputRule::Random => "Random",
        OutputRule::Weighted(_) => "Weighted",
        OutputRule::ByLabel(_) => "ByLabel",
        OutputRule::Script(_) => "Script",
    }
}

/// A label name defined by some Source, as a starting point for `ByLabel`.
fn first_label(model: &Model) -> String {
    model
        .objects
        .iter()
        .find_map(|o| match &o.kind {
            ObjectKind::Source { labels, .. } => labels.keys().next().cloned(),
            _ => None,
        })
        .unwrap_or_else(|| "tipo".into())
}

/// The movement mode choices; the first is "the model's" (stored as no value).
const MOVEMENT_CHOICES: [(&str, &str); 5] = [
    ("", "Padrão do modelo"),
    ("Legacy", "Legado (linha reta ou rede)"),
    ("Free", "Livre (menor caminho no piso)"),
    ("Network", "Pela rede de caminhos"),
    ("Aisles", "Pelos corredores"),
];

fn movement_name(m: Option<Movement>) -> &'static str {
    match m {
        None => "",
        Some(Movement::Legacy) => "Legacy",
        Some(Movement::Free) => "Free",
        Some(Movement::Network) => "Network",
        Some(Movement::Aisles) => "Aisles",
    }
}

fn movement_from(name: &str) -> Result<Option<Movement>, String> {
    Ok(match name {
        "" => None,
        "Legacy" => Some(Movement::Legacy),
        "Free" => Some(Movement::Free),
        "Network" => Some(Movement::Network),
        "Aisles" => Some(Movement::Aisles),
        _ => return Err(format!("modo de movimentação desconhecido: {name}")),
    })
}

/// How a mobile resource moves: its own mode (or the model's) and its clearance.
fn movement_fields(
    model: &Model,
    obj: &ObjectDef,
    movement: Option<Movement>,
    clearance: Option<f64>,
) -> Vec<Field> {
    let s = S_MOVE;
    // The clearance of its role: the one it has with no clearance of its own.
    let mut plain = obj.clone();
    if let ObjectKind::Transporter { clearance, .. } = &mut plain.kind {
        *clearance = None;
    }
    let by_role = model.clearance_of(&plain);
    vec![
        pick(
            s,
            "movement",
            "Movimentação",
            movement_name(movement),
            &MOVEMENT_CHOICES,
        )
        .help(
            "Como o recurso anda: o padrão do modelo (Simulação), em linha reta como no \
             formato antigo, livre pelo piso desviando dos obstáculos, pela rede de \
             caminhos ou só pelos corredores da sua classe.",
        ),
        Field::new(
            s,
            "clearance",
            "Folga dos obstáculos",
            FieldType::Float,
            clearance.map_or(Value::Null, Value::Float),
        )
        .optional(clearance.is_some())
        .min(0.0)
        .unit("m")
        .help(&format!(
            "Distância mínima de qualquer obstáculo. Desligada, vale a do papel do recurso \
             ({} m: meia largura mais 0,3 m).",
            format!("{by_role:.2}").replace('.', ",")
        )),
    ]
}

/// The access point of a fixed object (`access`), in its own coordinates.
fn access_fields(obj: &ObjectDef) -> Vec<Field> {
    let s = S_MOVE;
    let mut out = vec![
        group(s, "access", "Ponto de acesso próprio", obj.access.is_some()).help(
            "Onde os recursos móveis param para atender este objeto, em metros a partir do \
             seu centro e girando com ele. Desligado, é o lado -y. Também se arrasta na \
             cena, com \"Mostrar obstáculos e folgas\" ligado.",
        ),
    ];
    if let Some(a) = obj.access {
        out.push(float(s, "access.x", "X no objeto", a[0]).depth(1).unit("m"));
        out.push(float(s, "access.y", "Y no objeto", a[1]).depth(1).unit("m"));
    }
    out
}

/// A length in metres with a decimal comma (`1,80`).
fn metres(v: f64) -> String {
    format!("{v:.2}").replace('.', ",")
}

/// Height an object gets when its dimensions are turned on: the real one of its catalogue
/// asset, else a usual one for its kind.
fn default_height(obj: &ObjectDef) -> f64 {
    let visual = obj.visual.as_ref();
    let asset = visual.and_then(|v| v.asset.as_deref());
    let scale = visual.and_then(|v| v.scale).unwrap_or(1.0);
    // A real waiting area: the height of its element (a table, a shelf...).
    if let Some(w) = obj.waiting_area {
        return w.default_size()[2];
    }
    if let Some(e) = asset.and_then(|a| catalogue::entries().iter().find(|e| e.nome == a)) {
        return f64::from(e.tamanho_real_m.1) * scale;
    }
    match obj.kind {
        ObjectKind::Queue { .. } => 0.3,
        ObjectKind::Processor { .. } => 1.8,
        ObjectKind::Combiner { .. } | ObjectKind::Separator { .. } => 1.5,
        _ => 1.0,
    }
}

/// Dimensions of a fixed object (`size`) and, in a queue, its capacity by area
/// (`area_per_item`). A belt and a rack show theirs, which come from their fields.
fn size_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    let s = S_POSITION;
    let info = |text: String| {
        Field::new(
            s,
            "size.info",
            "Dimensões",
            FieldType::Info,
            Value::Text(text),
        )
    };
    let [l, w] = model.footprint(obj).unwrap_or([0.0, 0.0]);
    match &obj.kind {
        ObjectKind::Conveyor { .. } => {
            return vec![info(format!(
                "{} × {} m: o comprimento é o da esteira (Parâmetros)",
                metres(l),
                metres(w)
            ))];
        }
        ObjectKind::Rack {
            levels,
            level_height,
            ..
        } => {
            return vec![info(format!(
                "{} × {} × {} m: pelos vãos e níveis (Parâmetros)",
                metres(l),
                metres(w),
                metres(*levels as f64 * level_height)
            ))];
        }
        _ => {}
    }
    let mut out = vec![group(s, "size", "Dimensões próprias", obj.size.is_some()).help(
        "Comprimento, largura e altura do objeto em metros, no sentido dele. A área no piso muda o ponto de acesso e, com ele, as distâncias e os tempos dos operadores e veículos; numa espera com área por item, também a capacidade. O 3D estica o modelo para essas medidas. Desligado, vale a área padrão do tipo.",
    )];
    match obj.size {
        Some(d) => {
            for (i, (axis, label)) in [("x", "Comprimento"), ("y", "Largura"), ("z", "Altura")]
                .iter()
                .enumerate()
            {
                out.push(
                    float(s, &format!("size.{axis}"), label, d[i])
                        .depth(1)
                        .min(0.01)
                        .unit("m"),
                );
            }
        }
        None => out.push(info(format!("{} × {} m (padrão)", metres(l), metres(w)))),
    }
    if matches!(obj.kind, ObjectKind::Queue { .. }) {
        out.push(
            group(s, "area_per_item", "Capacidade pela área", obj.area_per_item.is_some())
                .help(
                    "A capacidade da espera sai da sua área no piso dividida pela área que cada item ocupa (só itens inteiros). Com uma capacidade dada também, vale a menor.",
                ),
        );
        if let Some(a) = obj.area_per_item {
            out.push(
                float(s, "area_per_item", "Área por item", a)
                    .depth(1)
                    .min(0.01)
                    .unit("m²"),
            );
            let fits = model
                .queue_capacity(obj)
                .map_or("sem limite".to_string(), |n| format!("{n} itens"));
            out.push(
                Field::new(
                    s,
                    "area_per_item.info",
                    "Cabem",
                    FieldType::Info,
                    Value::Text(fits),
                )
                .depth(1),
            );
        }
    }
    out
}

/// The edit of the dimension fields (`size`, `area_per_item`), or `None` for others.
fn edit_size(
    model: &Model,
    obj: &ObjectDef,
    head: &str,
    rest: &str,
    value: &Value,
) -> Option<Result<Command, String>> {
    let name = obj.name.clone();
    let start = || {
        let [l, w] = model.footprint(obj).unwrap_or([1.0, 1.0]);
        [l, w, default_height(obj)]
    };
    Some(match (head, rest) {
        ("size", "on") => bool_of(value, "size").map(|on| Command::SetSize {
            size: on.then(start),
            name,
        }),
        ("size", "x" | "y" | "z") => {
            let mut d = obj.size.unwrap_or_else(start);
            let k = match rest {
                "x" => 0,
                "y" => 1,
                _ => 2,
            };
            f64_of(value, "dimensão").map(|v| {
                d[k] = v;
                Command::SetSize {
                    name,
                    size: Some(d),
                }
            })
        }
        ("area_per_item", "on") => bool_of(value, "area_per_item").map(|on| {
            // One item per square metre, to start from.
            Command::SetAreaPerItem {
                area: on.then_some(1.0),
                name,
            }
        }),
        ("area_per_item", "") => f64_of(value, "área por item").map(|a| Command::SetAreaPerItem {
            name,
            area: Some(a),
        }),
        _ => return None,
    })
}

/// The edit of the movement fields (`movement`, `clearance`, `access`), or `None` for
/// other fields.
fn edit_movement(
    model: &Model,
    obj: &ObjectDef,
    head: &str,
    rest: &str,
    value: &Value,
) -> Option<Result<Command, String>> {
    let name = obj.name.clone();
    let mobile = matches!(obj.kind, ObjectKind::Transporter { .. });
    Some(match (head, rest) {
        ("movement", "") if mobile => value
            .as_text()
            .ok_or_else(|| "modo de movimentação esperado".to_string())
            .and_then(movement_from)
            .map(|movement| Command::SetMovement { name, movement }),
        ("clearance", "on") if mobile => {
            bool_of(value, "clearance").map(|on| Command::SetClearance {
                clearance: on.then(|| model.clearance_of(obj)),
                name,
            })
        }
        ("clearance", "") if mobile => match value {
            Value::Null => Ok(Command::SetClearance {
                name,
                clearance: None,
            }),
            v => f64_of(v, "folga").map(|c| Command::SetClearance {
                name,
                clearance: Some(c),
            }),
        },
        ("access", "on") if !mobile => bool_of(value, "access").map(|on| Command::SetAccess {
            // A point two metres in front of the object, on its -y side, to start from.
            access: on.then(|| [0.0, -2.0]),
            name,
        }),
        ("access", "x" | "y") if !mobile => {
            let mut a = obj.access.unwrap_or([0.0, -2.0]);
            f64_of(value, "ponto de acesso").map(|v| {
                a[usize::from(rest == "y")] = v;
                Command::SetAccess {
                    name,
                    access: Some(a),
                }
            })
        }
        _ => return None,
    })
}

fn kind_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    let s = S_PARAMS;
    let mut out = Vec::new();
    match &obj.kind {
        ObjectKind::Source {
            interarrival,
            first_at_zero,
            max_arrivals,
            labels,
            on_blocked,
            schedule,
            table,
            ..
        } => {
            out.push(
                dist_field(s, "interarrival", "Tempo entre chegadas", interarrival)
                    .help("Intervalo entre duas chegadas seguidas."),
            );
            out.push(boolean(
                s,
                "first_at_zero",
                "Primeira chegada em t = 0",
                *first_at_zero,
            ));
            out.push(
                Field::new(
                    s,
                    "max_arrivals",
                    "Máximo de chegadas",
                    FieldType::Int,
                    max_arrivals.map_or(Value::Null, |v| Value::Int(v as i64)),
                )
                .optional(max_arrivals.is_some())
                .min(1.0)
                .unit("itens"),
            );
            out.push(pick(
                s,
                "on_blocked",
                "Se a saída estiver bloqueada",
                &ron::debug(on_blocked),
                &[("Wait", "Esperar"), ("Drop", "Descartar (perda)")],
            ));
            out.push(
                Field::new(s, "labels", "Rótulos", FieldType::Ron, Value::Text(ron::labels(labels)))
                    .help("Valor sorteado para cada item, por nome: {\"tipo\": Discrete([(1.0, 3.0), (2.0, 1.0)])}."),
            );
            out.push(
                Field::new(
                    s,
                    "schedule",
                    "Chegadas por período",
                    FieldType::Ron,
                    schedule
                        .as_ref()
                        .map_or(Value::Null, |v| Value::Text(ron::schedule(v))),
                )
                .optional(schedule.is_some())
                .help(
                    "Ciclo e períodos com o próprio intervalo, taxa ou fechados: \
                     (cycle: 1440.0, periods: [(start: 0.0, end: 480.0, arrivals: Rate(0.5))]).",
                ),
            );
            // The choices are the model's item types (the table editor's "tipo" column).
            out.push(
                Field::new(
                    s,
                    "table",
                    "Tabela de chegadas",
                    FieldType::Ron,
                    table
                        .as_ref()
                        .map_or(Value::Null, |v| Value::Text(ron::table(v))),
                )
                .optional(table.is_some())
                .choices(
                    model
                        .item_types
                        .iter()
                        .map(|t| choice(&t.name, &t.name))
                        .collect(),
                )
                .help(
                    "Horário (instante, quantidade, tipo e rótulos de cada linha) ou sequência \
                     de linhas: (mode: Times, repeat: Cycle(1440.0), rows: [(at: 480.0, count: 12)]).",
                ),
            );
        }
        ObjectKind::Queue {
            capacity,
            discipline,
            batch,
            order,
            ..
        } => {
            // First: the kind of waiting area (a real one has places for the items).
            out.extend(waiting_fields(s, obj));
            out.push(
                Field::new(
                    s,
                    "capacity",
                    "Capacidade limitada",
                    FieldType::Int,
                    capacity.map_or(Value::Null, |v| Value::Int(v as i64)),
                )
                .optional(capacity.is_some())
                .min(1.0)
                .unit("itens")
                .help("Desligada, a espera não tem limite."),
            );
            out.push(
                pick(
                    s,
                    "discipline",
                    "Política de acesso",
                    &access_policy(*discipline, order.as_ref()),
                    &[
                        ("Fifo", "FIFO (primeiro a entrar, primeiro a sair)"),
                        ("Lifo", "LIFO (último a entrar, primeiro a sair)"),
                        ("Desc", "Prioridade: maior rótulo primeiro"),
                        ("Asc", "Prioridade: menor rótulo primeiro (prazo)"),
                    ],
                )
                .help("Qual item sai primeiro. Pela prioridade, sai antes o item com o maior (ou o menor) valor do rótulo; empates e itens sem o rótulo saem na ordem de chegada."),
            );
            if let Some(o) = order {
                out.push(
                    Field::new(
                        s,
                        "order.label",
                        "Rótulo da prioridade",
                        FieldType::Text,
                        Value::Text(o.label().to_owned()),
                    )
                    .depth(1),
                );
            }
            out.push(
                group(s, "batch", "Lote", batch.is_some())
                    .help("Segura os itens e libera em grupos."),
            );
            if let Some(b) = batch {
                out.push(
                    int(s, "batch.size", "Itens por lote", b.size as u64)
                        .min(1.0)
                        .depth(1),
                );
                out.push(
                    float(s, "batch.max_wait", "Espera máxima", b.max_wait)
                        .min(0.0)
                        .depth(1)
                        .help("Libera um lote incompleto depois deste tempo (0 = espera sempre)."),
                );
                out.push(
                    boolean(s, "batch.flush", "Fecha a entrada até o lote sair", b.flush).depth(1),
                );
            }
        }
        ObjectKind::Processor {
            capacity,
            setup,
            process,
            operators,
            also,
            changeover,
            recipe,
        } => {
            out.push(
                int(s, "capacity", "Capacidade", *capacity as u64)
                    .min(1.0)
                    .unit("itens"),
            );
            out.push(opt_dist_field(
                s,
                "setup",
                "Preparação (setup)",
                setup.as_ref(),
            ));
            out.push(
                Field::new(
                    s,
                    "changeover",
                    "Troca de produto",
                    FieldType::Ron,
                    Value::Text(ron::option(changeover, ron::changeover)),
                )
                .help("Preparação só quando o produto muda: o rótulo do produto e, se quiser, o tempo de cada troca (de, para); sem linha na matriz vale a preparação acima. Exemplo: (label: \"produto\", matrix: [(from: 1, to: 2, time: Const(30.0))]). None: preparação em toda peça."),
            );
            out.push(dist_field(s, "process", "Tempo de processamento", process));
            out.push(
                Field::new(
                    s,
                    "recipe",
                    "Tempo por receita",
                    FieldType::Ron,
                    Value::Text(ron::option(recipe, ron::recipe)),
                )
                .help("Tempo de processamento que depende do item: o rótulo da receita e o tempo de cada valor; os itens sem receita na tabela usam o tempo acima. Exemplo: (label: \"produto\", times: [(value: 1, time: Const(90.0)), (value: 2, time: Const(120.0))]). None: o mesmo tempo para todos."),
            );
            out.push(
                group(s, "operators", "Operadores", operators.is_some())
                    .help("Quem opera esta máquina: marque e escolha abaixo, ou arraste do nó laranja do operador (ou da equipe) até a máquina. O processo espera um deles livre."),
            );
            if let Some(op) = operators {
                let mut names = vec![];
                for n in mobile_names(model) {
                    names.push(choice(&n, &n));
                }
                if !names.iter().any(|c| c.value == op.team) {
                    names.push(choice(&op.team, &format!("{} (não existe)", op.team)));
                }
                out.push(
                    Field::new(
                        s,
                        "operators.team",
                        "Quem opera",
                        FieldType::Choice,
                        Value::Text(op.team.clone()),
                    )
                    .choices(names)
                    .depth(1),
                );
                out.push(
                    int(s, "operators.count", "Quantos por peça", op.count as u64)
                        .min(1.0)
                        .help("Operadores que a peça precisa ao mesmo tempo: ela só começa quando todos chegam, e eles ficam com ela até o fim. Com mais operadores na equipe do que capacidade × este número, os de sobra ficam parados.")
                        .depth(1),
                );
                out.push(
                    float(
                        s,
                        "operators.team_speedup",
                        "Ganho da equipe",
                        op.team_speedup.unwrap_or(1.0),
                    )
                    .help("Quantas vezes mais rápido a equipe faz a etapa: o tempo sorteado é dividido por este número. 1: sem ganho (o tempo é o mesmo com um ou com vários).")
                    .depth(1),
                );
                out.push(boolean(s, "operators.setup", "Durante a preparação", op.setup).depth(1));
                out.push(
                    boolean(
                        s,
                        "operators.process",
                        "Durante o processamento",
                        op.process,
                    )
                    .depth(1),
                );
                out.push(
                    Field::new(
                        s,
                        "also",
                        "Outras equipes ao mesmo tempo",
                        FieldType::Ron,
                        Value::Text(ron::operator_uses(also)),
                    )
                    .help("Equipes que precisam estar juntas com os operadores (um médico e um enfermeiro na mesma sala). Exemplo: [OperatorUse(team: \"enfermagem\")]. A etapa só começa quando todos chegam.")
                    .depth(1),
                );
            }
        }
        ObjectKind::Combiner {
            components,
            mode,
            setup,
            process,
            ..
        } => {
            out.push(
                Field::new(s, "components", "Componentes por entrada", FieldType::Ron, Value::Text(ron::usizes(components)))
                    .help("A entrada 1 traz o contêiner; a entrada k + 1 traz o k-ésimo número de itens. Exemplo: [2, 1]."),
            );
            out.push(pick(
                s,
                "mode",
                "Modo",
                &ron::debug(mode),
                &[
                    ("Pack", "Embalar (só o contêiner sai)"),
                    ("Join", "Juntar (componentes destruídos)"),
                    ("Batch", "Lote (todos saem)"),
                ],
            ));
            out.push(opt_dist_field(
                s,
                "setup",
                "Preparação (setup)",
                setup.as_ref(),
            ));
            out.push(dist_field(s, "process", "Tempo de processamento", process));
        }
        ObjectKind::Separator { mode, process, .. } => {
            let text = ron::debug(mode);
            let split = text
                .strip_prefix("Split(")
                .and_then(|r| r.strip_suffix(')'));
            let by = match mode {
                utsim_objects::SeparateMode::SplitBy { count, group } => Some((count, group)),
                _ => None,
            };
            let chosen = if by.is_some() {
                "SplitBy"
            } else if split.is_some() {
                "Split"
            } else {
                "Unpack"
            };
            out.push(pick(
                s,
                "mode",
                "Modo",
                chosen,
                &[
                    ("Unpack", "Desembalar"),
                    ("Split", "Dividir em cópias"),
                    ("SplitBy", "Dividir pelo rótulo"),
                ],
            ));
            if let Some(n) = split.and_then(|n| n.parse::<u64>().ok()) {
                out.push(int(s, "mode.copies", "Cópias", n).min(1.0).depth(1));
            }
            if let Some((count, group)) = by {
                for (key, label, v) in [
                    ("mode.count", "Rótulo com quantas partes", count),
                    ("mode.group", "Rótulo com o número da divisão", group),
                ] {
                    out.push(
                        Field::new(s, key, label, FieldType::Text, Value::Text(v.clone())).depth(1),
                    );
                }
            }
            out.push(dist_field(s, "process", "Tempo de processamento", process));
        }
        ObjectKind::Sink => {
            out.push(Field::new(
                s,
                "info",
                "Sem parâmetros",
                FieldType::Info,
                Value::Text("A saída só conta os itens que chegam.".into()),
            ));
        }
        ObjectKind::Transporter {
            speed,
            acceleration,
            deceleration,
            capacity,
            load_time,
            unload_time,
            home,
            agv,
            movement,
            clearance,
            ..
        } => {
            out.extend(movement_fields(model, obj, *movement, *clearance));
            out.push(
                float(s, "speed", "Velocidade máxima", *speed)
                    .min(0.001)
                    .unit("m/u.t."),
            );
            for (path, label, v) in [
                ("acceleration", "Aceleração", acceleration),
                ("deceleration", "Desaceleração", deceleration),
            ] {
                out.push(
                    Field::new(
                        s,
                        path,
                        label,
                        FieldType::Float,
                        v.map_or(Value::Null, Value::Float),
                    )
                    .optional(v.is_some())
                    .min(0.001)
                    .help("Desligada, a velocidade muda na hora."),
                );
            }
            out.push(
                int(s, "capacity", "Capacidade", *capacity as u64)
                    .min(1.0)
                    .unit("itens"),
            );
            out.push(dist_field(s, "load_time", "Tempo de carga", load_time));
            out.push(dist_field(
                s,
                "unload_time",
                "Tempo de descarga",
                unload_time,
            ));
            out.push(boolean(s, "home", "Voltar à base quando ocioso", *home));
            out.push(boolean(s, "agv", "AGV (controle de tráfego na rede)", *agv));
        }
        ObjectKind::Conveyor {
            length,
            speed,
            item_length,
            accumulating,
            ..
        } => {
            // A belt drawn by nodes: its length is the route's (read only), and the nodes
            // are edited with the drawing tool (`path_edit.rs`).
            match crate::path_edit::routes(model)
                .into_iter()
                .find(|r| r.name == obj.name)
            {
                Some(r) => {
                    let num = |v: f64, d: usize| format!("{v:.d$}").replace('.', ",");
                    out.push(Field::new(
                        s,
                        "route.info",
                        "Trajeto desenhado",
                        FieldType::Info,
                        Value::Text(format!(
                            "{} m em {} nós; passagem {} u.t.; cabem {}; maior inclinação {}° (máx. {}°)",
                            num(r.length, 2),
                            r.nodes.len(),
                            num(r.transit, 1),
                            r.capacity,
                            num(r.max_degrees, 1),
                            num(r.limit, 0)
                        )),
                    ));
                    out.push(
                        Field::new(
                            s,
                            "route.editar",
                            "Editar nós",
                            FieldType::Action,
                            Value::Bool(true),
                        )
                        .help("Abre a ferramenta de desenho com esta esteira: arraste os nós, mude a altura e o raio."),
                    );
                }
                None => out.push(
                    float(s, "length", "Comprimento", *length)
                        .min(0.001)
                        .unit("m"),
                ),
            }
            out.push(
                float(s, "speed", "Velocidade", *speed)
                    .min(0.001)
                    .unit("m/u.t."),
            );
            out.push(
                float(s, "item_length", "Comprimento por item", *item_length)
                    .min(0.001)
                    .unit("m"),
            );
            out.push(boolean(
                s,
                "accumulating",
                "Acumula itens quando a saída bloqueia",
                *accumulating,
            ));
        }
        ObjectKind::Rack {
            bays,
            levels,
            dwell,
            placement,
            bay_width,
            level_height,
            ..
        } => {
            out.push(int(s, "bays", "Colunas", *bays as u64).min(1.0));
            out.push(int(s, "levels", "Níveis", *levels as u64).min(1.0));
            out.push(opt_dist_field(
                s,
                "dwell",
                "Permanência mínima",
                dwell.as_ref(),
            ));
            let abc = match placement {
                utsim_objects::RackPlacement::Abc { a, b } => Some((*a, *b)),
                _ => None,
            };
            out.push(pick(
                s,
                "placement",
                "Regra de ocupação",
                if abc.is_some() {
                    "Abc".to_string()
                } else {
                    ron::debug(placement)
                }
                .as_str(),
                &[
                    ("FirstFree", "Primeira livre"),
                    ("Random", "Aleatória"),
                    ("Abc", "Curva ABC (rótulo classe)"),
                ],
            ));
            if let Some((a, b)) = abc {
                out.push(
                    float(s, "placement.a", "Fração das posições da classe A", a)
                        .min(0.0)
                        .depth(1),
                );
                out.push(
                    float(s, "placement.b", "Fração das posições da classe B", b)
                        .min(0.0)
                        .depth(1),
                );
            }
            out.push(
                float(s, "bay_width", "Largura da coluna", *bay_width)
                    .min(0.01)
                    .unit("m"),
            );
            out.push(
                float(s, "level_height", "Altura do nível", *level_height)
                    .min(0.01)
                    .unit("m"),
            );
            out.push(Field::new(
                s,
                "info",
                "Capacidade",
                FieldType::Info,
                Value::Text(format!("{} posições", bays * levels)),
            ));
        }
        ObjectKind::Dispatcher { team, rule, .. } => {
            let mut choices: Vec<Choice> = transporter_names(model)
                .iter()
                .map(|n| choice(n, n))
                .collect();
            for m in team {
                if !choices.iter().any(|c| &c.value == m) {
                    choices.push(choice(m, &format!("{m} (não existe)")));
                }
            }
            out.push(
                Field::new(
                    s,
                    "team",
                    "Equipe",
                    FieldType::Set,
                    Value::List(team.clone()),
                )
                .choices(choices)
                .help("Transportadores que o despachante comanda."),
            );
            out.push(pick(
                s,
                "rule",
                "Regra de escolha",
                &ron::debug(rule),
                &[
                    ("FirstAvailable", "Primeiro livre"),
                    ("RoundRobin", "Rodízio"),
                    ("Nearest", "Mais próximo"),
                ],
            ));
        }
    }
    out
}

fn output_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    let s = S_OUTPUT;
    let mut out = Vec::new();
    out.push(
        Field::new(
            s,
            "output",
            "Regra de saída",
            FieldType::Choice,
            Value::Text(output_id(&obj.output).into()),
        )
        .choices(OUTPUT_RULES.iter().map(|(v, l)| choice(v, l)).collect())
        .help("Para qual ligação vai cada item, na ordem das ligações de saída."),
    );
    match &obj.output {
        OutputRule::Weighted(w) => {
            let n = outputs(model, &obj.name);
            let mut f = Field::new(
                s,
                "output.weights",
                "Pesos",
                FieldType::Ron,
                Value::Text(ron::floats(w)),
            )
            .depth(1)
            .help("Peso de cada saída, na ordem das ligações. Exemplo: [3.0, 1.0].");
            if w.len() != n {
                f.help = format!(
                    "Há {} pesos, mas o objeto tem {n} saídas. {}",
                    w.len(),
                    f.help
                );
            }
            out.push(f);
        }
        OutputRule::ByLabel(label) => out.push(
            Field::new(
                s,
                "output.label",
                "Rótulo com o número da porta",
                FieldType::Text,
                Value::Text(label.clone()),
            )
            .depth(1),
        ),
        OutputRule::Script(src) => out.push(
            Field::new(
                s,
                "output.script",
                "Script",
                FieldType::Script,
                Value::Text(src.clone()),
            )
            .depth(1)
            .help("Devolve o número da porta (1, 2, ...)."),
        ),
        _ => {}
    }
    let mut choices = vec![choice("", "Nenhum (passagem direta)")];
    for n in mobile_names(model) {
        choices.push(choice(&n, &n));
    }
    let current = obj.transport.clone().unwrap_or_default();
    if !choices.iter().any(|c| c.value == current) {
        choices.push(choice(&current, &format!("{current} (não existe)")));
    }
    out.push(
        Field::new(
            s,
            "transport",
            "Transporte de saída",
            FieldType::Choice,
            Value::Text(current),
        )
        .choices(choices)
        .help("Transportador ou despachante que leva os itens para o próximo bloco."),
    );
    out.push(
        group(
            s,
            "transfer",
            "Transferência pela distância",
            obj.transfer.is_some(),
        )
        .help(
            "Os itens levam atraso + distância / velocidade para chegar ao próximo bloco \
             pelas ligações diretas (sem transportador).",
        ),
    );
    if let Some(t) = &obj.transfer {
        out.push(
            float(s, "transfer.speed", "Velocidade", t.speed)
                .min(0.001)
                .depth(1)
                .unit("m/u.t."),
        );
        out.push(
            float(s, "transfer.delay", "Atraso", t.delay)
                .min(0.0)
                .depth(1),
        );
        out.push(
            pick(
                s,
                "transfer.path",
                "Caminho",
                &format!("{:?}", t.path),
                &[("Straight", "reta"), ("Network", "pela rede")],
            )
            .depth(1),
        );
    }
    out.push(
        Field::new(
            s,
            "on_entry",
            "Script ao entrar um item",
            FieldType::Script,
            obj.on_entry.clone().map_or(Value::Null, Value::Text),
        )
        .optional(obj.on_entry.is_some())
        .help("Roda para cada item que entra, antes de qualquer outra regra."),
    );
    out
}

/// Choices of `on_end` (what happens to work in progress when a shift ends).
const ON_END: [(&str, &str); 2] = [("Pause", "Pausar"), ("Finish", "Terminar o trabalho")];

fn on_end_id(e: ShiftEnd) -> &'static str {
    match e {
        ShiftEnd::Pause => "Pause",
        ShiftEnd::Finish => "Finish",
    }
}

fn windows_text(t: &ShiftTable) -> String {
    let windows: Vec<String> = t
        .off
        .iter()
        .map(|w| {
            format!(
                "(start: {}, duration: {})",
                ron::float(w.start),
                ron::float(w.duration)
            )
        })
        .collect();
    format!("[{}]", windows.join(", "))
}

fn downtime_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    let s = S_DOWNTIME;
    let mut out = vec![
        group(s, "downtime", "Quebras", obj.downtime.is_some())
            .help("Falhas aleatórias, com o tempo contado no relógio."),
    ];
    if let Some(d) = &obj.downtime {
        out.push(dist_field(s, "downtime.mtbf", "Tempo entre falhas (MTBF)", &d.mtbf).depth(1));
        out.push(dist_field(s, "downtime.mttr", "Tempo de reparo (MTTR)", &d.mttr).depth(1));
    }
    out.push(
        group(s, "shifts", "Turnos", obj.shifts.is_some())
            .help("Janelas fora do turno, repetidas a cada período."),
    );
    let Some(spec) = &obj.shifts else {
        return out;
    };
    let source = if spec.calendar_name().is_some() {
        "calendar"
    } else {
        "inline"
    };
    let mut from = pick(
        s,
        "shifts.source",
        "Origem",
        source,
        &[
            ("inline", "Personalizado"),
            ("calendar", "Calendário do modelo"),
        ],
    )
    .depth(1);
    from.help = if model.calendars.is_empty() {
        "O modelo não tem calendários: crie um em \"calendars\" para usá-lo aqui.".into()
    } else {
        "Um calendário é a mesma tabela de turnos usada por vários objetos.".into()
    };
    out.push(from);
    match spec {
        ShiftSpec::Inline(t) => {
            out.push(
                float(s, "shifts.period", "Período", t.period)
                    .min(0.001)
                    .depth(1),
            );
            out.push(
                Field::new(
                    s,
                    "shifts.off",
                    "Pausas",
                    FieldType::Ron,
                    Value::Text(windows_text(t)),
                )
                .depth(1)
                .help("Início e duração de cada pausa dentro do período, em ordem."),
            );
            out.push(
                pick(
                    s,
                    "shifts.on_end",
                    "No fim do turno",
                    on_end_id(t.on_end),
                    &ON_END,
                )
                .depth(1),
            );
        }
        ShiftSpec::Calendar { name, on_end } => {
            let mut names: Vec<&str> = model.calendars.keys().map(String::as_str).collect();
            if !names.contains(&name.as_str()) {
                names.insert(0, name);
            }
            let choices: Vec<(&str, &str)> = names.iter().map(|n| (*n, *n)).collect();
            out.push(pick(s, "shifts.calendar", "Calendário", name, &choices).depth(1));
            let mut ends = vec![("", "(o do calendário)")];
            ends.extend(ON_END);
            out.push(
                pick(
                    s,
                    "shifts.on_end",
                    "No fim do turno",
                    on_end.map_or("", on_end_id),
                    &ends,
                )
                .depth(1),
            );
        }
    }
    out
}

/// Text shown for "no asset": the default look of the kind.
pub const DEFAULT_ASSET: &str = "(padrão do tipo)";

/// Library and catalogue assets as asset choices (value `""` is the default look).
pub fn asset_choices() -> Vec<Choice> {
    let mut out = vec![choice("", DEFAULT_ASSET)];
    for spec in utsim_assets::Spec::defaults() {
        out.push(Choice {
            value: spec.name().into(),
            label: spec.name().into(),
            help: "UTSim (gerado por utsim-assets)".into(),
            group: "Biblioteca do UTSim".into(),
        });
    }
    for e in catalogue::entries() {
        let size = e.tamanho_real_m;
        out.push(Choice {
            value: e.nome.clone(),
            label: e.nome.clone(),
            help: format!(
                "{}, {}\n{}\n{} ({:.2} x {:.2} x {:.2} m)",
                e.autor, e.licenca, e.uso, e.arquivo, size.0, size.2, size.1
            ),
            group: "Terceiros (assets/terceiros)".into(),
        });
    }
    // The Blender elements of their own (`cnc`, `prensa`, `espera`...); not the person
    // bodies (the person's sex picks them) nor the `_medio` files.
    if let Some(cat) = crate::modelos3d::Catalogo::find(None) {
        for (stem, e) in &cat.elementos {
            if stem.ends_with("_medio")
                || crate::modelos3d::PESSOAS.contains(&stem.as_str())
                || out.iter().any(|c| &c.value == stem)
            {
                continue;
            }
            let t = e.tamanho;
            out.push(Choice {
                value: stem.clone(),
                label: stem.clone(),
                help: format!("{} ({:.2} x {:.2} x {:.2} m)", e.arquivo, t[0], t[1], t[2]),
                group: "Elementos 3D (assets/modelos3d)".into(),
            });
        }
    }
    out
}

fn look_fields(obj: &ObjectDef) -> Vec<Field> {
    let s = S_LOOK;
    let v = obj.visual.clone().unwrap_or_default();
    let mut asset = Field::new(
        s,
        "visual.asset",
        "Ativo",
        FieldType::Asset,
        Value::Text(v.asset.clone().unwrap_or_default()),
    )
    .choices(asset_choices());
    if let Some(e) = v.asset.as_deref().and_then(catalogue::by_name) {
        asset.help = format!("{}, {}. {}", e.autor, e.licenca, e.uso);
    }
    let mut out = vec![
        asset,
        Field::new(
            s,
            "visual.color",
            "Cor",
            FieldType::Color,
            v.color.clone().map_or(Value::Null, Value::Text),
        )
        .optional(v.color.is_some()),
        float(s, "visual.scale", "Escala", v.scale.unwrap_or(1.0)).min(0.01),
    ];
    // A fixed object turns in "Posição" (Rotação); a mobile resource faces where it goes,
    // and this only turns its drawing.
    if is_mobile(&obj.kind) {
        out.push(
            float(s, "visual.yaw", "Giro do desenho", v.yaw.unwrap_or(0.0))
                .unit("graus")
                .help("Corrige para onde o modelo 3D olha; o recurso vira para onde anda."),
        );
    }
    out.push(
        Field::new(
            s,
            "visual.reset",
            "Restaurar padrão",
            FieldType::Action,
            Value::Bool(obj.visual.as_ref().is_some_and(|v| {
                is_mobile(&obj.kind)
                    || VisualDef {
                        yaw: None,
                        ..v.clone()
                    } != VisualDef::default()
            })),
        )
        .help("Volta à aparência padrão do tipo de objeto."),
    );
    out
}

/// Every field of object `obj`, in panel order.
pub fn object_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    let mut out = vec![
        Field::new(
            S_OBJECT,
            "name",
            "Nome",
            FieldType::Text,
            Value::Text(obj.name.clone()),
        ),
        Field::new(
            S_OBJECT,
            "kind",
            "Tipo",
            FieldType::Info,
            Value::Text(kind_label(&obj.kind).into()),
        ),
    ];
    for (i, (axis, label)) in [("x", "X"), ("y", "Y"), ("z", "Z (altura)")]
        .iter()
        .enumerate()
    {
        out.push(
            float(
                S_POSITION,
                &format!("position.{axis}"),
                label,
                obj.position[i],
            )
            .unit("m"),
        );
    }
    // A fixed object turns about its centre (`visual.yaw`): its drawing, its ports and its
    // area on the floor turn together (the editor's R and rotate handle set it too).
    if !is_mobile(&obj.kind) {
        out.push(
            float(
                S_POSITION,
                "visual.yaw",
                "Rotação",
                crate::document::yaw_of(model, obj),
            )
            .unit("°")
            .help("Giro em graus, anti-horário visto de cima (R gira 90°, Ctrl+R 15°)."),
        );
    }
    if !is_mobile(&obj.kind) {
        out.extend(size_fields(model, obj));
    }
    out.extend(serves_fields(model, obj));
    out.extend(kind_fields(model, obj));
    if !is_mobile(&obj.kind) {
        out.extend(output_fields(model, obj));
        out.extend(access_fields(obj));
    }
    out.extend(person_fields(model, obj));
    out.extend(downtime_fields(model, obj));
    out.extend(look_fields(obj));
    out
}

/// Who a person is (`person`: sex, height, mass, age, experience and measured capacity),
/// for a mobile resource drawn as a person or one that already has the block. Only a
/// record for the run; the 3D body follows sex, height and mass.
fn person_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    if !matches!(obj.kind, ObjectKind::Transporter { .. }) {
        return Vec::new();
    }
    if obj.person.is_none() && model.role_of(obj) != utsim_nav::Role::Person {
        return Vec::new();
    }
    let s = S_PERSON;
    let p = obj.person.as_deref().cloned().unwrap_or_default();
    let cap = p.capacity.unwrap_or_default();
    let opt = |path: &str, label: &str, v: Option<f64>, unit: &'static str, help: &str| {
        Field::new(
            s,
            path,
            label,
            FieldType::Float,
            v.map_or(Value::Null, Value::Float),
        )
        .optional(v.is_some())
        .min(0.0)
        .unit(unit)
        .help(help)
    };
    vec![
        pick(
            s,
            "person.sex",
            "Sexo",
            match p.sex {
                Some(utsim_core::Sex::Female) => "Female",
                Some(utsim_core::Sex::Male) => "Male",
                None => "",
            },
            &[
                ("", "Não informado"),
                ("Female", "Feminino"),
                ("Male", "Masculino"),
            ],
        )
        .help(
            "Escolhe o corpo no 3D (não informado, o corpo sai do nome). Nenhum padrão de \
             fadiga depende dele; com Feminino, carga acima de 20 kg dá o aviso da CLT, art. 390.",
        ),
        opt(
            "person.height_m",
            "Altura",
            p.height_m,
            "m",
            "A altura do boneco no 3D. Desligada, ela é sorteada pelo nome.",
        ),
        opt(
            "person.mass_kg",
            "Massa",
            p.mass_kg,
            "kg",
            "Com a altura, dá a largura do boneco no 3D pelo índice de massa corporal.",
        ),
        Field::new(
            s,
            "person.age",
            "Idade",
            FieldType::Int,
            p.age.map_or(Value::Null, |a| Value::Int(i64::from(a))),
        )
        .optional(p.age.is_some())
        .min(0.0)
        .unit("anos")
        .help("Só registro: a simulação não lê."),
        opt(
            "person.experience_years",
            "Experiência na função",
            p.experience_years,
            "anos",
            "Só registro: a simulação não lê.",
        ),
        opt(
            "person.capacity.load_kg",
            "Carga máxima medida",
            cap.load_kg,
            "kg",
            "Uma carga acima dela dá aviso no modelo.",
        ),
        opt(
            "person.capacity.force_n",
            "Força máxima medida",
            cap.force_n,
            "N",
            "Empurrar ou puxar. Só registro.",
        ),
    ]
}

/// The edit of the `person` fields, or `None` for other fields. A block already there stays,
/// even empty.
/// The kind of waiting area of a queue, and, for a real one, its layers and pitch.
fn waiting_fields(s: &'static str, obj: &ObjectDef) -> Vec<Field> {
    let w = obj.waiting_area;
    let mut out = vec![
        pick(
            s,
            "waiting_area.kind",
            "Tipo de espera",
            w.map_or("", |w| w.kind.name()),
            &[
                ("", "Fila simples (sem lugares)"),
                ("Table", "Mesa"),
                ("Pallet", "Palete"),
                ("Shelf", "Estante"),
                ("Floor", "Chão (área demarcada)"),
            ],
        )
        .help(
            "Numa espera real cada peça ocupa um lugar (o arranjo do tipo, recalculado pelas \
             Dimensões): só sai a peça sem nada em cima, a mais antiga delas primeiro, e quem \
             retira vai até a frente da peça. Muda o resultado. Fila simples: como antes.",
        ),
    ];
    if let Some(w) = w {
        out.push(
            Field::new(
                s,
                "waiting_area.layers",
                "Camadas",
                FieldType::Int,
                Value::Int(i64::from(w.layers())),
            )
            .min(1.0)
            .help("Quantas peças empilhadas em cada lugar (na estante, em cada prateleira)."),
        );
        out.push(
            float(s, "waiting_area.pitch", "Passo entre peças", w.pitch())
                .min(utsim_model::espera::ITEM)
                .unit("m")
                .help("Distância entre os centros de duas peças vizinhas."),
        );
        let n = w.places(obj.size).len();
        out.push(Field::new(
            s,
            "waiting_area.info",
            "Lugares",
            FieldType::Info,
            Value::Text(format!("{n} lugares")),
        ));
    }
    out
}

/// The edit of the `waiting_area` fields, or `None` for other fields.
fn edit_waiting(
    obj: &ObjectDef,
    head: &str,
    rest: &str,
    value: &Value,
) -> Option<Result<Command, String>> {
    if head != "waiting_area" {
        return None;
    }
    Some((|| {
        let mut w = obj.waiting_area.unwrap_or_default();
        match rest {
            "kind" => {
                let k = value.as_text().unwrap_or("");
                if k.is_empty() {
                    return Ok(set(&obj.name, "waiting_area", "None".into()));
                }
                w.kind = utsim_model::WaitingKind::by_name(k).ok_or_else(|| {
                    format!("tipo de espera: Table, Pallet, Shelf ou Floor, não {k}")
                })?;
            }
            "layers" => {
                let n = usize_of(value, "camadas", 1)?;
                w.layers = Some(u32::try_from(n).map_err(|_| "camadas demais")?);
            }
            "pitch" => w.pitch = Some(f64_of(value, "passo")?),
            _ => return Err(format!("espera não tem o campo \"{rest}\"")),
        }
        if let Some(e) = w.problem() {
            return Err(e);
        }
        let text = ::ron::to_string(&w).map_err(|e| format!("espera: {e}"))?;
        Ok(set(&obj.name, "waiting_area", format!("Some({text})")))
    })())
}

fn edit_person(
    obj: &ObjectDef,
    head: &str,
    rest: &str,
    value: &Value,
) -> Option<Result<Command, String>> {
    if head != "person" {
        return None;
    }
    Some((|| {
        let mut p = obj.person.as_deref().cloned().unwrap_or_default();
        let (field, on) = match rest.strip_suffix(".on") {
            Some(f) => (f, true),
            None => (rest, false),
        };
        // A number field turned on starts from a usual value; off (or Null) clears it.
        let number = |start: f64| -> Result<Option<f64>, String> {
            if on {
                return Ok(bool_of(value, "ligado")?.then_some(start));
            }
            match value {
                Value::Null => Ok(None),
                v => {
                    let x = f64_of(v, field)?;
                    if !(x.is_finite() && x > 0.0) {
                        return Err(format!("{field}: precisa ser maior que zero"));
                    }
                    Ok(Some(x))
                }
            }
        };
        let mut cap = p.capacity.unwrap_or_default();
        match field {
            "sex" => {
                p.sex = match value.as_text().unwrap_or("") {
                    "" => None,
                    "Female" | "Feminino" => Some(utsim_core::Sex::Female),
                    "Male" | "Masculino" => Some(utsim_core::Sex::Male),
                    other => return Err(format!("sexo: Female, Male ou vazio, não {other}")),
                }
            }
            "height_m" => p.height_m = number(1.70)?,
            "mass_kg" => p.mass_kg = number(70.0)?,
            "experience_years" => p.experience_years = number(1.0)?,
            "age" => {
                p.age = if on {
                    bool_of(value, "ligado")?.then_some(30)
                } else {
                    match value {
                        Value::Null => None,
                        v => Some(
                            u32::try_from(usize_of(v, "idade", 0)?)
                                .map_err(|_| "idade grande demais")?,
                        ),
                    }
                }
            }
            "capacity.load_kg" => cap.load_kg = number(20.0)?,
            "capacity.force_n" => cap.force_n = number(200.0)?,
            _ => return Err(format!("pessoa não tem o campo \"{field}\"")),
        }
        p.capacity = (cap != utsim_core::Capacity::default()).then_some(cap);
        // An empty block (`person: Some(())`) still marks a person: it is kept once there.
        let text = if p == utsim_core::Person::default() && obj.person.is_none() {
            "None".to_string()
        } else {
            format!(
                "Some({})",
                ::ron::to_string(&p).map_err(|e| format!("pessoa: {e}"))?
            )
        };
        Ok(set(&obj.name, "person", text))
    })())
}

/// The value of the "Política de acesso" of a wait: `Fifo`, `Lifo`, or `Desc` / `Asc` when
/// it orders by a label.
pub fn access_policy(
    discipline: utsim_objects::Discipline,
    order: Option<&utsim_objects::LabelOrder>,
) -> String {
    match order {
        Some(utsim_objects::LabelOrder::Desc(_)) => "Desc".into(),
        Some(utsim_objects::LabelOrder::Asc(_)) => "Asc".into(),
        None => ron::debug(&discipline),
    }
}

/// What a transporter, forklift, AGV or dispatcher serves (section "Atende"): the machines
/// it operates and the objects whose items it carries, and how to change them.
fn serves_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    if !is_mobile(&obj.kind) {
        return Vec::new();
    }
    let s = "Atende";
    let me = obj.name.as_str();
    let operates: Vec<&str> = model
        .objects
        .iter()
        .filter(|o| o.kind.operator_uses().any(|op| op.team == me))
        .map(|o| o.name.as_str())
        .collect();
    let carries: Vec<&str> = model
        .objects
        .iter()
        .filter(|o| o.transport.as_deref() == Some(me))
        .map(|o| o.name.as_str())
        .collect();
    let list = |v: &[&str]| {
        if v.is_empty() {
            "nenhum".to_owned()
        } else {
            v.join(", ")
        }
    };
    vec![
        Field::new(
            s,
            "serves.operates",
            "Opera as máquinas",
            FieldType::Info,
            Value::Text(list(&operates)),
        )
        .help("Arraste do nó laranja até a máquina para ligar; clique na linha tracejada e no × para tirar."),
        Field::new(
            s,
            "serves.carries",
            "Leva os itens de",
            FieldType::Info,
            Value::Text(list(&carries)),
        )
        .help("Arraste do nó laranja até o objeto (uma espera, uma chegada, ou com Shift uma máquina) para ligar."),
    ]
}

/// The model's name and description and its `run` settings, edited as object `"run"`.
pub fn run_fields(model: &Model) -> Vec<Field> {
    let s = S_RUN;
    let run: &RunSettings = &model.run;
    let marking = model.marking();
    vec![
        Field::new(
            S_MODEL,
            "model_name",
            "Nome do modelo",
            FieldType::Text,
            Value::Text(model.name.clone()),
        ),
        Field::new(
            S_MODEL,
            "description",
            "Descrição",
            FieldType::Script,
            Value::Text(model.description.clone()),
        )
        .help("Texto livre sobre o modelo, gravado no arquivo."),
        float(s, "stop_time", "Tempo de simulação", run.stop_time).min(0.001),
        float(s, "warmup", "Aquecimento", run.warmup)
            .min(0.0)
            .help("As estatísticas só contam depois deste tempo."),
        int(s, "seed", "Semente", run.seed),
        int(
            s,
            "replications",
            "Replicações",
            u64::from(run.replications),
        )
        .min(1.0),
        pick(
            S_MOVE,
            "movement",
            "Movimentação padrão",
            movement_name(run.movement),
            &MOVEMENT_CHOICES,
        )
        .help(
            "Como andam os recursos móveis que não escolheram um modo próprio. Vazio, \
             vale Livre (menor caminho no piso, desviando dos obstáculos).",
        ),
        pick(
            S_FLOOR,
            "marking.profile",
            "Perfil da marcação do piso",
            if matches!(marking.profile, Profile::Branco) {
                "Branco"
            } else {
                "Brasil"
            },
            &[
                ("Brasil", "Brasil (faixa amarela)"),
                ("Branco", "Branco (redação original da NR-26)"),
            ],
        )
        .help(
            "Cores das faixas e setas dos corredores. O programa não certifica \
             conformidade: confira o texto vigente das normas.",
        ),
        float(
            S_FLOOR,
            "marking.line_width",
            "Largura da faixa",
            marking.line_width,
        )
        .min(0.001)
        .unit("m")
        .help("A prática usual vai de 0,05 a 0,15 m."),
        float(
            S_FLOOR,
            "marking.crossing_stop",
            "Parada no cruzamento",
            marking.crossing_stop,
        )
        .min(0.0)
        .unit("s")
        .help("O veículo para esse tempo antes de um cruzamento com pedestres."),
        Field::new(
            S_FLOOR,
            "marking.floor_color",
            "Cor do chão",
            FieldType::Color,
            marking.floor_color.clone().map_or(Value::Null, Value::Text),
        )
        .optional(marking.floor_color.is_some())
        .help(
            "A cor do piso na cena 3D, gravada no modelo; a grade muda de tom para continuar \
             visível. Desligada, o cinza de sempre.",
        ),
    ]
}

/// Current value of field `key` as RON text, ready for `Command::Set` (the Bevy
/// editor's `spec::read_field`).
pub fn read_field(obj: &ObjectDef, key: &str) -> Option<String> {
    match key {
        "output" => return Some(ron::output(&obj.output)),
        "downtime" => return Some(ron::option(&obj.downtime, ron::downtime)),
        "shifts" => return Some(ron::option(&obj.shifts, ron::shift_spec)),
        "transport" => return Some(ron::option(&obj.transport, |t| ron::string(t))),
        "transfer" => return Some(ron::option(&obj.transfer, ron::transfer)),
        "on_entry" => return Some(ron::option(&obj.on_entry, |s| ron::string(s))),
        _ => {}
    }
    Some(match (&obj.kind, key) {
        (ObjectKind::Processor { operators, .. }, "operators") => {
            ron::option(operators, ron::operator_use)
        }
        (ObjectKind::Processor { also, .. }, "also") => ron::operator_uses(also),
        (ObjectKind::Processor { changeover, .. }, "changeover") => {
            ron::option(changeover, ron::changeover)
        }
        (ObjectKind::Processor { recipe, .. }, "recipe") => ron::option(recipe, ron::recipe),
        (ObjectKind::Transporter { speed, .. }, "speed") => ron::float(*speed),
        (ObjectKind::Transporter { acceleration, .. }, "acceleration") => {
            ron::option(acceleration, |v| ron::float(*v))
        }
        (ObjectKind::Transporter { deceleration, .. }, "deceleration") => {
            ron::option(deceleration, |v| ron::float(*v))
        }
        (ObjectKind::Transporter { capacity, .. }, "capacity") => capacity.to_string(),
        (ObjectKind::Transporter { load_time, .. }, "load_time") => ron::dist(load_time),
        (ObjectKind::Transporter { unload_time, .. }, "unload_time") => ron::dist(unload_time),
        (ObjectKind::Transporter { home, .. }, "home") => home.to_string(),
        (ObjectKind::Transporter { agv, .. }, "agv") => agv.to_string(),
        (ObjectKind::Conveyor { length, .. }, "length") => ron::float(*length),
        (ObjectKind::Conveyor { speed, .. }, "speed") => ron::float(*speed),
        (ObjectKind::Conveyor { item_length, .. }, "item_length") => ron::float(*item_length),
        (ObjectKind::Conveyor { accumulating, .. }, "accumulating") => accumulating.to_string(),
        (ObjectKind::Rack { bays, .. }, "bays") => bays.to_string(),
        (ObjectKind::Rack { levels, .. }, "levels") => levels.to_string(),
        (ObjectKind::Rack { dwell, .. }, "dwell") => ron::option(dwell, ron::dist),
        (ObjectKind::Rack { placement, .. }, "placement") => match placement {
            utsim_objects::RackPlacement::Abc { a, b } => {
                format!("Abc(a: {}, b: {})", ron::float(*a), ron::float(*b))
            }
            _ => ron::debug(placement),
        },
        (ObjectKind::Rack { bay_width, .. }, "bay_width") => ron::float(*bay_width),
        (ObjectKind::Rack { level_height, .. }, "level_height") => ron::float(*level_height),
        (ObjectKind::Dispatcher { team, .. }, "team") => ron::strings(team),
        (ObjectKind::Dispatcher { rule, .. }, "rule") => ron::debug(rule),
        (ObjectKind::Source { interarrival, .. }, "interarrival") => ron::dist(interarrival),
        (ObjectKind::Source { first_at_zero, .. }, "first_at_zero") => first_at_zero.to_string(),
        (ObjectKind::Source { max_arrivals, .. }, "max_arrivals") => {
            ron::option(max_arrivals, |v| v.to_string())
        }
        (ObjectKind::Source { labels, .. }, "labels") => ron::labels(labels),
        (ObjectKind::Source { on_blocked, .. }, "on_blocked") => ron::debug(on_blocked),
        (ObjectKind::Source { schedule, .. }, "schedule") => ron::option(schedule, ron::schedule),
        (ObjectKind::Source { table, .. }, "table") => ron::option(table, ron::table),
        (ObjectKind::Queue { capacity, .. }, "capacity") => {
            ron::option(capacity, |v| v.to_string())
        }
        (ObjectKind::Queue { discipline, .. }, "discipline") => ron::debug(discipline),
        (ObjectKind::Queue { order, .. }, "order") => match order {
            Some(utsim_objects::LabelOrder::Asc(l)) => format!("Some(Asc({}))", ron::string(l)),
            Some(utsim_objects::LabelOrder::Desc(l)) => format!("Some(Desc({}))", ron::string(l)),
            None => "None".into(),
        },
        (ObjectKind::Queue { batch, .. }, "batch") => {
            ron::option(batch, |b| ron::batch(b.size, b.max_wait, b.flush))
        }
        (ObjectKind::Processor { capacity, .. }, "capacity") => capacity.to_string(),
        (ObjectKind::Processor { setup, .. }, "setup")
        | (ObjectKind::Combiner { setup, .. }, "setup") => ron::option(setup, ron::dist),
        (ObjectKind::Processor { process, .. }, "process")
        | (ObjectKind::Combiner { process, .. }, "process")
        | (ObjectKind::Separator { process, .. }, "process") => ron::dist(process),
        (ObjectKind::Combiner { components, .. }, "components") => ron::usizes(components),
        (ObjectKind::Combiner { mode, .. }, "mode") => ron::debug(mode),
        (ObjectKind::Separator { mode, .. }, "mode") => match mode {
            utsim_objects::SeparateMode::SplitBy { count, group } => format!(
                "SplitBy(count: {}, group: {})",
                ron::string(count),
                ron::string(group)
            ),
            _ => ron::debug(mode),
        },
        _ => return None,
    })
}

/// RON of field `head` when it is switched on (`<head>.on`).
fn default_on(model: &Model, obj: &ObjectDef, head: &str) -> Result<String, String> {
    Ok(match head {
        "max_arrivals" => "100".into(),
        // One day: the default interval in the morning, closed the rest of the day.
        "schedule" => "(cycle: 1440.0, periods: [(start: 0.0, end: 480.0), (start: 480.0, end: 1440.0, arrivals: Closed)])".into(),
        // A daily timetable with one row: ten items at 08:00.
        "table" => "(mode: Times, repeat: Cycle(1440.0), rows: [(at: 480.0, count: 10)])".into(),
        "capacity" => "10".into(),
        "batch" => ron::batch(2, 0.0, false),
        "setup" => ron::dist(&Dist::Const(1.0)),
        "dwell" => ron::dist(&Dist::Const(10.0)),
        "acceleration" | "deceleration" => ron::float(1.0),
        "operators" => {
            let team = mobile_names(model)
                .into_iter()
                .next()
                .ok_or("crie antes um transportador ou um despachante para operar o processo")?;
            ron::operators(&team, 1, true, true)
        }
        "transfer" => "(speed: 1.0, delay: 0.0, path: Straight)".into(),
        "on_entry" => ron::string(""),
        "downtime" => ron::downtime(&utsim_core::Downtime {
            mtbf: Dist::Exp { mean: 1000.0 },
            mttr: Dist::Exp { mean: 10.0 },
        }),
        // The first calendar of the model, as the Bevy editor does; else a table.
        "shifts" => match model.calendars.keys().next() {
            Some(name) => ron::shift_spec(&ShiftSpec::calendar(name.clone())),
            None => ron::shifts(&default_table()),
        },
        _ => {
            return Err(format!(
                "{}.{head} não pode ser ligado ou desligado",
                obj.name
            ));
        }
    })
}

fn usize_of(v: &Value, what: &str, min: i64) -> Result<usize, String> {
    v.as_int()
        .filter(|i| *i >= min)
        .map(|i| i as usize)
        .ok_or_else(|| format!("{what} precisa ser um inteiro >= {min}"))
}

fn f64_of(v: &Value, what: &str) -> Result<f64, String> {
    v.as_f64().ok_or_else(|| format!("{what}: número esperado"))
}

fn bool_of(v: &Value, what: &str) -> Result<bool, String> {
    v.as_bool()
        .ok_or_else(|| format!("{what}: verdadeiro ou falso esperado"))
}

/// A number, or `Null` for "off".
fn opt_f64(v: &Value, what: &str) -> Result<Option<f64>, String> {
    match v {
        Value::Null => Ok(None),
        v => f64_of(v, what).map(Some),
    }
}

/// Changes `head.rest` of a typed copy of the object.
fn mutate(o: &mut ObjectDef, head: &str, rest: &str, v: &Value) -> Result<(), String> {
    let what = if rest.is_empty() {
        head.to_string()
    } else {
        format!("{head}.{rest}")
    };
    let w = what.as_str();
    match head {
        "transfer" => {
            let t = o.transfer.as_mut().ok_or("ligue a transferência antes")?;
            match rest {
                "speed" => t.speed = f64_of(v, w)?,
                "delay" => t.delay = f64_of(v, w)?,
                "path" => {
                    t.path = match v.as_text() {
                        Some("Straight") => utsim_core::TransferPath::Straight,
                        Some("Network") => utsim_core::TransferPath::Network,
                        _ => return Err("caminho: Straight ou Network".into()),
                    }
                }
                _ => return Err(format!("campo desconhecido: {w}")),
            }
            return Ok(());
        }
        "downtime" => {
            let d = o.downtime.as_mut().ok_or("ligue as quebras antes")?;
            let (which, sub) = rest.split_once('.').unwrap_or((rest, ""));
            return match which {
                "mtbf" => edit_dist(&mut d.mtbf, sub, v),
                "mttr" => edit_dist(&mut d.mttr, sub, v),
                _ => Err(format!("campo desconhecido: {w}")),
            };
        }
        "shifts" => {
            let spec = o.shifts.as_mut().ok_or("ligue os turnos antes")?;
            let end = |v: &Value| match v.as_text() {
                Some("Pause") => Ok(Some(ShiftEnd::Pause)),
                Some("Finish") => Ok(Some(ShiftEnd::Finish)),
                Some("") | None => Ok(None),
                Some(t) => Err(format!("fim do turno: Pause ou Finish, não {t}")),
            };
            match (spec, rest) {
                (ShiftSpec::Inline(t), "period") => t.period = f64_of(v, w)?,
                (ShiftSpec::Inline(t), "off") => {
                    let text = v.as_text().ok_or("texto RON esperado")?;
                    t.off = ron::parse(text).map_err(|e| format!("pausas inválidas: {e}"))?;
                }
                (ShiftSpec::Inline(t), "on_end") => t.on_end = end(v)?.unwrap_or_default(),
                (ShiftSpec::Calendar { name, .. }, "calendar") => {
                    let n = v.as_text().ok_or("nome do calendário esperado")?.trim();
                    if n.is_empty() {
                        return Err("escolha um calendário".into());
                    }
                    *name = n.to_string();
                }
                (ShiftSpec::Calendar { on_end, .. }, "on_end") => *on_end = end(v)?,
                _ => return Err(format!("campo desconhecido: {w}")),
            }
            return Ok(());
        }
        _ => {}
    }
    match (&mut o.kind, head) {
        (ObjectKind::Source { interarrival, .. }, "interarrival") => {
            edit_dist(interarrival, rest, v)?
        }
        (ObjectKind::Source { first_at_zero, .. }, "first_at_zero") => {
            *first_at_zero = bool_of(v, w)?
        }
        (ObjectKind::Source { max_arrivals, .. }, "max_arrivals") => {
            *max_arrivals = match v {
                Value::Null => None,
                v => Some(usize_of(v, w, 1)? as u64),
            }
        }
        (ObjectKind::Queue { capacity, .. }, "capacity") => {
            *capacity = match v {
                Value::Null => None,
                v => Some(usize_of(v, w, 1)?),
            }
        }
        (ObjectKind::Queue { batch, .. }, "batch") => {
            let b = batch.as_mut().ok_or("ligue o lote antes")?;
            match rest {
                "size" => b.size = usize_of(v, w, 1)?,
                "max_wait" => b.max_wait = f64_of(v, w)?,
                "flush" => b.flush = bool_of(v, w)?,
                _ => return Err(format!("campo desconhecido: {w}")),
            }
        }
        (ObjectKind::Processor { capacity, .. }, "capacity")
        | (ObjectKind::Transporter { capacity, .. }, "capacity") => *capacity = usize_of(v, w, 1)?,
        (ObjectKind::Processor { setup, .. }, "setup")
        | (ObjectKind::Combiner { setup, .. }, "setup")
        | (ObjectKind::Rack { dwell: setup, .. }, "dwell") => {
            let d = setup
                .as_mut()
                .ok_or_else(|| format!("ligue {head} antes"))?;
            edit_dist(d, rest, v)?;
        }
        (ObjectKind::Processor { process, .. }, "process")
        | (ObjectKind::Combiner { process, .. }, "process")
        | (ObjectKind::Separator { process, .. }, "process")
        | (
            ObjectKind::Transporter {
                load_time: process, ..
            },
            "load_time",
        )
        | (
            ObjectKind::Transporter {
                unload_time: process,
                ..
            },
            "unload_time",
        ) => edit_dist(process, rest, v)?,
        (ObjectKind::Processor { operators, .. }, "operators") => {
            let op = operators.as_mut().ok_or("ligue os operadores antes")?;
            match rest {
                "team" => op.team = v.as_text().ok_or("nome esperado")?.to_string(),
                "count" => op.count = usize_of(v, w, 1)?,
                "team_speedup" => {
                    let gain = f64_of(v, w)?;
                    if !(gain.is_finite() && gain > 0.0) {
                        return Err("o ganho da equipe deve ser maior que zero".into());
                    }
                    op.team_speedup = (gain != 1.0).then_some(gain);
                }
                "setup" => op.setup = bool_of(v, w)?,
                "process" => op.process = bool_of(v, w)?,
                _ => return Err(format!("campo desconhecido: {w}")),
            }
        }
        (ObjectKind::Transporter { speed, .. }, "speed")
        | (ObjectKind::Conveyor { speed, .. }, "speed") => *speed = f64_of(v, w)?,
        (ObjectKind::Transporter { acceleration, .. }, "acceleration") => {
            *acceleration = opt_f64(v, w)?
        }
        (ObjectKind::Transporter { deceleration, .. }, "deceleration") => {
            *deceleration = opt_f64(v, w)?
        }
        (ObjectKind::Transporter { home, .. }, "home") => *home = bool_of(v, w)?,
        (ObjectKind::Transporter { agv, .. }, "agv") => *agv = bool_of(v, w)?,
        (ObjectKind::Conveyor { length, .. }, "length") => *length = f64_of(v, w)?,
        (ObjectKind::Conveyor { item_length, .. }, "item_length") => *item_length = f64_of(v, w)?,
        (ObjectKind::Conveyor { accumulating, .. }, "accumulating") => {
            *accumulating = bool_of(v, w)?
        }
        (ObjectKind::Rack { bays, .. }, "bays") => *bays = usize_of(v, w, 1)?,
        (ObjectKind::Rack { levels, .. }, "levels") => *levels = usize_of(v, w, 1)?,
        (ObjectKind::Rack { bay_width, .. }, "bay_width") => *bay_width = f64_of(v, w)?,
        (ObjectKind::Rack { level_height, .. }, "level_height") => *level_height = f64_of(v, w)?,
        (ObjectKind::Dispatcher { team, .. }, "team") => {
            let on = bool_of(v, w)?;
            team.retain(|m| m != rest);
            if on {
                team.push(rest.to_string());
            }
        }
        _ => return Err(format!("{} não tem o campo \"{w}\"", o.name)),
    }
    Ok(())
}

fn set(obj: &str, field: &str, value: String) -> Command {
    Command::Set {
        key: format!("{obj}.{field}"),
        value,
    }
}

/// Fields whose text value is plain text, not the RON of the field.
const TEXT_FIELDS: [&str; 3] = ["transport", "on_entry", "output"];

/// The command that sets `path` of object `object` (`"run"` for the run settings) to
/// `value`. Nothing is applied here: the document validates the command.
pub fn edit(model: &Model, object: &str, path: &str, value: &Value) -> Result<Command, String> {
    if object == "run" {
        return edit_run(model, path, value);
    }
    let obj = model
        .objects
        .iter()
        .find(|o| o.name == object)
        .ok_or_else(|| format!("objeto \"{object}\" não existe"))?;
    let (head, rest) = path.split_once('.').unwrap_or((path, ""));
    match head {
        "name" => {
            let to = value.as_text().ok_or("nome esperado")?.trim().to_string();
            return Ok(Command::Rename {
                from: obj.name.clone(),
                to,
            });
        }
        "position" => {
            let mut position = obj.position;
            let k = match rest {
                "x" => 0,
                "y" => 1,
                "z" => 2,
                _ => return Err("posição: x, y ou z".into()),
            };
            position[k] = f64_of(value, "posição")?;
            return Ok(Command::Move {
                name: obj.name.clone(),
                position,
            });
        }
        "visual" => return edit_visual(model, obj, rest, value),
        _ => {}
    }
    if let Some(command) = edit_movement(model, obj, head, rest, value) {
        return command;
    }
    if let Some(command) = edit_size(model, obj, head, rest, value) {
        return command;
    }
    if let Some(command) = edit_person(obj, head, rest, value) {
        return command;
    }
    if let Some(command) = edit_waiting(obj, head, rest, value) {
        return command;
    }
    if rest == "on" {
        let on = bool_of(value, path)?;
        let text = if on {
            default_on(model, obj, head)?
        } else {
            "None".to_string()
        };
        return Ok(set(&obj.name, head, text));
    }
    if let ObjectKind::Queue { order, .. } = &obj.kind {
        match (head, rest) {
            ("discipline", "") => {
                let v = value.as_text().ok_or("política esperada")?;
                let label = order
                    .as_ref()
                    .map_or("prioridade".to_owned(), |o| o.label().to_owned());
                let (discipline, order) = match v {
                    "Fifo" | "Lifo" => (v.to_owned(), "None".to_owned()),
                    "Desc" | "Asc" => (
                        "Fifo".to_owned(),
                        format!("Some({v}({}))", ron::string(&label)),
                    ),
                    _ => return Err(format!("política de acesso desconhecida: {v}")),
                };
                return Ok(Command::Batch(vec![
                    set(&obj.name, "discipline", discipline),
                    set(&obj.name, "order", order),
                ]));
            }
            ("order", "label") => {
                let l = value.as_text().ok_or("rótulo esperado")?.trim().to_owned();
                if l.is_empty() {
                    return Err("o rótulo da prioridade não pode ficar vazio".into());
                }
                let kind = match order {
                    Some(utsim_objects::LabelOrder::Asc(_)) => "Asc",
                    _ => "Desc",
                };
                return Ok(set(
                    &obj.name,
                    "order",
                    format!("Some({kind}({}))", ron::string(&l)),
                ));
            }
            _ => {}
        }
    }
    match (head, rest) {
        ("transport", "") => {
            let t = value.as_text().unwrap_or("");
            let text = if t.is_empty() {
                "None".into()
            } else {
                ron::string(t)
            };
            return Ok(set(&obj.name, head, text));
        }
        ("on_entry", "") => {
            let t = value.as_text().ok_or("script esperado")?;
            return Ok(set(&obj.name, head, ron::string(t)));
        }
        ("output", _) => return edit_output(model, obj, rest, value),
        ("shifts", "source") => return edit_shift_source(model, obj, value),
        ("placement", _) if matches!(obj.kind, ObjectKind::Rack { .. }) => {
            let (mut a, mut b) = match &obj.kind {
                ObjectKind::Rack {
                    placement: utsim_objects::RackPlacement::Abc { a, b },
                    ..
                } => (Some(*a), Some(*b)),
                _ => (None, None),
            };
            let text = match (rest, value.as_text()) {
                ("", Some("Abc")) => format!(
                    "Abc(a: {}, b: {})",
                    ron::float(a.unwrap_or(0.2)),
                    ron::float(b.unwrap_or(0.3))
                ),
                ("", Some(t)) => t.to_string(),
                ("a" | "b", _) => {
                    let v = value.as_f64().ok_or("número esperado")?;
                    if rest == "a" {
                        a = Some(v);
                    } else {
                        b = Some(v);
                    }
                    format!(
                        "Abc(a: {}, b: {})",
                        ron::float(a.unwrap_or(0.2)),
                        ron::float(b.unwrap_or(0.3))
                    )
                }
                _ => return Err("regra: FirstFree, Random ou Abc".into()),
            };
            return Ok(set(&obj.name, head, text));
        }
        ("mode", _) if matches!(obj.kind, ObjectKind::Separator { .. }) => {
            let current = read_field(obj, "mode").unwrap_or_default();
            let text = match (rest, value.as_text()) {
                ("", Some("Unpack")) => "Unpack".to_string(),
                // Choosing "Split" again keeps the number of copies.
                ("", Some("Split")) if current.starts_with("Split(") => current,
                ("", Some("Split")) => "Split(2)".to_string(),
                ("", Some("SplitBy")) if current.starts_with("SplitBy") => current,
                ("", Some("SplitBy")) => {
                    "SplitBy(count: \"linhas\", group: \"pedido\")".to_string()
                }
                ("copies", _) => format!("Split({})", usize_of(value, "cópias", 1)?),
                ("count" | "group", Some(t)) => {
                    let (mut count, mut group) = match &obj.kind {
                        ObjectKind::Separator {
                            mode: utsim_objects::SeparateMode::SplitBy { count, group },
                            ..
                        } => (count.clone(), group.clone()),
                        _ => ("linhas".to_string(), "pedido".to_string()),
                    };
                    if t.trim().is_empty() {
                        return Err("o rótulo não pode ficar vazio".into());
                    }
                    if rest == "count" {
                        count = t.trim().to_string();
                    } else {
                        group = t.trim().to_string();
                    }
                    format!(
                        "SplitBy(count: {}, group: {})",
                        ron::string(&count),
                        ron::string(&group)
                    )
                }
                (_, Some(t)) => t.to_string(),
                _ => return Err("modo: Unpack ou Split".into()),
            };
            return Ok(set(&obj.name, head, text));
        }
        _ => {}
    }
    let is_dist = |f: &str| {
        matches!(
            f,
            "interarrival" | "setup" | "process" | "load_time" | "unload_time" | "dwell"
        )
    };
    // The RON of the whole field: a choice (`Fifo`), a list, a map.
    if rest.is_empty()
        && !is_dist(head)
        && !TEXT_FIELDS.contains(&head)
        && let Value::Text(t) = value
    {
        return Ok(set(&obj.name, head, t.clone()));
    }
    let mut copy = obj.clone();
    mutate(&mut copy, head, rest, value)?;
    let text = read_field(&copy, head)
        .ok_or_else(|| format!("{} não tem o campo \"{head}\"", obj.name))?;
    Ok(set(&obj.name, head, text))
}

/// The default table of new shifts: a one-hour break in the middle of a day.
fn default_table() -> ShiftTable {
    ShiftTable {
        period: 1440.0,
        off: vec![OffWindow {
            start: 720.0,
            duration: 60.0,
        }],
        on_end: ShiftEnd::Pause,
    }
}

/// `shifts.source`: a calendar of the model (the first one) or a table of its own (a
/// copy of the calendar it used, as the Bevy editor does).
fn edit_shift_source(model: &Model, obj: &ObjectDef, value: &Value) -> Result<Command, String> {
    let spec = obj.shifts.as_ref().ok_or("ligue os turnos antes")?;
    let text = match (value.as_text(), spec) {
        (Some("calendar"), ShiftSpec::Calendar { .. }) | (Some("inline"), ShiftSpec::Inline(_)) => {
            ron::shift_spec(spec)
        }
        (Some("calendar"), ShiftSpec::Inline(_)) => {
            let name = model
                .calendars
                .keys()
                .next()
                .ok_or("o modelo não tem calendários (crie um em \"calendars\")")?;
            ron::shift_spec(&ShiftSpec::calendar(name.clone()))
        }
        (Some("inline"), ShiftSpec::Calendar { name, on_end }) => {
            let mut t = model
                .calendars
                .get(name)
                .cloned()
                .unwrap_or_else(default_table);
            if let Some(e) = on_end {
                t.on_end = *e;
            }
            ron::shifts(&t)
        }
        _ => return Err("origem dos turnos: inline ou calendar".into()),
    };
    Ok(set(&obj.name, "shifts", text))
}

fn edit_run(model: &Model, path: &str, value: &Value) -> Result<Command, String> {
    let text = |v: &Value| v.as_text().map(str::to_string).ok_or("texto esperado");
    match path {
        "model_name" => {
            return Ok(Command::SetInfo {
                name: text(value)?,
                description: model.description.clone(),
            });
        }
        "description" => {
            return Ok(Command::SetInfo {
                name: model.name.clone(),
                description: text(value)?,
            });
        }
        _ => {}
    }
    let mut marking = model.marking();
    match path {
        "movement" => {
            let movement = movement_from(value.as_text().ok_or("modo esperado")?)?;
            return Ok(Command::SetRunMovement(movement));
        }
        "marking.profile" => {
            marking.profile = match value.as_text() {
                Some("Brasil") => Profile::Brasil,
                Some("Branco") => Profile::Branco,
                _ => return Err("perfil: Brasil ou Branco".into()),
            };
            return Ok(Command::SetFloorMarking(Some(Box::new(marking))));
        }
        "marking.line_width" => {
            marking.line_width = f64_of(value, "largura da faixa")?;
            return Ok(Command::SetFloorMarking(Some(Box::new(marking))));
        }
        "marking.crossing_stop" => {
            marking.crossing_stop = f64_of(value, "parada no cruzamento")?;
            return Ok(Command::SetFloorMarking(Some(Box::new(marking))));
        }
        "marking.floor_color" | "marking.floor_color.on" => {
            marking.floor_color = match value {
                Value::Null | Value::Bool(false) => None,
                Value::Bool(true) => Some(marking.floor_color.unwrap_or_else(|| "#4d4f52".into())),
                Value::Text(t) if t.trim().is_empty() => None,
                Value::Text(t) => {
                    let t = t.trim();
                    let t = if t.starts_with('#') {
                        t.to_string()
                    } else {
                        format!("#{t}")
                    };
                    if looks::parse_color(&t).is_none() {
                        return Err(format!("cor inválida: {t} (use #rrggbb)"));
                    }
                    Some(t.to_lowercase())
                }
                _ => return Err("cor: #rrggbb".into()),
            };
            // Back to the default profile with no colour: no `floor_marking` at all.
            let none = marking == utsim_model::FloorMarking::default();
            return Ok(Command::SetFloorMarking((!none).then(|| Box::new(marking))));
        }
        _ => {}
    }
    let text = match path {
        "stop_time" | "warmup" => ron::float(f64_of(value, path)?),
        "seed" => usize_of(value, path, 0)?.to_string(),
        "replications" => usize_of(value, path, 1)?.to_string(),
        _ => return Err(format!("run não tem o campo \"{path}\"")),
    };
    Ok(set("run", path, text))
}

fn edit_output(
    model: &Model,
    obj: &ObjectDef,
    rest: &str,
    value: &Value,
) -> Result<Command, String> {
    let rule = match rest {
        "" => {
            let t = value.as_text().ok_or("regra esperada")?;
            if t == output_id(&obj.output) {
                // Choosing the current rule again keeps its weights, label or script.
                return Ok(set(&obj.name, "output", ron::output(&obj.output)));
            }
            match t {
                "FirstAvailable" => OutputRule::FirstAvailable,
                "RoundRobin" => OutputRule::RoundRobin,
                "Random" => OutputRule::Random,
                "Weighted" => OutputRule::Weighted(vec![1.0; outputs(model, &obj.name).max(1)]),
                "ByLabel" => OutputRule::ByLabel(first_label(model)),
                // A script returning the first port.
                "Script" => OutputRule::Script("1".into()),
                raw => return Ok(set(&obj.name, "output", raw.to_string())),
            }
        }
        "weights" => {
            let t = value.as_text().ok_or("lista de pesos esperada")?;
            let w: Vec<f64> = ron::parse(t).map_err(|e| format!("pesos inválidos: {e}"))?;
            OutputRule::Weighted(w)
        }
        "label" => {
            OutputRule::ByLabel(value.as_text().ok_or("rótulo esperado")?.trim().to_string())
        }
        "script" => OutputRule::Script(value.as_text().ok_or("script esperado")?.to_string()),
        _ => return Err(format!("campo desconhecido: output.{rest}")),
    };
    Ok(set(&obj.name, "output", ron::output(&rule)))
}

fn edit_visual(
    model: &Model,
    obj: &ObjectDef,
    rest: &str,
    value: &Value,
) -> Result<Command, String> {
    let mut v: VisualDef = obj.visual.clone().unwrap_or_default();
    match rest {
        "asset" => {
            let t = value.as_text().unwrap_or("").trim();
            v.asset = (!t.is_empty()).then(|| t.to_string());
        }
        "color" | "color.on" => match value {
            Value::Null | Value::Bool(false) => v.color = None,
            Value::Bool(true) => v.color = Some(v.color.unwrap_or_else(|| "#d08030".into())),
            Value::Text(t) if t.trim().is_empty() => v.color = None,
            Value::Text(t) => {
                let t = t.trim();
                let t = if t.starts_with('#') {
                    t.to_string()
                } else {
                    format!("#{t}")
                };
                if looks::parse_color(&t).is_none() {
                    return Err(format!("cor inválida: {t} (use #rrggbb)"));
                }
                v.color = Some(t.to_lowercase());
            }
            _ => return Err("cor: #rrggbb".into()),
        },
        "scale" => {
            let s = f64_of(value, "escala")?;
            if s.is_nan() || s <= 0.0 {
                return Err("a escala precisa ser maior que zero".into());
            }
            v.scale = (s != 1.0).then_some(s);
        }
        "yaw" => {
            let y = f64_of(value, "giro")?;
            if !y.is_finite() {
                return Err("giro: um número de graus".into());
            }
            if !is_mobile(&obj.kind) {
                // The rotation of a fixed object: in [0, 360), a belt keeps 0. Its drawn
                // value typed back (a belt following its connection) changes nothing.
                let unset = obj.visual.as_ref().and_then(|v| v.yaw).is_none();
                if unset
                    && crate::document::normal_degrees(y) == crate::document::yaw_of(model, obj)
                {
                    return Ok(Command::SetVisual {
                        name: obj.name.clone(),
                        visual: obj.visual.clone(),
                    });
                }
                return Ok(crate::document::yaw_command(obj, y));
            }
            v.yaw = (y != 0.0).then_some(y);
        }
        // The rotation of a fixed object is its place on the floor, not its look: kept.
        "reset" => {
            let keep = v.yaw.filter(|_| !is_mobile(&obj.kind));
            v = VisualDef {
                yaw: keep,
                ..VisualDef::default()
            };
        }
        _ => return Err(format!("campo desconhecido: visual.{rest}")),
    }
    Ok(Command::SetVisual {
        name: obj.name.clone(),
        visual: (v != VisualDef::default()).then_some(v),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use utsim_edit::{Document, template};

    fn doc() -> Document {
        let mut d = Document::new("t");
        for (k, n) in [
            ("Source", "a"),
            ("Queue", "q"),
            ("Processor", "p"),
            ("Sink", "s"),
            ("Transporter", "emp"),
        ] {
            d.apply(Command::Add(Box::new(template(k, n, [0.0; 3]).unwrap())))
                .unwrap();
        }
        for (a, b) in [("a", "q"), ("q", "p"), ("p", "s")] {
            d.apply(Command::Connect {
                from: a.into(),
                to: b.into(),
            })
            .unwrap();
        }
        d
    }

    fn apply(d: &mut Document, obj: &str, path: &str, v: Value) {
        let c = edit(d.model(), obj, path, &v).unwrap_or_else(|e| panic!("{obj}.{path}: {e}"));
        d.apply(c).unwrap_or_else(|e| panic!("{obj}.{path}: {e}"));
    }

    fn object<'a>(d: &'a Document, n: &str) -> &'a ObjectDef {
        d.model().objects.iter().find(|o| o.name == n).unwrap()
    }

    fn field(d: &Document, obj: &str, path: &str) -> Field {
        object_fields(d.model(), object(d, obj))
            .into_iter()
            .find(|f| f.path == path)
            .unwrap_or_else(|| panic!("no field {path}"))
    }

    #[test]
    fn the_kind_of_waiting_area_comes_first_and_is_edited_through_the_panel() {
        let mut d = doc();
        let fields = object_fields(d.model(), object(&d, "q"));
        let first = fields
            .iter()
            .position(|f| f.path == "waiting_area.kind")
            .unwrap();
        let cap = fields.iter().position(|f| f.path == "capacity").unwrap();
        assert!(first < cap);
        assert_eq!(fields[first].value, Value::Text(String::new()));
        assert!(!fields.iter().any(|f| f.path == "waiting_area.layers"));
        // A pallet: its layers and pitch show, and the count of places.
        apply(
            &mut d,
            "q",
            "waiting_area.kind",
            Value::Text("Pallet".into()),
        );
        let w = object(&d, "q").waiting_area.unwrap();
        assert_eq!(w.kind, utsim_model::WaitingKind::Pallet);
        assert_eq!(field(&d, "q", "waiting_area.layers").value, Value::Int(3));
        assert_eq!(
            field(&d, "q", "waiting_area.info").value,
            Value::Text("18 lugares".into())
        );
        apply(&mut d, "q", "waiting_area.layers", Value::Int(2));
        assert_eq!(
            field(&d, "q", "waiting_area.info").value,
            Value::Text("12 lugares".into())
        );
        assert!(d.to_text().contains("waiting_area"), "{}", d.to_text());
        // Bad values are refused; back to a plain queue.
        assert!(edit(d.model(), "q", "waiting_area.pitch", &Value::Float(0.1)).is_err());
        assert!(
            edit(
                d.model(),
                "q",
                "waiting_area.kind",
                &Value::Text("Mesa".into())
            )
            .is_err()
        );
        apply(&mut d, "q", "waiting_area.kind", Value::Text(String::new()));
        assert!(object(&d, "q").waiting_area.is_none());
    }

    #[test]
    fn who_the_person_is_is_edited_through_the_panel() {
        let mut d = doc();
        // A person: the group shows, empty.
        apply(&mut d, "emp", "visual.asset", Value::Text("pessoa".into()));
        assert_eq!(
            field(&d, "emp", "person.sex").value,
            Value::Text(String::new())
        );
        assert_eq!(field(&d, "emp", "person.height_m").value, Value::Null);
        apply(&mut d, "emp", "person.sex", Value::Text("Female".into()));
        apply(&mut d, "emp", "person.height_m.on", Value::Bool(true));
        apply(&mut d, "emp", "person.height_m", Value::Float(1.62));
        apply(&mut d, "emp", "person.mass_kg", Value::Float(58.0));
        apply(&mut d, "emp", "person.age", Value::Int(34));
        apply(&mut d, "emp", "person.capacity.load_kg", Value::Float(18.0));
        let p = object(&d, "emp").person.as_deref().cloned().unwrap();
        assert_eq!(p.sex, Some(utsim_core::Sex::Female));
        assert_eq!(
            (p.height_m, p.mass_kg, p.age),
            (Some(1.62), Some(58.0), Some(34))
        );
        assert_eq!(p.capacity.unwrap().load_kg, Some(18.0));
        assert_eq!(
            field(&d, "emp", "person.height_m").value,
            Value::Float(1.62)
        );
        assert!(d.to_text().contains("height_m"), "{}", d.to_text());
        // Bad values are refused; clearing everything keeps the (empty) block.
        assert!(edit(d.model(), "emp", "person.mass_kg", &Value::Float(-3.0)).is_err());
        assert!(edit(d.model(), "emp", "person.sex", &Value::Text("X".into())).is_err());
        for f in ["sex", "height_m", "mass_kg", "age", "capacity.load_kg"] {
            let v = if f == "sex" {
                Value::Text(String::new())
            } else {
                Value::Null
            };
            apply(&mut d, "emp", &format!("person.{f}"), v);
        }
        assert_eq!(
            object(&d, "emp").person.as_deref(),
            Some(&utsim_core::Person::default())
        );
        // Not on a machine.
        assert!(
            !object_fields(d.model(), object(&d, "p"))
                .iter()
                .any(|f| f.path.starts_with("person."))
        );
    }

    #[test]
    fn movement_clearance_access_and_marking_are_edited_through_the_panel() {
        let mut d = doc();
        assert_eq!(
            field(&d, "emp", "movement").value,
            Value::Text(String::new())
        );
        apply(&mut d, "emp", "movement", Value::Text("Aisles".into()));
        assert_eq!(
            field(&d, "emp", "movement").value,
            Value::Text("Aisles".into())
        );
        apply(&mut d, "emp", "movement", Value::Text(String::new()));
        assert!(matches!(
            object(&d, "emp").kind,
            ObjectKind::Transporter { movement: None, .. }
        ));
        assert!(edit(d.model(), "emp", "movement", &Value::Text("Voar".into())).is_err());
        assert!(!field(&d, "emp", "clearance").enabled);
        apply(&mut d, "emp", "clearance.on", Value::Bool(true));
        assert!(field(&d, "emp", "clearance").enabled);
        apply(&mut d, "emp", "clearance", Value::Float(1.4));
        assert_eq!(field(&d, "emp", "clearance").value, Value::Float(1.4));
        apply(&mut d, "emp", "clearance.on", Value::Bool(false));
        assert_eq!(field(&d, "emp", "clearance").value, Value::Null);
        // Fixed objects have the access point, mobile ones do not.
        assert!(!field(&d, "p", "access").enabled);
        assert!(
            object_fields(d.model(), object(&d, "emp"))
                .iter()
                .all(|f| !f.path.starts_with("access"))
        );
        apply(&mut d, "p", "access.on", Value::Bool(true));
        apply(&mut d, "p", "access.x", Value::Float(1.5));
        assert_eq!(object(&d, "p").access, Some([1.5, -2.0]));
        apply(&mut d, "p", "access.on", Value::Bool(false));
        assert_eq!(object(&d, "p").access, None);
        // The run settings: the default mode and the marking.
        apply(&mut d, "run", "movement", Value::Text("Network".into()));
        assert_eq!(d.model().run.movement, Some(Movement::Network));
        apply(
            &mut d,
            "run",
            "marking.profile",
            Value::Text("Branco".into()),
        );
        apply(&mut d, "run", "marking.crossing_stop", Value::Float(3.0));
        // The floor colour: kept as #rrggbb, off again with Null; a bad colour is refused.
        apply(
            &mut d,
            "run",
            "marking.floor_color",
            Value::Text("D0C8A0".into()),
        );
        assert_eq!(d.model().marking().floor_color.as_deref(), Some("#d0c8a0"));
        assert!(d.to_text().contains("floor_color"), "{}", d.to_text());
        assert!(
            edit(
                d.model(),
                "run",
                "marking.floor_color",
                &Value::Text("bege".into())
            )
            .is_err()
        );
        apply(&mut d, "run", "marking.floor_color", Value::Null);
        assert_eq!(d.model().marking().floor_color, None);
        apply(&mut d, "run", "marking.line_width", Value::Float(0.12));
        let m = d.model().marking();
        assert_eq!(
            (m.profile, m.crossing_stop, m.line_width),
            (Profile::Branco, 3.0, 0.12)
        );
        assert!(edit(d.model(), "run", "marking.line_width", &Value::Float(0.0)).is_ok());
        assert!(
            d.apply(edit(d.model(), "run", "marking.line_width", &Value::Float(0.0)).unwrap())
                .is_err()
        );
        let fields = run_fields(d.model());
        for path in ["movement", "marking.profile", "marking.line_width"] {
            assert!(fields.iter().any(|f| f.path == path), "{path}");
        }
    }

    #[test]
    fn dimensions_and_the_area_of_a_queue_are_edited_through_the_panel() {
        let mut d = doc();
        // Off: the default footprint as information.
        assert_eq!(
            field(&d, "p", "size.info").value,
            Value::Text("1,80 × 1,80 m (padrão)".into())
        );
        // On: it starts from the footprint and a usual height, and each axis is edited.
        apply(&mut d, "p", "size.on", Value::Bool(true));
        assert_eq!(object(&d, "p").size, Some([1.8, 1.8, 1.8]));
        apply(&mut d, "p", "size.x", Value::Float(4.0));
        apply(&mut d, "p", "size.z", Value::Float(2.5));
        assert_eq!(object(&d, "p").size, Some([4.0, 1.8, 2.5]));
        assert_eq!(field(&d, "p", "size.y").value, Value::Float(1.8));
        assert_eq!(d.model().footprint(object(&d, "p")), Some([4.0, 1.8]));
        assert!(edit(d.model(), "p", "size.y", &Value::Float(0.0)).is_ok());
        assert!(
            d.apply(edit(d.model(), "p", "size.y", &Value::Float(0.0)).unwrap())
                .is_err()
        );
        apply(&mut d, "p", "size.on", Value::Bool(false));
        assert_eq!(object(&d, "p").size, None);
        // Mobile resources have no dimensions.
        assert!(
            object_fields(d.model(), object(&d, "emp"))
                .iter()
                .all(|f| !f.path.starts_with("size"))
        );
        // A queue: capacity by area.
        apply(&mut d, "q", "size.on", Value::Bool(true));
        apply(&mut d, "q", "size.x", Value::Float(3.0));
        apply(&mut d, "q", "size.y", Value::Float(2.0));
        apply(&mut d, "q", "area_per_item.on", Value::Bool(true));
        assert_eq!(object(&d, "q").area_per_item, Some(1.0));
        apply(&mut d, "q", "area_per_item", Value::Float(1.5));
        assert_eq!(
            field(&d, "q", "area_per_item.info").value,
            Value::Text("4 itens".into())
        );
        assert!(
            object_fields(d.model(), object(&d, "p"))
                .iter()
                .all(|f| !f.path.starts_with("area_per_item"))
        );
        apply(&mut d, "q", "area_per_item.on", Value::Bool(false));
        assert_eq!(object(&d, "q").area_per_item, None);
    }

    #[test]
    fn every_field_of_every_example_reads_back_unchanged() {
        // Writing each value back through `edit` leaves every model as it was.
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
        let mut n = 0;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|e| e != "ron") {
                continue;
            }
            let Ok(mut d) = Document::open(&path) else {
                continue;
            };
            let objects = d.model().objects.clone();
            for obj in &objects {
                for f in object_fields(d.model(), obj) {
                    let v = match (&f.ty, &f.value) {
                        (
                            FieldType::Info | FieldType::Action | FieldType::Group | FieldType::Set,
                            _,
                        ) => continue,
                        (_, Value::Null) => continue,
                        (_, v) => v.clone(),
                    };
                    let before = d.model().clone();
                    apply(&mut d, &obj.name, &f.path, v);
                    assert_eq!(
                        d.model(),
                        &before,
                        "{}: {}.{}",
                        path.display(),
                        obj.name,
                        f.path
                    );
                    n += 1;
                }
            }
            for f in run_fields(d.model()) {
                let before = d.model().clone();
                apply(&mut d, "run", &f.path, f.value.clone());
                assert_eq!(d.model(), &before, "run.{}", f.path);
            }
        }
        assert!(n > 200, "only {n} fields checked");
    }

    #[test]
    fn shifts_use_a_table_or_a_calendar_of_the_model() {
        let mut d = doc();
        // Without calendars: a table of its own, and "calendar" is refused.
        apply(&mut d, "p", "shifts.on", Value::Bool(true));
        assert_eq!(
            field(&d, "p", "shifts.source").value,
            Value::Text("inline".into())
        );
        assert!(
            edit(
                d.model(),
                "p",
                "shifts.source",
                &Value::Text("calendar".into())
            )
            .is_err()
        );
        apply(&mut d, "p", "shifts.period", Value::Float(480.0));
        // With a calendar: switching keeps the calendar's name, and back copies its table.
        d.apply(Command::Set {
            key: "calendars.turno_a".into(),
            value: "(period: 1440.0, off: [(start: 480.0, duration: 960.0)])".into(),
        })
        .unwrap();
        apply(&mut d, "p", "shifts.source", Value::Text("calendar".into()));
        assert_eq!(object(&d, "p").shifts, Some(ShiftSpec::calendar("turno_a")));
        let cal = field(&d, "p", "shifts.calendar");
        assert_eq!(cal.value, Value::Text("turno_a".into()));
        assert_eq!(cal.choices.len(), 1);
        assert_eq!(
            field(&d, "p", "shifts.on_end").value,
            Value::Text(String::new())
        );
        apply(&mut d, "p", "shifts.on_end", Value::Text("Finish".into()));
        assert_eq!(
            object(&d, "p").shifts,
            Some(ShiftSpec::Calendar {
                name: "turno_a".into(),
                on_end: Some(ShiftEnd::Finish),
            })
        );
        apply(&mut d, "p", "shifts.source", Value::Text("inline".into()));
        let Some(ShiftSpec::Inline(t)) = &object(&d, "p").shifts else {
            panic!("{:?}", object(&d, "p").shifts)
        };
        assert_eq!(
            (t.period, t.off.len(), t.on_end),
            (1440.0, 1, ShiftEnd::Finish)
        );
        // A new switch-on picks the first calendar, as the Bevy editor does.
        apply(&mut d, "q", "shifts.on", Value::Bool(true));
        assert_eq!(object(&d, "q").shifts, Some(ShiftSpec::calendar("turno_a")));
        assert!(
            edit(
                d.model(),
                "q",
                "shifts.on_end",
                &Value::Text("Talvez".into())
            )
            .is_err()
        );
    }

    #[test]
    fn a_source_has_arrivals_by_period_as_ron() {
        let mut d = doc();
        assert_eq!(field(&d, "a", "schedule").value, Value::Null);
        apply(&mut d, "a", "schedule.on", Value::Bool(true));
        let f = field(&d, "a", "schedule");
        assert!(f.enabled);
        let ObjectKind::Source { schedule, .. } = &object(&d, "a").kind else {
            panic!()
        };
        assert_eq!(schedule.as_ref().unwrap().periods.len(), 2);
        let text = "(cycle: 60.0, periods: [(start: 0.0, end: 30.0, arrivals: Rate(2.0))])";
        apply(&mut d, "a", "schedule", Value::Text(text.into()));
        assert_eq!(read_field(object(&d, "a"), "schedule").unwrap(), text);
        apply(&mut d, "a", "schedule.on", Value::Bool(false));
        assert_eq!(field(&d, "a", "schedule").value, Value::Null);
    }

    #[test]
    fn a_source_has_an_arrival_table_as_ron() {
        let mut d = doc();
        assert_eq!(field(&d, "a", "table").value, Value::Null);
        assert!(!field(&d, "a", "table").enabled);
        apply(&mut d, "a", "table.on", Value::Bool(true));
        let f = field(&d, "a", "table");
        assert!(f.enabled);
        let ObjectKind::Source { table, .. } = &object(&d, "a").kind else {
            panic!()
        };
        let t = table.as_ref().unwrap();
        assert_eq!((t.rows.len(), t.rows[0].count), (1, 10));
        let text = "(mode: Sequence, repeat: Loop, rows: [(count: 2, item_type: \"A\"), \
                    (count: 1, labels: {\"prioridade\": 2.0})])";
        apply(&mut d, "a", "table", Value::Text(text.into()));
        assert_eq!(read_field(object(&d, "a"), "table").unwrap(), text);
        apply(&mut d, "a", "table.on", Value::Bool(false));
        assert_eq!(field(&d, "a", "table").value, Value::Null);
    }

    #[test]
    fn the_model_name_and_description_are_run_fields() {
        let mut d = doc();
        let fields = run_fields(d.model());
        assert_eq!(fields[0].path, "model_name");
        assert_eq!(fields[0].section, S_MODEL);
        apply(&mut d, "run", "model_name", Value::Text("Linha".into()));
        apply(
            &mut d,
            "run",
            "description",
            Value::Text("Uma linha".into()),
        );
        assert_eq!(d.model().name, "Linha");
        assert_eq!(d.model().description, "Uma linha");
        assert!(
            edit(d.model(), "run", "model_name", &Value::Text(" ".into()))
                .and_then(|c| d.apply(c))
                .is_err()
        );
    }

    #[test]
    fn the_schema_has_sections_types_and_choices() {
        let d = doc();
        let fields = object_fields(d.model(), object(&d, "p"));
        let sections: Vec<&str> = fields.iter().map(|f| f.section).collect();
        for s in [S_OBJECT, S_POSITION, S_PARAMS, S_OUTPUT, S_DOWNTIME, S_LOOK] {
            assert!(sections.contains(&s), "{s}");
        }
        let process = field(&d, "p", "process");
        assert_eq!(process.ty, FieldType::Dist);
        let view = process.dist.unwrap();
        assert_eq!(FAMILIES[view.family], "Constante");
        assert_eq!(view.params[0].key, "value");
        assert_eq!(view.text, "Const(1.0)");
        let setup = field(&d, "p", "setup");
        assert!(setup.optional && !setup.enabled);
        let transport = field(&d, "p", "transport");
        assert_eq!(transport.choices.len(), 2, "nenhum + emp");
        let asset = field(&d, "p", "visual.asset");
        assert!(asset.choices.iter().any(|c| c.value == "maquina"));
        assert!(
            asset
                .choices
                .iter()
                .any(|c| c.group.starts_with("Terceiros"))
        );
        assert_eq!(field(&d, "p", "position.x").unit, "m");
        // Mobile resources have no output section.
        assert!(
            object_fields(d.model(), object(&d, "emp"))
                .iter()
                .all(|f| f.section != S_OUTPUT)
        );
    }

    #[test]
    fn distributions_change_family_and_parameters() {
        let mut d = doc();
        apply(&mut d, "p", "process.family", Value::Int(3));
        let ObjectKind::Processor { process, .. } = &object(&d, "p").kind else {
            panic!()
        };
        assert!(matches!(process, Dist::Triangular { .. }));
        assert!((process.mean() - 1.0).abs() < 1e-9);
        apply(&mut d, "p", "process.mode", Value::Float(1.2));
        apply(&mut d, "p", "process", Value::Text("Exp(mean: 4.0)".into()));
        assert_eq!(
            field(&d, "p", "process").value,
            Value::Text("Exp(mean: 4.0)".into())
        );
        apply(&mut d, "p", "process.family", Value::Int(19));
        apply(&mut d, "p", "process.offset", Value::Float(0.5));
        apply(
            &mut d,
            "p",
            "process.inner",
            Value::Text("Const(2.0)".into()),
        );
        assert_eq!(
            field(&d, "p", "process").value,
            Value::Text("Shifted(offset: 0.5, dist: Const(2.0))".into())
        );
        apply(&mut d, "a", "interarrival.family", Value::Int(7));
        apply(&mut d, "a", "interarrival.k", Value::Int(3));
        assert!(
            field(&d, "a", "interarrival")
                .value
                .as_text()
                .unwrap()
                .contains("k: 3")
        );
        assert!(
            edit(
                d.model(),
                "p",
                "process",
                &Value::Text("Exp(mean: -1.0)".into())
            )
            .is_err()
        );
        assert!(edit(d.model(), "p", "process.nope", &Value::Float(1.0)).is_err());
    }

    #[test]
    fn optional_groups_switch_on_and_keep_their_other_parts() {
        let mut d = doc();
        apply(&mut d, "q", "transfer.on", Value::Bool(true));
        apply(&mut d, "q", "transfer.speed", Value::Float(2.5));
        apply(&mut d, "q", "transfer.path", Value::Text("Network".into()));
        let t = object(&d, "q").transfer.unwrap();
        assert_eq!(
            (t.speed, t.delay, t.path),
            (2.5, 0.0, utsim_core::TransferPath::Network)
        );
        apply(&mut d, "q", "batch.on", Value::Bool(true));
        apply(&mut d, "q", "batch.size", Value::Int(5));
        assert_eq!(field(&d, "q", "batch.size").value, Value::Int(5));
        apply(&mut d, "q", "capacity.on", Value::Bool(true));
        apply(&mut d, "q", "capacity", Value::Int(3));
        apply(&mut d, "q", "capacity", Value::Null);
        assert_eq!(field(&d, "q", "capacity").value, Value::Null);
        apply(&mut d, "p", "downtime.on", Value::Bool(true));
        apply(&mut d, "p", "downtime.mttr.mean", Value::Float(7.0));
        assert_eq!(
            object(&d, "p").downtime.as_ref().unwrap().mttr,
            Dist::Exp { mean: 7.0 }
        );
        apply(&mut d, "p", "operators.on", Value::Bool(true));
        apply(&mut d, "p", "operators.count", Value::Int(1));
        apply(&mut d, "p", "shifts.on", Value::Bool(true));
        apply(
            &mut d,
            "p",
            "shifts.off",
            Value::Text("[(start: 10.0, duration: 5.0)]".into()),
        );
        assert!(matches!(
            object(&d, "p").shifts.as_ref(),
            Some(ShiftSpec::Inline(t)) if t.off.len() == 1
        ));
        apply(&mut d, "p", "shifts.on_end", Value::Text("Finish".into()));
        assert!(matches!(
            object(&d, "p").shifts.as_ref(),
            Some(ShiftSpec::Inline(t)) if t.on_end == ShiftEnd::Finish
        ));
        apply(&mut d, "p", "transfer.on", Value::Bool(true));
        apply(&mut d, "p", "transfer.on", Value::Bool(false));
        assert!(object(&d, "p").transfer.is_none());
        assert!(edit(d.model(), "q", "batch.size", &Value::Int(0)).is_err());
    }

    #[test]
    fn processor_with_a_product_changeover() {
        let mut d = doc();
        assert_eq!(
            field(&d, "p", "changeover").value,
            Value::Text("None".into())
        );
        apply(&mut d, "p", "setup.on", Value::Bool(true));
        apply(
            &mut d,
            "p",
            "changeover",
            Value::Text(
                "(label: \"produto\", matrix: [(from: 1.0, to: 2.0, time: Const(3.0))])".into(),
            ),
        );
        let ObjectKind::Processor { changeover, .. } = &object(&d, "p").kind else {
            unreachable!()
        };
        assert_eq!(changeover.as_ref().unwrap().label, "produto");
        assert_eq!(
            field(&d, "p", "changeover").value,
            Value::Text(
                "(label: \"produto\", matrix: [(from: 1.0, to: 2.0, time: Const(3.0))])".into()
            )
        );
        apply(&mut d, "p", "changeover", Value::Text("None".into()));
        let ObjectKind::Processor { changeover, .. } = &object(&d, "p").kind else {
            unreachable!()
        };
        assert!(changeover.is_none());
    }

    #[test]
    fn processor_with_a_recipe() {
        let mut d = doc();
        assert_eq!(field(&d, "p", "recipe").value, Value::Text("None".into()));
        let text = "(label: \"produto\", times: [(value: 1.0, time: Const(90.0))])";
        apply(&mut d, "p", "recipe", Value::Text(text.into()));
        let ObjectKind::Processor { recipe, .. } = &object(&d, "p").kind else {
            unreachable!()
        };
        assert_eq!(recipe.as_ref().unwrap().times.len(), 1);
        assert_eq!(field(&d, "p", "recipe").value, Value::Text(text.into()));
        apply(&mut d, "p", "recipe", Value::Text("None".into()));
        let ObjectKind::Processor { recipe, .. } = &object(&d, "p").kind else {
            unreachable!()
        };
        assert!(recipe.is_none());
    }

    #[test]
    fn processor_with_another_team_at_the_same_time() {
        let mut d = doc();
        d.apply(Command::Add(Box::new(
            template("Transporter", "enf", [0.0; 3]).unwrap(),
        )))
        .unwrap();
        // Without operators the field is not shown.
        assert!(
            !object_fields(d.model(), object(&d, "p"))
                .iter()
                .any(|f| f.path == "also")
        );
        apply(&mut d, "p", "operators.on", Value::Bool(true));
        assert_eq!(field(&d, "p", "also").value, Value::Text("[]".into()));
        apply(
            &mut d,
            "p",
            "also",
            Value::Text("[OperatorUse(team: \"enf\", count: 1)]".into()),
        );
        let ObjectKind::Processor { also, .. } = &object(&d, "p").kind else {
            unreachable!()
        };
        assert_eq!(also.len(), 1);
        assert_eq!(also[0].team, "enf");
        assert_eq!(
            field(&d, "p", "also").value,
            Value::Text("[(team: \"enf\", count: 1, setup: true, process: true)]".into())
        );
    }

    #[test]
    fn output_rules_transport_and_choices() {
        let mut d = doc();
        apply(&mut d, "q", "output", Value::Text("Weighted".into()));
        assert_eq!(object(&d, "q").output, OutputRule::Weighted(vec![1.0]));
        apply(&mut d, "q", "output.weights", Value::Text("[2.0]".into()));
        apply(&mut d, "q", "output", Value::Text("Script".into()));
        apply(&mut d, "q", "output.script", Value::Text("1".into()));
        apply(&mut d, "q", "transport", Value::Text("emp".into()));
        assert_eq!(object(&d, "q").transport.as_deref(), Some("emp"));
        apply(&mut d, "q", "transport", Value::Text(String::new()));
        assert_eq!(object(&d, "q").transport, None);
        apply(&mut d, "q", "discipline", Value::Text("Lifo".into()));
        assert_eq!(
            field(&d, "q", "discipline").value,
            Value::Text("Lifo".into())
        );
        apply(
            &mut d,
            "a",
            "labels",
            Value::Text("{\"tipo\": Const(1.0)}".into()),
        );
        assert!(
            edit(d.model(), "q", "discipline", &Value::Text("Nada".into()))
                .and_then(|c| d.apply(c))
                .is_err()
        );
    }

    #[test]
    fn name_position_look_and_run() {
        let mut d = doc();
        apply(&mut d, "p", "position.x", Value::Float(4.5));
        apply(&mut d, "p", "position.y", Value::Int(-2));
        assert_eq!(object(&d, "p").position, [4.5, -2.0, 0.0]);
        apply(&mut d, "p", "visual.asset", Value::Text("bancada".into()));
        apply(&mut d, "p", "visual.color", Value::Text("FF8000".into()));
        apply(&mut d, "p", "visual.scale", Value::Float(1.5));
        apply(&mut d, "p", "visual.yaw", Value::Float(90.0));
        let v = object(&d, "p").visual.clone().unwrap();
        assert_eq!(v.asset.as_deref(), Some("bancada"));
        assert_eq!(v.color.as_deref(), Some("#ff8000"));
        assert_eq!((v.scale, v.yaw), (Some(1.5), Some(90.0)));
        assert_eq!(field(&d, "p", "visual.reset").value, Value::Bool(true));
        apply(&mut d, "p", "visual.reset", Value::Null);
        // The look goes back to the default; the rotation (its place on the floor) stays.
        let kept = object(&d, "p").visual.clone().unwrap();
        assert_eq!(
            kept,
            VisualDef {
                yaw: Some(90.0),
                ..VisualDef::default()
            }
        );
        assert_eq!(field(&d, "p", "visual.reset").value, Value::Bool(false));
        // Typed: brought to [0, 360); 0 clears it (and the whole `visual`).
        apply(&mut d, "p", "visual.yaw", Value::Float(-90.0));
        assert_eq!(object(&d, "p").visual.as_ref().unwrap().yaw, Some(270.0));
        apply(&mut d, "p", "visual.yaw", Value::Float(720.0));
        assert!(object(&d, "p").visual.is_none());
        assert!(edit(d.model(), "p", "visual.color", &Value::Text("verde".into())).is_err());
        apply(&mut d, "p", "name", Value::Text("maquina".into()));
        assert!(
            d.model()
                .connections
                .contains(&("q".into(), "maquina".into()))
        );
        apply(&mut d, "run", "stop_time", Value::Float(50.0));
        apply(&mut d, "run", "replications", Value::Int(3));
        assert_eq!(
            (d.model().run.stop_time, d.model().run.replications),
            (50.0, 3)
        );
        assert!(edit(d.model(), "nada", "name", &Value::Text("x".into())).is_err());
    }

    #[test]
    fn dispatcher_team_and_separator_mode() {
        let mut d = doc();
        for (k, n) in [("Dispatcher", "desp"), ("Separator", "sep")] {
            d.apply(Command::Add(Box::new(template(k, n, [0.0; 3]).unwrap())))
                .unwrap();
        }
        apply(&mut d, "desp", "team.emp", Value::Bool(true));
        assert_eq!(
            field(&d, "desp", "team").value,
            Value::List(vec!["emp".into()])
        );
        apply(&mut d, "desp", "team.emp", Value::Bool(false));
        assert_eq!(field(&d, "desp", "team").value, Value::List(vec![]));
        apply(&mut d, "sep", "mode", Value::Text("Split".into()));
        apply(&mut d, "sep", "mode.copies", Value::Int(4));
        assert_eq!(field(&d, "sep", "mode.copies").value, Value::Int(4));
        apply(&mut d, "sep", "mode", Value::Text("Unpack".into()));
        assert_eq!(field(&d, "sep", "mode").value, Value::Text("Unpack".into()));
        apply(&mut d, "sep", "mode", Value::Text("SplitBy".into()));
        apply(&mut d, "sep", "mode.count", Value::Text("paletes".into()));
        apply(&mut d, "sep", "mode.group", Value::Text("caminhao".into()));
        assert_eq!(
            field(&d, "sep", "mode").value,
            Value::Text("SplitBy".into())
        );
        assert_eq!(
            field(&d, "sep", "mode.count").value,
            Value::Text("paletes".into())
        );
        assert_eq!(
            field(&d, "sep", "mode.group").value,
            Value::Text("caminhao".into())
        );
    }

    #[test]
    fn every_family_has_a_valid_default_and_parameters_round_trip() {
        for (i, name) in FAMILIES.iter().enumerate() {
            let d = family_default(i, 3.0);
            assert_eq!(family_index(&d), i, "{name}");
            assert_eq!(d.validate(), Ok(()), "{name}");
            let view = dist_view(&d);
            assert_eq!(parse_dist(&view.text).unwrap(), d, "{name}");
            let mut copy = d.clone();
            for p in &view.params {
                let v = if p.integer {
                    Value::Int(p.value as i64)
                } else {
                    Value::Float(p.value)
                };
                edit_dist(&mut copy, p.key, &v).unwrap();
            }
            assert_eq!(copy, d, "{name}");
        }
    }
}
