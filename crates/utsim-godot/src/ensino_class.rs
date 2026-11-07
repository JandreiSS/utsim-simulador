//! `UtsimEnsino`: the mission mode of UTSim Ensino inside the 3D client
//! (`docs/plano_ensino_e_jogo.md`, 5.4, decision E1). A `Node2D` of its own scene that
//! hosts the same `utsim_ensino_ui::App` as the eframe window, on an [`EguiCanvas`]: the
//! campaign, the missions, the grade and the class run with the logic of
//! `utsim-ensino-core`, so a mission gets the same grade in both hosts.
//!
//! The node takes every input while it is shown (the mode covers the window). A button
//! at the top goes back to the 3D client through the `sair` signal.

use std::time::Instant;

use egui::{Pos2, RawInput, Rect, Vec2, ViewportId};
use godot::classes::notify::CanvasItemNotification;
use godot::classes::{INode2D, InputEvent, Node2D};
use godot::prelude::*;
use utsim_ensino_store::perfil::Perfil;
use utsim_ensino_ui::App;

use crate::egui_canvas::{EguiCanvas, clipboard_get, read_input, screen_of};
use crate::egui_input::{Input, to_egui};

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct UtsimEnsino {
    ctx: egui::Context,
    canvas: EguiCanvas,
    app: Option<App>,
    erro: String,
    events: Vec<egui::Event>,
    start: Instant,
    ppp: f32,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for UtsimEnsino {
    fn init(base: Base<Node2D>) -> Self {
        let (app, erro) = match Perfil::padrao() {
            Some(p) => (Some(App::new(p)), String::new()),
            None => (None, "não encontrei a pasta do usuário".to_string()),
        };
        UtsimEnsino {
            ctx: egui::Context::default(),
            canvas: EguiCanvas::new(),
            app,
            erro,
            events: Vec::new(),
            start: Instant::now(),
            ppp: 1.0,
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_z_index(100);
        self.base_mut().set_process(true);
        self.base_mut().set_process_input(true);
    }

    fn process(&mut self, _delta: f64) {
        let Some(vp) = self.base().get_viewport() else {
            return;
        };
        let (size, ppp) = screen_of(&vp);
        self.ppp = ppp;
        let mut raw = RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(size[0] / ppp, size[1] / ppp),
            )),
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
        let mut sair = false;
        let (app, erro) = (&mut self.app, &self.erro);
        let out = self.ctx.run_ui(raw, |ui| {
            egui::Panel::top("ensino_volta").show(ui, |ui| {
                if ui.button("← Voltar ao UTSim 3D").clicked() {
                    sair = true;
                }
            });
            match app {
                Some(a) => a.draw(ui),
                None => {
                    ui.label(erro.as_str());
                }
            }
        });
        let item = self.base().get_canvas_item();
        self.canvas.present(&self.ctx, out, item, size, |_| None);
        if sair {
            self.base_mut()
                .call_deferred("emit_signal", &[StringName::from("sair").to_variant()]);
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
impl UtsimEnsino {
    /// The button at the top: back to the 3D client.
    #[signal]
    fn sair();

    /// Opens a mission of the open package by id (`ato4/ventilador`), as `--missao` does in
    /// the window. False when there is no identity yet or the mission does not exist.
    #[func]
    fn abrir_missao(&mut self, id: GString) -> bool {
        let Some(app) = self.app.as_mut() else {
            return false;
        };
        let id = id.to_string();
        match app.pacote.as_ref().and_then(|p| p.indice(&id)) {
            Some(i) if app.config.identidade.is_some() => {
                app.abrir_missao(i);
                true
            }
            _ => false,
        }
    }

    /// Opens a mission package (folder or `.utse`) in place of the campaign; the error
    /// text, empty when it opened.
    #[func]
    fn abrir_pacote(&mut self, caminho: GString) -> GString {
        let Some(app) = self.app.as_mut() else {
            return GString::from("sem perfil");
        };
        match app.abrir_pacote(std::path::Path::new(&caminho.to_string())) {
            Ok(()) => GString::new(),
            Err(e) => GString::from(e.as_str()),
        }
    }
}
