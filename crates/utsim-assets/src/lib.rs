//! Procedural, articulated 3D models of the UTSim appearance library, written as
//! binary glTF (`.glb`) by code only (no modelling tool, no download, no third-party licence).
//!
//! Every model is built at real size in metres from simple shapes (boxes, cylinders,
//! spheres) with flat PBR colours and a low triangle count. Each moving part is a
//! separate **named node with its pivot at the joint**, so an engine animates it by
//! setting the node's transform, without skeletons. The same bytes come out on every
//! platform (floats are quantized and the JSON is written in a fixed order), which lets
//! the repository keep the generated defaults in `assets/modelos/` and a test check that
//! they never drift.
//!
//! # Conventions
//! - Modelled in the UTSim frame: metres, **Z up**, `+x` forward (yaw 0), `+y` to the
//!   left, origin on the floor at the centre of the footprint.
//! - Written in glTF's Y-up frame: `(x, y, z) -> (x, z, -y)`. Assets face glTF `+X`, the
//!   UTSim heading convention (an engine that expects glTF's `+Z` forward turns them by
//!   -90 degrees about the vertical).
//! - The root node has the asset's name; the parts hang from it.
//! - Material `pintura` is the main paint of each asset (the one a `visual.color`
//!   override replaces); `luz` is a status light the engine recolours.
//!
//! # Nodes
//!
//! | Asset | Size (m, x × y × z) | Nodes (pivot) |
//! |---|---|---|
//! | `empilhadeira` | 3.43 × 1.10 × 2.25 (2.30 to the fork face) | `chassi`; `cabine` (seat, overhead guard); `luz` (beacon); `torre` (mast base); `carro` (child of `torre`; its origin height is the fork top height, 0.10 at rest); `garfos` (child of `carro`); `roda_de`, `roda_dd`, `roda_te`, `roda_td` (axle; pivot height = radius) |
//! | `paleteira` | 1.74 × 0.54 × 1.25 | `chassi`; `garfos` (top at 0.085); `timao` (joint on the pump); `roda_direcao`, `roda_e`, `roda_d` (axle) |
//! | `agv` | 1.13 × 0.81 × 0.35 | `chassi`; `luz` (status light band); `roda_e`, `roda_d` (axle) |
//! | `palete` | 1.20 × 1.00 × 0.144 | root only (static) |
//! | `caixa` | 0.40 × 0.30 × 0.30 | root only (material `papelao`) |
//! | `porta_paletes` | bays × bay width, depth, levels × level height + beam | `estrutura` |
//! | `esteira` | length × width × height | `estrutura`, `rolos` |
//! | `tanque` | 2r × 2r × (legs + height + roof) | `estrutura`, `casco` (shell base) |
//! | `maquina` | 1.80 × 1.52 × 2.20 | `chassi`; `porta`; `luz` (signal tower) |
//! | `bancada` | 1.50 × 0.75 × 1.50 | `estrutura` |
//! | `operador_simples` | 0.30 × 0.64 × 1.76 | `tronco` (hips); `cabeca` (neck, child of `tronco`); `braco_e`, `braco_d` (shoulders, children of `tronco`); `perna_e`, `perna_d` (hips) |
//! | `area` | length × width × 0.008 | `piso` |
//! | `mesa` | 0.70 × 1.40 × 1.17 (top at 0.75) | `estrutura` |
//! | `armario` | 0.46 × 0.90 × 1.80 | `estrutura` |
//! | `bandeja` | 0.35 × 0.27 × 0.15 | `estrutura` |
//! | `pilha_papeis` | 0.34 × 0.24 × 0.25 | `base`; `papeis` (pivot on the floor; scale it along the vertical to follow the cases waiting) |
//! | `divisoria` | 0.40 × 1.20 × 1.20 | `estrutura` |
//!
//! Limbs of `operador_simples` and wheels turn about the lateral axis, glTF `Z`: a
//! positive angle swings a limb forward; a wheel rolling forward turns by
//! `-distance / radius`.

pub mod builder;
pub mod glb;
pub mod loads;
pub mod office;
pub mod people;
pub mod structures;
pub mod vehicles;

pub use builder::Model;
pub use structures::{AreaSpec, ConveyorSpec, RackSpec, TankSpec};

/// Measurements the engines need to attach loads and drive the parts.
pub mod dims {
    /// Forklift: middle of the fork blades along `x` (where a load sits).
    pub const FORK_CENTER: f32 = 1.555;
    /// Forklift: fork top height at rest (the `carro` node's rest height).
    pub const FORK_REST: f32 = 0.10;
    /// Pallet jack: middle of the forks along `x` and their top height.
    pub const JACK_FORK_CENTER: f32 = 0.50;
    pub const JACK_FORK_TOP: f32 = 0.085;
    /// AGV: height of the load deck.
    pub const AGV_DECK: f32 = 0.35;
    /// Operator: shoulder and hip joint heights, arm and leg lengths.
    pub use crate::people::{ARM, HIP, LEG, SHOULDER};
}

/// One asset of the library, with its parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Spec {
    Empilhadeira,
    Paleteira,
    Agv,
    Palete,
    Caixa,
    PortaPaletes(RackSpec),
    Esteira(ConveyorSpec),
    Tanque(TankSpec),
    Maquina,
    Bancada,
    OperadorSimples,
    Area(AreaSpec),
    Mesa,
    Armario,
    Bandeja,
    PilhaPapeis,
    Divisoria,
}

