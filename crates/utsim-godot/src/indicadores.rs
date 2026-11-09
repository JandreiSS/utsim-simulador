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
    por_no: BTreeMap<String, [Vec<Linha>; 3]>,
}

fn indice(a: Aba) -> usize {
    match a {
        Aba::Qualidade => 0,
        Aba::Seguranca => 1,
        Aba::Manutencao => 2,
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
            "Itens refugados",
            mh(&v, false),
            "Itens que terminaram neste refugo, por replicação.",
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
            "Defeitos gerados",
            mh(&g, false),
            "Defeitos que este posto gerou, por replicação.",
        ));
        let e = col(&|q| Some(q.escapes.get(name).copied().unwrap_or(0.0)));
        out.push(linha(
            "Defeitos que escaparam",
            mh(&e, false),
            "Defeitos deste posto que chegaram ao cliente (falha externa), por replicação.",
        ));
    }
    if o.inspection.is_some() {
        let r = col(&|q| q.rendimento.get(name).copied());
        if !r.is_empty() {
            out.push(linha(
                "Rendimento de primeira",
                mh(&r, true),
                "Itens inspecionados uma só vez e entregues, sobre os inspecionados.",
            ));
        }
    }
    let c = col(&|q| Some(q.custo.get(name).copied().unwrap_or(0.0)));
    if c.iter().any(|x| *x != 0.0) {
        out.push(linha(
            "Custo da qualidade",
            mh(&c, false),
            "Avaliação, falha interna e falha externa atribuídas a este posto, por replicação.",
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
    let pessoa = movel && exps.iter().any(|e| e.zones.iter().any(|z| z.person == name));
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
                "Tempo em zonas",
                "Tempo desta pessoa dentro das zonas de risco, por replicação.",
            )
        } else {
            (
                "Exposição nas zonas",
                "Tempo de pessoas dentro das zonas onde este posto está, somado, por replicação.",
            )
        };
        if !zonas.is_empty() {
            out.push(linha("Zonas", zonas.join(", "), "Zonas de risco onde o posto está."));
        }
        out.push(linha(rot, mh(&tempo, false), ajuda));
        out.push(linha(
            "Entradas sem EPI",
            mh(&sem, false),
            "Entradas na zona sem o equipamento de proteção que ela pede, por replicação.",
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
            "Quase-conflitos",
            mh(&q, false),
            if movel {
                "Pessoa e veículo mais perto que o limiar, envolvendo este recurso, por replicação. Simulado: serve para comparar layouts, não estima acidentes reais."
            } else {
                "Pessoa e veículo mais perto que o limiar a até 3 m deste posto, por replicação. Simulado: serve para comparar layouts, não estima acidentes reais."
            },
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
        out.push(linha("Quebras", mh(&f, false), "Falhas do posto, por replicação."));
        if let Some(v) = custom("maintenance.mtbf") {
            out.push(linha(
                "MTBF",
                mh(&v, false),
                "Tempo médio entre falhas (tempo em pé sobre o número de falhas).",
            ));
        }
        if let Some(v) = custom("maintenance.mttr") {
            out.push(linha(
                "MTTR",
                mh(&v, false),
                "Tempo médio de reparo, da quebra até voltar a produzir.",
            ));
        }
        if let Some(v) = custom("maintenance.down_time") {
            out.push(linha(
                "Tempo parado",
                mh(&v, false),
                "Tempo em manutenção corretiva, por replicação.",
            ));
        }
        if let Some(v) = custom("oee.availability") {
            out.push(linha(
                "Disponibilidade",
                mh(&v, true),
                "Tempo produzindo sobre o tempo planejado (sem turno e preventivas).",
            ));
        }
        if let Some(v) = custom("maintenance.cost").filter(|v| v.iter().any(|x| *x != 0.0)) {
            out.push(linha(
                "Custo de manutenção",
                mh(&v, false),
                "Tempo parado, reparos e preventivas pelos custos do bloco, por replicação.",
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
            "Tempo parado",
            mh(&down, true),
            "Parcela do tempo em quebra, por replicação.",
        ));
        out.push(linha(
            "Disponibilidade",
            mh(&disp, true),
            "Tempo fora de quebra sobre o tempo planejado (sem turno).",
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
        let reps: Vec<Replica> = reports.iter().map(|r| replica(&m, r.replication).unwrap()).collect();
        let ind = Indicadores::montar(&m, &reports, &reps);
        let q = ind.de(&posto, Aba::Qualidade);
        assert!(
            q.iter().any(|l| l.rotulo == "Defeitos gerados" && l.valor != "-"),
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
        m.objects[p].downtime = Some(
            ron::from_str("(mtbf: Exp(mean: 60.0), mttr: Exp(mean: 5.0))").unwrap(),
        );
        let reports = m.run_replications().unwrap();
        let ind = Indicadores::montar(&m, &reports, &[]);
        let l = ind.de(&name, Aba::Manutencao);
        assert!(
            l.iter().any(|l| l.rotulo == "Disponibilidade"),
            "{l:?}"
        );
    }
}
