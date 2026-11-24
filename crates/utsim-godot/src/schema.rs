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
    CraneControl, Model, Movement, ObjectDef, ObjectKind, Profile, RunSettings, ShiftSpec,
    VisualDef, is_mobile,
};

use crate::catalogue;
use crate::looks;
use crate::ron_text as ron;
use crate::texto::{dx, fx, sx, tx, vx};

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

/// The title of a section in the language of the interface (the section itself stays the
/// Portuguese id above: it is what the panel remembers as open or closed).
pub fn section_title(id: &str) -> &str {
    match id {
        "Objeto" => tx("godot-secao-objeto"),
        "Posição" => tx("godot-secao-posicao"),
        "Parâmetros" => tx("godot-secao-parametros"),
        "Saída" => tx("godot-secao-saida"),
        "Quebras e turnos" => tx("godot-secao-quebras-e-turnos"),
        "Aparência" => tx("godot-secao-aparencia"),
        "Simulação" => tx("godot-secao-simulacao"),
        "Movimentação" => tx("godot-secao-movimentacao"),
        "Piso e corredores" => tx("godot-secao-piso-e-corredores"),
        "Modelo" => tx("godot-secao-modelo"),
        "Pessoa" => tx("godot-secao-pessoa"),
        other => other,
    }
}

/// Portuguese name of an object kind.
pub fn kind_label(kind: &ObjectKind) -> &'static str {
    match kind {
        ObjectKind::Source { .. } => tx("godot-objeto-chegada"),
        ObjectKind::Queue { .. } => tx("godot-objeto-espera"),
        ObjectKind::Processor { .. } => tx("godot-objeto-processo"),
        ObjectKind::Combiner { .. } => tx("godot-objeto-montagem"),
        ObjectKind::Separator { .. } => tx("godot-objeto-desmontagem"),
        ObjectKind::Sink => tx("godot-objeto-saida"),
        ObjectKind::Transporter { .. } => tx("godot-objeto-transportador"),
        ObjectKind::Crane { .. } => tx("godot-objeto-ponte-rolante"),
        ObjectKind::Dispatcher { .. } => tx("godot-objeto-despachante"),
        ObjectKind::Conveyor { .. } => tx("godot-objeto-esteira"),
        ObjectKind::Rack { .. } => tx("godot-objeto-porta-paletes"),
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

/// The names of [`FAMILIES`] in the language of the interface, in the same order.
pub fn family_names() -> Vec<String> {
    [
        "godot-dist-constante",
        "godot-dist-uniforme",
        "godot-dist-exponencial",
        "godot-dist-triangular",
        "godot-dist-normal",
        "godot-dist-lognormal",
        "godot-dist-gama",
        "godot-dist-erlang",
        "godot-dist-weibull",
        "godot-dist-beta",
        "godot-dist-log-logistica",
        "godot-dist-pearson-v",
        "godot-dist-pearson-vi",
        "godot-dist-discreta",
        "godot-dist-empirica",
        "godot-dist-poisson",
        "godot-dist-geometrica",
        "godot-dist-binomial",
        "godot-dist-binomial-negativa",
        "godot-dist-deslocada",
    ]
    .iter()
    .map(|key| tx(key).to_string())
    .collect()
}

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
        Dist::Const(v) => vec![("value", "godot-param-valor", F(v))],
        Dist::Uniform { min, max } => vec![
            ("min", "godot-param-minimo", F(min)),
            ("max", "godot-param-maximo", F(max)),
        ],
        Dist::Exp { mean } => vec![("mean", "godot-param-media", F(mean))],
        Dist::Triangular { min, mode, max } => vec![
            ("min", "godot-param-minimo", F(min)),
            ("mode", "godot-param-moda", F(mode)),
            ("max", "godot-param-maximo", F(max)),
        ],
        Dist::Normal { mean, sd } | Dist::LogNormal { mean, sd } => {
            vec![
                ("mean", "godot-param-media", F(mean)),
                ("sd", "godot-param-desvio-padrao", F(sd)),
            ]
        }
        Dist::Gamma { shape, scale }
        | Dist::Weibull { shape, scale }
        | Dist::LogLogistic { shape, scale }
        | Dist::PearsonV { shape, scale } => {
            vec![
                ("shape", "godot-param-forma", F(shape)),
                ("scale", "godot-param-escala", F(scale)),
            ]
        }
        Dist::Erlang { k, mean } => vec![
            ("k", "godot-param-fases", U(k)),
            ("mean", "godot-param-media", F(mean)),
        ],
        Dist::Beta {
            alpha,
            beta,
            min,
            max,
        } => vec![
            ("alpha", "godot-param-alfa", F(alpha)),
            ("beta", "godot-param-beta", F(beta)),
            ("min", "godot-param-minimo", F(min)),
            ("max", "godot-param-maximo", F(max)),
        ],
        Dist::PearsonVI {
            shape1,
            shape2,
            scale,
        } => vec![
            ("shape1", "godot-param-forma-1", F(shape1)),
            ("shape2", "godot-param-forma-2", F(shape2)),
            ("scale", "godot-param-escala", F(scale)),
        ],
        Dist::Poisson { mean } => vec![("mean", "godot-param-media", F(mean))],
        Dist::Geometric { p } => vec![("p", "godot-param-probabilidade", F(p))],
        Dist::Binomial { n, p } => vec![
            ("n", "godot-param-tentativas", U(n)),
            ("p", "godot-param-probabilidade", F(p)),
        ],
        Dist::NegBinomial { r, p } => {
            vec![
                ("r", "godot-param-sucessos", F(r)),
                ("p", "godot-param-probabilidade", F(p)),
            ]
        }
        Dist::Shifted { offset, .. } => vec![("offset", "godot-param-deslocamento", F(offset))],
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
                label: tx(label),
                value: *v,
                integer: false,
            },
            ParamRef::U(v) => Param {
                key,
                label: tx(label),
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
    let d: Dist =
        ron::parse(text).map_err(|e| fx("godot-schema-distribuicao_invalida", &[("e", vx(&e))]))?;
    d.validate()?;
    Ok(d)
}

