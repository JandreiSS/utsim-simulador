//! `UtsimTrajetos`: a ponte das esteiras e dos caminhos desenhados por nós
//! ([`crate::path_edit`]). Liga-se a um `UtsimDocument` (Editar) e também lê as esteiras de
//! um `UtsimPlayback` (a cena, nos dois modos).
//!
//! Coordenadas: o modelo `(x, y, z)` vira o Godot `(x, z, -y)`; a altura de um nó é o `y`
//! do Godot.

use godot::prelude::*;
use utsim_model::Model;

use crate::UtsimPlayback;
use crate::document_class::UtsimDocument;
use crate::path_edit::{self as pe, RouteRow};
use crate::put;

/// Ponto do modelo no Godot.
pub fn g(p: [f64; 3]) -> Vector3 {
    Vector3::new(p[0] as f32, p[2] as f32, -(p[1] as f32))
}

/// Ponto do Godot no modelo.
pub fn m(v: Vector3) -> [f64; 3] {
    [f64::from(v.x), -f64::from(v.z), f64::from(v.y)]
}

fn line(points: &[[f64; 3]]) -> PackedVector3Array {
    points.iter().map(|p| g(*p)).collect()
}

fn floats(v: &[f64]) -> PackedFloat64Array {
    v.iter().copied().collect()
}

fn route_dict(r: &RouteRow) -> VarDictionary {
    let mut d = VarDictionary::new();
    put(&mut d, "name", GString::from(r.name.as_str()));
    put(&mut d, "path", false);
    put(&mut d, "style", GString::from(pe::style_name(r.style)));
    put(&mut d, "drive", GString::from(pe::drive_name(r.drive)));
    put(&mut d, "nodes", line(&r.nodes));
    put(&mut d, "radii", floats(&r.radii));
    put(&mut d, "line", line(&r.line));
    put(&mut d, "width", r.width);
    put(&mut d, "length", r.length);
    put(&mut d, "max_degrees", r.max_degrees);
    put(&mut d, "limit", r.limit);
    put(&mut d, "too_steep", r.max_degrees > r.limit + 1e-9);
    put(&mut d, "speed", r.speed);
    put(&mut d, "transit", r.transit);
    put(&mut d, "capacity", r.capacity as i64);
    d
}

fn paths(model: &Model) -> VarArray {
    model
        .aisles
        .iter()
        .filter(|a| a.problem().is_none())
        .map(|a| {
            let mut d = VarDictionary::new();
            let nodes: Vec<[f64; 3]> = a.nodes().iter().map(|n| n.at).collect();
            let center = a.centerline();
            put(&mut d, "name", GString::from(a.name.as_str()));
            put(&mut d, "path", true);
            put(
                &mut d,
                "kind",
                GString::from(crate::aisle_edit::kind_name(a.kind)),
            );
            put(&mut d, "nodes", line(&nodes));
            put(
                &mut d,
                "radii",
                floats(
                    &a.nodes()
                        .iter()
                        .map(|n| n.radius.unwrap_or(0.0))
                        .collect::<Vec<_>>(),
                ),
            );
            put(&mut d, "line", line(&center));
            put(&mut d, "width", a.width);
            put(&mut d, "stair", a.stair);
            put(&mut d, "length", a.length());
            let worst = utsim_nav::path3d::degrees(utsim_nav::path3d::max_grade(&center));
            let limit = pe::max_slope(pe::FLOOR_PATH);
            put(&mut d, "max_degrees", worst);
            put(&mut d, "limit", limit);
            put(&mut d, "too_steep", worst > limit + 1e-9);
            d.to_variant()
        })
        .collect()
}

fn hit(v: Option<(String, usize, bool)>) -> VarDictionary {
    let mut d = VarDictionary::new();
    if let Some((name, index, path)) = v {
        put(&mut d, "name", GString::from(name.as_str()));
        put(&mut d, "index", index as i64);
        put(&mut d, "path", path);
    }
    d
}

fn pts(a: &PackedVector3Array) -> Vec<[f64; 3]> {
    a.as_slice().iter().map(|v| m(*v)).collect()
}

/// Esteiras e caminhos desenhados por nós.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimTrajetos {
    doc: Option<Gd<UtsimDocument>>,
    last_error: String,
    base: Base<RefCounted>,
}

