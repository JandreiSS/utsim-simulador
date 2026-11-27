//! What the simulation says about one node, shown at the top of its Qualidade, Segurança
//! and Manutenção tabs (front B of the family integration, 2026-11-09): the numbers the
//! dedicated apps compute, over the replications of the last run, with the half-width of
//! the 95 % interval.
//!
//! - Qualidade (`utsim_qualidade_core`, the same as UTSim Qualidade): defects the node
//!   generates, the ones that reach the customer, first-pass yield of an inspection,
//!   cost of quality by station, items a scrap sink received;
//! - Segurança (`utsim_model::exposure`, the same as UTSim Segurança do Trabalho): time
//!   of people in the zones where the node stands (or of the person, when the node is one),
//!   entries without the protective equipment, simulated near-conflicts close to it;
//! - Manutenção (the block's `maintenance.*` metrics, or its `down` state): breakdowns,
//!   MTBF, MTTR, time down, availability, cost.
//!
//! The quality and safety numbers need the per-item log and the trace, so they come from
//! one more run per replication ([`replica`]), done off the main thread by the bridge.

use crate::texto::tx;
use std::collections::BTreeMap;

use utsim_core::{ItemRecord, RunReport};
use utsim_edit::abas::Aba;
use utsim_model::exposure::{self, Exposure, Options, interval};
use utsim_model::quality::present_in;
use utsim_model::{Model, ModelError, ObjectKind, SinkRole};
use utsim_qualidade_core::custos;

use crate::ui_results::num;

/// Distance (m) from a fixed node within which a near-conflict counts as its own.
pub const RAIO: f64 = crate::seguranca::RAIO_POSTO;

/// One line of a tab: label, value, explanation.
#[derive(Clone, Debug, PartialEq)]
pub struct Linha {
    pub rotulo: String,
    pub valor: String,
    pub ajuda: String,
}

fn linha(rotulo: &str, valor: String, ajuda: &str) -> Linha {
    Linha {
        rotulo: rotulo.into(),
        valor,
        ajuda: ajuda.into(),
    }
}

/// `mean ± half-width` of the values (one per replication); `pct`: times 100, with `%`.
fn mh(values: &[f64], pct: bool) -> String {
    if values.is_empty() {
        return "-".into();
    }
    let i = interval(values);
    let (k, unit) = if pct { (100.0, " %") } else { (1.0, "") };
    if i.half_width.is_finite() {
        format!("{} ± {}{unit}", num(i.mean * k), num(i.half_width * k))
    } else {
        format!("{}{unit}", num(i.mean * k))
    }
}

/// Quality numbers of one replication, by node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Qualidade {
    defeitos: BTreeMap<String, f64>,
    escapes: BTreeMap<String, f64>,
    rendimento: BTreeMap<String, f64>,
    custo: BTreeMap<String, f64>,
}

impl Qualidade {
    pub fn de(model: &Model, items: &[ItemRecord]) -> Qualidade {
        let k = utsim_qualidade_core::simulado::de_itens(model, items);
        let c = custos::custos(model, items);
        // Defect type → the process that generates it.
        let mut origem: BTreeMap<&str, &str> = BTreeMap::new();
        for o in &model.objects {
            if let Some(q) = &o.quality {
                for d in &q.defects {
                    origem.insert(&d.name, &o.name);
                }
                for ch in &q.characteristics {
                    origem.insert(&ch.name, &o.name);
                }
            }
        }
        let entrega: Vec<usize> = model
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| {
                matches!(o.kind, ObjectKind::Sink) && o.sink_role != Some(SinkRole::Scrap)
            })
            .map(|(i, _)| i)
            .collect();
        let mut escapes = BTreeMap::new();
        for item in items {
            if item.ended_at.is_none() || !item.block.is_some_and(|b| entrega.contains(&b)) {
                continue;
            }
            for d in present_in(&item.labels) {
                if let Some(p) = origem.get(d.as_str()) {
                    *escapes.entry(p.to_string()).or_default() += 1.0;
                }
            }
        }
        Qualidade {
            defeitos: k.defeitos_por_posto,
            escapes,
            rendimento: k.rendimento_por_inspecao,
            custo: c.por_posto,
        }
    }
}

