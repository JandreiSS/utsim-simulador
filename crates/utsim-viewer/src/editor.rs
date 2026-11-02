//! The model editor (`Editar` mode) and the switch to the playback (`Simular`).
//!
//! The Bevy systems here are thin: they turn mouse and keyboard into calls to the
//! pure logic in `utsim_viewer::editing` and into [`Command`]s applied through the
//! [`Document`] (so everything is undoable and validated).

use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, TryRecvError};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContexts, egui};
use utsim_edit::arrange::{self, Align, Axis, Clipboard};
use utsim_edit::{CATEGORIES, Command, Document, Issue, PALETTE, from_palette, palette_entry};
use utsim_model::replay::Replay;
use utsim_viewer::editing::{
    EditView, LINE_PICK_PIXELS, Layer, NetworkView, SHORTCUTS, auto_layout_command, coalesce_key,
    connect_blocked, default_stop, fluid_edit_view, ground_to_model_snapped, group_issues,
    inside_box, logical_flags, name_base, nearest_segment, needs_layout, palette_icon,
    paste_offset, renamed, run_model, select_names, selection_of, toggle_selected,
};
use utsim_viewer::flows_ui::{self, FlowsState};
use utsim_viewer::fluids::FluidView;
use utsim_viewer::layout;
use utsim_viewer::picking::{self, Cycle};
use utsim_viewer::properties::{properties_panel, properties_panel_shared, run_panel};
use utsim_viewer::style;
use utsim_viewer::timeline::TransitSetting;

use crate::camera::CameraLock;
use crate::dock::{Dock, pointer_over_ui};
use crate::layers::{self, LayerPress, NetSel};
use crate::playback::{Data, Playback, Selection};
use crate::scene;
use crate::ui::{Label, capacity_labels, draw_labels};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Edit,
    Sim,
}

/// What a left click does in `Editar`.
#[derive(Clone, PartialEq, Debug)]
pub enum Tool {
    Select,
    /// Place a new object of this palette entry (`utsim_edit::PALETTE`, by id).
    Place(&'static str),
    /// Connect this object to the next block clicked.
    Connect(String),
    /// `Rede`: a click on the floor adds a node.
    Node,
    /// `Rede`: clicking node after node draws edges (from the pending node, if any; a
    /// click on the floor adds a node there and connects it).
    Edge(Option<String>),
    /// `Fluidos`: a click on the floor adds a fluid node of this kind
    /// (`utsim_edit::fluids::FLUID_PALETTE`, by id).
    FluidPlace(&'static str),
    /// `Fluidos`: clicking node after node links them (from the pending node, if any).
    FluidLink(Option<String>),
}

impl Tool {
    /// Text of the tool in the script assertions (`Ferramenta("...")`).
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))] // scripts are native-only
    pub fn script_name(&self) -> String {
        match self {
            Tool::Select => "Selecionar".to_string(),
            Tool::Place(k) => format!("Colocar:{k}"),
            Tool::Connect(f) => format!("Ligar:{f}"),
            Tool::Node => "Rede:No".to_string(),
            Tool::Edge(None) => "Rede:Aresta".to_string(),
            Tool::Edge(Some(n)) => format!("Rede:Aresta:{n}"),
            Tool::FluidPlace(k) => format!("Fluido:{k}"),
            Tool::FluidLink(None) => "Fluido:Ligar".to_string(),
            Tool::FluidLink(Some(n)) => format!("Fluido:Ligar:{n}"),
        }
    }
}

/// Tab of the left panel in `Editar` (objects layer).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LeftTab {
    /// The palette of objects to place.
    #[default]
    Library,
    /// The list of the objects already in the model.
    Objects,
}

/// A left-button gesture in progress.
struct Press {
    start: Vec2,
    /// Block under the cursor when the button went down.
    block: Option<usize>,
    /// Ground offset from the cursor to the block, so the block does not jump.
    grab: Vec2,
    moved: bool,
    /// Objects dragged with the block and where they were when the press began.
    starts: Vec<(String, [f64; 3])>,
    /// Shift (or Ctrl) was held: a press on the floor draws a selection box, a click
    /// on a block toggles it.
    shift: bool,
    /// The press already added its block to the selection (so the click does not undo it).
    toggled: bool,
    /// The press draws a selection box.
    band: bool,
}

type RunResult = Result<Replay, String>;

#[derive(Resource)]
pub struct Editor {
    pub mode: Mode,
    /// False when a recorded `.json` replay was opened: Simular only, nothing to edit.
    pub editable: bool,
    pub doc: Document,
    /// Revision of the document the view, issues and warnings were built from (they are
    /// rebuilt only when it changes: validation is not free on a model of hundreds of
    /// objects).
    refreshed: Option<u64>,
    /// File to create on the first save (a path given on the command line that does not exist yet).
    pub pending_path: Option<PathBuf>,
    pub view: EditView,
    pub issues: Vec<Issue>,
    /// Non-blocking warnings (`Document::warnings`), shown apart from the issues.
    pub warnings: Vec<Issue>,
    /// What `Editar` edits: objects, the path network or the fluid network.
    pub layer: Layer,
    /// Selected node or edge of the path network (`Rede`).
    pub net_sel: Option<NetSel>,
    /// Selected node of the fluid network (`Fluidos`).
    pub fluid_sel: Option<String>,
    /// New edges are two-way.
    pub new_two_way: bool,
    /// The fluid network of the document as drawn in `Editar`.
    pub fluid_view: FluidView,
    /// A left-button gesture of the `Rede` or `Fluidos` layer.
    pub layer_press: Option<LayerPress>,
    /// The primary selected object (its properties are shown).
    pub selected: Option<String>,
    /// The other selected objects (never the primary one).
    pub multi: Vec<String>,
    pub tool: Tool,
    /// Snap placed and dragged objects to the grid ("Encaixar na grade").
    pub snap: bool,
    /// Grid step, metres.
    pub grid_step: f64,
    clipboard: Clipboard,
    /// Pastes of the current clipboard so far (each one lands further away).
    pastes: u32,
    /// Palette entry being dragged towards the floor.
    pub palette_drag: Option<&'static str>,
    /// Selection box being drawn (window points: start, current).
    band: Option<(Vec2, Vec2)>,
    /// The "Atalhos" window is open.
    pub shortcuts_open: bool,
    /// Tab shown in the left panel.
    pub left_tab: LeftTab,
    /// Name filter of the object list.
    pub list_filter: String,
    /// Kind filter of the object list (Portuguese kind label).
    pub list_kind: Option<&'static str>,
    /// Object to bring into view (double-click in the list), consumed by the camera.
    pub focus_request: Option<String>,
    /// Selection cycle of repeated clicks over overlapping objects.
    cycle: Option<Cycle>,
    /// Objects under the cursor (nearest first) while it rests over the 3D view.
    pub stack_hover: Vec<String>,
    /// Object whose properties the right panel showed last (to rewind its scroll).
    props_for: Option<String>,
    press: Option<Press>,
    /// Snapped ground position under the cursor while placing.
    pub hover: Option<[f64; 3]>,
    pub status: String,
    /// Set to ask the camera to frame the whole model (consumed by `camera::frame_scene`).
    pub frame_request: bool,
    /// Text of the `stop_time` override used by quick simulation runs.
    pub stop_text: String,
    save_prompt: Option<String>,
    confirm_new: bool,
    show_blocked: bool,
    job: Option<Mutex<Receiver<RunResult>>>,
    transit: TransitSetting,
    /// The "Fluxos" panel (token flows): open flag and selection.
    pub flows: FlowsState,
    /// The "Resultados" window: analysis runs, scenarios and optimisation.
    pub results: crate::results_panel::Results,
}

enum Act {
    Undo,
    Redo,
    Save,
    SaveAs,
    New,
    Frame,
    Simulate,
    Edit,
    Delete,
    Copy,
    Cut,
    Paste,
    Duplicate,
    SelectAll,
    Align(Align),
    Distribute(Axis),
    Exec(Command),
}

