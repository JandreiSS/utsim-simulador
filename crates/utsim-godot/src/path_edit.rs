//! Esteiras e caminhos desenhados por nós no Editar, como se desenham ruas, sem tipos do
//! Godot: o encaixe de cada clique, a prévia do trecho (comprimento e inclinação), as
//! linhas que a cena desenha e as edições, cada uma um `utsim_edit::Command` (ou um
//! `Batch`, um passo de desfazer).
//!
//! Três ferramentas: a esteira simples, a esteira com canaletas e o caminho no piso. As
//! duas esteiras viram uma `Conveyor` com `route` (nós relativos à posição dela); o
//! caminho vira um corredor de pedestres com `heights` e `radii` (a rede de corredores
//! passa a segui-lo).
//!
//! Tudo aqui está em coordenadas do modelo (metros, `z` para cima); a classe converte
//! para o Godot.

use utsim_edit::Command;
use utsim_model::trajeto::{self as tr, BeltDrive, BeltRoute, BeltStyle};
use utsim_model::{Aisle, AisleKind, Model, ObjectKind, PathNode};

use crate::document::{Editor, snap_to};

/// Ids da biblioteca das três ferramentas.
pub const BELT_SIMPLE: &str = "EsteiraDesenhada";
pub const BELT_CHANNELED: &str = "EsteiraCanaletas";
pub const FLOOR_PATH: &str = "CaminhoPiso";
pub const IDS: [&str; 3] = [BELT_SIMPLE, BELT_CHANNELED, FLOOR_PATH];

/// Altura padrão da superfície de uma esteira nova (m): a de uma bancada.
pub const BELT_HEIGHT: f64 = 0.8;
/// Largura padrão de um caminho novo (m): a passagem principal de pedestres.
pub const PATH_WIDTH: f64 = 1.2;
/// Passo do encaixe de ângulo (graus).
pub const ANGLE_STEP: f64 = 15.0;

/// A ferramenta de uma entrada da biblioteca, se for uma das três.
pub fn tool_of(id: &str) -> Option<&'static str> {
    IDS.into_iter().find(|t| *t == id)
}

pub fn style_of(tool: &str) -> BeltStyle {
    if tool == BELT_CHANNELED {
        BeltStyle::Channeled
    } else {
        BeltStyle::Simple
    }
}

pub fn style_name(s: BeltStyle) -> &'static str {
    match s {
        BeltStyle::Simple => "simples",
        BeltStyle::Channeled => "canaletas",
    }
}

pub fn drive_name(d: BeltDrive) -> &'static str {
    match d {
        BeltDrive::Powered => "motorizada",
        BeltDrive::Gravity => "gravidade",
    }
}

/// No que um clique encaixou.
#[derive(Clone, Debug, PartialEq)]
pub struct Snap {
    pub at: [f64; 3],
    /// `porta` (ponta de esteira ou objeto), `no` (nó de trajeto), `caminho` (ponto ou
    /// linha de corredor), `angulo`, `grade` ou `livre`.
    pub what: &'static str,
    /// O objeto da porta (para ligar a esteira nova a ele).
    pub object: Option<String>,
    /// Ponta de saída (`true`) ou de entrada de uma esteira; `None` para os demais.
    pub output: Option<bool>,
}

fn d2(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}

fn abs_nodes(origin: [f64; 3], r: &BeltRoute) -> Vec<[f64; 3]> {
    r.nodes
        .iter()
        .map(|n| {
            [
                n.at[0] + origin[0],
                n.at[1] + origin[1],
                n.at[2] + origin[2],
            ]
        })
        .collect()
}