/// What one more run of a replication gives (only what the model needs).
#[derive(Clone, Debug, Default)]
pub struct Replica {
    pub qualidade: Option<Qualidade>,
    pub exposicao: Option<Exposure>,
}

/// Whether the model has quality blocks (`quality`, `inspection`, `rework`).
pub fn tem_qualidade(model: &Model) -> bool {
    model
        .objects
        .iter()
        .any(|o| o.quality.is_some() || o.inspection.is_some() || o.rework.is_some())
}

/// Whether there is anything to measure for safety: mobile resources (people and
/// vehicles) or zones.
pub fn tem_seguranca(model: &Model) -> bool {
    !model.zones.is_empty()
        || model
            .objects
            .iter()
            .any(|o| matches!(o.kind, ObjectKind::Transporter { .. }))
}

/// Runs replication `rep` again with the per-item log and the trace, as the model needs.
pub fn replica(model: &Model, rep: u32) -> Result<Replica, ModelError> {
    let (q, s) = (tem_qualidade(model), tem_seguranca(model));
    if !q && !s {
        return Ok(Replica::default());
    }
    model.validate()?;
    let mut sim = model.build(rep);
    if q {
        sim.enable_item_log();
    }
    if s {
        sim.enable_trace();
    }
    let report = model.drive(&mut sim)?;
    let qualidade = q.then(|| Qualidade::de(model, &sim.take_item_log()));
    let exposicao = s.then(|| {
        let tracks = exposure::tracks(model, &sim.take_trace(), report.end_time);
        exposure::measure(
            model,
            &tracks,
            rep,
            report.stats_from,
            report.end_time,
            Options::default(),
        )
    });
    Ok(Replica {
        qualidade,
        exposicao,
    })
}

/// The lines of every node, by tab, from the reports of a run and the extra runs done so
/// far (`reps` may be empty while they run: then only Manutenção has numbers).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Indicadores {
    /// Replications behind the numbers.
    pub replicacoes: usize,
    /// Whether the quality and safety numbers are still being computed.
    pub calculando: bool,
    por_no: BTreeMap<String, [Vec<Linha>; 4]>,
}

fn indice(a: Aba) -> usize {
    match a {
        Aba::Qualidade => 0,
        Aba::Seguranca => 1,
        Aba::Manutencao => 2,
        Aba::Ambiental => 3,
    }
}

impl Indicadores {
    /// The lines of `node` in tab `aba` (empty: nothing simulated to show).
    pub fn de(&self, node: &str, aba: Aba) -> &[Linha] {
        self.por_no
            .get(node)
            .map_or(&[], |t| t[indice(aba)].as_slice())
    }

    pub fn montar(model: &Model, reports: &[RunReport], reps: &[Replica]) -> Indicadores {
        let mut por_no = BTreeMap::new();
        for o in &model.objects {
            let t = [
                qualidade(model, &o.name, reports, reps),
                seguranca(model, &o.name, reps),
                manutencao(&o.name, reports),
                ambiental(model, &o.name, reports),
            ];
            if t.iter().any(|v| !v.is_empty()) {
                por_no.insert(o.name.clone(), t);
            }
        }
        Indicadores {
            replicacoes: reports.len(),
            calculando: false,
            por_no,
        }
    }
}

