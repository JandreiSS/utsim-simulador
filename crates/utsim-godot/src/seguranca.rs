//! Camada de seguranca e qualidade do cliente 3D (etapa QS6 de
//! `docs/plano_qualidade_e_seguranca.md`), sem tipos do Godot: toda a conta fica aqui e o
//! GDScript (`godot/scripts/camada_seguranca.gd`) so desenha.
//!
//! - zonas de risco no piso: cor pelo nivel da matriz ([`level_color`]) e poligono;
//! - EPI de cada pessoa no tempo: o EPI proprio mais o do posto que ela esta atendendo,
//!   trocado quando ela chega a outro posto ([`Timeline`]);
//! - pessoa dentro de uma zona sem o EPI que a zona pede ([`Seguranca::status`]);
//! - quase-conflitos simulados e mapa de calor (`utsim_exposicao`);
//! - itens com defeito (`defeitos > 0` no rotulo final do item) e postos com inspecao.
//!
//! Os quase-conflitos e a exposicao sao simulados: servem para comparar layouts, nao
//! estimam a probabilidade de um acidente real.

use std::collections::BTreeSet;

use utsim_exposicao::{Exposure, NearMiss, Options, Track, contains};
use utsim_model::{Model, ModelError, ZoneKind};

/// Distancia (m) ate o posto em que a pessoa conta como atendendo-o.
pub const RAIO_POSTO: f64 = 3.0;

/// Cor da zona pelo nivel da matriz de risco: 1 verde, 2 verde-amarelado, 3 amarelo,
/// 4 laranja, 5 ou mais vermelho. Sem nivel, um azul neutro.
pub fn level_color(level: Option<u8>) -> [f32; 3] {
    match level {
        None | Some(0) => [0.35, 0.55, 0.85],
        Some(1) => [0.18, 0.72, 0.30],
        Some(2) => [0.60, 0.80, 0.20],
        Some(3) => [0.95, 0.80, 0.15],
        Some(4) => [0.96, 0.52, 0.10],
        Some(_) => [0.88, 0.14, 0.14],
    }
}

/// Nome do tipo de zona, para o rotulo.
pub fn kind_name(kind: ZoneKind) -> &'static str {
    match kind {
        ZoneKind::Vehicle => "Veículos",
        ZoneKind::Machine => "Máquina",
        ZoneKind::SuspendedLoad => "Carga suspensa",
        ZoneKind::Noise => "Ruído",
        ZoneKind::Heat => "Calor",
        _ => "Outro",
    }
}

/// Uma zona pronta para desenhar.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneView {
    pub name: String,
    pub kind: &'static str,
    pub level: Option<u8>,
    pub color: [f32; 3],
    pub ppe: Vec<String>,
    /// Vertices no plano do modelo, em metros.
    pub polygon: Vec<[f64; 2]>,
    /// Onde pôr o nome (centroide do poligono).
    pub center: [f64; 2],
}

/// Centroide de um poligono (media ponderada pela area; media dos vertices quando a area
/// e nula).
pub fn polygon_center(polygon: &[[f64; 2]]) -> [f64; 2] {
    let n = polygon.len();
    if n == 0 {
        return [0.0, 0.0];
    }
    let (mut a2, mut cx, mut cy) = (0.0, 0.0, 0.0);
    for i in 0..n {
        let (p, q) = (polygon[i], polygon[(i + 1) % n]);
        let cross = p[0] * q[1] - q[0] * p[1];
        a2 += cross;
        cx += (p[0] + q[0]) * cross;
        cy += (p[1] + q[1]) * cross;
    }
    if a2.abs() < 1e-9 {
        let k = n as f64;
        return [
            polygon.iter().map(|p| p[0]).sum::<f64>() / k,
            polygon.iter().map(|p| p[1]).sum::<f64>() / k,
        ];
    }
    [cx / (3.0 * a2), cy / (3.0 * a2)]
}

/// As zonas do modelo prontas para desenhar.
pub fn zone_views(model: &Model) -> Vec<ZoneView> {
    model
        .zones
        .iter()
        .filter(|z| z.polygon.len() >= 3)
        .map(|z| ZoneView {
            name: z.name.clone(),
            kind: kind_name(z.kind),
            level: z.level,
            color: level_color(z.level),
            ppe: z.ppe.clone(),
            polygon: z.polygon.clone(),
            center: polygon_center(&z.polygon),
        })
        .collect()
}

