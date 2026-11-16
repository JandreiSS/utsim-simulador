//! `UtsimPainelVivo`: a live results panel (`ui_ao_vivo`) in a window of its own, that can
//! go to another monitor. The main scene puts it in a Godot `Window` that is not embedded
//! (a window of the operating system) when the user clicks "Janela" on a panel. It draws
//! the panel of its object with its own egui context, from the numbers the main interface
//! keeps updating ([`UtsimUi`]): several can be open at once and all stay live.
//!
//! "Voltar" puts the panel back over the 3D view and × closes it; the node then emits
//! `fechada` and the main scene closes the window. It also emits `fechada` when the panel
//! is gone (another model, Editar).

use std::time::Instant;

use egui::{Pos2, RawInput, Rect, Vec2, ViewportId};
use godot::classes::notify::CanvasItemNotification;
use godot::classes::{INode2D, InputEvent, Node2D};
use godot::prelude::*;

use crate::egui_canvas::{EguiCanvas, clipboard_get, read_input, screen_of};
use crate::egui_input::{Input, to_egui};
use crate::egui_widgets::Registry;
use crate::ui_class::UtsimUi;

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct UtsimPainelVivo {
    ctx: egui::Context,
    canvas: EguiCanvas,
    reg: Registry,
    events: Vec<egui::Event>,
    start: Instant,
    ppp: f32,
    nome: String,
    ui: Option<Gd<UtsimUi>>,
    /// `fechada` was emitted (once).
    fechou: bool,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for UtsimPainelVivo {
    fn init(base: Base<Node2D>) -> Self {
        let ctx = egui::Context::default();
        utsim_ui_tema::install(&ctx);
        UtsimPainelVivo {
            ctx,
            canvas: EguiCanvas::new(),
            reg: Registry::default(),
            events: Vec::new(),
            start: Instant::now(),
            ppp: 1.0,
            nome: String::new(),
            ui: None,
            fechou: false,
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_process(true);
        self.base_mut().set_process_input(true);
    }

    fn process(&mut self, _delta: f64) {
        let Some(ui) = self.ui.clone() else {
            return;
        };
        let Some(painel) = ui.bind().painel_solto(&self.nome) else {
            self.fechar();
            return;
        };
        let Some(vp) = self.base().get_viewport() else {
            return;
        };
        let (size, ppp) = screen_of(&vp);
        self.ppp = ppp;
        let screen = Rect::from_min_size(Pos2::ZERO, Vec2::new(size[0] / ppp, size[1] / ppp));
        let mut raw = RawInput {
            screen_rect: Some(screen),
            time: Some(self.start.elapsed().as_secs_f64()),
            focused: true,
            max_texture_side: Some(8192),
            events: std::mem::take(&mut self.events),
            ..RawInput::default()
        };
        raw.viewports
            .entry(ViewportId::ROOT)
            .or_default()
            .native_pixels_per_point = Some(ppp);
        let mut out = Vec::new();
        let reg = &mut self.reg;
        let output = self.ctx.run_ui(raw, |ui| {
            reg.begin_frame();
            out.clear();
            crate::ui_ao_vivo::show(ui, screen, 0, &painel, reg, &mut out);
        });
        let item = self.base().get_canvas_item();
        self.canvas.present(&self.ctx, output, item, size, |_| None);
        let mut ui = ui;
        for a in out {
            match a.as_str() {
                "vivo:fechar:0" => {
                    ui.bind_mut().vivo_fechar(&self.nome);
                    self.fechar();
                }
                "vivo:soltar:0" => {
                    ui.bind_mut().vivo_prender(&self.nome);
                    self.fechar();
                }
                // Pinning means nothing here: a panel in its own window stays.
                _ => {}
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Some(input) = read_input(&event) else {
            return;
        };
        let mac = cfg!(target_os = "macos");
        self.events
            .extend(to_egui(&input, self.ppp, mac, clipboard_get));
        if let Some(mut vp) = self.base().get_viewport() {
            vp.set_input_as_handled();
        }
    }

    fn on_notification(&mut self, what: CanvasItemNotification) {
        let ev = match what {
            CanvasItemNotification::WM_MOUSE_EXIT => Some(Input::PointerGone),
            CanvasItemNotification::WM_WINDOW_FOCUS_OUT => Some(Input::Focus(false)),
            CanvasItemNotification::WM_WINDOW_FOCUS_IN => Some(Input::Focus(true)),
            CanvasItemNotification::PREDELETE => {
                self.canvas.free_all();
                None
            }
            _ => None,
        };
        if let Some(i) = ev {
            let mac = cfg!(target_os = "macos");
            self.events.extend(to_egui(&i, self.ppp, mac, String::new));
        }
    }
}

#[godot_api]
impl UtsimPainelVivo {
    /// The panel is gone (closed, back over the view, or no longer open): the main scene
    /// closes the window.
    #[signal]
    fn fechada();

    /// The main interface whose panel of `nome` this window shows.
    #[func]
    fn setup(&mut self, ui: Gd<UtsimUi>, nome: GString) {
        self.ui = Some(ui);
        self.nome = nome.to_string();
        self.fechou = false;
    }

    /// The object whose panel it shows.
    #[func]
    fn nome(&self) -> GString {
        GString::from(self.nome.as_str())
    }

    /// The text of the widget `name` of the window's panel (`PainelVivo_Titulo`...), for
    /// the scripted tests; empty when it is not there.
    #[func]
    fn widget_text(&self, name: GString) -> GString {
        self.reg
            .get(&name.to_string())
            .map_or_else(GString::new, |w| GString::from(w.text.as_str()))
    }
}

impl UtsimPainelVivo {
    fn fechar(&mut self) {
        if self.fechou {
            return;
        }
        self.fechou = true;
        self.base_mut()
            .call_deferred("emit_signal", &[StringName::from("fechada").to_variant()]);
    }
}
