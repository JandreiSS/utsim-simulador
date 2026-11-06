//! Pecas de equipamento de protecao individual (EPI) para o operador simples, geradas por
//! codigo como o resto da biblioteca (sem ferramenta de modelagem, sem download).
//!
//! Os nomes sao os de `utsim_model::quality::PPE_NAMES`. Cada peca e um [`Model`] pequeno
//! que usa as mesmas coordenadas e os mesmos nomes de no do `operador_simples`
//! (`tronco`, `cabeca`, `braco_e`, `braco_d`, `perna_e`, `perna_d`, com os mesmos pivos):
//! quem monta a cena pendura o no da peca no no de mesmo nome do operador e a peca
//! acompanha o membro. Os materiais se chamam `epi_<peca>` (e `epi_<peca>_faixa` quando ha
//! um detalhe), para a cena poder recolorir.
//!
//! Para o manequim esqueletico, que tem outro esqueleto, `luvas` e `botina` tem tambem
//! uma versao centrada na origem ([`modelo_osso`]), que se prende a um osso da mao ou do pe.

use crate::builder::Model;
use crate::people::{ARM, HIP, SHOULDER};

/// Nomes de EPI que existem como peca, na ordem de `PPE_NAMES`.
pub const NOMES: &[&str] = &[
    "capacete",
    "oculos",
    "protetor_auricular",
    "luvas",
    "botina",
    "colete",
    "mascara",
    "avental",
];

/// Nos do `operador_simples` em que a peca `nome` se encaixa (os nos que ela contem).
pub fn nos(nome: &str) -> &'static [&'static str] {
    match nome {
        "capacete" | "oculos" | "protetor_auricular" | "mascara" => &["cabeca"],
        "colete" | "avental" => &["tronco"],
        "luvas" => &["braco_e", "braco_d"],
        "botina" => &["perna_e", "perna_d"],
        _ => &[],
    }
}

/// A peca `nome` no sistema de coordenadas do operador simples. `None` para um nome que
/// o 3D nao sabe vestir.
pub fn modelo(nome: &str) -> Option<Model> {
    let mut m = Model::new(&format!("epi_{nome}"));
    match nome {
        "capacete" => {
            let mat = m.material("epi_capacete", 0xf2f2f2, 0.05, 0.4);
            let faixa = m.material("epi_capacete_faixa", 0x1565c0, 0.0, 0.5);
            let n = m.node("cabeca", 0, [0.0, 0.0, 1.54]);
            m.sphere(n, mat, [0.0, 0.0, 1.655], 0.125, 12, 5, true);
            m.cylinder(n, mat, [0.0, 0.0, 1.650], [0.0, 0.0, 1.662], 0.15, 12);
            m.cuboid(n, faixa, [-0.01, -0.02, 1.74], [0.01, 0.02, 1.775]);
        }
        "oculos" => {
            let lente = m.translucent("epi_oculos", 0x9ad7ff, 0.45);
            let arm = m.material("epi_oculos_faixa", 0x222222, 0.0, 0.5);
            let n = m.node("cabeca", 0, [0.0, 0.0, 1.54]);
            for y in [-0.05f32, 0.05] {
                m.cuboid(n, lente, [0.105, y - 0.04, 1.615], [0.125, y + 0.04, 1.665]);
            }
            m.cuboid(n, arm, [0.105, -0.012, 1.635], [0.125, 0.012, 1.645]);
            for y in [-0.105f32, 0.105] {
                m.cuboid(n, arm, [-0.02, y - 0.004, 1.64], [0.125, y + 0.004, 1.65]);
            }
        }
        "protetor_auricular" => {
            let copa = m.material("epi_protetor_auricular", 0xd32f2f, 0.0, 0.55);
            let arco = m.material("epi_protetor_auricular_faixa", 0x333333, 0.1, 0.5);
            let n = m.node("cabeca", 0, [0.0, 0.0, 1.54]);
            for y in [-0.115f32, 0.115] {
                m.cylinder(
                    n,
                    copa,
                    [0.0, y - 0.025, 1.625],
                    [0.0, y + 0.025, 1.625],
                    0.055,
                    10,
                );
            }
            m.cuboid(n, arco, [-0.01, -0.12, 1.725], [0.01, 0.12, 1.735]);
            for y in [-0.115f32, 0.115] {
                m.cuboid(n, arco, [-0.01, y - 0.004, 1.63], [0.01, y + 0.004, 1.735]);
            }
        }
        "luvas" => {
            let mat = m.material("epi_luvas", 0xff6f00, 0.0, 0.8);
            for (nome, s) in [("braco_e", 1.0f32), ("braco_d", -1.0)] {
                let y = s * 0.265;
                let n = m.node(nome, 0, [0.0, y, SHOULDER]);
                m.cuboid(
                    n,
                    mat,
                    [-0.055, y - 0.058, SHOULDER - ARM - 0.06],
                    [0.055, y + 0.058, 0.86],
                );
            }
        }
        "botina" => {
            let mat = m.material("epi_botina", 0x5d4037, 0.0, 0.7);
            let ponta = m.material("epi_botina_faixa", 0x212121, 0.0, 0.6);
            for (nome, s) in [("perna_e", 1.0f32), ("perna_d", -1.0)] {
                let y = s * 0.10;
                let n = m.node(nome, 0, [0.0, y, HIP]);
                m.cuboid(n, mat, [-0.085, y - 0.075, 0.0], [0.08, y + 0.075, 0.22]);
                m.cuboid(n, mat, [0.0, y - 0.07, 0.0], [0.18, y + 0.07, 0.10]);
                m.cuboid(n, ponta, [0.15, y - 0.072, 0.0], [0.185, y + 0.072, 0.10]);
            }
        }
        "colete" => {
            let mat = m.material("epi_colete", 0xc6ff00, 0.0, 0.7);
            let faixa = m.material("epi_colete_faixa", 0xdde3e8, 0.3, 0.3);
            let n = m.node("tronco", 0, [0.0, 0.0, HIP]);
            m.cuboid(n, mat, [-0.135, -0.225, 1.00], [0.135, 0.225, 1.47]);
            for z in [1.08f32, 1.24, 1.38] {
                m.cuboid(n, faixa, [-0.14, -0.23, z], [0.14, 0.23, z + 0.03]);
            }
        }
        "mascara" => {
            let mat = m.material("epi_mascara", 0xeceff1, 0.0, 0.8);
            let elastico = m.material("epi_mascara_faixa", 0x455a64, 0.0, 0.7);
            let n = m.node("cabeca", 0, [0.0, 0.0, 1.54]);
            m.cuboid(n, mat, [0.085, -0.07, 1.575], [0.13, 0.07, 1.64]);
            for y in [-0.09f32, 0.09] {
                m.cuboid(
                    n,
                    elastico,
                    [-0.02, y - 0.004, 1.60],
                    [0.12, y + 0.004, 1.61],
                );
            }
        }
        "avental" => {
            let mat = m.material("epi_avental", 0x8d6e63, 0.0, 0.85);
            let alca = m.material("epi_avental_faixa", 0x3e2723, 0.0, 0.7);
            let n = m.node("tronco", 0, [0.0, 0.0, HIP]);
            m.cuboid(n, mat, [0.125, -0.19, 0.78], [0.14, 0.19, 1.34]);
            for y in [-0.15f32, 0.15] {
                m.cuboid(n, alca, [-0.13, y - 0.012, 1.34], [0.14, y + 0.012, 1.47]);
            }
        }
        _ => return None,
    }
    Some(m)
}