impl Editor {
    /// An editor over `doc`. When every object sits at the origin the automatic
    /// layout is applied once, as one undoable step.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))] // the browser build is read-only
    pub fn new(doc: Document, pending_path: Option<PathBuf>, stop: Option<f64>) -> Self {
        let stop = stop.unwrap_or_else(|| default_stop(doc.model().run.stop_time));
        let mut ed = Editor::blank(doc, pending_path, true);
        ed.stop_text = stop.to_string();
        if let Some(layout) = auto_layout_command(ed.doc.model()) {
            let _ = ed.doc.apply(layout);
            ed.status = "Layout automático aplicado: salve para manter as posições".into();
        } else if needs_layout(ed.doc.model()) && ed.doc.model().network.is_some() {
            ed.status = "Modelo com rede de caminhos: posicione os objetos nos nós da rede \
                         (o layout automático não foi aplicado)"
                .into();
        }
        ed.refresh();
        ed
    }

    /// A viewer for a recorded replay: no editing.
    pub fn read_only() -> Self {
        Editor::blank(Document::new(""), None, false)
    }

    fn blank(doc: Document, pending_path: Option<PathBuf>, editable: bool) -> Self {
        let view = EditView::of(doc.model());
        Editor {
            mode: if editable { Mode::Edit } else { Mode::Sim },
            editable,
            stop_text: default_stop(doc.model().run.stop_time).to_string(),
            doc,
            refreshed: None,
            pending_path,
            view,
            issues: Vec::new(),
            warnings: Vec::new(),
            layer: Layer::Objects,
            net_sel: None,
            fluid_sel: None,
            new_two_way: true,
            fluid_view: FluidView::default(),
            layer_press: None,
            selected: None,
            multi: Vec::new(),
            tool: Tool::Select,
            snap: true,
            grid_step: utsim_viewer::editing::GRID,
            clipboard: Clipboard::default(),
            pastes: 0,
            palette_drag: None,
            band: None,
            shortcuts_open: false,
            left_tab: LeftTab::Library,
            list_filter: String::new(),
            list_kind: None,
            focus_request: None,
            cycle: None,
            stack_hover: Vec::new(),
            props_for: None,
            press: None,
            hover: None,
            status: String::new(),
            frame_request: true,
            save_prompt: None,
            confirm_new: false,
            show_blocked: false,
            job: None,
            transit: TransitSetting::Auto,
            flows: FlowsState::default(),
            results: Default::default(),
        }
    }

    pub fn with_transit(mut self, transit: TransitSetting) -> Self {
        self.transit = transit;
        self
    }

    /// Recomputes what depends on the document (view, issues, stale selection).
    pub fn refresh(&mut self) {
        let revision = self.doc.revision();
        if self.refreshed != Some(revision) {
            self.refreshed = Some(revision);
            self.view = EditView::of(self.doc.model());
            self.issues = self.doc.issues();
            self.warnings = self.doc.warnings();
            self.fluid_view = fluid_edit_view(self.doc.model(), &self.view);
        }
        layers::prune(self);
        if self
            .selected
            .as_ref()
            .is_some_and(|s| self.view.index_of(s).is_none())
        {
            self.selected = None;
        }
        let view = &self.view;
        self.multi.retain(|n| view.index_of(n).is_some());
        if self.selected.is_none() && !self.multi.is_empty() {
            self.selected = Some(self.multi.remove(0));
        }
        if let Some(p) = &self.selected {
            self.multi.retain(|n| n != p);
        }
        if let Tool::Connect(from) = &self.tool
            && self.view.index_of(from).is_none()
        {
            self.tool = Tool::Select;
        }
    }

    /// File name shown in the window title.
    pub fn file_name(&self) -> String {
        self.doc
            .path()
            .map(PathBuf::from)
            .or_else(|| self.pending_path.clone())
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "sem título.ron".to_string())
    }

    pub fn exec(&mut self, command: Command) {
        let key = coalesce_key(&command);
        match self.doc.apply_coalescing(command, key.as_deref()) {
            Ok(()) => self.status.clear(),
            Err(e) => self.status = e,
        }
        self.refresh();
    }

    /// Starts a connection from `name`, unless it is a mobile resource (no ports):
    /// then explains, in Portuguese, which fields to use instead.
    fn start_connect(&mut self, name: String) {
        let kind = self.view.index_of(&name).map(|i| self.view.kinds[i]);
        if let Some(msg) = kind.and_then(connect_blocked) {
            self.status = msg.into();
            self.tool = Tool::Select;
        } else {
            self.tool = Tool::Connect(name);
        }
    }

    /// Every selected object, the primary one first.
    pub fn selection(&self) -> Vec<String> {
        selection_of(&self.selected, &self.multi)
    }

    /// Selects only `name` (or nothing).
    pub fn select_only(&mut self, name: Option<String>) {
        self.selected = name;
        self.multi.clear();
    }

    /// Grid step used for snapping, when snapping is on.
    pub fn snap_step(&self) -> Option<f64> {
        (self.snap && self.grid_step > 0.0).then_some(self.grid_step)
    }

    /// Applies a multi-object command, if there is one.
    fn exec_some(&mut self, command: Option<Command>) {
        if let Some(c) = command {
            self.exec(c);
        }
    }

    fn delete_selected(&mut self) {
        if self.layer != Layer::Objects {
            layers::delete(self);
            return;
        }
        let names = self.selection();
        if let Some(c) = arrange::delete_command(&names) {
            self.select_only(None);
            self.exec(c);
            if names.len() > 1 && self.status.is_empty() {
                self.status = format!("{} objetos excluídos (Ctrl+Z desfaz)", names.len());
            }
            self.tool = Tool::Select;
        }
    }

    fn copy(&mut self) -> bool {
        let names = self.selection();
        if names.is_empty() {
            self.status = "Nada selecionado para copiar".into();
            return false;
        }
        self.clipboard = Clipboard::copy(self.doc.model(), &names);
        self.pastes = 0;
        self.status = format!(
            "{} objeto(s) e {} ligação(ões) copiados",
            self.clipboard.objects.len(),
            self.clipboard.connections.len()
        );
        true
    }

    fn cut(&mut self) {
        if self.copy() {
            let n = self.clipboard.objects.len();
            self.delete_selected();
            self.status = format!("{n} objeto(s) recortados (Ctrl+V cola)");
        }
    }

    /// Pastes `clip` moved by `offset` and selects the copies.
    fn paste_clip(&mut self, clip: &Clipboard, offset: [f64; 3]) {
        let Some((command, names)) = clip.paste(self.doc.model(), offset) else {
            self.status = "A área de transferência está vazia (Ctrl+C copia)".into();
            return;
        };
        self.exec(command);
        if self.status.is_empty() {
            self.status = format!("{} objeto(s) colados: {}", names.len(), names.join(", "));
            let (mut p, mut o) = (None, Vec::new());
            select_names(&mut p, &mut o, names, false);
            self.selected = p;
            self.multi = o;
        }
        self.tool = Tool::Select;
    }

    fn paste(&mut self) {
        let clip = self.clipboard.clone();
        // After a cut the first paste goes back where the objects were.
        let gone = clip
            .objects
            .iter()
            .all(|o| self.view.index_of(&o.name).is_none());
        let offset = if gone && self.pastes == 0 {
            [0.0; 3]
        } else {
            paste_offset(self.pastes + 1)
        };
        self.pastes += 1;
        self.paste_clip(&clip, offset);
    }

    fn duplicate(&mut self) {
        let names = self.selection();
        if names.is_empty() {
            self.status = "Nada selecionado para duplicar".into();
            return;
        }
        let clip = Clipboard::copy(self.doc.model(), &names);
        self.paste_clip(&clip, paste_offset(1));
    }

    fn select_all(&mut self) {
        let names = self.view.names.clone();
        select_names(&mut self.selected, &mut self.multi, names, false);
    }

    /// Moves the selection by `delta` metres (arrow keys).
    fn nudge(&mut self, delta: [f64; 2]) {
        let names = self.selection();
        let command = arrange::move_command(self.doc.model(), &names, delta);
        self.exec_some(command);
    }

    /// Places a new object of palette entry `id` at `at` (model ground position).
    fn place(&mut self, id: &'static str, at: [f64; 3], keep_tool: bool) {
        let name = self.doc.unique_name(name_base(id));
        if let Some(object) = from_palette(id, &name, at) {
            self.exec(Command::Add(Box::new(object)));
            self.select_only(Some(name));
            if !keep_tool {
                self.tool = Tool::Select;
            }
        }
    }

    fn save(&mut self) {
        let target = if self.doc.path().is_some() {
            None
        } else if let Some(p) = self.pending_path.clone() {
            Some(p)
        } else {
            self.save_prompt = Some("modelo.ron".to_string());
            return;
        };
        let result = match target {
            None => self.doc.save(),
            Some(p) => self.doc.save_as(p),
        };
        self.status = match result {
            Ok(()) => format!("Salvo em {}", self.file_name()),
            Err(e) => format!("Erro ao salvar: {e}"),
        };
    }

    /// Opens the "Salvar como" dialog, proposing the current path.
    fn save_as_prompt(&mut self) {
        let current = self
            .doc
            .path()
            .map(PathBuf::from)
            .or_else(|| self.pending_path.clone());
        self.save_prompt = Some(current.map_or_else(
            || "modelo.ron".to_string(),
            |p| p.to_string_lossy().to_string(),
        ));
    }

    fn save_as_text(&mut self, text: &str) {
        let mut path = PathBuf::from(text.trim());
        if path.as_os_str().is_empty() {
            return;
        }
        if path.extension().is_none() {
            path.set_extension("ron");
        }
        self.status = match self.doc.save_as(&path) {
            Ok(()) => format!("Salvo em {}", path.display()),
            Err(e) => format!("Erro ao salvar: {e}"),
        };
        self.save_prompt = None;
    }

    fn new_document(&mut self) {
        self.doc = Document::new("novo");
        self.refreshed = None;
        self.pending_path = None;
        self.select_only(None);
        self.net_sel = None;
        self.fluid_sel = None;
        self.tool = Tool::Select;
        self.confirm_new = false;
        self.stop_text = default_stop(self.doc.model().run.stop_time).to_string();
        self.status.clear();
        self.frame_request = true;
        self.refresh();
    }

    fn request_new(&mut self) {
        if self.doc.is_dirty() {
            self.confirm_new = true;
        } else {
            self.new_document();
        }
    }

    /// Starts a replication on another thread, unless the model has issues.
    fn simulate(&mut self) {
        if self.job.is_some() {
            return;
        }
        if !self.issues.is_empty() {
            self.show_blocked = true;
            return;
        }
        let stop = match self.stop_text.trim().replace(',', ".").parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                self.status = "Tempo máximo inválido: use um número maior que zero".into();
                return;
            }
        };
        let model = run_model(self.doc.model(), stop);
        let (tx, rx) = mpsc::channel();
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || {
            let result = model.replay(0).map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
        // The browser has no threads here: run it in place (the result is already in the
        // channel when the next frame polls it).
        #[cfg(target_arch = "wasm32")]
        {
            let result = model.replay(0).map_err(|e| e.to_string());
            let _ = tx.send(result);
        }
        self.job = Some(Mutex::new(rx));
        self.status = "Simulando...".into();
    }

    fn run(&mut self, act: Act) {
        match act {
            Act::Undo => {
                self.doc.undo();
                self.refresh();
            }
            Act::Redo => {
                self.doc.redo();
                self.refresh();
            }
            Act::Save => self.save(),
            Act::SaveAs => self.save_as_prompt(),
            Act::New => self.request_new(),
            Act::Frame => self.frame_request = true,
            Act::Simulate => self.simulate(),
            Act::Edit => self.mode = Mode::Edit,
            Act::Delete => self.delete_selected(),
            Act::Copy => {
                self.copy();
            }
            Act::Cut => self.cut(),
            Act::Paste => self.paste(),
            Act::Duplicate => self.duplicate(),
            Act::SelectAll => self.select_all(),
            Act::Align(how) => {
                let names = self.selection();
                let command = arrange::align_command(self.doc.model(), &names, how);
                self.exec_some(command);
            }
            Act::Distribute(axis) => {
                let names = self.selection();
                let command = arrange::distribute_command(self.doc.model(), &names, axis);
                self.exec_some(command);
            }
            Act::Exec(c) => self.exec(c),
        }
    }
}

