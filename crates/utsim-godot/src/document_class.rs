//! `UtsimDocument`: the Godot class over [`crate::document::Editor`] (a
//! `utsim_edit::Document` with the editor's clipboard and drag state). Every method is a
//! thin conversion between Godot values and the pure core: positions in and out are
//! Godot coordinates (`(x, z, -y)` of the model), names are strings, and a property value
//! is a `Variant` turned into a [`Value`].
//!
//! Methods that change the model return `true` when the command was applied; on `false`
//! the reason is in [`UtsimDocument::last_error`] and nothing changed.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;

use godot::prelude::*;

use crate::document::{self, Editor, Properties, edit_replay, palette_model};
use crate::playback::Playback;
use crate::schema::{self, Field, Value};
use crate::{UtsimModel, UtsimPlayback, put, strings};

/// Godot point to model coordinates.
fn to_model(v: Vector3) -> [f64; 3] {
    [f64::from(v.x), -f64::from(v.z), f64::from(v.y)]
}

/// Model point to Godot coordinates.
fn to_godot(p: [f64; 3]) -> Vector3 {
    Vector3::new(p[0] as f32, p[2] as f32, -(p[1] as f32))
}

fn names_of(list: &PackedStringArray) -> Vec<String> {
    list.as_slice().iter().map(|s| s.to_string()).collect()
}

/// A property value from GDScript.
fn value_of(v: &Variant) -> Value {
    match v.get_type() {
        VariantType::NIL => Value::Null,
        VariantType::BOOL => Value::Bool(v.to::<bool>()),
        VariantType::INT => Value::Int(v.to::<i64>()),
        VariantType::FLOAT => Value::Float(v.to::<f64>()),
        VariantType::STRING | VariantType::STRING_NAME => {
            Value::Text(v.to::<GString>().to_string())
        }
        VariantType::PACKED_STRING_ARRAY => Value::List(names_of(&v.to::<PackedStringArray>())),
        _ => Value::Text(v.stringify().to_string()),
    }
}

fn variant_of(v: &Value) -> Variant {
    match v {
        Value::Null => Variant::nil(),
        Value::Bool(b) => b.to_variant(),
        Value::Int(i) => i.to_variant(),
        Value::Float(f) => f.to_variant(),
        Value::Text(t) => GString::from(t.as_str()).to_variant(),
        Value::List(l) => strings(l).to_variant(),
    }
}

fn field_dict(f: &Field) -> VarDictionary {
    let mut d = VarDictionary::new();
    put(&mut d, "path", GString::from(f.path.as_str()));
    put(&mut d, "label", GString::from(f.label.as_str()));
    put(&mut d, "type", f.ty.name());
    d.set(&"value".to_variant(), &variant_of(&f.value));
    let mut choices = VarArray::new();
    for c in &f.choices {
        let mut e = VarDictionary::new();
        put(&mut e, "value", GString::from(c.value.as_str()));
        put(&mut e, "label", GString::from(c.label.as_str()));
        put(&mut e, "help", GString::from(c.help.as_str()));
        put(&mut e, "group", GString::from(c.group.as_str()));
        choices.push(&e.to_variant());
    }
    put(&mut d, "choices", choices);
    put(&mut d, "unit", f.unit);
    put(&mut d, "help", GString::from(f.help.as_str()));
    put(&mut d, "section", f.section);
    put(&mut d, "depth", i64::from(f.depth));
    put(&mut d, "optional", f.optional);
    put(&mut d, "enabled", f.enabled);
    match f.min {
        Some(m) => put(&mut d, "min", m),
        None => d.set(&"min".to_variant(), &Variant::nil()),
    }
    if let Some(view) = &f.dist {
        let mut e = VarDictionary::new();
        put(&mut e, "family", view.family as i64);
        put(&mut e, "text", GString::from(view.text.as_str()));
        put(&mut e, "mean", view.mean);
        let mut params = VarArray::new();
        for p in &view.params {
            let mut q = VarDictionary::new();
            put(&mut q, "key", p.key);
            put(&mut q, "label", p.label);
            put(&mut q, "value", p.value);
            put(&mut q, "integer", p.integer);
            params.push(&q.to_variant());
        }
        put(&mut e, "params", params);
        put(&mut d, "dist", e);
    }
    d
}