fn qualidade(model: &Model, name: &str, reports: &[RunReport], reps: &[Replica]) -> Vec<Linha> {
    let mut out = Vec::new();
    let Some(o) = model.objects.iter().find(|o| o.name == name) else {
        return out;
    };
    if matches!(o.kind, ObjectKind::Sink) && o.sink_role == Some(SinkRole::Scrap) {
        let v: Vec<f64> = reports
            .iter()
            .filter_map(|r| r.block(name).map(|b| b.input as f64))
            .collect();
        out.push(linha(
            tx("godot-indicadores-itens_refugados"),
            mh(&v, false),
            tx("godot-indicadores-itens_que_terminaram_neste_refugo"),
        ));
    }
    let qs: Vec<&Qualidade> = reps.iter().filter_map(|r| r.qualidade.as_ref()).collect();
    if qs.is_empty() {
        return out;
    }
    let col = |f: &dyn Fn(&Qualidade) -> Option<f64>| -> Vec<f64> {
        qs.iter().filter_map(|q| f(q)).collect()
    };
    if o.quality.is_some() {
        let g = col(&|q| Some(q.defeitos.get(name).copied().unwrap_or(0.0)));
        out.push(linha(
            tx("godot-indicadores-defeitos_gerados"),
            mh(&g, false),
            tx("godot-indicadores-defeitos_que_este_posto_gerou"),
        ));
        let e = col(&|q| Some(q.escapes.get(name).copied().unwrap_or(0.0)));
        out.push(linha(
            tx("godot-indicadores-defeitos_que_escaparam"),
            mh(&e, false),
            tx("godot-indicadores-defeitos_deste_posto_que_chegaram"),
        ));
    }
    if o.inspection.is_some() {
        let r = col(&|q| q.rendimento.get(name).copied());
        if !r.is_empty() {
            out.push(linha(
                tx("godot-indicadores-rendimento_de_primeira"),
                mh(&r, true),
                tx("godot-indicadores-itens_inspecionados_uma_so_vez"),
            ));
        }
    }
    let c = col(&|q| Some(q.custo.get(name).copied().unwrap_or(0.0)));
    if c.iter().any(|x| *x != 0.0) {
        out.push(linha(
            tx("godot-indicadores-custo_da_qualidade"),
            mh(&c, false),
            tx("godot-indicadores-avaliacao_falha_interna_e_falha"),
        ));
    }
    out
}

fn seguranca(model: &Model, name: &str, reps: &[Replica]) -> Vec<Linha> {
    let mut out = Vec::new();
    let Some(o) = model.objects.iter().find(|o| o.name == name) else {
        return out;
    };
    let exps: Vec<&Exposure> = reps.iter().filter_map(|r| r.exposicao.as_ref()).collect();
    if exps.is_empty() {
        return out;
    }
    let movel = matches!(o.kind, ObjectKind::Transporter { .. });
    let pos = [o.position[0], o.position[1]];
    // The zones that count: where a fixed node stands; every zone, for a person.
    let zonas: Vec<&str> = model
        .zones
        .iter()
        .filter(|z| !movel && exposure::contains(&z.polygon, pos))
        .map(|z| z.name.as_str())
        .collect();
    let conta = |z: &exposure::ZoneExposure| {
        if movel {
            z.person == name
        } else {
            zonas.contains(&z.zone.as_str())
        }
    };
    let pessoa = movel
        && exps
            .iter()
            .any(|e| e.zones.iter().any(|z| z.person == name));
    if !zonas.is_empty() || pessoa {
        let tempo: Vec<f64> = exps
            .iter()
            .map(|e| e.zones.iter().filter(|z| conta(z)).map(|z| z.time).sum())
            .collect();
        let sem: Vec<f64> = exps
            .iter()
            .map(|e| {
                e.zones
                    .iter()
                    .filter(|z| conta(z))
                    .map(|z| f64::from(z.entries_without_ppe))
                    .sum()
            })
            .collect();
        let (rot, ajuda) = if movel {
            (
                tx("godot-indicadores-tempo_em_zonas"),
                tx("godot-indicadores-tempo_desta_pessoa_dentro_das"),
            )
        } else {
            (
                tx("godot-indicadores-exposicao_nas_zonas"),
                tx("godot-indicadores-tempo_de_pessoas_dentro_das"),
            )
        };
        if !zonas.is_empty() {
            out.push(linha(
                tx("godot-indicadores-zonas"),
                zonas.join(", "),
                tx("godot-indicadores-zonas_de_risco_onde_o"),
            ));
        }
        out.push(linha(rot, mh(&tempo, false), ajuda));
        out.push(linha(
            tx("godot-indicadores-entradas_sem_epi"),
            mh(&sem, false),
            tx("godot-indicadores-entradas_na_zona_sem_o"),
        ));
    }
    let perto = |n: &exposure::NearMiss| {
        if movel {
            n.person == name || n.vehicle == name
        } else {
            (n.at[0] - pos[0]).hypot(n.at[1] - pos[1]) <= RAIO
        }
    };
    let q: Vec<f64> = exps
        .iter()
        .map(|e| e.near_misses.iter().filter(|n| perto(n)).count() as f64)
        .collect();
    let ha_pessoas = exps
        .iter()
        .any(|e| !e.zones.is_empty() || !e.near_misses.is_empty());
    if movel || q.iter().any(|x| *x > 0.0) || (ha_pessoas && !zonas.is_empty()) {
        out.push(linha(
            tx("godot-indicadores-quase_conflitos"),
            mh(&q, false),
            if movel {
                tx("godot-indicadores-pessoa_e_veiculo_mais_perto")
            } else {
                tx("godot-indicadores-pessoa_e_veiculo_mais_perto-2")
            },
        ));
    }
    out
}

