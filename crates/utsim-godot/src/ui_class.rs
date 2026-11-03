//! `UtsimUi`: the interface around the 3D view, drawn by egui inside the Godot window.
//!
//! A `Node2D` the main scene puts in its `CanvasLayer`. It reads Godot's input in
//! `_input` (before the GUI and the 3D view), gives it to the egui [`Host`] and marks the
//! event handled when it belongs to egui; it runs an egui frame only when something changed
//! and draws the output with `RenderingServer` canvas items, one per clipped mesh, with
//! the egui textures uploaded as Godot textures. The actions clicked come out as the
//! `action(what)` signal, emitted deferred (outside this object's borrow), and the main
//! scene runs them as the old Godot buttons did.
//!
//! Only the core classes of Godot are generated for the bridge (no `__codegen-full`), so
//! the input events, `Image` and `DisplayServer` are reached through dynamic calls
//! (`get`, `call`): a handful per event or per texture change.

use std::collections::HashMap;
use std::time::Instant;

use egui::TextureId;
use godot::classes::input::CursorShape;
use godot::classes::notify::CanvasItemNotification;
use godot::classes::rendering_server::CanvasItemTextureFilter;
use godot::classes::{
    ClassDb, Engine, INode2D, Input as GdInput, InputEvent, Node2D, Object, Os, RenderingServer,
    Resource, Viewport,
};
use godot::prelude::*;

use crate::document_class::UtsimDocument;
use crate::egui_host::Host;
use crate::egui_input::{Input, Mods};
use crate::egui_mesh::{self, Change, Textures};
use crate::schema::{self, Value};
use crate::ui_bar::BarState;
use crate::ui_bottom::{PlayView, StatusView};
use crate::ui_left::{EditView, Entry, ObjectItem, Summary};
use crate::ui_props::{self, PropAction, PropsView};
use crate::{put, strings};