fn properties_dict(p: &Properties) -> VarDictionary {
    let mut d = VarDictionary::new();
    put(&mut d, "name", GString::from(p.name.as_str()));
    put(&mut d, "kind", p.kind);
    put(&mut d, "kind_label", p.kind_label);
    let mut fields = VarArray::new();
    for f in &p.fields {
        fields.push(&field_dict(f).to_variant());
    }
    put(&mut d, "fields", fields);
    put(&mut d, "problems", strings(&p.problems));
    put(&mut d, "warnings", strings(&p.warnings));
    put(&mut d, "outputs", strings(&p.outputs));
    put(&mut d, "inputs", strings(&p.inputs));
    d
}

fn issues_array(list: &[utsim_edit::Issue]) -> VarArray {
    let mut out = VarArray::new();
    for i in list {
        let mut e = VarDictionary::new();
        put(
            &mut e,
            "object",
            GString::from(i.object.as_deref().unwrap_or("")),
        );
        put(&mut e, "message", GString::from(i.message.as_str()));
        out.push(&e.to_variant());
    }
    out
}

/// The scene of a model as it is being edited (no run): `None` if building it panicked
/// (a model too broken to draw), which the caller reports.
fn edit_playback(
    model: &utsim_model::Model,
    dir: Option<&std::path::Path>,
    skeletal: bool,
) -> Option<Playback> {
    catch_unwind(AssertUnwindSafe(|| {
        Playback::new(&edit_replay(model), Some(model), dir, skeletal)
    }))
    .ok()
}

fn playback_object(core: Playback) -> Gd<UtsimPlayback> {
    Gd::from_init_fn(|base| UtsimPlayback {
        core: Some(core),
        base,
    })
}

/// A model being edited, with undo and redo.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimDocument {
    editor: Editor,
    last_error: String,
    base: Base<RefCounted>,
}

impl UtsimDocument {
    /// Records the outcome of an edit: `true` when it went through.
    fn done(&mut self, r: Result<(), String>) -> bool {
        match r {
            Ok(()) => {
                self.last_error.clear();
                true
            }
            Err(e) => {
                self.last_error = e;
                false
            }
        }
    }

    fn file_result(&mut self, r: Result<(), String>) -> VarDictionary {
        let ok = self.done(r);
        let mut d = VarDictionary::new();
        put(&mut d, "ok", ok);
        put(&mut d, "error", GString::from(self.last_error.as_str()));
        d
    }
}

#[godot_api]
impl UtsimDocument {
    /// Starts an empty model called `name`.
    #[func]
    fn new_model(&mut self, name: GString) {
        self.editor = Editor::new(&name.to_string());
        self.last_error.clear();
    }

    /// Opens `path`; the model may be incomplete (its problems are listed). Returns
    /// `{ok, error}`; on error the current document stays.
    #[func]
    fn open(&mut self, path: GString) -> VarDictionary {
        let r = Editor::open(&PathBuf::from(path.to_string())).map(|e| self.editor = e);
        self.file_result(r)
    }

    #[func]
    fn save(&mut self) -> VarDictionary {
        let r = self.editor.doc.save().map_err(|e| e.to_string());
        self.file_result(r)
    }

    #[func]
    fn save_as(&mut self, path: GString) -> VarDictionary {
        let r = self
            .editor
            .doc
            .save_as(PathBuf::from(path.to_string()))
            .map_err(|e| e.to_string());
        self.file_result(r)
    }

