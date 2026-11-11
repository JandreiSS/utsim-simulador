//! The egui side of the Godot interface (pure: no Godot types, unit tested).
//!
//! [`Host`] owns the `egui::Context` and everything the interface around the 3D view keeps
//! between frames: the input received since the last frame, the named widgets
//! ([`Registry`]), the state of the top bar and the free central area. The Godot node
//! ([`crate::ui_class::UtsimUi`]) feeds it the input, asks it whether an event belongs to
//! egui ([`Host::push`] returns true: the 3D view must not get it), runs a frame only when
//! something changed ([`Host::needs_frame`]) and paints the output.
//!
//! An event belongs to egui when the pointer is over a panel or a pop-up of egui, when a
//! press that started over egui is still held (a drag), or, for keys, when an egui text
//! field has the focus. The wheel over a panel is egui's even when the panel cannot scroll:
//! it never zooms the camera.

use egui::{Context, Event, FullOutput, Order, Pos2, RawInput, Rect, Vec2, ViewportId};

use crate::egui_input::{self, Input};
use crate::egui_widgets::Registry;
use crate::ui_ao_vivo;
use crate::ui_bar::{self, BarMemory, BarState};
use crate::ui_bottom::{self, PlayView, StatusView};
use crate::ui_flows::{self, FlowsMemory, FlowsView};
use crate::ui_form::{self, FormMemory, FormRow};
use crate::ui_left::{self, EditView, LeftMemory, SimView};
use crate::ui_pert::{self, PertMemory, PertView};
use crate::ui_props::{self, PropAction, PropsState, PropsView};
use crate::ui_results::{self, ResultsMemory, ResultsView};

/// What one frame produced.
pub struct Frame {
    pub output: FullOutput,
    /// The actions clicked, in order (`"novo"`, `"modo:simular"`...).
    pub actions: Vec<String>,
    /// What was done in the properties panel.
    pub props: Vec<PropAction>,
}

pub struct Host {
    pub ctx: Context,
    events: Vec<Event>,
    pub reg: Registry,
    pub bar: BarState,
    bar_mem: BarMemory,
    /// The properties panel (shown when set).
    pub props: Option<PropsView>,
    pub props_state: PropsState,
    /// Whether the left panel is shown (the library in Editar, the run in Simular).
    pub left: bool,
    edit: EditView,
    sim: SimView,
    pub left_mem: LeftMemory,
    status: StatusView,
    play: PlayView,
    /// The run the "Resultados" window shows, whether it is open, and what it keeps.
    pub results: Option<ResultsView>,
    pub results_open: bool,
    pub results_mem: ResultsMemory,
    /// The PERT/CPM window: the last result, whether it is open, and what it keeps.
    pub pert: Option<PertView>,
    pub pert_open: bool,
    pub pert_mem: PertMemory,
    /// The right panel of a tool (Rede, Corredor) or of a link, in place of the properties.
    pub form: Option<Vec<FormRow>>,
    pub form_mem: FormMemory,
    /// The modal dialog shown over everything (discard changes, shortcuts), if any.
    pub dialog: Option<ui_form::Dialog>,
    /// The "Valores reais…" window of a distribution field, if open.
    pub fitwin: Option<crate::ui_fitwin::FitWin>,
    /// The window just closed by a choice, for the bridge to read the values from.
    pub fitwin_done: Option<crate::ui_fitwin::FitWin>,
    /// The token flows window (Simular).
    pub flows: Option<FlowsView>,
    pub flows_open: bool,
    pub flows_mem: FlowsMemory,
    /// The live results panels of Simular (`ui_ao_vivo`), the most recent first.
    pub vivo: Vec<crate::ui_ao_vivo::Painel>,
    /// The performance line over the 3D view (F3).
    pub perf: String,
    /// The egui panels of the last frame, in points.
    panels: Vec<Rect>,
    /// The last pointer event was over egui.
    was_over: bool,
    /// What is left for the 3D view, in points (the screen minus the egui panels).
    central: Rect,
    ppp: f32,
    size: [f32; 2],
    dirty: bool,
    next_repaint: Option<f64>,
    /// A press that started over egui is held: the drag and its release are egui's.
    pointer_owned: bool,
    /// Frames run so far (the repaint test counts them).
    pub frames: u64,
    pub mac: bool,
}

impl Default for Host {
    fn default() -> Self {
        Self::new()
    }
}

impl Host {
    pub fn new() -> Self {
        let ctx = Context::default();
        utsim_ui_tema::install(&ctx);
        Host {
            ctx,
            events: Vec::new(),
            reg: Registry::default(),
            bar: BarState::default(),
            bar_mem: BarMemory::default(),
            props: None,
            props_state: PropsState::default(),
            left: false,
            edit: EditView::default(),
            sim: SimView::default(),
            left_mem: LeftMemory::default(),
            status: StatusView::default(),
            play: PlayView::default(),
            results: None,
            results_open: false,
            results_mem: ResultsMemory::default(),
            pert: None,
            pert_open: false,
            pert_mem: PertMemory::default(),
            form: None,
            form_mem: FormMemory::default(),
            dialog: None,
            fitwin: None,
            fitwin_done: None,
            perf: String::new(),
            flows: None,
            flows_open: false,
            flows_mem: FlowsMemory::default(),
            vivo: Vec::new(),
            panels: Vec::new(),
            was_over: false,
            central: Rect::NOTHING,
            ppp: 1.0,
            size: [0.0, 0.0],
            dirty: true,
            next_repaint: None,
            pointer_owned: false,
            frames: 0,
            mac: cfg!(target_os = "macos"),
        }
    }

