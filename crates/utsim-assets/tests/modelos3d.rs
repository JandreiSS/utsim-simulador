//! Os elementos 3D gerados no Blender (`assets/modelos3d`, scripts em `tools/blender`):
//! cada `.glb` bate com o `catalogo.json`, tem os pontos e os clipes que a interface usa,
//! os clipes duram 1 s e movem o que devem (a garra do robô chega à peça, a porta abre,
//! o garfo sobe), e os arquivos são leves. As pessoas (`pessoa_homem`, `pessoa_mulher`) têm
//! o esqueleto do boneco UAL1, o corpo com os materiais que o 3D recolore e um nó por EPI.
//!
//! O teste não roda o Blender: ele confere os arquivos versionados. Para gerar de novo,
//! `bash scripts/blender.sh`.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

use serde_json::Value;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/modelos3d")
}

fn catalogo() -> BTreeMap<String, Value> {
    let texto = std::fs::read_to_string(dir().join("catalogo.json")).expect("catalogo.json");
    serde_json::from_str(&texto).expect("catálogo em JSON")
}

type M4 = [[f32; 4]; 4];

fn mul(a: &M4, b: &M4) -> M4 {
    // Colunas: m[c][r].
    let mut o = [[0.0; 4]; 4];
    for c in 0..4 {
        for r in 0..4 {
            o[c][r] = (0..4).map(|k| a[k][r] * b[c][k]).sum();
        }
    }
    o
}

fn trs(t: [f32; 3], q: [f32; 4], s: [f32; 3]) -> M4 {
    let [x, y, z, w] = q;
    let r = [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y + z * w),
            2.0 * (x * z - y * w),
        ],
        [
            2.0 * (x * y - z * w),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z + x * w),
        ],
        [
            2.0 * (x * z + y * w),
            2.0 * (y * z - x * w),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ];
    [
        [r[0][0] * s[0], r[0][1] * s[0], r[0][2] * s[0], 0.0],
        [r[1][0] * s[1], r[1][1] * s[1], r[1][2] * s[1], 0.0],
        [r[2][0] * s[2], r[2][1] * s[2], r[2][2] * s[2], 0.0],
        [t[0], t[1], t[2], 1.0],
    ]
}

/// glTF (Y para cima) para o referencial do UTSim (Z para cima).
fn utsim(p: [f32; 3]) -> [f32; 3] {
    [p[0], -p[2], p[1]]
}

struct Glb {
    doc: gltf::Document,
    blob: Vec<u8>,
}

impl Glb {
    fn ler(arquivo: &str) -> Glb {
        let bytes = std::fs::read(dir().join(arquivo)).unwrap_or_else(|e| panic!("{arquivo}: {e}"));
        let g = gltf::Gltf::from_slice(&bytes).expect("GLB válido");
        Glb {
            blob: g.blob.clone().expect("bloco binário"),
            doc: g.document,
        }
    }

    fn raiz(&self) -> String {
        let roots: Vec<_> = self.doc.default_scene().expect("cena").nodes().collect();
        assert_eq!(roots.len(), 1, "uma raiz");
        roots[0].name().unwrap_or_default().to_string()
    }

    fn nomes(&self) -> BTreeSet<String> {
        self.doc
            .nodes()
            .map(|n| n.name().expect("nó com nome").to_string())
            .collect()
    }

    /// Posição (UTSim) de cada nó com o clipe `clipe` no fim (`fim`) ou no começo.
    fn pose(&self, clipe: Option<&str>, fim: bool) -> HashMap<String, [f32; 3]> {
        let mut locais: Vec<([f32; 3], [f32; 4], [f32; 3])> = self
            .doc
            .nodes()
            .map(|n| n.transform().decomposed())
            .collect();
        if let Some(nome) = clipe {
            let anim = self
                .doc
                .animations()
                .find(|a| a.name() == Some(nome))
                .unwrap_or_else(|| panic!("clipe {nome}"));
            for ch in anim.channels() {
                let r = ch.reader(|_| Some(&self.blob));
                let k = ch.target().node().index();
                use gltf::animation::util::ReadOutputs as O;
                match r.read_outputs().expect("saídas") {
                    O::Translations(it) => {
                        let v: Vec<_> = it.collect();
                        locais[k].0 = if fim { *v.last().unwrap() } else { v[0] };
                    }
                    O::Rotations(it) => {
                        let v: Vec<_> = it.into_f32().collect();
                        locais[k].1 = if fim { *v.last().unwrap() } else { v[0] };
                    }
                    O::Scales(it) => {
                        let v: Vec<_> = it.collect();
                        locais[k].2 = if fim { *v.last().unwrap() } else { v[0] };
                    }
                    O::MorphTargetWeights(_) => {}
                }
            }
        }
        let mut out = HashMap::new();
        fn andar(
            n: gltf::Node,
            pai: &M4,
            locais: &[([f32; 3], [f32; 4], [f32; 3])],
            out: &mut HashMap<String, [f32; 3]>,
        ) {
            let (t, q, s) = locais[n.index()];
            let m = mul(pai, &trs(t, q, s));
            out.insert(
                n.name().unwrap().to_string(),
                utsim([m[3][0], m[3][1], m[3][2]]),
            );
            for c in n.children() {
                andar(c, &m, locais, out);
            }
        }
        let id = trs([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0; 3]);
        for n in self.doc.default_scene().unwrap().nodes() {
            andar(n, &id, &locais, &mut out);
        }
        out
    }
}

