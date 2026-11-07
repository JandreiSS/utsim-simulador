//! The right panel of the tools of Editar (Rede, Corredor, Zona and a selected connection),
//! built from the model as the rows of [`crate::ui_form`] (step 7 of
//! `docs/plano_interface_rust.md`). The edits come back to the main scene as
//! `form:<action>:<value>`, as before; the rows used to be built by `principal.gd`.

use serde_json::{Value, json};
use utsim_edit::Issue;
use utsim_model::Model;

/// A number for people: about four significant digits, no trailing zeros (as the
/// interface's `_num`).
pub fn num(x: f64) -> String {
    if x == 0.0 || !x.is_finite() {
        return if x.is_nan() { "nan".into() } else { "0".into() };
    }
    let digits = (3 - x.abs().log10().floor() as i64).clamp(0, 6) as usize;
    let s = format!("{x:.digits$}");
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        s
    }
}

/// The issues whose message mentions one of `words`, as error rows.
fn issue_rows(rows: &mut Vec<Value>, issues: &[Issue], words: &[&str]) {
    for i in issues {
        if words.iter().any(|w| i.message.contains(w)) {
            rows.push(json!({"kind": "error", "text": i.message}));
        }
    }
}

/// The panel of a selected connection.
pub fn link(from: &str, to: &str) -> Vec<Value> {
    vec![
        json!({"kind": "title", "text": "Ligação"}),
        json!({"kind": "info", "name": "Ligacao_Texto", "text": format!("{from} → {to}")}),
        json!({"kind": "button", "name": "Ligacao_Remover", "text": "Remover ligação (Del)", "action": "ligacao_remover"}),
    ]
}

/// What the Rede tool has selected.
#[derive(Clone, Debug, Default)]
pub struct NetSel {
    pub node: String,
    pub edge: Option<(String, String)>,
    pub two_way: bool,
}

/// The panel of the Rede tool.
pub fn network(model: &Model, issues: &[Issue], sel: &NetSel) -> Vec<Value> {
    let nodes = crate::network_edit::nodes(model);
    let edges = crate::network_edit::edges(model);
    let controlled = edges.iter().filter(|e| e.controlled()).count();
    let mut rows = vec![
        json!({"kind": "title", "text": "Rede de caminhos"}),
        json!({"kind": "info", "name": "Rede_Resumo", "text": format!("{} nó(s), {} trecho(s), {} pista(s) controlada(s).", nodes.len(), edges.len(), controlled)}),
        json!({"kind": "help", "text": "Clique no chão para pôr um nó; com um nó armado (verde), o próximo clique o liga ao novo por um trecho. Clique num nó para armá-lo, arraste para movê-lo. Clique num trecho para mudar o sentido e a capacidade (com capacidade, a pista é controlada e fica em vermelho). Esc encerra o caminho; Del exclui o selecionado."}),
        json!({"kind": "check", "name": "Rede_NovosMaoDupla", "text": "Novos trechos em mão dupla", "on": sel.two_way, "action": "rede_novos_mao_dupla"}),
    ];
    if let Some(n) = nodes.iter().find(|n| n.name == sel.node) {
        rows.push(json!({"kind": "section", "text": format!("Nó {}", n.name)}));
        rows.push(json!({"kind": "line", "name": "Rede_NoNome", "label": "Nome", "value": n.name, "action": "rede_no_nome"}));
        rows.push(json!({"kind": "info", "text": format!("Posição ({}, {}) m", num(n.position[0]), num(n.position[1]))}));
        rows.push(json!({"kind": "button", "name": "Rede_NoExcluir", "text": "Excluir nó (Del)", "action": "rede_excluir"}));
    }
    if let Some((a, b)) = &sel.edge
        && let Some(e) = edges.iter().find(|e| &e.from == a && &e.to == b)
    {
        let cap = e.capacity.unwrap_or(0);
        rows.push(json!({"kind": "section", "text": format!("Trecho {} → {}", e.from, e.to)}));
        rows.push(json!({"kind": "info", "text": format!("Comprimento {} m", num(e.length))}));
        rows.push(json!({"kind": "check", "name": "Rede_TrechoMaoDupla", "text": "Mão dupla", "on": e.two_way, "action": "rede_mao_dupla"}));
        rows.push(json!({"kind": "number", "name": "Rede_TrechoCapacidade", "label": "Capacidade (veículos; 0 = sem controle)", "value": cap, "min": 0, "max": 100, "step": 1, "action": "rede_capacidade"}));
        if e.controlled() {
            rows.push(json!({"kind": "error", "text": format!("Pista controlada: no máximo {cap} veículo(s) ao mesmo tempo.")}));
        }
        rows.push(json!({"kind": "button", "name": "Rede_TrechoInverter", "text": "Inverter sentido", "enabled": !e.two_way, "action": "rede_inverter"}));
        rows.push(json!({"kind": "button", "name": "Rede_TrechoExcluir", "text": "Excluir trecho (Del)", "action": "rede_excluir"}));
    }
    issue_rows(&mut rows, issues, &["network", "rede", "nó"]);
    rows
}

