//! Mobile equipment: counterbalance forklift, pallet jack and AGV.

use crate::builder::Model;

/// Segments of wheels and other round parts (low poly).
const SEG: u32 = 16;

/// A wheel along the lateral axis: tyre plus a slightly wider rim, centred on `pivot`.
fn wheel(m: &mut Model, node: usize, tyre: usize, rim: usize, r: f32, width: f32) {
    let c = m.pivot(node);
    let (a, b) = (
        [c[0], c[1] - width / 2.0, c[2]],
        [c[0], c[1] + width / 2.0, c[2]],
    );
    m.cylinder(node, tyre, a, b, r, SEG);
    let rim_w = width / 2.0 + 0.008;
    m.cylinder(
        node,
        rim,
        [c[0], c[1] - rim_w, c[2]],
        [c[0], c[1] + rim_w, c[2]],
        r * 0.55,
        SEG,
    );
    // A hub bolt pattern so the spin is visible.
    for k in 0..3 {
        let t = std::f32::consts::TAU * k as f32 / 3.0;
        let (dx, dz) = (t.cos() * r * 0.32, t.sin() * r * 0.32);
        m.cuboid(
            node,
            tyre,
            [c[0] + dx - 0.02, c[1] - rim_w - 0.006, c[2] + dz - 0.02],
            [c[0] + dx + 0.02, c[1] + rim_w + 0.006, c[2] + dz + 0.02],
        );
    }
}

/// Counterbalance forklift, 2.3 m to the fork face, 3.45 m with the 1.15 m forks.
///
/// Nodes: `chassi`, `cabine` (seat, overhead guard), `luz` (beacon), `torre` (mast),
/// `carro` (carriage, child of `torre`; its height is the fork top height), `garfos`
/// (child of `carro`), `roda_de`, `roda_dd`, `roda_te`, `roda_td` (pivot on the axle,
/// pivot height = radius).
pub fn empilhadeira() -> Model {
    let mut m = Model::new("empilhadeira");
    let paint = m.material("pintura", 0xf2b705, 0.1, 0.55);
    let dark = m.material("contrapeso", 0x2e3135, 0.3, 0.6);
    let steel = m.material("aco", 0x3a3d42, 0.7, 0.45);
    let fork = m.material("garfo", 0x1e1f22, 0.6, 0.5);
    let tyre = m.material("pneu", 0x141414, 0.0, 0.9);
    let rim = m.material("aro", 0x8a8f96, 0.6, 0.4);
    let seat = m.material("banco", 0x1c1c1c, 0.0, 0.8);
    let beacon = m.emissive("luz", 0xff9d00);

    let chassi = m.node("chassi", 0, [0.0, 0.0, 0.0]);
    // Lower frame between the wheels, deck, counterweight, dashboard.
    m.cuboid(chassi, dark, [-1.22, -0.28, 0.10], [0.70, 0.28, 0.60]);
    m.cuboid(chassi, paint, [-1.25, -0.55, 0.60], [0.75, 0.55, 0.95]);
    m.cuboid(chassi, dark, [-1.30, -0.55, 0.55], [-0.82, 0.55, 1.15]);
    m.cuboid(chassi, paint, [0.35, -0.45, 0.95], [0.75, 0.45, 1.12]);
    // Fenders over the front wheels.
    for s in [-1.0f32, 1.0] {
        let (y0, y1) = if s > 0.0 {
            (0.30, 0.55)
        } else {
            (-0.55, -0.30)
        };
        m.cuboid(chassi, paint, [0.20, y0, 0.66], [0.72, y1, 0.70]);
    }
    // Steering column and wheel.
    m.cylinder(chassi, steel, [0.32, 0.0, 1.10], [0.22, 0.0, 1.38], 0.03, 8);
    m.cylinder(
        chassi,
        seat,
        [0.24, 0.0, 1.37],
        [0.20, 0.0, 1.41],
        0.17,
        SEG,
    );

    let cabine = m.node("cabine", 0, [0.0, 0.0, 0.95]);
    m.cuboid(cabine, seat, [-0.62, -0.25, 0.95], [-0.18, 0.25, 1.06]);
    m.cuboid(cabine, seat, [-0.66, -0.25, 1.06], [-0.56, 0.25, 1.55]);
    // Overhead guard: four posts and a slatted roof.
    let roof = 2.12;
    for (x, lean) in [(0.55f32, -0.10f32), (-0.90, 0.0)] {
        for y in [-0.48f32, 0.48] {
            m.bar(cabine, steel, [x, y, 0.95], [x + lean, y, roof], 0.06, 0.06);
        }
    }
    for y in [-0.48f32, 0.48] {
        m.cuboid(
            cabine,
            steel,
            [-0.93, y - 0.03, roof - 0.04],
            [0.48, y + 0.03, roof + 0.02],
        );
    }
    for k in 0..6 {
        let x = -0.85 + k as f32 * 0.25;
        m.cuboid(
            cabine,
            steel,
            [x - 0.02, -0.48, roof - 0.03],
            [x + 0.02, 0.48, roof + 0.01],
        );
    }

    let luz = m.node("luz", 0, [-0.80, 0.0, roof + 0.02]);
    m.cylinder(
        luz,
        beacon,
        [-0.80, 0.0, roof + 0.02],
        [-0.80, 0.0, roof + 0.12],
        0.06,
        10,
    );

    // Mast: two channels, cross members and the lift cylinder.
    let torre = m.node("torre", 0, [0.86, 0.0, 0.0]);
    for y in [-0.34f32, 0.34] {
        m.cuboid(torre, steel, [0.80, y - 0.05, 0.05], [0.92, y + 0.05, 2.25]);
    }
    for z in [0.30f32, 1.25, 2.18] {
        m.cuboid(torre, steel, [0.82, -0.34, z], [0.90, 0.34, z + 0.07]);
    }
    m.cylinder(torre, rim, [0.86, 0.0, 0.10], [0.86, 0.0, 1.70], 0.045, 10);

    // Carriage: its origin is the top of the fork blades (rest height 0.10 m).
    let carro = m.node("carro", torre, [0.95, 0.0, 0.10]);
    let z0 = 0.10;
    for dz in [0.0f32, 0.40] {
        m.cuboid(
            carro,
            steel,
            [0.93, -0.50, z0 + dz],
            [0.99, 0.50, z0 + dz + 0.10],
        );
    }
    // Load backrest grid.
    for k in 0..6 {
        let y = -0.45 + k as f32 * 0.18;
        m.cuboid(
            carro,
            steel,
            [0.94, y - 0.015, z0 + 0.50],
            [0.97, y + 0.015, z0 + 1.10],
        );
    }
    m.cuboid(
        carro,
        steel,
        [0.94, -0.50, z0 + 1.06],
        [0.97, 0.50, z0 + 1.10],
    );

    let garfos = m.node("garfos", carro, [0.95, 0.0, 0.10]);
    for y in [-0.25f32, 0.25] {
        // Blade (top surface at the carriage origin) and vertical shank.
        m.cuboid(
            garfos,
            fork,
            [0.98, y - 0.05, z0 - 0.05],
            [2.13, y + 0.05, z0],
        );
        m.cuboid(
            garfos,
            fork,
            [0.98, y - 0.05, z0 - 0.05],
            [1.03, y + 0.05, z0 + 0.52],
        );
    }

    for (name, x, y, r, w) in [
        ("roda_de", 0.45f32, 0.42f32, 0.33f32, 0.22f32),
        ("roda_dd", 0.45, -0.42, 0.33, 0.22),
        ("roda_te", -0.95, 0.40, 0.25, 0.18),
        ("roda_td", -0.95, -0.40, 0.25, 0.18),
    ] {
        let n = m.node(name, 0, [x, y, r]);
        wheel(&mut m, n, tyre, rim, r, w);
    }
    m
}