    /// File of the document, or `""` before the first save.
    #[func]
    fn path(&self) -> GString {
        self.editor.doc.path().map_or_else(GString::new, |p| {
            GString::from(p.to_string_lossy().replace('\\', "/").as_str())
        })
    }

    #[func]
    fn name(&self) -> GString {
        GString::from(self.editor.model().name.as_str())
    }

    #[func]
    fn last_error(&self) -> GString {
        GString::from(self.last_error.as_str())
    }

    #[func]
    fn is_dirty(&self) -> bool {
        self.editor.doc.is_dirty()
    }

    /// Changes with every edit, undo and redo: equal revisions mean the same model.
    #[func]
    fn revision(&self) -> i64 {
        self.editor.doc.revision() as i64
    }

    #[func]
    fn can_undo(&self) -> bool {
        self.editor.doc.can_undo()
    }

    #[func]
    fn can_redo(&self) -> bool {
        self.editor.doc.can_redo()
    }

    #[func]
    fn undo(&mut self) -> bool {
        self.editor.doc.undo()
    }

    #[func]
    fn redo(&mut self) -> bool {
        self.editor.doc.redo()
    }

    /// Ends a gesture (a colour drag), so the next edit is a step of its own.
    #[func]
    fn end_gesture(&mut self) {
        self.editor.doc.end_gesture();
    }

    /// The saved RON text of the model.
    #[func]
    fn text(&self) -> GString {
        GString::from(self.editor.doc.to_text().as_str())
    }

    /// Adds palette entry `id` at Godot point `at`, snapped to `step` (0: no grid).
    /// Returns the new name, or `""`.
    #[func]
    fn add(&mut self, id: GString, at: Vector3, step: f64) -> GString {
        let mut p = to_model(at);
        p[0] = document::snap_to(p[0], step);
        p[1] = document::snap_to(p[1], step);
        p[2] = 0.0;
        match self.editor.add(&id.to_string(), p) {
            Ok(name) => {
                self.last_error.clear();
                GString::from(name.as_str())
            }
            Err(e) => {
                self.last_error = e;
                GString::new()
            }
        }
    }

    #[func]
    fn remove(&mut self, names: PackedStringArray) -> bool {
        let r = self.editor.remove(&names_of(&names));
        self.done(r)
    }

    #[func]
    fn connect_objects(&mut self, from: GString, to: GString) -> bool {
        let r = self.editor.connect(&from.to_string(), &to.to_string());
        self.done(r)
    }

    #[func]
    fn disconnect_objects(&mut self, from: GString, to: GString) -> bool {
        let r = self.editor.disconnect(&from.to_string(), &to.to_string());
        self.done(r)
    }

    /// Moves output `index` (0-based) of `from` by `delta` places in the port order.
    #[func]
    fn move_connection(&mut self, from: GString, index: i64, delta: i64) -> bool {
        let r = self
            .editor
            .move_connection(&from.to_string(), index.max(0) as usize, delta);
        self.done(r)
    }

    #[func]
    fn begin_drag(&mut self, names: PackedStringArray) {
        self.editor.begin_drag(&names_of(&names));
    }

    /// Moves the dragged objects by `delta` (Godot floor vector) from where the drag
    /// started, snapping the first one to `step`.
    #[func]
    fn drag(&mut self, delta: Vector3, step: f64) -> bool {
        let d = to_model(delta);
        let r = self.editor.drag([d[0], d[1]], step);
        self.done(r)
    }

    #[func]
    fn end_drag(&mut self) {
        self.editor.end_drag();
    }

    /// Moves `names` by `delta` (Godot floor vector) in one step.
    #[func]
    fn nudge(&mut self, names: PackedStringArray, delta: Vector3) -> bool {
        let d = to_model(delta);
        let r = self.editor.nudge(&names_of(&names), [d[0], d[1]]);
        self.done(r)
    }

    #[func]
    fn copy(&mut self, names: PackedStringArray) -> i64 {
        self.editor.copy(&names_of(&names)) as i64
    }