/// Edits `d` at `sub`: `family` (index), a parameter key (number), `inner` (RON of the
/// distribution a `Deslocada` wraps), or `""`/`text` (RON of the whole distribution).
fn edit_dist(d: &mut Dist, sub: &str, v: &Value) -> Result<(), String> {
    match sub {
        "family" => {
            let i = v.as_int().ok_or(tx("godot-schema-familia_invalida"))?;
            let i = usize::try_from(i)
                .ok()
                .filter(|i| *i < FAMILIES.len())
                .ok_or(tx("godot-schema-familia_invalida"))?;
            *d = switch_family(d, i);
            Ok(())
        }
        "" | "text" => {
            let t = v.as_text().ok_or(tx("godot-schema-texto_ron_esperado"))?;
            *d = parse_dist(t)?;
            Ok(())
        }
        "inner" => {
            let Dist::Shifted { dist, .. } = d else {
                return Err(tx("godot-schema-so_a_distribuicao_deslocada_tem").into());
            };
            let t = v.as_text().ok_or(tx("godot-schema-texto_ron_esperado"))?;
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
            Err(fx(
                "godot-schema-a_distribuicao_nao_tem_o",
                &[("key", vx(&key))],
            ))
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

/// A choice label: a key of `godot.ftl` is translated, a name of the model stays as it is.
fn shown(label: &str) -> String {
    if label.starts_with("godot-") && crate::texto::TEXTOS.tem(label) {
        fx(label, &[])
    } else {
        label.to_string()
    }
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
    .choices(choices.iter().map(|(v, l)| choice(v, &shown(l))).collect())
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
    ("FirstAvailable", "godot-opcao-primeira-livre"),
    ("RoundRobin", "godot-opcao-rodizio"),
    ("Random", "godot-opcao-aleatoria"),
    ("Weighted", "godot-campo-pesos"),
    ("ByLabel", "godot-opcao-por-rotulo"),
    ("Script", "godot-campo-script"),
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
    ("", "godot-opcao-padrao-do-modelo"),
    ("Legacy", "godot-opcao-legado-linha-reta-ou-rede"),
    ("Free", "godot-opcao-livre-menor-caminho-no-piso"),
    ("Network", "godot-opcao-pela-rede-de-caminhos"),
    ("Aisles", "godot-opcao-pelos-corredores"),
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
        _ => {
            return Err(fx(
                "godot-schema-modo_de_movimentacao_desconhecido",
                &[("name", vx(&name))],
            ));
        }
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
            tx("godot-campo-movimentacao"),
            movement_name(movement),
            &MOVEMENT_CHOICES,
        )
        .help(tx("godot-ajuda-como-o-recurso-anda-o")),
        Field::new(
            s,
            "clearance",
            tx("godot-campo-folga-dos-obstaculos"),
            FieldType::Float,
            clearance.map_or(Value::Null, Value::Float),
        )
        .optional(clearance.is_some())
        .min(0.0)
        .unit("m")
        .help(&fx(
            "godot-ajuda-folga-papel",
            &[("m", dx(format!("{by_role:.2}")))],
        )),
    ]
}

/// The access point of a fixed object (`access`), in its own coordinates.
fn access_fields(obj: &ObjectDef) -> Vec<Field> {
    let s = S_MOVE;
    let mut out = vec![
        group(
            s,
            "access",
            tx("godot-campo-ponto-de-acesso-proprio"),
            obj.access.is_some(),
        )
        .help(tx("godot-ajuda-onde-os-recursos-moveis-param")),
    ];
    if let Some(a) = obj.access {
        out.push(
            float(s, "access.x", tx("godot-campo-x-no-objeto"), a[0])
                .depth(1)
                .unit("m"),
        );
        out.push(
            float(s, "access.y", tx("godot-campo-y-no-objeto"), a[1])
                .depth(1)
                .unit("m"),
        );
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
            tx("godot-campo-dimensoes"),
            FieldType::Info,
            Value::Text(text),
        )
    };
    let [l, w] = model.footprint(obj).unwrap_or([0.0, 0.0]);
    match &obj.kind {
        ObjectKind::Conveyor { .. } => {
            return vec![info(fx(
                "godot-dim-esteira",
                &[("l", sx(metres(l))), ("w", sx(metres(w)))],
            ))];
        }
        ObjectKind::Rack {
            levels,
            level_height,
            ..
        } => {
            return vec![info(fx(
                "godot-dim-porta-paletes",
                &[
                    ("l", sx(metres(l))),
                    ("w", sx(metres(w))),
                    ("h", sx(metres(*levels as f64 * level_height))),
                ],
            ))];
        }
        _ => {}
    }
    let mut out = vec![
        group(
            s,
            "size",
            tx("godot-campo-dimensoes-proprias"),
            obj.size.is_some(),
        )
        .help(tx("godot-ajuda-comprimento-largura-e-altura-do")),
    ];
    match obj.size {
        Some(d) => {
            let labels = if matches!(obj.kind, ObjectKind::Crane { .. }) {
                [
                    ("x", "godot-dim-comprimento-do-caminho"),
                    ("y", "godot-dim-vao"),
                    ("z", "godot-dim-altura-do-caminho"),
                ]
            } else {
                [
                    ("x", "godot-dim-comprimento"),
                    ("y", "godot-dim-largura"),
                    ("z", "godot-dim-altura"),
                ]
            };
            for (i, (axis, label)) in labels.iter().enumerate() {
                out.push(
                    float(s, &format!("size.{axis}"), tx(label), d[i])
                        .depth(1)
                        .min(0.01)
                        .unit("m"),
                );
            }
        }
        None => out.push(info(fx(
            "godot-dim-padrao",
            &[("l", sx(metres(l))), ("w", sx(metres(w)))],
        ))),
    }
    if matches!(obj.kind, ObjectKind::Queue { .. }) {
        out.push(
            group(
                s,
                "area_per_item",
                tx("godot-campo-capacidade-pela-area"),
                obj.area_per_item.is_some(),
            )
            .help(tx("godot-ajuda-a-capacidade-da-espera-sai")),
        );
        if let Some(a) = obj.area_per_item {
            out.push(
                float(s, "area_per_item", tx("godot-campo-area-por-item"), a)
                    .depth(1)
                    .min(0.01)
                    .unit("m²"),
            );
            let fits = model
                .queue_capacity(obj)
                .map_or(tx("godot-campo-sem-limite").to_string(), |n| {
                    fx("godot-campo-n-itens", &[("n", vx(&n))])
                });
            out.push(
                Field::new(
                    s,
                    "area_per_item.info",
                    tx("godot-campo-cabem"),
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
            .ok_or_else(|| tx("godot-schema-modo_de_movimentacao_esperado").to_string())
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
                dist_field(
                    s,
                    "interarrival",
                    tx("godot-campo-tempo-entre-chegadas"),
                    interarrival,
                )
                .help(tx("godot-ajuda-intervalo-entre-duas-chegadas-seguidas")),
            );
            out.push(boolean(
                s,
                "first_at_zero",
                tx("godot-campo-primeira-chegada-em-t-0"),
                *first_at_zero,
            ));
            out.push(
                Field::new(
                    s,
                    "max_arrivals",
                    tx("godot-campo-maximo-de-chegadas"),
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
                tx("godot-campo-se-a-saida-estiver-bloqueada"),
                &ron::debug(on_blocked),
                &[
                    ("Wait", "godot-opcao-esperar"),
                    ("Drop", "godot-opcao-descartar-perda"),
                ],
            ));
            out.push(
                Field::new(
                    s,
                    "labels",
                    tx("godot-campo-rotulos"),
                    FieldType::Ron,
                    Value::Text(ron::labels(labels)),
                )
                .help(tx("godot-ajuda-valor-sorteado-para-cada-item")),
            );
            out.push(
                Field::new(
                    s,
                    "schedule",
                    tx("godot-campo-chegadas-por-periodo"),
                    FieldType::Ron,
                    schedule
                        .as_ref()
                        .map_or(Value::Null, |v| Value::Text(ron::schedule(v))),
                )
                .optional(schedule.is_some())
                .help(tx("godot-ajuda-ciclo-e-periodos-com-o")),
            );
            // The choices are the model's item types (the table editor's "tipo" column).
            out.push(
                Field::new(
                    s,
                    "table",
                    tx("godot-campo-tabela-de-chegadas"),
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
                .help(tx("godot-ajuda-horario-instante-quantidade-tipo-e")),
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
                    tx("godot-campo-capacidade-limitada"),
                    FieldType::Int,
                    capacity.map_or(Value::Null, |v| Value::Int(v as i64)),
                )
                .optional(capacity.is_some())
                .min(1.0)
                .unit("itens")
                .help(tx("godot-ajuda-desligada-a-espera-nao-tem")),
            );
            out.push(
                pick(
                    s,
                    "discipline",
                    tx("godot-campo-politica-de-acesso"),
                    &access_policy(*discipline, order.as_ref()),
                    &[
                        ("Fifo", "godot-opcao-fifo-primeiro-a-entrar-primeiro"),
                        ("Lifo", "godot-opcao-lifo-ultimo-a-entrar-primeiro"),
                        ("Desc", "godot-opcao-prioridade-maior-rotulo-primeiro"),
                        ("Asc", "godot-opcao-prioridade-menor-rotulo-primeiro-prazo"),
                    ],
                )
                .help(tx("godot-ajuda-qual-item-sai-primeiro-pela")),
            );
            if let Some(o) = order {
                out.push(
                    Field::new(
                        s,
                        "order.label",
                        tx("godot-campo-rotulo-da-prioridade"),
                        FieldType::Text,
                        Value::Text(o.label().to_owned()),
                    )
                    .depth(1),
                );
            }
            out.push(
                group(s, "batch", tx("godot-campo-lote"), batch.is_some())
                    .help(tx("godot-ajuda-segura-os-itens-e-libera")),
            );
            if let Some(b) = batch {
                out.push(
                    int(
                        s,
                        "batch.size",
                        tx("godot-campo-itens-por-lote"),
                        b.size as u64,
                    )
                    .min(1.0)
                    .depth(1),
                );
                out.push(
                    float(
                        s,
                        "batch.max_wait",
                        tx("godot-campo-espera-maxima"),
                        b.max_wait,
                    )
                    .min(0.0)
                    .depth(1)
                    .help(tx("godot-ajuda-libera-um-lote-incompleto-depois")),
                );
                out.push(
                    boolean(
                        s,
                        "batch.flush",
                        tx("godot-campo-fecha-a-entrada-ate-o"),
                        b.flush,
                    )
                    .depth(1),
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
                int(
                    s,
                    "capacity",
                    tx("godot-campo-capacidade"),
                    *capacity as u64,
                )
                .min(1.0)
                .unit("itens"),
            );
            out.push(opt_dist_field(
                s,
                "setup",
                tx("godot-campo-preparacao-setup"),
                setup.as_ref(),
            ));
            out.push(
                Field::new(
                    s,
                    "changeover",
                    tx("godot-campo-troca-de-produto"),
                    FieldType::Ron,
                    Value::Text(ron::option(changeover, ron::changeover)),
                )
                .help(tx("godot-ajuda-preparacao-so-quando-o-produto")),
            );
            out.push(dist_field(
                s,
                "process",
                tx("godot-campo-tempo-de-processamento"),
                process,
            ));
            out.push(
                Field::new(
                    s,
                    "recipe",
                    tx("godot-campo-tempo-por-receita"),
                    FieldType::Ron,
                    Value::Text(ron::option(recipe, ron::recipe)),
                )
                .help(tx("godot-ajuda-tempo-de-processamento-que-depende")),
            );
            out.push(
                group(
                    s,
                    "operators",
                    tx("godot-campo-operadores"),
                    operators.is_some(),
                )
                .help(tx("godot-ajuda-quem-opera-esta-maquina-marque")),
            );
            if let Some(op) = operators {
                let mut names = vec![];
                for n in mobile_names(model) {
                    names.push(choice(&n, &n));
                }
                if !names.iter().any(|c| c.value == op.team) {
                    names.push(choice(
                        &op.team,
                        &fx("godot-campo-nao-existe", &[("nome", vx(&op.team))]),
                    ));
                }
                out.push(
                    Field::new(
                        s,
                        "operators.team",
                        tx("godot-campo-quem-opera"),
                        FieldType::Choice,
                        Value::Text(op.team.clone()),
                    )
                    .choices(names)
                    .depth(1),
                );
                out.push(
                    int(
                        s,
                        "operators.count",
                        tx("godot-campo-quantos-por-peca"),
                        op.count as u64,
                    )
                    .min(1.0)
                    .help(tx("godot-ajuda-operadores-que-a-peca-precisa"))
                    .depth(1),
                );
                out.push(
                    float(
                        s,
                        "operators.team_speedup",
                        tx("godot-campo-ganho-da-equipe"),
                        op.team_speedup.unwrap_or(1.0),
                    )
                    .help(tx("godot-ajuda-quantas-vezes-mais-rapido-a"))
                    .depth(1),
                );
                out.push(
                    boolean(
                        s,
                        "operators.setup",
                        tx("godot-campo-durante-a-preparacao"),
                        op.setup,
                    )
                    .depth(1),
                );
                out.push(
                    boolean(
                        s,
                        "operators.process",
                        tx("godot-campo-durante-o-processamento"),
                        op.process,
                    )
                    .depth(1),
                );
                out.push(
                    Field::new(
                        s,
                        "also",
                        tx("godot-campo-outras-equipes-ao-mesmo-tempo"),
                        FieldType::Ron,
                        Value::Text(ron::operator_uses(also)),
                    )
                    .help(tx("godot-ajuda-equipes-que-precisam-estar-juntas"))
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
                Field::new(
                    s,
                    "components",
                    tx("godot-campo-componentes-por-entrada"),
                    FieldType::Ron,
                    Value::Text(ron::usizes(components)),
                )
                .help(tx("godot-ajuda-a-entrada-1-traz-o")),
            );
            out.push(pick(
                s,
                "mode",
                tx("godot-campo-modo"),
                &ron::debug(mode),
                &[
                    ("Pack", "godot-opcao-embalar-so-o-conteiner-sai"),
                    ("Join", "godot-opcao-juntar-componentes-destruidos"),
                    ("Batch", "godot-opcao-lote-todos-saem"),
                ],
            ));
            out.push(opt_dist_field(
                s,
                "setup",
                tx("godot-campo-preparacao-setup"),
                setup.as_ref(),
            ));
            out.push(dist_field(
                s,
                "process",
                tx("godot-campo-tempo-de-processamento"),
                process,
            ));
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
                tx("godot-campo-modo"),
                chosen,
                &[
                    ("Unpack", "godot-opcao-desembalar"),
                    ("Split", "godot-opcao-dividir-em-copias"),
                    ("SplitBy", "godot-opcao-dividir-pelo-rotulo"),
                ],
            ));
            if let Some(n) = split.and_then(|n| n.parse::<u64>().ok()) {
                out.push(
                    int(s, "mode.copies", tx("godot-campo-copias"), n)
                        .min(1.0)
                        .depth(1),
                );
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
            out.push(dist_field(
                s,
                "process",
                tx("godot-campo-tempo-de-processamento"),
                process,
            ));
        }
        ObjectKind::Sink => {
            out.push(Field::new(
                s,
                "info",
                tx("godot-campo-sem-parametros"),
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
                float(s, "speed", tx("godot-campo-velocidade-maxima"), *speed)
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
                    .help(tx("godot-ajuda-desligada-a-velocidade-muda-na")),
                );
            }
            out.push(
                int(
                    s,
                    "capacity",
                    tx("godot-campo-capacidade"),
                    *capacity as u64,
                )
                .min(1.0)
                .unit("itens"),
            );
            out.push(dist_field(
                s,
                "load_time",
                tx("godot-campo-tempo-de-carga"),
                load_time,
            ));
            out.push(dist_field(
                s,
                "unload_time",
                tx("godot-campo-tempo-de-descarga"),
                unload_time,
            ));
            out.push(boolean(
                s,
                "home",
                tx("godot-campo-voltar-a-base-quando-ocioso"),
                *home,
            ));
            out.push(boolean(
                s,
                "agv",
                tx("godot-campo-agv-controle-de-trafego-na"),
                *agv,
            ));
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
                        tx("godot-campo-trajeto-desenhado"),
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
                            tx("godot-campo-editar-nos"),
                            FieldType::Action,
                            Value::Bool(true),
                        )
                        .help(tx("godot-ajuda-abre-a-ferramenta-de-desenho")),
                    );
                }
                None => out.push(
                    float(s, "length", tx("godot-campo-comprimento"), *length)
                        .min(0.001)
                        .unit("m"),
                ),
            }
            out.push(
                float(s, "speed", tx("godot-campo-velocidade"), *speed)
                    .min(0.001)
                    .unit("m/u.t."),
            );
            out.push(
                float(
                    s,
                    "item_length",
                    tx("godot-campo-comprimento-por-item"),
                    *item_length,
                )
                .min(0.001)
                .unit("m"),
            );
            out.push(boolean(
                s,
                "accumulating",
                tx("godot-campo-acumula-itens-quando-a-saida"),
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
            out.push(int(s, "bays", tx("godot-campo-colunas"), *bays as u64).min(1.0));
            out.push(int(s, "levels", tx("godot-campo-niveis"), *levels as u64).min(1.0));
            out.push(opt_dist_field(
                s,
                "dwell",
                tx("godot-campo-permanencia-minima"),
                dwell.as_ref(),
            ));
            let abc = match placement {
                utsim_objects::RackPlacement::Abc { a, b } => Some((*a, *b)),
                _ => None,
            };
            out.push(pick(
                s,
                "placement",
                tx("godot-campo-regra-de-ocupacao"),
                if abc.is_some() {
                    "Abc".to_string()
                } else {
                    ron::debug(placement)
                }
                .as_str(),
                &[
                    ("FirstFree", "godot-opcao-primeira-livre"),
                    ("Random", "godot-opcao-aleatoria"),
                    ("Abc", "godot-opcao-curva-abc-rotulo-classe"),
                ],
            ));
            if let Some((a, b)) = abc {
                out.push(
                    float(
                        s,
                        "placement.a",
                        tx("godot-campo-fracao-das-posicoes-da-classe"),
                        a,
                    )
                    .min(0.0)
                    .depth(1),
                );
                out.push(
                    float(
                        s,
                        "placement.b",
                        tx("godot-campo-fracao-das-posicoes-da-classe-2"),
                        b,
                    )
                    .min(0.0)
                    .depth(1),
                );
            }
            out.push(
                float(
                    s,
                    "bay_width",
                    tx("godot-campo-largura-da-coluna"),
                    *bay_width,
                )
                .min(0.01)
                .unit("m"),
            );
            out.push(
                float(
                    s,
                    "level_height",
                    tx("godot-campo-altura-do-nivel"),
                    *level_height,
                )
                .min(0.01)
                .unit("m"),
            );
            out.push(Field::new(
                s,
                "info",
                tx("godot-campo-capacidade"),
                FieldType::Info,
                Value::Text(fx("godot-campo-n-posicoes", &[("n", vx(&(bays * levels)))])),
            ));
        }
        ObjectKind::Crane { .. } => out.extend(crane_fields(model, obj)),
        ObjectKind::Dispatcher { team, rule, .. } => {
            let mut choices: Vec<Choice> = transporter_names(model)
                .iter()
                .map(|n| choice(n, n))
                .collect();
            for m in team {
                if !choices.iter().any(|c| &c.value == m) {
                    choices.push(choice(m, &fx("godot-campo-nao-existe", &[("nome", vx(m))])));
                }
            }
            out.push(
                Field::new(
                    s,
                    "team",
                    tx("godot-campo-equipe"),
                    FieldType::Set,
                    Value::List(team.clone()),
                )
                .choices(choices)
                .help(tx("godot-ajuda-transportadores-que-o-despachante-comanda")),
            );
            out.push(pick(
                s,
                "rule",
                tx("godot-campo-regra-de-escolha"),
                &ron::debug(rule),
                &[
                    ("FirstAvailable", "godot-opcao-primeiro-livre"),
                    ("RoundRobin", "godot-opcao-rodizio"),
                    ("Nearest", "godot-opcao-mais-proximo"),
                ],
            ));
        }
    }
    out
}