impl UtsimTrajetos {
    fn model<T>(&self, f: impl FnOnce(&Model) -> T) -> Option<T> {
        let d = self.doc.as_ref()?;
        let b = d.bind();
        Some(f(b.editor.model()))
    }

    /// Runs an edit on the document's editor; `true` when it went through.
    fn edit<T>(
        &mut self,
        f: impl FnOnce(&mut crate::document::Editor) -> Result<T, String>,
    ) -> Option<T> {
        let Some(d) = self.doc.as_mut() else {
            self.last_error = "sem documento".into();
            return None;
        };
        let r = f(&mut d.bind_mut().editor);
        match r {
            Ok(v) => {
                self.last_error.clear();
                Some(v)
            }
            Err(e) => {
                self.last_error = e;
                None
            }
        }
    }
}

#[godot_api]
impl UtsimTrajetos {
    /// Liga ao documento do Editar.
    #[func]
    fn ligar(&mut self, doc: Gd<UtsimDocument>) {
        self.doc = Some(doc);
    }

    #[func]
    fn last_error(&self) -> GString {
        GString::from(self.last_error.as_str())
    }

    /// Os ids da biblioteca das três ferramentas.
    #[func]
    fn ferramentas() -> PackedStringArray {
        pe::IDS.iter().map(|s| GString::from(*s)).collect()
    }

    /// Inclinação máxima (graus) da ferramenta.
    #[func]
    fn limite(tool: GString) -> f64 {
        pe::max_slope(&tool.to_string())
    }

    /// Encaixe de `at` (o `y` é a altura que a ferramenta quer): `{at, what, object,
    /// output}` (`output`: 1 ponta de saída, 0 de entrada, -1 outro).
    #[func]
    fn encaixe(
        &self,
        at: Vector3,
        step: f64,
        radius: f64,
        prev: Variant,
        skip: GString,
    ) -> VarDictionary {
        self.encaixe_tangente(at, step, radius, prev, Variant::nil(), skip)
    }

    /// `encaixe` com a direção do trecho anterior (`tangent`, um Vector3 do Godot, ou
    /// null): perto dela o nó segue reto (`what` = "tangente").
    #[func]
    fn encaixe_tangente(
        &self,
        at: Vector3,
        step: f64,
        radius: f64,
        prev: Variant,
        tangent: Variant,
        skip: GString,
    ) -> VarDictionary {
        let prev = prev.try_to::<Vector3>().ok().map(m);
        let tangent = tangent
            .try_to::<Vector3>()
            .ok()
            .map(|v| [f64::from(v.x), -f64::from(v.z)]);
        let skip = skip.to_string();
        let skip = (!skip.is_empty()).then_some(skip.as_str());
        let s = self
            .model(|model| pe::snap_along(model, m(at), step, radius, prev, tangent, skip))
            .unwrap_or(pe::Snap {
                at: m(at),
                what: "livre",
                object: None,
                output: None,
            });
        let mut d = VarDictionary::new();
        put(&mut d, "at", g(s.at));
        put(&mut d, "what", GString::from(s.what));
        put(
            &mut d,
            "object",
            GString::from(s.object.as_deref().unwrap_or("")),
        );
        put(&mut d, "output", s.output.map_or(-1, i64::from));
        d
    }

    /// Prévia do trecho de `a` a `b`: `{length, run, percent, degrees, too_steep, limit}`.
    #[func]
    fn previa(tool: GString, a: Vector3, b: Vector3) -> VarDictionary {
        let tool = tool.to_string();
        let p = pe::preview(&tool, m(a), m(b));
        let mut d = VarDictionary::new();
        put(&mut d, "length", p.length);
        put(&mut d, "run", p.run);
        put(&mut d, "percent", p.percent);
        put(&mut d, "degrees", p.degrees);
        put(&mut d, "too_steep", p.too_steep);
        put(&mut d, "limit", pe::max_slope(&tool));
        d
    }

