//! Loads: pallet and carton box.

use crate::builder::Model;

/// Wooden pallet, 1.2 x 1.0 x 0.144 m (EUR-like construction: 5 top boards, 3 cross
/// boards, 9 blocks, 3 bottom boards). Node: `palete` (the root itself, static).
pub fn palete() -> Model {
    let mut m = Model::new("palete");
    let wood = m.material("madeira", 0xc49a6c, 0.0, 0.85);
    let block = m.material("bloco", 0xa77b4f, 0.0, 0.9);
    let (hx, hy) = (0.6f32, 0.5f32);
    // Top deck: 5 boards along the length.
    for k in 0..5 {
        let y = -hy + 0.0725 + k as f32 * (2.0 * hy - 0.145) / 4.0;
        m.cuboid(0, wood, [-hx, y - 0.0725, 0.122], [hx, y + 0.0725, 0.144]);
    }
    let rows = [-hy + 0.0725, 0.0, hy - 0.0725];
    let cols = [-hx + 0.0725, 0.0, hx - 0.0725];
    // Cross boards under the deck, blocks, bottom boards.
    for &x in &cols {
        m.cuboid(0, wood, [x - 0.0725, -hy, 0.100], [x + 0.0725, hy, 0.122]);
        for &y in &rows {
            m.cuboid(
                0,
                block,
                [x - 0.0725, y - 0.0725, 0.022],
                [x + 0.0725, y + 0.0725, 0.100],
            );
        }
    }
    for &y in &rows {
        m.cuboid(0, wood, [-hx, y - 0.0725, 0.0], [hx, y + 0.0725, 0.022]);
    }
    m
}

/// Carton box, 0.40 x 0.30 x 0.30 m, with a tape strip and a label. Material `papelao`
/// is the one engines tint. Node: `caixa` (the root itself, static).
pub fn caixa() -> Model {
    let mut m = Model::new("caixa");
    let card = m.material("papelao", 0xb8875a, 0.0, 0.9);
    let tape = m.material("fita", 0xd8b98c, 0.0, 0.5);
    let label = m.material("etiqueta", 0xf2f2f2, 0.0, 0.7);
    m.cuboid(0, card, [-0.20, -0.15, 0.0], [0.20, 0.15, 0.30]);
    // Tape over the lid seam and down both ends.
    m.cuboid(0, tape, [-0.203, -0.025, 0.299], [0.203, 0.025, 0.303]);
    for s in [-1.0f32, 1.0] {
        let x = s * 0.2015;
        m.cuboid(
            0,
            tape,
            [x - 0.0015, -0.025, 0.20],
            [x + 0.0015, 0.025, 0.30],
        );
    }
    m.cuboid(0, label, [-0.12, -0.153, 0.10], [0.02, -0.149, 0.20]);
    m
}
