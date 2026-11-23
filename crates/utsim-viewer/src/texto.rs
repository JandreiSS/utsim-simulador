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

/// A number already formatted, with the decimal mark of the current language.
pub fn dx(s: String) -> Valor<'static> {
    utsim_idiomas::marca_decimal(&s).into()
}

/// A text as it is.
pub fn sx(s: String) -> Valor<'static> {
    s.into()
}

/// The name of a block state in the language of the interface (`utsim_scene::style::state_label`
/// stays Portuguese for the scripts and the Godot client).
pub fn estado(state: Option<utsim_core::State>) -> &'static str {
    use utsim_core::State;
    tx(match state {
        None => "estado-sem-estado",
        Some(State::Idle) => "estado-ocioso",
        Some(State::Empty) => "estado-vazio",
        Some(State::Generating) => "estado-gerando",
        Some(State::Releasing) => "estado-liberando",
        Some(State::Collecting) => "estado-coletando",
        Some(State::Setup) => "estado-preparacao",
        Some(State::Processing) => "estado-processando",
        Some(State::Blocked) => "estado-bloqueado",
        Some(State::WaitingOperator) => "estado-aguardando-operador",
        Some(State::WaitingTransport) => "estado-aguardando-transporte",
        Some(State::Down) => "estado-em-falha",
        Some(State::OffShift) => "estado-fora-do-turno",
        Some(State::TravelEmpty) => "estado-viajando-vazio",
        Some(State::TravelLoaded) => "estado-viajando-carregado",
        Some(State::Loading) => "estado-carregando",
        Some(State::Unloading) => "estado-descarregando",
        Some(State::Utilize) => "estado-trabalhando-no-posto",
        Some(State::Stopped) => "estado-parado",
        Some(State::WaitingRepair) => "estado-aguardando-tecnico",
        Some(State::Maintenance) => "estado-em-manutencao",
        Some(State::Resting) => "estado-em-pausa",
        Some(State::Entering) => "estado-operador-entrando",
        Some(State::Leaving) => "estado-operador-saindo",
    })
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

    /// Every key written in the sources (`"prop-..."`, `"editor-..."`...) is in
    /// `editor.ftl`: a mistyped key would show up as the key itself on screen.
    #[test]
    fn every_key_in_the_sources_exists() {
        const PREFIXES: [&str; 15] = [
            "atalho-",
            "camada-",
            "camadas-",
            "camera-",
            "cameras-",
            "dist-",
            "editor-",
            "estado-",
            "flows-",
            "fluxo-tipo-",
            "models-",
            "prop-",
            "res-",
            "tipo-",
            "ui-",
        ];
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    walk(&p, out);
                } else if p.extension().is_some_and(|x| x == "rs") {
                    out.push(p);
                }
            }
        }
        let mut files = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut files,
        );
        let mut missing = Vec::new();
        let mut seen = 0;
        for f in files {
            let text = std::fs::read_to_string(&f).unwrap();
            for piece in text.split('"').skip(1).step_by(2) {
                let key_like = !piece.is_empty()
                    && piece.chars().all(|c| {
                        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_'
                    });
                if key_like
                    && PREFIXES
                        .iter()
                        .any(|p| piece.len() > p.len() && piece.starts_with(p))
                {
                    seen += 1;
                    if !TEXTOS.tem(piece) {
                        missing.push(format!("{}: {piece}", f.display()));
                    }
                }
            }
        }
        assert!(seen > 400, "only {seen} keys found");
        assert!(missing.is_empty(), "{missing:#?}");
    }

    /// Every `fx(key, &[(name, value), ...])` call passes exactly the variables the
    /// Portuguese message uses: a wrong name would show `{$name}` on screen.
    #[test]
    fn every_fx_passes_the_variables_of_its_message() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let ftl = std::fs::read_to_string(root.join("../../idiomas/pt-BR/editor.ftl")).unwrap();
        let mut vars: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> =
            Default::default();
        let mut current: Option<String> = None;
        for line in ftl.lines() {
            let starts_entry =
                line.starts_with(|c: char| c.is_ascii_lowercase()) && line.contains(" =");
            if starts_entry {
                let key = line[..line.find(" =").unwrap()].trim().to_string();
                vars.entry(key.clone()).or_default();
                current = Some(key);
            } else if line.is_empty() || line.starts_with('#') {
                current = None;
            }
            if let Some(k) = &current {
                for piece in line.split('$').skip(1) {
                    let name: String = piece
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .collect();
                    vars.get_mut(k).unwrap().insert(name);
                }
            }
        }
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    walk(&p, out);
                } else if p.extension().is_some_and(|x| x == "rs") {
                    out.push(p);
                }
            }
        }
        let mut files = Vec::new();
        walk(&root.join("src"), &mut files);
        let mut wrong = Vec::new();
        let mut seen = 0;
        for f in files {
            let text = std::fs::read_to_string(&f).unwrap();
            for (at, _) in text.match_indices("fx(") {
                let before = text[..at].chars().next_back();
                if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
                    continue;
                }
                let Some(rest) = text[at + 3..].trim_start().strip_prefix('"') else {
                    continue;
                };
                let key = &rest[..rest.find('"').unwrap()];
                let after = rest[key.len() + 1..].trim_start();
                let Some(after) = after.strip_prefix(',') else {
                    continue;
                };
                let Some(list) = after.trim_start().strip_prefix("&[") else {
                    continue;
                };
                let mut depth = 1;
                let end = list
                    .char_indices()
                    .find(|&(_, c)| {
                        depth += match c {
                            '[' => 1,
                            ']' => -1,
                            _ => 0,
                        };
                        depth == 0
                    })
                    .map(|(i, _)| i)
                    .unwrap();
                let passed: std::collections::BTreeSet<String> = list[..end]
                    .split('(')
                    .skip(1)
                    .filter_map(|p| p.trim_start().strip_prefix('"'))
                    .filter_map(|p| p.split_once("\",").map(|(n, _)| n.to_string()))
                    .filter(|n| {
                        !n.is_empty() && n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    })
                    .collect();
                seen += 1;
                if vars.get(key) != Some(&passed) {
                    wrong.push(format!(
                        "{}: {key} {passed:?} vs {:?}",
                        f.display(),
                        vars.get(key)
                    ));
                }
            }
        }
        assert!(seen > 100, "only {seen} fx calls found");
        assert!(wrong.is_empty(), "{wrong:#?}");
    }
}