pub fn in_edit(editor: Res<Editor>) -> bool {
    editor.mode == Mode::Edit
}

pub fn in_sim(editor: Res<Editor>) -> bool {
    editor.mode == Mode::Sim
}

/// Keeps derived state in line with the document.
pub fn refresh(mut editor: ResMut<Editor>) {
    if editor.editable {
        editor.refresh();
    }
}

/// Picks up the result of the background simulation and switches to `Simular`.
pub fn poll_job(
    mut editor: ResMut<Editor>,
    mut data: ResMut<Data>,
    mut pb: ResMut<Playback>,
    mut selection: ResMut<Selection>,
) {
    let Some(job) = &editor.job else {
        return;
    };
    let received = match job.lock() {
        Ok(rx) => rx.try_recv(),
        Err(_) => Err(TryRecvError::Disconnected),
    };
    let result = match received {
        Ok(result) => result,
        Err(TryRecvError::Empty) => return,
        Err(TryRecvError::Disconnected) => Err("a simulação terminou sem resultado".to_string()),
    };
    editor.job = None;
    match result {
        Ok(replay) => {
            let logical = logical_flags(editor.doc.model(), &replay);
            *data =
                Data::new(replay, editor.transit, Some(editor.doc.model())).with_logical(logical);
            *pb = Playback::new(data.end);
            *selection = Selection(None);
            editor.mode = Mode::Sim;
            editor.status.clear();
            editor.frame_request = true;
        }
        Err(e) => editor.status = format!("Erro na simulação: {e}"),
    }
}

/// `Editar`: keyboard shortcuts.
pub fn keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut editor: ResMut<Editor>,
    mut contexts: EguiContexts,
) {
    if contexts
        .ctx_mut()
        .is_ok_and(|ctx| ctx.egui_wants_keyboard_input())
    {
        return;
    }
    let ctrl = keys.any_pressed([
        KeyCode::ControlLeft,
        KeyCode::ControlRight,
        KeyCode::SuperLeft,
        KeyCode::SuperRight,
    ]);
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let pressed = |k| keys.just_pressed(k);
    let objects = editor.layer == Layer::Objects;
    if ctrl && !objects {
        // Clipboard and selection shortcuts are for objects; history and file still work.
        if pressed(KeyCode::KeyZ) {
            editor.run(if shift { Act::Redo } else { Act::Undo });
        } else if pressed(KeyCode::KeyY) {
            editor.run(Act::Redo);
        } else if pressed(KeyCode::KeyS) {
            editor.run(if shift { Act::SaveAs } else { Act::Save });
        } else if pressed(KeyCode::KeyN) {
            editor.run(Act::New);
        }
        return;
    }
    if !objects {
        if pressed(KeyCode::Delete) {
            editor.run(Act::Delete);
        }
        if pressed(KeyCode::F1) {
            editor.shortcuts_open = !editor.shortcuts_open;
        }
        if pressed(KeyCode::Escape) {
            layers::escape(&mut editor);
            editor.shortcuts_open = false;
            editor.save_prompt = None;
            editor.confirm_new = false;
            editor.show_blocked = false;
        }
        return;
    }
    if ctrl {
        if pressed(KeyCode::KeyZ) {
            editor.run(if shift { Act::Redo } else { Act::Undo });
        } else if pressed(KeyCode::KeyY) {
            editor.run(Act::Redo);
        } else if pressed(KeyCode::KeyS) {
            editor.run(if shift { Act::SaveAs } else { Act::Save });
        } else if pressed(KeyCode::KeyN) {
            editor.run(Act::New);
        } else if pressed(KeyCode::KeyC) {
            editor.run(Act::Copy);
        } else if pressed(KeyCode::KeyX) {
            editor.run(Act::Cut);
        } else if pressed(KeyCode::KeyV) {
            editor.run(Act::Paste);
        } else if pressed(KeyCode::KeyD) {
            editor.run(Act::Duplicate);
        } else if pressed(KeyCode::KeyA) {
            editor.run(Act::SelectAll);
        }
        return;
    }
    if pressed(KeyCode::Delete) {
        editor.run(Act::Delete);
    }
    if pressed(KeyCode::F1) {
        editor.shortcuts_open = !editor.shortcuts_open;
    }
    let step = editor.snap_step().unwrap_or(0.1);
    for (key, delta) in [
        (KeyCode::ArrowLeft, [-step, 0.0]),
        (KeyCode::ArrowRight, [step, 0.0]),
        (KeyCode::ArrowUp, [0.0, step]),
        (KeyCode::ArrowDown, [0.0, -step]),
    ] {
        if pressed(key) {
            editor.nudge(delta);
        }
    }
    if pressed(KeyCode::KeyC)
        && let Some(sel) = editor.selected.clone()
    {
        editor.start_connect(sel);
    }
    if pressed(KeyCode::Escape) {
        editor.tool = Tool::Select;
        editor.palette_drag = None;
        editor.band = None;
        editor.shortcuts_open = false;
        editor.save_prompt = None;
        editor.confirm_new = false;
        editor.show_blocked = false;
    }
}

