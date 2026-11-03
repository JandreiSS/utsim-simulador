//! End-to-end tests: KPIs of a model with a known answer (M/M/1), the HTTP API through a
//! local client, the static page and the spreadsheet. No window is opened.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

use utsim_dash::config::TimeUnit;
use utsim_dash::{Dashboard, Server, Study, page};
use utsim_model::Model;

const MM1: &str = include_str!("../../../examples/mm1.ron");

/// M/M/1 with arrival rate 1 and service rate 1.25: rho 0.8, L = W = 4.
fn study(mut cfg: Dashboard) -> Study {
    let mut model = Model::parse(MM1).unwrap();
    model.run.replications = 8;
    cfg.time_unit = TimeUnit::Unit;
    Study::run(model, cfg, None, "teste").unwrap()
}

#[test]
fn mm1_kpis_match_queueing_theory() {
    let s = study(Dashboard::default());
    let package = s.package();
    let set = &package.kpis;
    let v = |id: &str| set.get(id).unwrap_or_else(|| panic!("sem {id}")).valor;
    assert!((v("vazao") - 1.0).abs() < 0.02, "vazão {}", v("vazao"));
    assert!(
        (v("lead_time") - 4.0).abs() < 0.4,
        "lead time {}",
        v("lead_time")
    );
    assert!((v("wip") - 4.0).abs() < 0.4, "wip {}", v("wip"));
    assert!((v("utilizacao_gargalo") - 80.0).abs() < 1.5);
    // The 90th percentile of an M/M/1 sojourn time is -W ln(0.1) = 9.21.
    assert!(
        (v("lead_time_p90") - 9.21).abs() < 0.9,
        "p90 {}",
        v("lead_time_p90")
    );
    assert_eq!(set.gargalo.as_deref(), Some("maquina"));
    assert!(set.little.as_ref().unwrap().vale);
    // No data, no number: nothing invented.
    for id in ["otd", "perdas", "oee:maquina"] {
        assert!(set.get(id).is_none(), "{id}");
    }
}

#[test]
fn goals_drive_the_light_and_unknown_ones_warn() {
    let cfg = Dashboard::parse(
        r#"Dashboard(targets: {"vazao": AtLeast(0.5), "wip": AtMost(1.0), "xyz": AtLeast(1.0)})"#,
    )
    .unwrap();
    let s = study(cfg);
    let package = s.package();
    let set = &package.kpis;
    let light = |id: &str| serde_json::to_value(set.get(id).unwrap().situacao).unwrap();
    assert_eq!(light("vazao"), "verde");
    assert_eq!(light("wip"), "vermelho");
    assert!(set.avisos.iter().any(|a| a.contains("xyz")));
}

fn http(addr: SocketAddr, request: &str) -> (u16, String, Vec<u8>) {
    let mut c = TcpStream::connect(addr).unwrap();
    c.write_all(request.as_bytes()).unwrap();
    let mut raw = Vec::new();
    c.read_to_end(&mut raw).unwrap();
    let at = raw.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
    let head = String::from_utf8_lossy(&raw[..at]).into_owned();
    let code = head.split_whitespace().nth(1).unwrap().parse().unwrap();
    (code, head, raw[at + 4..].to_vec())
}

fn get(addr: SocketAddr, path: &str) -> (u16, String, Vec<u8>) {
    let host = addr.port();
    http(
        addr,
        &format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{host}\r\n\r\n"),
    )
}

#[test]
fn the_api_answers_over_http() {
    let s = study(Dashboard::default());
    let server = Server::start(s, 0).unwrap();
    let a = server.addr();
    assert!(a.ip().is_loopback());

    let (code, _, body) = get(a, "/api/kpis");
    assert_eq!(code, 200);
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(
        v["kpis"]
            .as_array()
            .unwrap()
            .iter()
            .any(|k| k["id"] == "vazao")
    );

    let (code, _, body) = get(a, "/api/resumo");
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(code, 200);
    assert_eq!(v["progresso"]["concluido"], true);
    assert!(
        v["frase"]
            .as_str()
            .unwrap()
            .starts_with("Com 8 replicações")
    );

    let (code, _, body) = get(a, "/api/objeto/maquina");
    assert_eq!(code, 200);
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["tipo"], "processor");
    assert!(!v["gantt"].as_array().unwrap().is_empty());
    for path in [
        "/api/series",
        "/api/objetos",
        "/api/distribuicoes",
        "/api/pareto",
        "/api/gantt",
        "/api/estados",
        "/api/config",
        "/api/progresso",
    ] {
        assert_eq!(get(a, path).0, 200, "{path}");
    }
    assert_eq!(get(a, "/api/objeto/nao_existe").0, 404);

    // Page and static files.
    let (code, head, body) = get(a, "/");
    assert_eq!(code, 200);
    assert!(head.contains("text/html") && head.contains("Content-Security-Policy"));
    assert!(String::from_utf8_lossy(&body).contains("/app.js"));
    for f in ["/app.js", "/charts.js", "/app.css"] {
        assert_eq!(get(a, f).0, 200, "{f}");
    }

    // Downloads.
    let (code, head, body) = get(a, "/api/export/kpis.csv");
    assert!(
        code == 200
            && head.contains("attachment")
            && String::from_utf8_lossy(&body).starts_with("id,nome")
    );
    assert_eq!(get(a, "/api/export/painel.xlsx").2[..2], *b"PK");

    // A foreign Host (DNS rebinding) and writes are refused.
    let (code, _, _) = http(
        a,
        "GET /api/kpis HTTP/1.1\r\nHost: evil.example.com\r\n\r\n",
    );
    assert_eq!(code, 403);
    let h = format!("Host: 127.0.0.1:{}", a.port());
    assert_eq!(
        http(a, &format!("POST /api/cancelar HTTP/1.1\r\n{h}\r\n\r\n")).0,
        403
    );
    assert_eq!(
        http(a, &format!("DELETE /api/kpis HTTP/1.1\r\n{h}\r\n\r\n")).0,
        405
    );
    let ok = format!("POST /api/cancelar HTTP/1.1\r\n{h}\r\nX-UTSim: 1\r\n\r\n");
    assert_eq!(http(a, &ok).0, 200);
    server.stop();
}

