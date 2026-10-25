//! Stylised low-poly operator (the fallback person until rigged CC0 characters ship).

use crate::builder::Model;

/// Shoulder joint height (arms hang from here).
pub const SHOULDER: f32 = 1.44;
/// Hip joint height (legs swing from here).
pub const HIP: f32 = 0.92;
/// Distance from the shoulder to the centre of the hand.
pub const ARM: f32 = 0.67;
/// Distance from the hip to the floor (leg length).
pub const LEG: f32 = HIP;

/// Operator, 1.75 m tall, facing `+x`, with a hi-vis vest and a helmet.
///
/// Nodes: `tronco` (pivot at the hips; head and arms hang from it), `cabeca` (pivot at the
/// neck), `braco_e`, `braco_d` (pivots at the shoulders), `perna_e`, `perna_d` (pivots at
/// the hips). `_e` is the left side (`+y`), `_d` the right. A positive rotation about
/// glTF `+Z` (the lateral axis, asset `-y`) swings a limb forward. Material `pintura` is
/// the vest.
pub fn operador_simples() -> Model {
    let mut m = Model::new("operador_simples");
    let vest = m.material("pintura", 0xf57c00, 0.0, 0.7);
    let stripe = m.material("refletivo", 0xdde3e8, 0.3, 0.3);
    let shirt = m.material("camisa", 0x3b6ea5, 0.0, 0.8);
    let pants = m.material("calca", 0x2c3e50, 0.0, 0.85);
    let skin = m.material("pele", 0xe0ac69, 0.0, 0.7);
    let shoe = m.material("bota", 0x1b1b1b, 0.0, 0.6);
    let helmet = m.material("capacete", 0xffd200, 0.1, 0.4);

    let tronco = m.node("tronco", 0, [0.0, 0.0, HIP]);
    m.cuboid(
        tronco,
        pants,
        [-0.11, -0.18, HIP - 0.05],
        [0.11, 0.18, 1.02],
    );
    m.cuboid(tronco, vest, [-0.12, -0.21, 1.00], [0.12, 0.21, 1.47]);
    for z in [1.12f32, 1.28] {
        m.cuboid(
            tronco,
            stripe,
            [-0.125, -0.215, z],
            [0.125, 0.215, z + 0.035],
        );
    }
    m.cylinder(tronco, skin, [0.0, 0.0, 1.47], [0.0, 0.0, 1.54], 0.05, 8);

    let cabeca = m.node("cabeca", tronco, [0.0, 0.0, 1.54]);
    m.sphere(cabeca, skin, [0.01, 0.0, 1.625], 0.105, 10, 6, false);
    m.sphere(cabeca, helmet, [0.0, 0.0, 1.645], 0.115, 10, 4, true);
    m.cylinder(
        cabeca,
        helmet,
        [0.0, 0.0, 1.640],
        [0.0, 0.0, 1.652],
        0.13,
        10,
    );

    for (name, s) in [("braco_e", 1.0f32), ("braco_d", -1.0)] {
        let y = s * 0.265;
        let n = m.node(name, tronco, [0.0, y, SHOULDER]);
        let (y0, y1) = (y - 0.05, y + 0.05);
        m.cuboid(n, shirt, [-0.055, y0, 1.05], [0.055, y1, SHOULDER + 0.03]);
        m.cuboid(
            n,
            vest,
            [-0.06, y0 - 0.005, 1.30],
            [0.06, y1 + 0.005, SHOULDER + 0.04],
        );
        m.cuboid(
            n,
            skin,
            [-0.045, y0 + 0.01, SHOULDER - ARM - 0.05],
            [0.045, y1 - 0.01, 1.05],
        );
    }
    for (name, s) in [("perna_e", 1.0f32), ("perna_d", -1.0)] {
        let y = s * 0.10;
        let n = m.node(name, 0, [0.0, y, HIP]);
        m.cuboid(n, pants, [-0.075, y - 0.07, 0.08], [0.075, y + 0.07, HIP]);
        m.cuboid(n, shoe, [-0.08, y - 0.065, 0.0], [0.17, y + 0.065, 0.09]);
    }
    m
}
