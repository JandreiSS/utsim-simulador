//! A classe `UtsimAmbiental` da ponte: entrega ao GDScript (`camada_ambiental.gd`) os
//! números da camada Ambiental de uma replicação, calculados pelo UTSim Ambiental
//! ([`crate::ambiental`]). O GDScript só desenha.

use godot::prelude::*;
use utsim_ambiental_core::camada3d::{Camada3d, cor};

use crate::{UtsimModel, put, strings};

/// A camada Ambiental de uma replicação.
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimAmbiental {
    core: Option<Camada3d>,
    last_error: String,
    base: Base<RefCounted>,
}

fn rgb(c: [f32; 3]) -> Color {
    Color::from_rgb(c[0], c[1], c[2])
}

#[godot_api]
impl UtsimAmbiental {
    /// Calcula a camada da replicação `replication` de `model` (roda a replicação de novo,
    /// com a mesma semente). Falso em erro ou sem bloco ambiental, veja `last_error`.
    #[func]
    fn build(&mut self, model: Gd<UtsimModel>, replication: i64) -> bool {
        let bound = model.bind();
        let Some(m) = bound.model.as_ref() else {
            self.last_error = "nenhum modelo carregado".into();
            return false;
        };
        match crate::ambiental::camada(m, bound.path.as_deref(), replication.max(0) as u32) {
            Ok(c) => {
                self.core = Some(c);
                true
            }
            Err(e) => {
                self.last_error = e;
                self.core = None;
                false
            }
        }
    }

    #[func]
    fn last_error(&self) -> GString {
        GString::from(self.last_error.as_str())
    }

    /// Os nós com bloco: `[{name, index, energy_kwh, idle_kwh, co2e_kg, has_co2e, kw_max,
    /// fraction, color}]`. `fraction` é a energia sobre a do nó que mais gasta, e `color` a
    /// cor dela na escala de um só matiz.
    #[func]
    fn nodes(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = &self.core else { return out };
        for n in &c.nos {
            let mut d = VarDictionary::new();
            put(&mut d, "name", GString::from(n.nome.as_str()));
            put(&mut d, "index", n.indice as i64);
            put(&mut d, "energy_kwh", n.energia_kwh);
            put(&mut d, "idle_kwh", n.sem_valor_kwh);
            put(&mut d, "co2e_kg", n.co2e_kg.unwrap_or(0.0));
            put(&mut d, "has_co2e", n.co2e_kg.is_some());
            put(&mut d, "kw_max", n.kw_max);
            put(&mut d, "fraction", n.fracao_energia);
            put(&mut d, "color", rgb(cor(n.fracao_energia)));
            out.push(&d.to_variant());
        }
        out
    }

    /// A altura de cada coluna em `t` (0 a 1), na ordem de `nodes`.
    #[func]
    fn heights(&self, t: f64) -> PackedFloat32Array {
        let v: Vec<f32> = self.core.as_ref().map_or_else(Vec::new, |c| {
            c.alturas(t).into_iter().map(|x| x as f32).collect()
        });
        PackedFloat32Array::from(v.as_slice())
    }

    /// A potência de cada nó em `t`, em kW, na ordem de `nodes`.
    #[func]
    fn powers(&self, t: f64) -> PackedFloat64Array {
        let v: Vec<f64> = self.core.as_ref().map_or_else(Vec::new, |c| {
            c.potencias(t).into_iter().map(|x| x.0).collect()
        });
        PackedFloat64Array::from(v.as_slice())
    }

    /// O medidor da fábrica em `t`: `{in_window, kw, peak_kw, peak_t, tariff, in_peak,
    /// energy_kwh, co2e_kg, has_co2e}`. `kw` é a curva de carga do UTSim Ambiental no
    /// instante; `in_window` falso no aquecimento; `tariff` falso sem tarifa.
    #[func]
    fn meter(&self, t: f64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = &self.core else { return d };
        let kw = c.potencia(t);
        put(&mut d, "in_window", kw.is_some());
        put(&mut d, "kw", kw.unwrap_or(0.0));
        let (pt, pk) = c.pico.unwrap_or((0.0, 0.0));
        put(&mut d, "peak_kw", pk);
        put(&mut d, "peak_t", pt);
        let pico = c.em_pico(t);
        put(&mut d, "tariff", pico.is_some());
        put(&mut d, "in_peak", pico.unwrap_or(false));
        put(&mut d, "energy_kwh", c.energia_kwh);
        put(&mut d, "co2e_kg", c.co2e_kg.unwrap_or(0.0));
        put(&mut d, "has_co2e", c.co2e_kg.is_some());
        d
    }

    /// As peças refugadas na janela (pelo `item_id` dos quadros).
    #[func]
    fn scrap_items(&self) -> PackedInt64Array {
        let v: Vec<i64> = self.core.as_ref().map_or_else(Vec::new, |c| {
            c.refugo.iter().map(|p| p.item as i64).collect()
        });
        PackedInt64Array::from(v.as_slice())
    }

    /// A mochila da peça `item`: `{found, path, kwh, co2e_kg, has_co2e, missing, flows}`.
    /// `flows` é `[[fluxo, quantidade], ...]`; `missing`, os fatores que faltam.
    #[func]
    fn backpack(&self, item: i64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let found = self
            .core
            .as_ref()
            .and_then(|c| c.mochila(item.max(0) as u64).map(|p| (c, p)));
        put(&mut d, "found", found.is_some());
        let Some((c, p)) = found else { return d };
        put(&mut d, "path", strings(&c.caminho(p)));
        put(&mut d, "kwh", p.kwh);
        match &p.co2e_kg {
            Ok(k) => {
                put(&mut d, "co2e_kg", *k);
                put(&mut d, "has_co2e", true);
                put(&mut d, "missing", PackedStringArray::new());
            }
            Err(f) => {
                put(&mut d, "co2e_kg", 0.0);
                put(&mut d, "has_co2e", false);
                put(&mut d, "missing", strings(f));
            }
        }
        let mut fl = VarArray::new();
        for (f, q) in &p.fluxos {
            let mut par = VarArray::new();
            par.push(&GString::from(f.as_str()).to_variant());
            par.push(&q.to_variant());
            fl.push(&par.to_variant());
        }
        put(&mut d, "flows", fl);
        d
    }
}
