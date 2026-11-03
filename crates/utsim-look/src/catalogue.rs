//! The third-party asset catalogue (`assets/terceiros/catalogo.ron`): CC0 and CC-BY
//! models (people, machines, belts, storage) that `visual.asset` can name, besides the
//! built-in library of `utsim-assets`.
//!
//! The catalogue text is compiled in, so the asset picker lists the same names on the
//! desktop, in the browser and in the Godot interface. The `.glb` files themselves are read from
//! disk (desktop only) from the folder found by [`dir`]. Each entry carries the numbers
//! that put the file at real size facing `+x` (the front of the UTSim models):
//! `escala_real`, `giro_y_graus` and the footprint (`minimo_m`, `centro_m`), see
//! [`Entry::placement`].

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;

/// The catalogue as compiled into the program.
const TEXT: &str = include_str!("../../../assets/terceiros/catalogo.ron");

#[derive(Debug, Deserialize)]
struct Catalogue {
    #[allow(dead_code)] // read by the asset test (`utsim-assets/tests/terceiros.rs`)
    versao: u32,
    ativos: Vec<Entry>,
}

/// One third-party asset (the fields the interfaces use; the rest of the entry is ignored).
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Entry {
    /// Name used in `visual.asset`.
    pub nome: String,
    /// File, relative to the `assets/terceiros` folder.
    pub arquivo: String,
    pub autor: String,
    pub licenca: String,
    /// What the asset is for (Portuguese).
    pub uso: String,
    /// Bounding box size and minimum corner of the file, metres, glTF axes, unscaled.
    pub tamanho_m: (f32, f32, f32),
    pub minimo_m: (f32, f32, f32),
    /// Centre of the footprint `(x, z)`, glTF axes, unscaled.
    pub centro_m: (f32, f32),
    /// Rotation about the vertical that turns the file's front to `+x`, degrees.
    pub giro_y_graus: f32,
    /// Factor that takes the file to real size in metres.
    pub escala_real: f32,
    /// Animation clips (people).
    #[serde(default)]
    pub clipes: Vec<String>,
    /// Size once placed at real size, metres (for the picker's help text).
    #[serde(default)]
    pub tamanho_real_m: (f32, f32, f32),
    /// Seat of a vehicle that carries a driver (forklifts), see [`Motorista`].
    #[serde(default)]
    pub motorista: Option<Motorista>,
}

/// Where the driver sits in a vehicle of the catalogue: the anchor point of the hip joint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Motorista {
    /// Hip joint on the seat, in the frame of the placed vehicle (real size, centred on
    /// the footprint, floor at 0): `x` forward, `y` up, `z` to the right, metres.
    pub quadril_m: (f32, f32, f32),
    /// Extra turn of the driver about the vertical, degrees (0 faces the vehicle's front).
    #[serde(default)]
    pub giro_graus: f32,
    /// Size of the driver relative to the catalogue's mannequin (a small cab takes a
    /// smaller one).
    #[serde(default = "um")]
    pub escala: f32,
}

fn um() -> f32 {
    1.0
}

/// How an interface places the glTF scene of an entry under the block's root.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placement {
    /// Uniform scale (catalogue `escala_real` times `visual.scale`).
    pub scale: f32,
    /// Rotation about the vertical, radians (`giro_y_graus` plus `visual.yaw`).
    pub yaw: f32,
    /// Translation of the scene, in the root's space (Y up, the same in Bevy and Godot), that puts the footprint
    /// centre on the root and the lowest point on the floor.
    pub offset: [f32; 3],
}

impl Entry {
    /// Licence hint shown next to the name (author and licence).
    pub fn credit(&self) -> String {
        format!("{}, {}", self.autor, self.licenca)
    }

    /// Placement with the user's extra `scale` and `yaw` (radians) on top of the
    /// catalogue numbers.
    pub fn placement(&self, scale: f32, yaw: f32) -> Placement {
        let s = self.escala_real * scale;
        let yaw = self.giro_y_graus.to_radians() + yaw;
        // The point (cx, min_y, cz) of the file goes to the root's origin: rotate it
        // about Y like Bevy does (x' = x cos + z sin, z' = -x sin + z cos) and negate.
        let (cx, cy, cz) = (self.centro_m.0, self.minimo_m.1, self.centro_m.1);
        let (sin, cos) = yaw.sin_cos();
        let x = cx * cos + cz * sin;
        let z = -cx * sin + cz * cos;
        Placement {
            scale: s,
            yaw,
            offset: [-x * s, -cy * s, -z * s],
        }
    }

