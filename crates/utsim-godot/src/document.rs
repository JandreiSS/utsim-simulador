//! The editing core behind `UtsimDocument`, with no Godot types: a
//! [`utsim_edit::Document`] plus what the editor keeps between calls (the clipboard and
//! its paste count, the starting positions of a drag) and the views the Godot side
//! draws (objects, connections with their port numbers, properties as a schema).
//!
//! Every change is a [`Command`] applied by the document, so it is validated and undone
//! like in the Bevy editor; operations on several objects come from
//! [`utsim_edit::arrange`] as one `Batch`, so they are one undo step.
//!
//! Positions are model coordinates here (`x`, `y` on the floor plan, `z` up); `lib.rs`
//! converts from and to Godot coordinates with [`crate::coords`].

use std::path::Path;

use utsim_core::RunReport;
use utsim_edit::arrange::{self, Align, Axis, Clipboard};
use utsim_edit::{Command, Document, FieldSource, Issue, PaletteEntry};
use utsim_model::replay::{Replay, ReplayBlock, kind_name};
use utsim_model::{Model, ObjectKind, VisualDef, is_mobile};

use crate::schema::{self, Field, Value};

/// Undo key of a drag (consecutive moves merge into one step).
const DRAG_KEY: &str = "arrastar";

/// Offset of the `n`-th paste (n >= 1) of the same clipboard: two metres right and two
/// down per paste, so the copies do not land on top of each other (the Bevy editor's).
pub fn paste_offset(n: u32) -> [f64; 3] {
    let k = 2.0 * n.max(1) as f64;
    [k, -k, 0.0]
}

/// Default sampling interval of the content series: about 200 points over the
/// statistics window, rounded to 1, 2 or 5 times a power of ten (the Bevy results
/// window's choice).
pub fn default_sample_interval(stop_time: f64, warmup: f64) -> f64 {
    let span = (stop_time - warmup).max(1e-9);
    let raw = span / 200.0;
    let p = 10f64.powf(raw.log10().floor());
    [1.0, 2.0, 5.0, 10.0]
        .into_iter()
        .map(|m| m * p)
        .find(|v| *v >= raw)
        .unwrap_or(raw)
}

/// Default number of histogram bins.
pub const DEFAULT_HISTOGRAM_BINS: usize = 20;

/// Library entry of a stack: a `Queue` with `discipline: Lifo` (the Godot library adds
/// it right after "Fila", which stays FIFO).
pub const STACK_ID: &str = "QueueLifo";

/// The object library of the Godot editor: [`utsim_edit::PALETTE`] with the queue called
/// "Espera" (named `espera`, `espera_2`...): FIFO or LIFO is its "Política de acesso", not
/// a second entry. `STACK_ID` still adds a LIFO wait (older scripts), but is not listed.
pub fn palette() -> Vec<PaletteEntry> {
    utsim_edit::PALETTE
        .into_iter()
        .map(|e| {
            if e.id == "Queue" {
                PaletteEntry {
                    label: "Espera",
                    name_base: "espera",
                    ..e
                }
            } else {
                e
            }
        })
        .collect()
}

pub fn palette_entry(id: &str) -> Option<PaletteEntry> {
    if id == STACK_ID {
        let mut e = palette().into_iter().find(|e| e.id == "Queue")?;
        e.id = STACK_ID;
        return Some(e);
    }
    palette().into_iter().find(|e| e.id == id)
}

/// The command that adds an object of library entry `id` called `name` at `position`
/// (one undo step), or `None` for an unknown entry.
pub fn add_command(id: &str, name: &str, position: [f64; 3]) -> Option<Command> {
    if id == STACK_ID {
        let queue = utsim_edit::from_palette("Queue", name, position)?;
        return Some(Command::Batch(vec![
            Command::Add(Box::new(queue)),
            Command::Set {
                key: format!("{name}.discipline"),
                value: "Lifo".into(),
            },
        ]));
    }
    let object = utsim_edit::from_palette(id, name, position)?;
    Some(Command::Add(Box::new(object)))
}

/// Mobile resources are not wired by ports: the message shown when the user tries to
/// connect one (`None` for ordinary blocks).
pub fn connect_blocked(kind: &str) -> Option<&'static str> {
    match kind {
        "Transporter" | "Dispatcher" => Some(
            "Transportadores e despachantes não usam ligações de fluxo: arraste do nó laranja \
             deles até a máquina que operam (ou até o objeto cujos itens levam).",
        ),
        _ => None,
    }
}

/// Snaps `v` to a grid of `step` (no snapping when `step` is not positive). Never
/// returns `-0.0`.
pub fn snap_to(v: f64, step: f64) -> f64 {
    if step.is_finite() && step > 0.0 {
        (v / step).round() * step + 0.0
    } else {
        v + 0.0
    }
}

/// One object as the editor lists and draws it.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectRow {
    pub name: String,
    pub kind: &'static str,
    pub kind_label: &'static str,
    pub position: [f64; 3],
    pub mobile: bool,
    /// Problems attached to this object.
    pub problems: usize,
    pub warnings: usize,
}

/// One connection: `port` is its number among the outputs of `from` (1, 2...), the
/// order the output rule uses.
#[derive(Clone, Debug, PartialEq)]
pub struct ConnectionRow {
    pub from: String,
    pub to: String,
    pub port: usize,
    /// The source hands its items to a transporter: the link is only logical flow.
    pub logical: bool,
}

/// Everything the properties panel shows for one object.
#[derive(Clone, Debug, PartialEq)]
pub struct Properties {
    pub name: String,
    pub kind: &'static str,
    pub kind_label: &'static str,
    pub fields: Vec<Field>,
    pub problems: Vec<String>,
    pub warnings: Vec<String>,
    /// Destinations of the outgoing connections, in port order.
    pub outputs: Vec<String>,
    /// Sources of the incoming connections.
    pub inputs: Vec<String>,
}

/// The editor state of one open model.
pub struct Editor {
    pub doc: Document,
    clipboard: Clipboard,
    /// Pastes of the current clipboard so far (each one lands further away).
    pastes: u32,
    /// Positions at the start of the current drag.
    drag: Vec<(String, [f64; 3])>,
}

impl Default for Editor {
    fn default() -> Self {
        Editor::new("novo_modelo")
    }
}