impl Spec {
    /// Library name (also the default file stem).
    pub fn name(&self) -> &'static str {
        match self {
            Spec::Empilhadeira => "empilhadeira",
            Spec::Paleteira => "paleteira",
            Spec::Agv => "agv",
            Spec::Palete => "palete",
            Spec::Caixa => "caixa",
            Spec::PortaPaletes(_) => "porta_paletes",
            Spec::Esteira(_) => "esteira",
            Spec::Tanque(_) => "tanque",
            Spec::Maquina => "maquina",
            Spec::Bancada => "bancada",
            Spec::OperadorSimples => "operador_simples",
            Spec::Area(_) => "area",
            Spec::Mesa => "mesa",
            Spec::Armario => "armario",
            Spec::Bandeja => "bandeja",
            Spec::PilhaPapeis => "pilha_papeis",
            Spec::Divisoria => "divisoria",
        }
    }

    /// Every asset with its default parameters (the files of `assets/modelos/`).
    pub fn defaults() -> Vec<Spec> {
        vec![
            Spec::Empilhadeira,
            Spec::Paleteira,
            Spec::Agv,
            Spec::Palete,
            Spec::Caixa,
            Spec::PortaPaletes(RackSpec::default()),
            Spec::Esteira(ConveyorSpec::default()),
            Spec::Tanque(TankSpec::default()),
            Spec::Maquina,
            Spec::Bancada,
            Spec::OperadorSimples,
            Spec::Area(AreaSpec::default()),
            Spec::Mesa,
            Spec::Armario,
            Spec::Bandeja,
            Spec::PilhaPapeis,
            Spec::Divisoria,
        ]
    }

    /// The asset called `name` with its default parameters. Accepts the names of the
    /// table and a few aliases ("operador", "pessoa", "forklift", "rack", "conveyor").
    pub fn by_name(name: &str) -> Option<Spec> {
        let n = name.trim().to_lowercase();
        let alias = match n.as_str() {
            "operador" | "pessoa" | "person" => "operador_simples",
            "forklift" => "empilhadeira",
            "rack" => "porta_paletes",
            "conveyor" => "esteira",
            "tank" => "tanque",
            "machine" => "maquina",
            "desk" | "mesa_computador" => "mesa",
            "armario_pastas" => "armario",
            "bandeja_entrada" => "bandeja",
            "papeis" | "pilha_casos" => "pilha_papeis",
            other => other,
        };
        Spec::defaults().into_iter().find(|s| s.name() == alias)
    }

    /// Unique key of the asset with these parameters (the default ones are just the name).
    pub fn key(&self) -> String {
        match self {
            Spec::PortaPaletes(r) if *r != RackSpec::default() => format!(
                "porta_paletes_{}x{}_{}x{}_{}_{}",
                r.bays, r.levels, r.bay_width, r.level_height, r.depth, r.beam
            ),
            Spec::Esteira(c) if *c != ConveyorSpec::default() => {
                format!("esteira_{}_{}_{}_{}", c.length, c.width, c.height, c.pitch)
            }
            Spec::Tanque(t) if *t != TankSpec::default() => {
                format!("tanque_{}_{}_{}", t.radius, t.height, t.legs)
            }
            Spec::Area(a) if *a != AreaSpec::default() => {
                format!("area_{}_{}", a.length, a.width)
            }
            other => other.name().to_string(),
        }
    }

    /// The model (scene graph and meshes).
    pub fn model(&self) -> Model {
        match self {
            Spec::Empilhadeira => vehicles::empilhadeira(),
            Spec::Paleteira => vehicles::paleteira(),
            Spec::Agv => vehicles::agv(),
            Spec::Palete => loads::palete(),
            Spec::Caixa => loads::caixa(),
            Spec::PortaPaletes(r) => structures::porta_paletes(r),
            Spec::Esteira(c) => structures::esteira(c),
            Spec::Tanque(t) => structures::tanque(t),
            Spec::Maquina => structures::maquina(),
            Spec::Bancada => structures::bancada(),
            Spec::OperadorSimples => people::operador_simples(),
            Spec::Area(a) => structures::area(a),
            Spec::Mesa => office::mesa(),
            Spec::Armario => office::armario(),
            Spec::Bandeja => office::bandeja(),
            Spec::PilhaPapeis => office::pilha_papeis(),
            Spec::Divisoria => office::divisoria(),
        }
    }

    /// The `.glb` file.
    pub fn glb(&self) -> Vec<u8> {
        glb::write(&self.model())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_aliases_resolve() {
        for s in Spec::defaults() {
            assert_eq!(Spec::by_name(s.name()), Some(s));
            assert_eq!(s.key(), s.name());
        }
        assert_eq!(Spec::by_name("Operador"), Some(Spec::OperadorSimples));
        assert_eq!(Spec::by_name("nave"), None);
        let rack = Spec::PortaPaletes(RackSpec {
            bays: 6,
            ..RackSpec::default()
        });
        assert_eq!(rack.key(), "porta_paletes_6x4_1.4x1.5_1_0.08");
    }

    #[test]
    fn every_model_is_low_poly() {
        for s in Spec::defaults() {
            let t = s.model().triangles();
            assert!((12..=8000).contains(&t), "{}: {t} triângulos", s.name());
        }
    }

    #[test]
    fn a_long_conveyor_caps_its_rollers() {
        let m = Spec::Esteira(ConveyorSpec {
            length: 400.0,
            ..ConveyorSpec::default()
        })
        .model();
        let rolos = m.find("rolos").unwrap();
        let tris: usize = m.nodes[rolos]
            .parts
            .iter()
            .map(|p| p.geometry.indices.len() / 3)
            .sum();
        // 8 segments: 16 side and 16 cap triangles per roller.
        assert!(tris <= 32 * 400, "{tris}");
    }
}
