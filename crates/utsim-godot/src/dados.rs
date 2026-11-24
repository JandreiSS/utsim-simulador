//! Real data in the distribution fields (G3): what the properties panel needs to switch a
//! field between a distribution and observed values. No Godot types.
//!
//! - [`input_from_config`] turns the panel's choices (file, column, pasted values,
//!   instants, family, criterion) into a `DataInput`;
//! - [`fit_cached`] runs the automatic fit and keeps the outcome (record and the view the
//!   panel draws) in a [`FitCache`], so applying, undoing and redoing never fit twice;
//! - [`data_fields`] lists every distribution field of a model with its source;
//! - [`write_report`] writes the full HTML reports and the index of a model.

use crate::texto::{fx, tx, vx};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};
use utsim_fit::auto::{self, AutoFit, Choice, Criterion};
use utsim_fit::data::DataOptions;
use utsim_model::Model;
use utsim_model::dados::{self, DataInput, DataStatus, FittedRecord};

/// What one fit produced: the record kept in the model and the view of the panel.
pub struct CachedFit {
    pub record: FittedRecord,
    pub view: Value,
}

/// Fits already made, by field, data hash and settings. Shared with the thread.
#[derive(Clone, Default)]
pub struct FitCache(Arc<Mutex<HashMap<String, Arc<CachedFit>>>>);

impl FitCache {
    pub fn get(&self, key: &str) -> Option<Arc<CachedFit>> {
        self.0.lock().ok()?.get(key).cloned()
    }

    fn put(&self, key: String, fit: Arc<CachedFit>) {
        if let Ok(mut m) = self.0.lock() {
            m.insert(key, fit);
        }
    }
}

/// Cache key of an entry and its values.
pub fn cache_key(input: &DataInput, values: &[f64]) -> String {
    format!(
        "{}|{}|{}",
        input.field,
        auto::data_hash(values),
        input.settings_key()
    )
}

fn criterion_of(text: &str) -> Result<Criterion, String> {
    match text.to_lowercase().as_str() {
        "" | "aic" => Ok(Criterion::Aic),
        "bic" => Ok(Criterion::Bic),
        "ks" => Ok(Criterion::Ks),
        "ad" => Ok(Criterion::Ad),
        other => Err(fx(
            "godot-dados-criterio_desconhecido",
            &[("other", vx(&other))],
        )),
    }
}

/// Key of a criterion in the config.
pub fn criterion_key(c: Criterion) -> &'static str {
    match c {
        Criterion::Aic => "aic",
        Criterion::Bic => "bic",
        Criterion::Ks => "ks",
        Criterion::Ad => "ad",
    }
}

/// The entry for `field` from the panel's choices: `source` (file, `""` for none),
/// `column` (`""` = automatic), `timestamps`, `values` (numbers), `choose` (`"Best"`,
/// `"Empirical"` or a family name) and `criterion` (`aic`, `bic`, `ks`, `ad`).
pub fn input_from_config(field: &str, config: &Value) -> Result<DataInput, String> {
    let text = |k: &str| {
        config
            .get(k)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    };
    let values: Vec<f64> = config
        .get("values")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_f64).collect())
        .unwrap_or_default();
    let source = text("source");
    if source.is_some() && !values.is_empty() {
        return Err(tx("godot-dados-use_um_arquivo_ou_valores").into());
    }
    if source.is_none() && values.is_empty() {
        return Err(tx("godot-dados-escolha_um_arquivo_de_dados").into());
    }
    let mut input = DataInput::with_values(field, values);
    input.source = source;
    input.column = text("column");
    input.timestamps = config
        .get("timestamps")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    input.choose = match text("choose").as_deref() {
        None | Some("Best") => Choice::Best,
        Some("Empirical") => Choice::Empirical,
        Some(family) => Choice::Family(family.to_string()),
    };
    input.criterion = criterion_of(&text("criterion").unwrap_or_default())?;
    Ok(input)
}

