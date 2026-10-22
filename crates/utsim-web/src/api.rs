//! Plain Rust behind the WebAssembly bindings: text in, JSON/HTML text out.

use serde::Deserialize;
use serde_json::{Value, json};
use utsim_core::summarize;
use utsim_fit::data::{self, DataOptions};
use utsim_fit::report::{FitConfig, analyze};
use utsim_model::{Model, ModelError};

/// Example models shipped inside the page (name, source).
const EXAMPLES: &[(&str, &str)] = &[
    ("mm1", include_str!("../../../examples/mm1.ron")),
    (
        "atendimento_com_perda",
        include_str!("../../../examples/atendimento_com_perda.ron"),
    ),
    ("armazem", include_str!("../../../examples/armazem.ron")),
    (
        "celula_com_operadores",
        include_str!("../../../examples/celula_com_operadores.ron"),
    ),
    (
        "centro_de_distribuicao",
        include_str!("../../../examples/centro_de_distribuicao.ron"),
    ),
    (
        "fluxo_docas",
        include_str!("../../../examples/fluxo_docas.ron"),
    ),
    (
        "linha_com_bloqueio",
        include_str!("../../../examples/linha_com_bloqueio.ron"),
    ),
    (
        "linha_com_esteiras",
        include_str!("../../../examples/linha_com_esteiras.ron"),
    ),
    (
        "montagem_paletes",
        include_str!("../../../examples/montagem_paletes.ron"),
    ),
    (
        "rede_de_caminhos",
        include_str!("../../../examples/rede_de_caminhos.ron"),
    ),
    (
        "roteamento_por_script",
        include_str!("../../../examples/roteamento_por_script.ron"),
    ),
    (
        "roteamento_por_tipo",
        include_str!("../../../examples/roteamento_por_tipo.ron"),
    ),
    ("turnos", include_str!("../../../examples/turnos.ron")),
];

fn errors_of(e: &ModelError) -> Vec<String> {
    match e {
        ModelError::Invalid(list) => list.clone(),
        other => vec![other.to_string()],
    }
}

fn failure(errors: Vec<String>) -> String {
    json!({ "ok": false, "errors": errors }).to_string()
}

fn load(ron_text: &str) -> Result<Model, ModelError> {
    let model = Model::parse(ron_text)?;
    model.validate()?;
    Ok(model)
}

pub fn validate_model(ron_text: &str) -> String {
    let errors = match load(ron_text) {
        Ok(_) => Vec::new(),
        Err(e) => errors_of(&e),
    };
    json!(errors).to_string()
}

/// `replications == 0` keeps the number written in the model.
pub fn run_model(ron_text: &str, replications: u32) -> String {
    let mut model = match load(ron_text) {
        Ok(m) => m,
        Err(e) => return failure(errors_of(&e)),
    };
    if replications > 0 {
        model.run.replications = replications;
    }
    let reports = match model.run_replications() {
        Ok(r) => r,
        Err(e) => return failure(errors_of(&e)),
    };
    let summary = summarize(&reports);
    let first = &reports[0];
    let blocks: Vec<Value> = first
        .blocks
        .iter()
        .map(|b| {
            let prefix = format!("{}.", b.name);
            // Metric keys are `block.metric`; skip keys that belong to another block
            // whose name extends this one (e.g. "Fila" and "Fila.2").
            let metrics: serde_json::Map<String, Value> = summary
                .iter()
                .filter(|(k, _)| k.starts_with(&prefix))
                .filter(|(k, _)| {
                    !first.blocks.iter().any(|o| {
                        o.name.len() > b.name.len() && k.starts_with(&format!("{}.", o.name))
                    })
                })
                .map(|(k, s)| (k[prefix.len()..].to_string(), json!(s)))
                .collect();
            json!({ "name": b.name, "kind": b.kind, "metrics": metrics })
        })
        .collect();
    json!({
        "ok": true,
        "name": model.name,
        "replications": reports.len(),
        "stop_time": model.run.stop_time,
        "warmup": model.run.warmup,
        "events": reports.iter().map(|r| r.events).sum::<u64>(),
        "blocks": blocks,
        "summary": summary,
    })
    .to_string()
}

