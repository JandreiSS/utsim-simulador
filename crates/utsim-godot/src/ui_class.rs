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
use godot::classes::notify::CanvasItemNotification;
use godot::classes::{INode2D, InputEvent, Node2D, RenderingServer, Resource, Viewport};
use godot::prelude::*;

use crate::andamento;
use crate::document_class::UtsimDocument;
use crate::egui_host::Host;
use crate::egui_input::Input;
use crate::jobs_class::{UtsimJob, UtsimPert, UtsimResults, job_object, variant_of_json};
use crate::relogio::{Efeito, Relogio};
use crate::schema::{self, Value};
use crate::ui_bar::BarState;
use crate::ui_bottom::{PlayView, StatusView};
use crate::ui_left::{EditView, Entry, ObjectItem, Summary};
use crate::ui_pert::{self, PertView};
use crate::ui_props::{self, PropAction, PropsView};
use crate::ui_results::{self, ResultsView};
use crate::{put, strings};

use crate::egui_canvas::{clipboard_get, read_input};

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct UtsimUi {
    host: Host,
    start: Instant,
    canvas: crate::egui_canvas::EguiCanvas,
    /// The Godot texture of each egui texture.
    /// Interface scale (pixels per point) over the window's content scale.
    #[var]
    ui_scale: f32,
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
    props_key: Option<(u64, String, Option<String>)>,
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
    /// Why the last run asked of the "Resultados" window did not start.
    results_error: String,
    /// The playback whose token flows the Fluxos window shows.
    flows_pb: Option<Gd<crate::UtsimPlayback>>,
    /// The PERT/CPM shown (kept for the exports).
    pert: Option<Gd<UtsimPert>>,
    /// The playback clock of Simular and the replication it plays (for its events).
    relogio: Relogio,
    clock_pb: Option<Gd<crate::UtsimPlayback>>,
    /// What the last run says about each node (the node tabs), the quality and safety
    /// numbers still coming from a thread, and its stop flag.
    indicadores: Option<crate::indicadores::Indicadores>,
    ind_rx: Option<std::sync::mpsc::Receiver<crate::indicadores::Indicadores>>,
    ind_stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for UtsimUi {
    fn init(base: Base<Node2D>) -> Self {
        UtsimUi {
            host: Host::new(),
            start: Instant::now(),
            canvas: crate::egui_canvas::EguiCanvas::new(),
            ui_scale: 1.0,
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
            results_error: String::new(),
            pert: None,
            flows_pb: None,
            relogio: Relogio::default(),
            clock_pb: None,
            indicadores: None,
            ind_rx: None,
            ind_stop: Default::default(),
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
        self.poll_indicators();
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
        if let Some(v) = get("links_hidden")
            && let Ok(list) = v.try_to::<PackedStringArray>()
        {
            s.links_hidden = list.as_slice().iter().map(|g| g.to_string()).collect();
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
        if let Some(w) = self.host.reg.read(&name.to_string()) {
            put(&mut d, "text", GString::from(w.text.as_str()));
            put(&mut d, "enabled", w.enabled);
            if let Some(on) = w.checked {
                put(&mut d, "checked", on);
            }
            put(&mut d, "choices", strings(&w.choices));
            if let Some(n) = w.count {
                put(&mut d, "count", n as i64);
            }
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
        if let Some(x) = get("seguranca") {
            v.seguranca = x.try_to::<bool>().unwrap_or(v.seguranca);
        }
        if let Some(x) = get("calor") {
            v.calor = x.try_to::<bool>().unwrap_or(v.calor);
        }
        if let Some(x) = get("quase")
            && let Ok(list) = x.try_to::<VarArray>()
        {
            v.quase = list
                .iter_shared()
                .filter_map(|e| e.try_to::<VarArray>().ok())
                .filter_map(|p| Some((p.get(0)?.try_to::<f64>().ok()?, p.get(1)?.to_string())))
                .collect();
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

    /// The playback bar: `{visible}` (shown in Simular); what it shows of the time comes
    /// from the clock.
    #[func]
    fn set_playback_bar(&mut self, state: VarDictionary) {
        if let Some(x) = state.get(&"visible".to_variant()) {
            let mut v: PlayView = self.host.play().clone();
            v.visible = x.try_to::<bool>().unwrap_or(v.visible);
            self.host.set_play(v);
        }
        self.sync_clock();
    }

    /// The replication the clock plays (null: none): paused at 0.
    #[func]
    fn clock_load(&mut self, playback: Option<Gd<crate::UtsimPlayback>>) {
        let end = playback
            .as_ref()
            .and_then(|p| p.bind().core.as_ref().map(crate::playback::Playback::end));
        self.relogio.carregar(end);
        self.clock_pb = playback.filter(|_| end.is_some());
        self.sync_clock();
    }

    /// Plays or pauses (it only plays with a replication, in Simular).
    #[func]
    fn clock_play(&mut self, on: bool) {
        let simulando = self.host.play().visible;
        self.relogio.tocar(on, simulando);
        self.sync_clock();
    }

    /// Advances a frame of `dt` real seconds; true when the time moved (show it).
    #[func]
    fn clock_tick(&mut self, dt: f64) -> bool {
        let moved = self.relogio.avancar(dt);
        if moved {
            self.sync_clock();
        }
        moved
    }

    /// Jumps to `time` (clamped); false without a replication.
    #[func]
    fn clock_seek(&mut self, time: f64) -> bool {
        let ok = self.relogio.ir(time);
        self.sync_clock();
        ok
    }

    /// Pauses and jumps to the next (`direction` > 0) or previous event; true when the
    /// time jumped.
    #[func]
    fn clock_step(&mut self, direction: i64) -> bool {
        let t = self.relogio.t;
        let alvo = self
            .clock_pb
            .as_ref()
            .and_then(|p| {
                let b = p.bind();
                let core = b.core.as_ref()?;
                if direction > 0 {
                    core.next_event(t)
                } else {
                    core.prev_event(t)
                }
            })
            .unwrap_or(-1.0);
        let jumped = self.relogio.ao_evento(alvo);
        self.sync_clock();
        jumped
    }

    #[func]
    fn clock_t(&self) -> f64 {
        self.relogio.t
    }

    #[func]
    fn clock_playing(&self) -> bool {
        self.relogio.playing
    }

    #[func]
    fn clock_speed(&self) -> f64 {
        self.relogio.speed
    }

    #[func]
    fn set_clock_speed(&mut self, speed: f64) {
        self.relogio.velocidade(speed);
        self.sync_clock();
    }

    /// Follows a background job: shows its progress in the status bar and, when it ends,
    /// hides the bar and says how it ended. `{finished, message, call}`: `call` true when
    /// the main scene goes on with the result.
    #[func]
    fn poll_job(&mut self, mut job: Gd<crate::jobs_class::UtsimJob>) -> VarDictionary {
        let p = job.bind_mut().progress();
        let num = |k: &str| p.get(&k.to_variant()).and_then(|v| v.try_to::<i64>().ok());
        let finished = p
            .get(&"finished".to_variant())
            .and_then(|v| v.try_to::<bool>().ok())
            .unwrap_or(false);
        let mut out = VarDictionary::new();
        put(&mut out, "finished", finished);
        let mut st: StatusView = self.host.status().clone();
        if !finished {
            let ms = self.start.elapsed().as_millis() as u64;
            let (progress, percent) =
                andamento::barra(num("done").unwrap_or(0), num("total").unwrap_or(0), ms);
            st.progress = Some(progress as f32);
            st.percent = percent;
            self.host.set_status(st);
            return out;
        }
        st.progress = None;
        let (message, call) = {
            let j = job.bind();
            andamento::fim(
                &j.what().to_string(),
                j.kind_name(),
                j.ok(),
                j.cancelled(),
                &j.error().to_string(),
            )
        };
        self.host.set_status(st);
        put(&mut out, "message", message);
        put(&mut out, "call", call);
        out
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

    /// A new run for the "Resultados" window (`can_run`: false for a recorded replay,
    /// which cannot run again; `export_dir`: the CSV folder proposed when none was typed).
    #[func]
    fn set_results(&mut self, res: Gd<UtsimResults>, can_run: bool, export_dir: GString) {
        let Some(run) = res.bind().run().cloned() else {
            return;
        };
        self.start_indicators(&run);
        self.host
            .set_results(ResultsView::new(run, can_run), &export_dir.to_string());
    }

    /// Forgets the numbers of the last run in the node tabs (another model).
    #[func]
    fn clear_indicators(&mut self) {
        self.ind_stop
            .store(true, std::sync::atomic::Ordering::Relaxed);
        self.indicadores = None;
        self.ind_rx = None;
        self.props_key = None;
        self.host.invalidate();
    }

    /// Waits (at most two minutes) for the quality and safety numbers of the node tabs;
    /// false when they did not come. For the scripted runs.
    #[func]
    fn indicators_wait(&mut self) -> bool {
        if let Some(rx) = &self.ind_rx {
            match rx.recv_timeout(std::time::Duration::from_secs(120)) {
                Ok(ind) => self.indicadores = Some(ind),
                Err(_) => return false,
            }
            self.ind_rx = None;
            self.props_key = None;
            self.host.invalidate();
        }
        true
    }

    /// Opens or closes the "Resultados" window (it opens only with a run to show).
    #[func]
    fn set_results_open(&mut self, on: bool) {
        self.host.set_results_open(on);
    }

    #[func]
    fn results_open(&self) -> bool {
        self.host.results_open
    }

    /// The run of a button of the window (`aquecimento`, `mais`, `little`,
    /// `gargalo_movel`, `cenarios`, `otimizacao`) with what its form holds, or `null` with
    /// the reason in [`Self::results_error`] (shown under the button too).
    #[func]
    fn results_job(&mut self, kind: GString) -> Option<Gd<UtsimJob>> {
        let kind = kind.to_string();
        let v = self.host.results.as_ref()?;
        match ui_results::task(&kind, v, &self.host.results_mem) {
            Ok(task) => {
                self.results_error.clear();
                Some(job_object(task))
            }
            Err(e) => {
                self.host.results_mem.set_error(&kind, &e);
                self.results_error = e;
                self.host.invalidate();
                None
            }
        }
    }

    #[func]
    fn results_error(&self) -> GString {
        GString::from(self.results_error.as_str())
    }

    /// A finished run of the window: its tables and charts; returns the status line.
    #[func]
    fn results_done(&mut self, job: Gd<UtsimJob>) -> GString {
        let j = job.bind();
        let Some(v) = self.host.results.as_ref() else {
            return GString::new();
        };
        let line = ui_results::done(j.kind_name(), j.data_json(), v, &mut self.host.results_mem);
        self.host.invalidate();
        GString::from(line.as_str())
    }

    /// The right panel of a tool or a link, as rows in JSON (see `ui_form.rs`); `""`
    /// shows the properties again.
    #[func]
    fn show_form(&mut self, rows: GString) {
        let rows = rows.to_string();
        self.host
            .set_form((!rows.trim().is_empty()).then(|| crate::ui_form::parse(&rows)));
    }

    /// A modal dialog in JSON (see `ui_form::Dialog`); `""` closes it. Its buttons come
    /// back as `dialogo:<action>`.
    #[func]
    fn show_dialog(&mut self, dialog: GString) {
        self.host
            .set_dialog(crate::ui_form::parse_dialog(&dialog.to_string()));
    }

    /// The eye and the aim of a camera riding a block (`camera_follow.rs`): `[eye, aim]`, or
    /// empty for the cameras that ride nothing. `head`: a driver's eye, or null.
    #[func]
    fn follow_view(
        &self,
        mode: GString,
        at: Vector3,
        axis_x: Vector3,
        top: f64,
        head: Variant,
    ) -> PackedVector3Array {
        let v = |p: Vector3| [p.x as f64, p.y as f64, p.z as f64];
        let h = head.try_to::<Vector3>().ok().map(v);
        match crate::camera_follow::view(&mode.to_string(), v(at), v(axis_x), top, h) {
            Some((e, a)) => [e, a]
                .iter()
                .map(|p| Vector3::new(p[0] as f32, p[1] as f32, p[2] as f32))
                .collect(),
            None => PackedVector3Array::new(),
        }
    }

    /// How camera `mode` follows block `follow` each frame (`camera_follow::conducao`):
    /// `livre`, `alvo` (Seguir) or `olho` (Primeira pessoa, Do recurso).
    #[func]
    fn camera_drive(&self, mode: GString, follow: GString) -> GString {
        use crate::camera_follow::Conducao;
        match crate::camera_follow::conducao(&mode.to_string(), &follow.to_string()) {
            Conducao::Livre => "livre",
            Conducao::Alvo => "alvo",
            Conducao::Olho => "olho",
        }
        .into()
    }

    /// The block a new replication follows (`blocks`: the scene's `{name, mobile}`).
    #[func]
    fn camera_pick(&self, current: GString, blocks: VarArray) -> GString {
        let list: Vec<(String, bool)> = blocks
            .iter_shared()
            .filter_map(|b| b.try_to::<VarDictionary>().ok())
            .map(|b| {
                let name = b.get(&"name".to_variant()).map(|v| v.to_string());
                let mobile = b
                    .get(&"mobile".to_variant())
                    .and_then(|v| v.try_to::<bool>().ok());
                (name.unwrap_or_default(), mobile.unwrap_or(false))
            })
            .collect();
        GString::from(crate::camera_follow::seguido(&current.to_string(), &list).as_str())
    }

    /// Choosing camera `mode`: `{restore, notice}` (give back the blocks hidden on the
    /// sight line; the status message).
    #[func]
    fn camera_choice(&self, mode: GString, follow: GString) -> VarDictionary {
        let (m, f) = (mode.to_string(), follow.to_string());
        let mut d = VarDictionary::new();
        put(&mut d, "restore", crate::camera_follow::restaura_vista(&m));
        put(&mut d, "notice", crate::camera_follow::aviso(&m, &f));
        d
    }

    /// The floor grid of Editar: the lines (pairs of points) around `positions` (the
    /// objects, Godot) for the snap step `snap` (`grid.rs`).
    #[func]
    fn edit_grid(&self, positions: PackedVector3Array, snap: f64) -> PackedVector3Array {
        let p: Vec<[f64; 3]> = positions
            .as_slice()
            .iter()
            .map(|v| [v.x as f64, v.y as f64, v.z as f64])
            .collect();
        let (lo, hi, step) = crate::grid::edit_extent(&p, snap);
        grid_points(crate::grid::lines(lo, hi, step))
    }

    /// The floor grid of Simular, around the scene's bounds.
    #[func]
    fn sim_grid(&self, lo: Vector3, hi: Vector3) -> PackedVector3Array {
        let v = |p: Vector3| [p.x as f64, p.y as f64, p.z as f64];
        let (a, b, step) = crate::grid::sim_extent(v(lo), v(hi));
        grid_points(crate::grid::lines(a, b, step))
    }

    /// The action of a key press in the main scene (`shortcuts.rs`), or `""`.
    #[func]
    fn key_action(
        &self,
        keycode: i64,
        ctrl: bool,
        shift: bool,
        editing: bool,
        tool: GString,
    ) -> GString {
        let ctx = crate::shortcuts::KeyContext {
            editing,
            tool: tool.to_string(),
        };
        crate::shortcuts::key_action(keycode, ctrl, shift, &ctx)
            .unwrap_or_default()
            .as_str()
            .into()
    }

    /// What a click of the interface does (`acoes.rs`): the steps for the main scene to
    /// run, `{op, s, s2, s3, n, n2, x, b, lista}`. `state`: `{tool, place_id, snap, step,
    /// links_hidden}`.
    #[func]
    fn route_action(&self, what: GString, state: VarDictionary) -> VarArray {
        steps_array(crate::acoes::rotear(
            &what.to_string(),
            &route_state(&state),
        ))
    }

    /// What the key action `what` (from `key_action`) does, as [`UtsimUi::route_action`];
    /// empty when the key is not used.
    #[func]
    fn route_key(&self, what: GString, state: VarDictionary) -> VarArray {
        steps_array(crate::acoes::rotear_tecla(
            &what.to_string(),
            &route_state(&state),
        ))
    }

    #[func]
    fn dialog_open(&self) -> bool {
        self.host.dialog.is_some()
    }

    /// The performance line over the 3D view (`""`: hidden).
    #[func]
    fn set_performance(&mut self, text: GString) {
        self.host.set_perf(&text.to_string());
    }

    /// The playback of the Fluxos window: its flows and their counts over the run.
    #[func]
    fn set_flows_playback(&mut self, pb: Gd<crate::UtsimPlayback>) {
        let view = {
            let b = pb.bind();
            let Some(p) = b.core.as_ref() else {
                return;
            };
            let graphs = crate::extras::flow_graphs(p);
            let series = graphs
                .iter()
                .map(|g| crate::extras::flow_series(p, g, crate::ui_flows::SAMPLES))
                .collect();
            crate::ui_flows::FlowsView {
                graphs,
                series,
                counts: Vec::new(),
                t: 0.0,
            }
        };
        self.host.flows_mem = Default::default();
        self.host.flows = Some(view);
        self.flows_pb = Some(pb);
        self.flows_time(0.0);
    }

    /// The playback time: the tokens of the chosen flow at `t` (only while it is open).
    #[func]
    fn flows_time(&mut self, t: f64) {
        if !self.host.flows_open
            && self
                .host
                .flows
                .as_ref()
                .is_some_and(|v| !v.counts.is_empty())
        {
            return;
        }
        let (Some(pb), Some(v)) = (&self.flows_pb, self.host.flows.as_mut()) else {
            return;
        };
        let b = pb.bind();
        let Some(p) = b.core.as_ref() else {
            return;
        };
        let k = self.host.flows_mem.flow;
        v.counts = v
            .graphs
            .get(k)
            .map(|g| crate::extras::flow_counts(p, g, t))
            .unwrap_or_default();
        v.t = t;
        self.host.invalidate();
    }

    #[func]
    fn set_flows_open(&mut self, on: bool) {
        if self.host.flows_open != on {
            self.host.flows_open = on;
            self.host.flows_mem.zoom = None;
            self.host.invalidate();
        }
    }

    #[func]
    fn flows_open(&self) -> bool {
        self.host.flows_open
    }

    /// Values read from a CSV or text file into the "Valores reais…" window. Returns the
    /// error, or "" when they were added.
    #[func]
    fn fitwin_csv(&mut self, file: GString) -> GString {
        let Some(w) = self.host.fitwin.as_mut() else {
            return "a janela de valores não está aberta".into();
        };
        let r = utsim_fit::data::load(
            std::path::Path::new(&file.to_string()),
            &utsim_fit::data::DataOptions::default(),
        );
        let msg = match r {
            Ok(l) => {
                let n = l.values.len();
                w.rows
                    .extend(l.values.iter().map(|x| crate::ui_props::num_text(*x)));
                w.note = format!("{n} valor(es) lidos de {} (coluna {}).", file, l.column);
                String::new()
            }
            Err(e) => {
                w.note = format!("Não leu o arquivo: {e}");
                e
            }
        };
        self.host.invalidate();
        GString::from(msg.as_str())
    }

    /// Writes the report of the "Valores reais…" window into `dir`: `{ok, file, error}`.
    #[func]
    fn fitwin_report(&mut self, dir: GString) -> VarDictionary {
        let mut d = VarDictionary::new();
        let r = match self.host.fitwin.as_ref() {
            Some(w) => w.write_report(std::path::Path::new(&dir.to_string())),
            None => Err("a janela de valores não está aberta".into()),
        };
        match r {
            Ok(f) => {
                d.set("ok", true);
                d.set("file", &GString::from(f.to_string_lossy().as_ref()));
                if let Some(w) = self.host.fitwin.as_mut() {
                    w.note = format!("Relatório gravado em {}.", f.display());
                }
            }
            Err(e) => {
                d.set("ok", false);
                d.set("error", &GString::from(e.as_str()));
            }
        }
        self.host.invalidate();
        d
    }

    /// Waits for the comparison of the "Valores reais…" window (scripted runs). True
    /// when it is open and has a result.
    #[func]
    fn fitwin_wait(&mut self) -> bool {
        let Some(w) = self.host.fitwin.as_mut() else {
            return false;
        };
        w.wait();
        let ok = w.fit.is_some();
        self.host.invalidate();
        ok
    }

    #[func]
    fn fitwin_open(&self) -> bool {
        self.host.fitwin.is_some()
    }

    /// The state of the "Valores reais…" window for the scripted checks: `{aberta, linhas,
    /// comparado, escolhida, ordem, candidatas}`.
    #[func]
    fn fitwin_info(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        d.set("aberta", self.host.fitwin.is_some());
        if let Some(w) = &self.host.fitwin {
            d.set("linhas", w.rows.len() as i64);
            d.set("comparado", w.fit.is_some());
            d.set("escolhida", &GString::from(w.selected_family().as_str()));
            d.set("ordem", &GString::from(w.sort.key()));
            let n = w
                .fit
                .as_ref()
                .map_or(0, |(_, f)| f.report.ranking.candidates.len());
            d.set("candidatas", n as i64);
        }
        d
    }

    /// The file chosen in the dialog for the real-data form of the field `path`.
    #[func]
    fn set_data_file(&mut self, path: GString, file: GString) {
        self.host
            .props_state
            .set_data_file(&path.to_string(), &file.to_string());
        self.host.invalidate();
    }

    /// Opens the PERT/CPM window for the selection called `title` (empty until
    /// [`Self::set_pert`]); the Resultados window closes.
    #[func]
    fn open_pert(&mut self, title: GString) {
        self.pert = None;
        self.host.open_pert(&title.to_string());
    }

    #[func]
    fn close_pert(&mut self) {
        self.host.close_pert();
    }

    #[func]
    fn pert_open(&self) -> bool {
        self.host.pert_open
    }

    /// `{mode, deadline, samples}` of the PERT/CPM window (what to calculate next).
    #[func]
    fn pert_form(&self) -> VarDictionary {
        let m = &self.host.pert_mem;
        let mut d = VarDictionary::new();
        put(&mut d, "mode", GString::from(m.mode.as_str()));
        put(&mut d, "deadline", m.deadline_value());
        put(&mut d, "samples", m.samples_value());
        d
    }

    /// The line over the tabs: "Calculando..." or an error.
    #[func]
    fn set_pert_info(&mut self, info: GString, is_error: bool) {
        self.host.set_pert_info(&info.to_string(), is_error);
    }

    /// A computed PERT/CPM (`default_file`: the HTML report proposed when none was typed).
    #[func]
    fn set_pert(&mut self, p: Gd<UtsimPert>, default_file: GString) {
        let view = {
            let b = p.bind();
            let Some(out) = b.output() else {
                return;
            };
            PertView {
                data: out.to_json(),
                text: out.text(),
            }
        };
        self.host.set_pert(view, &default_file.to_string());
        self.pert = Some(p);
    }

    /// What the window shows (`UtsimPert.data()`), or `null`: the scripted checks.
    #[func]
    fn pert_data(&self) -> Variant {
        self.host
            .pert
            .as_ref()
            .map_or_else(Variant::nil, |v| variant_of_json(&v.data))
    }

    /// The objects on the critical path of the result shown.
    #[func]
    fn pert_critical(&self) -> PackedStringArray {
        strings(
            &self
                .host
                .pert
                .as_ref()
                .map(|v| ui_pert::critical_objects(&v.data))
                .unwrap_or_default(),
        )
    }

    /// The status line of the result shown.
    #[func]
    fn pert_status(&self) -> GString {
        let line = self
            .host
            .pert
            .as_ref()
            .map(|v| ui_pert::status_line(&v.data))
            .unwrap_or_default();
        GString::from(line.as_str())
    }

    /// Writes the HTML report (`html`) or the CSV tables next to it; the status line.
    #[func]
    fn pert_export(&mut self, html: bool) -> GString {
        let path = self.host.pert_mem.file.trim().to_owned();
        let Some(p) = &self.pert else {
            return GString::new();
        };
        let b = p.bind();
        let Some(out) = b.output() else {
            return GString::new();
        };
        let line = if path.is_empty() {
            "Escreva o arquivo do relatório.".to_owned()
        } else if html {
            match std::fs::write(&path, out.html()) {
                Ok(()) => format!("Relatório PERT/CPM gravado em {path}."),
                Err(e) => format!("Não gravou {path}: {e}"),
            }
        } else {
            let dir = std::path::Path::new(&path)
                .parent()
                .map(std::path::Path::to_path_buf)
                .unwrap_or_default();
            match out.write_csv(&dir) {
                Ok(files) => format!("{} tabela(s) CSV em {}.", files.len(), dir.display()),
                Err(e) => format!("Não gravou os CSV em {}: {e}", dir.display()),
            }
        };
        GString::from(line.as_str())
    }

    /// Writes the CSV files of the run into the folder typed; returns the status line.
    #[func]
    fn results_export(&mut self) -> GString {
        let dir = self.host.results_mem.export_dir.trim().to_owned();
        let line = match &self.host.results {
            None => String::new(),
            Some(_) if dir.is_empty() => "Escreva a pasta da exportação.".into(),
            Some(v) => {
                match utsim_model::csv::export(&std::path::PathBuf::from(&dir), &v.run.reports) {
                    Ok(files) => format!("{} arquivo(s) CSV em {dir}.", files.len()),
                    Err(e) => format!("Não exportou: {e}"),
                }
            }
        };
        GString::from(line.as_str())
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
        let parent = self.base().get_canvas_item();
        let (mini, user) = (self.miniature, &self.user);
        self.canvas
            .present(&self.host.ctx, frame.output, parent, size, |t| {
                if t == ui_props::MINIATURE {
                    mini
                } else {
                    user.get(&t).copied()
                }
            });
        for a in frame.actions {
            if let Some(rest) = a.strip_prefix("valores:") {
                self.fitwin_action(rest);
                continue;
            }
            if self.clock_bar_action(&a) {
                continue;
            }
            self.emit(&a);
        }
        for a in frame.props {
            self.prop_action(a);
        }
    }

    /// Opens "Valores reais…" for the field `path` of the object shown, with the values of
    /// the draft (typed, or read from its file).
    fn open_fitwin(&mut self, path: &str, draft: &serde_json::Value) {
        let text = draft["text"].as_str().unwrap_or_default();
        let source = draft["source"]
            .as_str()
            .unwrap_or_default()
            .trim()
            .to_owned();
        let mut rows: Vec<String> = Vec::new();
        if draft["origem"] == "arquivo" && !source.is_empty() {
            let p = std::path::PathBuf::from(&source);
            let base = self.host.props.as_ref().and_then(|v| v.base_dir.clone());
            let p = match base {
                Some(b) if p.is_relative() => b.join(p),
                _ => p,
            };
            let column = draft["column"].as_str().unwrap_or_default();
            let options = utsim_fit::data::DataOptions {
                column: (!column.is_empty()).then(|| column.to_owned()),
                timestamps: false,
            };
            if let Ok(l) = utsim_fit::data::load(&p, &options) {
                rows = l
                    .values
                    .iter()
                    .map(|x| crate::ui_props::num_text(*x))
                    .collect();
            }
        } else {
            let mut w = crate::ui_fitwin::FitWin::new("", "", Vec::new(), false);
            w.add_text(text);
            rows = w.rows;
        }
        let timestamps = draft["timestamps"].as_bool().unwrap_or(false);
        self.host.fitwin = Some(crate::ui_fitwin::FitWin::new(
            &self.object,
            path,
            rows,
            timestamps,
        ));
        self.host.invalidate();
    }

    /// An action of the "Valores reais…" window (after `valores:`).
    fn fitwin_action(&mut self, a: &str) {
        let (what, arg) = a.split_once('\t').unwrap_or((a, ""));
        let window = self.host.fitwin.as_ref().or(self.host.fitwin_done.as_ref());
        let Some(w) = window else {
            return;
        };
        let path = w.path.clone();
        match what {
            "usar" | "empirica" => {
                let choose = if what == "usar" { arg } else { "Empirical" };
                let draft = w.draft(choose);
                self.host.fitwin_done = None;
                self.emit(&format!("dados\taplicar\t{path}\t{draft}"));
            }
            "csv" => self.emit(&format!("dados\tjanela_csv\t{path}")),
            "relatorio" => self.emit(&format!("dados\tjanela_relatorio\t{path}")),
            _ => {}
        }
    }

    /// The playback bar's own actions (play, speed, time, events) go to the clock; the
    /// main scene is told only when the time jumped (`relogio`). True when handled.
    fn clock_bar_action(&mut self, what: &str) -> bool {
        if let Some(dir) = what.strip_prefix("evento:") {
            if self.clock_step(dir.parse::<i64>().unwrap_or(1)) {
                self.emit("relogio");
            }
            return true;
        }
        let simulando = self.host.play().visible;
        match self.relogio.acao(what, simulando) {
            Efeito::Nenhum => return false,
            Efeito::Estado => {}
            Efeito::Salto => self.emit("relogio"),
        }
        self.sync_clock();
        true
    }

    /// What the playback bar shows of the clock.
    fn sync_clock(&mut self) {
        let v: PlayView = self.host.play().clone();
        let r = &self.relogio;
        let new = PlayView {
            playing: r.playing,
            t: r.t,
            end: r.end.unwrap_or(0.0),
            speed: r.speed,
            ..v.clone()
        };
        if new != v {
            self.host.set_play(new);
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
            // The presets of the specialised lines, with the icon of their core kind.
            self.edit_state
                .library
                .extend(crate::linha::entries().into_iter().map(|e| Entry {
                    icon: self.icons.get(e.base_kind).copied(),
                    id: e.id,
                    label: e.label,
                    category: e.category,
                }));
            self.edit_state.categories = utsim_edit::CATEGORIES
                .iter()
                .map(|c| c.to_string())
                .chain(crate::linha::categories())
                .collect();
            self.edit_state.objects = match &self.doc {
                Some(d) => d
                    .bind()
                    .editor
                    .objects()
                    .into_iter()
                    .map(|o| ObjectItem {
                        // A model of a line shows the line's words for the kinds.
                        kind_label: crate::linha::kind_label(d.bind().editor.model(), o.kind)
                            .unwrap_or_else(|| o.kind_label.to_owned()),
                        name: o.name,
                        problems: o.problems,
                        warnings: o.warnings,
                    })
                    .collect(),
                None => Vec::new(),
            };
        }
        self.host.set_edit(self.edit_state.clone());
    }

    /// The numbers of the node tabs for a new run: Manutenção now, from the reports;
    /// Qualidade and Segurança from one more run per replication in a thread (with the
    /// per-item log and the trace), which a newer run stops.
    fn start_indicators(&mut self, run: &crate::results::RunOutput) {
        use crate::indicadores::{Indicadores, replica, tem_qualidade, tem_seguranca};
        use std::sync::atomic::{AtomicBool, Ordering};
        self.ind_stop.store(true, Ordering::Relaxed);
        let stop = std::sync::Arc::new(AtomicBool::new(false));
        self.ind_stop = stop.clone();
        let precisa = tem_qualidade(&run.model) || tem_seguranca(&run.model);
        let mut agora = Indicadores::montar(&run.model, &run.reports, &[]);
        agora.calculando = precisa;
        self.indicadores = Some(agora);
        self.ind_rx = None;
        self.props_key = None;
        self.host.invalidate();
        if !precisa {
            return;
        }
        let (tx, rx) = std::sync::mpsc::channel();
        self.ind_rx = Some(rx);
        let model = run.model.clone();
        let reports = run.reports.clone();
        std::thread::spawn(move || {
            let mut reps = Vec::new();
            for r in &reports {
                if stop.load(Ordering::Relaxed) {
                    return;
                }
                // A replication that fails here failed in the run too: left out.
                if let Ok(x) = replica(&model, r.replication) {
                    reps.push(x);
                }
            }
            if !stop.load(Ordering::Relaxed) {
                let _ = tx.send(Indicadores::montar(&model, &reports, &reps));
            }
        });
    }

    /// Takes the quality and safety numbers when the thread is done.
    fn poll_indicators(&mut self) {
        let Some(rx) = &self.ind_rx else {
            return;
        };
        match rx.try_recv() {
            Ok(ind) => {
                self.indicadores = Some(ind);
                self.ind_rx = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => return,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.ind_rx = None;
                if let Some(i) = self.indicadores.as_mut() {
                    i.calculando = false;
                }
            }
        }
        self.props_key = None;
        self.host.invalidate();
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
        let notice = d.trava.aviso();
        let key = (d.editor.doc.revision(), self.object.clone(), notice.clone());
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
        let data = d
            .data_list()
            .into_iter()
            .filter(|e| e["source"] == "dados")
            .filter_map(|e| {
                let k = e["field"].as_str()?.strip_prefix(&prefix)?.to_owned();
                Some((k, e))
            })
            .collect();
        let base_dir = d
            .editor
            .doc
            .path()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf());
        self.host.props = Some(PropsView {
            props,
            families: schema::FAMILIES.iter().map(|s| s.to_string()).collect(),
            data,
            data_families: utsim_fit::auto::family_names()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            base_dir,
            data_paths,
            extra: self.extra.clone(),
            driver: self.driver,
            miniature: self.miniature.is_some(),
            abas: if self.object == "run" {
                Vec::new()
            } else {
                d.editor.tab_fields(&self.object)
            },
            simulado: None,
            somente_leitura: notice,
        });
        if let Some(pv) = self.host.props.as_mut() {
            pv.simulado = simulated_view(self.indicadores.as_ref(), &self.object, &pv.abas);
        }
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
            PropAction::Data { what, path, draft } if what == "janela" => {
                self.open_fitwin(&path, &draft);
            }
            PropAction::Data { what, path, draft } => {
                self.emit(&format!("dados\t{what}\t{path}\t{draft}"))
            }
            PropAction::ArrivalsPreview => {
                if let Some(doc) = &self.doc {
                    let d = doc.bind();
                    let model = d.editor.model();
                    let key = crate::ui_arrivals::preview_key(model, &self.object);
                    let r = crate::ui_arrivals::run_preview(model, &self.object);
                    drop(d);
                    self.host.props_state.arrivals.set_run(key, r);
                    self.host.invalidate();
                }
            }
            PropAction::Miniature(asset) => self.emit(&format!("miniatura\t{asset}")),
            PropAction::Aba(acao) => self.tab_action(acao),
        }
    }

    /// An edit of the Qualidade, Segurança or Manutenção tab, through `utsim_edit::abas`
    /// (one undo step); a refusal stays under the field.
    fn tab_action(&mut self, acao: crate::ui_abas::AbaAcao) {
        use crate::ui_abas::AbaAcao;
        if let AbaAcao::AbrirApp(app) = &acao {
            // Saving first and handing the model over are the main scene's.
            self.emit(&format!("abrir_app:{app}:{}", self.object));
            return;
        }
        let Some(mut doc) = self.doc.clone() else {
            return;
        };
        let object = self.object.clone();
        let (caminho, shown, r) = {
            let mut d = doc.bind_mut();
            if let Some(quem) = d.trava.aviso() {
                (
                    String::new(),
                    String::new(),
                    Err(format!("{quem}: só leitura")),
                )
            } else {
                match acao {
                    AbaAcao::Editar { caminho, valor } => {
                        let r = d.editor.tab_edit(&object, &caminho, &valor);
                        (caminho, valor, r)
                    }
                    AbaAcao::Adicionar(lista) => {
                        let r = d.editor.tab_add(&object, &lista);
                        (lista, "adicionar".to_owned(), r)
                    }
                    AbaAcao::Remover(lista, i) => {
                        let r = d.editor.tab_remove(&object, &lista, i);
                        (format!("{lista}[{i}]"), "remover".to_owned(), r)
                    }
                    AbaAcao::AbrirApp(_) => return,
                }
            }
        };
        self.props_key = None;
        self.host.invalidate();
        match r {
            Ok(()) => {
                self.host.props_state.abas.erro = None;
                self.emit(&format!("prop_ok\t{object}\t{caminho}\t{shown}"));
            }
            Err(e) => {
                self.host.props_state.abas.erro = Some((caminho.clone(), e.clone()));
                self.emit(&format!("prop_erro\t{caminho}\t{e}"));
            }
        }
    }

    fn free_all(&mut self) {
        self.canvas.free_all();
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

fn grid_points(l: Vec<[f64; 3]>) -> PackedVector3Array {
    l.iter()
        .map(|p| Vector3::new(p[0] as f32, p[1] as f32, p[2] as f32))
        .collect()
}

/// The main scene's state the routing decisions need (`UtsimUi::route_action`).
fn route_state(d: &VarDictionary) -> crate::acoes::Estado {
    let text = |k: &str| {
        d.get(&k.to_variant())
            .map(|v| v.to_string())
            .unwrap_or_default()
    };
    crate::acoes::Estado {
        ferramenta: text("tool"),
        colocar: text("place_id"),
        encaixe: d
            .get(&"snap".to_variant())
            .and_then(|v| v.try_to::<bool>().ok())
            .unwrap_or(false),
        passo: d
            .get(&"step".to_variant())
            .and_then(|v| v.try_to::<f64>().ok())
            .unwrap_or(1.0),
        ocultas: d
            .get(&"links_hidden".to_variant())
            .and_then(|v| v.try_to::<PackedStringArray>().ok())
            .map(|l| l.as_slice().iter().map(|s| s.to_string()).collect())
            .unwrap_or_default(),
    }
}

/// The steps as dictionaries for the main scene.
fn steps_array(steps: Vec<crate::acoes::Passo>) -> VarArray {
    let mut out = VarArray::new();
    for p in steps {
        let (op, a) = p.campos();
        let mut d = VarDictionary::new();
        put(&mut d, "op", op);
        put(&mut d, "s", a.s);
        put(&mut d, "s2", a.s2);
        put(&mut d, "s3", a.s3);
        put(&mut d, "n", a.n);
        put(&mut d, "n2", a.n2);
        put(&mut d, "x", a.x);
        put(&mut d, "b", a.b);
        put(&mut d, "lista", strings(&a.lista));
        out.push(&d.to_variant());
    }
    out
}

/// The lines of the last run for `object`, in the order of its tabs.
fn simulated_view(
    ind: Option<&crate::indicadores::Indicadores>,
    object: &str,
    abas: &[(utsim_edit::abas::Aba, Vec<utsim_edit::abas::Campo>)],
) -> Option<crate::ui_abas::Simulado> {
    let ind = ind?;
    if abas.is_empty() {
        return None;
    }
    Some(crate::ui_abas::Simulado {
        nota: format!(
            "Na última simulação ({} {}), média ± semiamplitude do IC de 95 %:",
            ind.replicacoes,
            if ind.replicacoes == 1 {
                "replicação"
            } else {
                "replicações"
            }
        ),
        linhas: abas
            .iter()
            .map(|(a, _)| ind.de(object, *a).to_vec())
            .collect(),
        calculando: ind.calculando,
    })
}
