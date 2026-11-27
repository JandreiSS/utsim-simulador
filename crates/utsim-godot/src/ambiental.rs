//! Camada Ambiental do cliente 3D (fase AM13 de `docs/plano_ambiental.md` do UTSim
//! Ambiental), sem tipos do Godot. A conta inteira é a do UTSim Ambiental
//! (`utsim_ambiental_core::camada3d`): a potência da fábrica num instante é o degrau da
//! curva de carga do app; aqui só se escolhe o bloco ambiental do modelo. O GDScript
//! (`godot/scripts/camada_ambiental.gd`) só desenha.

use std::path::Path;

use utsim_ambiental_core::Lateral;
use utsim_ambiental_core::camada3d::Camada3d;
use utsim_model::Model;

/// O bloco ambiental do modelo: o de dentro do modelo ou, sem ele, o arquivo lateral
/// `<modelo>.ambiental.ron` ao lado do arquivo. `None`: o modelo não tem bloco.
pub fn lateral(model: &Model, path: Option<&Path>) -> Option<Lateral> {
    if let Some(l) = Lateral::do_modelo(model) {
        return Some(l);
    }
    let p = path?;
    utsim_ambiental_core::caminho_lateral(p)
        .exists()
        .then(|| utsim_ambiental_core::ler_do_modelo(p).ok())
        .flatten()
}

/// A camada da replicação `replication` (roda a replicação de novo, com a mesma semente da
/// reprodução).
pub fn camada(model: &Model, path: Option<&Path>, replication: u32) -> Result<Camada3d, String> {
    let l = lateral(model, path).ok_or_else(|| "o modelo não tem bloco ambiental".to_string())?;
    Camada3d::calcular(model, &l, replication)
}

#[cfg(test)]
mod testes {
    use super::*;

    fn exemplo() -> Model {
        let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/linha_verde.ron");
        Model::load(&p).unwrap()
    }

    /// A camada roda a mesma replicação que a reprodução: mesmo relatório, e o medidor é a
    /// soma das colunas.
    #[test]
    fn mesma_replicacao_da_reproducao() {
        let m = exemplo();
        let c = camada(&m, None, 0).unwrap();
        let rod = utsim_ambiental_core::rodar::simular_uma(&m, 0).unwrap();
        let rep = m.replay(0).unwrap();
        assert_eq!(rod.relatorio.end_time, rep.report.end_time);
        for (a, b) in rod.relatorio.blocks.iter().zip(&rep.report.blocks) {
            assert_eq!(a.input, b.input, "{}", a.name);
            assert_eq!(a.states, b.states, "{}", a.name);
        }
        assert_eq!(c.nos.len(), 4);
        assert!(!c.refugo.is_empty());
        let t = (c.janela.0 + c.janela.1) / 2.0;
        let soma: f64 = c.potencias(t).iter().map(|x| x.0).sum();
        assert!((soma - c.potencia(t).unwrap()).abs() < 1e-9);
        // Relógio das 06:00 em minutos: 18:30 é o minuto 750, no pico; 11:00, fora.
        assert_eq!(c.em_pico(750.0), Some(true));
        assert_eq!(c.em_pico(300.0), Some(false));
    }

    #[test]
    fn modelo_sem_bloco_nao_tem_camada() {
        let m = Model::parse(
            r#"Model(name: "m", run: (stop_time: 100.0),
               objects: [(name: "a", kind: Source(interarrival: Const(5.0))), (name: "b", kind: Sink)],
               connections: [("a", "b")])"#,
        )
        .unwrap();
        assert!(lateral(&m, None).is_none());
        assert!(camada(&m, None, 0).is_err());
    }
}
