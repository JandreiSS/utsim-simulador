//! Administrative work places (the Lean Office line, `docs/plano_linhas_especializadas.md`,
//! step 11): desk with a computer, filing cabinet, inbox tray, paper stack and partition.
//! All static, built from boxes and cylinders like the other structures.

use crate::builder::Model;

/// Desk with a monitor and a keyboard. The person sits on the `-x` side and faces `+x`;
/// the monitor is on the far edge. Node: `estrutura` (static).
pub fn mesa() -> Model {
    let mut m = Model::new("mesa");
    let top = m.material("tampo", 0xc9b28a, 0.0, 0.6);
    let frame = m.material("pintura", 0x5a6270, 0.4, 0.5);
    let screen = m.material("tela", 0x1e2530, 0.1, 0.3);
    let key = m.material("teclado", 0x2b2f36, 0.0, 0.8);
    let s = m.node("estrutura", 0, [0.0, 0.0, 0.0]);
    m.cuboid(s, top, [-0.35, -0.70, 0.72], [0.35, 0.70, 0.75]);
    for y in [-0.67f32, 0.67] {
        // Side panels instead of four legs, like most office desks.
        m.cuboid(s, frame, [-0.32, y - 0.02, 0.0], [0.32, y + 0.02, 0.72]);
    }
    m.cuboid(s, frame, [0.28, -0.65, 0.30], [0.30, 0.65, 0.70]);
    // Monitor on a stand, facing the person.
    m.cuboid(s, frame, [0.18, -0.10, 0.75], [0.26, 0.10, 0.76]);
    m.cuboid(s, frame, [0.21, -0.02, 0.76], [0.23, 0.02, 0.86]);
    m.cuboid(s, screen, [0.19, -0.28, 0.85], [0.23, 0.28, 1.17]);
    m.cuboid(s, key, [-0.20, -0.22, 0.75], [-0.05, 0.22, 0.77]);
    m
}

/// Filing cabinet with two doors and shelves of folders behind them. Front at `+x`.
/// Node: `estrutura` (static).
pub fn armario() -> Model {
    let mut m = Model::new("armario");
    let body = m.material("pintura", 0x8e98a3, 0.3, 0.5);
    let door = m.material("porta", 0xa7b0ba, 0.3, 0.5);
    let handle = m.material("puxador", 0x2b2f36, 0.6, 0.4);
    let s = m.node("estrutura", 0, [0.0, 0.0, 0.0]);
    m.cuboid(s, body, [-0.225, -0.45, 0.0], [0.205, 0.45, 1.80]);
    for (y0, y1) in [(-0.445f32, -0.005f32), (0.005, 0.445)] {
        m.cuboid(s, door, [0.205, y0, 0.05], [0.225, y1, 1.78]);
    }
    for y in [-0.04f32, 0.04] {
        m.cuboid(s, handle, [0.225, y - 0.01, 0.90], [0.235, y + 0.01, 1.10]);
    }
    m
}

/// Inbox tray: two stacked letter trays with a few sheets in the lower one. Node:
/// `estrutura` (static).
pub fn bandeja() -> Model {
    let mut m = Model::new("bandeja");
    let tray = m.material("pintura", 0x3b6ea5, 0.1, 0.6);
    let paper = m.material("papel", 0xf4f1e8, 0.0, 0.9);
    let s = m.node("estrutura", 0, [0.0, 0.0, 0.0]);
    for z in [0.0f32, 0.09] {
        m.cuboid(s, tray, [-0.175, -0.135, z], [0.175, 0.135, z + 0.01]);
        m.cuboid(s, tray, [-0.175, -0.135, z], [0.175, -0.125, z + 0.06]);
        m.cuboid(s, tray, [-0.175, 0.125, z], [0.175, 0.135, z + 0.06]);
        m.cuboid(s, tray, [-0.175, -0.135, z], [-0.165, 0.135, z + 0.06]);
    }
    // Posts holding the upper tray.
    for (x, y) in [
        (-0.16f32, -0.12f32),
        (-0.16, 0.12),
        (0.16, -0.12),
        (0.16, 0.12),
    ] {
        m.cuboid(
            s,
            tray,
            [x - 0.008, y - 0.008, 0.01],
            [x + 0.008, y + 0.008, 0.09],
        );
    }
    m.cuboid(s, paper, [-0.15, -0.105, 0.01], [0.15, 0.105, 0.04]);
    m.cuboid(s, paper, [-0.15, -0.105, 0.10], [0.15, 0.105, 0.115]);
    m
}

/// Stack of paper folders: the stock of cases waiting in an inbox. The `papeis` node has
/// its pivot on the floor, so an engine can scale it along the vertical to follow the
/// number of cases (0.25 m at rest). Nodes: `base` (static), `papeis`.
pub fn pilha_papeis() -> Model {
    let mut m = Model::new("pilha_papeis");
    let base = m.material("base", 0x5a6270, 0.2, 0.6);
    let folder = m.material("pintura", 0xd9a441, 0.0, 0.8);
    let paper = m.material("papel", 0xf4f1e8, 0.0, 0.9);
    let b = m.node("base", 0, [0.0, 0.0, 0.0]);
    m.cuboid(b, base, [-0.17, -0.12, 0.0], [0.17, 0.12, 0.01]);
    let p = m.node("papeis", 0, [0.0, 0.0, 0.01]);
    // Folders and sheets in alternate layers, slightly offset like a real pile.
    for k in 0..6 {
        let z = 0.01 + k as f32 * 0.04;
        let dx = if k % 2 == 0 { 0.0 } else { 0.01 };
        let mat = if k % 2 == 0 { folder } else { paper };
        m.cuboid(
            p,
            mat,
            [-0.155 + dx, -0.11, z],
            [0.155 + dx, 0.11, z + 0.04],
        );
    }
    m
}

/// Partition panel between desks, on two feet. Node: `estrutura` (static).
pub fn divisoria() -> Model {
    let mut m = Model::new("divisoria");
    let panel = m.material("pintura", 0x7f8f84, 0.0, 0.9);
    let frame = m.material("moldura", 0x4a4f57, 0.5, 0.4);
    let s = m.node("estrutura", 0, [0.0, 0.0, 0.0]);
    m.cuboid(s, panel, [-0.025, -0.58, 0.05], [0.025, 0.58, 1.18]);
    m.cuboid(s, frame, [-0.03, -0.60, 1.18], [0.03, 0.60, 1.20]);
    for y in [-0.60f32, 0.56] {
        m.cuboid(s, frame, [-0.03, y, 0.03], [0.03, y + 0.04, 1.20]);
    }
    for y in [-0.50f32, 0.50] {
        m.cuboid(s, frame, [-0.20, y - 0.03, 0.0], [0.20, y + 0.03, 0.03]);
    }
    m
}