/// O EPI de uma pessoa ao longo do tempo: o proprio mais o do posto atendido.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Timeline {
    /// EPI do proprio objeto da pessoa (vale sempre).
    pub own: BTreeSet<String>,
    /// `(instante de chegada, posto, EPI do posto)`, em ordem de tempo.
    pub stations: Vec<(f64, String, BTreeSet<String>)>,
}

impl Timeline {
    /// O posto atendido e o EPI vestido no instante `t` (o proprio mais o do ultimo posto a
    /// que a pessoa chegou ate `t`).
    pub fn at(&self, t: f64) -> (Option<&str>, BTreeSet<String>) {
        let k = self.stations.partition_point(|(t0, _, _)| *t0 <= t);
        let mut ppe = self.own.clone();
        if k == 0 {
            return (None, ppe);
        }
        let (_, name, extra) = &self.stations[k - 1];
        ppe.extend(extra.iter().cloned());
        (Some(name.as_str()), ppe)
    }
}

/// A linha do tempo de EPI de uma pessoa: a cada parada, o posto atendido mais proximo
/// (ate [`RAIO_POSTO`]) entre os que ela atende. So grava quando o posto muda.
pub fn timeline(model: &Model, track: &Track) -> Timeline {
    let obj = &model.objects[track.object];
    let served = model.served_by(&obj.name);
    let mut out = Timeline {
        own: obj.ppe.iter().cloned().collect(),
        stations: Vec::new(),
    };
    for m in &track.moves {
        let near = served
            .iter()
            .map(|s| {
                let d =
                    ((s.position[0] - m.p1[0]).powi(2) + (s.position[1] - m.p1[1]).powi(2)).sqrt();
                (d, *s)
            })
            .filter(|(d, _)| *d <= RAIO_POSTO)
            .min_by(|a, b| a.0.total_cmp(&b.0));
        let Some((_, s)) = near else { continue };
        if out.stations.last().is_some_and(|(_, n, _)| *n == s.name) {
            continue;
        }
        out.stations
            .push((m.t1, s.name.clone(), s.ppe.iter().cloned().collect()));
    }
    out
}

/// A zona em que o ponto `p` esta e o EPI que ela pede e a pessoa nao veste: a primeira
/// zona com falta, `None` se nenhuma zona onde ela esta pede algo que lhe falta.
pub fn missing_in_zones(
    zones: &[ZoneView],
    p: [f64; 2],
    ppe: &BTreeSet<String>,
) -> Option<(String, Vec<String>)> {
    for z in zones {
        if !contains(&z.polygon, p) {
            continue;
        }
        let missing: Vec<String> = z
            .ppe
            .iter()
            .filter(|e| !ppe.contains(*e))
            .cloned()
            .collect();
        if !missing.is_empty() {
            return Some((z.name.clone(), missing));
        }
    }
    None
}

/// O estado de seguranca de uma pessoa num instante.
#[derive(Clone, Debug, PartialEq)]
pub struct PersonStatus {
    pub name: String,
    pub station: Option<String>,
    pub ppe: Vec<String>,
    /// Zona onde esta sem o EPI pedido (vazio: em regra).
    pub zone: String,
    pub missing: Vec<String>,
}

/// Tudo da camada de uma replicacao.
pub struct Seguranca {
    pub zones: Vec<ZoneView>,
    people: Vec<(Track, Timeline)>,
    pub exposure: Exposure,
    /// Chaves (`item_id` dos quadros) dos itens que terminaram com `defeitos > 0`.
    pub defective: Vec<i64>,
    /// Postos com bloco de inspecao.
    pub inspections: Vec<String>,
}

