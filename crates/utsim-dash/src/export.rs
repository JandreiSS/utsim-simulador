//! Exports of the dashboard: CSV (always), JSON, `.xlsx` (feature `xlsx`) and, in
//! [`crate::page`], the frozen HTML page.

use utsim_model::csv::{num, row};

use crate::data::Package;
use crate::kpi::Situation;

fn situation_text(s: Situation) -> &'static str {
    match s {
        Situation::Verde => "verde",
        Situation::Amarelo => "amarelo",
        Situation::Vermelho => "vermelho",
        Situation::SemMeta => "sem meta",
        Situation::Indefinido => "indefinido",
    }
}

const KPI_HEAD: [&str; 18] = [
    "id",
    "nome",
    "unidade",
    "n",
    "media",
    "ic_inferior",
    "ic_superior",
    "p5",
    "p25",
    "p50",
    "p75",
    "p95",
    "meta_sentido",
    "meta",
    "prob_cumprir",
    "prob_inferior",
    "prob_superior",
    "situacao",
];

/// One line per KPI: mean, confidence interval, percentiles, goal, probability, light.
pub fn kpis_csv(p: &Package) -> String {
    let mut out = KPI_HEAD.join(",");
    out.push('\n');
    for k in &p.kpis.kpis {
        let ic = k.ic.unwrap_or([f64::NAN; 2]);
        let (sentido, meta) = k.meta.as_ref().map_or((String::new(), f64::NAN), |m| {
            (m.sentido.to_string(), m.valor)
        });
        let (pp, plo, phi) = k
            .prob
            .as_ref()
            .map_or((f64::NAN, f64::NAN, f64::NAN), |q| (q.p, q.lo, q.hi));
        row(
            &mut out,
            &[
                k.id.clone(),
                k.nome.clone(),
                k.unidade.clone(),
                k.n.to_string(),
                num(k.valor),
                num(ic[0]),
                num(ic[1]),
                num(k.p5),
                num(k.p25),
                num(k.p50),
                num(k.p75),
                num(k.p95),
                sentido,
                num(meta),
                num(pp),
                num(plo),
                num(phi),
                situation_text(k.situacao).to_string(),
            ],
        );
    }
    out
}

/// The table of objects, the same CSV `utsim run --csv` writes.
pub fn objects_csv(p: &Package) -> String {
    utsim_model::csv::objects_csv(&p.stats)
}

/// Replication by replication: the value of every KPI in each one.
pub fn replications_csv(p: &Package) -> String {
    let mut out = String::from("replicacao");
    for k in &p.kpis.kpis {
        out.push(',');
        out.push_str(&utsim_model::csv::field(&k.id));
    }
    out.push('\n');
    let n = p
        .kpis
        .kpis
        .iter()
        .map(|k| k.valores.len())
        .max()
        .unwrap_or(0);
    for r in 0..n {
        let mut fields = vec![r.to_string()];
        fields.extend(
            p.kpis
                .kpis
                .iter()
                .map(|k| num(k.valores.get(r).copied().unwrap_or(f64::NAN))),
        );
        row(&mut out, &fields);
    }
    out
}

/// Every route of the API in one JSON object (the page embeds the same map).
pub fn json(p: &Package) -> String {
    serde_json::to_string_pretty(&p.routes).unwrap_or_default()
}

