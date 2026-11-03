//! End-to-end: `utsim check` lists the movement mode of every resource; `utsim rede gerar`
//! writes a network generated from the free path into the model.

use std::path::{Path, PathBuf};
use std::process::Command;

fn utsim(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_utsim"))
        .args(args)
        .output()
        .expect("runs utsim");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn examples() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples")
}

const MODEL: &str = r#"// planta de teste
Model(
    format: 2,
    name: "rede",
    run: (stop_time: 1000.0),
    objects: [
        (name: "src", kind: Source(interarrival: Const(1000.0), first_at_zero: true, max_arrivals: 1),
         position: (0.0, 5.0, 0.0)),
        (name: "q", kind: Queue(), transport: "frota", position: (0.0, 0.0, 0.0), access: (0.0, -3.0)),
        (name: "out", kind: Sink, position: (10.0, 0.0, 0.0), access: (0.0, -3.0)),
        (name: "frota", kind: Transporter(speed: 1.0, clearance: 0.5), position: (0.0, -3.0, 0.0)),
    ],
    connections: [("src", "q"), ("q", "out")],
    obstacles: [(name: "pilar", polygon: [(4.0, -4.0), (6.0, -4.0), (6.0, -2.0), (4.0, -2.0)])],
)
"#;

#[test]
fn check_shows_modes_and_rede_gerar_writes_the_network() {
    let (ok, stdout, stderr) = utsim(&[
        "check",
        examples()
            .join("celula_com_operadores.ron")
            .to_str()
            .unwrap(),
    ]);
    assert!(ok, "{stderr}");
    assert!(
        stdout.contains("movimentação: ") && stdout.contains("legado"),
        "{stdout}"
    );

    let dir = std::env::temp_dir().join(format!("utsim-rede-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("planta.ron");
    std::fs::write(&path, MODEL).unwrap();
    let p = path.to_str().unwrap();
    let (ok, stdout, stderr) = utsim(&["check", p]);
    assert!(ok, "{stderr}");
    assert!(
        stdout.contains("movimentação: frota: livre") && stdout.contains("folga 0,50 m"),
        "{stdout}"
    );

    let (ok, stdout, stderr) = utsim(&["rede", "gerar", p, "--usar"]);
    assert!(ok, "{stderr}");
    assert!(stdout.contains("rede gerada:"), "{stdout}");
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.starts_with("// planta de teste\n"), "{text}");
    assert!(
        text.contains("generated:") && text.contains("movement: Network"),
        "{text}"
    );
    let (ok, stdout, stderr) = utsim(&["check", p]);
    assert!(ok, "{stderr}");
    assert!(!stderr.contains("aviso"), "{stderr}");
    assert!(stdout.contains("frota: pela rede"), "{stdout}");

    // Walls of a DXF plan become obstacles of the model (and the plan is recorded).
    let wall = [
        "0",
        "SECTION",
        "2",
        "HEADER",
        "9",
        "$INSUNITS",
        "70",
        "6",
        "0",
        "ENDSEC",
        "0",
        "SECTION",
        "2",
        "ENTITIES",
        "0",
        "LINE",
        "8",
        "PAREDES",
        "10",
        "8.0",
        "20",
        "-8.0",
        "11",
        "8.0",
        "21",
        "-2.0",
        "0",
        "ENDSEC",
        "0",
        "EOF",
    ]
    .join("\n");
    let dxf = dir.join("galpao.dxf");
    std::fs::write(&dxf, wall + "\n").unwrap();
    let (ok, stdout, stderr) = utsim(&["planta", dxf.to_str().unwrap(), "--obstaculos", p]);
    assert!(ok, "{stderr}");
    assert!(
        stdout.contains("1 obstáculos da planta gravados"),
        "{stdout}"
    );
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(
        text.contains("planta_1") && text.contains("pilar") && text.contains("galpao.dxf"),
        "{text}"
    );
    // The network generated before is now stale.
    let (ok, _, stderr) = utsim(&["check", p]);
    assert!(ok && stderr.contains("gere de novo"), "{stderr}");
    let (ok, _, stderr) = utsim(&["rede", "gerar", p]);
    assert!(ok, "{stderr}");
    let (ok, _, stderr) = utsim(&["check", p]);
    assert!(ok && !stderr.contains("aviso"), "{stderr}");

    // A hand-drawn network is not replaced without --substituir.
    let hand = MODEL.replace(
        "    obstacles:",
        "    network: (nodes: [(name: \"a\", position: (0.0, -3.0, 0.0))], edges: []),\n    obstacles:",
    );
    std::fs::write(&path, hand).unwrap();
    let (ok, _, stderr) = utsim(&["rede", "gerar", p]);
    assert!(!ok && stderr.contains("--substituir"), "{stderr}");
    std::fs::remove_dir_all(&dir).unwrap();
}