/// Where the ray meets the ground plane, as Bevy `(x, z)`.
fn ray_ground(ray: Ray3d) -> Option<Vec2> {
    let dir = *ray.direction;
    if dir.y.abs() < 1e-6 {
        return None;
    }
    let t = -ray.origin.y / dir.y;
    (t > 0.0).then(|| {
        let p = ray.origin + dir * t;
        Vec2::new(p.x, p.z)
    })
}

/// Every block hit by the ray, nearest first (ties keep the model order).
fn pick_stack(view: &EditView, ray: Ray3d) -> Vec<usize> {
    let mut hits: Vec<(f32, usize)> = Vec::new();
    for (i, geom) in view.geoms.iter().enumerate() {
        let h = geom.half;
        let center = Vec3::from(view.positions[i]) + Vec3::Y * h[1];
        if let Some(d) = scene::ray_box(
            ray.origin,
            *ray.direction,
            center - Vec3::from(h),
            center + Vec3::from(h),
        ) {
            hits.push((d, i));
        }
    }
    hits.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    hits.into_iter().map(|(_, i)| i).collect()
}

/// `Editar`: select (Shift adds or removes, Shift-drag on the floor draws a box), drag
/// the selection, place (also by dropping a palette entry), connect and disconnect with
/// the left button.
#[allow(clippy::too_many_arguments)] // Bevy system: each parameter is a resource or query
pub fn mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut editor: ResMut<Editor>,
    mut lock: ResMut<CameraLock>,
    dock: Res<Dock>,
    mut contexts: EguiContexts,
) {
    let editor = &mut *editor;
    let over_ui = contexts
        .ctx_mut()
        .is_ok_and(|ctx| pointer_over_ui(ctx, &dock));
    let (cam, cam_tf) = *camera;
    let cursor = window.cursor_position();
    let ray = cursor.and_then(|c| cam.viewport_to_world(cam_tf, c).ok());
    let ground = ray.and_then(ray_ground);

    let shift = keys.any_pressed([
        KeyCode::ShiftLeft,
        KeyCode::ShiftRight,
        KeyCode::ControlLeft,
        KeyCode::ControlRight,
    ]);
    if editor.layer != Layer::Objects {
        editor.stack_hover.clear();
        let pointer = layers::Pointer {
            cursor,
            ray,
            ground,
            over_ui,
            shift,
            just_pressed: buttons.just_pressed(MouseButton::Left),
            pressed: buttons.pressed(MouseButton::Left),
            just_released: buttons.just_released(MouseButton::Left),
        };
        layers::mouse(editor, &pointer, cam, cam_tf, &mut lock);
        return;
    }
    // Dropping a palette entry dragged from the panel: only the 3D area counts (egui
    // still owns the pointer while its drag lasts).
    let in_view = cursor.is_some_and(|c| dock.free.contains(egui::pos2(c.x, c.y)));
    let step = editor.snap_step();
    let ghost = editor.palette_drag.is_some() || matches!(editor.tool, Tool::Place(_));
    editor.hover = if ghost && (in_view && (editor.palette_drag.is_some() || !over_ui)) {
        ground.map(|g| ground_to_model_snapped(g.x, g.y, step))
    } else {
        None
    };
    if buttons.just_released(MouseButton::Left)
        && let Some(id) = editor.palette_drag.take()
    {
        if let (true, Some(at)) = (in_view, editor.hover) {
            editor.place(id, at, false);
        }
        editor.press = None;
        lock.0 = false;
        return;
    }

    // Objects under the cursor, for the tooltip of overlapping objects.
    let stack: Vec<usize> = match ray {
        Some(ray) if !over_ui && in_view => pick_stack(&editor.view, ray),
        _ => Vec::new(),
    };
    editor.stack_hover = if editor.tool == Tool::Select && stack.len() > 1 {
        stack
            .iter()
            .map(|&i| editor.view.names[i].clone())
            .collect()
    } else {
        Vec::new()
    };

    if buttons.just_pressed(MouseButton::Left) {
        editor.press = None;
        lock.0 = false;
        if !over_ui && let (Some(start), Some(ray)) = (cursor, ray) {
            let hits = pick_stack(&editor.view, ray);
            let names: Vec<String> = hits.iter().map(|&i| editor.view.names[i].clone()).collect();
            // A press on the spot of the last click keeps that click's object, so a
            // drag moves what is highlighted, not whatever is on top.
            let kept = editor
                .cycle
                .as_ref()
                .filter(|c| c.holds([start.x, start.y], &names))
                .and_then(|c| editor.view.index_of(c.current()));
            let block = kept.or(hits.first().copied());
            let mut press = Press {
                start,
                block,
                grab: Vec2::ZERO,
                moved: false,
                starts: Vec::new(),
                shift,
                toggled: false,
                band: false,
            };
            if editor.tool == Tool::Select {
                match block {
                    Some(i) => {
                        let name = editor.view.names[i].clone();
                        let mut chosen = editor.selection().contains(&name);
                        if !chosen && shift {
                            toggle_selected(&mut editor.selected, &mut editor.multi, &name);
                            press.toggled = true;
                            chosen = true;
                        }
                        if !chosen {
                            editor.select_only(Some(name.clone()));
                        }
                        // The block drags the whole selection when it belongs to it.
                        let group = if chosen {
                            editor.selection()
                        } else {
                            vec![name.clone()]
                        };
                        // The dragged block first: its start is the reference.
                        let mut names = vec![name.clone()];
                        names.extend(group.into_iter().filter(|n| *n != name));
                        press.starts = arrange::positions(editor.doc.model(), &names);
                        lock.0 = true;
                        if let Some(g) = ground {
                            let p = editor.view.positions[i];
                            press.grab = Vec2::new(p[0], p[2]) - g;
                        }
                    }
                    None if shift => {
                        press.band = true;
                        lock.0 = true;
                        editor.band = Some((start, start));
                    }
                    None => {}
                }
            }
            editor.press = Some(press);
        }
    }

    if buttons.pressed(MouseButton::Left)
        && editor.tool == Tool::Select
        && let Some(cursor) = cursor
        && let Some(press) = editor.press.as_mut()
    {
        if cursor.distance(press.start) > 4.0 {
            press.moved = true;
        }
        if press.band {
            let start = press.start;
            editor.band = Some((start, cursor));
        } else if press.moved
            && let (Some(g), Some((_, first))) = (ground, press.starts.first())
        {
            // Dragging the pressed block (and the selection with it) along the ground,
            // one coalesced undo step; the dragged block lands on the grid.
            let first = *first;
            let at = ground_to_model_snapped(g.x + press.grab.x, g.y + press.grab.y, step);
            let delta = [at[0] - first[0], at[1] - first[1]];
            let command = arrange::moved_from(&press.starts, delta);
            let name = press.starts[0].0.clone();
            let current = editor
                .doc
                .model()
                .objects
                .iter()
                .find(|o| o.name == name)
                .map(|o| o.position);
            if current.is_some_and(|c| c[0] != at[0] || c[1] != at[1]) {
                if let Err(e) = editor.doc.apply_coalescing(command, Some("mover:selecao")) {
                    editor.status = e;
                }
                editor.refresh();
            }
        }
    }

    if buttons.just_released(MouseButton::Left) {
        lock.0 = false;
        // Ends a look edit made in the panel (colour picker) as well as a drag.
        editor.doc.end_gesture();
        let Some(press) = editor.press.take() else {
            return;
        };
        if press.band {
            editor.band = None;
            if let Some(end) = cursor.filter(|_| press.moved) {
                let points: Vec<Option<[f32; 2]>> = (0..editor.view.names.len())
                    .map(|i| {
                        let h = editor.view.geoms[i].half[1];
                        let p = Vec3::from(editor.view.positions[i]) + Vec3::Y * h;
                        cam.world_to_viewport(cam_tf, p).ok().map(|v| [v.x, v.y])
                    })
                    .collect();
                let inside = inside_box(&points, [press.start.x, press.start.y], [end.x, end.y]);
                let names: Vec<String> = inside
                    .into_iter()
                    .map(|i| editor.view.names[i].clone())
                    .collect();
                let n = names.len();
                select_names(&mut editor.selected, &mut editor.multi, names, true);
                editor.status = format!(
                    "{n} objeto(s) na caixa, {} selecionado(s)",
                    editor.selection().len()
                );
            }
            return;
        }
        if press.moved {
            editor.doc.end_gesture();
            return;
        }
        let Some(cursor) = cursor.filter(|c| c.distance(press.start) <= 4.0) else {
            return;
        };
        click(editor, cam, cam_tf, cursor, &press, ground, ray);
    }
}

