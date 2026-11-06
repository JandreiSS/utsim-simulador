//! The committed models (`assets/modelos/*.glb`) are exactly what the generator writes,
//! and each file parses back as glTF with the documented nodes and real dimensions.

use std::collections::HashMap;
use std::path::PathBuf;

use utsim_assets::{ConveyorSpec, RackSpec, Spec};

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/modelos")
}

#[test]
fn committed_files_match_the_generator_byte_for_byte() {
    for spec in Spec::defaults() {
        let path = dir().join(format!("{}.glb", spec.key()));
        let committed = std::fs::read(&path).unwrap_or_else(|e| {
            panic!(
                "{}: {e} (gere com `cargo run -p utsim-assets`)",
                path.display()
            )
        });
        let generated = spec.glb();
        assert!(
            committed == generated,
            "{} está desatualizado: rode `cargo run -p utsim-assets` e versione o resultado",
            path.display()
        );
        // Deterministic within a run too.
        assert_eq!(generated, spec.glb());
    }
}

/// Node names, world pivots and the bounds (asset coordinates, Z up) of a GLB.
struct Parsed {
    pivots: HashMap<String, [f32; 3]>,
    min: [f32; 3],
    max: [f32; 3],
    root: String,
}

fn parse(bytes: &[u8]) -> Parsed {
    let gltf = gltf::Gltf::from_slice(bytes).expect("GLB válido");
    let scene = gltf.default_scene().expect("cena padrão");
    let roots: Vec<_> = scene.nodes().collect();
    assert_eq!(roots.len(), 1, "uma raiz");
    let mut out = Parsed {
        pivots: HashMap::new(),
        min: [f32::MAX; 3],
        max: [f32::MIN; 3],
        root: roots[0].name().unwrap_or_default().to_string(),
    };
    fn walk(node: gltf::Node, origin: [f32; 3], out: &mut Parsed) {
        let (t, r, s) = node.transform().decomposed();
        assert_eq!(r, [0.0, 0.0, 0.0, 1.0], "sem rotação em repouso");
        assert_eq!(s, [1.0, 1.0, 1.0], "sem escala");
        let here = [origin[0] + t[0], origin[1] + t[1], origin[2] + t[2]];
        let name = node.name().expect("nó com nome").to_string();
        assert!(
            out.pivots.insert(name.clone(), here).is_none(),
            "nome repetido: {name}"
        );
        if let Some(mesh) = node.mesh() {
            for p in mesh.primitives() {
                let a = p.get(&gltf::Semantic::Positions).expect("POSITION");
                let num = |v: Option<gltf::json::Value>| -> [f32; 3] {
                    let v = v.expect("min/max");
                    let arr = v.as_array().expect("array");
                    [0, 1, 2].map(|k| arr[k].as_f64().unwrap() as f32)
                };
                let (lo, hi) = (num(a.min()), num(a.max()));
                for k in 0..3 {
                    out.min[k] = out.min[k].min(here[k] + lo[k]);
                    out.max[k] = out.max[k].max(here[k] + hi[k]);
                }
                assert!(p.indices().is_some());
                assert!(p.get(&gltf::Semantic::Normals).is_some());
                assert!(p.material().index().is_some());
            }
        }
        for child in node.children() {
            walk(child, here, out);
        }
    }
    walk(roots[0].clone(), [0.0; 3], &mut out);
    // Back to asset coordinates: glTF (x, y, z) -> (x, -z, y).
    let z_up = |p: [f32; 3]| [p[0], -p[2], p[1]];
    for p in out.pivots.values_mut() {
        *p = z_up(*p);
    }
    let (a, b) = (z_up(out.min), z_up(out.max));
    out.min = [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])];
    out.max = [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])];
    out
}

