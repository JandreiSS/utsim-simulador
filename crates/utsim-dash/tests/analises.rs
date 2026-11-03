//! The analyses that run after the replications (exact Gantt, heat map, scenarios,
//! correlation, PERT tornado) and the series with a band. No window is opened.

use serde_json::Value;
use utsim_dash::analyses::ScenarioSet;
use utsim_dash::config::TimeUnit;
use utsim_dash::{Dashboard, Study, page};
use utsim_model::Model;
use utsim_model::experiment::Scenario;

const MM1: &str = include_str!("../../../examples/mm1.ron");
const AGVS: &str = include_str!("../../../examples/agvs.ron");

fn mm1() -> Model {
    let mut model = Model::parse(MM1).unwrap();
    model.run.replications = 5;
    model.run.stop_time = 6000.0;
    model.run.warmup = 500.0;
    model
}

fn scenario(name: &str, mean: &str) -> Scenario {
    Scenario {
        name: name.to_string(),
        set: [("maquina.process".to_string(), format!("Exp(mean: {mean})"))]
            .into_iter()
            .collect(),
    }
}

#[test]
fn scenarios_correlation_series_and_pert_are_published() {
    let mut cfg = Dashboard::parse(
        r#"Dashboard(
            groups: {"posto": ["fila", "maquina"]},
            pert: Some((from: Some("fila"), to: Some("maquina"), samples: 2000)),
        )"#,
    )
    .unwrap();
    cfg.time_unit = TimeUnit::Unit;
    let set = ScenarioSet {
        name: "capacidade".into(),
        scenarios: vec![scenario("base", "0.8"), scenario("rapida", "0.5")],
        measures: vec!["fila.content.mean".into(), "maquina.utilization".into()],
    };
    let study = Study::run_with(mm1(), cfg, None, "teste", Some(set)).unwrap();
    let p = study.progress();
    assert!(p.concluido && p.etapa.is_empty());
    let pk = study.package();
    let r = |k: &str| -> &Value { pk.routes.get(k).unwrap_or_else(|| panic!("sem {k}")) };

    // Scenarios: utilisation 0.8 against 0.5 differs, by every method.
    let c = r("/api/cenarios");
    assert_eq!(c["cenarios"], serde_json::json!(["base", "rapida"]));
    let util = c["metricas"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["metrica"] == "maquina.utilization")
        .unwrap();
    assert!(util["anova"]["p"].as_f64().unwrap() < 1e-6);
    assert_eq!(util["tukey"]["pares"][0]["difere"], true);
    assert_eq!(util["bonferroni"]["comparacoes"], 1);
    assert!(
        util["bonferroni"]["pares"][0]["diferenca"]
            .as_f64()
            .unwrap()
            > 0.25
    );

    // Correlation of the replications.
    let k = r("/api/correlacao");
    assert_eq!(k["n"], 5);
    assert!(k["metricas"].as_array().unwrap().len() >= 2);

    // Series: throughput per window near the arrival rate 1, and the group's WIP.
    let s = r("/api/series");
    let tp = s["vazao"]["pontos"].as_array().unwrap();
    assert_eq!(tp.len(), 20, "a twentieth of the window each");
    let mean: f64 = tp.iter().map(|p| p[1].as_f64().unwrap()).sum::<f64>() / tp.len() as f64;
    assert!((mean - 1.0).abs() < 0.05, "vazão {mean}");
    assert_eq!(
        s["grupos"][0]["objetos"],
        serde_json::json!(["fila", "maquina"])
    );
    assert!(!s["grupos"][0]["pontos"].as_array().unwrap().is_empty());

    // Exact Gantt from the trace replaces the sampled one.
    let g = r("/api/gantt");
    assert_eq!(g["fonte"], "rastro");
    let maq = g["linhas"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["rotulo"] == "maquina")
        .unwrap();
    let segs = maq["segmentos"].as_array().unwrap();
    assert!(segs.len() > 100, "every state change: {}", segs.len());

    // PERT tornado.
    let t = r("/api/pert");
    assert!(t.get("erro").is_none(), "{t}");
    assert!(!t["tornado"].as_array().unwrap().is_empty());

    // No transporter: the heat map is empty, not an error.
    let m = r("/api/mapa_calor");
    assert_eq!(m["total"]["movimento"], 0.0);

    // The frozen page carries the new routes.
    let html = page::static_html(&pk);
    for key in [
        "/api/cenarios",
        "/api/correlacao",
        "/api/mapa_calor",
        "/api/pert",
    ] {
        assert!(html.contains(key), "{key}");
    }
}

#[test]
fn heat_map_of_the_agv_fleet() {
    let mut model = Model::parse(AGVS).unwrap();
    model.run.replications = 2;
    let study = Study::run(model, Dashboard::default(), None, "teste").unwrap();
    let pk = study.package();
    let m = &pk.routes["/api/mapa_calor"];
    assert_eq!(m["celula"], 0.5);
    assert!(m["total"]["movimento"].as_f64().unwrap() > 0.0);
    assert!(!m["recurso"].as_array().unwrap().is_empty());
    assert_eq!(m["classe"][0]["nome"], "AGVs");
    let cells = m["total"]["celulas"].as_array().unwrap();
    assert!(!cells.is_empty());
    // [col, row, time, passages], inside the grid.
    let (cols, rows) = (
        m["colunas"].as_u64().unwrap(),
        m["linhas"].as_u64().unwrap(),
    );
    for c in cells {
        assert!(c[0].as_u64().unwrap() < cols && c[1].as_u64().unwrap() < rows);
    }
    assert!(!m["objetos"].as_array().unwrap().is_empty());
    // Without scenarios and PERT, there are no such routes.
    assert!(!pk.routes.contains_key("/api/cenarios"));
    assert!(!pk.routes.contains_key("/api/pert"));
}