/// The panel's choices of an existing entry (the inverse of [`input_from_config`]).
pub fn config_of(input: &DataInput) -> Value {
    json!({
        "source": input.source.clone().unwrap_or_default(),
        "column": input.column.clone().unwrap_or_default(),
        "timestamps": input.timestamps,
        "values": input.values,
        "choose": match &input.choose {
            Choice::Best => "Best".to_string(),
            Choice::Empirical => "Empirical".to_string(),
            Choice::Family(f) => f.clone(),
        },
        "criterion": criterion_key(input.criterion),
    })
}

fn bins_for(n: usize) -> usize {
    (((n.max(2) as f64).log2().ceil() as usize) + 1).clamp(5, 30)
}

/// Histogram as densities plus the density of the chosen family over the same range.
fn chart(fit: &AutoFit) -> Value {
    let data = &fit.report.data;
    let (lo, hi) = (fit.support.min, fit.support.max);
    let span = if hi > lo { hi - lo } else { 1.0 };
    let bins = bins_for(data.len());
    let width = span / bins as f64;
    let mut counts = vec![0usize; bins];
    for &x in data {
        let k = (((x - lo) / width).floor() as usize).min(bins - 1);
        counts[k] += 1;
    }
    let n = data.len().max(1) as f64;
    let density: Vec<f64> = counts.iter().map(|&c| c as f64 / (n * width)).collect();
    let curve: Vec<[f64; 2]> = match fit.chosen {
        Some(i) => match &fit.report.ranking.candidates[i].fitted {
            Some(f) => (0..=80)
                .map(|k| {
                    let x = lo + span * k as f64 / 80.0;
                    [x, f.model.pdf(x)]
                })
                .filter(|p| p[1].is_finite())
                .collect(),
            None => Vec::new(),
        },
        None => Vec::new(),
    };
    json!({"lo": lo, "hi": hi, "counts": counts, "density": density, "curve": curve})
}

fn ranking(fit: &AutoFit, limit: usize) -> Vec<Value> {
    let cands = &fit.report.ranking.candidates;
    fit.order
        .iter()
        .filter(|&&i| cands[i].fit_error.is_none())
        .take(limit)
        .map(|&i| {
            let c = &cands[i];
            let status = if fit.chosen == Some(i) {
                "escolhida"
            } else if c.accepted() {
                "aceita"
            } else {
                "rejeitada"
            };
            let p = |t: &Option<utsim_fit::gof::TestStat>| t.as_ref().map(|t| t.p_value);
            json!({
                "family": c.family,
                "params": c.params.iter().map(|(k, v)| format!("{k}={v:.4}")).collect::<Vec<_>>().join(" "),
                "criterion": fit.criterion.value(c),
                "aic": c.aic,
                "bic": c.bic,
                "ks_p": p(&c.gof.ks),
                "ad_p": p(&c.gof.anderson_darling),
                "status": status,
                "chosen": fit.chosen == Some(i),
            })
        })
        .collect()
}

/// What the panel shows of a fit.
pub fn view_of(input: &DataInput, fit: &AutoFit) -> Value {
    let params: Vec<Value> = fit
        .chosen
        .map(|i| {
            fit.report.ranking.candidates[i]
                .params
                .iter()
                .map(|(k, v)| json!({"label": k, "value": v}))
                .collect()
        })
        .unwrap_or_default();
    json!({
        "field": input.field,
        "family": fit.family,
        "dist": fit.ron(),
        "params": params,
        "n": fit.support.n,
        "hash": fit.hash,
        "support": fit.support.description,
        "criterion": fit.criterion.label(),
        "empirical": fit.chosen.is_none(),
        "fallback": fit.empirical_fallback,
        "justification": fit.justification,
        "warnings": fit.warnings,
        "chart": chart(fit),
        "ranking": ranking(fit, 6),
        "settings": input.settings_key(),
    })
}

/// Fits the values of `input` (a cached fit is returned as it is).
pub fn fit_cached(
    input: &DataInput,
    base: Option<&Path>,
    cache: &FitCache,
) -> Result<Arc<CachedFit>, String> {
    let values = input.read_values(base)?;
    let key = cache_key(input, &values);
    if let Some(hit) = cache.get(&key) {
        return Ok(hit);
    }
    let fit = auto::auto_fit(&input.field, &values, &input.config())?;
    let record = FittedRecord {
        family: fit.family.clone(),
        dist: fit.dist.clone(),
        hash: fit.hash.clone(),
        n: fit.support.n,
        settings: input.settings_key(),
        warnings: fit.warnings.clone(),
    };
    let done = Arc::new(CachedFit {
        record,
        view: view_of(input, &fit),
    });
    cache.put(key, done.clone());
    Ok(done)
}

