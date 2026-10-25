//! Fixed equipment: selective rack, roller conveyor, tank, machine, workbench and a
//! painted floor area. The rack, the conveyor, the tank and the area are parametric.

use crate::builder::Model;

/// Selective pallet rack. The bays run along `x`, the depth along `y`; level `k` has its
/// load surface (top of the beams) at `k * level_height + beam`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RackSpec {
    pub bays: usize,
    pub levels: usize,
    /// Width of one bay (distance between upright centres), metres.
    pub bay_width: f32,
    /// Height of one level, metres.
    pub level_height: f32,
    /// Depth of the upright frames, metres.
    pub depth: f32,
    /// Height of a beam, metres.
    pub beam: f32,
}

impl Default for RackSpec {
    fn default() -> Self {
        RackSpec {
            bays: 3,
            levels: 4,
            bay_width: 1.4,
            level_height: 1.5,
            depth: 1.0,
            beam: 0.08,
        }
    }
}

impl RackSpec {
    /// Height of the uprights.
    pub fn height(&self) -> f32 {
        self.levels as f32 * self.level_height + self.beam
    }

    pub fn width(&self) -> f32 {
        self.bays as f32 * self.bay_width
    }
}

/// Rack: uprights with bracing (blue) and a pair of beams per level (orange). Material
/// `pintura` is the beams. Node: `estrutura` (static).
pub fn porta_paletes(spec: &RackSpec) -> Model {
    let mut m = Model::new("porta_paletes");
    let upright = m.material("montante", 0x1f4e9a, 0.3, 0.5);
    let beam = m.material("pintura", 0xe8590c, 0.2, 0.5);
    let plate = m.material("aco", 0x55595f, 0.6, 0.5);
    let s = m.node("estrutura", 0, [0.0, 0.0, 0.0]);
    let (w, d, h) = (spec.width(), spec.depth, spec.height());
    let post = 0.08f32.min(spec.bay_width * 0.2);
    let yf = d / 2.0 - post / 2.0;
    for k in 0..=spec.bays {
        let x = k as f32 * spec.bay_width - w / 2.0;
        for y in [-yf, yf] {
            m.cuboid(
                s,
                upright,
                [x - post / 2.0, y - post / 2.0, 0.0],
                [x + post / 2.0, y + post / 2.0, h],
            );
            m.cuboid(
                s,
                plate,
                [x - 0.07, y - 0.07, 0.0],
                [x + 0.07, y + 0.07, 0.01],
            );
        }
        // Frame bracing: horizontals and a zig-zag of diagonals between the two posts.
        let panels = ((h / 0.9).round() as usize).max(1);
        let step = h / panels as f32;
        for p in 0..=panels {
            let z = (p as f32 * step).clamp(0.15, h - 0.05);
            m.bar(s, upright, [x, -yf, z], [x, yf, z], 0.03, 0.03);
        }
        for p in 0..panels {
            let (z0, z1) = (
                (p as f32 * step).max(0.15),
                ((p + 1) as f32 * step).min(h - 0.05),
            );
            let (a, b) = if p % 2 == 0 { (-yf, yf) } else { (yf, -yf) };
            m.bar(s, upright, [x, a, z0], [x, b, z1], 0.025, 0.025);
        }
    }
    // Beams: front and back of every load level, top at `k * level_height + beam`.
    let bt = 0.05f32;
    for k in 0..spec.levels {
        let z = k as f32 * spec.level_height;
        for b in 0..spec.bays {
            let x0 = b as f32 * spec.bay_width - w / 2.0 + post / 2.0;
            let x1 = x0 + spec.bay_width - post;
            for y in [-(d / 2.0 - bt / 2.0), d / 2.0 - bt / 2.0] {
                m.cuboid(
                    s,
                    beam,
                    [x0, y - bt / 2.0, z],
                    [x1, y + bt / 2.0, z + spec.beam],
                );
            }
        }
    }
    m
}