    #[func]
    fn cut(&mut self, names: PackedStringArray) -> i64 {
        match self.editor.cut(&names_of(&names)) {
            Ok(n) => n as i64,
            Err(e) => {
                self.last_error = e;
                0
            }
        }
    }

    #[func]
    fn has_clipboard(&self) -> bool {
        self.editor.has_clipboard()
    }

    /// Pastes the clipboard; returns the new names (empty on error).
    #[func]
    fn paste(&mut self) -> PackedStringArray {
        match self.editor.paste() {
            Ok(n) => strings(&n),
            Err(e) => {
                self.last_error = e;
                PackedStringArray::new()
            }
        }
    }

    #[func]
    fn duplicate(&mut self, names: PackedStringArray) -> PackedStringArray {
        match self.editor.duplicate(&names_of(&names)) {
            Ok(n) => strings(&n),
            Err(e) => {
                self.last_error = e;
                PackedStringArray::new()
            }
        }
    }

    /// `how`: `esquerda`, `centro_x`, `direita`, `topo`, `centro_y` or `base`.
    #[func]
    fn align(&mut self, names: PackedStringArray, how: GString) -> bool {
        let r = self.editor.align(&names_of(&names), &how.to_string());
        self.done(r)
    }

    /// `axis`: `x` or `y` (model plan axes).
    #[func]
    fn distribute(&mut self, names: PackedStringArray, axis: GString) -> bool {
        let r = self.editor.distribute(&names_of(&names), &axis.to_string());
        self.done(r)
    }

    /// Sets property `path` of `object` (`"run"`: the run settings) to `value`, and the
    /// same on `others` when they are of its kind. See `schema` for the paths.
    #[func]
    fn set_property(
        &mut self,
        object: GString,
        path: GString,
        value: Variant,
        others: PackedStringArray,
    ) -> bool {
        let r = self.editor.set_property(
            &object.to_string(),
            &path.to_string(),
            &value_of(&value),
            &names_of(&others),
        );
        self.done(r)
    }

    /// The current RON text of field `key` of `object` (as `Command::Set` takes it), or
    /// `""`.
    #[func]
    fn field_text(&self, object: GString, key: GString) -> GString {
        let o = object.to_string();
        self.editor
            .model()
            .objects
            .iter()
            .find(|x| x.name == o)
            .and_then(|x| schema::read_field(x, &key.to_string()))
            .map_or_else(GString::new, |t| GString::from(t.as_str()))
    }

    /// What differs between the model of file `got` and that of file `want` (one line
    /// per difference, empty when they are the same model; comments and formatting do not
    /// count). When a file cannot be read, the only line starts with `erro:`.
    #[func]
    fn file_differences(got: GString, want: GString) -> PackedStringArray {
        let got = PathBuf::from(got.to_string());
        let want = PathBuf::from(want.to_string());
        match document::file_differences(&got, &want) {
            Ok(lines) => strings(&lines),
            Err(e) => strings(&[format!("erro: {e}")]),
        }
    }

    /// Objects in model order: `{name, kind, kind_label, position (Godot), model_position,
    /// mobile, problems, warnings}`.
    #[func]
    fn objects(&self) -> VarArray {
        let mut out = VarArray::new();
        for o in self.editor.objects() {
            let mut d = VarDictionary::new();
            put(&mut d, "name", GString::from(o.name.as_str()));
            put(&mut d, "kind", o.kind);
            put(&mut d, "kind_label", o.kind_label);
            put(&mut d, "position", to_godot(o.position));
            put(
                &mut d,
                "model_position",
                Vector3::new(
                    o.position[0] as f32,
                    o.position[1] as f32,
                    o.position[2] as f32,
                ),
            );
            put(&mut d, "mobile", o.mobile);
            put(&mut d, "problems", o.problems as i64);
            put(&mut d, "warnings", o.warnings as i64);
            out.push(&d.to_variant());
        }
        out
    }