/// A workbook with one sheet per table: summary, KPIs, objects, replications, series.
#[cfg(feature = "xlsx")]
pub fn xlsx(p: &Package) -> Result<Vec<u8>, String> {
    use rust_xlsxwriter::{Format, Workbook, Worksheet, XlsxError};

    fn go(p: &Package) -> Result<Vec<u8>, XlsxError> {
        let bold = Format::new().set_bold();
        let mut book = Workbook::new();

        let sheet = book.add_worksheet();
        sheet.set_name("Resumo")?;
        sheet.write_string_with_format(0, 0, &p.title, &bold)?;
        let sentence = p.routes["/api/resumo"]["frase"].as_str().unwrap_or("");
        sheet.write_string(2, 0, sentence)?;
        sheet.set_column_width(0, 100)?;

        let sheet = book.add_worksheet();
        sheet.set_name("KPIs")?;
        for (c, h) in KPI_HEAD.iter().enumerate() {
            sheet.write_string_with_format(0, c as u16, *h, &bold)?;
        }
        for (i, k) in p.kpis.kpis.iter().enumerate() {
            let r = i as u32 + 1;
            let ic = k.ic.unwrap_or([f64::NAN; 2]);
            let texts = [(0u16, &k.id), (1, &k.nome), (2, &k.unidade)];
            for (c, t) in texts {
                sheet.write_string(r, c, t)?;
            }
            let (sentido, meta) = k
                .meta
                .as_ref()
                .map_or(("", f64::NAN), |m| (m.sentido, m.valor));
            let (pp, plo, phi) = k
                .prob
                .as_ref()
                .map_or((f64::NAN, f64::NAN, f64::NAN), |q| (q.p, q.lo, q.hi));
            let nums = [
                (3u16, k.n as f64),
                (4, k.valor),
                (5, ic[0]),
                (6, ic[1]),
                (7, k.p5),
                (8, k.p25),
                (9, k.p50),
                (10, k.p75),
                (11, k.p95),
                (13, meta),
                (14, pp),
                (15, plo),
                (16, phi),
            ];
            for (c, v) in nums {
                if v.is_finite() {
                    sheet.write_number(r, c, v)?;
                }
            }
            sheet.write_string(r, 12, sentido)?;
            sheet.write_string(r, 17, situation_text(k.situacao))?;
        }
        sheet.set_column_width(1, 34)?;

        let sheet = book.add_worksheet();
        sheet.set_name("Objetos")?;
        let head = [
            "objeto",
            "tipo",
            "entradas",
            "saidas",
            "conteudo_medio",
            "permanencia_media",
            "ocupacao",
            "ocupacao_no_turno",
        ];
        for (c, h) in head.iter().enumerate() {
            sheet.write_string_with_format(0, c as u16, *h, &bold)?;
        }
        for (i, s) in p.stats.iter().enumerate() {
            let r = i as u32 + 1;
            sheet.write_string(r, 0, &s.name)?;
            sheet.write_string(r, 1, &s.kind)?;
            let cells = [
                (2u16, Some(s.input.mean)),
                (3, Some(s.output.mean)),
                (4, Some(s.content.mean)),
                (5, Some(s.staytime.mean)),
                (6, s.utilization.map(|u| u.mean)),
                (7, s.utilization_in_shift.map(|u| u.mean)),
            ];
            for (c, v) in cells {
                if let Some(v) = v.filter(|v| v.is_finite()) {
                    sheet.write_number(r, c, v)?;
                }
            }
        }
        sheet.set_column_width(0, 24)?;

        let sheet = book.add_worksheet();
        sheet.set_name("Replicações")?;
        sheet.write_string_with_format(0, 0, "replicacao", &bold)?;
        for (c, k) in p.kpis.kpis.iter().enumerate() {
            sheet.write_string_with_format(0, c as u16 + 1, &k.id, &bold)?;
        }
        let n = p
            .kpis
            .kpis
            .iter()
            .map(|k| k.valores.len())
            .max()
            .unwrap_or(0);
        for r in 0..n {
            sheet.write_number(r as u32 + 1, 0, r as f64)?;
            for (c, k) in p.kpis.kpis.iter().enumerate() {
                if let Some(v) = k.valores.get(r).filter(|v| v.is_finite()) {
                    sheet.write_number(r as u32 + 1, c as u16 + 1, *v)?;
                }
            }
        }

        let sheet: &mut Worksheet = book.add_worksheet();
        sheet.set_name("Série WIP")?;
        for (c, h) in ["tempo", "media", "ic_inferior", "ic_superior", "p5", "p95"]
            .iter()
            .enumerate()
        {
            sheet.write_string_with_format(0, c as u16, *h, &bold)?;
        }
        for (i, pt) in p.wip_series.iter().enumerate() {
            for (c, v) in pt.iter().enumerate() {
                sheet.write_number(i as u32 + 1, c as u16, *v)?;
            }
        }
        book.save_to_buffer()
    }
    go(p).map_err(|e| format!("não foi possível gerar a planilha: {e}"))
}

#[cfg(not(feature = "xlsx"))]
pub fn xlsx(_: &Package) -> Result<Vec<u8>, String> {
    Err("esta versão foi compilada sem a planilha (.xlsx); use o CSV".to_string())
}