fn perto(a: [f32; 3], b: [f32; 3], tol: f32) -> bool {
    (0..3).all(|k| (a[k] - b[k]).abs() <= tol)
}

fn v3(v: &Value) -> [f32; 3] {
    let a = v.as_array().expect("lista");
    [0, 1, 2].map(|k| a[k].as_f64().unwrap() as f32)
}

#[test]
fn o_catalogo_e_os_arquivos_batem() {
    let cat = catalogo();
    let arquivos: BTreeSet<String> = std::fs::read_dir(dir())
        .unwrap()
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter(|n| n.ends_with(".glb"))
        .collect();
    let listados: BTreeSet<String> = cat
        .values()
        .map(|v| v["arquivo"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        arquivos, listados,
        "arquivos .glb fora do catálogo (ou faltando)"
    );
    for (chave, item) in cat.iter().filter(|(k, _)| !k.starts_with("pessoa_")) {
        let g = Glb::ler(item["arquivo"].as_str().unwrap());
        let elemento = chave.trim_end_matches("_medio");
        assert_eq!(g.raiz(), elemento);
        let nomes = g.nomes();
        let mut esperados: BTreeSet<String> = item["nos"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        let pontos = item["pontos"].as_object().unwrap();
        esperados.extend(pontos.keys().cloned());
        assert_eq!(nomes, esperados, "{chave}: nós");
        // Pontos no lugar do catálogo, em repouso.
        let repouso = g.pose(None, false);
        for (nome, p) in pontos {
            let got = repouso[nome];
            assert!(
                perto(got, v3(&p["pos"]), 2e-3),
                "{chave}: {nome} em {got:?}"
            );
        }
        // Clipes: os do catálogo, 1 s cada, mexendo nos nós listados.
        let clipes = item["clipes"].as_object().unwrap();
        let nomes_clipes: BTreeSet<_> = g
            .doc
            .animations()
            .map(|a| a.name().unwrap().to_string())
            .collect();
        assert_eq!(
            nomes_clipes,
            clipes.keys().cloned().collect(),
            "{chave}: clipes"
        );
        for a in g.doc.animations() {
            let nome = a.name().unwrap();
            let mut alvos = BTreeSet::new();
            for ch in a.channels() {
                alvos.insert(ch.target().node().name().unwrap().to_string());
                let entrada = ch.sampler().input();
                let max = entrada.max().unwrap().as_array().unwrap()[0]
                    .as_f64()
                    .unwrap();
                let min = entrada.min().unwrap().as_array().unwrap()[0]
                    .as_f64()
                    .unwrap();
                assert!(
                    min >= -1e-4 && (max - 1.0).abs() < 1e-3,
                    "{chave}/{nome}: dura {min}..{max} s"
                );
            }
            let listados: BTreeSet<String> = clipes[nome]["nos"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect();
            assert_eq!(alvos, listados, "{chave}/{nome}: nós animados");
        }
        // No piso e com o tamanho do catálogo.
        assert!(
            item["minimo"][2].as_f64().unwrap().abs() < 2e-3,
            "{chave}: fora do piso"
        );
    }
}

#[test]
fn os_arquivos_sao_leves() {
    let cat = catalogo();
    let mut total = 0u64;
    for (chave, item) in &cat {
        let tam = std::fs::metadata(dir().join(item["arquivo"].as_str().unwrap()))
            .unwrap()
            .len();
        total += tam;
        let tri = item["triangulos"].as_u64().unwrap();
        let (lim_kb, lim_tri) = if chave.starts_with("pessoa_") {
            (400, 7000)
        } else if chave.ends_with("_medio") {
            (160, 4000)
        } else {
            (64, 1500)
        };
        assert!(tam <= lim_kb * 1024, "{chave}: {tam} bytes");
        assert!(tri <= lim_tri, "{chave}: {tri} triângulos");
    }
    assert!(
        total <= 5 * 512 * 1024,
        "assets/modelos3d tem {total} bytes"
    );
}

/// O que a interface e a ponte esperam de cada elemento.
/// (elemento, nós, pontos, clipes).
type Contrato = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
);

const CONTRATO: &[Contrato] = &[
    // (elemento, nós, pontos, clipes)
    (
        "cnc",
        &["chassi", "porta", "luz", "fuso", "torre", "tampa"],
        &[
            "ponto_operador",
            "ponto_porta",
            "ponto_peca",
            "ponto_entrada",
            "ponto_saida",
        ],
        &["ciclo", "porta_abrir", "setup", "quebra"],
    ),
    (
        "prensa",
        &[
            "chassi", "porta", "luz", "martelo", "volante", "matriz", "tampa",
        ],
        &[
            "ponto_operador",
            "ponto_porta",
            "ponto_peca",
            "ponto_entrada",
            "ponto_saida",
        ],
        &["ciclo", "porta_abrir", "setup", "quebra"],
    ),
    (
        "injetora",
        &["chassi", "porta", "luz", "molde", "unidade", "tampa"],
        &[
            "ponto_operador",
            "ponto_porta",
            "ponto_peca",
            "ponto_entrada",
            "ponto_saida",
        ],
        &["ciclo", "porta_abrir", "setup", "quebra"],
    ),
    (
        "posto",
        &["estrutura", "grampo", "luz"],
        &[
            "ponto_operador",
            "ponto_porta",
            "ponto_peca",
            "ponto_entrada",
            "ponto_saida",
        ],
        &["ciclo"],
    ),
    (
        "robo",
        &[
            "pedestal",
            "base",
            "braco",
            "antebraco",
            "garra",
            "dedo_e",
            "dedo_d",
        ],
        &[
            "ponto_garra",
            "ponto_entrada",
            "ponto_saida",
            "ponto_operador",
        ],
        &["pegar", "levar", "soltar", "ciclo"],
    ),
    (
        "celula",
        &["cerca", "portao", "luz"],
        &["ponto_porta", "ponto_operador"],
        &["porta_abrir"],
    ),
    (
        "esteira",
        &["estrutura", "rolo_00"],
        &["ponto_entrada", "ponto_saida"],
        &["rolar"],
    ),
    (
        "empilhadeira",
        &[
            "chassi", "cabine", "luz", "torre", "carro", "garfos", "roda_de", "roda_dd", "roda_te",
            "roda_td",
        ],
        &["ponto_carga", "ponto_operador"],
        &["garfo_subir"],
    ),
    (
        "agv",
        &["chassi", "mesa", "luz", "roda_e", "roda_d"],
        &["ponto_carga"],
        &["mesa_subir"],
    ),
    (
        "porta_paletes",
        &["estrutura"],
        &["ponto_vao_0_0", "ponto_vao_2_4"],
        &[],
    ),
    (
        "estante",
        &["estrutura"],
        &["ponto_vao_0_0", "ponto_vao_0_3"],
        &[],
    ),
    ("peca", &["corpo"], &[], &[]),
    ("palete", &["corpo"], &[], &[]),
    ("caixa", &["corpo"], &[], &[]),
];

#[test]
fn cada_elemento_tem_os_nos_pontos_e_clipes_do_contrato() {
    let cat = catalogo();
    let elementos: BTreeSet<&str> = cat
        .keys()
        .filter(|k| !k.starts_with("pessoa_"))
        .map(|k| k.trim_end_matches("_medio"))
        .collect();
    let contrato: BTreeSet<&str> = CONTRATO.iter().map(|c| c.0).collect();
    assert_eq!(elementos, contrato);
    for &(nome, nos, pontos, clipes) in CONTRATO {
        for chave in [nome.to_string(), format!("{nome}_medio")] {
            let item = &cat[&chave];
            for n in nos {
                assert!(
                    item["nos"].as_array().unwrap().iter().any(|v| v == n),
                    "{chave}: falta o nó {n}"
                );
            }
            for p in pontos {
                assert!(
                    item["pontos"].get(*p).is_some(),
                    "{chave}: falta o ponto {p}"
                );
            }
            let tem: BTreeSet<&str> = item["clipes"]
                .as_object()
                .unwrap()
                .keys()
                .map(|s| s.as_str())
                .collect();
            assert_eq!(tem, clipes.iter().copied().collect(), "{chave}: clipes");
        }
    }
}

#[test]
fn o_operador_fica_fora_da_maquina_e_olha_para_ela() {
    let cat = catalogo();
    for nome in ["cnc", "prensa", "injetora", "posto"] {
        let item = &cat[nome];
        let min = v3(&item["minimo"]);
        let op = &item["pontos"]["ponto_operador"];
        let pos = v3(&op["pos"]);
        assert!(
            pos[1] < min[1],
            "{nome}: operador dentro da máquina ({pos:?})"
        );
        assert_eq!(
            op["giro"].as_f64().unwrap(),
            90.0,
            "{nome}: operador olha para +y"
        );
        // A porta fica entre o operador e a máquina.
        let porta = v3(&item["pontos"]["ponto_porta"]["pos"]);
        assert!(
            porta[1] >= pos[1] && (porta[0] - pos[0]).abs() < 0.3,
            "{nome}: porta"
        );
    }
    // Na célula, o operador entra: a porta fica fora da cerca e o posto dentro.
    let c = &cat["celula"];
    let min = v3(&c["minimo"]);
    assert!(v3(&c["pontos"]["ponto_porta"]["pos"])[1] < min[1]);
    assert!(v3(&c["pontos"]["ponto_operador"]["pos"])[1] > min[1]);
}

#[test]
fn os_clipes_movem_o_que_devem() {
    for sufixo in ["", "_medio"] {
        // Robô: a garra chega à peça no fim de `pegar` e à saída no fim de `levar`; o
        // `soltar` termina no repouso.
        let g = Glb::ler(&format!("robo{sufixo}.glb"));
        let rep = g.pose(None, false);
        let pegou = g.pose(Some("pegar"), true);
        let levou = g.pose(Some("levar"), true);
        let soltou = g.pose(Some("soltar"), true);
        let alvo_e = rep["ponto_entrada"];
        let alvo_s = rep["ponto_saida"];
        let garra = |p: &HashMap<String, [f32; 3]>| p["ponto_garra"];
        // O ponto da garra fica na ponta dos dedos, onde vai a base da peça.
        assert!(
            perto(garra(&pegou), alvo_e, 0.01),
            "pegar: {:?}",
            garra(&pegou)
        );
        assert!(
            perto(garra(&levou), alvo_s, 0.01),
            "levar: {:?}",
            garra(&levou)
        );
        assert!(
            perto(garra(&soltou), garra(&rep), 1e-3),
            "soltar volta ao repouso"
        );
        // Dedos fechados ao pegar.
        let abertura = |p: &HashMap<String, [f32; 3]>| p["dedo_e"][1] - p["dedo_d"][1];
        assert!(abertura(&pegou) < abertura(&rep) - 0.05);

        // Porta das máquinas: fechada em 0, deslocada em 1.
        for (m, dist) in [("cnc", 0.8), ("prensa", 0.55), ("injetora", 1.3)] {
            let g = Glb::ler(&format!("{m}{sufixo}.glb"));
            let a = g.pose(Some("porta_abrir"), false)["porta"];
            let b = g.pose(Some("porta_abrir"), true)["porta"];
            let d = ((0..3).map(|k| (b[k] - a[k]).powi(2)).sum::<f32>()).sqrt();
            assert!((d - dist).abs() < 1e-3, "{m}: porta anda {d}");
            assert!(
                perto(a, g.pose(None, false)["porta"], 1e-4),
                "{m}: porta fechada em repouso"
            );
        }

        // Empilhadeira: o ponto de carga sobe o curso do catálogo; AGV idem.
        let cat = catalogo();
        let g = Glb::ler(&format!("empilhadeira{sufixo}.glb"));
        let curso = cat["empilhadeira"]["dados"]["garfo_curso"]
            .as_f64()
            .unwrap() as f32;
        let z0 = g.pose(None, false)["ponto_carga"][2];
        let z1 = g.pose(Some("garfo_subir"), true)["ponto_carga"][2];
        assert!((z1 - z0 - curso).abs() < 1e-3);
        let g = Glb::ler(&format!("agv{sufixo}.glb"));
        let curso = cat["agv"]["dados"]["mesa_curso"].as_f64().unwrap() as f32;
        let z0 = g.pose(None, false)["ponto_carga"][2];
        let z1 = g.pose(Some("mesa_subir"), true)["ponto_carga"][2];
        assert!((z1 - z0 - curso).abs() < 1e-3);

        // Clipes em laço terminam como começam (sem salto na volta).
        for (m, c) in [
            ("cnc", "ciclo"),
            ("prensa", "ciclo"),
            ("injetora", "ciclo"),
            ("robo", "ciclo"),
            ("esteira", "rolar"),
            ("cnc", "setup"),
            ("prensa", "setup"),
            ("injetora", "setup"),
        ] {
            let g = Glb::ler(&format!("{m}{sufixo}.glb"));
            let a = g.pose(Some(c), false);
            let b = g.pose(Some(c), true);
            for (n, p) in &a {
                assert!(perto(*p, b[n], 2e-3), "{m}/{c}: {n} salta na volta");
            }
        }
    }
}

/// Os EPIs que o modelo conhece (`utsim_model::quality::PPE_NAMES`): um nó `epi_<nome>` cada.
const EPIS: &[&str] = &[
    "capacete",
    "oculos",
    "protetor_auricular",
    "luvas",
    "botina",
    "colete",
    "mascara",
    "avental",
];

#[test]
fn as_pessoas_tem_corpo_epis_e_o_esqueleto_do_ual1() {
    let cat = catalogo();
    let ual = {
        let p = dir().join("../terceiros/quaternius_ual/UAL1_Standard.glb");
        let g = gltf::Gltf::open(p).expect("UAL1");
        let skin = g.document.skins().next().expect("esqueleto do UAL1");
        skin.joints()
            .map(|j| (j.name().unwrap().to_string(), j.transform().decomposed()))
            .collect::<BTreeMap<_, _>>()
    };
    for sexo in ["homem", "mulher"] {
        let chave = format!("pessoa_{sexo}");
        let item = &cat[&chave];
        let g = Glb::ler(item["arquivo"].as_str().unwrap());
        assert_eq!(g.raiz(), chave);
        // Malhas: o corpo e um nó por EPI, todas presas ao mesmo esqueleto.
        let malhas: BTreeSet<String> = g
            .doc
            .nodes()
            .filter(|n| n.mesh().is_some())
            .map(|n| {
                assert!(n.skin().is_some(), "{chave}: {:?} sem esqueleto", n.name());
                n.name().unwrap().to_string()
            })
            .collect();
        let mut esperadas: BTreeSet<String> = EPIS.iter().map(|e| format!("epi_{e}")).collect();
        esperadas.insert("corpo".into());
        assert_eq!(malhas, esperadas, "{chave}: malhas");
        let nos: BTreeSet<String> = item["nos"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert_eq!(nos, esperadas, "{chave}: nós do catálogo");
        let lista: Vec<&str> = item["dados"]["epis"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(
            lista.iter().copied().collect::<BTreeSet<_>>(),
            EPIS.iter().copied().collect()
        );
        // Mesmos ossos e mesma pose de repouso do UAL1: as animações dele servem.
        let skin = g.doc.skins().next().expect("esqueleto");
        let ossos: BTreeMap<String, _> = skin
            .joints()
            .map(|j| (j.name().unwrap().to_string(), j.transform().decomposed()))
            .collect();
        assert_eq!(
            ossos.keys().collect::<Vec<_>>(),
            ual.keys().collect::<Vec<_>>(),
            "{chave}: ossos"
        );
        for (nome, (t, q, _)) in &ossos {
            let (t0, q0, _) = ual[nome];
            assert!(
                (0..3).all(|k| (t[k] - t0[k]).abs() < 1e-4)
                    && (0..4).all(|k| (q[k] - q0[k]).abs() < 1e-4),
                "{chave}: osso {nome} fora da pose do UAL1"
            );
        }
        assert_eq!(g.doc.animations().count(), 0, "{chave}: sem animações próprias");
        // Todo vértice pesado (soma 1) e o corpo com os materiais que o 3D recolore.
        for malha in g.doc.meshes() {
            for prim in malha.primitives() {
                let r = prim.reader(|_| Some(&g.blob));
                let pesos = r.read_weights(0).expect("pesos").into_f32();
                for w in pesos {
                    let s: f32 = w.iter().sum();
                    assert!((s - 1.0).abs() < 0.01, "{chave}/{:?}: peso {s}", malha.name());
                }
            }
        }
        let corpo = g.doc.meshes().find(|m| m.name() == Some("corpo")).unwrap();
        let mats: BTreeSet<&str> = corpo
            .primitives()
            .filter_map(|p| p.material().name())
            .collect();
        for m in item["dados"]["recolorir"].as_array().unwrap() {
            assert!(mats.contains(m.as_str().unwrap()), "{chave}: falta o material {m}");
        }
        // Em pé no piso, com a altura do esqueleto.
        let alt = item["tamanho"][2].as_f64().unwrap();
        let base = item["dados"]["altura_base"].as_f64().unwrap();
        assert!((alt - base).abs() < 0.03, "{chave}: altura {alt}");
        assert!(item["minimo"][2].as_f64().unwrap().abs() < 0.01, "{chave}: fora do piso");
    }
}
