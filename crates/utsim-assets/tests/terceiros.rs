//! Third-party assets (`assets/terceiros`): every file in the catalogue exists, parses as
//! glTF, has the listed clips, nodes and real dimensions, and every pack carries its
//! licence. Only CC0 and CC-BY (with credit) are accepted.

use std::collections::{BTreeSet, HashSet};
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct Catalogo {
    versao: u32,
    ativos: Vec<Ativo>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Ativo {
    nome: String,
    arquivo: String,
    pacote: String,
    autor: String,
    licenca: String,
    uso: String,
    tamanho_m: (f32, f32, f32),
    minimo_m: (f32, f32, f32),
    centro_m: (f32, f32),
    eixo_cima: String,
    eixo_frente: String,
    frente_verificada: bool,
    giro_y_graus: f32,
    escala_real: f32,
    tamanho_real_m: (f32, f32, f32),
    clipes: Vec<String>,
    mapa_movimento: Vec<(String, String)>,
    nos: Vec<(String, String)>,
}

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/terceiros")
}

fn catalogo() -> Catalogo {
    let text = std::fs::read_to_string(dir().join("catalogo.ron")).expect("catalogo.ron");
    ron::from_str(&text).expect("catalogo.ron válido")
}

fn mul(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut r = [[0.0; 4]; 4];
    for (c, col) in r.iter_mut().enumerate() {
        for (row, cell) in col.iter_mut().enumerate() {
            *cell = (0..4).map(|k| a[k][row] * b[c][k]).sum();
        }
    }
    r
}

/// World-space box of every mesh at rest (skins ignored: bind pose), and node names.
fn medir(bytes: &[u8]) -> ([f32; 3], [f32; 3], HashSet<String>, BTreeSet<String>) {
    let gltf = gltf::Gltf::from_slice(bytes).expect("GLB válido");
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    let mut nodes = HashSet::new();
    fn walk(
        node: gltf::Node,
        parent: [[f32; 4]; 4],
        min: &mut [f32; 3],
        max: &mut [f32; 3],
        names: &mut HashSet<String>,
    ) {
        let m = mul(parent, node.transform().matrix());
        if let Some(n) = node.name() {
            names.insert(n.to_string());
        }
        if let Some(mesh) = node.mesh() {
            for p in mesh.primitives() {
                let a = p.get(&gltf::Semantic::Positions).expect("POSITION");
                let num = |v: Option<gltf::json::Value>| -> [f32; 3] {
                    let v = v.expect("min/max");
                    let arr = v.as_array().expect("array");
                    [0, 1, 2].map(|k| arr[k].as_f64().unwrap() as f32)
                };
                let (lo, hi) = (num(a.min()), num(a.max()));
                for i in 0..8 {
                    let c = [
                        if i & 1 == 0 { lo[0] } else { hi[0] },
                        if i & 2 == 0 { lo[1] } else { hi[1] },
                        if i & 4 == 0 { lo[2] } else { hi[2] },
                    ];
                    for k in 0..3 {
                        let v = m[0][k] * c[0] + m[1][k] * c[1] + m[2][k] * c[2] + m[3][k];
                        min[k] = min[k].min(v);
                        max[k] = max[k].max(v);
                    }
                }
            }
        }
        for child in node.children() {
            walk(child, m, min, max, names);
        }
    }
    let ident = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let scene = gltf.default_scene().or_else(|| gltf.scenes().next());
    for root in scene.expect("cena").nodes() {
        walk(root, ident, &mut min, &mut max, &mut nodes);
    }
    let clips = gltf
        .animations()
        .filter_map(|a| a.name().map(str::to_string))
        .collect();
    (min, max, nodes, clips)
}