/// A click (press and release without moving) in `Editar`.
fn click(
    editor: &mut Editor,
    cam: &Camera,
    cam_tf: &GlobalTransform,
    cursor: Vec2,
    press: &Press,
    ground: Option<Vec2>,
    ray: Option<Ray3d>,
) {
    let block = press.block;
    match editor.tool.clone() {
        Tool::Place(id) => {
            let Some(g) = ground else { return };
            let at = ground_to_model_snapped(g.x, g.y, editor.snap_step());
            // Shift keeps the tool, to place several in a row.
            editor.place(id, at, press.shift);
        }
        Tool::Connect(from) => {
            if let Some(i) = block {
                let to = editor.view.names[i].clone();
                if to == from {
                    editor.status = "Escolha outro bloco como destino".into();
                    return;
                }
                if let Some(msg) = connect_blocked(editor.view.kinds[i]) {
                    editor.status = msg.into();
                    editor.tool = Tool::Select;
                    return;
                }
                editor.exec(Command::Connect {
                    from: from.clone(),
                    to,
                });
                editor.select_only(Some(from));
            }
            editor.tool = Tool::Select;
        }
        Tool::Select => {
            if let Some(i) = block {
                let name = editor.view.names[i].clone();
                if press.shift {
                    if !press.toggled {
                        toggle_selected(&mut editor.selected, &mut editor.multi, &name);
                    }
                } else {
                    // Clicking again on the same spot walks through the objects there.
                    let stack: Vec<String> = ray
                        .map(|ray| pick_stack(&editor.view, ray))
                        .unwrap_or_default()
                        .into_iter()
                        .map(|k| editor.view.names[k].clone())
                        .collect();
                    if stack.len() > 1 {
                        let at = [cursor.x, cursor.y];
                        let cycle = Cycle::advance(editor.cycle.as_ref(), at, stack);
                        let chosen = cycle.current().to_string();
                        editor.cycle = Some(cycle);
                        editor.select_only(Some(chosen));
                    } else {
                        editor.cycle = None;
                        editor.select_only(Some(name));
                    }
                }
                return;
            }
            if press.shift {
                return;
            }
            let mut screen = Vec::new();
            let mut owners = Vec::new();
            for (n, &(a, b)) in editor.view.connections.iter().enumerate() {
                if let Some((s, e)) =
                    scene::connection_line(&editor.view.positions, &editor.view.geoms, a, b)
                    && let (Ok(ps), Ok(pe)) = (
                        cam.world_to_viewport(cam_tf, s),
                        cam.world_to_viewport(cam_tf, e),
                    )
                {
                    screen.push(([ps.x, ps.y], [pe.x, pe.y]));
                    owners.push(n);
                }
            }
            match nearest_segment([cursor.x, cursor.y], &screen, LINE_PICK_PIXELS) {
                Some(k) => {
                    let (a, b) = editor.view.connections[owners[k]];
                    let (from, to) = (editor.view.names[a].clone(), editor.view.names[b].clone());
                    editor.exec(Command::Disconnect {
                        from: from.clone(),
                        to: to.clone(),
                    });
                    editor.status = format!("Ligação {from} -> {to} removida (Ctrl+Z desfaz)");
                }
                None => editor.select_only(None),
            }
        }
        // The network and fluid tools belong to their layers (`layers.rs`).
        Tool::Node | Tool::Edge(_) | Tool::FluidPlace(_) | Tool::FluidLink(_) => {}
    }
}

/// `Editar`: connections, grid, outlines (selection white, issues red, connection
/// source yellow) and the ghost of the block being placed.
pub fn draw_edit(editor: Res<Editor>, mut gizmos: Gizmos) {
    let view = &editor.view;
    let mut outlines: Vec<(usize, Color)> = Vec::new();
    // The object selection is only outlined in its own layer.
    let objects = editor.layer == Layer::Objects;
    if let Some(i) = editor
        .selected
        .as_deref()
        .filter(|_| objects)
        .and_then(|s| view.index_of(s))
    {
        outlines.push((i, Color::srgb(1.0, 1.0, 1.0)));
    }
    for i in editor
        .multi
        .iter()
        .filter(|_| objects)
        .filter_map(|s| view.index_of(s))
    {
        outlines.push((i, Color::srgb(0.45, 0.80, 1.0)));
    }
    if let Tool::Connect(from) = &editor.tool
        && let Some(i) = view.index_of(from)
    {
        outlines.push((i, Color::srgb(1.0, 0.85, 0.2)));
    }
    for issue in &editor.issues {
        if let Some(i) = issue.object.as_deref().and_then(|o| view.index_of(o)) {
            outlines.retain(|(j, _)| *j != i);
            outlines.push((i, Color::srgb(1.0, 0.25, 0.2)));
        }
    }
    scene::draw_world(
        &mut gizmos,
        &view.positions,
        &view.geoms,
        &view.connections,
        &view.logical,
        &outlines,
        Some(editor.snap_step().map_or(1.0, |s| s as f32)),
    );
    scene::draw_network(&mut gizmos, &NetworkView::of(editor.doc.model()));
    layers::draw(&editor, &mut gizmos);
    let placing = match (&editor.tool, editor.palette_drag) {
        (_, Some(id)) => Some(id),
        (Tool::Place(id), None) => Some(*id),
        _ => None,
    };
    if let (Some(at), Some(entry)) = (editor.hover, placing.and_then(palette_entry)) {
        let (h, _) = style::extents(style::shape_of(entry.kind));
        let base = Vec3::from(layout::to_bevy(at));
        gizmos.cube(
            Transform::from_translation(base + Vec3::Y * h[1]).with_scale(Vec3::new(
                h[0] * 2.0,
                h[1] * 2.0,
                h[2] * 2.0,
            )),
            Color::srgb(0.3, 0.9, 0.4),
        );
    }
}

/// Window title: file name, `*` when there are unsaved changes.
pub fn title(
    editor: Res<Editor>,
    data: Res<Data>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut last: Local<String>,
) {
    let title = if editor.editable {
        let dirty = if editor.doc.is_dirty() { "*" } else { "" };
        format!("UTSim - {}{dirty}", editor.file_name())
    } else {
        format!("UTSim - {}", data.replay.model)
    };
    if *last != title {
        window.title = title.clone();
        *last = title;
    }
}

