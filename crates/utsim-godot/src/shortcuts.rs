//! The keyboard shortcuts of the main scene (step 7, part 3, of
//! `docs/plano_interface_rust.md`): which action a key asks for, given the mode and the
//! tool. The main scene runs the action by name; the table is tested here and is the same
//! one the "?" window lists (`document::SHORTCUTS`).

use egui::Key;

/// What the key decides on: the mode and the tool of Editar.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeyContext {
    pub editing: bool,
    pub tool: String,
}

/// The action of a key press (Godot keycode, Ctrl or Cmd, Shift), or `None` when the key
/// is not a shortcut here (it goes on to whoever else wants it).
///
/// Actions: `desfazer`, `refazer`, `salvar`, `salvar_como`, `abrir`, `novo`,
/// `organizar:<copiar|recortar|colar|duplicar>`, `selecionar_tudo`, `atalhos`,
/// `desempenho`, `modo:alternar`, `enquadrar`, `reproduzir`, `excluir`, `cancelar`,
/// `mover:<dx>:<dy>`, `ferramenta:<nome>` (the tool, or `selecionar` when it is already
/// on), `zona_fechar`, `corredor_fechar` and `grade`.
pub fn key_action(keycode: i64, ctrl: bool, shift: bool, ctx: &KeyContext) -> Option<String> {
    let key = crate::egui_input::key_of(keycode)?;
    let s = |a: &str| Some(a.to_owned());
    if ctrl {
        return match key {
            Key::Z if ctx.editing && shift => s("refazer"),
            Key::Z if ctx.editing => s("desfazer"),
            Key::Y if ctx.editing => s("refazer"),
            Key::S if shift => s("salvar_como"),
            Key::S => s("salvar"),
            Key::O => s("abrir"),
            Key::N => s("novo"),
            Key::C => s("organizar:copiar"),
            Key::X => s("organizar:recortar"),
            Key::V => s("organizar:colar"),
            Key::D => s("organizar:duplicar"),
            Key::A => s("selecionar_tudo"),
            _ => None,
        };
    }
    match key {
        Key::F1 => return s("atalhos"),
        Key::F3 => return s("desempenho"),
        Key::Tab => return s("modo:alternar"),
        Key::F => return s("enquadrar"),
        Key::Space if !ctx.editing => return s("reproduzir"),
        _ => {}
    }
    if !ctx.editing {
        return None;
    }
    let toggle = |t: &str| {
        Some(format!(
            "ferramenta:{}",
            if ctx.tool == t { "selecionar" } else { t }
        ))
    };
    match key {
        Key::Delete | Key::Backspace => s("excluir"),
        Key::Escape => s("cancelar"),
        Key::ArrowLeft => s("mover:-1:0"),
        Key::ArrowRight => s("mover:1:0"),
        Key::ArrowUp => s("mover:0:1"),
        Key::ArrowDown => s("mover:0:-1"),
        Key::L => toggle("ligar"),
        Key::N => toggle("rede"),
        Key::C => toggle("corredor"),
        Key::E => toggle("equipe"),
        Key::Z => toggle("zona"),
        Key::Enter if ctx.tool == "zona" => s("zona_fechar"),
        Key::Enter if ctx.tool == "corredor" => s("corredor_fechar"),
        Key::G => s("grade"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPECIAL: i64 = 1 << 22;

    fn ctx(editing: bool, tool: &str) -> KeyContext {
        KeyContext {
            editing,
            tool: tool.into(),
        }
    }

    fn act(code: i64, ctrl: bool, shift: bool, c: &KeyContext) -> Option<String> {
        key_action(code, ctrl, shift, c)
    }

    #[test]
    fn ctrl_shortcuts_work_in_both_modes_and_undo_only_in_editar() {
        let e = ctx(true, "selecionar");
        let s = ctx(false, "selecionar");
        let z = 'Z' as i64;
        assert_eq!(act(z, true, false, &e).as_deref(), Some("desfazer"));
        assert_eq!(act(z, true, true, &e).as_deref(), Some("refazer"));
        assert_eq!(act('Y' as i64, true, false, &e).as_deref(), Some("refazer"));
        assert_eq!(act(z, true, false, &s), None, "no undo while simulating");
        assert_eq!(
            act('S' as i64, true, true, &s).as_deref(),
            Some("salvar_como")
        );
        assert_eq!(act('S' as i64, true, false, &s).as_deref(), Some("salvar"));
        assert_eq!(
            act('D' as i64, true, false, &e).as_deref(),
            Some("organizar:duplicar")
        );
        assert_eq!(act('Q' as i64, true, false, &e), None);
    }

    #[test]
    fn the_keys_of_editar_toggle_the_tools_and_follow_the_tool() {
        let sel = ctx(true, "selecionar");
        assert_eq!(
            act('L' as i64, false, false, &sel).as_deref(),
            Some("ferramenta:ligar")
        );
        assert_eq!(
            act('L' as i64, false, false, &ctx(true, "ligar")).as_deref(),
            Some("ferramenta:selecionar")
        );
        assert_eq!(
            act('Z' as i64, false, false, &sel).as_deref(),
            Some("ferramenta:zona")
        );
        assert_eq!(
            act(SPECIAL + 15, false, false, &sel).as_deref(),
            Some("mover:-1:0")
        );
        assert_eq!(
            act(SPECIAL + 8, false, false, &sel).as_deref(),
            Some("excluir")
        );
        assert_eq!(
            act(SPECIAL + 4, false, false, &sel).as_deref(),
            Some("excluir")
        );
        assert_eq!(
            act(SPECIAL + 1, false, false, &sel).as_deref(),
            Some("cancelar")
        );
        // Enter closes what the tool is drawing, and is nothing for the others.
        assert_eq!(
            act(SPECIAL + 5, false, false, &ctx(true, "zona")).as_deref(),
            Some("zona_fechar")
        );
        assert_eq!(
            act(SPECIAL + 6, false, false, &ctx(true, "corredor")).as_deref(),
            Some("corredor_fechar")
        );
        assert_eq!(act(SPECIAL + 5, false, false, &sel), None);
        // Simulating: the tool keys are nothing; Space plays; F frames; Tab switches.
        let sim = ctx(false, "selecionar");
        assert_eq!(act('L' as i64, false, false, &sim), None);
        assert_eq!(act(32, false, false, &sim).as_deref(), Some("reproduzir"));
        assert_eq!(act(32, false, false, &sel), None);
        assert_eq!(
            act('F' as i64, false, false, &sel).as_deref(),
            Some("enquadrar")
        );
        assert_eq!(
            act(SPECIAL + 2, false, false, &sim).as_deref(),
            Some("modo:alternar")
        );
        assert_eq!(
            act(SPECIAL + 28, false, false, &sim).as_deref(),
            Some("atalhos")
        );
    }
}