    /// Real size `(length along +x, height, width)` once placed, metres.
    pub fn real_size(&self) -> [f32; 3] {
        let (x, y, z) = self.tamanho_m;
        let s = self.escala_real;
        // A quarter turn swaps the horizontal axes.
        let quarter = (self.giro_y_graus / 90.0).round() as i32 % 2 != 0;
        if quarter {
            [z * s, y * s, x * s]
        } else {
            [x * s, y * s, z * s]
        }
    }
}

/// Every entry, in catalogue order.
pub fn entries() -> &'static [Entry] {
    static CATALOGUE: OnceLock<Vec<Entry>> = OnceLock::new();
    CATALOGUE.get_or_init(|| match ron::from_str::<Catalogue>(TEXT) {
        Ok(c) => c.ativos,
        Err(_) => Vec::new(),
    })
}

/// The entry called `name` (exact, ignoring case and surrounding spaces).
pub fn by_name(name: &str) -> Option<&'static Entry> {
    let n = name.trim();
    entries().iter().find(|e| e.nome.eq_ignore_ascii_case(n))
}

/// The `assets/terceiros` folder: `UTSIM_TERCEIROS` when set, else the first
/// `assets/terceiros` holding `catalogo.ron` above the executable or the working
/// directory, else the folder of the source tree this crate was built from.
pub fn dir() -> PathBuf {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| search(None)).clone()
}

/// Like [`dir`], but looks above `start` (the model's folder) first.
pub fn dir_from(start: Option<&Path>) -> PathBuf {
    match start {
        Some(s) => search(Some(s)),
        None => dir(),
    }
}

fn search(start: Option<&Path>) -> PathBuf {
    if let Some(d) = std::env::var_os("UTSIM_TERCEIROS") {
        return PathBuf::from(d);
    }
    let starts = [
        start.map(Path::to_path_buf),
        std::env::current_exe().ok(),
        std::env::current_dir().ok(),
    ];
    for s in starts.into_iter().flatten() {
        for a in s.ancestors() {
            let d = a.join("assets").join("terceiros");
            if d.join("catalogo.ron").is_file() {
                return d;
            }
        }
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/terceiros")
}

/// The file of `entry` on disk.
pub fn file_of(entry: &Entry) -> PathBuf {
    dir().join(&entry.arquivo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn the_compiled_catalogue_parses_and_its_files_exist() {
        let all = entries();
        assert!(all.len() >= 30, "{}", all.len());
        for e in all {
            assert!(file_of(e).is_file(), "{}", e.arquivo);
            assert!(e.escala_real > 0.0 && !e.credit().is_empty());
            // Names never shadow the built-in library.
            assert!(utsim_assets::Spec::by_name(&e.nome).is_none(), "{}", e.nome);
        }
        assert_eq!(by_name(" Maquina_Kenney ").unwrap().autor, "Kenney");
        assert!(by_name("maquina").is_none());
        // The model's folder is searched first; any folder of the tree finds the same one.
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(dir_from(Some(here)).join("catalogo.ron").is_file());
        let p = by_name("pessoa_operador").unwrap();
        assert!(p.arquivo.ends_with("UAL1_Standard.glb"));
        assert!(p.tamanho_real_m.1 > 1.0, "{:?}", p.tamanho_real_m);
    }

    #[test]
    fn placement_centres_the_footprint_at_real_size_facing_x() {
        // An off-centre file (the forklift's origin is 54 m away from its body).
        let e = by_name("empilhadeira_kolosstudios").unwrap();
        let p = e.placement(1.0, 0.0);
        assert!((p.yaw - FRAC_PI_2).abs() < 1e-6);
        // The footprint centre of the file lands on the root's origin, the floor at 0.
        let (sin, cos) = p.yaw.sin_cos();
        let (cx, cz) = e.centro_m;
        let x = (cx * cos + cz * sin) * p.scale + p.offset[0];
        let z = (-cx * sin + cz * cos) * p.scale + p.offset[2];
        assert!(x.abs() < 1e-3 && z.abs() < 1e-3, "{x} {z}");
        assert!((e.minimo_m.1 * p.scale + p.offset[1]).abs() < 1e-6);
        // A miniature is scaled up, and the user's scale multiplies it.
        let b = by_name("bancada_kenney").unwrap();
        assert_eq!(b.placement(1.0, 0.0).scale, 4.0);
        assert_eq!(b.placement(0.5, 0.0).scale, 2.0);
        // The +z front turned to +x: the long side of the forklift is now along x.
        let size = e.real_size();
        assert!(size[0] > size[2], "{size:?}");
    }
}