/// Top bar (both modes, only when the model is editable): mode switch, file and
/// history buttons, simulation time limit, status; plus the small dialogs.
pub fn toolbar(
    mut contexts: EguiContexts,
    mut editor: ResMut<Editor>,
    mut dock: ResMut<Dock>,
    mut visual: ResMut<crate::models::VisualMode>,
    mut lighting: ResMut<crate::scene::Lighting>,
    mut cameras: ResMut<crate::camera_modes::CameraModes>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    if !editor.editable {
        return Ok(());
    }
    let mut acts: Vec<Act> = Vec::new();
    let ed = &mut *editor;
    let mut top = dock.ui(ctx, "topo");
    egui::Panel::top("barra").show(&mut top, |ui| {
        ui.horizontal(|ui| {
            let editing = ed.mode == Mode::Edit;
            if ui.selectable_label(editing, "Editar").clicked() && !editing {
                acts.push(Act::Edit);
            }
            let running = ed.job.is_some();
            if ui
                .add_enabled(!running, egui::Button::new("▶ Simular").selected(!editing))
                .clicked()
                && editing
            {
                acts.push(Act::Simulate);
            }
            ui.label("até t =");
            ui.add_enabled(
                editing,
                egui::TextEdit::singleline(&mut ed.stop_text).desired_width(56.0),
            );
            ui.separator();
            // File and history buttons stay visible in `Simular`, only disabled.
            ui.add_enabled_ui(editing, |ui| {
                if ui.button("Novo").on_hover_text("Ctrl+N").clicked() {
                    acts.push(Act::New);
                }
                if ui.button("Salvar").on_hover_text("Ctrl+S").clicked() {
                    acts.push(Act::Save);
                }
                if ui
                    .button("Salvar como...")
                    .on_hover_text("Ctrl+Shift+S")
                    .clicked()
                {
                    acts.push(Act::SaveAs);
                }
                if ui
                    .add_enabled(ed.doc.can_undo(), egui::Button::new("Desfazer"))
                    .on_hover_text("Ctrl+Z")
                    .clicked()
                {
                    acts.push(Act::Undo);
                }
                if ui
                    .add_enabled(ed.doc.can_redo(), egui::Button::new("Refazer"))
                    .on_hover_text("Ctrl+Y")
                    .clicked()
                {
                    acts.push(Act::Redo);
                }
            });
            ui.separator();
            if ui.button("Enquadrar (F)").clicked() {
                acts.push(Act::Frame);
            }
            if ui
                .selectable_label(ed.flows.open, "Fluxos")
                .on_hover_text("Lógica do usuário: fluxos de tokens")
                .clicked()
            {
                ed.flows.open = !ed.flows.open;
            }
            if ui
                .selectable_label(ed.results.open, "Resultados")
                .on_hover_text("Replicações, resumo com intervalos de confiança, gráficos, cenários e otimização")
                .clicked()
            {
                let path = ed.doc.path().map(PathBuf::from).or_else(|| ed.pending_path.clone());
                ed.results.toggle(ed.doc.model(), path.as_deref());
            }
            if ui
                .selectable_label(ed.shortcuts_open, "Atalhos")
                .on_hover_text("Atalhos de teclado e mouse (F1)")
                .clicked()
            {
                ed.shortcuts_open = !ed.shortcuts_open;
            }
            ui.separator();
            crate::models::toggle(ui, &mut visual, &mut lighting);
            ui.separator();
            crate::camera_modes::selector(ui, &mut cameras);
            if running {
                ui.spinner();
            }
            if !ed.status.is_empty() {
                ui.separator();
                ui.label(&ed.status);
            }
        });
    });
    dock.free = top.available_rect_before_wrap();

    if ed.save_prompt.is_some() {
        let mut save = false;
        let mut cancel = false;
        egui::Window::new("Salvar como")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                if let Some(text) = ed.save_prompt.as_mut() {
                    ui.label(
                        "Caminho do arquivo (.ron), relativo à pasta do programa ou completo:",
                    );
                    let field = ui.add(egui::TextEdit::singleline(text).desired_width(420.0));
                    if field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        save = true;
                    }
                    field.request_focus();
                    let mut target = PathBuf::from(text.trim());
                    if target.extension().is_none() {
                        target.set_extension("ron");
                    }
                    let same = ed.doc.path().is_some_and(|p| p == target.as_path());
                    if !text.trim().is_empty() && target.exists() && !same {
                        ui.colored_label(
                            egui::Color32::from_rgb(240, 200, 80),
                            "Esse arquivo já existe e será sobrescrito.",
                        );
                    }
                }
                ui.horizontal(|ui| {
                    save |= ui.button("Salvar").clicked();
                    cancel = ui.button("Cancelar").clicked();
                });
            });
        if save && let Some(text) = ed.save_prompt.clone() {
            ed.save_as_text(&text);
        } else if cancel {
            ed.save_prompt = None;
        }
    }

    if ed.confirm_new {
        egui::Window::new("Novo modelo")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Há alterações não salvas. Descartá-las e começar um modelo novo?");
                ui.horizontal(|ui| {
                    if ui.button("Descartar").clicked() {
                        ed.new_document();
                    }
                    if ui.button("Cancelar").clicked() {
                        ed.confirm_new = false;
                    }
                });
            });
    }

    if ed.shortcuts_open {
        let mut open = true;
        egui::Window::new("Atalhos do editor")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                egui::Grid::new("atalhos")
                    .num_columns(2)
                    .striped(true)
                    .spacing([18.0, 4.0])
                    .show(ui, |ui| {
                        for (keys, what) in SHORTCUTS {
                            ui.strong(keys);
                            ui.label(what);
                            ui.end_row();
                        }
                    });
                ui.separator();
                ui.weak("F1 ou Esc fecha esta janela.");
            });
        if !open {
            ed.shortcuts_open = false;
        }
    }

    if ed.show_blocked {
        egui::Window::new("Não é possível simular")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Corrija os problemas do modelo antes de simular:");
                issues_ui(ui, &ed.issues, &mut ed.selected);
                if ui.button("Fechar").clicked() {
                    ed.show_blocked = false;
                }
            });
    }

    for act in acts {
        editor.run(act);
    }
    Ok(())
}

/// The "Resultados" window (both modes, editable models only). Its jobs are polled
/// even while the window is closed.
pub fn results_window(mut contexts: EguiContexts, mut editor: ResMut<Editor>) -> Result {
    let ctx = contexts.ctx_mut()?;
    if !editor.editable {
        return Ok(());
    }
    let ed = &mut *editor;
    let path = ed
        .doc
        .path()
        .map(PathBuf::from)
        .or_else(|| ed.pending_path.clone());
    let requests = crate::results_panel::window(
        ctx,
        &mut ed.results,
        ed.doc.model(),
        path.as_deref(),
        ed.issues.len(),
    );
    for request in requests {
        match request {
            crate::results_panel::Request::Exec(command) => {
                editor.exec(command);
                if editor.status.is_empty() {
                    editor.status = "Valores gravados no modelo (Ctrl+Z desfaz)".into();
                }
            }
        }
    }
    Ok(())
}

pub fn flows_open(editor: Res<Editor>) -> bool {
    editor.editable && editor.flows.open
}

/// The docked "Fluxos" panel (bottom, both modes). In `Editar` its edits are applied to
/// the document; in `Simular` the diagram shows the live token counts of the replay.
pub fn flows_panel(
    mut contexts: EguiContexts,
    mut editor: ResMut<Editor>,
    mut dock: ResMut<Dock>,
    data: Res<Data>,
    pb: Res<Playback>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let ed = &mut *editor;
    let editing = ed.mode == Mode::Edit;
    let lookup = |name: &str| data.flow_block(name);
    let live = (!editing).then(|| flows_ui::Live {
        timeline: &data.timeline,
        flow_block: &lookup,
        t: pb.t,
    });
    let mut ui = dock.ui(ctx, "fluxos");
    let mut commands: Vec<Command> = Vec::new();
    egui::Panel::bottom("fluxos")
        .resizable(true)
        .default_size(340.0)
        .size_range(180.0..=620.0)
        .show(&mut ui, |ui| {
            commands = flows_ui::panel_body(ui, &mut ed.flows, &ed.doc, live, editing);
        });
    dock.free = ui.available_rect_before_wrap();
    for command in commands {
        editor.exec(command);
    }
    Ok(())
}

/// Issues grouped by object; clicking an object name selects it.
fn issues_ui(ui: &mut egui::Ui, issues: &[Issue], selected: &mut Option<String>) {
    if issues.is_empty() {
        ui.label("Nenhum problema: o modelo pode ser simulado.");
        return;
    }
    let red = egui::Color32::from_rgb(255, 120, 110);
    for (object, messages) in group_issues(issues) {
        match object {
            None => {
                ui.strong("Modelo");
            }
            Some(name) => {
                if ui.link(name.clone()).clicked() {
                    *selected = Some(name);
                }
            }
        }
        for message in messages {
            ui.colored_label(red, format!("- {message}"));
        }
    }
}

/// Warnings grouped by object, in yellow: the model runs, but probably not as meant.
fn warnings_ui(ui: &mut egui::Ui, warnings: &[Issue], selected: &mut Option<String>) {
    let yellow = egui::Color32::from_rgb(240, 200, 80);
    for (object, messages) in group_issues(warnings) {
        match object {
            None => {
                ui.strong("Modelo");
            }
            Some(name) => {
                if ui.link(name.clone()).clicked() {
                    *selected = Some(name);
                }
            }
        }
        for message in messages {
            ui.colored_label(yellow, format!("- {message}"));
        }
    }
}