    /// Takes one input event; true when it belongs to egui (the caller marks it handled,
    /// so neither the Godot controls nor the 3D view get it).
    pub fn push(&mut self, input: &Input, clipboard: impl FnOnce() -> String) -> bool {
        let typing = self.ctx.text_edit_focused();
        let over = input.pos().is_some_and(|p| self.is_over(p));
        let consumed = match input {
            Input::Button { .. } if input.is_wheel() => over,
            Input::Button { pressed: true, .. } => {
                if over {
                    self.pointer_owned = true;
                }
                over
            }
            Input::Button { pressed: false, .. } => std::mem::take(&mut self.pointer_owned),
            Input::Motion { .. } => self.pointer_owned,
            Input::Key { .. } => typing,
            Input::PointerGone | Input::Focus(_) => false,
        };
        // The pointer moving over the 3D view, away from egui, changes nothing egui shows:
        // the event waits for the next frame instead of asking for one.
        let quiet = matches!(input, Input::Motion { .. }) && !over && !self.was_over && !consumed;
        if input.pos().is_some() {
            self.was_over = over;
        }
        for e in egui_input::to_egui(input, self.ppp, self.mac, clipboard) {
            // Tab is the mode switch of the main scene: egui only gets it inside a text
            // field (it would move the keyboard focus between buttons).
            if !typing
                && matches!(
                    e,
                    Event::Key {
                        key: egui::Key::Tab,
                        ..
                    }
                )
            {
                continue;
            }
            self.events.push(e);
        }
        if !quiet {
            self.dirty = true;
        }
        consumed
    }

    /// Sets what the top bar shows; a frame runs only if it changed.
    pub fn set_bar(&mut self, s: BarState) {
        if s != self.bar {
            self.bar = s;
            self.dirty = true;
        }
    }

    /// Sets what the left panel shows in Editar; a frame runs only if it changed.
    pub fn set_edit(&mut self, v: EditView) {
        self.left = true;
        if v != self.edit {
            self.edit = v;
            self.dirty = true;
        }
    }

    pub fn edit(&self) -> &EditView {
        &self.edit
    }

    /// Sets what the left panel shows in Simular.
    pub fn set_sim(&mut self, v: SimView) {
        self.left = true;
        if v != self.sim {
            self.sim = v;
            self.dirty = true;
        }
    }

    pub fn sim(&self) -> &SimView {
        &self.sim
    }

    /// Sets what the status bar shows.
    pub fn set_status(&mut self, v: StatusView) {
        if v != self.status {
            self.status = v;
            self.dirty = true;
        }
    }

    pub fn status(&self) -> &StatusView {
        &self.status
    }

    /// Sets what the playback bar shows (Simular).
    pub fn set_play(&mut self, v: PlayView) {
        if v != self.play {
            self.play = v;
            self.dirty = true;
        }
    }

    pub fn play(&self) -> &PlayView {
        &self.play
    }

    /// A new run for the "Resultados" window (`export_dir`: the folder proposed for the
    /// CSV files when none was typed).
    pub fn set_results(&mut self, v: ResultsView, export_dir: &str) {
        self.results_mem.on_new_results(&v, export_dir);
        self.results = Some(v);
        self.dirty = true;
    }

    /// Opens or closes the "Resultados" window (it opens only with a run to show).
    pub fn set_results_open(&mut self, on: bool) {
        let on = on && self.results.is_some();
        if on != self.results_open {
            self.results_open = on;
            self.dirty = true;
        }
    }

    /// Opens the PERT/CPM window for a new selection (`title`), empty until the result.
    pub fn open_pert(&mut self, title: &str) {
        self.pert = None;
        self.pert_open = true;
        self.pert_mem.title = title.to_owned();
        self.pert_mem.mode = "simulado".into();
        self.pert_mem.info.clear();
        self.results_open = false;
        self.dirty = true;
    }

    /// A computed PERT/CPM for the open window.
    pub fn set_pert(&mut self, v: PertView, default_file: &str) {
        if self.pert_mem.file.trim().is_empty() {
            self.pert_mem.file = default_file.to_owned();
        }
        self.pert_mem.info.clear();
        self.pert = Some(v);
        self.dirty = true;
    }

    /// The line over the PERT/CPM tabs while it calculates, or why it did not.
    pub fn set_pert_info(&mut self, info: &str, is_error: bool) {
        self.pert_mem.info = info.to_owned();
        self.pert_mem.info_error = is_error;
        self.dirty = true;
    }

    /// The rows of the tool panel (`None`: the properties again).
    pub fn set_form(&mut self, rows: Option<Vec<FormRow>>) {
        if let Some(r) = &rows {
            self.form_mem.sync(r);
        }
        if rows != self.form {
            self.form = rows;
            self.dirty = true;
        }
    }

    pub fn set_dialog(&mut self, d: Option<ui_form::Dialog>) {
        if d != self.dialog {
            self.dialog = d;
            self.dirty = true;
        }
    }

    pub fn set_perf(&mut self, text: &str) {
        if text != self.perf {
            self.perf = text.to_owned();
            self.dirty = true;
        }
    }

    pub fn close_pert(&mut self) {
        if self.pert_open {
            self.pert_open = false;
            self.dirty = true;
        }
    }

    /// The rectangle of the "Resultados" window, in points: the free central area of the
    /// last frame, inside a margin.
    pub fn results_rect(&self) -> Rect {
        self.central.shrink(ui_results::MARGIN)
    }