/// Roller conveyor section along `x`, centred on the origin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConveyorSpec {
    pub length: f32,
    /// Outside width (between the side frames), metres.
    pub width: f32,
    /// Height of the top of the rollers (the transport surface), metres.
    pub height: f32,
    /// Distance between roller axes, metres.
    pub pitch: f32,
}

impl Default for ConveyorSpec {
    fn default() -> Self {
        ConveyorSpec {
            length: 3.0,
            width: 0.8,
            height: 0.75,
            pitch: 0.125,
        }
    }
}

/// Most rollers drawn; longer conveyors space them wider.
const MAX_ROLLERS: usize = 400;

/// Roller conveyor: side frames, legs every 1.5 m at most and rollers. Material
/// `pintura` is the side frames. Nodes: `estrutura`, `rolos` (static).
pub fn esteira(spec: &ConveyorSpec) -> Model {
    let mut m = Model::new("esteira");
    let frame = m.material("pintura", 0x5b6470, 0.4, 0.5);
    let roller = m.material("rolo", 0xb8bec6, 0.8, 0.3);
    let leg = m.material("perna", 0x2f3338, 0.5, 0.6);
    let (l, w, h) = (
        spec.length.max(0.2),
        spec.width.max(0.2),
        spec.height.max(0.05),
    );
    let s = m.node("estrutura", 0, [0.0, 0.0, 0.0]);
    let frame_h = (0.14f32).min(h * 0.6);
    let ft = 0.04f32;
    for y in [-(w / 2.0 - ft / 2.0), w / 2.0 - ft / 2.0] {
        m.cuboid(
            s,
            frame,
            [-l / 2.0, y - ft / 2.0, h - frame_h],
            [l / 2.0, y + ft / 2.0, h + 0.01],
        );
    }
    let spans = ((l / 1.5).ceil() as usize).max(1);
    for k in 0..=spans {
        let x = (-l / 2.0 + k as f32 * l / spans as f32).clamp(-l / 2.0 + 0.03, l / 2.0 - 0.03);
        for y in [-(w / 2.0 - ft / 2.0), w / 2.0 - ft / 2.0] {
            m.cuboid(
                s,
                leg,
                [x - 0.025, y - 0.02, 0.0],
                [x + 0.025, y + 0.02, h - frame_h],
            );
        }
        let zb = ((h - frame_h) * 0.35).max(0.01);
        m.cuboid(
            s,
            leg,
            [x - 0.02, -w / 2.0 + ft, zb - 0.015],
            [x + 0.02, w / 2.0 - ft, zb + 0.015],
        );
    }
    let r = (0.025f32).min(h * 0.2);
    let mut pitch = spec.pitch.max(2.2 * r);
    if (l / pitch) as usize > MAX_ROLLERS {
        pitch = l / MAX_ROLLERS as f32;
    }
    let count = ((l / pitch).floor() as usize).max(1);
    let rolos = m.node("rolos", 0, [0.0, 0.0, h - r]);
    let start = -(count as f32 - 1.0) * pitch / 2.0;
    for k in 0..count {
        let x = start + k as f32 * pitch;
        m.cylinder(
            rolos,
            roller,
            [x, -w / 2.0 + ft, h - r],
            [x, w / 2.0 - ft, h - r],
            r,
            8,
        );
    }
    m
}

/// Vertical tank on legs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TankSpec {
    pub radius: f32,
    /// Height of the cylindrical shell, metres.
    pub height: f32,
    /// Height of the legs (bottom of the shell), metres.
    pub legs: f32,
}

impl Default for TankSpec {
    fn default() -> Self {
        TankSpec {
            radius: 1.0,
            height: 2.6,
            legs: 0.6,
        }
    }
}