/// `Editar`: palette, properties of the selected object, connections and issues.
pub fn edit_panels(
    mut contexts: EguiContexts,
    mut editor: ResMut<Editor>,
    mut dock: ResMut<Dock>,
    mut lod: ResMut<crate::lod::Lod>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let ed = &mut *editor;
    let (cam, cam_tf) = *camera;
    let mut acts: Vec<Act> = Vec::new();

    let mut left = dock.ui(ctx, "paleta");
    egui::Panel::left("paleta")
        .resizable(false)
        .exact_size(172.0)
        .show(&mut left, |ui| {
            layers::layer_switch(ui, ed);
            ui.separator();
            if ed.layer == Layer::Objects {
                ui.horizontal(|ui| {
                    for (tab, label) in [
                        (LeftTab::Library, "Biblioteca"),
                        (LeftTab::Objects, "Objetos"),
                    ] {
                        if ui.selectable_label(ed.left_tab == tab, label).clicked() {
                            ed.left_tab = tab;
                        }
                    }
                });
                ui.separator();
                match ed.left_tab {
                    LeftTab::Library => {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, true])
                            .show(ui, |ui| palette_ui(ui, ed));
                    }
                    LeftTab::Objects => objects_ui(ui, ed),
                }
            } else {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| layers::tools_ui(ui, ed));
            }
        });
    dock.free = left.available_rect_before_wrap();
    if let Some(id) = ed.palette_drag
        && let Some(entry) = palette_entry(id)
        && let Some(p) = ctx.pointer_latest_pos()
    {
        // The entry follows the pointer while it is dragged to the floor.
        egui::Area::new(egui::Id::new("paleta_arrasto"))
            .order(egui::Order::Tooltip)
            .fixed_pos(p + egui::vec2(14.0, 8.0))
            .interactable(false)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.label(format!("{} {}", palette_icon(id), entry.label));
                });
            });
    }
    if let Some((a, b)) = ed.band {
        let rect = egui::Rect::from_two_pos(egui::pos2(a.x, a.y), egui::pos2(b.x, b.y));
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("caixa_selecao"),
        ));
        let blue = egui::Color32::from_rgb(115, 205, 255);
        painter.rect_filled(rect, 0.0, blue.gamma_multiply(0.12));
        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(1.5, blue),
            egui::StrokeKind::Inside,
        );
    }

    let mut right = dock.ui(ctx, "propriedades");
    egui::Panel::right("propriedades")
        .default_size(330.0)
        .size_range(260.0..=600.0)
        .show(&mut right, |ui| {
            ui.heading("Propriedades");
            // Another object (or none) selected: start again at the top, where the name is.
            let mut scroll = egui::ScrollArea::vertical().auto_shrink([false, false]);
            if ed.props_for != ed.selected {
                ed.props_for = ed.selected.clone();
                scroll = scroll.vertical_scroll_offset(0.0);
            }
            scroll.show(ui, |ui| {
                match ed.selected.clone() {
                    _ if ed.layer != Layer::Objects => {
                        for command in layers::properties_ui(ui, ed) {
                            acts.push(Act::Exec(command));
                        }
                    }
                    None => {
                        ui.label("Selecione um bloco para editar suas propriedades.");
                        ui.separator();
                        for command in run_panel(ui, &ed.doc) {
                            acts.push(Act::Exec(command));
                        }
                    }
                    Some(name) if !ed.multi.is_empty() => {
                        selection_ui(ui, ed, &name, &mut acts);
                    }
                    Some(name) => {
                        ui.horizontal(|ui| {
                            ui.strong(&name);
                            if ui.button("Ligar (C)").clicked() {
                                ed.start_connect(name.clone());
                            }
                            if ui.button("Excluir (Del)").clicked() {
                                acts.push(Act::Delete);
                            }
                        });
                        if matches!(ed.tool, Tool::Connect(_)) {
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 215, 80),
                                "Clique no bloco de destino (Esc cancela).",
                            );
                        }
                        ui.separator();
                        for command in properties_panel(ui, &ed.doc, &name) {
                            acts.push(Act::Exec(command));
                        }
                        ui.separator();
                        connections_ui(ui, &ed.doc, &name, &mut acts);
                    }
                }
                ui.separator();
                ui.strong(format!("Problemas ({})", ed.issues.len()));
                let before = ed.selected.clone();
                issues_ui(ui, &ed.issues, &mut ed.selected);
                if !ed.warnings.is_empty() {
                    ui.separator();
                    ui.strong(format!("Avisos ({})", ed.warnings.len()));
                    warnings_ui(ui, &ed.warnings, &mut ed.selected);
                }
                if ed.selected != before {
                    ed.multi.clear();
                    ed.layer = Layer::Objects;
                    ed.tool = Tool::Select;
                }
            });
        });
    dock.free = right.available_rect_before_wrap();

    let chosen = ed.selection();
    // Big models: only the nearest labels are laid out (see `lod.rs`).
    let mask = crate::lod::label_mask(
        &lod,
        cam_tf.translation(),
        ed.view.positions.iter().map(|p| Vec3::from(*p)),
        |i| chosen.contains(&ed.view.names[i]),
    );
    let mut labels: Vec<Label> = (0..ed.view.names.len())
        .filter(|i| mask[*i])
        .map(|i| Label {
            text: ed.view.names[i].clone(),
            position: ed.view.positions[i],
            height: ed.view.geoms[i].label_height(0),
            selected: chosen.contains(&ed.view.names[i]),
        })
        .collect();
    lod.stats.labels_drawn = labels.len();
    lod.stats.labels_total = mask.len();
    labels.extend(capacity_labels(&NetworkView::of(ed.doc.model())));
    labels.extend(layers::labels(ed));
    draw_labels(ctx, cam, cam_tf, &labels, dock.free);
    if ed.stack_hover.len() > 1
        && let Some(p) = ctx.pointer_latest_pos()
    {
        let hint = picking::stack_hint(&ed.stack_hover, ed.selected.as_deref());
        egui::Area::new(egui::Id::new("pilha_sob_cursor"))
            .order(egui::Order::Tooltip)
            .fixed_pos(p + egui::vec2(16.0, 14.0))
            .interactable(false)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.set_max_width(320.0);
                    ui.label(hint);
                });
            });
    }

    // Commands from the property panel may rename the selected object.
    let property_commands: Vec<Command> = acts
        .iter()
        .filter_map(|a| match a {
            Act::Exec(c) => Some(c.clone()),
            _ => None,
        })
        .collect();
    if let Some(sel) = ed.selected.clone() {
        ed.selected = Some(renamed(&property_commands, &sel));
    }
    layers::follow_renames(ed, &property_commands);
    for act in acts {
        editor.run(act);
    }
    Ok(())
}

/// The object library: every palette entry grouped by category (click, then click on
/// the floor; or drag it to the floor), and the grid settings.
fn palette_ui(ui: &mut egui::Ui, ed: &mut Editor) {
    for category in CATEGORIES {
        egui::CollapsingHeader::new(category)
            .default_open(true)
            .show(ui, |ui| {
                for entry in PALETTE.iter().filter(|e| e.category == category) {
                    let on = ed.tool == Tool::Place(entry.id) || ed.palette_drag == Some(entry.id);
                    // Icon and label are separate texts, so scripts find the label exactly.
                    let text = (palette_icon(entry.id), entry.label);
                    let response = ui
                        .add(
                            egui::Button::selectable(on, text).sense(egui::Sense::click_and_drag()),
                        )
                        .on_hover_text(format!(
                            "{}: clique e depois clique no chão, ou arraste até o chão",
                            entry.kind
                        ));
                    if response.drag_started() {
                        ed.palette_drag = Some(entry.id);
                    }
                    if response.clicked() {
                        ed.tool = if on {
                            Tool::Select
                        } else {
                            Tool::Place(entry.id)
                        };
                    }
                }
            });
    }
    ui.separator();
    ui.checkbox(&mut ed.snap, "Encaixar na grade");
    ui.horizontal(|ui| {
        ui.label("Passo (m)");
        ui.add_enabled(
            ed.snap,
            egui::DragValue::new(&mut ed.grid_step)
                .range(0.1..=10.0)
                .speed(0.05)
                .max_decimals(2),
        );
    });
    ui.separator();
    ui.weak("Esc cancela. Shift + clique no chão coloca vários.");
    ui.weak("Atalhos: F1.");
}