/// Encaixe de um ponto do piso `p` (em planta; `z` é a altura que a ferramenta quer):
/// 1. porta: a ponta de uma esteira (na altura dela) ou o centro de um objeto fixo, a
///    menos de `radius`;
/// 2. nó de outro trajeto ou ponto de corredor, a menos de `radius` (com a altura dele);
/// 3. com `prev`, o ângulo em passos de [`ANGLE_STEP`] e o comprimento no passo da grade;
/// 4. a grade.
///
/// `skip` é a esteira que se edita (não encaixa nela mesma).
pub fn snap(
    model: &Model,
    p: [f64; 3],
    step: f64,
    radius: f64,
    prev: Option<[f64; 3]>,
    skip: Option<&str>,
) -> Snap {
    let mut best: Option<(f64, Snap)> = None;
    let offer = |d: f64, s: Snap, best: &mut Option<(f64, Snap)>| {
        if d < radius && best.as_ref().is_none_or(|(bd, _)| d < *bd) {
            *best = Some((d, s));
        }
    };
    for (name, i, e) in model.belt_ends() {
        if skip == Some(name.as_str()) {
            continue;
        }
        for (end, output) in [(i, false), (e, true)] {
            offer(
                d2(p, end),
                Snap {
                    at: end,
                    what: "porta",
                    object: Some(name.clone()),
                    output: Some(output),
                },
                &mut best,
            );
        }
    }
    for o in model.objects.iter().filter(|o| {
        !matches!(
            o.kind,
            ObjectKind::Conveyor { .. }
                | ObjectKind::Transporter { .. }
                | ObjectKind::Dispatcher { .. }
        )
    }) {
        offer(
            d2(p, o.position),
            Snap {
                at: [o.position[0], o.position[1], p[2]],
                what: "porta",
                object: Some(o.name.clone()),
                output: None,
            },
            &mut best,
        );
    }
    if let Some((_, s)) = best {
        return s;
    }
    for o in &model.objects {
        if skip == Some(o.name.as_str()) {
            continue;
        }
        if let Some(r) = tr::valid_route(&o.kind) {
            for n in abs_nodes(o.position, r) {
                offer(
                    d2(p, n),
                    Snap {
                        at: n,
                        what: "no",
                        object: None,
                        output: None,
                    },
                    &mut best,
                );
            }
        }
    }
    for a in &model.aisles {
        for (k, q) in a.points.iter().enumerate() {
            let z = a.heights.get(k).copied().unwrap_or(0.0);
            offer(
                d2(p, [q[0], q[1], 0.0]),
                Snap {
                    at: [q[0], q[1], z],
                    what: "caminho",
                    object: None,
                    output: None,
                },
                &mut best,
            );
        }
    }
    if let Some((_, s)) = best {
        return s;
    }
    if let Some(q) = prev {
        let (dx, dy) = (p[0] - q[0], p[1] - q[1]);
        let len = (dx * dx + dy * dy).sqrt();
        if len > 1e-9 {
            let ang = dy.atan2(dx).to_degrees();
            let snapped = (ang / ANGLE_STEP).round() * ANGLE_STEP;
            // Only when the pointer is near the angle (a third of a step either way).
            if (ang - snapped).abs() < ANGLE_STEP / 3.0 {
                let l = if step > 0.0 {
                    snap_to(len, step).max(step)
                } else {
                    len
                };
                let r = snapped.to_radians();
                return Snap {
                    at: [q[0] + l * r.cos(), q[1] + l * r.sin(), p[2]],
                    what: "angulo",
                    object: None,
                    output: None,
                };
            }
        }
    }
    if step > 0.0 {
        return Snap {
            at: [snap_to(p[0], step), snap_to(p[1], step), p[2]],
            what: "grade",
            object: None,
            output: None,
        };
    }
    Snap {
        at: p,
        what: "livre",
        object: None,
        output: None,
    }
}

/// O que se mostra ao desenhar um trecho de `a` a `b`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Preview {
    /// Comprimento 3D do trecho (m).
    pub length: f64,
    /// Comprimento em planta (m).
    pub run: f64,
    /// Inclinação em % e em graus (com sinal: subindo é positivo).
    pub percent: f64,
    pub degrees: f64,
    /// Passa do limite da ferramenta.
    pub too_steep: bool,
}

/// Inclinação máxima da ferramenta (graus): a do tipo de esteira; num caminho, a de uma
/// rampa de pedestres (8,33 %, 1:12).
pub fn max_slope(tool: &str) -> f64 {
    if tool == FLOOR_PATH {
        (1.0f64 / 12.0).atan().to_degrees()
    } else {
        style_of(tool).max_slope()
    }
}

pub fn preview(tool: &str, a: [f64; 3], b: [f64; 3]) -> Preview {
    let run = d2(a, b);
    let rise = b[2] - a[2];
    let length = (run * run + rise * rise).sqrt();
    let grade = if run > 1e-9 { rise / run } else { 0.0 };
    let degrees = grade.atan().to_degrees();
    Preview {
        length,
        run,
        percent: grade * 100.0,
        degrees,
        too_steep: degrees.abs() > max_slope(tool) + 1e-9,
    }
}

