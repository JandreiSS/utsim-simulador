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
use utsim_edit::{Command, Document, Issue, LIBRARY, template};
use utsim_model::replay::Replay;
use utsim_viewer::editing::{
    EditView, LINE_PICK_PIXELS, NetworkView, auto_layout_command, connect_blocked, default_stop,
    ground_to_model, group_issues, logical_flags, name_base, nearest_segment, needs_layout,
    renamed, run_model,
};
use utsim_viewer::flows_ui::{self, FlowsState};
use utsim_viewer::layout;
use utsim_viewer::properties::{properties_panel, run_panel};
use utsim_viewer::style;
use utsim_viewer::timeline::TransitSetting;

use crate::camera::CameraLock;
use crate::dock::{Dock, pointer_over_ui};
use crate::playback::{Data, Playback, Selection};
use crate::scene;
use crate::ui::{Label, draw_labels};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Edit,
    Sim,
}

/// What a left click does in `Editar`.
#[derive(Clone, PartialEq, Debug)]
pub enum Tool {
    Select,
    /// Place a new object of this kind (one of `LIBRARY`).
    Place(&'static str),
    /// Connect this object to the next block clicked.
    Connect(String),
}

/// A left-button gesture in progress.
struct Press {
    start: Vec2,
    /// Block under the cursor when the button went down.
    block: Option<usize>,
    /// Ground offset from the cursor to the block, so the block does not jump.
    grab: Vec2,
    moved: bool,
}

type RunResult = Result<Replay, String>;

#[derive(Resource)]
pub struct Editor {
    pub mode: Mode,
    /// False when a recorded `.json` replay was opened: Simular only, nothing to edit.
    pub editable: bool,
    pub doc: Document,
    /// File to create on the first save (a path given on the command line that does not exist yet).
    pub pending_path: Option<PathBuf>,
    pub view: EditView,
    pub issues: Vec<Issue>,
    pub selected: Option<String>,
    pub tool: Tool,
    press: Option<Press>,
    /// Snapped ground position under the cursor while placing.
    hover: Option<[f64; 3]>,
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
}

enum Act {
    Undo,
    Redo,
    Save,
    New,
    Frame,
    Simulate,
    Edit,
    Delete,
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
            pending_path,
            view,
            issues: Vec::new(),
            selected: None,
            tool: Tool::Select,
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
        }
    }

    pub fn with_transit(mut self, transit: TransitSetting) -> Self {
        self.transit = transit;
        self
    }

    /// Recomputes what depends on the document (view, issues, stale selection).
    pub fn refresh(&mut self) {
        self.view = EditView::of(self.doc.model());
        self.issues = self.doc.issues();
        if self
            .selected
            .as_ref()
            .is_some_and(|s| self.view.index_of(s).is_none())
        {
            self.selected = None;
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

    fn exec(&mut self, command: Command) {
        match self.doc.apply(command) {
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

    fn delete_selected(&mut self) {
        if let Some(name) = self.selected.take() {
            self.exec(Command::Remove { name });
            self.tool = Tool::Select;
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
        self.pending_path = None;
        self.selected = None;
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
            Act::New => self.request_new(),
            Act::Frame => self.frame_request = true,
            Act::Simulate => self.simulate(),
            Act::Edit => self.mode = Mode::Edit,
            Act::Delete => self.delete_selected(),
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
            *data = Data::from_replay(replay, editor.transit)
                .with_model(editor.doc.model())
                .with_network(NetworkView::of(editor.doc.model()))
                .with_logical(logical);
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
    if ctrl {
        if pressed(KeyCode::KeyZ) {
            editor.run(if shift { Act::Redo } else { Act::Undo });
        } else if pressed(KeyCode::KeyY) {
            editor.run(Act::Redo);
        } else if pressed(KeyCode::KeyS) {
            editor.run(Act::Save);
        } else if pressed(KeyCode::KeyN) {
            editor.run(Act::New);
        }
        return;
    }
    if pressed(KeyCode::Delete) {
        editor.run(Act::Delete);
    }
    if pressed(KeyCode::KeyC)
        && let Some(sel) = editor.selected.clone()
    {
        editor.start_connect(sel);
    }
    if pressed(KeyCode::Escape) {
        editor.tool = Tool::Select;
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

/// Nearest block hit by the ray.
fn pick_block(view: &EditView, ray: Ray3d) -> Option<usize> {
    let mut best: Option<(f32, usize)> = None;
    for (i, geom) in view.geoms.iter().enumerate() {
        let h = geom.half;
        let center = Vec3::from(view.positions[i]) + Vec3::Y * h[1];
        if let Some(d) = scene::ray_box(
            ray.origin,
            *ray.direction,
            center - Vec3::from(h),
            center + Vec3::from(h),
        ) && best.is_none_or(|(bd, _)| d < bd)
        {
            best = Some((d, i));
        }
    }
    best.map(|(_, i)| i)
}

/// `Editar`: select, drag, place, connect and disconnect with the left button.
pub fn mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut editor: ResMut<Editor>,
    mut lock: ResMut<CameraLock>,
    dock: Res<Dock>,
    mut contexts: EguiContexts,
) {
    let over_ui = contexts
        .ctx_mut()
        .is_ok_and(|ctx| pointer_over_ui(ctx, &dock));
    let (cam, cam_tf) = *camera;
    let cursor = window.cursor_position();
    let ray = cursor.and_then(|c| cam.viewport_to_world(cam_tf, c).ok());
    let ground = ray.and_then(ray_ground);

    editor.hover = match editor.tool {
        Tool::Place(_) if !over_ui => ground.map(|g| ground_to_model(g.x, g.y)),
        _ => None,
    };

    if buttons.just_pressed(MouseButton::Left) {
        editor.press = None;
        lock.0 = false;
        if !over_ui && let (Some(start), Some(ray)) = (cursor, ray) {
            let block = pick_block(&editor.view, ray);
            let mut press = Press {
                start,
                block,
                grab: Vec2::ZERO,
                moved: false,
            };
            if let Some(i) = block
                && editor.tool == Tool::Select
            {
                editor.selected = Some(editor.view.names[i].clone());
                lock.0 = true;
                if let Some(g) = ground {
                    let p = editor.view.positions[i];
                    press.grab = Vec2::new(p[0], p[2]) - g;
                }
            }
            editor.press = Some(press);
        }
    }

    // Dragging the pressed block along the ground, one coalesced undo step.
    if buttons.pressed(MouseButton::Left)
        && editor.tool == Tool::Select
        && let (Some(cursor), Some(g)) = (cursor, ground)
        && let Some(press) = editor.press.as_mut()
        && let Some(i) = press.block
    {
        if cursor.distance(press.start) > 4.0 {
            press.moved = true;
        }
        if press.moved {
            let at = ground_to_model(g.x + press.grab.x, g.y + press.grab.y);
            let name = editor.view.names[i].clone();
            let current = editor.doc.model().objects[i].position;
            if at[0] != current[0] || at[1] != current[1] {
                let key = format!("mover:{name}");
                let command = Command::Move {
                    name,
                    position: [at[0], at[1], current[2]],
                };
                if let Err(e) = editor.doc.apply_coalescing(command, Some(&key)) {
                    editor.status = e;
                }
                editor.refresh();
            }
        }
    }

    if buttons.just_released(MouseButton::Left) {
        lock.0 = false;
        let Some(press) = editor.press.take() else {
            return;
        };
        if press.moved {
            editor.doc.end_gesture();
            return;
        }
        let Some(cursor) = cursor.filter(|c| c.distance(press.start) <= 4.0) else {
            return;
        };
        click(&mut editor, cam, cam_tf, cursor, press.block, ground);
    }
}

/// A click (press and release without moving) in `Editar`.
fn click(
    editor: &mut Editor,
    cam: &Camera,
    cam_tf: &GlobalTransform,
    cursor: Vec2,
    block: Option<usize>,
    ground: Option<Vec2>,
) {
    match editor.tool.clone() {
        Tool::Place(kind) => {
            let Some(g) = ground else { return };
            let name = editor.doc.unique_name(name_base(kind));
            if let Some(object) = template(kind, &name, ground_to_model(g.x, g.y)) {
                editor.exec(Command::Add(Box::new(object)));
                editor.selected = Some(name);
                editor.tool = Tool::Select;
            }
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
                editor.selected = Some(from);
            }
            editor.tool = Tool::Select;
        }
        Tool::Select => {
            if block.is_some() {
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
                None => editor.selected = None,
            }
        }
    }
}

/// `Editar`: connections, grid, outlines (selection white, issues red, connection
/// source yellow) and the ghost of the block being placed.
pub fn draw_edit(editor: Res<Editor>, mut gizmos: Gizmos) {
    let view = &editor.view;
    let mut outlines: Vec<(usize, Color)> = Vec::new();
    if let Some(i) = editor.selected.as_deref().and_then(|s| view.index_of(s)) {
        outlines.push((i, Color::srgb(1.0, 1.0, 1.0)));
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
        Some(1.0),
    );
    scene::draw_network(&mut gizmos, &NetworkView::of(editor.doc.model()));
    if let (Some(at), Tool::Place(kind)) = (editor.hover, &editor.tool) {
        let (h, _) = style::extents(style::shape_of(kind));
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
                    ui.label("Nome do arquivo (.ron):");
                    let field = ui.text_edit_singleline(text);
                    if field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        save = true;
                    }
                    field.request_focus();
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

/// `Editar`: palette, properties of the selected object, connections and issues.
pub fn edit_panels(
    mut contexts: EguiContexts,
    mut editor: ResMut<Editor>,
    mut dock: ResMut<Dock>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let ed = &mut *editor;
    let (cam, cam_tf) = *camera;
    let mut acts: Vec<Act> = Vec::new();

    let mut left = dock.ui(ctx, "paleta");
    egui::Panel::left("paleta")
        .resizable(false)
        .exact_size(150.0)
        .show(&mut left, |ui| {
            ui.heading("Paleta");
            for (kind, label) in LIBRARY {
                let on = ed.tool == Tool::Place(kind);
                if ui.selectable_label(on, label).clicked() {
                    ed.tool = if on { Tool::Select } else { Tool::Place(kind) };
                }
            }
            ui.separator();
            ui.weak("Escolha um tipo e clique no chão.");
            ui.weak("Esc cancela.");
        });
    dock.free = left.available_rect_before_wrap();

    let mut right = dock.ui(ctx, "propriedades");
    egui::Panel::right("propriedades")
        .default_size(330.0)
        .size_range(260.0..=600.0)
        .show(&mut right, |ui| {
            ui.heading("Propriedades");
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    match ed.selected.clone() {
                        None => {
                            ui.label("Selecione um bloco para editar suas propriedades.");
                            ui.separator();
                            for command in run_panel(ui, &ed.doc) {
                                acts.push(Act::Exec(command));
                            }
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
                    issues_ui(ui, &ed.issues, &mut ed.selected);
                });
        });
    dock.free = right.available_rect_before_wrap();

    let labels: Vec<Label> = (0..ed.view.names.len())
        .map(|i| Label {
            text: ed.view.names[i].clone(),
            position: ed.view.positions[i],
            height: ed.view.geoms[i].label_height(0),
            selected: ed.selected.as_deref() == Some(ed.view.names[i].as_str()),
        })
        .collect();
    draw_labels(ctx, cam, cam_tf, &labels, dock.free);

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
    for act in acts {
        editor.run(act);
    }
    Ok(())
}

fn connections_ui(ui: &mut egui::Ui, doc: &Document, name: &str, acts: &mut Vec<Act>) {
    let model = doc.model();
    let outputs: Vec<&String> = model
        .connections
        .iter()
        .filter(|(a, _)| a == name)
        .map(|(_, b)| b)
        .collect();
    let inputs: Vec<&String> = model
        .connections
        .iter()
        .filter(|(_, b)| b == name)
        .map(|(a, _)| a)
        .collect();
    ui.strong("Ligações");
    if outputs.is_empty() && inputs.is_empty() {
        ui.weak("Nenhuma. Use \"Ligar\" e clique no destino.");
    }
    for other in inputs {
        ui.horizontal(|ui| {
            ui.label(format!("{other} -> {name}"));
            if ui
                .small_button("x")
                .on_hover_text("Remover ligação")
                .clicked()
            {
                acts.push(Act::Exec(Command::Disconnect {
                    from: other.clone(),
                    to: name.to_string(),
                }));
            }
        });
    }
    for other in outputs {
        ui.horizontal(|ui| {
            ui.label(format!("{name} -> {other}"));
            if ui
                .small_button("x")
                .on_hover_text("Remover ligação")
                .clicked()
            {
                acts.push(Act::Exec(Command::Disconnect {
                    from: name.to_string(),
                    to: other.clone(),
                }));
            }
        });
    }
}