#[test]
fn every_catalogued_file_loads_with_its_clips_nodes_and_size() {
    let cat = catalogo();
    assert_eq!(cat.versao, 1);
    assert!(cat.ativos.len() >= 30, "catálogo muito pequeno");
    let mut names = HashSet::new();
    for a in &cat.ativos {
        assert!(names.insert(a.nome.clone()), "nome repetido: {}", a.nome);
        let path = dir().join(&a.arquivo);
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let (min, max, nodes, clips) = medir(&bytes);
        let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
        let want = [a.tamanho_m.0, a.tamanho_m.1, a.tamanho_m.2];
        for k in 0..3 {
            assert!(
                (size[k] - want[k]).abs() <= 0.01 * want[k].max(0.1),
                "{}: tamanho {size:?} difere de {want:?}",
                a.nome
            );
        }
        let real = [a.tamanho_real_m.0, a.tamanho_real_m.1, a.tamanho_real_m.2];
        assert!(a.escala_real > 0.0, "{}: escala", a.nome);
        for k in 0..3 {
            let esperado = want[k] * a.escala_real;
            assert!(
                (real[k] - esperado).abs() <= 0.01 * esperado.max(0.1),
                "{}: tamanho_real_m incoerente com a escala",
                a.nome
            );
        }
        assert_eq!(a.eixo_cima, "+Y");
        assert!(matches!(a.eixo_frente.as_str(), "+Z" | "+X" | "-Z" | "-X"));
        let listed: BTreeSet<&str> = a.clipes.iter().map(String::as_str).collect();
        assert_eq!(
            listed,
            clips.iter().map(String::as_str).collect::<BTreeSet<_>>(),
            "{}: clipes do catálogo diferem do arquivo",
            a.nome
        );
        for (movimento, clipe) in &a.mapa_movimento {
            assert!(
                matches!(
                    movimento.as_str(),
                    "Idle"
                        | "Moving"
                        | "Carrying"
                        | "Loading"
                        | "Unloading"
                        | "Working"
                        | "Blocked"
                        | "Driving"
                        | "Pushing"
                        | "Sitting"
                ),
                "{}: movimento desconhecido {movimento}",
                a.nome
            );
            for parte in clipe.split('+') {
                assert!(
                    clips.contains(parte),
                    "{}: o clipe {parte} do mapa não existe no arquivo",
                    a.nome
                );
            }
        }
        for (no, _) in &a.nos {
            assert!(nodes.contains(no), "{}: nó {no} não existe", a.nome);
        }
        assert!(["CC0-1.0", "CC-BY-3.0"].contains(&a.licenca.as_str()));
    }
}

#[test]
fn every_pack_has_its_licence_and_every_glb_is_catalogued() {
    let cat = catalogo();
    let creditos = std::fs::read_to_string(dir().join("CREDITOS.md")).expect("CREDITOS.md");
    let catalogued: HashSet<String> = cat.ativos.iter().map(|a| a.arquivo.clone()).collect();
    let mut total = 0u64;
    for entry in std::fs::read_dir(dir()).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }
        let pack = entry.file_name().to_string_lossy().to_string();
        assert!(
            entry.path().join("LICENCA.txt").is_file(),
            "{pack}: sem LICENCA.txt"
        );
        assert!(creditos.contains(&pack), "{pack}: falta em CREDITOS.md");
        let licenca = std::fs::read_to_string(entry.path().join("LICENCA.txt")).unwrap();
        assert!(licenca.contains("https://"), "{pack}: licença sem URL");
        for f in std::fs::read_dir(entry.path()).unwrap() {
            let f = f.unwrap();
            let name = f.file_name().to_string_lossy().to_string();
            let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
            assert!(
                ext == "glb" || ext == "txt",
                "{pack}/{name}: só .glb e .txt"
            );
            total += f.metadata().unwrap().len();
            if ext == "glb" {
                assert!(
                    catalogued.contains(&format!("{pack}/{name}")),
                    "{pack}/{name}: arquivo fora do catálogo"
                );
            }
        }
    }
    assert!(
        total < 40_000_000,
        "ativos de terceiros passam de 40 MB: {total}"
    );
    // CC-BY needs the author named in the credits.
    for a in cat.ativos.iter().filter(|a| a.licenca == "CC-BY-3.0") {
        assert!(
            creditos.contains(&a.autor),
            "{}: crédito CC-BY ausente",
            a.nome
        );
    }
}
