//! A classe `UtsimSeguranca` da ponte: entrega ao GDScript (`camada_seguranca.gd`) os
//! dados prontos da camada de seguranca e qualidade de uma replicacao, calculados em
//! [`crate::seguranca`]. O GDScript so desenha.

use godot::prelude::*;

use crate::seguranca::{Seguranca, heat_values};
use crate::{UtsimModel, put, strings};

/// A camada de seguranca e qualidade de uma replicacao.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimSeguranca {
    core: Option<Seguranca>,
    last_error: String,
    base: Base<RefCounted>,
}

fn ponto(p: [f64; 2], y: f32) -> Vector3 {
    // Plano do modelo (x, y) para o Godot (x, altura, -y).
    Vector3::new(p[0] as f32, y, -(p[1] as f32))
}

#[godot_api]
impl UtsimSeguranca {
    /// Calcula a camada da replicacao `replication` de `model` (roda a replicacao de novo,
    /// com a mesma semente). Falso em erro, veja `last_error`.
    #[func]
    fn build(&mut self, model: Gd<UtsimModel>, replication: i64) -> bool {
        let bound = model.bind();
        let Some(m) = bound.model.as_ref() else {
            self.last_error = "nenhum modelo carregado".into();
            return false;
        };
        match Seguranca::new(m, replication.max(0) as u32) {
            Ok(s) => {
                self.core = Some(s);
                true
            }
            Err(e) => {
                self.last_error = e.to_string();
                self.core = None;
                false
            }
        }
    }

    #[func]
    fn last_error(&self) -> GString {
        GString::from(self.last_error.as_str())
    }

    /// Nomes de EPI que o 3D sabe vestir.
    #[func]
    fn epi_names() -> PackedStringArray {
        strings(utsim_assets::epi::NOMES)
    }

    /// Nos do operador simples em que a peca de EPI `name` se encaixa.
    #[func]
    fn epi_nodes(name: GString) -> PackedStringArray {
        strings(utsim_assets::epi::nos(&name.to_string()))
    }

    /// Os bytes `.glb` da peca de EPI `name` (vazio para um nome desconhecido). Com `bone`,
    /// a versao centrada na origem para um osso do manequim (so luvas e botina).
    #[func]
    fn epi_glb(name: GString, bone: bool) -> PackedByteArray {
        let bytes = utsim_assets::epi::glb(&name.to_string(), bone).unwrap_or_default();
        PackedByteArray::from(bytes.as_slice())
    }

    /// As zonas: `[{name, kind, level, color, ppe, points, center}]`. `points` e o
    /// poligono no piso (coordenadas do Godot) e `center` onde pôr o nome.
    #[func]
    fn zones(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = &self.core else { return out };
        for z in &c.zones {
            let mut d = VarDictionary::new();
            put(&mut d, "name", GString::from(z.name.as_str()));
            put(&mut d, "kind", GString::from(z.kind));
            put(&mut d, "level", z.level.map_or(0, i64::from));
            put(
                &mut d,
                "color",
                Color::from_rgb(z.color[0], z.color[1], z.color[2]),
            );
            put(&mut d, "ppe", strings(&z.ppe));
            let pts: Vec<Vector3> = z.polygon.iter().map(|p| ponto(*p, 0.0)).collect();
            put(&mut d, "points", PackedVector3Array::from(pts.as_slice()));
            put(&mut d, "center", ponto(z.center, 0.0));
            out.push(&d.to_variant());
        }
        out
    }

    /// Nomes das pessoas (blocos `Transporter` a pe) da replicacao.
    #[func]
    fn people(&self) -> PackedStringArray {
        self.core
            .as_ref()
            .map_or_else(PackedStringArray::new, |c| strings(&c.people()))
    }

    /// O estado de cada pessoa em `t`: `[{name, station, ppe, zone, missing}]`. `zone`
    /// vazio: em regra. Com `zone`, `missing` lista o EPI pedido que ela nao veste.
    #[func]
    fn status(&self, t: f64) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = &self.core else { return out };
        for p in c.status(t) {
            let mut d = VarDictionary::new();
            put(&mut d, "name", GString::from(p.name.as_str()));
            put(
                &mut d,
                "station",
                GString::from(p.station.as_deref().unwrap_or("")),
            );
            put(&mut d, "ppe", strings(&p.ppe));
            put(&mut d, "zone", GString::from(p.zone.as_str()));
            put(&mut d, "missing", strings(&p.missing));
            out.push(&d.to_variant());
        }
        out
    }

    /// Os quase-conflitos simulados, em ordem de tempo: `[{t, person, vehicle, distance,
    /// duration, position, text}]`.
    #[func]
    fn near_misses(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = &self.core else { return out };
        for n in c.near_misses() {
            let mut d = VarDictionary::new();
            put(&mut d, "t", n.t);
            put(&mut d, "person", GString::from(n.person.as_str()));
            put(&mut d, "vehicle", GString::from(n.vehicle.as_str()));
            put(&mut d, "distance", n.min_distance);
            put(&mut d, "duration", n.duration);
            put(&mut d, "position", ponto(n.at, 0.0));
            put(
                &mut d,
                "text",
                GString::from(
                    format!(
                        "{:.0} s  {} e {} a {:.1} m",
                        n.t, n.person, n.vehicle, n.min_distance
                    )
                    .as_str(),
                ),
            );
            out.push(&d.to_variant());
        }
        out
    }

    /// Itens que terminaram com defeito (`defeitos > 0`), pelo `item_id` dos quadros.
    #[func]
    fn defective_items(&self) -> PackedInt64Array {
        self.core.as_ref().map_or_else(PackedInt64Array::new, |c| {
            PackedInt64Array::from(c.defective.as_slice())
        })
    }

    /// Postos com bloco de inspecao.
    #[func]
    fn inspections(&self) -> PackedStringArray {
        self.core
            .as_ref()
            .map_or_else(PackedStringArray::new, |c| strings(&c.inspections))
    }

    /// O mapa de calor: `{origin, cell, nx, ny, values}` com `values` de 0 a 1 em ordem de
    /// linha (`y * nx + x`), `origin` o canto do piso em coordenadas do Godot.
    #[func]
    fn heat(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = &self.core else { return d };
        let h = &c.exposure.heat;
        put(&mut d, "origin", ponto(h.origin, 0.0));
        put(&mut d, "cell", h.cell);
        put(&mut d, "nx", h.nx as i64);
        put(&mut d, "ny", h.ny as i64);
        put(
            &mut d,
            "values",
            PackedFloat32Array::from(heat_values(&c.exposure).as_slice()),
        );
        d
    }
}