/// The axes of a crane, as the inspector lists them: field, label, what it moves.
const CRANE_AXES: [(&str, &str, &str); 3] = [
    ("bridge", "Ponte", "ao longo do caminho"),
    ("trolley", "Carro", "ao longo da viga, no vão"),
    ("hoist", "Gancho", "subindo e descendo a carga"),
];

/// Fields of an overhead crane (`ObjectKind::Crane`): the three axes, the hook, the
/// handling times, who runs it, the runway it may share and the area under the load.
/// The runway length and the span are the object's own dimensions (Posição).
fn crane_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    let s = S_PARAMS;
    let ObjectKind::Crane {
        bridge,
        trolley,
        hoist,
        hook_height,
        attach,
        detach,
        capacity_t,
        motion,
        weight_label,
        home,
        load_zone,
        operator,
        bridge_gap,
        keep_clear,
    } = &obj.kind
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for ((path, label, what), axis) in CRANE_AXES.iter().zip([bridge, trolley, hoist]) {
        out.push(Field::new(
            s,
            &format!("{path}.info"),
            label,
            FieldType::Info,
            Value::Text(fx("godot-campo-movimento", &[("o_que", vx(&what))])),
        ));
        out.push(
            float(
                s,
                &format!("{path}.speed"),
                tx("godot-campo-velocidade"),
                axis.speed,
            )
            .depth(1)
            .min(0.001)
            .unit("m/u.t."),
        );
        for (sub, sub_label, v) in [
            ("acceleration", "Aceleração", axis.acceleration),
            ("deceleration", "Desaceleração", axis.deceleration),
        ] {
            out.push(
                Field::new(
                    s,
                    &format!("{path}.{sub}"),
                    sub_label,
                    FieldType::Float,
                    v.map_or(Value::Null, Value::Float),
                )
                .optional(v.is_some())
                .depth(1)
                .min(0.001)
                .help(tx("godot-ajuda-desligada-a-velocidade-muda-na")),
            );
        }
    }
    out.push(
        pick(
            s,
            "motion",
            tx("godot-campo-ordem-dos-movimentos"),
            &ron::debug(motion),
            &[
                ("Safe", "godot-opcao-segura-sobe-anda-desce"),
                ("Sequential", "godot-opcao-um-eixo-de-cada-vez"),
                ("Parallel", "godot-opcao-os-tres-juntos"),
            ],
        )
        .help(tx("godot-ajuda-segura-o-gancho-sobe-a")),
    );
    out.push(
        float(
            s,
            "hook_height",
            tx("godot-campo-altura-de-passagem-do-gancho"),
            *hook_height,
        )
        .min(0.0)
        .unit("m"),
    );
    out.push(
        float(s, "capacity_t", tx("godot-campo-capacidade"), *capacity_t)
            .min(0.001)
            .unit("t"),
    );
    out.push(
        Field::new(
            s,
            "weight_label",
            tx("godot-campo-rotulo-do-peso-do-item"),
            FieldType::Text,
            Value::Text(weight_label.clone()),
        )
        .help(tx("godot-ajuda-rotulo-com-o-peso-de")),
    );
    out.push(dist_field(
        s,
        "attach",
        tx("godot-campo-tempo-de-engate"),
        attach,
    ));
    out.push(dist_field(
        s,
        "detach",
        tx("godot-campo-tempo-de-desengate"),
        detach,
    ));
    out.push(boolean(
        s,
        "home",
        tx("godot-campo-voltar-a-posicao-propria-quando"),
        *home,
    ));
    let (control, team) = match operator {
        CraneControl::Cabin => ("Cabin", None),
        CraneControl::Pendant { team } => ("Pendant", Some(team)),
        CraneControl::Radio { team } => ("Radio", Some(team)),
    };
    out.push(
        pick(
            s,
            "operator",
            tx("godot-campo-controle"),
            control,
            &[
                ("Cabin", "godot-opcao-cabine-operador-nao-modelado"),
                ("Pendant", "godot-opcao-botoeira-a-pessoa-anda-com"),
                ("Radio", "godot-opcao-radio-a-pessoa-encontra-a"),
            ],
        )
        .help(tx("godot-ajuda-na-botoeira-a-ponte-nao")),
    );
    if let Some(team) = team {
        // People and dispatchers run the crane; not other cranes.
        let mut choices: Vec<Choice> = model
            .objects
            .iter()
            .filter(|o| {
                matches!(
                    o.kind,
                    ObjectKind::Transporter { .. } | ObjectKind::Dispatcher { .. }
                )
            })
            .map(|o| choice(&o.name, &o.name))
            .collect();
        if !choices.iter().any(|c| &c.value == team) {
            choices.push(choice(
                team,
                &fx("godot-campo-nao-existe", &[("nome", vx(team))]),
            ));
        }
        out.push(
            Field::new(
                s,
                "operator.team",
                tx("godot-campo-equipe-do-operador"),
                FieldType::Choice,
                Value::Text(team.clone()),
            )
            .choices(choices)
            .depth(1),
        );
    }
    out.push(
        float(
            s,
            "load_zone",
            tx("godot-campo-meia-largura-da-area-sob"),
            *load_zone,
        )
        .min(0.0)
        .unit("m")
        .help(tx("godot-ajuda-meio-lado-do-quadrado-sob")),
    );
    out.push(
        boolean(
            s,
            "keep_clear",
            tx("godot-campo-afastar-as-pessoas-do-caminho"),
            *keep_clear,
        )
        .help(tx("godot-ajuda-antes-de-andar-carregada-a")),
    );
    out.push(
        float(
            s,
            "bridge_gap",
            tx("godot-campo-folga-entre-pontes"),
            *bridge_gap,
        )
        .min(0.0)
        .unit("m")
        .help(tx("godot-ajuda-distancia-mantida-entre-os-ganchos")),
    );
    let lanes = model.crane_lanes();
    let lane_text = match lanes.get(obj.name.as_str()) {
        Some(lane) => {
            let mut others: Vec<&str> = lanes
                .iter()
                .filter(|(n, l)| l.id == lane.id && **n != obj.name.as_str())
                .map(|(n, _)| *n)
                .collect();
            others.sort_unstable();
            fx(
                "godot-campo-divide-caminho",
                &[("com", sx(others.join(", "))), ("m", sx(metres(lane.gap)))],
            )
        }
        None if obj.size.is_none() => {
            "Ligue as dimensões (Posição): comprimento do caminho × vão.".to_string()
        }
        None => "Sozinha no caminho".to_string(),
    };
    out.push(Field::new(
        s,
        "lane.info",
        tx("godot-campo-caminho"),
        FieldType::Info,
        Value::Text(lane_text),
    ));
    out
}