/// Right panel for several selected objects: actions on all of them, align and
/// distribute, and the properties shared by objects of one kind (each edit applies to
/// every selected object, as one undo step).
fn selection_ui(ui: &mut egui::Ui, ed: &mut Editor, primary: &str, acts: &mut Vec<Act>) {
    let names = ed.selection();
    ui.strong(format!("{} objetos selecionados", names.len()));
    ui.label(names.join(", "));
    ui.horizontal_wrapped(|ui| {
        if ui.button("Copiar").on_hover_text("Ctrl+C").clicked() {
            acts.push(Act::Copy);
        }
        if ui.button("Duplicar").on_hover_text("Ctrl+D").clicked() {
            acts.push(Act::Duplicate);
        }
        if ui.button("Excluir todos").on_hover_text("Del").clicked() {
            acts.push(Act::Delete);
        }
    });
    ui.separator();
    ui.strong("Alinhar");
    ui.horizontal(|ui| {
        for (label, how) in [
            ("Esquerda", Align::Left),
            ("Centro X", Align::CenterX),
            ("Direita", Align::Right),
        ] {
            if ui.button(label).clicked() {
                acts.push(Act::Align(how));
            }
        }
    });
    ui.horizontal(|ui| {
        for (label, how) in [
            ("Topo", Align::Top),
            ("Centro Y", Align::CenterY),
            ("Base", Align::Bottom),
        ] {
            if ui.button(label).clicked() {
                acts.push(Act::Align(how));
            }
        }
    });
    ui.horizontal(|ui| {
        let three = names.len() >= 3;
        for (label, axis) in [("Distribuir em x", Axis::X), ("Distribuir em y", Axis::Y)] {
            if ui
                .add_enabled(three, egui::Button::new(label))
                .on_hover_text("Espaça igualmente entre os dois extremos (3 objetos ou mais)")
                .clicked()
            {
                acts.push(Act::Distribute(axis));
            }
        }
    });
    ui.separator();
    if arrange::same_kind(ed.doc.model(), &names) {
        ui.strong("Propriedades de todos");
        ui.weak(format!(
            "Os campos mostram os valores de {primary}; cada alteração vale para os {} objetos.",
            names.len()
        ));
        ui.separator();
        let edits: Vec<Command> = properties_panel_shared(ui, &ed.doc, primary)
            .into_iter()
            .filter(|c| !matches!(c, Command::Rename { .. }))
            .collect();
        if let Some(c) = arrange::fan_out(&edits, primary, &names) {
            acts.push(Act::Exec(c));
        }
    } else {
        ui.weak(
            "Objetos de tipos diferentes: selecione só objetos do mesmo tipo para editar \
             uma propriedade de todos.",
        );
    }
}

/// The "Objetos" tab: every object of the model, filtered by name and kind. A click
/// selects (Shift or Ctrl adds or removes), a double click also frames the object.
fn objects_ui(ui: &mut egui::Ui, ed: &mut Editor) {
    ui.add(
        egui::TextEdit::singleline(&mut ed.list_filter)
            .hint_text("Buscar por nome")
            .desired_width(f32::INFINITY),
    );
    let mut kinds: Vec<&'static str> = ed
        .doc
        .model()
        .objects
        .iter()
        .map(|o| utsim_viewer::properties::spec::kind_name(&o.kind))
        .collect();
    kinds.sort_unstable();
    kinds.dedup();
    if ed.list_kind.is_some_and(|k| !kinds.contains(&k)) {
        ed.list_kind = None;
    }
    egui::ComboBox::from_id_salt("lista_tipo")
        .width(ui.available_width() - 8.0)
        .selected_text(ed.list_kind.unwrap_or("Todos os tipos"))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut ed.list_kind, None, "Todos os tipos");
            for kind in kinds {
                ui.selectable_value(&mut ed.list_kind, Some(kind), kind);
            }
        });
    let shown = picking::filter_objects(ed.doc.model(), &ed.list_filter, ed.list_kind);
    let total = ed.doc.model().objects.len();
    ui.weak(format!("{} de {} objetos", shown.len(), total));
    let multi_key = ui.input(|i| i.modifiers.shift || i.modifiers.command);
    let chosen = ed.selection();
    let red = egui::Color32::from_rgb(255, 120, 110);
    let mut clicked: Option<(String, bool, bool)> = None;
    egui::ScrollArea::vertical()
        .id_salt("lista_objetos")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for &i in &shown {
                let object = &ed.doc.model().objects[i];
                let on = chosen.contains(&object.name);
                let broken = ed
                    .issues
                    .iter()
                    .any(|x| x.object.as_deref() == Some(object.name.as_str()));
                let name = if broken {
                    egui::RichText::new(&object.name).color(red)
                } else {
                    egui::RichText::new(&object.name)
                };
                let kind = utsim_viewer::properties::spec::kind_name(&object.kind);
                let response = ui
                    .add(
                        egui::Button::selectable(on, name)
                            .min_size(egui::vec2(ui.available_width(), 0.0)),
                    )
                    .on_hover_text(format!(
                        "{kind}. Clique seleciona, Shift soma à seleção, duplo clique enquadra"
                    ));
                if response.double_clicked() {
                    clicked = Some((object.name.clone(), false, true));
                } else if response.clicked() {
                    clicked = Some((object.name.clone(), multi_key, false));
                }
            }
        });
    if let Some((name, add, frame)) = clicked {
        if add {
            toggle_selected(&mut ed.selected, &mut ed.multi, &name);
        } else {
            ed.select_only(Some(name.clone()));
        }
        ed.tool = Tool::Select;
        if frame {
            ed.focus_request = Some(name);
        }
    }
}

/// Labels of the buttons that move a connection in its order.
const MOVE_UP: &str = "Subir";
const MOVE_DOWN: &str = "Descer";

/// One connection of the order lists: its label and the buttons to move it up or down
/// (swapping with the neighbour in the same list) and to remove it.
fn order_row(
    ui: &mut egui::Ui,
    list: &[&(String, String)],
    k: usize,
    label: String,
    acts: &mut Vec<Act>,
) {
    let this = list[k];
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        if ui
            .add_enabled(k > 0, egui::Button::new(MOVE_UP).small())
            .on_hover_text("Troca de lugar com a anterior (Ctrl+Z desfaz)")
            .clicked()
        {
            acts.push(Act::Exec(Command::SwapConnections {
                first: list[k - 1].clone(),
                second: this.clone(),
            }));
        }
        if ui
            .add_enabled(k + 1 < list.len(), egui::Button::new(MOVE_DOWN).small())
            .on_hover_text("Troca de lugar com a seguinte (Ctrl+Z desfaz)")
            .clicked()
        {
            acts.push(Act::Exec(Command::SwapConnections {
                first: this.clone(),
                second: list[k + 1].clone(),
            }));
        }
        if ui
            .small_button("x")
            .on_hover_text("Remover ligação")
            .clicked()
        {
            acts.push(Act::Exec(Command::Disconnect {
                from: this.0.clone(),
                to: this.1.clone(),
            }));
        }
    });
}

/// "Ligações" of the object: outputs in port order and inputs in pull-priority order,
/// each with buttons to move it up or down and to remove it.
fn connections_ui(ui: &mut egui::Ui, doc: &Document, name: &str, acts: &mut Vec<Act>) {
    let model = doc.model();
    let outputs: Vec<&(String, String)> = model
        .connections
        .iter()
        .filter(|(a, _)| a == name)
        .collect();
    let inputs: Vec<&(String, String)> = model
        .connections
        .iter()
        .filter(|(_, b)| b == name)
        .collect();
    ui.strong("Ligações");
    if outputs.is_empty() && inputs.is_empty() {
        ui.weak("Nenhuma. Use \"Ligar\" e clique no destino.");
        return;
    }
    ui.weak(
        "A ordem importa. Saídas: a ordem é o número da porta (1, 2, ...) que as regras de \
         saída usam. Entradas: a primeira tem prioridade quando o objeto puxa um item.",
    );
    if !outputs.is_empty() {
        ui.label("Saídas, por porta:");
    }
    for k in 0..outputs.len() {
        let label = format!("porta {}: {name} -> {}", k + 1, outputs[k].1);
        order_row(ui, &outputs, k, label, acts);
    }
    if !inputs.is_empty() {
        ui.label("Entradas, por prioridade:");
    }
    for k in 0..inputs.len() {
        let label = format!("{}a: {} -> {name}", k + 1, inputs[k].0);
        order_row(ui, &inputs, k, label, acts);
    }
}
