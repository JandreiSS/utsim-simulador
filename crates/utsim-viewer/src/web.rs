//! Browser entry point (`wasm32`): read-only playback of a replay handed over by the
//! page. The page (an iframe, see `web/viewer.html`) sets `window.UTSIM_REPLAY` to the
//! JSON text and then initialises the wasm module, whose start function calls `main`.
//! The accepted text is either a bare replay or the `{ok, replay}` envelope returned by
//! `utsim-web`'s `replay(ron, stop)`.

use bevy::prelude::*;
use utsim_model::replay::Replay;
use utsim_viewer::timeline::TransitSetting;

use crate::editor::Editor;
use crate::playback::Data;

/// Selector of the `<canvas>` the page provides.
const CANVAS: &str = "#utsim-canvas";

fn report(message: &str) {
    web_sys::console::error_1(&message.into());
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("utsim-msg"))
    {
        el.set_text_content(Some(message));
    }
}

fn parse(text: &str) -> Result<Replay, String> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("replay inválido: {e}"))?;
    if value.get("ok") == Some(&serde_json::Value::Bool(false)) {
        return Err("o modelo tem problemas; corrija na aba Simular".into());
    }
    let value = match value.get("replay") {
        Some(inner) => inner.clone(),
        None => value,
    };
    serde_json::from_value(value).map_err(|e| format!("replay inválido: {e}"))
}

fn read_replay() -> Result<Replay, String> {
    let window = web_sys::window().ok_or("sem janela")?;
    let text = js_sys::Reflect::get(&window, &"UTSIM_REPLAY".into())
        .ok()
        .and_then(|v| v.as_string())
        .ok_or("a página não forneceu UTSIM_REPLAY")?;
    parse(&text)
}

pub fn main() {
    let replay = match read_replay() {
        Ok(r) => r,
        Err(e) => {
            report(&e);
            return;
        }
    };
    let data = Data::from_replay(replay, TransitSetting::Auto);
    let editor = Editor::read_only().with_transit(TransitSetting::Auto);
    let window = Window {
        title: "UTSim".to_string(),
        canvas: Some(CANVAS.to_string()),
        fit_canvas_to_parent: true,
        prevent_default_event_handling: true,
        ..default()
    };
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }));
    crate::configure(&mut app, editor, data).run();
}