/// The edit of a crane field, or `None` for other objects and fields.
fn edit_crane(
    model: &Model,
    obj: &ObjectDef,
    head: &str,
    rest: &str,
    value: &Value,
) -> Option<Result<Command, String>> {
    let ObjectKind::Crane {
        attach,
        detach,
        operator,
        ..
    } = &obj.kind
    else {
        return None;
    };
    // The fields every object has (downtime, shifts, transport…) go on to the others.
    const OWN: [&str; 14] = [
        "bridge",
        "trolley",
        "hoist",
        "hook_height",
        "attach",
        "detach",
        "capacity_t",
        "motion",
        "weight_label",
        "home",
        "load_zone",
        "operator",
        "bridge_gap",
        "keep_clear",
    ];
    if !OWN.contains(&head) {
        return None;
    }
    let name = obj.name.as_str();
    let axis_of = |head: &str| -> Option<utsim_objects::Axis> {
        let ObjectKind::Crane {
            bridge,
            trolley,
            hoist,
            ..
        } = &obj.kind
        else {
            return None;
        };
        match head {
            "bridge" => Some(*bridge),
            "trolley" => Some(*trolley),
            "hoist" => Some(*hoist),
            _ => None,
        }
    };
    let axis_text = |a: &utsim_objects::Axis| {
        let opt = |key: &str, v: Option<f64>| {
            v.map_or(String::new(), |v| {
                format!(", {key}: Some({})", ron::float(v))
            })
        };
        format!(
            "(speed: {}{}{})",
            ron::float(a.speed),
            opt("acceleration", a.acceleration),
            opt("deceleration", a.deceleration)
        )
    };
    let what = if rest.is_empty() {
        head.to_string()
    } else {
        format!("{head}.{rest}")
    };
    let w = what.as_str();
    Some((|| -> Result<Command, String> {
        if let Some(mut a) = axis_of(head) {
            match rest {
                "speed" => {
                    let v = f64_of(value, w)?;
                    if !(v.is_finite() && v > 0.0) {
                        return Err(tx("godot-schema-a_velocidade_deve_ser_maior").into());
                    }
                    a.speed = v;
                }
                "acceleration.on" => a.acceleration = bool_of(value, w)?.then_some(1.0),
                "deceleration.on" => a.deceleration = bool_of(value, w)?.then_some(1.0),
                "acceleration" => a.acceleration = opt_f64(value, w)?,
                "deceleration" => a.deceleration = opt_f64(value, w)?,
                _ => return Err(fx("godot-schema-campo_desconhecido", &[("w", vx(&w))])),
            }
            return Ok(set(name, head, axis_text(&a)));
        }
        match (head, rest) {
            ("attach" | "detach", _) => {
                let mut d = if head == "attach" { attach } else { detach }.clone();
                edit_dist(&mut d, rest, value)?;
                Ok(set(name, head, ron::dist(&d)))
            }
            ("hook_height" | "capacity_t" | "load_zone" | "bridge_gap", "") => {
                let v = f64_of(value, w)?;
                if !v.is_finite() || v < 0.0 || (head == "capacity_t" && v <= 0.0) {
                    return Err(format!("{w}: número maior ou igual a zero"));
                }
                Ok(set(name, head, ron::float(v)))
            }
            ("home" | "keep_clear", "") => Ok(set(name, head, bool_of(value, w)?.to_string())),
            ("weight_label", "") => {
                let t = value
                    .as_text()
                    .ok_or(tx("godot-schema-rotulo_esperado"))?
                    .trim();
                if t.is_empty() {
                    return Err(tx("godot-schema-o_rotulo_do_peso_nao").into());
                }
                Ok(set(name, head, ron::string(t)))
            }
            ("motion", "") => match value.as_text() {
                Some(m @ ("Safe" | "Sequential" | "Parallel")) => Ok(set(name, head, m.into())),
                _ => Err(tx("godot-schema-ordem_safe_sequential_ou_parallel").into()),
            },
            ("operator", "") => {
                let first = || {
                    model.objects.iter().find_map(|o| {
                        matches!(
                            o.kind,
                            ObjectKind::Transporter { .. } | ObjectKind::Dispatcher { .. }
                        )
                        .then_some(o.name.as_str())
                    })
                };
                let text = match value.as_text() {
                    Some("Cabin") => "Cabin".to_string(),
                    Some(k @ ("Pendant" | "Radio")) => {
                        let team = operator
                            .team()
                            .or_else(first)
                            .ok_or(tx("godot-schema-crie_antes_uma_pessoa_ou"))?;
                        format!("{k}(team: {})", ron::string(team))
                    }
                    _ => return Err(tx("godot-schema-controle_cabin_pendant_ou_radio").into()),
                };
                Ok(set(name, head, text))
            }
            ("operator", "team") => {
                let team = value.as_text().ok_or("nome esperado")?.trim();
                if team.is_empty() {
                    return Err(tx("godot-schema-escolha_a_equipe_do_operador").into());
                }
                let kind = match operator {
                    CraneControl::Radio { .. } => "Radio",
                    CraneControl::Pendant { .. } => "Pendant",
                    CraneControl::Cabin => {
                        return Err(tx("godot-schema-escolha_antes_botoeira_ou_radio").into());
                    }
                };
                Ok(set(
                    name,
                    head,
                    format!("{kind}(team: {})", ron::string(team)),
                ))
            }
            _ => Err(format!("{name} não tem o campo \"{w}\"")),
        }
    })())
}