/// Tank: shell with a conical roof and a bottom, four legs, a ladder and a nozzle.
/// Material `pintura` is the shell. Nodes: `estrutura` (legs, ladder), `casco` (shell).
pub fn tanque(spec: &TankSpec) -> Model {
    let mut m = Model::new("tanque");
    let shell = m.material("pintura", 0xc7ccd1, 0.8, 0.3);
    let steel = m.material("aco", 0x5f656c, 0.7, 0.45);
    let (r, h, legs) = (
        spec.radius.max(0.1),
        spec.height.max(0.1),
        spec.legs.max(0.0),
    );
    let s = m.node("estrutura", 0, [0.0, 0.0, 0.0]);
    for k in 0..4 {
        let t = std::f32::consts::FRAC_PI_4 + k as f32 * std::f32::consts::FRAC_PI_2;
        let (x, y) = (t.cos() * r * 0.8, t.sin() * r * 0.8);
        m.cuboid(
            s,
            steel,
            [x - 0.05, y - 0.05, 0.0],
            [x + 0.05, y + 0.05, legs + 0.1],
        );
    }
    // Ladder on the -y side.
    let top = legs + h;
    for x in [-0.2f32, 0.2] {
        m.cuboid(
            s,
            steel,
            [x - 0.02, -r - 0.12, legs],
            [x + 0.02, -r - 0.08, top + 0.1],
        );
    }
    let rungs = ((h / 0.3) as usize).max(1);
    for k in 0..=rungs {
        let z = legs + 0.15 + k as f32 * (h - 0.1) / rungs as f32;
        m.cuboid(
            s,
            steel,
            [-0.2, -r - 0.115, z - 0.015],
            [0.2, -r - 0.085, z + 0.015],
        );
    }
    let casco = m.node("casco", 0, [0.0, 0.0, legs]);
    m.cylinder(casco, shell, [0.0, 0.0, legs], [0.0, 0.0, top], r, 20);
    m.frustum(
        casco,
        shell,
        [0.0, 0.0, top],
        [0.0, 0.0, top + 0.25 * r],
        r,
        0.12 * r,
        20,
    );
    m.cylinder(
        casco,
        steel,
        [0.0, 0.0, top + 0.25 * r],
        [0.0, 0.0, top + 0.25 * r + 0.12],
        0.1 * r,
        10,
    );
    m.cylinder(
        casco,
        steel,
        [r - 0.05, 0.0, legs + 0.3],
        [r + 0.2, 0.0, legs + 0.3],
        0.06,
        10,
    );
    m
}

/// Generic processing machine, 1.8 x 1.4 m, 2.2 m to the top of the light tower.
/// The front (door window, control panel) faces `-y`. Material `pintura` is the
/// cabinet. Nodes: `chassi`, `porta` (door window), `luz` (status light; engines colour
/// it by the state).
pub fn maquina() -> Model {
    let mut m = Model::new("maquina");
    let base = m.material("base", 0x2b2e33, 0.4, 0.6);
    let paint = m.material("pintura", 0xe6e9ed, 0.2, 0.45);
    let accent = m.material("faixa", 0x2b6cb0, 0.2, 0.45);
    let glass = m.translucent("vidro", 0x6f8fa8, 0.45);
    let panel = m.material("painel", 0x3a3f46, 0.3, 0.5);
    let light = m.emissive("luz", 0x46d160);

    let c = m.node("chassi", 0, [0.0, 0.0, 0.0]);
    m.cuboid(c, base, [-0.90, -0.70, 0.0], [0.90, 0.70, 0.15]);
    m.cuboid(c, paint, [-0.85, -0.65, 0.15], [0.85, 0.65, 1.75]);
    m.cuboid(c, accent, [-0.86, -0.66, 1.60], [0.86, 0.66, 1.70]);
    m.cuboid(c, base, [-0.80, -0.60, 1.75], [0.80, 0.60, 1.85]);
    // Control panel on an arm, with three buttons.
    m.cuboid(c, panel, [0.45, -0.80, 0.95], [0.82, -0.66, 1.50]);
    for (k, rgb) in [0x2ecc40u32, 0xff4136, 0xffdc00].iter().enumerate() {
        let mat = m.material(&format!("botao_{k}"), *rgb, 0.0, 0.5);
        let x = 0.52 + k as f32 * 0.11;
        m.cylinder(c, mat, [x, -0.80, 1.05], [x, -0.82, 1.05], 0.025, 8);
    }
    // Light tower pole.
    m.cylinder(c, panel, [0.70, 0.45, 1.85], [0.70, 0.45, 2.00], 0.02, 8);

    let porta = m.node("porta", 0, [-0.10, -0.66, 0.15]);
    m.cuboid(porta, paint, [-0.70, -0.68, 0.30], [0.30, -0.65, 1.50]);
    m.cuboid(porta, glass, [-0.60, -0.69, 0.60], [0.20, -0.67, 1.40]);

    let luz = m.node("luz", 0, [0.70, 0.45, 2.00]);
    m.cylinder(
        luz,
        light,
        [0.70, 0.45, 2.00],
        [0.70, 0.45, 2.20],
        0.055,
        12,
    );
    m
}

