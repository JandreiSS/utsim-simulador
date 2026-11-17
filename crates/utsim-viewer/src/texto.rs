//! The texts of the simulator editor in the five languages of the family: the keys of
//! `idiomas/<código>/editor.ftl` (`docs/traducao.md`). What goes into the model (object
//! names, kinds, the `.ron` file) does not change with the language.
//!
//! The chosen language is kept in `%APPDATA%/UTSim/editor.json` (or `$XDG_CONFIG_HOME`,
//! `~/.config`); `UTSIM_PREFERENCIAS_EDITOR` names another file. A script (`--roteiro`)
//! always runs in Portuguese, since its steps find buttons by their Portuguese labels.

use std::fmt::Display;

use utsim_idiomas::{Catalogo, Idioma, Valor};

/// The texts of the editor in the five languages, embedded from `idiomas/`.
pub static TEXTOS: Catalogo = utsim_idiomas::catalogo!("editor");

/// The text of a key in the current language.
pub fn tx(chave: &'static str) -> &'static str {
    TEXTOS.t(chave)
}

/// The text of a key with named values.
pub fn fx(chave: &str, args: &[(&str, Valor)]) -> String {
    TEXTOS.f(chave, args)
}

/// A value: a number when it reads back the same (so plurals work), else text.
pub fn vx(x: &impl Display) -> Valor<'static> {
    let s = x.to_string();
    if let Ok(n) = s.parse::<i64>()
        && n.to_string() == s
    {
        return n.into();
    }
    s.into()
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct Preferencias {
    /// The language of the interface (`pt-BR`, `en`...); absent: the system's.
    #[serde(skip_serializing_if = "Option::is_none")]
    idioma: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn arquivo() -> Option<std::path::PathBuf> {
    if let Some(p) = std::env::var_os("UTSIM_PREFERENCIAS_EDITOR") {
        return Some(p.into());
    }
    let base = std::env::var_os("APPDATA")
        .or_else(|| std::env::var_os("XDG_CONFIG_HOME"))
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config"))
        })?;
    Some(base.join("UTSim").join("editor.json"))
}

/// The language saved on this computer, if any.
pub fn idioma_salvo() -> Option<Idioma> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let texto = std::fs::read_to_string(arquivo()?).ok()?;
        let p: Preferencias = serde_json::from_str(&texto).ok()?;
        Idioma::de_codigo(p.idioma.as_deref()?)
    }
    #[cfg(target_arch = "wasm32")]
    None
}

/// Uses `i` from now on and saves it for the next time (the browser build only uses it).
pub fn escolher_idioma(i: Idioma) {
    utsim_idiomas::definir(i);
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(p) = arquivo() {
        if let Some(d) = p.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let prefs = Preferencias {
            idioma: Some(i.codigo().to_string()),
        };
        if let Ok(t) = serde_json::to_string_pretty(&prefs) {
            let _ = std::fs::write(p, t);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_editor_in_the_five_languages() {
        assert!(TEXTOS.faltas().is_empty(), "{:?}", TEXTOS.faltas());
        let mut vistos = Vec::new();
        for i in Idioma::TODOS {
            utsim_idiomas::definir_nesta_thread(Some(i));
            vistos.push(tx("editor-desfazer"));
        }
        utsim_idiomas::definir_nesta_thread(None);
        vistos.dedup();
        assert_eq!(vistos.len(), Idioma::TODOS.len(), "{vistos:?}");
    }
}