fn output_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    let s = S_OUTPUT;
    let mut out = Vec::new();
    out.push(
        Field::new(
            s,
            "output",
            tx("godot-campo-regra-de-saida"),
            FieldType::Choice,
            Value::Text(output_id(&obj.output).into()),
        )
        .choices(OUTPUT_RULES.iter().map(|(v, l)| choice(v, l)).collect())
        .help(tx("godot-ajuda-para-qual-ligacao-vai-cada")),
    );
    match &obj.output {
        OutputRule::Weighted(w) => {
            let n = outputs(model, &obj.name);
            let mut f = Field::new(
                s,
                "output.weights",
                tx("godot-campo-pesos"),
                FieldType::Ron,
                Value::Text(ron::floats(w)),
            )
            .depth(1)
            .help(tx("godot-ajuda-peso-de-cada-saida-na"));
            if w.len() != n {
                f.help = fx(
                    "godot-campo-pesos-e-saidas",
                    &[
                        ("pesos", vx(&w.len())),
                        ("n", vx(&n)),
                        ("ajuda", sx(f.help.clone())),
                    ],
                );
            }
            out.push(f);
        }
        OutputRule::ByLabel(label) => out.push(
            Field::new(
                s,
                "output.label",
                tx("godot-campo-rotulo-com-o-numero-da"),
                FieldType::Text,
                Value::Text(label.clone()),
            )
            .depth(1),
        ),
        OutputRule::Script(src) => out.push(
            Field::new(
                s,
                "output.script",
                tx("godot-campo-script"),
                FieldType::Script,
                Value::Text(src.clone()),
            )
            .depth(1)
            .help(tx("godot-ajuda-devolve-o-numero-da-porta")),
        ),
        _ => {}
    }
    let mut choices = vec![choice("", tx("godot-campo-nenhum-passagem-direta"))];
    for n in mobile_names(model) {
        choices.push(choice(&n, &n));
    }
    let current = obj.transport.clone().unwrap_or_default();
    if !choices.iter().any(|c| c.value == current) {
        choices.push(choice(
            &current,
            &fx("godot-campo-nao-existe", &[("nome", vx(&current))]),
        ));
    }
    out.push(
        Field::new(
            s,
            "transport",
            tx("godot-campo-transporte-de-saida"),
            FieldType::Choice,
            Value::Text(current),
        )
        .choices(choices)
        .help(tx("godot-ajuda-transportador-ou-despachante-que-leva")),
    );
    out.push(
        group(
            s,
            "transfer",
            tx("godot-campo-transferencia-pela-distancia"),
            obj.transfer.is_some(),
        )
        .help(tx("godot-ajuda-os-itens-levam-atraso-distancia")),
    );
    if let Some(t) = &obj.transfer {
        out.push(
            float(s, "transfer.speed", tx("godot-campo-velocidade"), t.speed)
                .min(0.001)
                .depth(1)
                .unit("m/u.t."),
        );
        out.push(
            float(s, "transfer.delay", tx("godot-campo-atraso"), t.delay)
                .min(0.0)
                .depth(1),
        );
        out.push(
            pick(
                s,
                "transfer.path",
                tx("godot-campo-caminho"),
                &format!("{:?}", t.path),
                &[
                    ("Straight", "godot-opcao-reta"),
                    ("Network", "godot-opcao-pela-rede"),
                ],
            )
            .depth(1),
        );
    }
    out.push(
        Field::new(
            s,
            "on_entry",
            tx("godot-campo-script-ao-entrar-um-item"),
            FieldType::Script,
            obj.on_entry.clone().map_or(Value::Null, Value::Text),
        )
        .optional(obj.on_entry.is_some())
        .help(tx("godot-ajuda-roda-para-cada-item-que")),
    );
    out
}

/// Choices of `on_end` (what happens to work in progress when a shift ends).
const ON_END: [(&str, &str); 2] = [
    ("Pause", "godot-turno-pausar"),
    ("Finish", "godot-turno-terminar"),
];

fn on_end_id(e: ShiftEnd) -> &'static str {
    match e {
        ShiftEnd::Pause => "Pause",
        ShiftEnd::Finish => "Finish",
    }
}