/// Manual pallet jack, 1.65 m long with 1.15 m forks, 0.54 m wide, handle at 1.25 m.
///
/// Nodes: `chassi` (pump and frame), `garfos` (fork top at the pivot height, 0.085 m),
/// `timao` (tiller, pivot on the pump), `roda_direcao`, `roda_e`, `roda_d` (load rollers).
pub fn paleteira() -> Model {
    let mut m = Model::new("paleteira");
    let paint = m.material("pintura", 0xc8102e, 0.2, 0.5);
    let steel = m.material("aco", 0x34373c, 0.7, 0.45);
    let tyre = m.material("pneu", 0x1a1a1a, 0.0, 0.85);
    let rim = m.material("aro", 0x9aa0a6, 0.6, 0.4);
    let grip = m.material("punho", 0x111111, 0.0, 0.9);

    let chassi = m.node("chassi", 0, [-0.30, 0.0, 0.0]);
    m.cuboid(chassi, paint, [-0.42, -0.27, 0.10], [-0.15, 0.27, 0.22]);
    m.cylinder(
        chassi,
        steel,
        [-0.30, 0.0, 0.22],
        [-0.30, 0.0, 0.48],
        0.06,
        12,
    );
    m.cuboid(chassi, paint, [-0.36, -0.08, 0.44], [-0.24, 0.08, 0.50]);

    let garfos = m.node("garfos", 0, [0.0, 0.0, 0.085]);
    for y in [-0.19f32, 0.19] {
        m.cuboid(
            garfos,
            paint,
            [-0.20, y - 0.08, 0.035],
            [1.15, y + 0.08, 0.085],
        );
        // Tapered tip.
        m.cuboid(
            garfos,
            paint,
            [1.15, y - 0.06, 0.040],
            [1.22, y + 0.06, 0.070],
        );
    }
    m.cuboid(garfos, paint, [-0.22, -0.27, 0.035], [-0.15, 0.27, 0.10]);

    let timao = m.node("timao", 0, [-0.30, 0.0, 0.48]);
    m.bar(
        timao,
        steel,
        [-0.30, 0.0, 0.48],
        [-0.47, 0.0, 1.16],
        0.04,
        0.04,
    );
    m.cuboid(timao, grip, [-0.52, -0.16, 1.15], [-0.44, 0.16, 1.25]);

    let n = m.node("roda_direcao", 0, [-0.30, 0.0, 0.10]);
    m.cylinder(
        n,
        tyre,
        [-0.30, -0.06, 0.10],
        [-0.30, 0.06, 0.10],
        0.10,
        SEG,
    );
    m.cylinder(
        n,
        rim,
        [-0.30, -0.065, 0.10],
        [-0.30, 0.065, 0.10],
        0.05,
        SEG,
    );
    for (name, y) in [("roda_e", 0.19f32), ("roda_d", -0.19)] {
        let n = m.node(name, 0, [1.00, y, 0.04]);
        m.cylinder(
            n,
            tyre,
            [1.00, y - 0.05, 0.04],
            [1.00, y + 0.05, 0.04],
            0.04,
            10,
        );
        m.cuboid(n, rim, [0.995, y - 0.055, 0.02], [1.005, y + 0.055, 0.06]);
    }
    m
}