/// The Ambiental tab: the node's energy in the statistics window from its power per state
/// group (the simple account of `utsim_model::environment`; UTSim Ambiental does the full
/// one), the share without added value and the energy per part.
fn ambiental(model: &Model, name: &str, reports: &[RunReport]) -> Vec<Linha> {
    use utsim_model::environment::{Grupo, energia_por_grupo};
    let Some(def) = model
        .objects
        .iter()
        .find(|o| o.name == name)
        .and_then(|o| o.environment.as_deref())
    else {
        return Vec::new();
    };
    let hu = model
        .environment
        .as_deref()
        .map_or(1.0 / 3600.0, |c| c.clock.time_unit.em_horas());
    let mut total = Vec::new();
    let mut sem = Vec::new();
    let mut por_peca = Vec::new();
    for r in reports {
        let Some(b) = r.block(name) else { continue };
        let g = energia_por_grupo(def, b, r.end_time - r.stats_from, hu);
        let t: f64 = g.iter().sum();
        total.push(t);
        sem.push(
            Grupo::TODOS
                .iter()
                .filter(|x| x.sem_valor())
                .map(|x| g[x.indice()])
                .sum::<f64>(),
        );
        if b.output > 0 {
            por_peca.push(t / b.output as f64);
        }
    }
    if total.is_empty() {
        return Vec::new();
    }
    let media = |v: &[f64]| v.iter().sum::<f64>() / v.len().max(1) as f64;
    let mut out = vec![linha(
        tx("godot-indicadores-energia"),
        format!("{:.1} kWh", media(&total)),
        tx("godot-indicadores-energia_ajuda"),
    )];
    let e = media(&total);
    if e > 0.0 {
        out.push(linha(
            tx("godot-indicadores-energia_sem_valor"),
            format!("{:.0} %", 100.0 * media(&sem) / e),
            tx("godot-indicadores-energia_sem_valor_ajuda"),
        ));
    }
    if !por_peca.is_empty() {
        out.push(linha(
            tx("godot-indicadores-kwh_por_peca"),
            format!("{:.3} kWh", media(&por_peca)),
            tx("godot-indicadores-kwh_por_peca_ajuda"),
        ));
    }
    out
}