/// Godot's `Image.FORMAT_RGBA8`.
const FORMAT_RGBA8: i64 = 5;

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct UtsimUi {
    host: Host,
    start: Instant,
    textures: Textures,
    /// The Godot texture of each egui texture.
    gpu: HashMap<TextureId, Rid>,
    /// Canvas items, one per mesh drawn (kept and reused between frames).
    items: Vec<Rid>,
    /// Interface scale (pixels per point) over the window's content scale.
    #[var]
    ui_scale: f32,
    cursor: Option<egui::CursorIcon>,
    /// The document the properties panel edits.
    doc: Option<Gd<UtsimDocument>>,
    /// The object shown in the properties panel (`run` for the model), the others
    /// selected with it, the line under the title and the driver state.
    object: String,
    others: Vec<String>,
    extra: String,
    driver: i32,
    props_on: bool,
    /// The revision and object the cached view was built for.
    props_key: Option<(u64, String)>,
    /// The texture of the miniature's `SubViewport` (owned by Godot, never freed here).
    miniature: Option<Rid>,
    /// The library icons: entry id to egui texture, and those textures to Godot's (owned
    /// by Godot's resources, kept alive by `icon_refs`).
    icons: HashMap<String, TextureId>,
    user: HashMap<TextureId, Rid>,
    icon_refs: Vec<Gd<Resource>>,
    /// The revision the object list was built for (`None`: read it again).
    objects_rev: Option<u64>,
    /// What the main scene set for the left panel of Editar (the objects come from the
    /// document).
    edit_state: EditView,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for UtsimUi {
    fn init(base: Base<Node2D>) -> Self {
        UtsimUi {
            host: Host::new(),
            start: Instant::now(),
            textures: Textures::default(),
            gpu: HashMap::new(),
            items: Vec::new(),
            ui_scale: 1.0,
            cursor: None,
            doc: None,
            object: String::new(),
            others: Vec::new(),
            extra: String::new(),
            driver: -1,
            props_on: false,
            props_key: None,
            miniature: None,
            icons: HashMap::new(),
            user: HashMap::new(),
            icon_refs: Vec::new(),
            objects_rev: None,
            edit_state: EditView::default(),
            base,
        }
    }

    fn ready(&mut self) {
        // Over the Godot panels: its pop-ups (menus, lists) open over them.
        self.base_mut().set_z_index(100);
        self.base_mut().set_process(true);
        self.base_mut().set_process_input(true);
    }

    fn process(&mut self, _delta: f64) {
        self.frame_if_needed();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Some(input) = read_input(&event) else {
            return;
        };
        let press = matches!(input, Input::Button { pressed: true, .. }) && !input.is_wheel();
        let consumed = self.host.push(&input, clipboard_get);
        if consumed && let Some(mut vp) = self.base().get_viewport() {
            // A click on egui takes the keyboard from a Godot field (the object search, a
            // field of a Godot panel): what is typed next goes to egui or to the shortcuts.
            if press {
                vp.gui_release_focus();
            }
            vp.set_input_as_handled();
        }
    }

    fn on_notification(&mut self, what: CanvasItemNotification) {
        match what {
            CanvasItemNotification::WM_MOUSE_EXIT => {
                self.host.push(&Input::PointerGone, String::new);
            }
            CanvasItemNotification::WM_WINDOW_FOCUS_OUT => {
                self.host.push(&Input::Focus(false), String::new);
            }
            CanvasItemNotification::WM_WINDOW_FOCUS_IN => {
                self.host.push(&Input::Focus(true), String::new);
            }
            CanvasItemNotification::PREDELETE => self.free_all(),
            _ => {}
        }
    }
}

#[godot_api]
impl UtsimUi {
    /// An action of the interface (`novo`, `abrir`, `modo:simular`, `ferramenta:ligar`,
    /// `organizar:alinhar:esquerda`, `grade:ligar`, `passo:0.5`, `analisar:resultados`...).
    #[signal]
    fn action(what: GString);

    /// What the top bar shows: `{editing, tool, snap, step, can_undo, can_redo, title,
    /// title_tip}` (missing keys keep their value).
    #[func]
    fn set_bar(&mut self, state: VarDictionary) {
        let mut s: BarState = self.host.bar.clone();
        let get = |k: &str| state.get(&k.to_variant());
        if let Some(v) = get("editing") {
            s.editing = v.try_to::<bool>().unwrap_or(s.editing);
        }
        if let Some(v) = get("tool") {
            s.tool = v
                .try_to::<GString>()
                .map(|g| g.to_string())
                .unwrap_or(s.tool);
        }
        if let Some(v) = get("snap") {
            s.snap = v.try_to::<bool>().unwrap_or(s.snap);
        }
        if let Some(v) = get("step") {
            s.step = v.try_to::<f64>().unwrap_or(s.step);
        }
        if let Some(v) = get("can_undo") {
            s.can_undo = v.try_to::<bool>().unwrap_or(s.can_undo);
        }
        if let Some(v) = get("can_redo") {
            s.can_redo = v.try_to::<bool>().unwrap_or(s.can_redo);
        }
        if let Some(v) = get("title") {
            s.title = v
                .try_to::<GString>()
                .map(|g| g.to_string())
                .unwrap_or(s.title);
        }
        if let Some(v) = get("title_tip") {
            s.title_tip = v
                .try_to::<GString>()
                .map(|g| g.to_string())
                .unwrap_or(s.title_tip);
        }
        self.host.set_bar(s);
    }

    /// The part of the window egui leaves to the 3D view, in pixels.
    #[func]
    fn central_rect(&self) -> Rect2 {
        let [x, y, w, h] = self.host.central_pixels();
        Rect2::new(Vector2::new(x, y), Vector2::new(w, h))
    }

    /// Whether screen point `p` is over an egui panel or pop-up.
    #[func]
    fn is_over(&self, p: Vector2) -> bool {
        self.host.is_over([p.x, p.y])
    }

    /// Whether an egui text field has the keyboard focus.
    #[func]
    fn is_typing(&self) -> bool {
        self.host.typing()
    }

    /// The rectangle of the named widget in pixels (zero size when it is not on screen).
    #[func]
    fn widget_rect(&self, name: GString) -> Rect2 {
        match self
            .host
            .reg
            .rect_pixels(&name.to_string(), self.host.ppp())
        {
            Some([x, y, w, h]) => Rect2::new(Vector2::new(x, y), Vector2::new(w, h)),
            None => Rect2::default(),
        }
    }

    #[func]
    fn has_widget(&self, name: GString) -> bool {
        self.host.reg.get(&name.to_string()).is_some()
    }

    /// Whether the named widget was drawn, also when it is scrolled out of sight.
    #[func]
    fn knows_widget(&self, name: GString) -> bool {
        self.host.reg.knows(&name.to_string())
    }

    /// `{text, enabled, checked, choices}` of the named widget, or `{}`.
    #[func]
    fn widget_info(&self, name: GString) -> VarDictionary {
        let mut d = VarDictionary::new();
        if let Some(w) = self.host.reg.get(&name.to_string()) {
            put(&mut d, "text", GString::from(w.text.as_str()));
            put(&mut d, "enabled", w.enabled);
            if let Some(on) = w.checked {
                put(&mut d, "checked", on);
            }
            put(&mut d, "choices", strings(&w.choices));
        }
        d
    }

    /// The names of the widgets on screen.
    #[func]
    fn widget_names(&self) -> PackedStringArray {
        strings(&self.host.reg.names())
    }

    /// Picks `item` in the named list, as choosing it in the open list does (the
    /// scripted `escolher` step). False when the list or the item is not there.
    #[func]
    fn choose(&mut self, name: GString, item: GString) -> bool {
        self.host.reg.request(&name.to_string(), &item.to_string())
    }

    /// Asks for a new frame (something the interface shows changed).
    #[func]
    fn repaint(&mut self) {
        self.props_key = None;
        self.host.invalidate();
    }

    /// The document the properties panel reads and edits.
    #[func]
    fn set_document(&mut self, doc: Gd<UtsimDocument>) {
        self.doc = Some(doc);
        self.props_key = None;
        self.host.invalidate();
    }

    /// Shows the properties of `object` (`"run"`: the model), with `others` selected with
    /// it (the edits apply to them too), `extra` under the title and the forklift driver
    /// state (`-1` none, `0` hidden, `1` shown).
    #[func]
    fn show_properties(
        &mut self,
        object: GString,
        others: PackedStringArray,
        extra: GString,
        driver: i64,
    ) {
        self.object = object.to_string();
        self.others = others.as_slice().iter().map(|s| s.to_string()).collect();
        self.extra = extra.to_string();
        self.driver = driver as i32;
        self.props_on = true;
        self.props_key = None;
        self.host.invalidate();
    }

    /// Shows or hides the properties panel (hidden: the Godot panel or nothing).
    #[func]
    fn set_properties_visible(&mut self, on: bool) {
        if self.props_on != on {
            self.props_on = on;
            self.props_key = None;
            self.host.invalidate();
        }
    }

    #[func]
    fn properties_visible(&self) -> bool {
        self.props_on
    }

    /// The icons of the library: `{id: Texture2D}` (`res://icones/<id>.svg`).
    #[func]
    fn set_library_icons(&mut self, icons: VarDictionary) {
        self.icons.clear();
        self.user.clear();
        self.icon_refs.clear();
        for (k, (id, tex)) in icons.iter_shared().enumerate() {
            let Ok(res) = tex.try_to::<Gd<Resource>>() else {
                continue;
            };
            let t = TextureId::User(100 + k as u64);
            self.user.insert(t, res.get_rid());
            self.icons.insert(id.to_string(), t);
            self.icon_refs.push(res);
        }
        self.objects_rev = None;
        self.edit_state.library.clear();
        self.host.invalidate();
    }

    /// The left panel of Editar: `{armed, selected, obstacles}` (missing keys keep their
    /// value); the library and the objects come from the bridge and the document.
    #[func]
    fn set_edit_state(&mut self, state: VarDictionary) {
        let get = |k: &str| state.get(&k.to_variant());
        if let Some(v) = get("armed") {
            self.edit_state.armed = v.to_string();
        }
        if let Some(v) = get("selected")
            && let Ok(list) = v.try_to::<PackedStringArray>()
        {
            self.edit_state.selected = list.as_slice().iter().map(|s| s.to_string()).collect();
        }
        if let Some(v) = get("obstacles") {
            self.edit_state.obstacles = v.try_to::<bool>().unwrap_or(false);
        }
        // The main scene calls this after every change of the document, also after
        // opening another model (whose revision may equal the old one's): the objects are
        // read again.
        self.objects_rev = None;
        self.refresh_edit();
    }

    /// The left panel of Simular: `{reps, status, blocks, follow, camera, labels, shadows,
    /// summary: {title, bottleneck, lines}}` (missing keys keep their value; `follow` is
    /// the index into `blocks`, -1 for none).
    #[func]
    fn set_sim_state(&mut self, state: VarDictionary) {
        let mut v = self.host.sim().clone();
        let get = |k: &str| state.get(&k.to_variant());
        if let Some(x) = get("reps") {
            v.reps = x.try_to::<i64>().unwrap_or(v.reps);
        }
        if let Some(x) = get("status") {
            v.status = x.to_string();
        }
        if let Some(x) = get("blocks")
            && let Ok(list) = x.try_to::<PackedStringArray>()
        {
            v.blocks = list.as_slice().iter().map(|s| s.to_string()).collect();
        }
        if let Some(x) = get("follow") {
            v.follow = x
                .try_to::<i64>()
                .ok()
                .filter(|k| *k >= 0)
                .map(|k| k as usize);
        }
        if let Some(x) = get("camera") {
            v.camera = x.to_string();
        }
        if let Some(x) = get("labels") {
            v.labels = x.try_to::<bool>().unwrap_or(v.labels);
        }
        if let Some(x) = get("shadows") {
            v.shadows = x.try_to::<bool>().unwrap_or(v.shadows);
        }
        if let Some(x) = get("summary")
            && let Ok(d) = x.try_to::<VarDictionary>()
        {
            let text = |k: &str| {
                d.get(&k.to_variant())
                    .map(|x| x.to_string())
                    .unwrap_or_default()
            };
            v.summary = Summary {
                title: text("title"),
                bottleneck: text("bottleneck"),
                lines: d
                    .get(&"lines".to_variant())
                    .and_then(|x| x.try_to::<PackedStringArray>().ok())
                    .map(|l| l.as_slice().iter().map(|s| s.to_string()).collect())
                    .unwrap_or_default(),
            };
        }
        self.host.set_sim(v);
    }

    /// The status bar: `{text, progress, percent}`; `progress` below 0 hides the progress
    /// bar and "Cancelar" (no job).
    #[func]
    fn set_status_bar(&mut self, state: VarDictionary) {
        let mut v: StatusView = self.host.status().clone();
        let get = |k: &str| state.get(&k.to_variant());
        if let Some(x) = get("text") {
            v.text = x.to_string();
        }
        if let Some(x) = get("progress") {
            let p = x.try_to::<f64>().unwrap_or(-1.0);
            v.progress = (p >= 0.0).then_some(p as f32);
        }
        if let Some(x) = get("percent") {
            v.percent = x.try_to::<bool>().unwrap_or(v.percent);
        }
        self.host.set_status(v);
    }

    /// The text of the status bar.
    #[func]
    fn status_text(&self) -> GString {
        GString::from(self.host.status().text.as_str())
    }

    /// The playback bar: `{visible, playing, t, end, speed}` (missing keys keep their value).
    #[func]
    fn set_playback_bar(&mut self, state: VarDictionary) {
        let mut v: PlayView = self.host.play().clone();
        let get = |k: &str| state.get(&k.to_variant());
        if let Some(x) = get("visible") {
            v.visible = x.try_to::<bool>().unwrap_or(v.visible);
        }
        if let Some(x) = get("playing") {
            v.playing = x.try_to::<bool>().unwrap_or(v.playing);
        }
        if let Some(x) = get("t") {
            v.t = x.try_to::<f64>().unwrap_or(v.t);
        }
        if let Some(x) = get("end") {
            v.end = x.try_to::<f64>().unwrap_or(v.end);
        }
        if let Some(x) = get("speed") {
            v.speed = x.try_to::<f64>().unwrap_or(v.speed);
        }
        self.host.set_play(v);
    }

    /// The `SubViewport` the asset miniature is drawn in: its texture is shown in the panel.
    #[func]
    fn set_miniature(&mut self, viewport: Gd<Viewport>) {
        let rid = viewport.get_viewport_rid();
        self.miniature = Some(RenderingServer::singleton().viewport_get_texture(rid));
        self.props_key = None;
        self.host.invalidate();
    }

    /// Scrolls the named widget into view (a scripted step will act on it). False when it
    /// was not drawn.
    #[func]
    fn reveal(&mut self, name: GString) -> bool {
        self.host.reg.reveal(&name.to_string())
    }

    /// Frames run so far (frames run only when something changed).
    #[func]
    fn frames_run(&self) -> i64 {
        self.host.frames as i64
    }

    /// Runs and paints a frame now if one is needed (scripts call it after a change).
    #[func]
    fn update_now(&mut self) {
        self.frame_if_needed();
    }
}

impl UtsimUi {
    fn screen(&self) -> ([f32; 2], f32) {
        let Some(vp) = self.base().get_viewport() else {
            return ([0.0, 0.0], 1.0);
        };
        let size = vp.get_visible_rect().size;
        let content = vp
            .get_window()
            .map(|w| w.get_content_scale_factor())
            .unwrap_or(1.0);
        let content = if content > 0.0 { content } else { 1.0 };
        ([size.x, size.y], content * self.ui_scale.max(0.25))
    }

    fn frame_if_needed(&mut self) {
        let (size, ppp) = self.screen();
        if size[0] < 1.0 || size[1] < 1.0 {
            return;
        }
        let now = self.start.elapsed().as_secs_f64();
        if !self.host.needs_frame(now, size, ppp) {
            return;
        }
        self.refresh_props_view();
        self.refresh_edit();
        let frame = self.host.frame(now, size, ppp);
        let mut output = frame.output;
        let mut textures = std::mem::take(&mut output.textures_delta);
        self.platform(&output.platform_output);
        for (id, deltas) in &textures.set {
            for d in deltas {
                match self.textures.apply(*id, d) {
                    Change::Created => self.upload(*id, true),
                    Change::Updated => self.upload(*id, false),
                    Change::Ignored => {}
                }
            }
        }
        let prims = self
            .host
            .ctx
            .tessellate(output.shapes, output.pixels_per_point);
        self.paint(&prims, output.pixels_per_point, size);
        for id in &textures.free {
            self.textures.free(*id);
            if let Some(rid) = self.gpu.remove(id) {
                RenderingServer::singleton().free_rid(rid);
            }
        }
        // Applied above (egui checks that every delta was handled).
        textures.clear();
        for a in frame.actions {
            self.emit(&a);
        }
        for a in frame.props {
            self.prop_action(a);
        }
    }

    /// Emits `action(what)` after this frame (outside this object's borrow).
    fn emit(&mut self, what: &str) {
        let args = [
            StringName::from("action").to_variant(),
            GString::from(what).to_variant(),
        ];
        self.base_mut().call_deferred("emit_signal", &args);
    }

    /// The left panel of Editar: the library with its icons, the document's objects (read
    /// again only when the document changed) and the state the main scene set.
    fn refresh_edit(&mut self) {
        let rev = self.doc.as_ref().map(|d| d.bind().editor.doc.revision());
        if rev != self.objects_rev || self.edit_state.library.is_empty() {
            self.objects_rev = rev;
            self.edit_state.library = crate::document::palette()
                .iter()
                .map(|p| Entry {
                    id: p.id.to_owned(),
                    label: p.label.to_owned(),
                    category: p.category.to_owned(),
                    icon: self.icons.get(p.id).copied(),
                })
                .collect();
            self.edit_state.categories = utsim_edit::CATEGORIES
                .iter()
                .map(|c| c.to_string())
                .collect();
            self.edit_state.objects = match &self.doc {
                Some(d) => d
                    .bind()
                    .editor
                    .objects()
                    .into_iter()
                    .map(|o| ObjectItem {
                        name: o.name,
                        kind_label: o.kind_label.to_owned(),
                        problems: o.problems,
                        warnings: o.warnings,
                    })
                    .collect(),
                None => Vec::new(),
            };
        }
        self.host.set_edit(self.edit_state.clone());
    }

    /// The properties view from the document, rebuilt only when the document or the
    /// object shown changed.
    fn refresh_props_view(&mut self) {
        let Some(doc) = self.doc.clone().filter(|_| self.props_on) else {
            self.host.props = None;
            self.props_key = None;
            return;
        };
        let d = doc.bind();
        let key = (d.editor.doc.revision(), self.object.clone());
        if self.props_key.as_ref() == Some(&key) && self.host.props.is_some() {
            return;
        }
        let props = d.editor.properties(&self.object);
        let prefix = format!("{}.", props.name);
        let data_paths = d
            .data_list()
            .iter()
            .filter_map(|f| {
                f["field"]
                    .as_str()
                    .and_then(|k| k.strip_prefix(&prefix))
                    .map(str::to_owned)
            })
            .collect();
        self.host.props = Some(PropsView {
            props,
            families: schema::FAMILIES.iter().map(|s| s.to_string()).collect(),
            data_paths,
            extra: self.extra.clone(),
            driver: self.driver,
            miniature: self.miniature.is_some(),
        });
        self.props_key = Some(key);
    }

    /// Applies an edit of the properties panel through the document (validation, undo)
    /// and tells the main scene; the rest goes to the main scene as actions.
    fn prop_action(&mut self, a: PropAction) {
        match a {
            PropAction::Set {
                path,
                value,
                end_gesture,
            } => {
                let Some(mut doc) = self.doc.clone() else {
                    return;
                };
                let object = self.object.clone();
                let r = {
                    let mut d = doc.bind_mut();
                    let r = d.editor.set_property(&object, &path, &value, &self.others);
                    if r.is_ok() && end_gesture {
                        d.editor.doc.end_gesture();
                    }
                    r
                };
                self.props_key = None;
                self.host.invalidate();
                match r {
                    Ok(()) => {
                        let shown = value_string(&value);
                        if path == "name" {
                            self.object = shown.trim().to_owned();
                        }
                        self.emit(&format!("prop_ok\t{object}\t{path}\t{shown}"));
                    }
                    Err(e) => self.emit(&format!("prop_erro\t{path}\t{e}")),
                }
            }
            PropAction::EndGesture => {
                if let Some(mut doc) = self.doc.clone() {
                    doc.bind_mut().editor.doc.end_gesture();
                }
                self.emit("prop_fim");
            }
            PropAction::Issue(m) => self.emit(&format!("problema\t{m}")),
            PropAction::Arrange(w) => self.emit(&format!("organizar:{w}")),
            PropAction::Reorder(k, d) => self.emit(&format!("saida_ordem\t{k}\t{d}")),
            PropAction::Disconnect(to) => self.emit(&format!("saida_remover\t{to}")),
            PropAction::Driver(on) => {
                let o = self.object.clone();
                self.emit(&format!("motorista\t{o}\t{on}"));
            }
            PropAction::DataMode(p) => self.emit(&format!("dados_reais\t{p}")),
            PropAction::Miniature(asset) => self.emit(&format!("miniatura\t{asset}")),
        }
    }

    /// Clipboard, cursor and links.
    fn platform(&mut self, out: &egui::PlatformOutput) {
        for c in &out.commands {
            match c {
                egui::OutputCommand::CopyText(text) => clipboard_set(text),
                egui::OutputCommand::OpenUrl(u) => {
                    Os::singleton().shell_open(u.url.as_str());
                }
                egui::OutputCommand::CopyImage(_) => {}
            }
        }
        if self.cursor != Some(out.cursor_icon) {
            self.cursor = Some(out.cursor_icon);
            let shape = cursor_shape(out.cursor_icon);
            GdInput::singleton()
                .set_default_cursor_shape_ex()
                .shape(shape)
                .done();
        }
    }

    /// Sends the CPU copy of texture `id` to Godot (a new texture when `create`).
    fn upload(&mut self, id: TextureId, create: bool) {
        let Some(img) = self.textures.images.get(&id) else {
            return;
        };
        let mut image: Gd<Object> = match ClassDb::singleton().instantiate("Image").try_to() {
            Ok(i) => i,
            Err(_) => return,
        };
        let bytes = PackedByteArray::from(img.rgba.as_slice());
        image.call(
            "set_data",
            &[
                (img.size[0] as i64).to_variant(),
                (img.size[1] as i64).to_variant(),
                false.to_variant(),
                FORMAT_RGBA8.to_variant(),
                bytes.to_variant(),
            ],
        );
        let mut rs = RenderingServer::singleton();
        match self.gpu.get(&id).copied() {
            Some(rid) if !create => {
                rs.call(
                    "texture_2d_update",
                    &[rid.to_variant(), image.to_variant(), 0i64.to_variant()],
                );
            }
            old => {
                if let Some(rid) = old {
                    rs.free_rid(rid);
                }
                let rid = rs
                    .call("texture_2d_create", &[image.to_variant()])
                    .try_to::<Rid>()
                    .unwrap_or(Rid::Invalid);
                self.gpu.insert(id, rid);
            }
        }
    }

    /// Draws the meshes, one canvas item each, clipped.
    fn paint(&mut self, prims: &[egui::ClippedPrimitive], ppp: f32, size: [f32; 2]) {
        let parent = self.base().get_canvas_item();
        let mut rs = RenderingServer::singleton();
        let mut used = 0;
        for prim in prims {
            let Some(m) = egui_mesh::convert(prim, ppp, size) else {
                continue;
            };
            if used == self.items.len() {
                let item = rs.canvas_item_create();
                rs.canvas_item_set_parent(item, parent);
                rs.canvas_item_set_default_texture_filter(item, CanvasItemTextureFilter::LINEAR);
                rs.canvas_item_set_clip(item, true);
                self.items.push(item);
            }
            let item = self.items[used];
            rs.canvas_item_clear(item);
            rs.canvas_item_set_draw_index(item, used as i32);
            let [x, y, w, h] = m.clip;
            rs.canvas_item_set_custom_rect_ex(item, true)
                .rect(Rect2::new(Vector2::new(x, y), Vector2::new(w, h)))
                .done();
            let indices = PackedInt32Array::from(m.indices.as_slice());
            let points: PackedVector2Array =
                m.points.iter().map(|p| Vector2::new(p[0], p[1])).collect();
            let colors: PackedColorArray = m
                .colors
                .iter()
                .map(|c| Color::from_rgba(c[0], c[1], c[2], c[3]))
                .collect();
            let uvs: PackedVector2Array = m.uvs.iter().map(|p| Vector2::new(p[0], p[1])).collect();
            let texture = m
                .texture
                .and_then(|t| {
                    if t == ui_props::MINIATURE {
                        self.miniature
                    } else if let Some(rid) = self.user.get(&t) {
                        Some(*rid)
                    } else {
                        self.gpu.get(&t).copied()
                    }
                })
                .unwrap_or(Rid::Invalid);
            rs.canvas_item_add_triangle_array_ex(item, &indices, &points, &colors)
                .uvs(&uvs)
                .texture(texture)
                .done();
            used += 1;
        }
        for item in &self.items[used..] {
            rs.canvas_item_clear(*item);
        }
    }

    fn free_all(&mut self) {
        let mut rs = RenderingServer::singleton();
        for item in self.items.drain(..) {
            rs.free_rid(item);
        }
        for (_, rid) in self.gpu.drain() {
            rs.free_rid(rid);
        }
    }
}

/// The plain data of a Godot input event (dynamic reads: the event classes are not
/// generated for the bridge).
fn read_input(ev: &Gd<InputEvent>) -> Option<Input> {
    let class = ev.get_class().to_string();
    let b = |k: &str| ev.get(k).try_to::<bool>().unwrap_or(false);
    let mods = || Mods {
        shift: b("shift_pressed"),
        ctrl: b("ctrl_pressed"),
        alt: b("alt_pressed"),
        meta: b("meta_pressed"),
    };
    let pos = || {
        let p = ev.get("position").try_to::<Vector2>().unwrap_or_default();
        [p.x, p.y]
    };
    match class.as_str() {
        "InputEventMouseMotion" => Some(Input::Motion {
            pos: pos(),
            mods: mods(),
        }),
        "InputEventMouseButton" => Some(Input::Button {
            pos: pos(),
            button: ev.get("button_index").try_to::<i64>().unwrap_or(0),
            pressed: b("pressed"),
            factor: ev.get("factor").try_to::<f32>().unwrap_or(1.0),
            mods: mods(),
        }),
        "InputEventKey" => Some(Input::Key {
            keycode: ev.get("keycode").try_to::<i64>().unwrap_or(0),
            unicode: ev.get("unicode").try_to::<i64>().unwrap_or(0).max(0) as u32,
            pressed: b("pressed"),
            echo: b("echo"),
            mods: mods(),
        }),
        _ => None,
    }
}

/// A value as the main scene's status line shows it.
fn value_string(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(x) => ui_props::num_text(*x),
        Value::Text(t) => t.clone(),
        Value::List(l) => l.join(", "),
    }
}