pub fn replay(ron_text: &str, stop: f64) -> String {
    let mut model = match load(ron_text) {
        Ok(m) => m,
        Err(e) => return failure(errors_of(&e)),
    };
    if stop > 0.0 && stop.is_finite() {
        model.run.stop_time = stop;
        if model.run.warmup >= stop {
            model.run.warmup = 0.0;
        }
    }
    match model.replay(0) {
        Ok(r) => match serde_json::to_value(&r) {
            Ok(v) => json!({ "ok": true, "replay": v }).to_string(),
            Err(e) => failure(vec![e.to_string()]),
        },
        Err(e) => failure(errors_of(&e)),
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct FitOptions {
    column: Option<String>,
    timestamps: bool,
    alpha: Option<f64>,
    bootstrap: Option<usize>,
    seed: Option<u64>,
}

pub fn fit_data(csv_text: &str, options_json: &str) -> Result<String, String> {
    let opts: FitOptions = if options_json.trim().is_empty() {
        FitOptions::default()
    } else {
        serde_json::from_str(options_json).map_err(|e| format!("opções inválidas: {e}"))?
    };
    let alpha = opts.alpha.unwrap_or(0.05);
    if !(alpha > 0.0 && alpha < 1.0) {
        return Err(format!("alpha deve estar entre 0 e 1 (recebido {alpha})"));
    }
    let loaded = data::parse(
        csv_text,
        &DataOptions {
            column: opts.column.filter(|c| !c.trim().is_empty()),
            timestamps: opts.timestamps,
        },
    )?;
    let mut config = FitConfig {
        alpha,
        ..FitConfig::default()
    };
    if let Some(b) = opts.bootstrap {
        config.gof.bootstrap = b;
    }
    if let Some(s) = opts.seed {
        config.gof.seed = s;
    }
    let dataset = format!("dados colados ({})", loaded.column);
    Ok(analyze(&dataset, &loaded.values, &config).to_html())
}

pub fn examples() -> String {
    let list: Vec<Value> = EXAMPLES
        .iter()
        .map(|(name, text)| json!({ "name": name, "text": text }))
        .collect();
    Value::Array(list).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mm1() -> &'static str {
        EXAMPLES[0].1
    }

    fn parse(s: &str) -> Value {
        serde_json::from_str(s).unwrap()
    }

    #[test]
    fn mm1_is_valid() {
        assert_eq!(validate_model(mm1()), "[]");
    }

    #[test]
    fn syntax_error_is_reported() {
        let v = parse(&validate_model("(nome: "));
        assert_eq!(v.as_array().unwrap().len(), 1);
    }

    #[test]
    fn all_examples_validate() {
        for (name, text) in EXAMPLES {
            assert_eq!(validate_model(text), "[]", "{name}");
        }
        assert_eq!(parse(&examples()).as_array().unwrap().len(), EXAMPLES.len());
    }

    #[test]
    fn run_matches_core_results() {
        let v = parse(&run_model(mm1(), 3));
        assert_eq!(v["ok"], true);
        assert_eq!(v["replications"], 3);
        let mut m = Model::parse(mm1()).unwrap();
        m.run.replications = 3;
        let expected = summarize(&m.run_replications().unwrap());
        let key = v["blocks"][0]["name"].as_str().unwrap().to_string();
        let got = v["summary"][format!("{key}.output")]["mean"]
            .as_f64()
            .unwrap();
        // serde_json parses floats without exact round-trip, hence the tolerance.
        assert!((got - expected[&format!("{key}.output")].mean).abs() < 1e-6);
        assert!(v["blocks"][0]["metrics"]["output"]["mean"].is_number());
    }

    #[test]
    fn run_reports_invalid_model() {
        let v = parse(&run_model("(", 1));
        assert_eq!(v["ok"], false);
        assert!(!v["errors"].as_array().unwrap().is_empty());
    }

    #[test]
    fn replay_has_records() {
        let v = parse(&replay(mm1(), 50.0));
        assert_eq!(v["ok"], true);
        assert!(!v["replay"]["records"].as_array().unwrap().is_empty());
        assert_eq!(v["replay"]["stop_time"], 50.0);
    }

    #[test]
    fn fit_produces_html() {
        let csv = "tempo\n1.2\n0.8\n2.5\n0.4\n1.9\n3.1\n0.7\n1.1\n2.2\n0.9\n1.6\n0.5\n";
        let html = fit_data(csv, r#"{"bootstrap":20}"#).unwrap();
        assert!(html.contains("<h"));
        assert!(fit_data("", "").is_err());
        assert!(fit_data(csv, r#"{"alpha":2}"#).is_err());
    }
}