/// Comprimento total (3D, com as curvas) de um trajeto ainda em desenho.
pub fn draft_length(points: &[[f64; 3]], radii: &[f64]) -> f64 {
    tr::path_length(&nodes_of(points, radii, [0.0; 3]))
}

fn nodes_of(points: &[[f64; 3]], radii: &[f64], origin: [f64; 3]) -> Vec<PathNode> {
    points
        .iter()
        .enumerate()
        .map(|(i, p)| PathNode {
            at: [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2]],
            radius: radii.get(i).copied().filter(|r| *r > 0.0),
        })
        .collect()
}

/// Uma esteira desenhada como a cena e o painel a mostram.
#[derive(Clone, Debug, PartialEq)]
pub struct RouteRow {
    pub name: String,
    pub style: BeltStyle,
    pub drive: BeltDrive,
    /// Nós absolutos e raios (0: quina viva).
    pub nodes: Vec<[f64; 3]>,
    pub radii: Vec<f64>,
    /// A linha real (curvas amostradas), absoluta.
    pub line: Vec<[f64; 3]>,
    pub width: f64,
    pub length: f64,
    /// Maior inclinação (graus) e o limite.
    pub max_degrees: f64,
    pub limit: f64,
    /// Velocidade equivalente (m/s) e tempo de passagem (s).
    pub speed: f64,
    pub transit: f64,
    /// Itens que cabem (comprimento ÷ item).
    pub capacity: usize,
}

pub fn routes(model: &Model) -> Vec<RouteRow> {
    model
        .objects
        .iter()
        .filter_map(|o| {
            let r = tr::valid_route(&o.kind)?;
            let ObjectKind::Conveyor {
                speed, item_length, ..
            } = &o.kind
            else {
                return None;
            };
            let line = r.line(o.position);
            let length = r.length();
            let v = r.speed(*speed);
            Some(RouteRow {
                name: o.name.clone(),
                style: r.style,
                drive: r.drive,
                nodes: abs_nodes(o.position, r),
                radii: r.nodes.iter().map(|n| n.radius.unwrap_or(0.0)).collect(),
                max_degrees: utsim_nav::path3d::degrees(tr::max_grade(&line)),
                line,
                width: r.width(),
                length,
                limit: r.max_slope(),
                speed: v,
                transit: if v > 0.0 { length / v } else { 0.0 },
                capacity: if *item_length > 0.0 {
                    (length / item_length).floor() as usize
                } else {
                    0
                },
            })
        })
        .collect()
}

/// Nó de trajeto a menos de `radius` de `p` (em planta): `(esteira ou caminho, índice,
/// é caminho)`; o mais perto.
pub fn node_near(model: &Model, p: [f64; 3], radius: f64) -> Option<(String, usize, bool)> {
    let mut best: Option<(f64, (String, usize, bool))> = None;
    let mut take = |d: f64, v: (String, usize, bool)| {
        if d < radius && best.as_ref().is_none_or(|(bd, _)| d < *bd) {
            best = Some((d, v));
        }
    };
    for r in routes(model) {
        for (i, n) in r.nodes.iter().enumerate() {
            take(d2(p, *n), (r.name.clone(), i, false));
        }
    }
    for a in &model.aisles {
        for (i, q) in a.points.iter().enumerate() {
            take(d2(p, [q[0], q[1], 0.0]), (a.name.clone(), i, true));
        }
    }
    best.map(|(_, v)| v)
}

/// Trecho de trajeto cuja linha passa a menos de `radius` de `p`: `(nome, índice do nó
/// de onde o trecho sai, é caminho)`.
pub fn segment_near(model: &Model, p: [f64; 3], radius: f64) -> Option<(String, usize, bool)> {
    let q = [p[0], p[1]];
    let mut best: Option<(f64, (String, usize, bool))> = None;
    let mut take = |d: f64, v: (String, usize, bool)| {
        if d < radius && best.as_ref().is_none_or(|(bd, _)| d < *bd) {
            best = Some((d, v));
        }
    };
    let seg =
        |a: [f64; 3], b: [f64; 3]| utsim_nav::geom::point_segment(q, [a[0], a[1]], [b[0], b[1]]);
    for r in routes(model) {
        for (i, w) in r.nodes.windows(2).enumerate() {
            take(seg(w[0], w[1]), (r.name.clone(), i, false));
        }
    }
    for a in &model.aisles {
        for (i, w) in a.points.windows(2).enumerate() {
            take(
                seg([w[0][0], w[0][1], 0.0], [w[1][0], w[1][1], 0.0]),
                (a.name.clone(), i, true),
            );
        }
    }
    best.map(|(_, v)| v)
}