fn close(a: f32, b: f32, tol: f32) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn every_model_has_its_nodes_and_real_dimensions() {
    // name, nodes, size (x, y, z) in metres.
    let table: &[(&str, &[&str], [f32; 3])] = &[
        (
            "empilhadeira",
            &[
                "chassi", "cabine", "luz", "torre", "carro", "garfos", "roda_de", "roda_dd",
                "roda_te", "roda_td",
            ],
            [3.43, 1.10, 2.25],
        ),
        (
            "paleteira",
            &[
                "chassi",
                "garfos",
                "timao",
                "roda_direcao",
                "roda_e",
                "roda_d",
            ],
            [1.74, 0.54, 1.25],
        ),
        (
            "agv",
            &["chassi", "luz", "roda_e", "roda_d"],
            [1.13, 0.81, 0.35],
        ),
        ("palete", &[], [1.2, 1.0, 0.144]),
        ("caixa", &[], [0.406, 0.306, 0.303]),
        ("porta_paletes", &["estrutura"], [4.34, 1.06, 6.08]),
        ("esteira", &["estrutura", "rolos"], [3.0, 0.8, 0.76]),
        ("tanque", &["estrutura", "casco"], [2.2, 2.12, 3.57]),
        ("maquina", &["chassi", "porta", "luz"], [1.8, 1.52, 2.2]),
        ("bancada", &["estrutura"], [1.5, 0.75, 1.5]),
        (
            "operador_simples",
            &[
                "tronco", "cabeca", "braco_e", "braco_d", "perna_e", "perna_d",
            ],
            [0.3, 0.64, 1.76],
        ),
        ("area", &["piso"], [4.0, 1.4, 0.008]),
        ("mesa", &["estrutura"], [0.70, 1.40, 1.17]),
        ("armario", &["estrutura"], [0.46, 0.90, 1.80]),
        ("bandeja", &["estrutura"], [0.35, 0.27, 0.15]),
        ("pilha_papeis", &["base", "papeis"], [0.34, 0.24, 0.25]),
        ("divisoria", &["estrutura"], [0.40, 1.20, 1.20]),
    ];
    assert_eq!(table.len(), Spec::defaults().len());
    for &(name, nodes, size) in table {
        let bytes = std::fs::read(dir().join(format!("{name}.glb"))).unwrap();
        let p = parse(&bytes);
        assert_eq!(p.root, name);
        for n in nodes {
            assert!(p.pivots.contains_key(*n), "{name}: falta o nó {n}");
        }
        assert_eq!(p.pivots.len(), nodes.len() + 1, "{name}: nós a mais");
        for (k, want) in size.iter().enumerate() {
            let got = p.max[k] - p.min[k];
            assert!(
                close(got, *want, 0.02),
                "{name}: dimensão {k} = {got}, esperado {want}"
            );
        }
        // Everything stands on the floor.
        assert!(close(p.min[2], 0.0, 0.002), "{name}: base em {}", p.min[2]);
    }
}

#[test]
fn joints_sit_where_the_engines_expect_them() {
    let p = parse(&Spec::Empilhadeira.glb());
    for (wheel, r) in [("roda_de", 0.33), ("roda_dd", 0.33), ("roda_te", 0.25)] {
        assert!(close(p.pivots[wheel][2], r, 1e-3), "{wheel} no eixo");
    }
    assert!(close(
        p.pivots["carro"][2],
        utsim_assets::dims::FORK_REST,
        1e-3
    ));
    assert!(close(p.pivots["garfos"][2], p.pivots["carro"][2], 1e-4));
    // The forks reach 2.13 m: their middle is the documented load point.
    assert!(close(p.max[0], 2.13, 1e-3));

    let p = parse(&Spec::OperadorSimples.glb());
    let d = utsim_assets::dims::SHOULDER;
    assert!(close(p.pivots["braco_e"][2], d, 1e-3));
    assert!(p.pivots["braco_e"][1] > 0.0 && p.pivots["braco_d"][1] < 0.0);
    assert!(close(p.pivots["perna_e"][2], utsim_assets::dims::HIP, 1e-3));
    assert!(close(p.max[2], 1.75, 0.02), "altura {}", p.max[2]);
}

#[test]
fn parametric_rack_and_conveyor_follow_their_parameters() {
    let spec = RackSpec {
        bays: 6,
        levels: 4,
        bay_width: 1.2,
        level_height: 1.5,
        depth: 1.0,
        beam: 0.08,
    };
    let p = parse(&Spec::PortaPaletes(spec).glb());
    assert!(close(p.max[2], 6.08, 1e-3));
    // Uprights centred on the bay lines: width plus the base plates.
    assert!(close(p.max[0] - p.min[0], 7.2 + 0.14, 1e-3));
    let p = parse(
        &Spec::Esteira(ConveyorSpec {
            length: 8.0,
            width: 1.1,
            height: 0.25,
            pitch: 0.125,
        })
        .glb(),
    );
    assert!(close(p.max[0] - p.min[0], 8.0, 1e-3));
    assert!(close(p.max[1] - p.min[1], 1.1, 1e-3));
    assert!(close(p.max[2], 0.26, 1e-3));
}