/// A versao centrada na origem de `luvas` e `botina`, para prender a um osso do manequim
/// esqueletico (a origem do modelo fica no osso). Outros nomes: `None`.
pub fn modelo_osso(nome: &str) -> Option<Model> {
    let mut m = Model::new(&format!("epi_{nome}_osso"));
    match nome {
        "luvas" => {
            let mat = m.material("epi_luvas", 0xff6f00, 0.0, 0.8);
            m.sphere(0, mat, [0.0, 0.0, 0.0], 0.065, 8, 5, false);
        }
        "botina" => {
            let mat = m.material("epi_botina", 0x5d4037, 0.0, 0.7);
            m.cuboid(0, mat, [-0.07, -0.06, -0.06], [0.07, 0.06, 0.06]);
        }
        _ => return None,
    }
    Some(m)
}

/// Os bytes `.glb` da peca `nome` (ou da versao de osso, com `osso`).
pub fn glb(nome: &str, osso: bool) -> Option<Vec<u8>> {
    let m = if osso {
        modelo_osso(nome)
    } else {
        modelo(nome)
    }?;
    Some(crate::glb::write(&m))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finito(m: &Model) -> bool {
        m.nodes.iter().all(|n| {
            n.pivot.iter().all(|v| v.is_finite())
                && n.parts.iter().all(|p| {
                    p.geometry
                        .positions
                        .iter()
                        .chain(p.geometry.normals.iter())
                        .all(|v| v.iter().all(|x| x.is_finite()))
                        && p.geometry
                            .indices
                            .iter()
                            .all(|&i| (i as usize) < p.geometry.positions.len())
                })
        })
    }

    #[test]
    fn toda_peca_existe_sem_nan() {
        for nome in NOMES {
            let m = modelo(nome).unwrap_or_else(|| panic!("falta a peca {nome}"));
            assert!(finito(&m), "{nome}");
            assert!(m.triangles() > 0, "{nome}");
            assert!(m.triangles() < 600, "{nome}: peca pesada");
            assert!(glb(nome, false).is_some_and(|b| b.starts_with(b"glTF")));
        }
        assert!(modelo("capa_de_chuva").is_none());
    }

    #[test]
    fn os_nos_batem_com_o_operador() {
        let operador = crate::people::operador_simples();
        for nome in NOMES {
            let m = modelo(nome).unwrap();
            let esperados = nos(nome);
            assert!(!esperados.is_empty(), "{nome}");
            for no in esperados {
                let na_peca = m.find(no).unwrap_or_else(|| panic!("{nome} sem no {no}"));
                let no_op = operador.find(no).expect("no do operador");
                assert_eq!(
                    m.nodes[na_peca].pivot, operador.nodes[no_op].pivot,
                    "{nome}/{no}"
                );
                assert!(!m.nodes[na_peca].parts.is_empty(), "{nome}/{no}");
            }
        }
    }

    #[test]
    fn as_pecas_cabem_no_operador() {
        // Nada fora de uma caixa folgada em volta do operador (1,76 m de altura).
        for nome in NOMES {
            let (lo, hi) = modelo(nome).unwrap().bounds();
            assert!(lo[2] >= -0.01 && hi[2] <= 1.85, "{nome}: altura");
            assert!(
                lo[0] > -0.4 && hi[0] < 0.4 && lo[1] > -0.4 && hi[1] < 0.4,
                "{nome}"
            );
        }
    }

    #[test]
    fn versao_de_osso_so_para_luvas_e_botina() {
        for nome in NOMES {
            let m = modelo_osso(nome);
            assert_eq!(m.is_some(), matches!(*nome, "luvas" | "botina"), "{nome}");
            if let Some(m) = m {
                assert!(finito(&m));
            }
        }
    }
}
