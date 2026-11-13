//! The status bar and the playback bar of the Godot interface, in egui (pure: no Godot
//! types, unit tested).
//!
//! The status bar runs along the bottom of the window: the status line (cut with "…" and
//! the whole text in the tooltip when it does not fit), and, while a job runs on the
//! bridge's thread, its progress and "Cancelar". The playback bar (Simular) floats over the
//! 3D view just above it: ◀ evento, Reproduzir/Pausar, evento ▶, the speed, the timeline
//! and `t = … / …`.
//!
//! Actions: `cancelar`, `evento:-1`, `evento:1`, `reproduzir`, `velocidade:2`,
//! `tempo:<s>`. Names: `Status`, `Progresso`, `Botao_Cancelar`, `Reproducao_Anterior`,
//! `Reproducao_Play`, `Reproducao_Proximo`, `Reproducao_Velocidade` (as the Godot nodes),
//! `Reproducao_Tempo` (the timeline) and `Reproducao_T` (the time).

use egui::{Align, Layout, Rect, RichText, Sense, Stroke, Ui, UiBuilder, Vec2};
use utsim_ui_tema::{colors, sizes};

use crate::egui_widgets::Registry;
use crate::ui_props::{named, named_list};

/// Height of the status bar, in points (the Godot bar had the same).
pub const STATUS_HEIGHT: f32 = 40.0;
/// The playback bar: its height and its gap to the status bar.
pub const PLAY_HEIGHT: f32 = 46.0;
pub const PLAY_GAP: f32 = 6.0;
/// The playback speeds.
pub const SPEEDS: [f64; 9] = [0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0];

/// What the status bar shows.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StatusView {
    pub text: String,
    /// A job is running: its progress in 0..1 (`None`: no job).
    pub progress: Option<f32>,
    /// Whether the progress is known (shown with its percentage) or just moves.
    pub percent: bool,
}

/// What the playback bar shows (Simular).
#[derive(Clone, Debug, PartialEq)]
pub struct PlayView {
    pub visible: bool,
    pub playing: bool,
    pub t: f64,
    pub end: f64,
    pub speed: f64,
}

impl Default for PlayView {
    fn default() -> Self {
        PlayView {
            visible: false,
            playing: false,
            t: 0.0,
            end: 0.0,
            speed: 1.0,
        }
    }
}

/// A speed without trailing zeros (`0.25x`, `1x`).
pub fn speed_label(v: f64) -> String {
    let s = format!("{v}");
    format!("{}x", s.strip_suffix(".0").unwrap_or(&s))
}

/// Draws the status bar at the bottom of `ui` (a panel: what remains is above it).
pub fn show_status(ui: &mut Ui, v: &StatusView, reg: &mut Registry, out: &mut Vec<String>) {
    let frame = egui::Frame::new()
        .fill(colors::PANEL)
        .inner_margin(egui::Margin::symmetric(12, 0))
        .stroke(Stroke::NONE);
    egui::Panel::bottom("utsim_barra_status")
        .exact_size(STATUS_HEIGHT)
        .resizable(false)
        .show_separator_line(true)
        .frame(frame)
        .show(ui, |ui| {
            let bar = ui.max_rect();
            let r = ui.allocate_rect(bar, Sense::hover());
            named(ui, reg, "BarraInferior", &r, "");
            let mut child = ui.new_child(
                UiBuilder::new()
                    .max_rect(bar)
                    .layout(Layout::right_to_left(Align::Center)),
            );
            // Configurações, at the right end (the top bar has no room left).
            let label = format!("\u{2699} {}", crate::texto::tx("barra-configuracoes"));
            let r = child
                .add(egui::Button::new(RichText::new(&label).small()).frame(false))
                .on_hover_text(crate::texto::tx("barra-configuracoes-dica"));
            named(&child, reg, "Botao_Configuracoes", &r, &label);
            if r.clicked() {
                out.push("configuracoes".into());
            }
            if let Some(p) = v.progress {
                let r = child
                    .button("Cancelar")
                    .on_hover_text("Para na próxima replicação");
                named(&child, reg, "Botao_Cancelar", &r, "Cancelar");
                if r.clicked() {
                    out.push("cancelar".into());
                }
                let mut bar = egui::ProgressBar::new(p.clamp(0.0, 1.0)).desired_width(220.0);
                if v.percent {
                    bar = bar.show_percentage();
                } else {
                    bar = bar.animate(true);
                }
                let r = child.add(bar);
                named(&child, reg, "Progresso", &r, &format!("{:.0}%", p * 100.0));
            }
            child.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let r = ui
                    .add(
                        egui::Label::new(RichText::new(&v.text).color(colors::TEXT))
                            .truncate()
                            .sense(Sense::hover()),
                    )
                    .on_hover_text(&v.text);
                named(ui, reg, "Status", &r, &v.text);
            });
        });
}

