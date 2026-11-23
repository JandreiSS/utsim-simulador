//! The texts of UTSim 3D (the Godot client) in the five languages of the family: the keys
//! of `idiomas/<código>/godot.ftl` (`docs/traducao.md`, round 8). The screens read them by
//! [`tx`]; the GDScript side through the bridge.

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
        const PREFIXES: [&str; 5] = ["aba-", "barra-", "configuracoes-", "props-", "vivo-"];
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
}