/// Workbench for operators, 1.5 x 0.75 m, top at 0.9 m, with a back panel. The working
/// side faces `-y`. Node: `estrutura` (static).
pub fn bancada() -> Model {
    let mut m = Model::new("bancada");
    let top = m.material("tampo", 0xa0764a, 0.0, 0.7);
    let steel = m.material("pintura", 0x4a6a8a, 0.4, 0.5);
    let board = m.material("painel", 0x8c949c, 0.2, 0.7);
    let s = m.node("estrutura", 0, [0.0, 0.0, 0.0]);
    m.cuboid(s, top, [-0.75, -0.375, 0.86], [0.75, 0.375, 0.90]);
    for x in [-0.70f32, 0.70] {
        for y in [-0.32f32, 0.32] {
            m.cuboid(
                s,
                steel,
                [x - 0.025, y - 0.025, 0.0],
                [x + 0.025, y + 0.025, 0.86],
            );
        }
    }
    m.cuboid(s, steel, [-0.70, -0.32, 0.14], [0.70, 0.32, 0.17]);
    m.cuboid(s, board, [-0.75, 0.33, 0.90], [0.75, 0.37, 1.50]);
    m.cuboid(s, steel, [-0.75, 0.30, 1.30], [0.75, 0.33, 1.33]);
    m
}

/// Painted floor area (a queue or buffer zone).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AreaSpec {
    pub length: f32,
    pub width: f32,
}

impl Default for AreaSpec {
    fn default() -> Self {
        AreaSpec {
            length: 4.0,
            width: 1.4,
        }
    }
}

/// Floor area: a grey plate with a yellow border line and chevrons at the exit (`+x`).
/// Node: `piso` (static).
pub fn area(spec: &AreaSpec) -> Model {
    let mut m = Model::new("area");
    let floor = m.material("piso", 0x4a4f57, 0.0, 0.95);
    let paint = m.material("pintura", 0xf2c500, 0.0, 0.7);
    let s = m.node("piso", 0, [0.0, 0.0, 0.0]);
    let (hl, hw, b) = (
        spec.length.max(0.2) / 2.0,
        spec.width.max(0.2) / 2.0,
        0.06f32,
    );
    m.cuboid(s, floor, [-hl, -hw, 0.0], [hl, hw, 0.004]);
    m.cuboid(s, paint, [-hl, -hw, 0.0], [hl, -hw + b, 0.008]);
    m.cuboid(s, paint, [-hl, hw - b, 0.0], [hl, hw, 0.008]);
    m.cuboid(s, paint, [-hl, -hw, 0.0], [-hl + b, hw, 0.008]);
    m.cuboid(s, paint, [hl - b, -hw, 0.0], [hl, hw, 0.008]);
    // An arrow at the exit end.
    let x = hl - 0.35;
    m.bar(
        s,
        paint,
        [x - 0.18, -0.16, 0.006],
        [x, 0.0, 0.006],
        0.05,
        0.004,
    );
    m.bar(
        s,
        paint,
        [x - 0.18, 0.16, 0.006],
        [x, 0.0, 0.006],
        0.05,
        0.004,
    );
    m
}
