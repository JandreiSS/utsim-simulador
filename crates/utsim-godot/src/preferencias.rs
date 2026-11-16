//! The preferences of UTSim 3D on this computer, in `%APPDATA%/UTSim/utsim3d.json` (or
//! `$XDG_CONFIG_HOME`, `~/.config`); `UTSIM_PREFERENCIAS` names another file (the headless
//! tests use a fresh one). Only the screen: nothing here goes into the model.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferencias {
    /// The distribution fields open as code (RON text) instead of the form.
    pub distribuicao_codigo: bool,
    /// The language of the interface (`pt-BR`, `en`...); absent: the system's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idioma: Option<String>,
}

impl Preferencias {
    /// The language to use: the saved one, else the system's ([`utsim_idiomas::Idioma::inicial`]).
    pub fn lingua(&self) -> utsim_idiomas::Idioma {
        self.idioma
            .as_deref()
            .and_then(utsim_idiomas::Idioma::de_codigo)
            .unwrap_or_else(utsim_idiomas::Idioma::inicial)
    }
}

impl Preferencias {
    /// The file of the preferences.
    pub fn arquivo() -> Option<PathBuf> {
        if let Some(p) = std::env::var_os("UTSIM_PREFERENCIAS") {
            return Some(PathBuf::from(p));
        }
        let base = std::env::var_os("APPDATA")
            .or_else(|| std::env::var_os("XDG_CONFIG_HOME"))
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
        Some(base.join("UTSim").join("utsim3d.json"))
    }

    /// The preferences saved, or the default ones (no file, or one that does not read).
    pub fn carregar() -> Preferencias {
        Self::arquivo()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_default()
    }

    /// Saves them, creating the folder.
    pub fn gravar(&self) -> Result<(), String> {
        let p = Self::arquivo().ok_or("sem pasta de configuração")?;
        if let Some(d) = p.parent() {
            std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        let t = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(p, t).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preferences_read_back_and_default_to_the_form() {
        assert!(!Preferencias::default().distribuicao_codigo);
        let p = Preferencias {
            distribuicao_codigo: true,
            idioma: Some("en".into()),
        };
        assert_eq!(p.lingua(), utsim_idiomas::Idioma::En);
        let t = serde_json::to_string(&p).unwrap();
        assert_eq!(serde_json::from_str::<Preferencias>(&t).unwrap(), p);
        // An older or partial file keeps the defaults for what it lacks.
        assert_eq!(
            serde_json::from_str::<Preferencias>("{}").unwrap(),
            Preferencias::default()
        );
    }
}