/// What the Corredor tool has selected, and what the next aisle starts as.
#[derive(Clone, Debug, Default)]
pub struct AisleSel {
    pub selected: String,
    pub vertex: Option<usize>,
    pub new_kind: String,
    pub new_width: f64,
    pub new_two_way: bool,
}

const AISLE_KINDS: [(&str, &str); 3] = [
    ("Pedestres", "pedestrian"),
    ("Empilhadeiras", "forklift"),
    ("Misto", "mixed"),
];

/// The panel of the Corredor tool.
pub fn aisles(model: &Model, issues: &[Issue], warnings: &[Issue], sel: &AisleSel) -> Vec<Value> {
    let list = crate::aisle_edit::aisles(model);
    let crossings = crate::aisle_edit::crossings(model).len();
    let count = |k: &str| list.iter().filter(|a| a.kind == k).count();
    let kinds: Vec<Value> = AISLE_KINDS.iter().map(|(l, k)| json!([l, k])).collect();
    let mut rows = vec![
        json!({"kind": "title", "text": "Corredores de circulação"}),
        json!({"kind": "info", "name": "Corredor_Resumo", "text": format!(
            "{} corredor(es): {} de pedestres, {} de empilhadeiras, {} mistos; {} cruzamento(s).",
            list.len(), count("pedestrian"), count("forklift"), list.len() - count("pedestrian") - count("forklift"), crossings)}),
        json!({"kind": "help", "text": "Clique no chão para pôr os pontos do eixo; Enter ou duplo clique termina, Backspace volta um ponto, Esc cancela. Os pontos encaixam nos outros corredores. Clique num corredor para editá-lo, arraste um vértice para movê-lo, Shift + clique na linha acrescenta um vértice. As faixas amarelas, a zebra dos cruzamentos e as setas aparecem no piso."}),
    ];
    match list.iter().find(|a| a.name == sel.selected) {
        None => {
            rows.push(json!({"kind": "section", "text": "Novos corredores"}));
            rows.push(json!({"kind": "choice", "name": "Corredor_NovoTipo", "label": "Tipo", "items": kinds, "value": sel.new_kind, "action": "corredor_tipo"}));
            rows.push(json!({"kind": "number", "name": "Corredor_NovaLargura", "label": "Largura (m)", "value": sel.new_width, "min": 0.3, "max": 20.0, "step": 0.05, "action": "corredor_largura"}));
            rows.push(json!({"kind": "check", "name": "Corredor_NovosMaoDupla", "text": "Mão dupla", "on": sel.new_two_way, "action": "corredor_mao_dupla"}));
        }
        Some(a) => {
            rows.push(json!({"kind": "section", "text": format!("Corredor {}", a.name)}));
            rows.push(json!({"kind": "line", "name": "Corredor_Nome", "label": "Nome", "value": a.name, "action": "corredor_nome"}));
            rows.push(json!({"kind": "info", "name": "Corredor_Comprimento", "text": format!("{} ponto(s), {} m de comprimento", a.points.len(), num(a.length))}));
            rows.push(json!({"kind": "choice", "name": "Corredor_Tipo", "label": "Tipo", "items": kinds, "value": a.kind, "action": "corredor_tipo"}));
            rows.push(json!({"kind": "number", "name": "Corredor_Largura", "label": "Largura (m)", "value": a.width, "min": 0.3, "max": 20.0, "step": 0.05, "action": "corredor_largura"}));
            rows.push(json!({"kind": "check", "name": "Corredor_MaoDupla", "text": "Mão dupla", "on": a.two_way, "action": "corredor_mao_dupla"}));
            if a.kind != "forklift" {
                rows.push(json!({"kind": "check", "name": "Corredor_Secundaria", "text": "Passagem secundária de pedestres (mínimo menor)", "on": a.secondary, "action": "corredor_secundaria"}));
            }
            rows.push(json!({"kind": "choice", "name": "Corredor_Perfil", "label": "Perfil", "items": [["Padrão do modelo", ""], ["Brasil (amarelo)", "Brasil"], ["Branco", "Branco"]], "value": a.profile, "action": "corredor_perfil"}));
            if let Some(v) = sel.vertex {
                rows.push(json!({"kind": "info", "name": "Corredor_Vertice", "text": format!("Vértice {} selecionado", v + 1)}));
                rows.push(json!({"kind": "button", "name": "Corredor_VerticeRemover", "text": "Remover vértice", "enabled": a.points.len() > 2, "action": "corredor_vertice_remover"}));
            }
            rows.push(json!({"kind": "button", "name": "Corredor_Inverter", "text": "Inverter sentido", "enabled": !a.two_way, "action": "corredor_inverter"}));
            rows.push(json!({"kind": "button", "name": "Corredor_Excluir", "text": "Excluir corredor (Del)", "action": "corredor_excluir"}));
            let quoted = format!("'{}'", a.name);
            for w in warnings.iter().filter(|w| w.message.contains(&quoted)) {
                rows.push(json!({"kind": "warning", "name": "Corredor_Aviso", "text": format!("• {}", w.message)}));
            }
        }
    }
    issue_rows(&mut rows, issues, &["corredor", "Aisles"]);
    rows
}