    /// A linha real (curvas amostradas) de um trajeto em desenho e o seu comprimento:
    /// `{line, length}`.
    #[func]
    fn rascunho(points: PackedVector3Array, radii: PackedFloat64Array) -> VarDictionary {
        let p = pts(&points);
        let r = radii.as_slice().to_vec();
        let nodes: Vec<utsim_model::PathNode> = p
            .iter()
            .enumerate()
            .map(|(i, q)| utsim_model::PathNode {
                at: *q,
                radius: r.get(i).copied().filter(|v| *v > 0.0),
            })
            .collect();
        let mut d = VarDictionary::new();
        put(&mut d, "line", line(&utsim_model::trajeto::sample(&nodes)));
        put(&mut d, "length", pe::draft_length(&p, &r));
        d
    }

    /// As esteiras desenhadas do documento.
    #[func]
    fn esteiras(&self) -> VarArray {
        self.model(|model| {
            pe::routes(model)
                .iter()
                .map(|r| route_dict(r).to_variant())
                .collect()
        })
        .unwrap_or_default()
    }

    /// Os caminhos no piso (corredores) com as alturas e as curvas.
    #[func]
    fn caminhos(&self) -> VarArray {
        self.model(paths).unwrap_or_default()
    }

    /// As esteiras desenhadas de uma reprodução (a cena), com o índice do bloco:
    /// `{block, ...}` como `esteiras()`.
    #[func]
    fn da_cena(pb: Gd<UtsimPlayback>) -> VarArray {
        let b = pb.bind();
        let Some(core) = b.core.as_ref() else {
            return VarArray::new();
        };
        let Some(model) = core.model.as_ref() else {
            return VarArray::new();
        };
        // The index of the block as `static_scene()` lists it (what the scene builds).
        let names: Vec<String> = core.blocks.iter().map(|b| b.name.clone()).collect();
        pe::routes(model)
            .iter()
            .filter_map(|r| {
                let i = names.iter().position(|n| *n == r.name)?;
                let mut d = route_dict(r);
                put(&mut d, "block", i as i64);
                Some(d.to_variant())
            })
            .collect()
    }

    /// Quanto a superfície de cada esteira desenhada andou até `t` (m), na ordem de
    /// `da_cena`: para quando a esteira quebra, sai do turno ou, sem acúmulo, enquanto a
    /// peça da frente espera; na de gravidade, só enquanto leva peças (os roletes giram
    /// sob elas).
    #[func]
    fn avancos(pb: Gd<UtsimPlayback>, t: f64) -> PackedFloat64Array {
        let b = pb.bind();
        let Some(core) = b.core.as_ref() else {
            return PackedFloat64Array::new();
        };
        let Some(model) = core.model.as_ref() else {
            return PackedFloat64Array::new();
        };
        pe::routes(model)
            .iter()
            .filter_map(|r| {
                let view = core.blocks.iter().find(|v| v.name == r.name)?;
                let gravity = r.drive == utsim_model::BeltDrive::Gravity;
                Some(
                    core.scene
                        .timeline
                        .belt_track(view.b)
                        .map_or(0.0, |tr| tr.surface_travel(t, gravity)),
                )
            })
            .collect()
    }

    /// Os caminhos no piso de uma reprodução (a cena desenha as rampas).
    #[func]
    fn caminhos_da_cena(pb: Gd<UtsimPlayback>) -> VarArray {
        let b = pb.bind();
        b.core
            .as_ref()
            .and_then(|c| c.model.as_ref())
            .map(paths)
            .unwrap_or_default()
    }

    /// Nó de esteira ou de caminho a menos de `radius` m de `at` (em planta): `{name,
    /// index, path}` ou `{}`.
    #[func]
    fn no_perto(&self, at: Vector3, radius: f64) -> VarDictionary {
        hit(self
            .model(|model| pe::node_near(model, m(at), radius))
            .flatten())
    }

    /// Trecho a menos de `radius` m de `at`: `{name, index (nó de onde sai), path}` ou `{}`.
    #[func]
    fn trecho_perto(&self, at: Vector3, radius: f64) -> VarDictionary {
        hit(self
            .model(|model| pe::segment_near(model, m(at), radius))
            .flatten())
    }

