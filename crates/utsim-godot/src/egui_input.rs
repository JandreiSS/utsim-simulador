//! Godot input to egui events (pure: no Godot types, unit tested).
//!
//! [`crate::ui_class::UtsimUi`] reads each `InputEvent` into an [`Input`] (positions in
//! viewport pixels, Godot key codes and mouse button indices) and [`to_egui`] turns it into
//! the egui events of that input, in points (pixels over `pixels_per_point`).

use egui::{Event, Key, Modifiers, MouseWheelUnit, PointerButton, Pos2, TouchPhase, Vec2};

/// Godot's `KEY_SPECIAL` bit: the keys that are not characters.
pub(crate) const SPECIAL: i64 = 1 << 22;

/// The modifier keys held during an event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mods {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Mods {
    /// egui's modifiers: `command` is Ctrl, or Cmd on macOS.
    pub fn to_egui(self, mac: bool) -> Modifiers {
        Modifiers {
            alt: self.alt,
            ctrl: self.ctrl,
            shift: self.shift,
            mac_cmd: mac && self.meta,
            command: if mac { self.meta } else { self.ctrl },
        }
    }
}

/// One Godot input event, as plain data.
#[derive(Clone, Debug, PartialEq)]
pub enum Input {
    /// `InputEventMouseMotion`.
    Motion { pos: [f32; 2], mods: Mods },
    /// `InputEventMouseButton`: `button` is Godot's `MouseButton` (1 left, 2 right,
    /// 3 middle, 4 to 7 the wheel, 8 and 9 the extra buttons); `factor` is the wheel's.
    Button {
        pos: [f32; 2],
        button: i64,
        pressed: bool,
        factor: f32,
        mods: Mods,
    },
    /// `InputEventKey`: `keycode` is Godot's `Key`, `unicode` the character typed (0 for
    /// none).
    Key {
        keycode: i64,
        unicode: u32,
        pressed: bool,
        echo: bool,
        mods: Mods,
    },
    /// The mouse left the window.
    PointerGone,
    /// The window gained or lost the focus.
    Focus(bool),
}

impl Input {
    /// The pointer position of a mouse event, in pixels.
    pub fn pos(&self) -> Option<[f32; 2]> {
        match self {
            Input::Motion { pos, .. } | Input::Button { pos, .. } => Some(*pos),
            _ => None,
        }
    }

    /// Whether this is a turn of the mouse wheel.
    pub fn is_wheel(&self) -> bool {
        matches!(self, Input::Button { button: 4..=7, .. })
    }
}

/// The egui events of `input`. `ppp` is pixels per point; `clipboard` gives the system
/// clipboard's text, read only for a paste (Ctrl+V).
pub fn to_egui(
    input: &Input,
    ppp: f32,
    mac: bool,
    clipboard: impl FnOnce() -> String,
) -> Vec<Event> {
    let ppp = if ppp > 0.0 { ppp } else { 1.0 };
    let at = |p: [f32; 2]| Pos2::new(p[0] / ppp, p[1] / ppp);
    match *input {
        Input::Motion { pos, .. } => vec![Event::PointerMoved(at(pos))],
        Input::Button {
            pos,
            button,
            pressed,
            factor,
            mods,
        } => {
            let modifiers = mods.to_egui(mac);
            if (4..=7).contains(&button) {
                // A notch is a press and a release: scroll once, on the press.
                if !pressed {
                    return vec![];
                }
                let f = if factor > 0.0 { factor } else { 1.0 };
                let delta = match button {
                    4 => Vec2::new(0.0, f),
                    5 => Vec2::new(0.0, -f),
                    6 => Vec2::new(f, 0.0),
                    _ => Vec2::new(-f, 0.0),
                };
                return vec![Event::MouseWheel {
                    unit: MouseWheelUnit::Line,
                    delta,
                    phase: TouchPhase::Move,
                    modifiers,
                }];
            }
            let Some(button) = pointer_button(button) else {
                return vec![];
            };
            vec![
                Event::PointerMoved(at(pos)),
                Event::PointerButton {
                    pos: at(pos),
                    button,
                    pressed,
                    modifiers,
                },
            ]
        }
        Input::Key {
            keycode,
            unicode,
            pressed,
            echo,
            mods,
        } => {
            let modifiers = mods.to_egui(mac);
            let mut out = Vec::new();
            if let Some(key) = key_of(keycode) {
                if pressed && modifiers.command && !modifiers.alt {
                    match key {
                        Key::C if !echo => out.push(Event::Copy),
                        Key::X if !echo => out.push(Event::Cut),
                        Key::V => out.push(Event::Paste(clipboard())),
                        _ => {}
                    }
                }
                out.push(Event::Key {
                    key,
                    physical_key: None,
                    pressed,
                    repeat: echo,
                    modifiers,
                });
            }
            if pressed && let Some(text) = typed(unicode, mods) {
                out.push(Event::Text(text));
            }
            out
        }
        Input::PointerGone => vec![Event::PointerGone],
        Input::Focus(on) => vec![Event::WindowFocused(on)],
    }
}