fn manutencao(name: &str, reports: &[RunReport]) -> Vec<Linha> {
    let blocks: Vec<_> = reports.iter().filter_map(|r| r.block(name)).collect();
    if blocks.is_empty() {
        return Vec::new();
    }
    let custom = |k: &str| -> Option<Vec<f64>> {
        blocks
            .iter()
            .map(|b| b.custom.get(k).copied())
            .collect::<Option<Vec<f64>>>()
    };
    let frac = |b: &&utsim_core::BlockReport, s: &str| b.states.get(s).copied().unwrap_or(0.0);
    let mut out = Vec::new();
    if let Some(f) = custom("maintenance.failures") {
        out.push(linha(
            tx("godot-indicadores-quebras"),
            mh(&f, false),
            tx("godot-indicadores-falhas_do_posto_por_replicacao"),
        ));
        if let Some(v) = custom("maintenance.mtbf") {
            out.push(linha(
                "MTBF",
                mh(&v, false),
                tx("godot-indicadores-tempo_medio_entre_falhas_tempo"),
            ));
        }
        if let Some(v) = custom("maintenance.mttr") {
            out.push(linha(
                "MTTR",
                mh(&v, false),
                tx("godot-indicadores-tempo_medio_de_reparo_da"),
            ));
        }
        if let Some(v) = custom("maintenance.down_time") {
            out.push(linha(
                tx("godot-indicadores-tempo_parado"),
                mh(&v, false),
                tx("godot-indicadores-tempo_em_manutencao_corretiva_por"),
            ));
        }
        if let Some(v) = custom("oee.availability") {
            out.push(linha(
                tx("godot-indicadores-disponibilidade"),
                mh(&v, true),
                tx("godot-indicadores-tempo_produzindo_sobre_o_tempo"),
            ));
        }
        if let Some(v) = custom("maintenance.cost").filter(|v| v.iter().any(|x| *x != 0.0)) {
            out.push(linha(
                tx("godot-indicadores-custo_de_manutencao"),
                mh(&v, false),
                tx("godot-indicadores-tempo_parado_reparos_e_preventivas"),
            ));
        }
        return out;
    }
    // A breakdown without the maintenance block: only the time in the `down` state.
    let down: Vec<f64> = blocks.iter().map(|b| frac(b, "down")).collect();
    if down.iter().any(|x| *x > 0.0) {
        let disp: Vec<f64> = blocks
            .iter()
            .map(|b| {
                let planned = (1.0 - frac(b, "off_shift")).max(1e-9);
                (1.0 - frac(b, "down") / planned).clamp(0.0, 1.0)
            })
            .collect();
        out.push(linha(
            tx("godot-indicadores-tempo_parado"),
            mh(&down, true),
            tx("godot-indicadores-parcela_do_tempo_em_quebra"),
        ));
        out.push(linha(
            tx("godot-indicadores-disponibilidade"),
            mh(&disp, true),
            tx("godot-indicadores-tempo_fora_de_quebra_sobre"),
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modelo() -> Model {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../examples/inspecao_e_retrabalho.ron"
        );
        Model::load(path).expect(path)
    }

    #[test]
    fn mean_and_half_width() {
        assert_eq!(mh(&[], false), "-");
        assert_eq!(mh(&[2.0], false), "2");
        assert_eq!(mh(&[0.5], true), "50 %");
        assert!(mh(&[1.0, 3.0], false).starts_with("2 ± "));
    }

    #[test]
    fn a_quality_model_shows_its_numbers_on_the_nodes() {
        let mut m = modelo();
        m.run.replications = 2;
        let reports = m.run_replications().unwrap();
        // Before the extra runs: no quality numbers yet.
        let cedo = Indicadores::montar(&m, &reports, &[]);
        let posto = m
            .objects
            .iter()
            .find(|o| o.quality.is_some())
            .expect("posto com qualidade")
            .name
            .clone();
        assert!(cedo.de(&posto, Aba::Qualidade).is_empty());
        let reps: Vec<Replica> = reports
            .iter()
            .map(|r| replica(&m, r.replication).unwrap())
            .collect();
        let ind = Indicadores::montar(&m, &reports, &reps);
        let q = ind.de(&posto, Aba::Qualidade);
        assert!(
            q.iter()
                .any(|l| l.rotulo == "Defeitos gerados" && l.valor != "-"),
            "{q:?}"
        );
        if let Some(insp) = m.objects.iter().find(|o| o.inspection.is_some()) {
            assert!(
                ind.de(&insp.name, Aba::Qualidade)
                    .iter()
                    .any(|l| l.rotulo == "Rendimento de primeira"),
                "{:?}",
                ind.de(&insp.name, Aba::Qualidade)
            );
        }
        assert_eq!(ind.replicacoes, 2);
    }

    #[test]
    fn breakdowns_show_on_maintenance() {
        let mut m = modelo();
        m.run.replications = 2;
        let p = m
            .objects
            .iter()
            .position(|o| matches!(o.kind, ObjectKind::Processor { .. }))
            .unwrap();
        let name = m.objects[p].name.clone();
        m.objects[p].downtime =
            Some(ron::from_str("(mtbf: Exp(mean: 60.0), mttr: Exp(mean: 5.0))").unwrap());
        let reports = m.run_replications().unwrap();
        let ind = Indicadores::montar(&m, &reports, &[]);
        let l = ind.de(&name, Aba::Manutencao);
        assert!(l.iter().any(|l| l.rotulo == "Disponibilidade"), "{l:?}");
    }
}