    /// Esteira nova da ferramenta `tool` (ligada a `from` e `to`, se dados); devolve o nome
    /// ou "" (ver `last_error`).
    #[func]
    fn nova_esteira(
        &mut self,
        tool: GString,
        points: PackedVector3Array,
        radii: PackedFloat64Array,
        from: GString,
        to: GString,
    ) -> GString {
        let (tool, p, r) = (tool.to_string(), pts(&points), radii.as_slice().to_vec());
        let (from, to) = (from.to_string(), to.to_string());
        let from = (!from.is_empty()).then_some(from.as_str());
        let to = (!to.is_empty()).then_some(to.as_str());
        self.edit(|e| e.add_belt_route(&tool, &p, &r, from, to))
            .map(|n| GString::from(n.as_str()))
            .unwrap_or_default()
    }

    /// Esteira nova que pode começar (`from_line`: desvio) ou acabar (`to_line`: junção)
    /// na linha de outra esteira desenhada; devolve o nome ou "".
    #[allow(clippy::too_many_arguments)]
    #[func]
    fn nova_esteira_t(
        &mut self,
        tool: GString,
        points: PackedVector3Array,
        radii: PackedFloat64Array,
        from: GString,
        to: GString,
        from_line: GString,
        to_line: GString,
    ) -> GString {
        let (tool, p, r) = (tool.to_string(), pts(&points), radii.as_slice().to_vec());
        let texts = [from, to, from_line, to_line].map(|g| g.to_string());
        fn opt(s: &str) -> Option<&str> {
            (!s.is_empty()).then_some(s)
        }
        self.edit(|e| {
            e.add_belt_joined(
                &tool,
                &p,
                &r,
                opt(&texts[0]),
                opt(&texts[1]),
                opt(&texts[2]),
                opt(&texts[3]),
            )
        })
        .map(|n| GString::from(n.as_str()))
        .unwrap_or_default()
    }

    /// Caminho no piso novo; devolve o nome ou "".
    #[func]
    fn novo_caminho(
        &mut self,
        points: PackedVector3Array,
        radii: PackedFloat64Array,
        width: f64,
    ) -> GString {
        let (p, r) = (pts(&points), radii.as_slice().to_vec());
        self.edit(|e| e.add_floor_path(&p, &r, width))
            .map(|n| GString::from(n.as_str()))
            .unwrap_or_default()
    }

    /// Põe o nó em `at` (X, Y e altura exatos).
    #[func]
    fn por_no(&mut self, name: GString, path: bool, index: i64, at: Vector3) -> bool {
        let n = name.to_string();
        self.edit(|e| e.set_route_node(&n, path, index.max(0) as usize, m(at)))
            .is_some()
    }

    /// Desloca os nós `indices` por `d` (Godot), um passo de desfazer.
    #[func]
    fn deslocar(
        &mut self,
        name: GString,
        path: bool,
        indices: PackedInt32Array,
        d: Vector3,
    ) -> bool {
        let n = name.to_string();
        let idx: Vec<usize> = indices
            .as_slice()
            .iter()
            .map(|i| (*i).max(0) as usize)
            .collect();
        let dm = [f64::from(d.x), -f64::from(d.z), f64::from(d.y)];
        self.edit(|e| e.nudge_route_nodes(&n, path, &idx, dm))
            .is_some()
    }

    #[func]
    fn por_raio(&mut self, name: GString, path: bool, index: i64, radius: f64) -> bool {
        let n = name.to_string();
        self.edit(|e| e.set_route_radius(&n, path, index.max(0) as usize, radius))
            .is_some()
    }

    /// Divide o trecho que sai do nó `index` com um nó novo em `at`.
    #[func]
    fn inserir_no(&mut self, name: GString, path: bool, index: i64, at: Vector3) -> bool {
        let n = name.to_string();
        self.edit(|e| e.insert_route_node(&n, path, index.max(0) as usize, m(at)))
            .is_some()
    }

    #[func]
    fn tirar_no(&mut self, name: GString, path: bool, index: i64) -> bool {
        let n = name.to_string();
        self.edit(|e| e.remove_route_node(&n, path, index.max(0) as usize))
            .is_some()
    }

    /// Caminho como escada (só pedestres) ou não.
    #[func]
    fn por_escada(&mut self, name: GString, on: bool) -> bool {
        let n = name.to_string();
        self.edit(|e| e.set_stair(&n, on)).is_some()
    }