/// The entry with the fit already recorded, so that applying it does not fit again.
pub fn seeded(
    input: &DataInput,
    base: Option<&Path>,
    cache: &FitCache,
) -> Result<DataInput, String> {
    let fit = fit_cached(input, base, cache)?;
    let mut input = input.clone();
    input.fitted = Some(fit.record.clone());
    Ok(input)
}

/// Where the data of an entry stand now.
pub fn status_text(input: &DataInput) -> &'static str {
    match input.status {
        DataStatus::Pending => "pendente",
        DataStatus::Fitted => "ajustado",
        DataStatus::Reused => tx("godot-dados-registro_reaproveitado"),
        DataStatus::DataChanged { .. } => tx("godot-dados-dados_mudaram"),
        DataStatus::SettingsChanged => tx("godot-dados-opcoes_mudaram"),
        DataStatus::RecordOnly { .. } => tx("godot-dados-dados_ilegiveis"),
    }
}

/// The entry of `field` as the panel shows it: choices, record and whether the data
/// changed since the last fit.
pub fn entry_view(input: &DataInput, base: Option<&Path>, cache: &FitCache) -> Value {
    let mut stale = false;
    let mut error = String::new();
    let mut cached = Value::Null;
    match input.read_values(base) {
        Ok(values) => {
            if let Some(rec) = &input.fitted {
                stale = rec.hash != auto::data_hash(&values);
            }
            if let Some(hit) = cache.get(&cache_key(input, &values)) {
                cached = hit.view.clone();
            }
        }
        Err(e) => error = e,
    }
    let changed_on_load = match &input.status {
        DataStatus::DataChanged {
            old_hash,
            old_family,
        } => fx(
            "godot-dados-os_dados_mudaram_desde_o",
            &[("old_family", vx(&old_family)), ("old_hash", vx(&old_hash))],
        ),
        _ => String::new(),
    };
    json!({
        "field": input.field,
        "config": config_of(input),
        "label": input.source_label(),
        "status": status_text(input),
        "stale": stale,
        "changed_on_load": changed_on_load,
        "error": error,
        "family": input.fitted.as_ref().map(|r| r.family.clone()),
        "dist": input.fitted.as_ref().map(|r| ron::ser::to_string(&r.dist).unwrap_or_default()),
        "n": input.fitted.as_ref().map(|r| r.n),
        "hash": input.fitted.as_ref().map(|r| r.hash.clone()),
        "view": cached,
    })
}

/// Every distribution field of the model: `{field, source: "dist" | "dados", ...}`.
pub fn data_fields(model: &Model, base: Option<&Path>, cache: &FitCache) -> Vec<Value> {
    dados::dist_fields(model)
        .into_iter()
        .map(|field| match model.data.iter().find(|i| i.field == field) {
            Some(input) => {
                let mut v = entry_view(input, base, cache);
                v["source"] = json!("dados");
                v
            }
            None => json!({"field": field, "source": "dist"}),
        })
        .collect()
}

/// Column names of a data file (header names, or positions from 1 without a header).
pub fn columns_of_text(text: &str) -> Vec<String> {
    let Some(line) = text.lines().map(str::trim).find(|l| !l.is_empty()) else {
        return Vec::new();
    };
    let sep = [';', '\t', ',']
        .into_iter()
        .find(|s| line.contains(*s))
        .unwrap_or(';');
    let cells: Vec<String> = line
        .split(sep)
        .map(|c| c.trim().trim_matches('"').to_string())
        .collect();
    let numeric = |c: &String| c.replace(',', ".").parse::<f64>().is_ok();
    if cells.iter().any(|c| !numeric(c)) {
        cells
    } else {
        (1..=cells.len()).map(|k| k.to_string()).collect()
    }
}