/// The panel of the Zona tool: what it does, and the zone selected.
pub fn zones(model: &Model, issues: &[Issue], selected: &str) -> Vec<Value> {
    let mut rows = vec![
        json!({"kind": "title", "text": "Zonas de risco"}),
        json!({"kind": "info", "name": "Zona_Resumo", "text": format!("{} zona(s) de risco no piso.", model.zones.len())}),
        json!({"kind": "help", "text": "Clique no chão para pôr os cantos; o primeiro ponto, Enter ou duplo clique fecha a zona, Backspace volta um ponto, Esc cancela. Clique numa zona para editá-la: arraste o meio para mover, arraste um vértice para mudar a forma; Del exclui. Os pontos encaixam na grade."}),
    ];
    if let Some(z) = model.zones.iter().find(|z| z.name == selected) {
        let kinds: Vec<Value> = crate::zone_edit::KINDS
            .iter()
            .map(|(k, key, _)| json!([crate::zone_edit::kind_label(*k), key]))
            .collect();
        rows.push(json!({"kind": "section", "text": format!("Zona {}", z.name)}));
        rows.push(json!({"kind": "line", "name": "Zona_Nome", "label": "Nome", "value": z.name, "action": "zona_nome"}));
        rows.push(json!({"kind": "info", "name": "Zona_Pontos", "text": format!("{} canto(s)", z.polygon.len())}));
        rows.push(json!({"kind": "choice", "name": "Zona_Tipo", "label": "Tipo", "items": kinds, "value": crate::zone_edit::kind_key(z.kind), "action": "zona_tipo"}));
        rows.push(json!({"kind": "number", "name": "Zona_Nivel", "label": "Nível de risco (1 a 5; 0 = sem nível)", "value": z.level.unwrap_or(0), "min": 0, "max": 5, "step": 1, "action": "zona_nivel"}));
        rows.push(json!({"kind": "section", "text": "EPI exigido para entrar"}));
        for item in crate::zone_edit::ppe_choices(model) {
            rows.push(json!({"kind": "check", "name": format!("Zona_EPI_{item}"), "text": item.replace('_', " "), "on": z.ppe.contains(&item), "action": format!("zona_epi_{item}")}));
        }
        rows.push(json!({"kind": "button", "name": "Zona_Excluir", "text": "Excluir zona (Del)", "action": "zona_excluir"}));
    }
    issue_rows(&mut rows, issues, &["zona", "zone"]);
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui_form::{FormRow, parse};
    use utsim_edit::{Command, Document};
    use utsim_model::quality::{ZoneDef, ZoneKind};

    fn names(rows: &[Value]) -> Vec<String> {
        let parsed = parse(&Value::Array(rows.to_vec()).to_string());
        assert_eq!(parsed.len(), rows.len(), "every row parses");
        parsed
            .iter()
            .filter_map(|r| match r {
                FormRow::Text { name, .. }
                | FormRow::Check { name, .. }
                | FormRow::Number { name, .. }
                | FormRow::Line { name, .. }
                | FormRow::Choice { name, .. }
                | FormRow::Button { name, .. } => Some(name.clone()),
                _ => None,
            })
            .filter(|n| !n.is_empty())
            .collect()
    }

    #[test]
    fn numbers_read_like_the_interface() {
        assert_eq!(num(24.0), "24");
        assert_eq!(num(1.2), "1.2");
        assert_eq!(num(2.71828), "2.718");
        assert_eq!(num(0.0), "0");
        assert_eq!(num(1234.6), "1235");
    }

    #[test]
    fn the_zone_panel_shows_the_zone_selected_with_its_kind_level_and_ppe() {
        let mut d = Document::new("z");
        d.apply(Command::AddZone(ZoneDef {
            name: "prensa".into(),
            kind: ZoneKind::Noise,
            polygon: vec![[0.0, 0.0], [2.0, 0.0], [2.0, 2.0]],
            level: Some(4),
            ppe: vec!["protetor_auricular".into()],
        }))
        .unwrap();
        let none = zones(d.model(), &[], "");
        assert!(names(&none).contains(&"Zona_Resumo".to_owned()));
        assert!(!names(&none).contains(&"Zona_Nome".to_owned()));
        let rows = zones(d.model(), &[], "prensa");
        let n = names(&rows);
        for w in [
            "Zona_Nome",
            "Zona_Tipo",
            "Zona_Nivel",
            "Zona_EPI_capacete",
            "Zona_EPI_protetor_auricular",
            "Zona_Excluir",
        ] {
            assert!(n.contains(&w.to_owned()), "{w}: {n:?}");
        }
        let on = rows
            .iter()
            .find(|r| r["name"] == "Zona_EPI_protetor_auricular")
            .unwrap();
        assert_eq!(on["on"], true);
        let level = rows.iter().find(|r| r["name"] == "Zona_Nivel").unwrap();
        assert_eq!(level["value"], 4);
    }

    #[test]
    fn the_network_and_aisle_panels_follow_the_selection() {
        let mut d = Document::new("r");
        let model = d.model().clone();
        let empty = network(
            &model,
            &[],
            &NetSel {
                two_way: true,
                ..Default::default()
            },
        );
        assert!(names(&empty).contains(&"Rede_Resumo".to_owned()));
        assert!(
            empty[1]["text"]
                .as_str()
                .unwrap()
                .starts_with("0 nó(s), 0 trecho(s)")
        );
        let a = aisles(
            d.model(),
            &[],
            &[],
            &AisleSel {
                new_kind: "pedestrian".into(),
                new_width: 1.2,
                new_two_way: true,
                ..Default::default()
            },
        );
        let n = names(&a);
        assert!(
            n.contains(&"Corredor_NovoTipo".to_owned()) && !n.contains(&"Corredor_Nome".to_owned())
        );
        // A link panel; an issue about the network shows on the network panel.
        assert_eq!(
            names(&link("a", "b")),
            vec!["Ligacao_Texto", "Ligacao_Remover"]
        );
        let issue = Issue {
            object: None,
            message: "rede: o nó x está solto".into(),
        };
        let rows = network(d.model(), &[issue], &NetSel::default());
        assert!(rows.iter().any(|r| r["kind"] == "error"));
        let _ = &mut d;
    }
}