    /// Connections in model order: `{from, to, port, logical}` (`port` counts from 1
    /// among the outputs of `from`).
    #[func]
    fn connections(&self) -> VarArray {
        let mut out = VarArray::new();
        for c in self.editor.connections() {
            let mut d = VarDictionary::new();
            put(&mut d, "from", GString::from(c.from.as_str()));
            put(&mut d, "to", GString::from(c.to.as_str()));
            put(&mut d, "port", c.port as i64);
            put(&mut d, "logical", c.logical);
            out.push(&d.to_variant());
        }
        out
    }

    /// Problems that keep the model from running: `{object, message}`.
    #[func]
    fn issues(&self) -> VarArray {
        issues_array(&self.editor.doc.issues())
    }

    /// Warnings about a model that runs: `{object, message}`.
    #[func]
    fn warnings(&self) -> VarArray {
        issues_array(&self.editor.doc.warnings())
    }

    /// The properties of `object` (or of the run settings for `"run"`) as a schema:
    /// `{name, kind, kind_label, fields, problems, warnings, outputs, inputs}`.
    #[func]
    fn properties(&self, object: GString) -> VarDictionary {
        properties_dict(&self.editor.properties(&object.to_string()))
    }

    /// The model as it is now, ready to run (`UtsimModel.run` and `playback`). It is not
    /// validated here: `UtsimModel.is_loaded` says whether it can run.
    #[func]
    fn as_model(&self) -> Gd<UtsimModel> {
        let model = self.editor.model().clone();
        let valid = model.validate().is_ok();
        let path = self.editor.doc.path().map(|p| p.to_path_buf());
        Gd::from_init_fn(|base| UtsimModel {
            model: valid.then_some(model),
            path,
            last_error: String::new(),
            base,
        })
    }

    /// The scene of the model as it is now, with no run (Editar): the same static layout
    /// as a playback, every block where the model puts it. `null` if it cannot be drawn.
    #[func]
    fn edit_scene(&mut self, skeletal_people: bool) -> Option<Gd<UtsimPlayback>> {
        let dir = self
            .editor
            .doc
            .path()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf());
        match edit_playback(self.editor.model(), dir.as_deref(), skeletal_people) {
            Some(core) => Some(playback_object(core)),
            None => {
                self.last_error = "não foi possível desenhar o modelo".into();
                None
            }
        }
    }

    /// The scene of one object of palette entry `id` at the origin (the ghost shown while
    /// placing it), or `null`.
    #[func]
    fn palette_scene(id: GString, skeletal_people: bool) -> Option<Gd<UtsimPlayback>> {
        let model = palette_model(&id.to_string())?;
        edit_playback(&model, None, skeletal_people).map(playback_object)
    }

    /// The object library: `{id, kind, label, category, name_base, asset}` in
    /// [`utsim_edit::PALETTE`] order.
    #[func]
    fn palette() -> VarArray {
        let mut out = VarArray::new();
        for e in utsim_edit::PALETTE {
            let mut d = VarDictionary::new();
            put(&mut d, "id", e.id);
            put(&mut d, "kind", e.kind);
            put(&mut d, "label", e.label);
            put(&mut d, "category", e.category);
            put(&mut d, "name_base", e.name_base);
            put(&mut d, "asset", e.asset.unwrap_or(""));
            out.push(&d.to_variant());
        }
        out
    }

    #[func]
    fn categories() -> PackedStringArray {
        strings(&utsim_edit::CATEGORIES)
    }

    /// Distribution family names, in the order of a field's `dist.family`.
    #[func]
    fn families() -> PackedStringArray {
        strings(&schema::FAMILIES)
    }

    /// The "Atalhos" table: `[[shortcut, what it does], ...]`.
    #[func]
    fn shortcuts() -> VarArray {
        let mut out = VarArray::new();
        for (k, v) in document::SHORTCUTS {
            out.push(&strings(&[k, v]).to_variant());
        }
        out
    }
}