impl Editor {
    pub fn new(name: &str) -> Editor {
        Editor::with(Document::new(name))
    }

    pub fn open(path: &Path) -> Result<Editor, String> {
        Document::open(path)
            .map(Editor::with)
            .map_err(|e| e.to_string())
    }

    fn with(doc: Document) -> Editor {
        Editor {
            doc,
            clipboard: Clipboard::default(),
            pastes: 0,
            drag: Vec::new(),
        }
    }

    pub fn model(&self) -> &Model {
        self.doc.model()
    }

    pub fn apply(&mut self, command: Command) -> Result<(), String> {
        self.doc.apply(command)
    }

    /// The drag of the Equipe tool from `from` to `to` (`docs/plano_chegadas_e_equipes_3d.md`,
    /// E1b), one undo step; returns what changed, in a sentence:
    /// - dispatcher → transporter: the member joins the team, or leaves it;
    /// - object → transporter or dispatcher: who carries its items out (`transport`), or,
    ///   with `operators` and a processor, where its operators come from; the same drag
    ///   again clears it.
    pub fn team_link(&mut self, from: &str, to: &str, operators: bool) -> Result<String, String> {
        use utsim_edit::teams;
        let model = self.model();
        let find = |n: &str| {
            model
                .objects
                .iter()
                .find(|o| o.name == n)
                .ok_or_else(|| format!("objeto '{n}' não existe"))
        };
        let (a, b) = (find(from)?, find(to)?);
        if from == to {
            return Err("arraste de um objeto até outro".into());
        }
        let (command, said) = match (&a.kind, &b.kind) {
            (ObjectKind::Dispatcher { team, .. }, ObjectKind::Transporter { .. }) => {
                let leaving = team.iter().any(|m| m == to);
                (
                    teams::team_toggle(model, from, to)?,
                    if leaving {
                        format!("{to} saiu da equipe de {from}.")
                    } else {
                        format!("{to} entrou na equipe de {from}.")
                    },
                )
            }
            (ObjectKind::Processor { operators: op, .. }, _) if operators => {
                let same = op.as_ref().is_some_and(|o| o.team == to);
                let new = (!same).then(|| {
                    // A new request: one operator for setup and process (the defaults).
                    let mut o = match op.clone() {
                        Some(o) => o,
                        None => ron::from_str(r#"(team: "")"#).expect("default operators"),
                    };
                    o.team = to.to_owned();
                    o
                });
                (
                    teams::set_operators(model, from, new.as_ref())?,
                    if same {
                        format!("{from} não pede mais operadores de {to}.")
                    } else {
                        format!("{from} pede operadores de {to}.")
                    },
                )
            }
            (_, ObjectKind::Transporter { .. } | ObjectKind::Dispatcher { .. }) => {
                let same = a.transport.as_deref() == Some(to);
                (
                    teams::set_transport(model, from, (!same).then_some(to))?,
                    if same {
                        format!("{from} não pede mais transporte a {to}.")
                    } else {
                        format!("{from} pede transporte a {to}.")
                    },
                )
            }
            _ => {
                return Err("arraste do despachante a um transportador, ou de um objeto ao transportador ou despachante que o atende".into());
            }
        };
        self.apply(command)?;
        Ok(said)
    }

    /// Switches on the content series (`run.sample_interval`, from the simulated time)
    /// and the histograms (`run.histogram_bins`) that the model leaves off, as one undo
    /// step. Returns the values written (`None` for one already on); an error when both
    /// were on.
    pub fn enable_samples(&mut self) -> Result<(Option<f64>, Option<usize>), String> {
        let run = &self.model().run;
        let interval = run
            .sample_interval
            .is_none()
            .then(|| default_sample_interval(run.stop_time, run.warmup));
        let bins = run
            .histogram_bins
            .is_none()
            .then_some(DEFAULT_HISTOGRAM_BINS);
        let mut all = Vec::new();
        if let Some(v) = interval {
            all.push(Command::Set {
                key: "run.sample_interval".into(),
                value: format!("Some({})", crate::ron_text::float(v)),
            });
        }
        if let Some(b) = bins {
            all.push(Command::Set {
                key: "run.histogram_bins".into(),
                value: format!("Some({b})"),
            });
        }
        if all.is_empty() {
            return Err("a série e os histogramas já estão ligados no modelo".into());
        }
        self.apply(Command::Batch(all))?;
        self.doc.end_gesture();
        Ok((interval, bins))
    }

    fn apply_some(&mut self, command: Option<Command>, none: &str) -> Result<(), String> {
        match command {
            Some(c) => self.apply(c),
            None => Err(none.to_string()),
        }
    }

    /// Adds an object of palette entry `id` at `position` (model coordinates) with a free
    /// name; returns the name.
    pub fn add(&mut self, id: &str, position: [f64; 3]) -> Result<String, String> {
        if let Some(placed) = crate::linha::add_command(&self.doc, id, position) {
            let (name, command) = placed?;
            self.apply(command)?;
            return Ok(name);
        }
        let entry = palette_entry(id).ok_or_else(|| format!("tipo desconhecido: {id}"))?;
        let name = self.doc.unique_name(entry.name_base);
        let command =
            add_command(id, &name, position).ok_or_else(|| format!("tipo desconhecido: {id}"))?;
        self.apply(command)?;
        Ok(name)
    }

    pub fn remove(&mut self, names: &[String]) -> Result<(), String> {
        self.apply_some(arrange::delete_command(names), "nada selecionado")
    }

    pub fn connect(&mut self, from: &str, to: &str) -> Result<(), String> {
        for end in [from, to] {
            if let Some(o) = self.model().objects.iter().find(|o| o.name == end)
                && let Some(msg) = connect_blocked(kind_name(&o.kind))
            {
                return Err(msg.to_string());
            }
        }
        self.apply(Command::Connect {
            from: from.into(),
            to: to.into(),
        })
    }

    pub fn disconnect(&mut self, from: &str, to: &str) -> Result<(), String> {
        self.apply(Command::Disconnect {
            from: from.into(),
            to: to.into(),
        })
    }

    /// Destinations of `name`'s outgoing connections, in port order.
    pub fn outputs(&self, name: &str) -> Vec<String> {
        self.model()
            .connections
            .iter()
            .filter(|(a, _)| a == name)
            .map(|(_, b)| b.clone())
            .collect()
    }

    pub fn inputs(&self, name: &str) -> Vec<String> {
        self.model()
            .connections
            .iter()
            .filter(|(_, b)| b == name)
            .map(|(a, _)| a.clone())
            .collect()
    }

    /// Moves output `index` (0-based) of `from` by `delta` places in the port order, as
    /// one undo step: the outputs from the first changed port on are disconnected and
    /// connected again in the new order (`Connect` appends, so the earlier ports keep
    /// their place).
    pub fn move_connection(&mut self, from: &str, index: usize, delta: i64) -> Result<(), String> {
        let outs = self.outputs(from);
        let target = index as i64 + delta;
        if index >= outs.len() || target < 0 || target >= outs.len() as i64 {
            return Err("não há para onde mover esta ligação".into());
        }
        let target = target as usize;
        let mut order = outs.clone();
        let moved = order.remove(index);
        order.insert(target, moved);
        let first = index.min(target);
        let mut batch: Vec<Command> = outs[first..]
            .iter()
            .map(|to| Command::Disconnect {
                from: from.into(),
                to: to.clone(),
            })
            .collect();
        batch.extend(order[first..].iter().map(|to| Command::Connect {
            from: from.into(),
            to: to.clone(),
        }));
        self.apply(Command::Batch(batch))
    }

    /// Starts dragging `names` (their positions now are the reference of [`Self::drag`]).
    pub fn begin_drag(&mut self, names: &[String]) {
        self.doc.end_gesture();
        self.drag = arrange::positions(self.model(), names);
    }

    /// Moves the dragged objects by `delta` (plan `x`, `y`) from where the drag started;
    /// every call of one drag is one undo step. With a grid `step` (> 0) the delta is
    /// adjusted so the first dragged object lands on the grid (the others keep their
    /// offsets to it).
    pub fn drag(&mut self, delta: [f64; 2], step: f64) -> Result<(), String> {
        let Some((_, first)) = self.drag.first() else {
            return Err("nenhum arrasto começou".into());
        };
        let delta = [
            snap_to(first[0] + delta[0], step) - first[0],
            snap_to(first[1] + delta[1], step) - first[1],
        ];
        let unchanged = self.drag.iter().all(|(name, start)| {
            self.model()
                .objects
                .iter()
                .find(|o| &o.name == name)
                .is_some_and(|o| {
                    o.position[0] == start[0] + delta[0] && o.position[1] == start[1] + delta[1]
                })
        });
        if unchanged {
            return Ok(());
        }
        let command = arrange::moved_from(&self.drag, delta);
        self.doc.apply_coalescing(command, Some(DRAG_KEY))
    }

    pub fn end_drag(&mut self) {
        self.drag.clear();
        self.doc.end_gesture();
    }

    /// Moves `names` by `delta` in one step (the arrow keys).
    pub fn nudge(&mut self, names: &[String], delta: [f64; 2]) -> Result<(), String> {
        self.apply_some(
            arrange::move_command(self.model(), names, delta),
            "nada selecionado",
        )
    }

    pub fn copy(&mut self, names: &[String]) -> usize {
        self.clipboard = Clipboard::copy(self.model(), names);
        self.pastes = 0;
        self.clipboard.objects.len()
    }

    pub fn cut(&mut self, names: &[String]) -> Result<usize, String> {
        let n = self.copy(names);
        if n == 0 {
            return Err("nada selecionado".into());
        }
        self.remove(names)?;
        Ok(n)
    }

    pub fn has_clipboard(&self) -> bool {
        !self.clipboard.is_empty()
    }

    /// Pastes the clipboard; returns the new names. After a cut the first paste goes back
    /// where the objects were; each later paste lands further away.
    pub fn paste(&mut self) -> Result<Vec<String>, String> {
        let gone = self
            .clipboard
            .objects
            .iter()
            .all(|o| !self.model().objects.iter().any(|m| m.name == o.name));
        let offset = if gone && self.pastes == 0 {
            [0.0; 3]
        } else {
            paste_offset(self.pastes + 1)
        };
        let (command, names) = self
            .clipboard
            .paste(self.model(), offset)
            .ok_or("a área de transferência está vazia (Ctrl+C copia)")?;
        self.apply(command)?;
        self.pastes += 1;
        Ok(names)
    }

    /// Copies `names` and pastes them next to the originals, keeping the clipboard.
    pub fn duplicate(&mut self, names: &[String]) -> Result<Vec<String>, String> {
        let clip = Clipboard::copy(self.model(), names);
        let (command, new) = clip
            .paste(self.model(), paste_offset(1))
            .ok_or("nada selecionado")?;
        self.apply(command)?;
        Ok(new)
    }

    /// `how`: `esquerda`, `centro_x`, `direita`, `topo`, `centro_y` or `base`.
    pub fn align(&mut self, names: &[String], how: &str) -> Result<(), String> {
        let how = match how {
            "esquerda" => Align::Left,
            "centro_x" => Align::CenterX,
            "direita" => Align::Right,
            "topo" => Align::Top,
            "centro_y" => Align::CenterY,
            "base" => Align::Bottom,
            _ => return Err(format!("alinhamento desconhecido: {how}")),
        };
        self.apply_some(
            arrange::align_command(self.model(), names, how),
            "selecione dois objetos ou mais para alinhar",
        )
    }

    /// `axis`: `x` or `y`.
    pub fn distribute(&mut self, names: &[String], axis: &str) -> Result<(), String> {
        let axis = match axis {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => return Err(format!("eixo desconhecido: {axis}")),
        };
        self.apply_some(
            arrange::distribute_command(self.model(), names, axis),
            "selecione três objetos ou mais para distribuir",
        )
    }

    /// Sets property `path` of `object` (`"run"` for the run settings), and the same on
    /// `others` when they are of the same kind (one undo step). Look edits of one gesture
    /// (dragging in the colour picker) merge until [`Document::end_gesture`].
    pub fn set_property(
        &mut self,
        object: &str,
        path: &str,
        value: &Value,
        others: &[String],
    ) -> Result<(), String> {
        let mut command = schema::edit(self.model(), object, path, value)?;
        // Editing (or switching off) a field fed by real data takes it back to a plain
        // distribution first, in the same undo step.
        if let Some(field) = self.data_field_of(object, path) {
            let current = utsim_model::dados::get_dist(self.model(), &field)?
                .ok_or_else(|| format!("'{field}' não tem distribuição"))?;
            command = Command::Batch(vec![
                Command::SetFieldSource {
                    field,
                    source: FieldSource::Dist(current),
                },
                command,
            ]);
        }
        let mut all = vec![object.to_string()];
        all.extend(others.iter().cloned());
        let command = if others.is_empty() || path == "name" || path.starts_with("position.") {
            command
        } else if arrange::same_kind(self.model(), &all) {
            arrange::fan_out(std::slice::from_ref(&command), object, others).unwrap_or(command)
        } else {
            command
        };
        let key = path
            .starts_with("visual.")
            .then(|| format!("aparencia:{}", all.join(",")));
        self.doc.apply_coalescing(command, key.as_deref())
    }

    /// The key of the data entry that feeds the distribution `path` of `object` belongs
    /// to (`path` may be the field itself, `process.family`, `setup.on`...).
    fn data_field_of(&self, object: &str, path: &str) -> Option<String> {
        self.model()
            .data
            .iter()
            .map(|i| i.field.as_str())
            .find(|f| {
                f.strip_prefix(object)
                    .and_then(|r| r.strip_prefix('.'))
                    .is_some_and(|field| path == field || path.starts_with(&format!("{field}.")))
            })
            .map(str::to_string)
    }

    pub fn objects(&self) -> Vec<ObjectRow> {
        let issues = self.doc.issues();
        let warnings = self.doc.warnings();
        let count = |list: &[Issue], n: &str| {
            list.iter()
                .filter(|i| i.object.as_deref() == Some(n))
                .count()
        };
        self.model()
            .objects
            .iter()
            .map(|o| ObjectRow {
                name: o.name.clone(),
                kind: kind_name(&o.kind),
                kind_label: schema::kind_label(&o.kind),
                position: o.position,
                mobile: is_mobile(&o.kind),
                problems: count(&issues, &o.name),
                warnings: count(&warnings, &o.name),
            })
            .collect()
    }

    pub fn connections(&self) -> Vec<ConnectionRow> {
        let model = self.model();
        let mut ports: Vec<(&str, usize)> = Vec::new();
        model
            .connections
            .iter()
            .map(|(a, b)| {
                let port = match ports.iter_mut().find(|(n, _)| *n == a.as_str()) {
                    Some((_, k)) => {
                        *k += 1;
                        *k
                    }
                    None => {
                        ports.push((a.as_str(), 1));
                        1
                    }
                };
                ConnectionRow {
                    from: a.clone(),
                    to: b.clone(),
                    port,
                    logical: model
                        .objects
                        .iter()
                        .any(|o| &o.name == a && o.transport.is_some()),
                }
            })
            .collect()
    }

    /// The properties of `object`, or of the run settings for `"run"` (or an unknown
    /// name), with the issues that concern it.
    pub fn properties(&self, object: &str) -> Properties {
        let issues = self.doc.issues();
        let warnings = self.doc.warnings();
        let model = self.model();
        let Some(obj) = model.objects.iter().find(|o| o.name == object) else {
            // The model's own list: what belongs to no object, and what talks about an
            // aisle (its warnings also name a vehicle, which would file them under it).
            let general = |list: &[Issue]| -> Vec<String> {
                list.iter()
                    .filter(|i| {
                        i.object.is_none()
                            || model
                                .aisles
                                .iter()
                                .any(|a| i.message.contains(&format!("'{}'", a.name)))
                    })
                    .map(|i| i.message.clone())
                    .collect()
            };
            return Properties {
                name: "run".into(),
                kind: "run",
                kind_label: "Modelo",
                fields: schema::run_fields(model),
                problems: general(&issues),
                warnings: general(&warnings),
                outputs: Vec::new(),
                inputs: Vec::new(),
            };
        };
        let mine = |list: &[Issue]| -> Vec<String> {
            list.iter()
                .filter(|i| i.object.as_deref() == Some(object))
                .map(|i| i.message.clone())
                .collect()
        };
        Properties {
            name: obj.name.clone(),
            kind: kind_name(&obj.kind),
            kind_label: schema::kind_label(&obj.kind),
            fields: schema::object_fields(model, obj),
            problems: mine(&issues),
            warnings: mine(&warnings),
            outputs: self.outputs(object),
            inputs: self.inputs(object),
        }
    }
}

/// The static part of a replay for the model as it is being edited (no run, no
/// validation): one block per object, flow and fluid node, like
/// `Model::empty_replay`, and only the connections whose ends exist. The edit view draws
/// it with the same scene code as the playback.
pub fn edit_replay(model: &Model) -> Replay {
    let index = |name: &str| model.objects.iter().position(|o| o.name == name);
    Replay {
        model: model.name.clone(),
        replication: 0,
        stop_time: 0.0,
        blocks: model
            .objects
            .iter()
            .map(|o| ReplayBlock {
                name: o.name.clone(),
                kind: kind_name(&o.kind).to_string(),
                position: o.position,
            })
            .chain(model.flows.iter().map(|f| ReplayBlock {
                name: f.name.clone(),
                kind: "Flow".to_string(),
                position: [0.0; 3],
            }))
            .chain(model.fluids.iter().map(|f| ReplayBlock {
                name: f.name.clone(),
                kind: "Fluid".to_string(),
                position: [0.0; 3],
            }))
            .collect(),
        connections: model
            .connections
            .iter()
            .filter_map(|(a, b)| Some((index(a)?, index(b)?)))
            .collect(),
        records: Vec::new(),
        report: RunReport {
            replication: 0,
            seed: model.run.seed,
            stats_from: 0.0,
            end_time: 0.0,
            events: 0,
            blocks: Vec::new(),
        },
        fluids: model.fluids.clone(),
    }
}

/// Compact RON of a value, for comparing two models part by part.
fn ron_of<T: serde::Serialize>(v: &T) -> String {
    ron::to_string(v).unwrap_or_else(|e| format!("<{e}>"))
}

/// What differs between model `got` and model `want`, in Portuguese, one line per
/// difference (empty when they are the same model): the name, the description, the run
/// settings, each object (by name, so the order of the list does not matter), the
/// connections (in order: it gives the ports) and the other parts as a whole. Comments
/// and formatting of the files do not count. The scripted check of the Godot interface
/// compares the `.ron` it saved with an example this way.
pub fn model_differences(got: &Model, want: &Model) -> Vec<String> {
    let mut out = Vec::new();
    if got.name != want.name {
        out.push(format!("nome: {:?} em vez de {:?}", got.name, want.name));
    }
    if got.description != want.description {
        out.push(format!(
            "descrição: {:?} em vez de {:?}",
            got.description, want.description
        ));
    }
    if got.run != want.run {
        out.push(format!(
            "run: {} em vez de {}",
            ron_of(&got.run),
            ron_of(&want.run)
        ));
    }
    for w in &want.objects {
        match got.objects.iter().find(|g| g.name == w.name) {
            None => out.push(format!("falta o objeto {}", w.name)),
            Some(g) if g != w => out.push(format!(
                "objeto {}: {} em vez de {}",
                w.name,
                ron_of(g),
                ron_of(w)
            )),
            Some(_) => {}
        }
    }
    for g in &got.objects {
        if !want.objects.iter().any(|w| w.name == g.name) {
            out.push(format!("objeto a mais: {}", g.name));
        }
    }
    if got.connections != want.connections {
        out.push(format!(
            "ligações: {} em vez de {}",
            ron_of(&got.connections),
            ron_of(&want.connections)
        ));
    }
    let rest = |m: &Model| {
        let mut m = m.clone();
        m.name.clear();
        m.description.clear();
        m.objects.clear();
        m.connections.clear();
        m.run = want.run.clone();
        ron_of(&m)
    };
    if rest(got) != rest(want) {
        out.push("outras partes do modelo (calendários, rede, fluxos, fluidos) diferem".into());
    }
    out
}

/// [`model_differences`] between the models of two files.
pub fn file_differences(got: &Path, want: &Path) -> Result<Vec<String>, String> {
    let read = |p: &Path| {
        Document::open(p)
            .map(|d| d.model().clone())
            .map_err(|e| format!("{}: {e}", p.display()))
    };
    Ok(model_differences(&read(got)?, &read(want)?))
}

/// A model with only object `name` of `model`, at the origin and with `asset` as its
/// `visual.asset` (`""` for the default look of its kind; scale, turn and colour kept):
/// the miniature of the asset picker. A rack or a conveyor keeps its own size. `None`
/// when there is no such object.
pub fn look_model(model: &Model, name: &str, asset: &str) -> Option<Model> {
    let mut object = model.objects.iter().find(|o| o.name == name)?.clone();
    object.position = [0.0; 3];
    let mut visual = object.visual.clone().unwrap_or_default();
    let asset = asset.trim();
    visual.asset = (!asset.is_empty()).then(|| asset.to_string());
    object.visual = (visual != VisualDef::default()).then_some(visual);
    // Settings that name other objects (a team, a transporter) would not validate alone;
    // the miniature only needs the look.
    let mut doc = Document::new("miniatura");
    doc.apply(Command::Add(Box::new(object))).ok()?;
    Some(doc.model().clone())
}

/// A model with only palette entry `id` at the origin (the ghost shown while placing).
pub fn palette_model(id: &str) -> Option<Model> {
    if id.contains('/') {
        return crate::linha::preset_model(id);
    }
    let mut doc = Document::new("fantasma");
    let base = palette_entry(id)?.name_base;
    doc.apply(add_command(id, base, [0.0; 3])?).ok()?;
    Some(doc.model().clone())
}

/// The keyboard and mouse shortcuts of the Godot editor, as the "Atalhos" window shows
/// them (shortcut, what it does).
pub const SHORTCUTS: [(&str, &str); 33] = [
    ("Clique", "seleciona o bloco ou a ligação"),
    (
        "Clique de novo no mesmo ponto",
        "passa para o próximo bloco sobreposto",
    ),
    ("Shift + clique", "põe ou tira o bloco da seleção"),
    (
        "Shift + arrastar no chão",
        "seleção por caixa (soma à seleção)",
    ),
    ("Arrastar no chão", "seleção por caixa (troca a seleção)"),
    ("Arrastar um bloco", "move o bloco ou a seleção inteira"),
    (
        "Arrastar da porta de saída (verde)",
        "liga ao bloco (ou à porta de entrada) onde soltar",
    ),
    ("Botão direito + arrastar", "gira a vista"),
    ("Shift + botão direito + arrastar", "desloca a vista"),
    ("Roda do mouse", "aproxima e afasta"),
    (
        "Biblioteca: clique e clique no chão",
        "coloca um objeto (Shift mantém a ferramenta)",
    ),
    (
        "Biblioteca: arrastar até o chão",
        "coloca um objeto onde soltar",
    ),
    ("Setas", "move a seleção um passo da grade"),
    ("Del", "exclui a seleção (ou a ligação selecionada)"),
    (
        "L",
        "ferramenta Ligar: arraste da origem ao destino, ou clique e clique",
    ),
    (
        "C",
        "ferramenta Corredor: clique pontos no chão, Enter ou duplo clique termina",
    ),
    (
        "E (ou Alt com Ligar)",
        "ferramenta Equipe: arraste do despachante ao transportador, ou do objeto a quem leva os itens (Shift: operadores do processo)",
    ),
    (
        "Esc",
        "cancela a ferramenta, a ligação ou o corredor começado, ou a seleção",
    ),
    ("F", "enquadra o modelo"),
    ("G", "liga ou desliga o encaixe na grade"),
    ("Tab", "alterna entre Editar e Simular"),
    ("F1", "mostra ou esconde esta janela"),
    ("Ctrl + A", "seleciona todos os objetos"),
    ("Ctrl + C", "copia a seleção"),
    ("Ctrl + X", "recorta a seleção"),
    (
        "Ctrl + V",
        "cola (nomes únicos, ligações internas preservadas)",
    ),
    ("Ctrl + D", "duplica a seleção"),
    ("Ctrl + Z", "desfaz"),
    ("Ctrl + Y ou Ctrl + Shift + Z", "refaz"),
    ("Ctrl + S", "salva"),
    ("Ctrl + Shift + S", "salva como"),
    ("Ctrl + O", "abre um modelo"),
    ("Ctrl + N", "novo modelo"),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn line() -> Editor {
        let mut e = Editor::new("teste");
        for (i, id) in ["Source", "Queue", "Processor", "Sink"].iter().enumerate() {
            e.add(id, [3.0 * i as f64, 0.0, 0.0]).unwrap();
        }
        for (a, b) in [
            ("chegada", "espera"),
            ("espera", "processo"),
            ("processo", "saida"),
        ] {
            e.connect(a, b).unwrap();
        }
        e
    }

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    fn position(e: &Editor, n: &str) -> [f64; 3] {
        e.model()
            .objects
            .iter()
            .find(|o| o.name == n)
            .unwrap()
            .position
    }

    #[test]
    fn a_line_built_from_the_palette_runs_and_undoes_step_by_step() {
        let mut e = line();
        assert!(e.doc.issues().is_empty(), "{:?}", e.doc.issues());
        assert_eq!(e.objects().len(), 4);
        assert_eq!(e.objects()[2].kind_label, "Processo");
        e.set_property(
            "processo",
            "process",
            &Value::Text("Const(2.0)".into()),
            &[],
        )
        .unwrap();
        assert!(e.model().run_replications().is_ok());
        for _ in 0..8 {
            assert!(e.doc.undo());
        }
        assert!(e.model().objects.is_empty());
        assert!(!e.doc.undo());
        while e.doc.redo() {}
        assert_eq!(e.model().connections.len(), 3);
    }

    #[test]
    fn the_wait_is_one_entry_and_its_policy_chooses_lifo_or_a_priority() {
        let p = palette();
        assert!(p.iter().all(|e| e.id != STACK_ID), "no second entry");
        let q = p.iter().find(|e| e.id == "Queue").unwrap();
        assert_eq!((q.label, q.name_base), ("Espera", "espera"));
        let mut e = line();
        let name = e.add("Queue", [0.0, 6.0, 0.0]).unwrap();
        assert_eq!(name, "espera_2");
        assert_eq!(e.objects().last().unwrap().kind_label, "Espera");
        let policy = |e: &Editor, n: &str| {
            schema::object_fields(e.model(), object(e, n))
                .into_iter()
                .find(|f| f.path == "discipline")
                .unwrap()
        };
        assert_eq!(policy(&e, &name).label, "Política de acesso");
        assert_eq!(policy(&e, &name).value, Value::Text("Fifo".into()));
        // LIFO: one undo step; the scene marks it.
        e.set_property(&name, "discipline", &Value::Text("Lifo".into()), &[])
            .unwrap();
        assert_eq!(
            schema::read_field(object(&e, &name), "discipline").as_deref(),
            Some("Lifo")
        );
        let pb =
            crate::playback::Playback::new(&edit_replay(e.model()), Some(e.model()), None, false);
        let at = pb.blocks.iter().position(|b| b.name == name).unwrap();
        assert!(pb.blocks[at].stack);
        assert_eq!(pb.frame(0.0).label[at], "espera_2 · LIFO");
        // A priority: FIFO plus `order`, never LIFO with `order` (the model's rule).
        e.set_property(&name, "discipline", &Value::Text("Desc".into()), &[])
            .unwrap();
        assert_eq!(
            schema::read_field(object(&e, &name), "discipline").as_deref(),
            Some("Fifo")
        );
        assert_eq!(
            schema::read_field(object(&e, &name), "order").as_deref(),
            Some("Some(Desc(\"prioridade\"))")
        );
        assert_eq!(policy(&e, &name).value, Value::Text("Desc".into()));
        e.set_property(&name, "order.label", &Value::Text("urgencia".into()), &[])
            .unwrap();
        assert_eq!(
            schema::read_field(object(&e, &name), "order").as_deref(),
            Some("Some(Desc(\"urgencia\"))")
        );
        e.set_property(&name, "discipline", &Value::Text("Lifo".into()), &[])
            .unwrap();
        assert_eq!(
            schema::read_field(object(&e, &name), "order").as_deref(),
            Some("None")
        );
        assert!(
            e.doc
                .issues()
                .iter()
                .all(|i| !format!("{i:?}").contains("os dois"))
        );
        assert!(e.doc.undo(), "one step per change");
        assert_eq!(
            schema::read_field(object(&e, &name), "order").as_deref(),
            Some("Some(Desc(\"urgencia\"))")
        );
        // Older scripts: the stack id still adds a LIFO wait.
        let ghost = palette_model(STACK_ID).unwrap();
        assert_eq!(
            schema::read_field(&ghost.objects[0], "discipline").as_deref(),
            Some("Lifo")
        );
    }

    fn object<'a>(e: &'a Editor, name: &str) -> &'a utsim_model::ObjectDef {
        e.model().objects.iter().find(|o| o.name == name).unwrap()
    }

    #[test]
    fn series_and_histograms_are_switched_on_as_one_undo_step() {
        assert_eq!(default_sample_interval(2000.0, 100.0), 10.0);
        assert_eq!(default_sample_interval(3000.0, 100.0), 20.0);
        let mut e = line();
        e.set_property("run", "stop_time", &Value::Float(2000.0), &[])
            .unwrap();
        e.doc.end_gesture();
        let (interval, bins) = e.enable_samples().unwrap();
        assert_eq!((interval, bins), (Some(10.0), Some(DEFAULT_HISTOGRAM_BINS)));
        assert_eq!(e.model().run.sample_interval, Some(10.0));
        assert_eq!(e.model().run.histogram_bins, Some(DEFAULT_HISTOGRAM_BINS));
        assert!(e.model().run_replications().is_ok());
        assert!(e.enable_samples().is_err(), "both already on");
        assert!(e.doc.undo());
        assert_eq!(e.model().run.sample_interval, None);
        assert_eq!(e.model().run.histogram_bins, None);
        assert_eq!(e.model().run.stop_time, 2000.0, "one step only");
    }

    #[test]
    fn mobile_resources_cannot_be_connected() {
        let mut e = line();
        let agv = e.add("Agv", [0.0, 5.0, 0.0]).unwrap();
        assert_eq!(agv, "agv");
        let err = e.connect("espera", &agv).unwrap_err();
        assert!(err.contains("nó laranja"), "{err}");
        assert!(e.connect("espera", "espera").is_err());
        assert!(
            e.connect("espera", "processo").is_err(),
            "already connected"
        );
    }

    #[test]
    fn connections_have_port_numbers_and_can_be_reordered() {
        let mut e = line();
        e.add("Sink", [6.0, 3.0, 0.0]).unwrap();
        e.add("Sink", [6.0, 6.0, 0.0]).unwrap();
        e.connect("processo", "saida_2").unwrap();
        e.connect("processo", "saida_3").unwrap();
        let ports: Vec<(String, usize)> = e
            .connections()
            .into_iter()
            .filter(|c| c.from == "processo")
            .map(|c| (c.to, c.port))
            .collect();
        assert_eq!(
            ports,
            vec![
                ("saida".into(), 1),
                ("saida_2".into(), 2),
                ("saida_3".into(), 3)
            ]
        );
        e.move_connection("processo", 2, -2).unwrap();
        assert_eq!(
            e.outputs("processo"),
            names(&["saida_3", "saida", "saida_2"])
        );
        e.move_connection("processo", 0, 1).unwrap();
        assert_eq!(
            e.outputs("processo"),
            names(&["saida", "saida_3", "saida_2"])
        );
        assert!(e.move_connection("processo", 2, 1).is_err());
        // One undo step per reorder.
        e.doc.undo();
        assert_eq!(
            e.outputs("processo"),
            names(&["saida_3", "saida", "saida_2"])
        );
        assert_eq!(e.outputs("chegada"), names(&["espera"]));
        e.disconnect("processo", "saida").unwrap();
        assert_eq!(e.outputs("processo"), names(&["saida_3", "saida_2"]));
    }

    #[test]
    fn a_drag_is_one_undo_step_and_nudge_moves_by_the_step() {
        let mut e = line();
        let sel = names(&["espera", "processo"]);
        e.begin_drag(&sel);
        for k in 1..=5 {
            e.drag([k as f64, 0.5 * k as f64], 0.0).unwrap();
        }
        e.end_drag();
        assert_eq!(position(&e, "espera"), [8.0, 2.5, 0.0]);
        assert_eq!(position(&e, "processo"), [11.0, 2.5, 0.0]);
        e.doc.undo();
        assert_eq!(position(&e, "espera"), [3.0, 0.0, 0.0]);
        e.nudge(&sel, [0.0, -1.0]).unwrap();
        assert_eq!(position(&e, "processo"), [6.0, -1.0, 0.0]);
        assert!(e.drag([1.0, 1.0], 0.0).is_err(), "no drag going on");
        e.begin_drag(&sel);
        e.drag([0.3, 0.8], 1.0).unwrap();
        e.end_drag();
        assert_eq!(
            position(&e, "espera"),
            [3.0, 0.0, 0.0],
            "snapped to the grid"
        );
    }

    #[test]
    fn copy_paste_cut_and_duplicate() {
        let mut e = line();
        assert_eq!(e.copy(&names(&["espera", "processo"])), 2);
        let pasted = e.paste().unwrap();
        assert_eq!(pasted, names(&["espera_2", "processo_2"]));
        assert_eq!(position(&e, "espera_2"), [5.0, -2.0, 0.0]);
        assert!(e.outputs("espera_2").contains(&"processo_2".to_string()));
        let again = e.paste().unwrap();
        assert_eq!(position(&e, again[0].as_str()), [7.0, -4.0, 0.0]);
        let dup = e.duplicate(&names(&["saida"])).unwrap();
        assert_eq!(dup, names(&["saida_2"]));
        assert_eq!(e.cut(&names(&["chegada"])).unwrap(), 1);
        assert!(!e.model().objects.iter().any(|o| o.name == "chegada"));
        assert_eq!(e.paste().unwrap(), names(&["chegada"]));
        assert_eq!(
            position(&e, "chegada"),
            [0.0, 0.0, 0.0],
            "a cut pastes back in place"
        );
        assert!(e.has_clipboard());
    }

    #[test]
    fn align_and_distribute() {
        let mut e = line();
        e.nudge(&names(&["espera"]), [0.0, 2.0]).unwrap();
        e.align(&names(&["chegada", "espera", "processo"]), "topo")
            .unwrap();
        assert_eq!(position(&e, "processo")[1], 2.0);
        e.nudge(&names(&["espera"]), [1.0, 0.0]).unwrap();
        e.distribute(&names(&["chegada", "espera", "processo"]), "x")
            .unwrap();
        assert_eq!(position(&e, "espera")[0], 3.0);
        assert!(e.align(&names(&["espera"]), "esquerda").is_err());
        assert!(e.align(&names(&["espera", "saida"]), "nada").is_err());
    }

    #[test]
    fn properties_carry_issues_and_fan_out_to_the_same_kind() {
        let mut e = Editor::new("t");
        e.add("Processor", [0.0; 3]).unwrap();
        e.add("Processor", [2.0, 0.0, 0.0]).unwrap();
        let p = e.properties("processo");
        assert_eq!(p.kind_label, "Processo");
        assert!(
            !p.problems.is_empty(),
            "a processor with no input is a problem"
        );
        e.set_property(
            "processo",
            "capacity",
            &Value::Int(3),
            &names(&["processo_2"]),
        )
        .unwrap();
        let caps: Vec<String> = e
            .model()
            .objects
            .iter()
            .map(|o| schema::read_field(o, "capacity").unwrap())
            .collect();
        assert_eq!(caps, vec!["3", "3"]);
        e.doc.undo();
        assert_eq!(
            schema::read_field(&e.model().objects[1], "capacity").unwrap(),
            "1"
        );
        // Look edits of one gesture are one step.
        e.set_property(
            "processo",
            "visual.color",
            &Value::Text("#102030".into()),
            &[],
        )
        .unwrap();
        e.set_property(
            "processo",
            "visual.color",
            &Value::Text("#405060".into()),
            &[],
        )
        .unwrap();
        e.doc.end_gesture();
        e.doc.undo();
        assert!(e.model().objects[0].visual.is_none());
        let run = e.properties("run");
        assert_eq!(run.kind, "run");
        assert!(run.fields.iter().any(|f| f.path == "stop_time"));
    }

    #[test]
    fn the_edit_replay_draws_invalid_models_and_the_palette_ghosts_exist() {
        let mut e = line();
        e.apply(Command::Connect {
            from: "saida".into(),
            to: "chegada".into(),
        })
        .unwrap();
        e.add("Dispatcher", [0.0, 9.0, 0.0]).unwrap();
        assert!(!e.doc.issues().is_empty());
        let r = edit_replay(e.model());
        assert_eq!(r.blocks.len(), 5);
        assert_eq!(r.connections.len(), 4);
        for entry in palette() {
            let m = palette_model(entry.id).unwrap();
            assert_eq!(m.objects.len(), 1, "{}", entry.id);
        }
        assert!(palette_model("Nada").is_none());
    }

    #[test]
    fn saving_and_opening_keeps_the_model() {
        let mut e = line();
        let dir = std::env::temp_dir().join(format!("utsim-godot-doc-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("linha.ron");
        e.doc.save_as(&path).unwrap();
        assert!(!e.doc.is_dirty());
        let back = Editor::open(&path).unwrap();
        assert_eq!(back.model(), e.model());
        std::fs::remove_dir_all(&dir).ok();
        assert!(Editor::open(&dir.join("nada.ron")).is_err());
    }

    /// The steps of the scripted check `godot/roteiros/g2_tutorial_01.json`, through the
    /// same calls the interface makes (palette, snapping, the panel's paths, connect),
    /// give the model of `examples/tutorial_01_caixa.ron`.
    #[test]
    fn the_tutorial_01_is_built_from_the_palette_and_the_panel() {
        let mut e = Editor::new("novo_modelo");
        // Dropped near the grid points, as the mouse would.
        for (id, x) in [
            ("Source", 0.2),
            ("Queue", 3.9),
            ("Processor", 8.3),
            ("Sink", 11.8),
        ] {
            let at = [snap_to(x, 1.0), snap_to(-0.3, 1.0), 0.0];
            e.add(id, at).unwrap();
        }
        let set = |e: &mut Editor, object: &str, path: &str, v: Value| {
            e.set_property(object, path, &v, &[])
                .unwrap_or_else(|err| panic!("{object}.{path}: {err}"));
        };
        // The library names it "espera"; the example calls it "fila".
        set(&mut e, "espera", "name", Value::Text("fila".into()));
        set(&mut e, "processo", "name", Value::Text("caixa".into()));
        set(&mut e, "chegada", "interarrival.mean", Value::Float(5.0));
        set(&mut e, "caixa", "process.family", Value::Int(2));
        set(&mut e, "caixa", "process.mean", Value::Float(4.0));
        for (a, b) in [("chegada", "fila"), ("fila", "caixa"), ("caixa", "saida")] {
            e.connect(a, b).unwrap();
        }
        set(
            &mut e,
            "run",
            "model_name",
            Value::Text("Tutorial 01: um caixa".into()),
        );
        set(
            &mut e,
            "run",
            "description",
            Value::Text("Fila M/M/1 para comparar a simulação com a fórmula".into()),
        );
        set(&mut e, "run", "stop_time", Value::Float(100000.0));
        set(&mut e, "run", "warmup", Value::Float(2000.0));
        set(&mut e, "run", "seed", Value::Int(42));
        set(&mut e, "run", "replications", Value::Int(10));
        assert!(e.doc.issues().is_empty(), "{:?}", e.doc.issues());
        let dir = std::env::temp_dir().join(format!("utsim-godot-t01-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let saved = dir.join("tutorial_01.ron");
        e.doc.save_as(&saved).unwrap();
        let example =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/tutorial_01_caixa.ron");
        let diff = file_differences(&saved, &example).unwrap();
        assert!(diff.is_empty(), "{diff:#?}");
        // One change shows up as one line.
        set(&mut e, "caixa", "process.mean", Value::Float(3.0));
        let diff = model_differences(e.model(), Editor::open(&example).unwrap().model());
        assert_eq!(diff.len(), 1, "{diff:?}");
        assert!(diff[0].starts_with("objeto caixa:"), "{diff:?}");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn differences_name_what_changed() {
        let a = line();
        let mut b = line();
        assert!(model_differences(a.model(), b.model()).is_empty());
        b.remove(&names(&["saida"])).unwrap();
        b.doc
            .apply(Command::SetInfo {
                name: "outro".into(),
                description: String::new(),
            })
            .unwrap();
        let diff = model_differences(b.model(), a.model());
        assert!(diff.iter().any(|d| d.starts_with("nome:")), "{diff:?}");
        assert!(diff.iter().any(|d| d == "falta o objeto saida"), "{diff:?}");
        assert!(diff.iter().any(|d| d.starts_with("ligações:")), "{diff:?}");
        let diff = model_differences(a.model(), b.model());
        assert!(diff.iter().any(|d| d == "objeto a mais: saida"), "{diff:?}");
        assert!(file_differences(Path::new("nada.ron"), Path::new("nada.ron")).is_err());
    }

    #[test]
    fn the_miniature_has_one_object_with_the_asset_asked() {
        let e = line();
        let m = look_model(e.model(), "processo", "bancada_kenney").unwrap();
        assert_eq!(m.objects.len(), 1);
        assert!(m.connections.is_empty());
        let o = &m.objects[0];
        assert_eq!(o.position, [0.0; 3]);
        assert_eq!(
            o.visual.as_ref().unwrap().asset.as_deref(),
            Some("bancada_kenney")
        );
        let m = look_model(e.model(), "processo", " ").unwrap();
        assert_eq!(m.objects[0].visual, None, "the default look");
        assert!(look_model(e.model(), "nada", "").is_none());
        // It draws like any edit scene.
        let r = edit_replay(&m);
        assert_eq!(r.blocks.len(), 1);
    }

    #[test]
    fn snapping() {
        assert_eq!(snap_to(1.26, 0.5), 1.5);
        assert_eq!(snap_to(-0.1, 1.0), 0.0);
        assert!(snap_to(-0.1, 1.0).is_sign_positive());
        assert_eq!(snap_to(1.26, 0.0), 1.26);
    }
}
