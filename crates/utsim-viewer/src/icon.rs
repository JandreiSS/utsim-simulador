//! Ícone da janela do `utsim-view` (barra de título e barra de tarefas).
//!
//! No Windows, o ícone do arquivo `.exe` vem do `build.rs`; aqui definimos o da janela em
//! execução, que vale também no Linux. O PNG é embutido no binário.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WINIT_WINDOWS;

/// PNG de 64x64 gerado por `tools/icone` a partir de `assets/icone/utsim.svg`.
const ICONE_PNG: &[u8] = include_bytes!("../../../assets/icone/utsim-64.png");

/// Registra o ícone da janela principal assim que ela existir.
pub struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, definir_icone);
    }
}

/// Decodifica o PNG embutido em RGBA de 8 bits: `(largura, altura, bytes)`.
fn decodificar(png_bytes: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    let mut leitor = png::Decoder::new(std::io::Cursor::new(png_bytes))
        .read_info()
        .ok()?;
    let mut buffer = vec![0; leitor.output_buffer_size()?];
    let info = leitor.next_frame(&mut buffer).ok()?;
    if info.color_type != png::ColorType::Rgba || info.bit_depth != png::BitDepth::Eight {
        return None;
    }
    buffer.truncate(info.buffer_size());
    Some((info.width, info.height, buffer))
}

fn definir_icone(janela: Query<Entity, With<PrimaryWindow>>, mut pronto: Local<bool>) {
    if *pronto {
        return;
    }
    let Ok(entidade) = janela.single() else {
        return;
    };
    WINIT_WINDOWS.with_borrow(|janelas| {
        let Some(janela) = janelas.get_window(entidade) else {
            return;
        };
        // Se algo falhar, a janela fica com o ícone padrão do sistema.
        *pronto = true;
        if let Some((largura, altura, rgba)) = decodificar(ICONE_PNG)
            && let Ok(icone) = winit::window::Icon::from_rgba(rgba, largura, altura)
        {
            janela.set_window_icon(Some(icone));
        }
    });
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn icone_embutido_decodifica() {
        let (l, a, rgba) = decodificar(ICONE_PNG).expect("PNG RGBA de 8 bits");
        assert_eq!((l, a), (64, 64));
        assert_eq!(rgba.len(), 64 * 64 * 4);
        assert!(rgba.chunks(4).any(|p| p[3] > 0), "deve ter pixels visíveis");
    }
}