impl Seguranca {
    pub fn new(model: &Model, replication: u32) -> Result<Self, ModelError> {
        let replay = model.replay(replication)?;
        let stop = replay.report.end_time;
        let tracks = utsim_exposicao::tracks(model, &replay.records, stop);
        let exposure = utsim_exposicao::measure(
            model,
            &tracks,
            replication,
            replay.report.stats_from,
            stop,
            Options::default(),
        );
        let people = tracks
            .into_iter()
            .filter(Track::is_person)
            .map(|t| {
                let tl = timeline(model, &t);
                (t, tl)
            })
            .collect();
        let (_, items) = model.run_item_log(replication)?;
        let defective = defective_keys(&items);
        Ok(Seguranca {
            zones: zone_views(model),
            people,
            exposure,
            defective,
            inspections: model
                .objects
                .iter()
                .filter(|o| o.inspection.is_some())
                .map(|o| o.name.clone())
                .collect(),
        })
    }

    /// Nomes das pessoas.
    pub fn people(&self) -> Vec<String> {
        self.people.iter().map(|(t, _)| t.name.clone()).collect()
    }

    /// O estado de cada pessoa no instante `t`.
    pub fn status(&self, t: f64) -> Vec<PersonStatus> {
        self.people
            .iter()
            .map(|(track, tl)| {
                let (station, ppe) = tl.at(t);
                let (zone, missing) =
                    missing_in_zones(&self.zones, track.at(t), &ppe).unwrap_or_default();
                PersonStatus {
                    name: track.name.clone(),
                    station: station.map(str::to_string),
                    ppe: ppe.into_iter().collect(),
                    zone,
                    missing,
                }
            })
            .collect()
    }

    pub fn near_misses(&self) -> &[NearMiss] {
        &self.exposure.near_misses
    }
}

/// Chaves dos itens cujo rotulo final `defeitos` e maior que zero.
pub fn defective_keys(items: &[utsim_core::ItemRecord]) -> Vec<i64> {
    items
        .iter()
        .filter(|r| {
            r.labels
                .get("defeitos")
                .and_then(|v| v.as_num())
                .is_some_and(|n| n > 0.0)
        })
        .map(|r| r.key as i64)
        .collect()
}