    /// Move o trajeto inteiro por `d` (Godot).
    #[func]
    fn mover_trajeto(&mut self, name: GString, path: bool, d: Vector3) -> bool {
        let n = name.to_string();
        let dm = [f64::from(d.x), -f64::from(d.z), f64::from(d.y)];
        self.edit(|e| e.move_route(&n, path, dm)).is_some()
    }

    /// Gira o trajeto inteiro `degrees` (anti-horário visto de cima) em volta do meio.
    #[func]
    fn girar_trajeto(&mut self, name: GString, path: bool, degrees: f64) -> bool {
        let n = name.to_string();
        self.edit(|e| e.rotate_route(&n, path, degrees)).is_some()
    }

    /// Curva o trecho que sai do nó `index` pela alça do meio arrastada até `at`.
    #[func]
    fn curvar(&mut self, name: GString, path: bool, index: i64, at: Vector3) -> bool {
        let n = name.to_string();
        self.edit(|e| e.bend(&n, path, index.max(0) as usize, m(at)))
            .is_some()
    }

    /// Trajeto paralelo a `offset` m (positivo: à esquerda do sentido); devolve o nome ou "".
    #[func]
    fn paralelo(&mut self, name: GString, path: bool, offset: f64) -> GString {
        let n = name.to_string();
        self.edit(|e| e.parallel(&n, path, offset))
            .map(|s| GString::from(s.as_str()))
            .unwrap_or_default()
    }

    /// Rampas de acesso de pedestre (1:12) nas pontas altas de um caminho; devolve
    /// quantas pôs (0: nenhuma, ver `last_error`).
    #[func]
    fn rampas_de_acesso(&mut self, name: GString) -> i64 {
        let n = name.to_string();
        self.edit(|e| e.access_ramps(&n)).map_or(0, |k| k as i64)
    }

    /// Prolonga a esteira pela saída (ou pela entrada, com `at_start`) até `at`.
    #[func]
    fn prolongar(&mut self, name: GString, at_start: bool, at: Vector3) -> bool {
        let n = name.to_string();
        self.edit(|e| e.extend_route(&n, at_start, m(at))).is_some()
    }

    /// Nós `from..=to` em inclinação constante.
    #[func]
    fn inclinacao_constante(&mut self, name: GString, path: bool, from: i64, to: i64) -> bool {
        let n = name.to_string();
        self.edit(|e| e.constant_slope(&n, path, from.max(0) as usize, to.max(0) as usize))
            .is_some()
    }

    /// Nós `from..=to` todos na altura `z` (m).
    #[func]
    fn nivelar(&mut self, name: GString, path: bool, from: i64, to: i64, z: f64) -> bool {
        let n = name.to_string();
        self.edit(|e| e.level_route(&n, path, from.max(0) as usize, to.max(0) as usize, z))
            .is_some()
    }

    /// `simples` ou `canaletas`.
    #[func]
    fn por_tipo(&mut self, name: GString, style: GString) -> bool {
        let (n, s) = (name.to_string(), style.to_string());
        self.edit(|e| e.set_route_style(&n, &s)).is_some()
    }

    /// `motorizada` ou `gravidade`.
    #[func]
    fn por_acionamento(&mut self, name: GString, drive: GString) -> bool {
        let (n, s) = (name.to_string(), drive.to_string());
        self.edit(|e| e.set_route_drive(&n, &s)).is_some()
    }

    #[func]
    fn por_largura(&mut self, name: GString, width: f64) -> bool {
        let n = name.to_string();
        self.edit(|e| e.set_route_width(&n, width)).is_some()
    }

    /// Inclinação máxima própria (graus); 0 volta à do tipo.
    #[func]
    fn por_inclinacao_maxima(&mut self, name: GString, degrees: f64) -> bool {
        let n = name.to_string();
        self.edit(|e| e.set_route_max_slope(&n, degrees)).is_some()
    }

    /// Divide a esteira no nó `index` em duas, em série; devolve o nome da segunda.
    #[func]
    fn dividir(&mut self, name: GString, index: i64) -> GString {
        let n = name.to_string();
        self.edit(|e| e.split_route(&n, index.max(0) as usize))
            .map(|s| GString::from(s.as_str()))
            .unwrap_or_default()
    }
}