fn display_server() -> Option<Gd<Object>> {
    Engine::singleton().get_singleton("DisplayServer")
}

fn clipboard_get() -> String {
    display_server()
        .map(|mut d| d.call("clipboard_get", &[]).to_string())
        .unwrap_or_default()
}

fn clipboard_set(text: &str) {
    if let Some(mut d) = display_server() {
        d.call("clipboard_set", &[GString::from(text).to_variant()]);
    }
}

fn cursor_shape(c: egui::CursorIcon) -> CursorShape {
    use egui::CursorIcon as C;
    match c {
        C::PointingHand => CursorShape::POINTING_HAND,
        C::Text | C::VerticalText => CursorShape::IBEAM,
        C::ResizeHorizontal | C::ResizeColumn | C::ResizeEast | C::ResizeWest => CursorShape::HSIZE,
        C::ResizeVertical | C::ResizeRow | C::ResizeNorth | C::ResizeSouth => CursorShape::VSIZE,
        C::ResizeNeSw | C::ResizeNorthEast | C::ResizeSouthWest => CursorShape::BDIAGSIZE,
        C::ResizeNwSe | C::ResizeNorthWest | C::ResizeSouthEast => CursorShape::FDIAGSIZE,
        C::Grab | C::Grabbing | C::Move | C::AllScroll => CursorShape::MOVE,
        C::NotAllowed | C::NoDrop => CursorShape::FORBIDDEN,
        C::Wait => CursorShape::WAIT,
        C::Progress => CursorShape::BUSY,
        C::Help => CursorShape::HELP,
        C::Crosshair | C::Cell => CursorShape::CROSS,
        _ => CursorShape::ARROW,
    }
}