/// The text a key press types, if any (no control characters; Ctrl alone is a shortcut,
/// Ctrl+Alt is AltGr on Windows and still types).
fn typed(unicode: u32, mods: Mods) -> Option<String> {
    if unicode < 32 || unicode == 127 || (mods.ctrl && !mods.alt) || mods.meta {
        return None;
    }
    char::from_u32(unicode).map(String::from)
}

/// egui's pointer button of Godot's `MouseButton`.
pub fn pointer_button(button: i64) -> Option<PointerButton> {
    Some(match button {
        1 => PointerButton::Primary,
        2 => PointerButton::Secondary,
        3 => PointerButton::Middle,
        8 => PointerButton::Extra1,
        9 => PointerButton::Extra2,
        _ => return None,
    })
}

/// egui's key of Godot's `Key` code, for the keys egui uses (editing, navigation,
/// shortcuts).
pub fn key_of(code: i64) -> Option<Key> {
    if code & SPECIAL != 0 {
        return Some(match code - SPECIAL {
            1 => Key::Escape,
            2 | 3 => Key::Tab,
            4 => Key::Backspace,
            5 | 6 => Key::Enter,
            7 => Key::Insert,
            8 => Key::Delete,
            13 => Key::Home,
            14 => Key::End,
            15 => Key::ArrowLeft,
            16 => Key::ArrowUp,
            17 => Key::ArrowRight,
            18 => Key::ArrowDown,
            19 => Key::PageUp,
            20 => Key::PageDown,
            n @ 28..=47 => F_KEYS[(n - 28) as usize],
            _ => return None,
        });
    }
    Some(match code {
        32 => Key::Space,
        39 => Key::Quote,
        43 => Key::Plus,
        44 => Key::Comma,
        45 => Key::Minus,
        46 => Key::Period,
        47 => Key::Slash,
        48..=57 => DIGITS[(code - 48) as usize],
        59 => Key::Semicolon,
        61 => Key::Equals,
        65..=90 => LETTERS[(code - 65) as usize],
        91 => Key::OpenBracket,
        92 => Key::Backslash,
        93 => Key::CloseBracket,
        96 => Key::Backtick,
        _ => return None,
    })
}

const F_KEYS: [Key; 20] = [
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::F13,
    Key::F14,
    Key::F15,
    Key::F16,
    Key::F17,
    Key::F18,
    Key::F19,
    Key::F20,
];

const DIGITS: [Key; 10] = [
    Key::Num0,
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Num5,
    Key::Num6,
    Key::Num7,
    Key::Num8,
    Key::Num9,
];

const LETTERS: [Key; 26] = [
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
];

#[cfg(test)]
mod tests {
    use super::*;

    const ENTER: i64 = SPECIAL | 5;
    const F1: i64 = SPECIAL | 28;
    const F3: i64 = SPECIAL | 30;
    const LEFT: i64 = SPECIAL | 15;

    fn no_clip() -> String {
        panic!("a área de transferência só é lida ao colar")
    }

    #[test]
    fn positions_are_converted_to_points() {
        let ev = to_egui(
            &Input::Motion {
                pos: [300.0, 90.0],
                mods: Mods::default(),
            },
            1.5,
            false,
            no_clip,
        );
        assert_eq!(ev, vec![Event::PointerMoved(Pos2::new(200.0, 60.0))]);
    }

