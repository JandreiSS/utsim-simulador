//! The texts of UTSim 3D (the Godot client) in the five languages of the family: the keys
//! of `idiomas/<código>/godot.ftl` (`docs/traducao.md`, round 8). The screens read them by
//! [`tx`]; the GDScript side through the bridge.

use std::fmt::Display;

use utsim_idiomas::{Catalogo, Valor};

/// The texts of UTSim 3D in the five languages, embedded from `idiomas/`.
pub static TEXTOS: Catalogo = utsim_idiomas::catalogo!("godot");

/// The text of a key in the language of now.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_settings_texts_exist_in_portuguese() {
        utsim_idiomas::definir_nesta_thread(Some(utsim_idiomas::Idioma::PtBr));
        assert_eq!(tx("configuracoes-titulo"), "Configurações");
        assert_eq!(tx("configuracoes-formulario"), "Formulário");
        assert!(tx("configuracoes-distribuicoes-ajuda").contains("{ }"));
        utsim_idiomas::definir_nesta_thread(None);
    }

    /// Every key written in the sources (`"aba-..."`, `"vivo-..."`...) is in `godot.ftl`,
    /// in the five languages: a mistyped key would show up as the key itself on screen.
    #[test]
    fn every_key_in_the_sources_exists() {
        assert!(TEXTOS.faltas().is_empty(), "{:?}", TEXTOS.faltas());
        const PREFIXES: [&str; 6] = [
            "aba-",
            "barra-",
            "configuracoes-",
            "godot-",
            "props-",
            "vivo-",
        ];
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut missing = Vec::new();
        let mut seen = 0;
        for e in std::fs::read_dir(&src).unwrap().flatten() {
            let p = e.path();
            if p.extension().is_none_or(|x| x != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&p).unwrap();
            for piece in text.split('"').skip(1).step_by(2) {
                let key_like = piece
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_');
                if key_like
                    && PREFIXES
                        .iter()
                        .any(|x| piece.len() > x.len() && piece.starts_with(x))
                {
                    seen += 1;
                    if !TEXTOS.tem(piece) {
                        missing.push(format!("{}: {piece}", p.display()));
                    }
                }
            }
        }
        assert!(seen > 40, "only {seen} keys found");
        assert!(missing.is_empty(), "{missing:#?}");
    }

    /// Every key the GDScript side asks for (`T("godot-gd-...")`) is in `godot.ftl`.
    #[test]
    fn every_key_of_the_scripts_exists() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../godot/scripts");
        let mut missing = Vec::new();
        let mut seen = 0;
        for e in std::fs::read_dir(&dir).unwrap().flatten() {
            let p = e.path();
            if p.extension().is_none_or(|x| x != "gd") {
                continue;
            }
            let text = std::fs::read_to_string(&p).unwrap();
            for piece in text.split('"').skip(1).step_by(2) {
                if piece.starts_with(&["godot", "gd-"].join("-"))
                    && piece.chars().all(|c| {
                        c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_'
                    })
                {
                    seen += 1;
                    if !TEXTOS.tem(piece) {
                        missing.push(format!("{}: {piece}", p.display()));
                    }
                }
            }
        }
        assert!(seen > 100, "only {seen} keys found");
        assert!(missing.is_empty(), "{missing:#?}");
    }
}
