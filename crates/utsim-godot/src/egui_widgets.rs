//! Stable names of the egui widgets (pure: no Godot types, unit tested).
//!
//! The scripted UI runs (`godot/roteiros/*.json`, `roteiro.gd`) find the controls by node
//! name (`Modo_Editar`, `Botao_Novo`, `Prop_name`...). The widgets drawn by egui keep those
//! names: each frame the interface records here the rectangle, the text and the state of
//! every named widget it drew, and the bridge answers `widget_rect(name)` and the like from
//! it, so a step that clicked a Godot button now clicks the egui widget at the same name.
//! Choices that open a list (a combo box) can also be made by name ([`Registry::request`]),
//! as the `escolher` step does with Godot's `OptionButton`.

use std::collections::{BTreeMap, HashMap};

use egui::{Rect, Response};

/// What the interface knows about a named widget of the last frame drawn.
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetInfo {
    /// Where it is, in points.
    pub rect: Rect,
    /// Its text (label, value or selected choice).
    pub text: String,
    pub enabled: bool,
    /// A toggle's state (`None` for a plain button or a field).
    pub checked: Option<bool>,
    /// The choices of a list, in order (for `escolher`).
    pub choices: Vec<String>,
}

/// The named widgets of the last frame drawn, and the pending scripted requests.
#[derive(Default, Debug)]
pub struct Registry {
    widgets: BTreeMap<String, WidgetInfo>,
    requests: HashMap<String, String>,
}

impl Registry {
    /// Forgets the widgets of the previous frame (call before drawing a new one).
    pub fn begin_frame(&mut self) {
        self.widgets.clear();
    }

    /// Records the widget `name` drawn with `response`.
    pub fn put(&mut self, name: &str, response: &Response, text: impl Into<String>) {
        self.put_info(
            name,
            WidgetInfo {
                rect: response.rect,
                text: text.into(),
                enabled: response.enabled(),
                checked: None,
                choices: Vec::new(),
            },
        );
    }

    /// Records a toggle (check box, segmented or toggle button).
    pub fn put_toggle(&mut self, name: &str, response: &Response, text: &str, on: bool) {
        self.put(name, response, text);
        if let Some(w) = self.widgets.get_mut(name) {
            w.checked = Some(on);
        }
    }

    /// Records a list with its choices (`text` is the selected one).
    pub fn put_list(&mut self, name: &str, response: &Response, text: &str, choices: Vec<String>) {
        self.put(name, response, text);
        if let Some(w) = self.widgets.get_mut(name) {
            w.choices = choices;
        }
    }

    pub fn put_info(&mut self, name: &str, info: WidgetInfo) {
        // Only what is on screen counts: a widget clipped away has a zero rectangle.
        if info.rect.is_positive() {
            self.widgets.insert(name.to_owned(), info);
        }
    }

    pub fn get(&self, name: &str) -> Option<&WidgetInfo> {
        self.widgets.get(name)
    }

    /// The rectangle of `name` in pixels, `[x, y, width, height]`.
    pub fn rect_pixels(&self, name: &str, ppp: f32) -> Option<[f32; 4]> {
        self.get(name).map(|w| {
            let r = w.rect;
            [
                r.min.x * ppp,
                r.min.y * ppp,
                r.width() * ppp,
                r.height() * ppp,
            ]
        })
    }

    /// The names drawn in the last frame, sorted.
    pub fn names(&self) -> Vec<String> {
        self.widgets.keys().cloned().collect()
    }

    /// Asks the widget `name` to take the choice `item` the next time it is drawn. False
    /// when the widget is not on screen or has no such choice.
    pub fn request(&mut self, name: &str, item: &str) -> bool {
        let known = self
            .get(name)
            .is_some_and(|w| w.choices.iter().any(|c| c == item));
        if known {
            self.requests.insert(name.to_owned(), item.to_owned());
        }
        known
    }

    /// The pending choice for `name`, taken (the widget applies it once).
    pub fn take_request(&mut self, name: &str) -> Option<String> {
        self.requests.remove(name)
    }

    pub fn has_requests(&self) -> bool {
        !self.requests.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Pos2, Vec2};

    fn info(x: f32, text: &str) -> WidgetInfo {
        WidgetInfo {
            rect: Rect::from_min_size(Pos2::new(x, 4.0), Vec2::new(40.0, 20.0)),
            text: text.to_owned(),
            enabled: true,
            checked: None,
            choices: vec!["0.5 m".into(), "1 m".into()],
        }
    }

    #[test]
    fn names_are_kept_for_one_frame() {
        let mut r = Registry::default();
        r.put_info("Botao_Novo", info(10.0, "Novo"));
        r.put_info("Modo_Editar", info(60.0, "Editar"));
        assert_eq!(r.names(), vec!["Botao_Novo", "Modo_Editar"]);
        assert_eq!(
            r.rect_pixels("Botao_Novo", 2.0),
            Some([20.0, 8.0, 80.0, 40.0])
        );
        assert_eq!(
            r.get("Modo_Editar").map(|w| w.text.as_str()),
            Some("Editar")
        );
        r.begin_frame();
        assert!(r.get("Botao_Novo").is_none());
    }

    #[test]
    fn hidden_widgets_are_not_listed() {
        let mut r = Registry::default();
        let mut w = info(0.0, "x");
        w.rect = Rect::NOTHING;
        r.put_info("Oculto", w);
        assert!(r.get("Oculto").is_none());
    }

    #[test]
    fn choices_are_requested_by_name() {
        let mut r = Registry::default();
        r.put_info("Grade_Passo", info(0.0, "1 m"));
        assert!(!r.request("Grade_Passo", "3 m"), "não é uma das escolhas");
        assert!(!r.request("Outro", "1 m"), "não está na tela");
        assert!(r.request("Grade_Passo", "0.5 m"));
        assert!(r.has_requests());
        assert_eq!(r.take_request("Grade_Passo").as_deref(), Some("0.5 m"));
        assert_eq!(r.take_request("Grade_Passo"), None);
    }
}