/// The columns of a file, or the error.
pub fn columns_of(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        fx(
            "godot-dados-nao_foi_possivel_ler",
            &[("path", vx(&path.display())), ("e", vx(&e))],
        )
    })?;
    Ok(columns_of_text(&text))
}

/// Numbers pasted as text (any separator, decimal comma accepted).
pub fn parse_values(text: &str) -> Result<Vec<f64>, String> {
    utsim_fit::data::parse(text, &DataOptions::default()).map(|l| l.values)
}

/// `path` relative to the model folder (with `/`) when it is inside it, else as given.
pub fn relative_to(base: Option<&Path>, path: &str) -> String {
    let p = PathBuf::from(path);
    if let Some(b) = base
        && let Ok(rel) = p.strip_prefix(b)
    {
        return rel.to_string_lossy().replace('\\', "/");
    }
    path.replace('\\', "/")
}

/// Writes the reports of all the entries of `model` and `index.html` into `dir`;
/// returns `{index, files, failed}`.
pub fn write_report(model: &Model, base: Option<&Path>, dir: &Path) -> Result<Value, String> {
    if model.data.is_empty() {
        return Err(tx("godot-dados-nenhum_campo_do_modelo_usa").into());
    }
    let (written, failed) = dados::write_reports(model, base, dir).map_err(|e| {
        fx(
            "godot-dados-nao_foi_possivel_gravar_em",
            &[("dir", vx(&dir.display())), ("e", vx(&e))],
        )
    })?;
    Ok(json!({
        "index": written.first().map(|p| p.to_string_lossy().replace('\\', "/")),
        "files": written.len(),
        "failed": failed,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples")
            .join(name)
    }

    #[test]
    fn config_round_trip_and_errors() {
        let c = json!({"source": "a.csv", "column": "t", "timestamps": true,
            "choose": "Gama", "criterion": "ks"});
        let i = input_from_config("x.process", &c).unwrap();
        assert_eq!(i.source.as_deref(), Some("a.csv"));
        assert!(i.timestamps);
        assert_eq!(i.choose, Choice::Family("Gama".into()));
        assert_eq!(i.criterion, Criterion::Ks);
        assert_eq!(config_of(&i)["criterion"], "ks");
        assert!(input_from_config("x.process", &json!({})).is_err());
        assert!(input_from_config("x.process", &json!({"source": "a", "values": [1.0]})).is_err());
        assert!(
            input_from_config("x.process", &json!({"values": [1.0], "criterion": "zzz"})).is_err()
        );
    }

    #[test]
    fn columns_and_values() {
        assert_eq!(
            columns_of_text("cliente;tempo\nC1;3,2\n"),
            ["cliente", "tempo"]
        );
        assert_eq!(columns_of_text("1.5\n2.5\n"), ["1"]);
        assert_eq!(parse_values("1.5\n2.5\n3\n").unwrap(), [1.5, 2.5, 3.0]);
    }

    #[test]
    fn fit_is_cached_and_view_has_chart_and_ranking() {
        let base = example("");
        let cache = FitCache::default();
        let c = json!({"source": "dados/tempos_atendimento.csv", "column": "tempo_atendimento"});
        let input = input_from_config("a.process", &c).unwrap();
        let first = fit_cached(&input, Some(&base), &cache).unwrap();
        let again = fit_cached(&input, Some(&base), &cache).unwrap();
        assert!(Arc::ptr_eq(&first, &again));
        let v = &first.view;
        assert!(v["n"].as_u64().unwrap() > 10);
        assert!(!v["chart"]["counts"].as_array().unwrap().is_empty());
        assert!(!v["chart"]["curve"].as_array().unwrap().is_empty());
        assert!(v["ranking"].as_array().unwrap().len() >= 2);
        assert_eq!(v["ranking"][0]["chosen"], true);
        let seeded = seeded(&input, Some(&base), &cache).unwrap();
        assert_eq!(seeded.fitted.unwrap().family, first.record.family);
    }

    #[test]
    fn data_fields_list_every_distribution_field() {
        let model = Model::load(example("tutorial_06_atendimento.ron")).unwrap();
        let list = data_fields(&model, None, &FitCache::default());
        assert!(list.iter().any(|f| f["field"] == "atendentes.process"));
        assert!(list.iter().all(|f| f["source"] == "dist"));
        let m = Model::load(example("dados_reais.ron")).unwrap();
        let list = data_fields(&m, Some(&example("")), &FitCache::default());
        assert!(
            list.iter()
                .any(|f| f["source"] == "dados" && f["stale"] == false)
        );
    }

    #[test]
    fn report_is_written() {
        let m = Model::load(example("dados_reais.ron")).unwrap();
        let dir = std::env::temp_dir().join("utsim_godot_dados_relatorio");
        let _ = std::fs::remove_dir_all(&dir);
        let r = write_report(&m, Some(&example("")), &dir).unwrap();
        assert_eq!(r["failed"], 0);
        assert!(dir.join("index.html").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn editor() -> crate::document::Editor {
        crate::document::Editor::open(&example("tutorial_06_atendimento.ron")).unwrap()
    }

    fn csv() -> String {
        example("dados/tempos_atendimento.csv")
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn config() -> Value {
        json!({"source": csv(), "column": "tempo_atendimento"})
    }

    fn apply(e: &mut crate::document::Editor, cache: &FitCache) {
        let input = input_from_config("atendentes.process", &config()).unwrap();
        let input = seeded(&input, None, cache).unwrap();
        e.apply(utsim_edit::Command::SetFieldSource {
            field: "atendentes.process".into(),
            source: utsim_edit::FieldSource::Data(Box::new(input)),
        })
        .unwrap();
    }

    #[test]
    fn switching_a_field_to_data_is_undoable() {
        let mut e = editor();
        let cache = FitCache::default();
        let before = utsim_model::dados::get_dist(e.model(), "atendentes.process")
            .unwrap()
            .unwrap();
        apply(&mut e, &cache);
        assert_eq!(e.model().data.len(), 1);
        let fitted = utsim_model::dados::get_dist(e.model(), "atendentes.process")
            .unwrap()
            .unwrap();
        assert_ne!(fitted, before);
        assert_eq!(
            e.model().data[0].fitted.as_ref().unwrap().dist,
            fitted,
            "the field holds the fitted distribution"
        );
        assert!(e.doc.undo());
        assert!(e.model().data.is_empty());
        assert_eq!(
            utsim_model::dados::get_dist(e.model(), "atendentes.process")
                .unwrap()
                .unwrap(),
            before
        );
        assert!(e.doc.redo());
        assert_eq!(e.model().data.len(), 1);
        let list = data_fields(e.model(), None, &cache);
        let entry = list
            .iter()
            .find(|f| f["field"] == "atendentes.process")
            .unwrap();
        assert_eq!(entry["source"], "dados");
        assert!(
            entry["view"]["ranking"].is_array(),
            "view comes from the cache"
        );
        assert_eq!(entry["stale"], false);
    }

    #[test]
    fn editing_a_field_with_data_goes_back_to_a_distribution() {
        let mut e = editor();
        apply(&mut e, &FitCache::default());
        e.set_property(
            "atendentes",
            "process.family",
            &crate::schema::Value::Int(2),
            &[],
        )
        .unwrap();
        assert!(e.model().data.is_empty());
        // One undo step brings the data back.
        assert!(e.doc.undo());
        assert_eq!(e.model().data.len(), 1);
    }

    #[test]
    fn every_distribution_of_the_panel_is_a_data_field() {
        for name in [
            "tutorial_06_atendimento.ron",
            "agvs.ron",
            "centro_de_distribuicao.ron",
            "turnos.ron",
            "armazem.ron",
            "linha_com_esteiras.ron",
        ] {
            let e = crate::document::Editor::open(&example(name)).unwrap();
            let listed = utsim_model::dados::dist_fields(e.model());
            for o in e.model().objects.iter() {
                for f in crate::schema::object_fields(e.model(), o) {
                    if f.ty == crate::schema::FieldType::Dist {
                        let key = format!("{}.{}", o.name, f.path);
                        assert!(listed.contains(&key), "{name}: {key} not in {listed:?}");
                    }
                }
            }
        }
    }
}