/// Draws the playback bar in `rect` (points).
pub fn show_playback(
    ui: &mut Ui,
    rect: Rect,
    v: &PlayView,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    ui.painter().rect(
        rect,
        sizes::RADIUS as f32,
        colors::PANEL,
        Stroke::new(1.0, colors::BORDER),
        egui::StrokeKind::Inside,
    );
    let r = ui.allocate_rect(rect, Sense::hover());
    named(ui, reg, "BarraReproducao", &r, "");
    let inner = rect.shrink2(Vec2::new(8.0, 6.0));
    let mut ui = ui.new_child(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::left_to_right(Align::Center)),
    );
    ui.spacing_mut().item_spacing.x = 6.0;
    let mut button = |ui: &mut Ui, name: &str, text: &str, tip: &str, w: f32| -> bool {
        let r = ui
            .add(egui::Button::new(text).min_size(Vec2::new(w, 30.0)))
            .on_hover_text(tip);
        named(ui, reg, name, &r, text);
        r.clicked()
    };
    if button(
        &mut ui,
        "Reproducao_Anterior",
        "◀ evento",
        "Evento anterior",
        0.0,
    ) {
        out.push("evento:-1".into());
    }
    let play = if v.playing { "Pausar" } else { "Reproduzir" };
    if button(
        &mut ui,
        "Reproducao_Play",
        play,
        "Reproduzir ou pausar (espaço)",
        110.0,
    ) {
        out.push("reproduzir".into());
    }
    if button(
        &mut ui,
        "Reproducao_Proximo",
        "evento ▶",
        "Próximo evento",
        0.0,
    ) {
        out.push("evento:1".into());
    }
    let shown = speed_label(v.speed);
    let mut pick = reg
        .take_request("Reproducao_Velocidade")
        .and_then(|item| SPEEDS.iter().copied().find(|s| speed_label(*s) == item));
    let r = egui::ComboBox::from_id_salt("utsim_reproducao_velocidade")
        .selected_text(&shown)
        .width(72.0)
        .show_ui(&mut ui, |ui| {
            for s in SPEEDS {
                if ui
                    .selectable_label((s - v.speed).abs() < 1e-9, speed_label(s))
                    .clicked()
                {
                    pick = Some(s);
                }
            }
        });
    let resp = r.response.on_hover_text("Velocidade da reprodução");
    named_list(
        &ui,
        reg,
        "Reproducao_Velocidade",
        &resp,
        &shown,
        SPEEDS.iter().map(|s| speed_label(*s)).collect(),
    );
    if let Some(s) = pick {
        out.push(format!("velocidade:{s}"));
    }
    let time = format!("t = {:.2} / {:.2}", v.t, v.end);
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        let r = ui.add_sized(
            [170.0, 30.0],
            egui::Label::new(RichText::new(&time).color(colors::TEXT)),
        );
        named(ui, reg, "Reproducao_T", &r, &time);
        let mut t = v.t;
        ui.spacing_mut().slider_width = (ui.available_width() - 8.0).max(60.0);
        let r = ui.add(
            egui::Slider::new(&mut t, 0.0..=v.end.max(1e-9))
                .show_value(false)
                .trailing_fill(true),
        );
        named(ui, reg, "Reproducao_Tempo", &r, &format!("{:.2}", v.t));
        if r.changed() {
            out.push(format!("tempo:{t}"));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Context, Event, Modifiers, PointerButton, Pos2, RawInput};

    fn run(
        ctx: &Context,
        reg: &mut Registry,
        s: &StatusView,
        p: &PlayView,
        events: Vec<Event>,
        _t: f64,
    ) -> Vec<String> {
        // egui wants the time never to go back: one clock for every frame of a test.
        thread_local!(static CLOCK: std::cell::Cell<f64> = const { std::cell::Cell::new(0.0) });
        let t = CLOCK.with(|c| {
            c.set(c.get() + 0.05);
            c.get()
        });
        let mut out = Vec::new();
        let input = RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 720.0))),
            time: Some(t),
            events,
            ..RawInput::default()
        };
        ctx.run_ui(input, |ui| {
            reg.begin_frame();
            out.clear();
            show_status(ui, s, reg, &mut out);
            let below = ui.available_rect_before_wrap();
            let rect = Rect::from_min_max(
                Pos2::new(300.0, below.max.y - PLAY_GAP - PLAY_HEIGHT),
                Pos2::new(1270.0, below.max.y - PLAY_GAP),
            );
            show_playback(ui, rect, p, reg, &mut out);
        })
        .drop_without_applying_deltas();
        out
    }

    fn click(
        ctx: &Context,
        reg: &mut Registry,
        s: &StatusView,
        p: &PlayView,
        name: &str,
    ) -> Vec<String> {
        let at = reg
            .get(name)
            .unwrap_or_else(|| panic!("{name}"))
            .rect
            .center();
        let mut out = Vec::new();
        for (k, ev) in [
            Event::PointerMoved(at),
            Event::PointerButton {
                pos: at,
                button: PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::NONE,
            },
            Event::PointerButton {
                pos: at,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::NONE,
            },
        ]
        .into_iter()
        .enumerate()
        {
            out.extend(run(ctx, reg, s, p, vec![ev], 1.0 + k as f64 * 0.05));
        }
        out
    }

    #[test]
    fn the_status_bar_shows_the_job_and_cancels_it() {
        let ctx = Context::default();
        utsim_ui_tema::install(&ctx);
        let mut reg = Registry::default();
        let mut s = StatusView {
            text: "Arraste um objeto da Biblioteca para o chão.".into(),
            ..StatusView::default()
        };
        let p = PlayView::default();
        run(&ctx, &mut reg, &s, &p, vec![], 0.0);
        assert!(reg.get("Status").unwrap().text.contains("Biblioteca"));
        assert!(reg.get("Progresso").is_none() && reg.get("Botao_Cancelar").is_none());
        let status = reg.get("Status").unwrap().rect;
        assert!(status.max.y <= 720.5 && status.min.y >= 720.0 - STATUS_HEIGHT - 0.5);
        s.progress = Some(0.4);
        s.percent = true;
        run(&ctx, &mut reg, &s, &p, vec![], 0.5);
        assert!(reg.get("Progresso").is_some());
        assert_eq!(
            click(&ctx, &mut reg, &s, &p, "Botao_Cancelar"),
            vec!["cancelar".to_owned()]
        );
    }

    #[test]
    fn the_playback_bar_plays_steps_and_seeks() {
        let ctx = Context::default();
        utsim_ui_tema::install(&ctx);
        let mut reg = Registry::default();
        let s = StatusView::default();
        let p = PlayView {
            visible: true,
            playing: false,
            t: 10.0,
            end: 100.0,
            speed: 1.0,
        };
        run(&ctx, &mut reg, &s, &p, vec![], 0.0);
        assert_eq!(reg.get("Reproducao_T").unwrap().text, "t = 10.00 / 100.00");
        assert_eq!(reg.get("Reproducao_Play").unwrap().text, "Reproduzir");
        assert_eq!(
            click(&ctx, &mut reg, &s, &p, "Reproducao_Play"),
            vec!["reproduzir".to_owned()]
        );
        assert_eq!(
            click(&ctx, &mut reg, &s, &p, "Reproducao_Proximo"),
            vec!["evento:1".to_owned()]
        );
        assert_eq!(
            click(&ctx, &mut reg, &s, &p, "Reproducao_Anterior"),
            vec!["evento:-1".to_owned()]
        );
        let out = click(&ctx, &mut reg, &s, &p, "Reproducao_Tempo");
        let t: f64 = out
            .iter()
            .find_map(|a| a.strip_prefix("tempo:"))
            .and_then(|x| x.parse().ok())
            .unwrap_or_else(|| panic!("{out:?}"));
        assert!((t - 50.0).abs() < 5.0, "o meio da linha do tempo: {t}");
        assert!(reg.request("Reproducao_Velocidade", "10x"));
        assert_eq!(
            run(&ctx, &mut reg, &s, &p, vec![], 3.0),
            vec!["velocidade:10".to_owned()]
        );
        // Everything inside the window, nothing overlapping the time.
        let time = reg.get("Reproducao_T").unwrap().rect;
        let line = reg.get("Reproducao_Tempo").unwrap().rect;
        assert!(line.max.x <= time.min.x + 0.5, "{line:?} {time:?}");
        assert!(time.max.x <= 1270.5);
    }

    #[test]
    fn speeds_print_without_zeros() {
        assert_eq!(speed_label(0.25), "0.25x");
        assert_eq!(speed_label(1.0), "1x");
        assert_eq!(speed_label(100.0), "100x");
    }
}