/// Low-profile AGV, 1.1 x 0.8 x 0.35 m, with a status light band.
///
/// Nodes: `chassi` (body, deck, bumpers, casters), `luz` (light band around the body;
/// the engine colours it by the motion), `roda_e`, `roda_d` (drive wheels).
pub fn agv() -> Model {
    let mut m = Model::new("agv");
    let paint = m.material("pintura", 0xdfe3e8, 0.1, 0.5);
    let deck = m.material("borracha", 0x2b2d31, 0.0, 0.9);
    let bumper = m.material("para_choque", 0xf5c400, 0.0, 0.6);
    let sensor = m.material("sensor", 0x101114, 0.3, 0.3);
    let tyre = m.material("pneu", 0x161616, 0.0, 0.85);
    let light = m.emissive("luz", 0xe8f4ff);

    let chassi = m.node("chassi", 0, [0.0, 0.0, 0.0]);
    m.cuboid(chassi, deck, [-0.50, -0.34, 0.02], [0.50, 0.34, 0.12]);
    m.cuboid(chassi, paint, [-0.55, -0.40, 0.12], [0.55, 0.40, 0.33]);
    m.cuboid(chassi, deck, [-0.50, -0.36, 0.33], [0.50, 0.36, 0.35]);
    // Yellow bumper at the front, dark at the back, so the heading reads at a glance.
    for (s, mat) in [(1.0f32, bumper), (-1.0, deck)] {
        let x = s * 0.555;
        m.cuboid(
            chassi,
            mat,
            [x - 0.015, -0.38, 0.10],
            [x + 0.015, 0.38, 0.18],
        );
    }
    // Chevron on the deck pointing forward.
    for y in [-0.12f32, 0.12] {
        m.bar(
            chassi,
            bumper,
            [0.10, y, 0.352],
            [0.30, 0.0, 0.352],
            0.05,
            0.004,
        );
    }
    // Front safety scanner window.
    m.cuboid(chassi, sensor, [0.553, -0.15, 0.26], [0.565, 0.15, 0.31]);
    // Corner casters.
    for x in [-0.42f32, 0.42] {
        for y in [-0.28f32, 0.28] {
            m.cylinder(
                chassi,
                tyre,
                [x, y - 0.02, 0.025],
                [x, y + 0.02, 0.025],
                0.025,
                8,
            );
        }
    }

    let luz = m.node("luz", 0, [0.0, 0.0, 0.24]);
    let (z0, z1) = (0.225, 0.255);
    m.cuboid(luz, light, [-0.56, -0.405, z0], [0.56, -0.395, z1]);
    m.cuboid(luz, light, [-0.56, 0.395, z0], [0.56, 0.405, z1]);
    m.cuboid(luz, light, [0.555, -0.30, z0], [0.565, 0.30, z1]);
    m.cuboid(luz, light, [-0.565, -0.30, z0], [-0.555, 0.30, z1]);

    for (name, y) in [("roda_e", 0.37f32), ("roda_d", -0.37)] {
        let n = m.node(name, 0, [0.0, y, 0.10]);
        m.cylinder(
            n,
            tyre,
            [0.0, y - 0.03, 0.10],
            [0.0, y + 0.03, 0.10],
            0.10,
            SEG,
        );
        m.cuboid(n, paint, [-0.02, y - 0.035, 0.08], [0.02, y + 0.035, 0.12]);
    }
    m
}