/// Checks that every opened tag is closed (void elements and script/style bodies aside).
fn balanced(html: &str) -> bool {
    let mut rest = html;
    let mut stack: Vec<String> = Vec::new();
    while let Some(i) = rest.find('<') {
        rest = &rest[i + 1..];
        if rest.starts_with('!') {
            rest = &rest[rest.find('>').unwrap() + 1..];
            continue;
        }
        let end = rest.find('>').unwrap();
        let tag = &rest[..end];
        rest = &rest[end + 1..];
        let (closing, name) = match tag.strip_prefix('/') {
            Some(n) => (true, n),
            None => (false, tag),
        };
        let name = name
            .split([' ', '\n', '/'])
            .next()
            .unwrap()
            .to_ascii_lowercase();
        if matches!(name.as_str(), "meta" | "link" | "br" | "input") {
            continue;
        }
        if closing {
            if stack.pop().as_deref() != Some(name.as_str()) {
                return false;
            }
        } else if name == "script" || name == "style" {
            let close = format!("</{name}>");
            rest = &rest[rest.find(&close).unwrap() + close.len()..];
        } else {
            stack.push(name);
        }
    }
    stack.is_empty()
}

#[test]
fn the_static_page_is_valid_and_self_contained() {
    let s = study(Dashboard::default());
    let html = page::static_html(&s.package());
    assert!(html.starts_with("<!doctype html>") && html.contains("lang=\"pt-BR\""));
    assert!(balanced(&html));
    // No external resource: nothing is fetched from the network.
    for bad in ["src=\"http", "href=\"http", "<link", "@import"] {
        assert!(!html.contains(bad), "{bad}");
    }
    // The embedded data parses back and has every route.
    let marker = "window.UTSIM_DADOS = ";
    let start = html.find(marker).unwrap() + marker.len();
    let end = html[start..].find(";</script>").unwrap() + start;
    let data: serde_json::Value = serde_json::from_str(&html[start..end]).unwrap();
    for k in [
        "/api/resumo",
        "/api/kpis",
        "/api/objeto/maquina",
        "/api/series",
    ] {
        assert!(data.get(k).is_some(), "{k}");
    }
    assert!(balanced(&page::live_html()));
}

#[test]
fn export_folder_and_spreadsheet_read_back() {
    let s = study(Dashboard::default());
    let dir = std::env::temp_dir().join(format!("utsim_dash_{}", std::process::id()));
    let files = page::export_folder(&dir, &s.package()).unwrap();
    for f in [
        "index.html",
        "kpis.csv",
        "objetos.csv",
        "replicacoes.csv",
        "painel.json",
        "painel.xlsx",
    ] {
        assert!(files.iter().any(|x| x == f), "{f}");
    }
    let csv = std::fs::read_to_string(dir.join("kpis.csv")).unwrap();
    assert!(
        csv.lines()
            .any(|l| l.starts_with("vazao,Vazão,itens/u.t.,8,"))
    );
    // The workbook is a zip with the five sheets and the KPI ids in its strings.
    let file = std::fs::File::open(dir.join("painel.xlsx")).unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();
    let mut book = String::new();
    zip.by_name("xl/workbook.xml")
        .unwrap()
        .read_to_string(&mut book)
        .unwrap();
    for name in ["Resumo", "KPIs", "Objetos", "Replicações", "Série WIP"] {
        assert!(book.contains(name), "{name}");
    }
    let mut strings = String::new();
    zip.by_name("xl/sharedStrings.xml")
        .unwrap()
        .read_to_string(&mut strings)
        .unwrap();
    assert!(strings.contains("lead_time_p90") && strings.contains("maquina"));
    let _ = std::fs::remove_dir_all(&dir);
}
