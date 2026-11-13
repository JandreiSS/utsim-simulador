//! The texts of UTSim 3D (the Godot client) in the five languages of the family: the keys
//! of `idiomas/<código>/godot.ftl` (`docs/traducao.md`, round 8). The screens read them by
//! [`tx`]; the GDScript side through the bridge.

use utsim_idiomas::Catalogo;

/// The texts of UTSim 3D in the five languages, embedded from `idiomas/`.
pub static TEXTOS: Catalogo = utsim_idiomas::catalogo!("godot");

/// The text of a key in the language of now.
pub fn tx(chave: &'static str) -> &'static str {
    TEXTOS.t(chave)
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
}