    #[test]
    fn a_click_moves_then_presses() {
        let ev = to_egui(
            &Input::Button {
                pos: [10.0, 20.0],
                button: 1,
                pressed: true,
                factor: 1.0,
                mods: Mods {
                    shift: true,
                    ..Mods::default()
                },
            },
            1.0,
            false,
            no_clip,
        );
        assert_eq!(ev.len(), 2);
        assert!(matches!(ev[0], Event::PointerMoved(_)));
        match &ev[1] {
            Event::PointerButton {
                button,
                pressed,
                modifiers,
                ..
            } => {
                assert_eq!(*button, PointerButton::Primary);
                assert!(*pressed);
                assert!(modifiers.shift);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn the_wheel_scrolls_once_per_notch() {
        let wheel = |button, pressed| Input::Button {
            pos: [0.0, 0.0],
            button,
            pressed,
            factor: 0.0,
            mods: Mods::default(),
        };
        assert!(wheel(5, true).is_wheel());
        assert!(!wheel(1, true).is_wheel());
        let down = to_egui(&wheel(5, true), 1.0, false, no_clip);
        assert!(
            matches!(down[0], Event::MouseWheel { delta, unit: MouseWheelUnit::Line, .. } if delta == Vec2::new(0.0, -1.0))
        );
        let up = to_egui(&wheel(4, true), 1.0, false, no_clip);
        assert!(matches!(up[0], Event::MouseWheel { delta, .. } if delta.y > 0.0));
        assert!(to_egui(&wheel(5, false), 1.0, false, no_clip).is_empty());
    }

    #[test]
    fn keys_map_to_egui() {
        assert_eq!(key_of(65), Some(Key::A));
        assert_eq!(key_of(90), Some(Key::Z));
        assert_eq!(key_of(48), Some(Key::Num0));
        assert_eq!(key_of(ENTER), Some(Key::Enter));
        assert_eq!(key_of(SPECIAL | 6), Some(Key::Enter));
        assert_eq!(key_of(F1), Some(Key::F1));
        assert_eq!(key_of(F3), Some(Key::F3));
        assert_eq!(key_of(LEFT), Some(Key::ArrowLeft));
        assert_eq!(
            key_of(SPECIAL | 25),
            None,
            "Shift sozinho não é tecla do egui"
        );
    }

    #[test]
    fn typing_gives_text_but_shortcuts_do_not() {
        let key = |unicode, ctrl, alt| Input::Key {
            keycode: 65,
            unicode,
            pressed: true,
            echo: false,
            mods: Mods {
                ctrl,
                alt,
                ..Mods::default()
            },
        };
        let ev = to_egui(&key('á' as u32, false, false), 1.0, false, no_clip);
        assert!(ev.contains(&Event::Text("á".into())));
        let ev = to_egui(&key('a' as u32, true, false), 1.0, false, no_clip);
        assert!(!ev.iter().any(|e| matches!(e, Event::Text(_))));
        // AltGr (Ctrl+Alt on Windows) still types.
        let ev = to_egui(&key('@' as u32, true, true), 1.0, false, no_clip);
        assert!(ev.contains(&Event::Text("@".into())));
        // Enter types no character.
        let enter = Input::Key {
            keycode: ENTER,
            unicode: 13,
            pressed: true,
            echo: false,
            mods: Mods::default(),
        };
        assert!(
            !to_egui(&enter, 1.0, false, no_clip)
                .iter()
                .any(|e| matches!(e, Event::Text(_)))
        );
    }

    #[test]
    fn copy_cut_and_paste_use_the_clipboard() {
        let ctrl = |code| Input::Key {
            keycode: code,
            unicode: 0,
            pressed: true,
            echo: false,
            mods: Mods {
                ctrl: true,
                ..Mods::default()
            },
        };
        assert!(to_egui(&ctrl(67), 1.0, false, no_clip).contains(&Event::Copy));
        assert!(to_egui(&ctrl(88), 1.0, false, no_clip).contains(&Event::Cut));
        let ev = to_egui(&ctrl(86), 1.0, false, || "colado".to_owned());
        assert!(ev.contains(&Event::Paste("colado".into())));
    }

    #[test]
    fn command_is_ctrl_except_on_mac() {
        let m = Mods {
            ctrl: true,
            ..Mods::default()
        };
        assert!(m.to_egui(false).command);
        assert!(!m.to_egui(true).command);
        let cmd = Mods {
            meta: true,
            ..Mods::default()
        };
        assert!(cmd.to_egui(true).command && cmd.to_egui(true).mac_cmd);
    }
}