fn round_mm(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0 + 0.0
}

fn round3(p: [f64; 3]) -> [f64; 3] {
    p.map(round_mm)
}

impl Editor {
    fn route_of(&self, name: &str) -> Result<([f64; 3], BeltRoute), String> {
        let o = self
            .model()
            .objects
            .iter()
            .find(|o| o.name == name)
            .ok_or_else(|| format!("'{name}' não existe"))?;
        match &o.kind {
            ObjectKind::Conveyor { route: Some(r), .. } => Ok((o.position, (**r).clone())),
            ObjectKind::Conveyor { .. } => Err(format!("'{name}' é uma esteira reta, sem nós")),
            _ => Err(format!("'{name}' não é uma esteira")),
        }
    }

    fn put_route(&mut self, name: &str, route: BeltRoute) -> Result<(), String> {
        self.apply(Command::SetRoute {
            name: name.to_string(),
            route: Some(Box::new(route)),
        })
    }

    /// Esteira nova pelos `points` (absolutos; ao menos 2 diferentes), com os raios por
    /// nó, do tipo da ferramenta. Fica ligada aos objetos das portas onde começa e acaba
    /// (`from` → esteira → `to`). Um passo de desfazer; devolve o nome.
    pub fn add_belt_route(
        &mut self,
        tool: &str,
        points: &[[f64; 3]],
        radii: &[f64],
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<String, String> {
        let mut pts: Vec<[f64; 3]> = points.iter().map(|p| round3(*p)).collect();
        pts.dedup();
        if pts.len() < 2 {
            return Err("a esteira precisa de ao menos 2 nós diferentes".into());
        }
        let origin = [pts[0][0], pts[0][1], 0.0];
        let route = BeltRoute {
            nodes: nodes_of(&pts, radii, origin),
            style: style_of(tool),
            drive: BeltDrive::Powered,
            max_slope: None,
            width: None,
        };
        if let Some(p) = tr::problem(&route.nodes, "esteira") {
            return Err(p);
        }
        let base = if tool == BELT_CHANNELED {
            "esteira_canaletas"
        } else {
            "esteira"
        };
        let name = self.doc.unique_name(base);
        let obj = utsim_edit::from_palette("Conveyor", &name, origin)
            .ok_or("a biblioteca não tem esteira")?;
        let mut batch = vec![
            Command::Add(Box::new(obj)),
            Command::SetRoute {
                name: name.clone(),
                route: Some(Box::new(route)),
            },
        ];
        if let Some(f) = from.filter(|f| *f != name) {
            batch.push(Command::Connect {
                from: f.to_string(),
                to: name.clone(),
            });
        }
        if let Some(t) = to.filter(|t| *t != name && Some(*t) != from) {
            batch.push(Command::Connect {
                from: name.clone(),
                to: t.to_string(),
            });
        }
        self.apply(Command::Batch(batch))?;
        Ok(name)
    }

    /// Caminho no piso novo (corredor de pedestres) pelos `points`, com alturas (rampas) e
    /// raios. Um passo de desfazer; devolve o nome.
    pub fn add_floor_path(
        &mut self,
        points: &[[f64; 3]],
        radii: &[f64],
        width: f64,
    ) -> Result<String, String> {
        let mut pts: Vec<[f64; 3]> = points.iter().map(|p| round3(*p)).collect();
        pts.dedup();
        if pts.len() < 2 {
            return Err("o caminho precisa de ao menos 2 pontos diferentes".into());
        }
        let name = self.doc.unique_aisle_name();
        let mut aisle = Aisle {
            name: name.clone(),
            kind: AisleKind::Pedestrian,
            points: pts.iter().map(|p| [p[0], p[1]]).collect(),
            width: if width > 0.0 { width } else { PATH_WIDTH },
            two_way: true,
            secondary: false,
            profile: None,
            heights: Vec::new(),
            radii: Vec::new(),
        };
        for (i, p) in pts.iter().enumerate() {
            aisle.set_height(i, p[2]);
            aisle.set_radius(i, radii.get(i).copied().unwrap_or(0.0));
        }
        if let Some(p) = aisle.problem() {
            return Err(p);
        }
        self.apply(Command::AddAisle(Box::new(aisle)))?;
        Ok(name)
    }

    fn edit_route(
        &mut self,
        name: &str,
        f: impl FnOnce(&mut BeltRoute, [f64; 3]),
    ) -> Result<(), String> {
        let (origin, mut r) = self.route_of(name)?;
        f(&mut r, origin);
        self.put_route(name, r)
    }

    fn edit_path(&mut self, name: &str, f: impl FnOnce(&mut Aisle)) -> Result<(), String> {
        let mut a = self
            .model()
            .aisles
            .iter()
            .find(|a| a.name == name)
            .cloned()
            .ok_or_else(|| format!("não existe o caminho '{name}'"))?;
        f(&mut a);
        self.apply(Command::SetAisle {
            name: name.to_string(),
            aisle: Box::new(a),
        })
    }

    /// Põe o nó `index` em `p` (absoluto: X, Y e altura exatos).
    pub fn set_route_node(
        &mut self,
        name: &str,
        path: bool,
        index: usize,
        p: [f64; 3],
    ) -> Result<(), String> {
        let p = round3(p);
        if path {
            return self.edit_path(name, |a| {
                if index < a.points.len() {
                    a.points[index] = [p[0], p[1]];
                    a.set_height(index, p[2]);
                }
            });
        }
        let (_, r) = self.route_of(name)?;
        if index >= r.nodes.len() {
            return Err(format!("'{name}' não tem o nó {}", index + 1));
        }
        self.edit_route(name, |r, o| {
            r.nodes[index].at = [p[0] - o[0], p[1] - o[1], p[2] - o[2]];
        })
    }

    /// Desloca os nós `indices` por `d` (mover vários juntos; o ajuste fino pelo teclado).
    pub fn nudge_route_nodes(
        &mut self,
        name: &str,
        path: bool,
        indices: &[usize],
        d: [f64; 3],
    ) -> Result<(), String> {
        if path {
            return self.edit_path(name, |a| {
                let n = a.points.len();
                for &i in indices.iter().filter(|i| **i < n) {
                    a.points[i] = [
                        round_mm(a.points[i][0] + d[0]),
                        round_mm(a.points[i][1] + d[1]),
                    ];
                    let z = a.heights.get(i).copied().unwrap_or(0.0);
                    a.set_height(i, round_mm(z + d[2]));
                }
            });
        }
        self.edit_route(name, |r, _| {
            let n = r.nodes.len();
            for &i in indices.iter().filter(|i| **i < n) {
                let n = &mut r.nodes[i];
                n.at = round3([n.at[0] + d[0], n.at[1] + d[1], n.at[2] + d[2]]);
            }
        })
    }

    /// Raio da curva no nó `index` (0: quina viva).
    pub fn set_route_radius(
        &mut self,
        name: &str,
        path: bool,
        index: usize,
        radius: f64,
    ) -> Result<(), String> {
        if !(radius.is_finite() && radius >= 0.0) {
            return Err("o raio deve ser um número >= 0 (metros)".into());
        }
        if path {
            return self.edit_path(name, |a| a.set_radius(index, radius));
        }
        self.edit_route(name, |r, _| {
            if let Some(n) = r.nodes.get_mut(index) {
                n.radius = (radius > 0.0).then_some(round_mm(radius));
            }
        })
    }

    /// Divide o trecho que sai do nó `index` com um nó novo em `p` (absoluto).
    pub fn insert_route_node(
        &mut self,
        name: &str,
        path: bool,
        index: usize,
        p: [f64; 3],
    ) -> Result<(), String> {
        let p = round3(p);
        if path {
            return self.edit_path(name, |a| a.insert_point(index + 1, [p[0], p[1]], p[2]));
        }
        self.edit_route(name, |r, o| {
            let at = (index + 1).min(r.nodes.len());
            r.nodes
                .insert(at, PathNode::new([p[0] - o[0], p[1] - o[1], p[2] - o[2]]));
        })
    }

    /// Tira o nó `index` (ficam ao menos 2).
    pub fn remove_route_node(
        &mut self,
        name: &str,
        path: bool,
        index: usize,
    ) -> Result<(), String> {
        if path {
            let n = self
                .model()
                .aisles
                .iter()
                .find(|a| a.name == name)
                .map_or(0, |a| a.points.len());
            if n <= 2 {
                return Err("o caminho precisa de ao menos 2 pontos".into());
            }
            return self.edit_path(name, |a| a.remove_point(index));
        }
        let (_, r) = self.route_of(name)?;
        if r.nodes.len() <= 2 {
            return Err("a esteira precisa de ao menos 2 nós".into());
        }
        self.edit_route(name, |r, _| {
            if index < r.nodes.len() {
                r.nodes.remove(index);
            }
        })
    }

    /// Prolonga a esteira pela ponta de saída (ou de entrada, com `at_start`) até `p`.
    pub fn extend_route(&mut self, name: &str, at_start: bool, p: [f64; 3]) -> Result<(), String> {
        let p = round3(p);
        self.edit_route(name, |r, o| {
            let n = PathNode::new([p[0] - o[0], p[1] - o[1], p[2] - o[2]]);
            if at_start {
                r.nodes.insert(0, n);
            } else {
                r.nodes.push(n);
            }
        })
    }

    /// Nós `from..=to` em inclinação constante entre as duas pontas (nivelado se as duas
    /// têm a mesma altura).
    pub fn constant_slope(
        &mut self,
        name: &str,
        path: bool,
        from: usize,
        to: usize,
    ) -> Result<(), String> {
        let (from, to) = (from.min(to), from.max(to));
        if path {
            return self.edit_path(name, |a| {
                let mut nodes = a.nodes();
                utsim_nav::path3d::constant_slope(&mut nodes, from, to);
                for (i, n) in nodes.iter().enumerate() {
                    a.set_height(i, round_mm(n.at[2]));
                }
            });
        }
        self.edit_route(name, |r, _| {
            utsim_nav::path3d::constant_slope(&mut r.nodes, from, to);
            for n in &mut r.nodes {
                n.at[2] = round_mm(n.at[2]);
            }
        })
    }

    /// Todos os nós `from..=to` na altura `z` (absoluta).
    pub fn level_route(
        &mut self,
        name: &str,
        path: bool,
        from: usize,
        to: usize,
        z: f64,
    ) -> Result<(), String> {
        let (from, to) = (from.min(to), from.max(to));
        let z = round_mm(z);
        if path {
            return self.edit_path(name, |a| {
                for i in from..=to.min(a.points.len().saturating_sub(1)) {
                    a.set_height(i, z);
                }
            });
        }
        self.edit_route(name, |r, o| {
            for n in r.nodes.iter_mut().take(to + 1).skip(from) {
                n.at[2] = z - o[2];
            }
        })
    }

    pub fn set_route_style(&mut self, name: &str, style: &str) -> Result<(), String> {
        let s = match style {
            "simples" => BeltStyle::Simple,
            "canaletas" => BeltStyle::Channeled,
            _ => return Err(format!("tipo de esteira desconhecido: {style}")),
        };
        self.edit_route(name, |r, _| r.style = s)
    }

    pub fn set_route_drive(&mut self, name: &str, drive: &str) -> Result<(), String> {
        let d = match drive {
            "motorizada" => BeltDrive::Powered,
            "gravidade" => BeltDrive::Gravity,
            _ => return Err(format!("acionamento desconhecido: {drive}")),
        };
        self.edit_route(name, |r, _| r.drive = d)
    }

    pub fn set_route_width(&mut self, name: &str, width: f64) -> Result<(), String> {
        if !(width.is_finite() && width > 0.0) {
            return Err("a largura deve ser > 0 (metros)".into());
        }
        self.edit_route(name, |r, _| r.width = Some(round_mm(width)))
    }

    /// Inclinação máxima própria (graus); 0: a do tipo.
    pub fn set_route_max_slope(&mut self, name: &str, degrees: f64) -> Result<(), String> {
        if !(degrees.is_finite() && (0.0..90.0).contains(&degrees)) {
            return Err("a inclinação máxima fica entre 0 e 90 graus".into());
        }
        self.edit_route(name, |r, _| {
            r.max_slope = (degrees > 0.0).then_some(degrees)
        })
    }

    /// Divide a esteira no nó `index` (do meio) em duas, ligadas em série: a primeira
    /// fica com o nome e as entradas, a segunda com as saídas. Um passo de desfazer;
    /// devolve o nome da segunda.
    pub fn split_route(&mut self, name: &str, index: usize) -> Result<String, String> {
        let (origin, r) = self.route_of(name)?;
        if index == 0 || index + 1 >= r.nodes.len() {
            return Err("divida num nó do meio da esteira".into());
        }
        let obj = self
            .model()
            .objects
            .iter()
            .find(|o| o.name == name)
            .cloned()
            .ok_or_else(|| format!("'{name}' não existe"))?;
        let abs = abs_nodes(origin, &r);
        let mut first = r.clone();
        first.nodes.truncate(index + 1);
        if let Some(n) = first.nodes.last_mut() {
            n.radius = None;
        }
        let tail: Vec<[f64; 3]> = abs[index..].to_vec();
        let o2 = [tail[0][0], tail[0][1], origin[2]];
        let mut second = r.clone();
        second.nodes = tail
            .iter()
            .zip(r.nodes[index..].iter())
            .map(|(p, n)| PathNode {
                at: round3([p[0] - o2[0], p[1] - o2[1], p[2] - o2[2]]),
                radius: n.radius,
            })
            .collect();
        second.nodes[0].radius = None;
        let name2 = self.doc.unique_name(&obj.name);
        let mut copy = obj.clone();
        copy.name = name2.clone();
        copy.position = o2;
        let outs: Vec<String> = self
            .model()
            .connections
            .iter()
            .filter(|(f, _)| f == name)
            .map(|(_, t)| t.clone())
            .collect();
        let mut batch = vec![
            Command::Add(Box::new(copy)),
            Command::SetRoute {
                name: name2.clone(),
                route: Some(Box::new(second)),
            },
            Command::SetRoute {
                name: name.to_string(),
                route: Some(Box::new(first)),
            },
        ];
        for t in &outs {
            batch.push(Command::Disconnect {
                from: name.to_string(),
                to: t.clone(),
            });
            batch.push(Command::Connect {
                from: name2.clone(),
                to: t.clone(),
            });
        }
        batch.push(Command::Connect {
            from: name.to_string(),
            to: name2.clone(),
        });
        self.apply(Command::Batch(batch))?;
        Ok(name2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn belt(e: &mut Editor, pts: &[[f64; 3]]) -> String {
        e.add_belt_route(BELT_SIMPLE, pts, &[], None, None).unwrap()
    }

    #[test]
    fn a_drawn_belt_is_one_undo_step_and_its_length_is_the_route() {
        let mut e = Editor::new("t");
        let name = belt(&mut e, &[[1.0, 2.0, 0.8], [5.0, 2.0, 0.8], [5.0, 5.0, 1.8]]);
        let rows = routes(e.model());
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].nodes[0], [1.0, 2.0, 0.8]);
        let want = 4.0 + 10f64.sqrt();
        assert!((rows[0].length - want).abs() < 1e-9);
        let o = e.model().objects.iter().find(|o| o.name == name).unwrap();
        assert!(
            matches!(o.kind, ObjectKind::Conveyor { length, .. } if (length - want).abs() < 1e-3)
        );
        e.doc.undo();
        assert!(e.model().objects.is_empty());
    }

    #[test]
    fn a_node_height_is_set_exactly_and_nudged() {
        let mut e = Editor::new("t");
        let name = belt(&mut e, &[[0.0, 0.0, 0.8], [4.0, 0.0, 0.8]]);
        e.set_route_node(&name, false, 1, [4.0, 0.0, 1.8]).unwrap();
        assert_eq!(routes(e.model())[0].nodes[1], [4.0, 0.0, 1.8]);
        e.nudge_route_nodes(&name, false, &[0, 1], [0.0, 0.0, 0.01])
            .unwrap();
        let r = &routes(e.model())[0];
        assert_eq!(r.nodes[0][2], 0.81);
        assert_eq!(r.nodes[1][2], 1.81);
        e.doc.undo();
        assert_eq!(routes(e.model())[0].nodes[0][2], 0.8);
    }

    #[test]
    fn moving_the_belt_moves_its_nodes() {
        let mut e = Editor::new("t");
        let name = belt(&mut e, &[[0.0, 0.0, 0.8], [4.0, 0.0, 0.8]]);
        e.apply(Command::Move {
            name: name.clone(),
            position: [10.0, 0.0, 0.0],
        })
        .unwrap();
        assert_eq!(routes(e.model())[0].nodes[1], [14.0, 0.0, 0.8]);
    }

    #[test]
    fn constant_slope_and_level() {
        let mut e = Editor::new("t");
        let name = belt(&mut e, &[[0.0, 0.0, 0.8], [2.0, 0.0, 3.0], [4.0, 0.0, 1.8]]);
        e.constant_slope(&name, false, 0, 2).unwrap();
        assert_eq!(routes(e.model())[0].nodes[1][2], 1.3);
        e.level_route(&name, false, 0, 2, 1.0).unwrap();
        assert!(routes(e.model())[0].nodes.iter().all(|n| n[2] == 1.0));
    }

    #[test]
    fn a_belt_is_split_in_two_in_series() {
        let mut e = Editor::new("t");
        let name = belt(&mut e, &[[0.0, 0.0, 0.8], [4.0, 0.0, 0.8], [4.0, 4.0, 0.8]]);
        let out = e.add("Sink", [8.0, 4.0, 0.0]).unwrap();
        e.apply(Command::Connect {
            from: name.clone(),
            to: out.clone(),
        })
        .unwrap();
        let second = e.split_route(&name, 1).unwrap();
        let m = e.model();
        assert!(m.connections.contains(&(name.clone(), second.clone())));
        assert!(m.connections.contains(&(second.clone(), out)));
        let rows = routes(m);
        let total: f64 = rows.iter().map(|r| r.length).sum();
        assert!((total - 8.0).abs() < 1e-9);
        assert!(m.validate().is_ok());
    }

    #[test]
    fn a_new_belt_snaps_to_a_port_and_connects_to_it() {
        let mut e = Editor::new("t");
        let src = e.add("Source", [0.0, 0.0, 0.0]).unwrap();
        let s = snap(e.model(), [0.2, 0.1, 0.8], 0.5, 0.5, None, None);
        assert_eq!(s.what, "porta");
        assert_eq!(s.object.as_deref(), Some(src.as_str()));
        let name = e
            .add_belt_route(
                BELT_CHANNELED,
                &[s.at, [5.0, 0.0, 0.8]],
                &[],
                s.object.as_deref(),
                None,
            )
            .unwrap();
        assert!(e.model().connections.contains(&(src, name.clone())));
        assert_eq!(routes(e.model())[0].style, BeltStyle::Channeled);
        // The next belt starts at this one's output end, at its height.
        let s = snap(e.model(), [5.1, 0.1, 0.0], 0.5, 0.5, None, None);
        assert_eq!(s.at, [5.0, 0.0, 0.8]);
        assert_eq!(s.output, Some(true));
    }

    #[test]
    fn angles_snap_in_steps_of_15_degrees() {
        let e = Editor::new("t");
        let s = snap(
            e.model(),
            [3.0, 0.1, 0.8],
            0.5,
            0.3,
            Some([0.0, 0.0, 0.8]),
            None,
        );
        assert_eq!(s.what, "angulo");
        assert!((s.at[1]).abs() < 1e-9 && (s.at[0] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn preview_shows_length_and_slope() {
        let p = preview(BELT_SIMPLE, [0.0, 0.0, 0.0], [4.0, 0.0, 3.0]);
        assert_eq!(p.length, 5.0);
        assert_eq!(p.percent, 75.0);
        assert!(p.too_steep);
        let ramp = preview(FLOOR_PATH, [0.0, 0.0, 0.0], [12.0, 0.0, 1.0]);
        assert!(!ramp.too_steep);
    }

    #[test]
    fn a_floor_path_is_an_aisle_with_ramps_and_curves() {
        let mut e = Editor::new("t");
        let name = e
            .add_floor_path(
                &[[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 6.0, 0.5]],
                &[0.0, 2.0, 0.0],
                0.0,
            )
            .unwrap();
        let a = &e.model().aisles[0];
        assert_eq!(a.name, name);
        assert_eq!(a.heights, vec![0.0, 0.0, 0.5]);
        assert_eq!(a.radii, vec![0.0, 2.0, 0.0]);
        e.nudge_route_nodes(&name, true, &[2], [0.0, 0.0, -0.5])
            .unwrap();
        assert!(e.model().aisles[0].heights.is_empty());
        e.insert_route_node(&name, true, 0, [5.0, 0.0, 0.0])
            .unwrap();
        assert_eq!(e.model().aisles[0].points.len(), 4);
        assert_eq!(e.model().aisles[0].radii, vec![0.0, 0.0, 2.0, 0.0]);
    }
}