    /// Whether the live panels are on screen: some open, in Simular, with no window over
    /// the view.
    pub fn vivo_visivel(&self) -> bool {
        !self.vivo.is_empty()
            && !self.bar.editing
            && !self.pert_open
            && !(self.results_open && self.results.is_some())
    }

    /// Asks for a frame (something outside egui changed what it shows).
    pub fn invalidate(&mut self) {
        self.dirty = true;
    }

    /// Whether a frame must run now (`now` in seconds, `size` in pixels).
    pub fn needs_frame(&self, now: f64, size: [f32; 2], ppp: f32) -> bool {
        self.frames == 0
            || self.dirty
            || size != self.size
            || ppp != self.ppp
            || self.reg.has_requests()
            || self.next_repaint.is_some_and(|t| now >= t)
    }

    /// Runs one egui frame for a screen of `size` pixels.
    pub fn frame(&mut self, now: f64, size: [f32; 2], ppp: f32) -> Frame {
        let ppp = if ppp > 0.0 { ppp } else { 1.0 };
        self.ppp = ppp;
        self.size = size;
        self.dirty = false;
        let mut raw = RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(size[0] / ppp, size[1] / ppp),
            )),
            time: Some(now),
            focused: true,
            max_texture_side: Some(8192),
            events: std::mem::take(&mut self.events),
            ..RawInput::default()
        };
        raw.viewports
            .entry(ViewportId::ROOT)
            .or_default()
            .native_pixels_per_point = Some(ppp);
        let mut actions = Vec::new();
        let mut props_out = Vec::new();
        let mut central = self.central;
        let mut panels = Vec::new();
        let (reg, bar, mem) = (&mut self.reg, &self.bar, &mut self.bar_mem);
        let (props, pst) = (&self.props, &mut self.props_state);
        let (left, edit, sim, lmem) = (self.left, &self.edit, &self.sim, &mut self.left_mem);
        let (status, play) = (&self.status, &self.play);
        let (results, rmem) = (
            self.results.as_ref().filter(|_| self.results_open),
            &mut self.results_mem,
        );
        let (pert_open, pert, pmem) = (self.pert_open, self.pert.as_ref(), &mut self.pert_mem);
        let (form, fmem) = (self.form.as_ref(), &mut self.form_mem);
        let (dialog, perf) = (self.dialog.as_ref(), self.perf.as_str());
        let fitwin = &mut self.fitwin;
        let (flows, flmem) = (
            self.flows.as_ref().filter(|_| self.flows_open),
            &mut self.flows_mem,
        );
        let props_on = props.is_some();
        let vivo = &self.vivo;
        let output = self.ctx.run_ui(raw, |ui| {
            // A pass may run twice (egui discards the first when the layout changed):
            // only the last one counts.
            reg.begin_frame();
            actions.clear();
            props_out.clear();
            panels.clear();
            let screen = ui.max_rect();
            ui_bar::show(ui, bar, mem, reg, &mut actions);
            ui_bottom::show_status(ui, status, reg, &mut actions);
            let below = ui.available_rect_before_wrap();
            panels.push(Rect::from_min_max(
                screen.min,
                Pos2::new(screen.max.x, below.min.y),
            ));
            panels.push(Rect::from_min_max(
                Pos2::new(screen.min.x, below.max.y),
                screen.max,
            ));
            central = below;
            if left {
                let rect = Rect::from_min_max(
                    below.min,
                    Pos2::new(below.min.x + ui_left::WIDTH, below.max.y),
                );
                ui_left::show(ui, rect, bar.editing, edit, sim, lmem, reg, &mut actions);
                panels.push(rect);
                central.min.x = rect.max.x;
            }
            let right = Rect::from_min_max(
                Pos2::new(below.max.x - ui_props::WIDTH, below.min.y),
                Pos2::new(below.max.x, below.max.y.max(below.min.y + 40.0)),
            );
            if let (Some(rows), true) = (form, props_on) {
                ui_form::show_panel(ui, right, rows, fmem, reg, &mut actions);
                panels.push(right);
                central.max.x = right.min.x;
            } else if let Some(v) = props {
                ui_props::show(ui, right, v, pst, reg, &mut props_out);
                panels.push(right);
                central.max.x = right.min.x;
            }
            if !bar.editing && play.visible {
                let bottom = central.max.y - ui_bottom::PLAY_GAP;
                let rect = Rect::from_min_max(
                    Pos2::new(central.min.x + 10.0, bottom - ui_bottom::PLAY_HEIGHT),
                    Pos2::new((central.max.x - 10.0).max(central.min.x + 300.0), bottom),
                );
                ui_bottom::show_playback(ui, rect, play, reg, &mut actions);
                panels.push(rect);
                central.max.y = rect.min.y;
            }
            // The "Resultados" window covers the free area (the view stays framed as if it
            // were not there: it is closed to look at the scene).
            if !perf.is_empty() {
                ui.painter().text(
                    central.min + egui::vec2(10.0, 8.0),
                    egui::Align2::LEFT_TOP,
                    perf,
                    egui::FontId::proportional(utsim_ui_tema::sizes::SMALL),
                    utsim_ui_tema::colors::TEXT_WEAK,
                );
            }
            let rect = central.shrink(ui_results::MARGIN);
            if rect.width() > 200.0 && rect.height() > 150.0 {
                if pert_open {
                    // The PERT/CPM window takes the same place (one window at a time).
                    ui_pert::show(ui, rect, pert, pmem, reg, &mut actions);
                    panels.push(rect);
                } else if let Some(v) = results {
                    ui_results::show(ui, rect, v, rmem, reg, &mut actions);
                    panels.push(rect);
                }
            }
            let mut live_area = central.shrink(ui_results::MARGIN);
            if let Some(v) = flows {
                // On the right of the free area, over the view (and over a window).
                let area = central.shrink(ui_results::MARGIN);
                let rect = Rect::from_min_max(
                    Pos2::new((area.max.x - ui_flows::WIDTH).max(area.min.x), area.min.y),
                    area.max,
                );
                if rect.width() > 200.0 && rect.height() > 150.0 {
                    ui_flows::show(ui, rect, v, flmem, reg, &mut actions);
                    panels.push(rect);
                    live_area.max.x = rect.min.x - ui_ao_vivo::GAP;
                }
            }
            // The live results panels: Simular only, in the bottom-right corner of the
            // view, not over the Resultados or PERT/CPM window that covers it.
            if !bar.editing && !pert_open && results.is_none() {
                let rects = ui_ao_vivo::rects(live_area, vivo.len());
                for (k, (rect, p)) in rects.into_iter().zip(vivo).enumerate() {
                    ui_ao_vivo::show(ui, rect, k, p, reg, &mut actions);
                    panels.push(rect);
                }
            }
            if let Some(fw) = fitwin.as_mut() {
                // Over the panels, under a dialog; its backdrop takes the clicks.
                crate::ui_fitwin::show(ui.ctx(), fw, reg, &mut actions);
                panels.push(screen);
            }
            if let Some(d) = dialog {
                // Over everything; egui's modal keeps the panels under it from clicks.
                if let Some(a) = ui_form::show_dialog(ui.ctx(), d, reg) {
                    actions.push(a);
                }
            }
        });
        if actions.iter().any(|a| a == "resultados:fechar") {
            self.results_open = false;
            self.dirty = true;
        }
        if actions.iter().any(|a| a.starts_with("dialogo:")) {
            self.dialog = None;
            self.dirty = true;
        }
        if actions.iter().any(|a| {
            a == "valores:fechar" || a.starts_with("valores:usar") || a == "valores:empirica"
        }) {
            // Closed, or a distribution chosen (the action still goes to the bridge).
            if !actions.iter().any(|a| a == "valores:fechar") {
                self.fitwin_done = self.fitwin.take();
            } else {
                self.fitwin = None;
            }
            self.dirty = true;
        }
        if actions.iter().any(|a| a == "fluxos:fechar") {
            self.flows_open = false;
            self.dirty = true;
        }
        // The clicks of the live panels are the host's (close, pin).
        let before = actions.len();
        let vivo = &mut self.vivo;
        actions.retain(|a| !crate::ui_ao_vivo::acao(vivo, a));
        if actions.len() != before {
            self.dirty = true;
        }
        if actions.iter().any(|a| a == "pert:fechar") {
            self.pert_open = false;
            self.dirty = true;
        }
        self.central = central;
        self.panels = panels;
        self.frames += 1;
        // A button keeps no keyboard focus: the keys stay the main scene's shortcuts.
        if let Some(id) = self.ctx.memory(|m| m.focused())
            && !self.ctx.text_edit_focused()
        {
            self.ctx.memory_mut(|m| m.surrender_focus(id));
        }
        let delay = output
            .viewport_output
            .get(&ViewportId::ROOT)
            .map(|v| v.repaint_delay.as_secs_f64())
            .unwrap_or(f64::INFINITY);
        self.next_repaint = (delay < 1.0e6).then_some(now + delay);
        Frame {
            output,
            actions,
            props: props_out,
        }
    }

    /// Whether the screen point `px` (pixels) is over an egui panel or pop-up.
    pub fn is_over(&self, px: [f32; 2]) -> bool {
        if self.frames == 0 {
            return false;
        }
        let p = Pos2::new(px[0] / self.ppp, px[1] / self.ppp);
        if self.panels.iter().any(|r| r.contains(p)) {
            return true;
        }
        self.ctx
            .layer_id_at(p)
            .is_some_and(|l| l.order != Order::Background)
    }

    /// The free central area in pixels, `[x, y, width, height]`.
    pub fn central_pixels(&self) -> [f32; 4] {
        let r = self.central;
        if !r.is_positive() {
            return [0.0, 0.0, self.size[0], self.size[1]];
        }
        [
            r.min.x * self.ppp,
            r.min.y * self.ppp,
            r.width() * self.ppp,
            r.height() * self.ppp,
        ]
    }

    pub fn ppp(&self) -> f32 {
        self.ppp
    }

    /// Whether an egui text field has the keyboard focus.
    pub fn typing(&self) -> bool {
        self.ctx.text_edit_focused()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egui_input::Mods;

    const SIZE: [f32; 2] = [1600.0, 900.0];

    fn no_clip() -> String {
        String::new()
    }

    fn drop_frame(f: Frame) {
        f.output.drop_without_applying_deltas();
    }

    fn host() -> Host {
        let mut h = Host::new();
        h.set_bar(BarState {
            title: "modelo".into(),
            ..BarState::default()
        });
        // Two frames: the first measures the bar.
        drop_frame(h.frame(0.0, SIZE, 1.0));
        drop_frame(h.frame(0.1, SIZE, 1.0));
        h
    }

    fn button(pos: [f32; 2], button: i64, pressed: bool) -> Input {
        Input::Button {
            pos,
            button,
            pressed,
            factor: 1.0,
            mods: Mods::default(),
        }
    }

    fn center(h: &Host, name: &str) -> [f32; 2] {
        let r = h
            .reg
            .rect_pixels(name, 1.0)
            .unwrap_or_else(|| panic!("{name}"));
        [r[0] + r[2] / 2.0, r[1] + r[3] / 2.0]
    }

    #[test]
    fn the_bar_is_egui_and_the_rest_is_the_view() {
        let h = host();
        assert!(h.is_over([400.0, 20.0]));
        assert!(!h.is_over([800.0, 450.0]));
        let c = h.central_pixels();
        assert_eq!(c[0], 0.0);
        assert!(
            (c[1] - utsim_ui_tema::sizes::BAR_HEIGHT).abs() <= 1.5,
            "{c:?}"
        );
        assert_eq!(c[2], 1600.0);
    }

    #[test]
    fn the_wheel_over_the_bar_never_reaches_the_view() {
        let mut h = host();
        assert!(h.push(&button([400.0, 20.0], 5, true), no_clip));
        assert!(h.push(&button([400.0, 20.0], 5, false), no_clip));
        assert!(!h.push(&button([800.0, 450.0], 5, true), no_clip));
    }

    #[test]
    fn a_click_on_a_button_gives_its_action() {
        let mut h = host();
        let at = center(&h, "Botao_Novo");
        let mut actions = Vec::new();
        for (k, input) in [
            Input::Motion {
                pos: at,
                mods: Mods::default(),
            },
            button(at, 1, true),
            button(at, 1, false),
        ]
        .iter()
        .enumerate()
        {
            let consumed = h.push(input, no_clip);
            assert_eq!(consumed, k > 0, "passo {k}");
            let f = h.frame(1.0 + k as f64 * 0.1, SIZE, 1.0);
            actions.extend(f.actions);
            f.output.drop_without_applying_deltas();
        }
        assert_eq!(actions, vec!["novo".to_owned()]);
    }

    #[test]
    fn a_drag_from_the_view_stays_the_views() {
        let mut h = host();
        assert!(!h.push(&button([800.0, 450.0], 2, true), no_clip));
        let over_bar = Input::Motion {
            pos: [400.0, 20.0],
            mods: Mods::default(),
        };
        assert!(!h.push(&over_bar, no_clip));
        assert!(!h.push(&button([400.0, 20.0], 2, false), no_clip));
    }

    #[test]
    fn keys_go_to_the_shortcuts_unless_typing() {
        let mut h = host();
        let tab = Input::Key {
            keycode: (1 << 22) | 2,
            unicode: 0,
            pressed: true,
            echo: false,
            mods: Mods::default(),
        };
        assert!(!h.push(&tab, no_clip));
        assert!(!h.typing());
    }

    #[test]
    fn frames_run_only_when_needed() {
        let mut h = host();
        // Let the animations and tooltips settle.
        for k in 0..30 {
            if !h.needs_frame(5.0 + k as f64, SIZE, 1.0) {
                break;
            }
            drop_frame(h.frame(5.0 + k as f64, SIZE, 1.0));
        }
        assert!(!h.needs_frame(100.0, SIZE, 1.0), "sem mudança, sem quadro");
        assert!(h.needs_frame(100.0, [1280.0, 720.0], 1.0), "a janela mudou");
        h.set_bar(h.bar.clone());
        assert!(
            !h.needs_frame(100.0, SIZE, 1.0),
            "o mesmo estado não redesenha"
        );
        let mut s = h.bar.clone();
        s.can_undo = true;
        h.set_bar(s);
        assert!(h.needs_frame(100.0, SIZE, 1.0));
    }

    /// What the bridge would send to Godot for the frames of `h`, checked the way
    /// `UtsimUi` uses it: every texture set before a mesh uses it, uvs inside the texture,
    /// clips inside the viewport, indices inside the mesh, straight-alpha colours in 0..1.
    /// (Headless Godot has no renderer, so the canvas commands are checked here.)
    fn check_canvas(h: &mut Host, size: [f32; 2], frames: usize) -> (usize, usize) {
        use crate::egui_mesh::{self, Change, Textures};
        let mut textures = Textures::default();
        let (mut meshes, mut vertices) = (0, 0);
        for k in 0..frames {
            let f = h.frame(10.0 + k as f64, size, 1.0);
            let mut out = f.output;
            let mut delta = std::mem::take(&mut out.textures_delta);
            for (id, ds) in &delta.set {
                for d in ds {
                    assert_ne!(
                        textures.apply(*id, d),
                        Change::Ignored,
                        "remendo sem textura {id:?}"
                    );
                }
            }
            for img in textures.images.values() {
                assert_eq!(img.rgba.len(), img.size[0] * img.size[1] * 4);
            }
            let prims = h.ctx.tessellate(out.shapes, out.pixels_per_point);
            for prim in &prims {
                let Some(m) = egui_mesh::convert(prim, out.pixels_per_point, size) else {
                    continue;
                };
                meshes += 1;
                vertices += m.points.len();
                let t = m.texture.expect("textura");
                assert!(
                    textures.images.contains_key(&t) || matches!(t, egui::TextureId::User(_)),
                    "textura {t:?} usada antes de ser criada"
                );
                assert_eq!(m.points.len(), m.colors.len());
                assert_eq!(m.points.len(), m.uvs.len());
                assert_eq!(m.indices.len() % 3, 0);
                assert!(
                    m.indices
                        .iter()
                        .all(|&i| i >= 0 && (i as usize) < m.points.len())
                );
                for uv in &m.uvs {
                    let ok = |v: f32| (-1e-4..=1.0001).contains(&v);
                    assert!(ok(uv[0]) && ok(uv[1]), "uv {uv:?}");
                }
                for c in &m.colors {
                    assert!(c.iter().all(|x| (0.0..=1.0).contains(x)), "cor {c:?}");
                }
                let [x, y, w, hh] = m.clip;
                assert!(
                    x >= 0.0 && y >= 0.0 && w > 0.0 && hh > 0.0,
                    "recorte {:?}",
                    m.clip
                );
                assert!(
                    x + w <= size[0] + 0.5 && y + hh <= size[1] + 0.5,
                    "recorte fora {:?}",
                    m.clip
                );
                // Nothing is drawn outside the clip (cut on the CPU).
                for &i in &m.indices {
                    let p = m.points[i as usize];
                    assert!(
                        p[0] >= x - 0.01
                            && p[0] <= x + w + 0.01
                            && p[1] >= y - 0.01
                            && p[1] <= y + hh + 0.01,
                        "vértice {p:?} fora do recorte {:?}",
                        m.clip
                    );
                }
            }
            for id in &delta.free {
                textures.free(*id);
            }
            delta.clear();
        }
        (meshes, vertices)
    }

    #[test]
    fn the_canvas_commands_of_the_bar_are_consistent() {
        for size in [[1600.0, 900.0], [1280.0, 720.0]] {
            let mut h = Host::new();
            h.set_bar(BarState {
                title: "tutorial_01: um caixa".into(),
                ..BarState::default()
            });
            let (meshes, vertices) = check_canvas(&mut h, size, 3);
            assert!(
                meshes > 0 && vertices > 100,
                "{size:?}: {meshes} malhas, {vertices} vértices"
            );
            let c = h.central_pixels();
            assert!(c[1] > 40.0 && c[1] < 50.0, "{c:?}");
        }
    }

    #[test]
    fn the_canvas_commands_with_the_properties_panel_are_consistent() {
        let mut e = crate::document::Editor::new("teste");
        let name = e.add("Processor", [0.0, 0.0, 0.0]).unwrap();
        for size in [[1600.0, 900.0], [1280.0, 720.0]] {
            let mut h = Host::new();
            h.props = Some(crate::ui_props::PropsView {
                props: e.properties(&name),
                families: crate::schema::FAMILIES
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                data_paths: Vec::new(),
                extra: String::new(),
                driver: -1,
                miniature: true,
                data: std::collections::HashMap::new(),
                data_families: Vec::new(),
                base_dir: None,
                abas: e.tab_fields(&name),
                somente_leitura: None,
                simulado: None,
            });
            let (meshes, _) = check_canvas(&mut h, size, 3);
            assert!(meshes > 0);
            let c = h.central_pixels();
            assert!(
                (c[0] + c[2] - (size[0] - crate::ui_props::WIDTH)).abs() < 1.0,
                "{c:?}"
            );
            // The panel and the status bar under it are egui's.
            assert!(h.is_over([size[0] - 100.0, 300.0]));
            assert!(h.is_over([size[0] - 100.0, size[1] - 10.0]));
            assert!(!h.is_over([size[0] / 3.0, 400.0]));
            // The panel stops above the status bar.
            let panel = h.reg.get("PainelPropriedades").unwrap().rect;
            assert!(
                (panel.max.y - (size[1] - crate::ui_bottom::STATUS_HEIGHT)).abs() < 1.5,
                "{panel:?}"
            );
        }
    }

    #[test]
    fn the_canvas_commands_with_the_arrival_editors_are_consistent() {
        use crate::schema::Value;
        let mut e = crate::document::Editor::new("teste");
        let name = e.add("Source", [0.0, 0.0, 0.0]).unwrap();
        // Both at once is a model error, but both editors are drawn.
        for path in ["table.on", "schedule.on"] {
            e.set_property(&name, path, &Value::Bool(true), &[])
                .unwrap();
        }
        e.set_property(
            &name,
            "table",
            &Value::Text(
                "(mode: Times, repeat: Cycle(1440.0), rows: [(at: 480.0, count: 12, \
                 labels: {\"prioridade\": 2.0}), (at: 600.0, count: 5)])"
                    .into(),
            ),
            &[],
        )
        .unwrap();
        for size in [[1600.0, 900.0], [1280.0, 720.0]] {
            let mut h = Host::new();
            h.props = Some(crate::ui_props::PropsView {
                props: e.properties(&name),
                families: crate::schema::FAMILIES
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                data_paths: Vec::new(),
                extra: String::new(),
                driver: -1,
                miniature: false,
                data: std::collections::HashMap::new(),
                data_families: Vec::new(),
                base_dir: None,
                abas: e.tab_fields(&name),
                somente_leitura: None,
                simulado: None,
            });
            let (meshes, _) = check_canvas(&mut h, size, 3);
            assert!(meshes > 0);
            for w in [
                "Prop_table_modo",
                "Prop_table_repetir",
                "Prop_table_ciclo",
                "Prop_table_linha_1_at",
                "Prop_table_linha_2_count",
                "Prop_table_linha_2_tipo",
                "Prop_table_linha_1_rotulos",
                "Prop_table_linha_2_remover",
                "Prop_table_mais",
                "Prop_table_colar",
                "Prop_table_ordenar",
                "Prop_table_total",
                "Prop_schedule_ciclo",
                "Prop_schedule_faixa",
                "Prop_schedule_periodo_1_inicio",
                "Prop_schedule_periodo_2_modo",
                "Prop_schedule_periodo_2_valor",
                "Prop_schedule_mais",
                "Prop_schedule_grafico",
                "Prop_schedule_total",
            ] {
                assert!(h.reg.knows(w), "{size:?}: {w} ausente");
            }
            assert!(
                h.reg
                    .read("Prop_table_total")
                    .is_some_and(|w| w.text.starts_with("17 itens")),
                "{:?}",
                h.reg.read("Prop_table_total")
            );
        }
    }

    fn edit_view() -> crate::ui_left::EditView {
        use crate::ui_left::{Entry, ObjectItem};
        crate::ui_left::EditView {
            library: crate::document::palette()
                .iter()
                .map(|p| Entry {
                    id: p.id.into(),
                    label: p.label.into(),
                    category: p.category.into(),
                    icon: Some(egui::TextureId::User(100)),
                })
                .collect(),
            categories: utsim_edit::CATEGORIES
                .iter()
                .map(|c| c.to_string())
                .collect(),
            armed: "Queue".into(),
            objects: (0..40)
                .map(|k| ObjectItem {
                    name: format!("objeto_com_um_nome_comprido_{k}"),
                    kind_label: "Processo".into(),
                    problems: k % 3,
                    warnings: k % 2,
                })
                .collect(),
            selected: vec!["objeto_com_um_nome_comprido_1".into()],
            obstacles: true,
        }
    }

    fn sim_view() -> crate::ui_left::SimView {
        crate::ui_left::SimView {
            status: "5 replicações concluídas. Reproduzindo a replicação 0.".into(),
            blocks: (0..30).map(|k| format!("bloco_{k} (Processo)")).collect(),
            follow: Some(1),
            summary: crate::ui_left::Summary {
                title: "Resultados (5 replicações)".into(),
                bottleneck: "processo".into(),
                lines: (0..12).map(|k| format!("bloco_{k}: 120 saídas")).collect(),
            },
            ..crate::ui_left::SimView::default()
        }
    }

    #[test]
    fn the_canvas_commands_with_the_left_panel_and_the_status_are_consistent() {
        for size in [[1600.0, 900.0], [1280.0, 720.0]] {
            let mut h = Host::new();
            h.set_edit(edit_view());
            h.set_status(crate::ui_bottom::StatusView {
                text: "Arraste um objeto da Biblioteca para o chão, ou abra um modelo (Ctrl+O). "
                    .repeat(4),
                progress: Some(0.3),
                percent: true,
            });
            let (meshes, vertices) = check_canvas(&mut h, size, 3);
            assert!(meshes > 0 && vertices > 100, "{size:?}");
            let c = h.central_pixels();
            assert!((c[0] - crate::ui_left::WIDTH).abs() < 1.0, "{c:?}");
            assert!(
                (c[1] + c[3] - (size[1] - crate::ui_bottom::STATUS_HEIGHT)).abs() < 1.5,
                "{c:?}"
            );
            assert!(h.is_over([100.0, 300.0]), "a biblioteca é do egui");
            for name in [
                "Biblioteca_Source",
                "Objetos_Lista",
                "Ver_Obstaculos",
                "Status",
            ] {
                assert!(h.reg.get(name).is_some(), "{name} {size:?}");
            }
            // The status line is cut to its place, never under the progress bar.
            let status = h.reg.get("Status").unwrap().rect;
            let progress = h.reg.get("Progresso").unwrap().rect;
            assert!(
                status.max.x <= progress.min.x + 0.5,
                "{status:?} {progress:?}"
            );
        }
    }

    #[test]
    fn the_canvas_commands_of_simular_are_consistent() {
        for size in [[1600.0, 900.0], [1280.0, 720.0]] {
            let mut h = Host::new();
            h.set_bar(BarState {
                editing: false,
                ..BarState::default()
            });
            h.set_sim(sim_view());
            h.set_play(crate::ui_bottom::PlayView {
                visible: true,
                playing: true,
                t: 12.5,
                end: 480.0,
                speed: 2.0,
            });
            let (meshes, _) = check_canvas(&mut h, size, 3);
            assert!(meshes > 0);
            for name in [
                "Simular_Replicacoes",
                "Botao_RodarDeNovo",
                "Simular_Camera",
                "Simular_Resumo",
                "Reproducao_Play",
                "Reproducao_Tempo",
                "Reproducao_T",
            ] {
                assert!(h.reg.get(name).is_some(), "{name} {size:?}");
            }
            let bar = h.reg.get("BarraReproducao").unwrap().rect;
            let c = h.central_pixels();
            assert!(bar.min.x >= c[0] && bar.max.x <= size[0], "{bar:?}");
            assert!(
                bar.max.y <= size[1] - crate::ui_bottom::STATUS_HEIGHT,
                "{bar:?}"
            );
            assert_eq!(h.reg.get("Reproducao_Play").unwrap().text, "Pausar");
            assert!(h.is_over([bar.center().x, bar.center().y]));
        }
    }

    /// The live panel of Simular: in the bottom-right corner above the playback bar, its
    /// lines named, and its X and "Fixar" handled by the host (not sent to the bridge).
    #[test]
    fn the_live_panel_sits_in_the_corner_and_closes_with_its_x() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/inspecao_e_retrabalho.ron");
        let model = utsim_model::Model::load(&path).expect("model");
        let replay = model.replay(0).expect("replay");
        let vivo = crate::ao_vivo::AoVivo::new(&replay, Some(&model), |_| None);
        let mut h = Host::new();
        h.set_bar(BarState {
            editing: false,
            ..BarState::default()
        });
        h.left = true;
        h.set_sim(sim_view());
        h.set_play(crate::ui_bottom::PlayView {
            visible: true,
            playing: false,
            t: 2400.0,
            end: 4800.0,
            speed: 1.0,
        });
        crate::ui_ao_vivo::abrir(&mut h.vivo, "inspecao");
        h.vivo[0].leitura = vivo.leitura("inspecao", 2400.0);
        for k in 0..3 {
            drop_frame(h.frame(k as f64 * 0.1, SIZE, 1.0));
        }
        for name in [
            "PainelVivo",
            "PainelVivo_Titulo",
            "PainelVivo_Fechar",
            "PainelVivo_Fixar",
            "PainelVivo_Processados",
            "PainelVivo_Fila",
            "PainelVivo_Refugo",
            "PainelVivo_Ocupado",
        ] {
            let w = h.reg.get(name).unwrap_or_else(|| panic!("{name}"));
            assert!(w.rect.is_positive(), "{name} {:?}", w.rect);
        }
        let n = vivo.numeros("inspecao", 2400.0).unwrap();
        assert_eq!(
            h.reg.get("PainelVivo_Processados").unwrap().text,
            n.saidas.to_string()
        );
        assert_eq!(
            h.reg.get("PainelVivo_Titulo").unwrap().text,
            "inspecao (processo)"
        );
        let panel = h.reg.get("PainelVivo").unwrap().rect;
        let bar = h.reg.get("BarraReproducao").unwrap().rect;
        let left = h.reg.get("Simular_Blocos").map(|w| w.rect);
        assert!(panel.max.y <= bar.min.y, "{panel:?} {bar:?}");
        assert!(
            (panel.max.x - (SIZE[0] - ui_results::MARGIN)).abs() < 1.0,
            "{panel:?}"
        );
        if let Some(l) = left {
            assert!(!panel.intersects(l), "{panel:?} {l:?}");
        }
        assert!(h.is_over([panel.center().x, panel.center().y]));
        // Pin, then close: the host does it, nothing goes to the bridge.
        let mut now = 1.0;
        for (name, fixed) in [("PainelVivo_Fixar", true), ("PainelVivo_Fechar", false)] {
            let at = center(&h, name);
            let mut actions = Vec::new();
            for input in &[
                Input::Motion {
                    pos: at,
                    mods: Mods::default(),
                },
                button(at, 1, true),
                button(at, 1, false),
            ] {
                h.push(input, no_clip);
                now += 0.1;
                let f = h.frame(now, SIZE, 1.0);
                actions.extend(f.actions);
                f.output.drop_without_applying_deltas();
            }
            assert!(actions.is_empty(), "{actions:?}");
            if fixed {
                assert!(h.vivo[0].fixo);
            } else {
                assert!(h.vivo.is_empty());
            }
        }
    }

    /// In Simular, with the panel's field and the playback bar on screen, a click outside
    /// egui and the keys typed after it stay Godot's (a field of a Godot panel gets them).
    #[test]
    fn keys_typed_in_a_godot_field_stay_godots_in_simular() {
        let mut h = Host::new();
        h.set_bar(BarState {
            editing: false,
            ..BarState::default()
        });
        h.set_sim(crate::ui_left::SimView::default());
        h.set_play(crate::ui_bottom::PlayView {
            visible: true,
            end: 100.0,
            ..Default::default()
        });
        for k in 0..3 {
            drop_frame(h.frame(k as f64, SIZE, 1.0));
        }
        assert!(!h.push(&button([900.0, 300.0], 1, true), no_clip));
        drop_frame(h.frame(4.0, SIZE, 1.0));
        assert!(!h.push(&button([900.0, 300.0], 1, false), no_clip));
        drop_frame(h.frame(5.0, SIZE, 1.0));
        for (code, uni) in [(0i64, 97u32), ((1 << 22) | 5, 0)] {
            let key = Input::Key {
                keycode: code,
                unicode: uni,
                pressed: true,
                echo: false,
                mods: Mods::default(),
            };
            assert!(!h.push(&key, no_clip), "tecla {code} consumida");
            drop_frame(h.frame(6.0, SIZE, 1.0));
        }
    }

    #[test]
    fn the_scenarios_area_takes_new_lines() {
        let mut h = host();
        h.set_results(crate::ui_results::tests::view("servidores.ron", 2), "");
        h.set_results_open(true);
        h.results_mem.tab = 3;
        drop_frame(h.frame(1.0, SIZE, 1.0));
        drop_frame(h.frame(1.1, SIZE, 1.0));
        let at = center(&h, "Res_CenariosTexto");
        h.push(&button(at, 1, true), no_clip);
        drop_frame(h.frame(1.2, SIZE, 1.0));
        h.push(&button(at, 1, false), no_clip);
        drop_frame(h.frame(1.3, SIZE, 1.0));
        let mut t = 1.4;
        for ch in "[a]\nb".chars() {
            let (code, unicode) = if ch == '\n' {
                (crate::egui_input::SPECIAL | 5, 0)
            } else {
                (ch as i64, ch as u32)
            };
            for pressed in [true, false] {
                h.push(
                    &Input::Key {
                        keycode: code,
                        unicode: if pressed { unicode } else { 0 },
                        pressed,
                        echo: false,
                        mods: Mods::default(),
                    },
                    no_clip,
                );
                drop_frame(h.frame(t, SIZE, 1.0));
                t += 0.1;
            }
        }
        assert_eq!(h.results_mem.sc_text, "[a]\nb");
    }
}