/// Mapa de calor normalizado: `(origem, lado da celula, nx, ny, valores de 0 a 1)`. O
/// valor mistura a presenca de pessoas (relativa ao maximo) e soma 1 onde houve
/// quase-conflito (limitado a 1).
pub fn heat_values(exposure: &Exposure) -> Vec<f32> {
    let h = &exposure.heat;
    let max = h.presence.iter().cloned().fold(0.0f64, f64::max);
    h.presence
        .iter()
        .zip(&h.near_misses)
        .map(|(p, n)| {
            let base = if max > 0.0 { p / max } else { 0.0 };
            ((base * 0.7) + if *n > 0 { 0.3 } else { 0.0 }).clamp(0.0, 1.0) as f32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODELO: &str = r#"Model(
        name: "t",
        run: (stop_time: 600.0, replications: 1),
        zones: [
            (name: "prensa_area", kind: Machine, level: Some(4),
             polygon: [(8.0, -4.0), (16.0, -4.0), (16.0, 4.0), (8.0, 4.0)],
             ppe: ["capacete", "oculos"]),
        ],
        objects: [],
        connections: [],
    )"#;

    fn modelo() -> Model {
        Model::parse(MODELO).expect("modelo valido")
    }

    fn set(v: &[&str]) -> BTreeSet<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_cor_vai_do_verde_ao_vermelho() {
        let cores: Vec<[f32; 3]> = (1..=5).map(|n| level_color(Some(n))).collect();
        assert!(cores[0][1] > cores[0][0], "nível 1 verde");
        assert!(cores[4][0] > 0.8 && cores[4][1] < 0.3, "nível 5 vermelho");
        // O verde some e o vermelho cresce ao longo da escala.
        for w in cores.windows(2) {
            assert_ne!(w[0], w[1]);
        }
        assert!(cores[3][0] > cores[0][0]);
        assert_eq!(level_color(Some(9)), level_color(Some(5)));
        assert_eq!(level_color(None), level_color(Some(0)));
        for c in &cores {
            assert!(c.iter().all(|v| (0.0..=1.0).contains(v)));
        }
    }

    #[test]
    fn centroide_do_retangulo() {
        let c = polygon_center(&[[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
        assert!((c[0] - 2.0).abs() < 1e-9 && (c[1] - 1.0).abs() < 1e-9);
        assert_eq!(polygon_center(&[]), [0.0, 0.0]);
        // Poligono degenerado (colinear): media dos vertices, sem NaN.
        let d = polygon_center(&[[0.0, 0.0], [2.0, 0.0], [4.0, 0.0]]);
        assert!(d[0].is_finite() && (d[0] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn zonas_do_modelo_viram_vistas() {
        let v = zone_views(&modelo());
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].kind, "Máquina");
        assert_eq!(v[0].color, level_color(Some(4)));
        assert!((v[0].center[0] - 12.0).abs() < 1e-9);
    }

    #[test]
    fn o_epi_efetivo_troca_ao_chegar_ao_posto() {
        let tl = Timeline {
            own: set(&["botina"]),
            stations: vec![
                (10.0, "prensa".into(), set(&["oculos", "luvas"])),
                (50.0, "solda".into(), set(&["mascara"])),
            ],
        };
        let (posto, ppe) = tl.at(5.0);
        assert_eq!(posto, None);
        assert_eq!(ppe, set(&["botina"]));
        let (posto, ppe) = tl.at(10.0);
        assert_eq!(posto, Some("prensa"));
        assert_eq!(ppe, set(&["botina", "oculos", "luvas"]));
        let (posto, ppe) = tl.at(60.0);
        assert_eq!(posto, Some("solda"));
        assert_eq!(ppe, set(&["botina", "mascara"]));
    }

    #[test]
    fn pessoa_na_zona_sem_o_epi_pedido() {
        let zonas = zone_views(&modelo());
        let dentro = [12.0, 0.0];
        let fora = [0.0, 0.0];
        // Sem nada: faltam os dois.
        let (zona, falta) = missing_in_zones(&zonas, dentro, &set(&[])).unwrap();
        assert_eq!(zona, "prensa_area");
        assert_eq!(falta, vec!["capacete".to_string(), "oculos".to_string()]);
        // Com um: falta o outro.
        let (_, falta) = missing_in_zones(&zonas, dentro, &set(&["capacete"])).unwrap();
        assert_eq!(falta, vec!["oculos".to_string()]);
        // Completa: em regra. Fora da zona: em regra mesmo sem EPI.
        assert!(missing_in_zones(&zonas, dentro, &set(&["capacete", "oculos", "luvas"])).is_none());
        assert!(missing_in_zones(&zonas, fora, &set(&[])).is_none());
    }

    #[test]
    fn itens_com_defeito_pelo_rotulo_final() {
        use utsim_core::ItemRecord;
        use utsim_core::item::Value;
        let mut a = ItemRecord {
            key: 7,
            ..ItemRecord::default()
        };
        a.labels.insert("defeitos".into(), Value::Num(2.0));
        let mut b = ItemRecord {
            key: 8,
            ..ItemRecord::default()
        };
        b.labels.insert("defeitos".into(), Value::Num(0.0));
        let c = ItemRecord::default();
        assert_eq!(defective_keys(&[a, b, c]), vec![7]);
    }

    /// Ponta a ponta no exemplo de empilhadeira e pessoa: nada de NaN, o calor fica em
    /// 0..1 e o estado cobre todas as pessoas.
    #[test]
    fn exemplo_cruzamento_de_ponta_a_ponta() {
        let caminho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/cruzamento_empilhadeira.ron");
        let model = Model::load(&caminho).expect("exemplo");
        model.validate().expect("valido");
        let s = Seguranca::new(&model, 0).expect("camada");
        assert!(!s.zones.is_empty());
        assert!(!s.people().is_empty());
        for t in [0.0, 30.0, 120.0, 599.0] {
            let st = s.status(t);
            assert_eq!(st.len(), s.people().len());
            for p in &st {
                assert_eq!(p.zone.is_empty(), p.missing.is_empty());
            }
        }
        let calor = heat_values(&s.exposure);
        assert!(
            calor
                .iter()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
        );
        for n in s.near_misses() {
            assert!(n.at[0].is_finite() && n.at[1].is_finite() && n.t >= 0.0);
        }
    }
}