/// A size in metres with two decimals and the decimal mark of the language.
fn dx_text(v: impl Into<f64>) -> String {
    utsim_idiomas::marca_decimal(&format!("{:.2}", v.into())).to_string()
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
        group(
            s,
            "downtime",
            tx("godot-campo-quebras"),
            obj.downtime.is_some(),
        )
        .help(tx("godot-ajuda-falhas-aleatorias-com-o-tempo")),
    ];
    if let Some(d) = &obj.downtime {
        out.push(
            dist_field(
                s,
                "downtime.mtbf",
                tx("godot-campo-tempo-entre-falhas-mtbf"),
                &d.mtbf,
            )
            .depth(1),
        );
        out.push(
            dist_field(
                s,
                "downtime.mttr",
                tx("godot-campo-tempo-de-reparo-mttr"),
                &d.mttr,
            )
            .depth(1),
        );
    }
    out.push(
        group(s, "shifts", tx("godot-campo-turnos"), obj.shifts.is_some())
            .help(tx("godot-ajuda-janelas-fora-do-turno-repetidas")),
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
        tx("godot-campo-origem"),
        source,
        &[
            ("inline", "godot-opcao-personalizado"),
            ("calendar", "godot-opcao-calendario-do-modelo"),
        ],
    )
    .depth(1);
    from.help = if model.calendars.is_empty() {
        tx("godot-turno-sem-calendarios").into()
    } else {
        tx("godot-turno-calendario-ajuda").into()
    };
    out.push(from);
    match spec {
        ShiftSpec::Inline(t) => {
            out.push(
                float(s, "shifts.period", tx("godot-campo-periodo"), t.period)
                    .min(0.001)
                    .depth(1),
            );
            out.push(
                Field::new(
                    s,
                    "shifts.off",
                    tx("godot-campo-pausas"),
                    FieldType::Ron,
                    Value::Text(windows_text(t)),
                )
                .depth(1)
                .help(tx("godot-ajuda-inicio-e-duracao-de-cada")),
            );
            out.push(
                pick(
                    s,
                    "shifts.on_end",
                    tx("godot-campo-no-fim-do-turno"),
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
            out.push(
                pick(
                    s,
                    "shifts.calendar",
                    tx("godot-campo-calendario"),
                    name,
                    &choices,
                )
                .depth(1),
            );
            let mut ends = vec![("", "godot-turno-do-calendario")];
            ends.extend(ON_END);
            out.push(
                pick(
                    s,
                    "shifts.on_end",
                    tx("godot-campo-no-fim-do-turno"),
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
pub const DEFAULT_ASSET: &str = "godot-visual-padrao-do-tipo";

/// Library and catalogue assets as asset choices (value `""` is the default look).
pub fn asset_choices() -> Vec<Choice> {
    let mut out = vec![choice("", tx(DEFAULT_ASSET))];
    for spec in utsim_assets::Spec::defaults() {
        out.push(Choice {
            value: spec.name().into(),
            label: spec.name().into(),
            help: tx("godot-visual-gerado").into(),
            group: tx("godot-visual-grupo-biblioteca").into(),
        });
    }
    for e in catalogue::entries() {
        let size = e.tamanho_real_m;
        out.push(Choice {
            value: e.nome.clone(),
            label: e.nome.clone(),
            help: format!(
                "{}, {}\n{}\n{} ({} x {} x {} m)",
                e.autor,
                e.licenca,
                e.uso,
                e.arquivo,
                dx_text(size.0),
                dx_text(size.2),
                dx_text(size.1)
            ),
            group: tx("godot-visual-grupo-terceiros").into(),
        });
    }
    // The Blender elements of their own (`cnc`, `prensa`, `espera`...); not the person
    // bodies (the person's sex picks them), the `_medio` files, the clip files nor what a
    // person carries (the crane's radio control).
    if let Some(cat) = crate::modelos3d::Catalogo::find(None) {
        for (stem, e) in &cat.elementos {
            if stem.ends_with("_medio")
                || stem.starts_with("pessoa_clipes")
                || stem == crate::modelos3d::RADIO
                || crate::modelos3d::PESSOAS.contains(&stem.as_str())
                || out.iter().any(|c| &c.value == stem)
            {
                continue;
            }
            let t = e.tamanho;
            out.push(Choice {
                value: stem.clone(),
                label: stem.clone(),
                help: format!(
                    "{} ({} x {} x {} m)",
                    e.arquivo,
                    dx_text(t[0]),
                    dx_text(t[1]),
                    dx_text(t[2])
                ),
                group: tx("godot-visual-grupo-elementos").into(),
            });
        }
    }
    out
}

/// Text of the default item model (the box) in Portuguese; the screen shows it translated,
/// and either is read back as "the default".
pub const DEFAULT_ITEM: &str = "caixa (padrão)";

/// The models an item can be drawn with (`visual.item` of a source): the box (`""`) and
/// the Blender elements, but the person bodies, the `_medio` files and the clip files.
pub fn item_choices() -> Vec<Choice> {
    let mut out = vec![choice("", tx("godot-visual-caixa-padrao"))];
    if let Some(cat) = crate::modelos3d::Catalogo::find(None) {
        for (stem, e) in &cat.elementos {
            if stem.ends_with("_medio")
                || stem.starts_with("pessoa_clipes")
                || crate::modelos3d::PESSOAS.contains(&stem.as_str())
            {
                continue;
            }
            let t = e.tamanho;
            out.push(Choice {
                value: stem.clone(),
                label: stem.clone(),
                help: fx(
                    "godot-visual-no-tamanho-da-peca",
                    &[
                        ("arquivo", sx(e.arquivo.clone())),
                        ("x", dx(format!("{:.2}", t[0]))),
                        ("y", dx(format!("{:.2}", t[1]))),
                        ("z", dx(format!("{:.2}", t[2]))),
                    ],
                ),
                group: tx("godot-visual-grupo-elementos").into(),
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
        tx("godot-campo-ativo"),
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
            tx("godot-campo-cor"),
            FieldType::Color,
            v.color.clone().map_or(Value::Null, Value::Text),
        )
        .optional(v.color.is_some()),
        float(
            s,
            "visual.scale",
            tx("godot-campo-escala"),
            v.scale.unwrap_or(1.0),
        )
        .min(0.01),
    ];
    // A source picks how its items are drawn (the box by default).
    if matches!(obj.kind, ObjectKind::Source { .. }) {
        let mut item = Field::new(
            s,
            "visual.item",
            tx("godot-campo-modelo-da-peca"),
            FieldType::Asset,
            Value::Text(v.item.clone().unwrap_or_default()),
        )
        .choices(item_choices());
        item.help = tx("godot-visual-modelo-da-peca-ajuda").into();
        out.push(item);
    }
    // A fixed object turns in "Posição" (Rotação); a mobile resource faces where it goes,
    // and this only turns its drawing.
    if is_mobile(&obj.kind) {
        out.push(
            float(
                s,
                "visual.yaw",
                tx("godot-campo-giro-do-desenho"),
                v.yaw.unwrap_or(0.0),
            )
            .unit("graus")
            .help(tx("godot-ajuda-corrige-para-onde-o-modelo")),
        );
    }
    out.push(
        Field::new(
            s,
            "visual.reset",
            tx("godot-campo-restaurar-padrao"),
            FieldType::Action,
            Value::Bool(obj.visual.as_ref().is_some_and(|v| {
                is_mobile(&obj.kind)
                    || VisualDef {
                        yaw: None,
                        item: None,
                        ..v.clone()
                    } != VisualDef::default()
            })),
        )
        .help(tx("godot-ajuda-volta-a-aparencia-padrao-do")),
    );
    out
}

/// Every field of object `obj`, in panel order.
pub fn object_fields(model: &Model, obj: &ObjectDef) -> Vec<Field> {
    let mut out = vec![
        Field::new(
            S_OBJECT,
            "name",
            tx("godot-campo-nome"),
            FieldType::Text,
            Value::Text(obj.name.clone()),
        ),
        Field::new(
            S_OBJECT,
            "kind",
            tx("godot-campo-tipo"),
            FieldType::Info,
            Value::Text(kind_label(&obj.kind).into()),
        ),
    ];
    for (i, (axis, label)) in [("x", "X"), ("y", "Y"), ("z", tx("godot-eixo-z"))]
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
                tx("godot-campo-rotacao"),
                crate::document::yaw_of(model, obj),
            )
            .unit("°")
            .help(tx("godot-ajuda-giro-em-graus-anti-horario")),
        );
    }
    // The crane moves but has its own area (runway length × span).
    if !is_mobile(&obj.kind) || matches!(obj.kind, ObjectKind::Crane { .. }) {
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
            tx("godot-campo-sexo"),
            match p.sex {
                Some(utsim_core::Sex::Female) => "Female",
                Some(utsim_core::Sex::Male) => "Male",
                None => "",
            },
            &[
                ("", "godot-opcao-nao-informado"),
                ("Female", "godot-opcao-feminino"),
                ("Male", "godot-opcao-masculino"),
            ],
        )
        .help(tx("godot-ajuda-escolhe-o-corpo-no-3d")),
        opt(
            "person.height_m",
            tx("godot-campo-altura"),
            p.height_m,
            "m",
            tx("godot-ajuda-a-altura-do-boneco-no"),
        ),
        opt(
            "person.mass_kg",
            tx("godot-campo-massa"),
            p.mass_kg,
            "kg",
            tx("godot-ajuda-com-a-altura-da-a"),
        ),
        Field::new(
            s,
            "person.age",
            tx("godot-campo-idade"),
            FieldType::Int,
            p.age.map_or(Value::Null, |a| Value::Int(i64::from(a))),
        )
        .optional(p.age.is_some())
        .min(0.0)
        .unit("anos")
        .help(tx("godot-ajuda-so-registro-a-simulacao-nao")),
        opt(
            "person.experience_years",
            tx("godot-campo-experiencia-na-funcao"),
            p.experience_years,
            "anos",
            tx("godot-ajuda-so-registro-a-simulacao-nao"),
        ),
        opt(
            "person.capacity.load_kg",
            tx("godot-campo-carga-maxima-medida"),
            cap.load_kg,
            "kg",
            tx("godot-ajuda-uma-carga-acima-dela-da"),
        ),
        opt(
            "person.capacity.force_n",
            tx("godot-campo-forca-maxima-medida"),
            cap.force_n,
            "N",
            tx("godot-ajuda-empurrar-ou-puxar-so-registro"),
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
            tx("godot-campo-tipo-de-espera"),
            w.map_or("", |w| w.kind.name()),
            &[
                ("", "godot-opcao-fila-simples-sem-lugares"),
                ("Table", "godot-opcao-mesa"),
                ("Pallet", "godot-opcao-palete"),
                ("Shelf", "godot-opcao-estante"),
                ("Floor", "godot-opcao-chao-area-demarcada"),
            ],
        )
        .help(tx("godot-ajuda-numa-espera-real-cada-peca")),
    ];
    if let Some(w) = w {
        out.push(
            Field::new(
                s,
                "waiting_area.layers",
                tx("godot-campo-camadas"),
                FieldType::Int,
                Value::Int(i64::from(w.layers())),
            )
            .min(1.0)
            .help(tx("godot-ajuda-quantas-pecas-empilhadas-em-cada")),
        );
        out.push(
            float(
                s,
                "waiting_area.pitch",
                tx("godot-campo-passo-entre-pecas"),
                w.pitch(),
            )
            .min(utsim_model::espera::ITEM)
            .unit("m")
            .help(tx("godot-ajuda-distancia-entre-os-centros-de")),
        );
        let n = w.places(obj.size).len();
        out.push(Field::new(
            s,
            "waiting_area.info",
            tx("godot-campo-lugares"),
            FieldType::Info,
            Value::Text(fx("godot-campo-n-lugares", &[("n", vx(&n))])),
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
                    fx("godot-schema-tipo_de_espera_table_pallet", &[("k", vx(&k))])
                })?;
            }
            "layers" => {
                let n = usize_of(value, "camadas", 1)?;
                w.layers = Some(u32::try_from(n).map_err(|_| tx("godot-schema-camadas_demais"))?);
            }
            "pitch" => w.pitch = Some(f64_of(value, "passo")?),
            _ => {
                return Err(fx(
                    "godot-schema-espera_nao_tem_o_campo",
                    &[("rest", vx(&rest))],
                ));
            }
        }
        if let Some(e) = w.problem() {
            return Err(e);
        }
        let text = ::ron::to_string(&w).map_err(|e| fx("godot-schema-espera", &[("e", vx(&e))]))?;
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
                    other => {
                        return Err(fx(
                            "godot-schema-sexo_female_male_ou_vazio",
                            &[("other", vx(&other))],
                        ));
                    }
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
                                .map_err(|_| tx("godot-schema-idade_grande_demais"))?,
                        ),
                    }
                }
            }
            "capacity.load_kg" => cap.load_kg = number(20.0)?,
            "capacity.force_n" => cap.force_n = number(200.0)?,
            _ => {
                return Err(fx(
                    "godot-schema-pessoa_nao_tem_o_campo",
                    &[("field", vx(&field))],
                ));
            }
        }
        p.capacity = (cap != utsim_core::Capacity::default()).then_some(cap);
        // An empty block (`person: Some(())`) still marks a person: it is kept once there.
        let text = if p == utsim_core::Person::default() && obj.person.is_none() {
            "None".to_string()
        } else {
            format!(
                "Some({})",
                ::ron::to_string(&p).map_err(|e| fx("godot-schema-pessoa", &[("e", vx(&e))]))?
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
            tx("godot-campo-opera-as-maquinas"),
            FieldType::Info,
            Value::Text(list(&operates)),
        )
        .help(tx("godot-ajuda-arraste-do-no-laranja-ate")),
        Field::new(
            s,
            "serves.carries",
            tx("godot-campo-leva-os-itens-de"),
            FieldType::Info,
            Value::Text(list(&carries)),
        )
        .help(tx("godot-ajuda-arraste-do-no-laranja-ate-2")),
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
            tx("godot-campo-nome-do-modelo"),
            FieldType::Text,
            Value::Text(model.name.clone()),
        ),
        Field::new(
            S_MODEL,
            "description",
            tx("godot-campo-descricao"),
            FieldType::Script,
            Value::Text(model.description.clone()),
        )
        .help(tx("godot-ajuda-texto-livre-sobre-o-modelo")),
        float(
            s,
            "stop_time",
            tx("godot-campo-tempo-de-simulacao"),
            run.stop_time,
        )
        .min(0.001),
        float(s, "warmup", tx("godot-campo-aquecimento"), run.warmup)
            .min(0.0)
            .help(tx("godot-ajuda-as-estatisticas-so-contam-depois")),
        int(s, "seed", tx("godot-campo-semente"), run.seed),
        int(
            s,
            "replications",
            tx("godot-campo-replicacoes"),
            u64::from(run.replications),
        )
        .min(1.0),
        pick(
            S_MOVE,
            "movement",
            tx("godot-campo-movimentacao-padrao"),
            movement_name(run.movement),
            &MOVEMENT_CHOICES,
        )
        .help(tx("godot-ajuda-como-andam-os-recursos-moveis")),
        pick(
            S_FLOOR,
            "marking.profile",
            tx("godot-campo-perfil-da-marcacao-do-piso"),
            if matches!(marking.profile, Profile::Branco) {
                "Branco"
            } else {
                "Brasil"
            },
            &[
                ("Brasil", "godot-opcao-brasil-faixa-amarela"),
                ("Branco", "godot-opcao-branco-redacao-original-da-nr"),
            ],
        )
        .help(tx("godot-ajuda-cores-das-faixas-e-setas")),
        float(
            S_FLOOR,
            "marking.line_width",
            tx("godot-campo-largura-da-faixa"),
            marking.line_width,
        )
        .min(0.001)
        .unit("m")
        .help(tx("godot-ajuda-a-pratica-usual-vai-de")),
        float(
            S_FLOOR,
            "marking.crossing_stop",
            tx("godot-campo-parada-no-cruzamento"),
            marking.crossing_stop,
        )
        .min(0.0)
        .unit("s")
        .help(tx("godot-ajuda-o-veiculo-para-esse-tempo")),
        Field::new(
            S_FLOOR,
            "marking.floor_color",
            tx("godot-campo-cor-do-chao"),
            FieldType::Color,
            marking.floor_color.clone().map_or(Value::Null, Value::Text),
        )
        .optional(marking.floor_color.is_some())
        .help(tx("godot-ajuda-a-cor-do-piso-na")),
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
                .ok_or(tx("godot-schema-crie_antes_um_transportador_ou"))?;
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
            let t = o
                .transfer
                .as_mut()
                .ok_or(tx("godot-schema-ligue_a_transferencia_antes"))?;
            match rest {
                "speed" => t.speed = f64_of(v, w)?,
                "delay" => t.delay = f64_of(v, w)?,
                "path" => {
                    t.path = match v.as_text() {
                        Some("Straight") => utsim_core::TransferPath::Straight,
                        Some("Network") => utsim_core::TransferPath::Network,
                        _ => return Err(tx("godot-schema-caminho_straight_ou_network").into()),
                    }
                }
                _ => return Err(fx("godot-schema-campo_desconhecido", &[("w", vx(&w))])),
            }
            return Ok(());
        }
        "downtime" => {
            let d = o
                .downtime
                .as_mut()
                .ok_or(tx("godot-schema-ligue_as_quebras_antes"))?;
            let (which, sub) = rest.split_once('.').unwrap_or((rest, ""));
            return match which {
                "mtbf" => edit_dist(&mut d.mtbf, sub, v),
                "mttr" => edit_dist(&mut d.mttr, sub, v),
                _ => Err(fx("godot-schema-campo_desconhecido", &[("w", vx(&w))])),
            };
        }
        "shifts" => {
            let spec = o
                .shifts
                .as_mut()
                .ok_or(tx("godot-schema-ligue_os_turnos_antes"))?;
            let end = |v: &Value| match v.as_text() {
                Some("Pause") => Ok(Some(ShiftEnd::Pause)),
                Some("Finish") => Ok(Some(ShiftEnd::Finish)),
                Some("") | None => Ok(None),
                Some(t) => Err(fx("godot-schema-fim_do_turno_pause_ou", &[("t", vx(&t))])),
            };
            match (spec, rest) {
                (ShiftSpec::Inline(t), "period") => t.period = f64_of(v, w)?,
                (ShiftSpec::Inline(t), "off") => {
                    let text = v.as_text().ok_or(tx("godot-schema-texto_ron_esperado"))?;
                    t.off = ron::parse(text)
                        .map_err(|e| fx("godot-schema-pausas_invalidas", &[("e", vx(&e))]))?;
                }
                (ShiftSpec::Inline(t), "on_end") => t.on_end = end(v)?.unwrap_or_default(),
                (ShiftSpec::Calendar { name, .. }, "calendar") => {
                    let n = v.as_text().ok_or("nome do calendário esperado")?.trim();
                    if n.is_empty() {
                        return Err(tx("godot-schema-escolha_um_calendario").into());
                    }
                    *name = n.to_string();
                }
                (ShiftSpec::Calendar { on_end, .. }, "on_end") => *on_end = end(v)?,
                _ => return Err(fx("godot-schema-campo_desconhecido", &[("w", vx(&w))])),
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
            let b = batch
                .as_mut()
                .ok_or(tx("godot-schema-ligue_o_lote_antes"))?;
            match rest {
                "size" => b.size = usize_of(v, w, 1)?,
                "max_wait" => b.max_wait = f64_of(v, w)?,
                "flush" => b.flush = bool_of(v, w)?,
                _ => return Err(fx("godot-schema-campo_desconhecido", &[("w", vx(&w))])),
            }
        }
        (ObjectKind::Processor { capacity, .. }, "capacity")
        | (ObjectKind::Transporter { capacity, .. }, "capacity") => *capacity = usize_of(v, w, 1)?,
        (ObjectKind::Processor { setup, .. }, "setup")
        | (ObjectKind::Combiner { setup, .. }, "setup")
        | (ObjectKind::Rack { dwell: setup, .. }, "dwell") => {
            let d = setup
                .as_mut()
                .ok_or_else(|| fx("godot-schema-ligue_antes", &[("head", vx(&head))]))?;
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
            let op = operators
                .as_mut()
                .ok_or(tx("godot-schema-ligue_os_operadores_antes"))?;
            match rest {
                "team" => op.team = v.as_text().ok_or("nome esperado")?.to_string(),
                "count" => op.count = usize_of(v, w, 1)?,
                "team_speedup" => {
                    let gain = f64_of(v, w)?;
                    if !(gain.is_finite() && gain > 0.0) {
                        return Err(tx("godot-schema-o_ganho_da_equipe_deve").into());
                    }
                    op.team_speedup = (gain != 1.0).then_some(gain);
                }
                "setup" => op.setup = bool_of(v, w)?,
                "process" => op.process = bool_of(v, w)?,
                _ => return Err(fx("godot-schema-campo_desconhecido", &[("w", vx(&w))])),
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
        .ok_or_else(|| fx("godot-schema-objeto_nao_existe", &[("object", vx(&object))]))?;
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
                _ => return Err(tx("godot-schema-posicao_x_y_ou_z").into()),
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
    if let Some(command) = edit_crane(model, obj, head, rest, value) {
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
                let v = value
                    .as_text()
                    .ok_or(tx("godot-schema-politica_esperada"))?;
                let label = order
                    .as_ref()
                    .map_or("prioridade".to_owned(), |o| o.label().to_owned());
                let (discipline, order) = match v {
                    "Fifo" | "Lifo" => (v.to_owned(), "None".to_owned()),
                    "Desc" | "Asc" => (
                        "Fifo".to_owned(),
                        format!("Some({v}({}))", ron::string(&label)),
                    ),
                    _ => {
                        return Err(fx(
                            "godot-schema-politica_de_acesso_desconhecida",
                            &[("v", vx(&v))],
                        ));
                    }
                };
                return Ok(Command::Batch(vec![
                    set(&obj.name, "discipline", discipline),
                    set(&obj.name, "order", order),
                ]));
            }
            ("order", "label") => {
                let l = value
                    .as_text()
                    .ok_or(tx("godot-schema-rotulo_esperado"))?
                    .trim()
                    .to_owned();
                if l.is_empty() {
                    return Err(tx("godot-schema-o_rotulo_da_prioridade_nao").into());
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
            let t = value.as_text().ok_or(tx("godot-schema-script_esperado"))?;
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
                _ => return Err(tx("godot-schema-regra_firstfree_random_ou_abc").into()),
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
                        return Err(tx("godot-schema-o_rotulo_nao_pode_ficar").into());
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
                _ => return Err(tx("godot-schema-modo_unpack_ou_split").into()),
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
    let spec = obj
        .shifts
        .as_ref()
        .ok_or(tx("godot-schema-ligue_os_turnos_antes"))?;
    let text = match (value.as_text(), spec) {
        (Some("calendar"), ShiftSpec::Calendar { .. }) | (Some("inline"), ShiftSpec::Inline(_)) => {
            ron::shift_spec(spec)
        }
        (Some("calendar"), ShiftSpec::Inline(_)) => {
            let name = model
                .calendars
                .keys()
                .next()
                .ok_or(tx("godot-schema-o_modelo_nao_tem_calendarios"))?;
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
        _ => return Err(tx("godot-schema-origem_dos_turnos_inline_ou").into()),
    };
    Ok(set(&obj.name, "shifts", text))
}

fn edit_run(model: &Model, path: &str, value: &Value) -> Result<Command, String> {
    let text = |v: &Value| {
        v.as_text()
            .map(str::to_string)
            .ok_or(tx("godot-schema-texto_esperado"))
    };
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
            let movement = movement_from(value.as_text().ok_or(tx("godot-schema-modo_esperado"))?)?;
            return Ok(Command::SetRunMovement(movement));
        }
        "marking.profile" => {
            marking.profile = match value.as_text() {
                Some("Brasil") => Profile::Brasil,
                Some("Branco") => Profile::Branco,
                _ => return Err(tx("godot-schema-perfil_brasil_ou_branco").into()),
            };
            return Ok(Command::SetFloorMarking(Some(Box::new(marking))));
        }
        "marking.line_width" => {
            marking.line_width = f64_of(value, tx("godot-schema-largura_da_faixa"))?;
            return Ok(Command::SetFloorMarking(Some(Box::new(marking))));
        }
        "marking.crossing_stop" => {
            marking.crossing_stop = f64_of(value, tx("godot-schema-parada_no_cruzamento"))?;
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
                        return Err(fx("godot-schema-cor_invalida_use_rrggbb", &[("t", vx(&t))]));
                    }
                    Some(t.to_lowercase())
                }
                _ => return Err(tx("godot-schema-cor_rrggbb").into()),
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
        _ => {
            return Err(fx(
                "godot-schema-run_nao_tem_o_campo",
                &[("path", vx(&path))],
            ));
        }
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
            let t = value.as_text().ok_or(tx("godot-schema-regra_esperada"))?;
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
            let t = value
                .as_text()
                .ok_or(tx("godot-schema-lista_de_pesos_esperada"))?;
            let w: Vec<f64> =
                ron::parse(t).map_err(|e| fx("godot-schema-pesos_invalidos", &[("e", vx(&e))]))?;
            OutputRule::Weighted(w)
        }
        "label" => OutputRule::ByLabel(
            value
                .as_text()
                .ok_or(tx("godot-schema-rotulo_esperado"))?
                .trim()
                .to_string(),
        ),
        "script" => OutputRule::Script(
            value
                .as_text()
                .ok_or(tx("godot-schema-script_esperado"))?
                .to_string(),
        ),
        _ => {
            return Err(fx(
                "godot-schema-campo_desconhecido_output",
                &[("rest", vx(&rest))],
            ));
        }
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
        "item" => {
            let t = value.as_text().unwrap_or("").trim();
            v.item = (!t.is_empty() && t != DEFAULT_ITEM && t != tx("godot-visual-caixa-padrao"))
                .then(|| t.to_string());
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
                    return Err(fx("godot-schema-cor_invalida_use_rrggbb", &[("t", vx(&t))]));
                }
                v.color = Some(t.to_lowercase());
            }
            _ => return Err(tx("godot-schema-cor_rrggbb").into()),
        },
        "scale" => {
            let s = f64_of(value, "escala")?;
            if s.is_nan() || s <= 0.0 {
                return Err(tx("godot-schema-a_escala_precisa_ser_maior").into());
            }
            v.scale = (s != 1.0).then_some(s);
        }
        "yaw" => {
            let y = f64_of(value, "giro")?;
            if !y.is_finite() {
                return Err(tx("godot-schema-giro_um_numero_de_graus").into());
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
        // The model of the items is not the object's look: kept too.
        "reset" => {
            let keep = v.yaw.filter(|_| !is_mobile(&obj.kind));
            v = VisualDef {
                yaw: keep,
                item: v.item.clone(),
                ..VisualDef::default()
            };
        }
        _ => {
            return Err(fx(
                "godot-schema-campo_desconhecido_visual",
                &[("rest", vx(&rest))],
            ));
        }
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
    fn the_crane_fields_are_edited_in_the_inspector() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/ponte_rolante.ron");
        let mut d = Document::open(&path).expect("exemplo da ponte");
        let crane = |d: &Document| match &object(d, "ponte").kind {
            ObjectKind::Crane {
                bridge,
                hoist,
                operator,
                keep_clear,
                bridge_gap,
                motion,
                attach,
                ..
            } => (
                *bridge,
                *hoist,
                operator.clone(),
                *keep_clear,
                *bridge_gap,
                *motion,
                attach.clone(),
            ),
            _ => panic!("não é ponte"),
        };
        assert_eq!(field(&d, "ponte", "size.y").label, "Vão");
        apply(&mut d, "ponte", "bridge.speed", Value::Float(0.75));
        apply(&mut d, "ponte", "hoist.acceleration.on", Value::Bool(true));
        apply(&mut d, "ponte", "hoist.acceleration", Value::Float(0.2));
        apply(&mut d, "ponte", "keep_clear", Value::Bool(true));
        apply(&mut d, "ponte", "bridge_gap", Value::Float(3.5));
        apply(&mut d, "ponte", "motion", Value::Text("Parallel".into()));
        apply(&mut d, "ponte", "attach.mode", Value::Float(50.0));
        let (bridge, hoist, _, keep_clear, gap, motion, _) = crane(&d);
        assert_eq!(bridge.speed, 0.75);
        assert_eq!(hoist.acceleration, Some(0.2));
        assert!(keep_clear);
        assert_eq!(gap, 3.5);
        assert_eq!(motion, utsim_objects::CraneMotion::Parallel);
        assert_eq!(
            field(&d, "ponte", "hoist.acceleration").value,
            Value::Float(0.2)
        );
        // From the cabin to the pendant, then to another team, then to the radio.
        d.apply(Command::Add(Box::new(
            template("Transporter", "op", [0.0; 3]).unwrap(),
        )))
        .unwrap();
        let team = "op".to_string();
        apply(&mut d, "ponte", "operator", Value::Text("Pendant".into()));
        apply(&mut d, "ponte", "operator.team", Value::Text(team.clone()));
        apply(&mut d, "ponte", "operator", Value::Text("Radio".into()));
        assert_eq!(crane(&d).2, CraneControl::Radio { team: team.clone() });
        assert_eq!(field(&d, "ponte", "operator.team").value, Value::Text(team));
        apply(&mut d, "ponte", "operator", Value::Text("Cabin".into()));
        assert!(
            object_fields(d.model(), object(&d, "ponte"))
                .iter()
                .all(|f| f.path != "operator.team")
        );
        assert!(edit(d.model(), "ponte", "bridge.speed", &Value::Float(0.0)).is_err());
        assert_eq!(
            field(&d, "ponte", "lane.info").value,
            Value::Text("Sozinha no caminho".into())
        );
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
