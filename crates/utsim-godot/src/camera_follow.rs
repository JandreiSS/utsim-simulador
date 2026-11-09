//! Where the cameras that ride a block put the eye and what they look at (step 7, part 3,
//! of `docs/plano_interface_rust.md`): "Primeira pessoa" (a person's eye height, a little
//! ahead) and "Do recurso" (the driving position: above the vehicle, or a forklift
//! driver's head). Godot coordinates (`y` up). The main scene places the camera and hides
//! what is on the sight line.

pub type V3 = [f64; 3];

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn mul(a: V3, k: f64) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}

/// The horizontal direction the block faces (its `x` axis on the floor); `+x` when it
/// points straight up or down.
pub fn flat_forward(axis_x: V3) -> V3 {
    let l = (axis_x[0] * axis_x[0] + axis_x[2] * axis_x[2]).sqrt();
    if l < 1e-3 {
        [1.0, 0.0, 0.0]
    } else {
        [axis_x[0] / l, 0.0, axis_x[2] / l]
    }
}

/// The eye and the aim of camera `mode` riding a block at `at` facing `axis_x`: `top` is
/// the block's height, `head` a forklift driver's eye when it has one. `None` for the
/// cameras that do not ride a block (`orbita`, `planta`, `seguir`).
pub fn view(mode: &str, at: V3, axis_x: V3, top: f64, head: Option<V3>) -> Option<(V3, V3)> {
    let fwd = flat_forward(axis_x);
    match mode {
        "primeira_pessoa" => {
            let eye = add(add(at, [0.0, 1.6, 0.0]), mul(fwd, 0.35));
            Some((eye, add(add(eye, mul(fwd, 10.0)), [0.0, -1.2, 0.0])))
        }
        "recurso" => {
            let (eye, eye_y) = match head {
                Some(h) => (h, h[1] - at[1]),
                None => {
                    let y = (top + 0.6).clamp(1.4, 2.4);
                    (add(at, [0.0, y, 0.0]), y)
                }
            };
            Some((eye, add(add(eye, mul(fwd, 10.0)), [0.0, -eye_y * 0.8, 0.0])))
        }
        _ => None,
    }
}

/// How the camera of Simular follows the block chosen in the list, each frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Conducao {
    /// Órbita and Planta (or no block chosen): the camera is the user's.
    Livre,
    /// Seguir: the orbit camera aims at the block.
    Alvo,
    /// Primeira pessoa and Do recurso: the eye rides the block ([`view`]).
    Olho,
}

/// How camera `mode` follows `seguido` (empty: none chosen).
pub fn conducao(mode: &str, seguido: &str) -> Conducao {
    match mode {
        _ if seguido.is_empty() => Conducao::Livre,
        "seguir" => Conducao::Alvo,
        "primeira_pessoa" | "recurso" => Conducao::Olho,
        _ => Conducao::Livre,
    }
}

/// The block a new replication follows: the one followed before when it is still there,
/// else the first mobile one (`blocos`: name and mobile), else none.
pub fn seguido(atual: &str, blocos: &[(String, bool)]) -> String {
    if !atual.is_empty() && blocos.iter().any(|(n, _)| n == atual) {
        return atual.to_owned();
    }
    blocos
        .iter()
        .find(|(_, movel)| *movel)
        .map(|(n, _)| n.clone())
        .unwrap_or_default()
}

/// Whether choosing `mode` gives back the blocks hidden on a riding camera's sight line.
pub fn restaura_vista(mode: &str) -> bool {
    !matches!(mode, "primeira_pessoa" | "recurso")
}

/// What the status bar says when camera `mode` is chosen.
pub fn aviso(mode: &str, seguido: &str) -> String {
    let label = crate::ui_left::CAMERAS
        .iter()
        .find(|(id, _)| *id == mode)
        .map_or(mode, |(_, l)| *l);
    match mode {
        "orbita" => "Câmera de órbita.".to_owned(),
        "planta" => {
            "Planta (2D): vista de cima, sem perspectiva; arraste com o botão direito para mover."
                .to_owned()
        }
        _ if seguido.is_empty() => "Escolha na lista o bloco que a câmera acompanha.".to_owned(),
        _ => format!("Câmera {}: {seguido}.", label.to_lowercase()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_person_is_at_eye_height_a_little_ahead_looking_down_the_way() {
        let (eye, aim) = view(
            "primeira_pessoa",
            [2.0, 0.0, 3.0],
            [0.0, 0.0, -2.0],
            1.0,
            None,
        )
        .unwrap();
        assert_eq!(eye, [2.0, 1.6, 3.0 - 0.35]);
        let want = [2.0, 0.4, 3.0 - 0.35 - 10.0];
        assert!((0..3).all(|k| (aim[k] - want[k]).abs() < 1e-12), "{aim:?}");
    }

    #[test]
    fn the_resource_camera_sits_above_the_vehicle_or_at_the_driver() {
        // A low AGV: at least 1.4 m; a tall one: at most 2.4 m.
        let (eye, _) = view("recurso", [0.0; 3], [1.0, 0.0, 0.0], 0.3, None).unwrap();
        assert_eq!(eye[1], 1.4);
        let (eye, _) = view("recurso", [0.0; 3], [1.0, 0.0, 0.0], 5.0, None).unwrap();
        assert_eq!(eye[1], 2.4);
        // A forklift's driver: the eye is the head, looking ahead and down from there.
        let (eye, aim) = view(
            "recurso",
            [0.0; 3],
            [1.0, 0.0, 0.0],
            2.0,
            Some([0.1, 1.9, 0.0]),
        )
        .unwrap();
        assert_eq!(eye, [0.1, 1.9, 0.0]);
        assert!((aim[1] - (1.9 - 1.9 * 0.8)).abs() < 1e-12 && aim[0] > 10.0);
        // A block facing straight up still looks along +x; the other cameras ride nothing.
        assert_eq!(flat_forward([0.0, 1.0, 0.0]), [1.0, 0.0, 0.0]);
        assert!(view("orbita", [0.0; 3], [1.0, 0.0, 0.0], 1.0, None).is_none());
    }

    #[test]
    fn the_follow_decisions() {
        assert_eq!(conducao("seguir", "agv_1"), Conducao::Alvo);
        assert_eq!(conducao("recurso", "agv_1"), Conducao::Olho);
        assert_eq!(conducao("primeira_pessoa", ""), Conducao::Livre);
        assert_eq!(conducao("orbita", "agv_1"), Conducao::Livre);
        let blocos = vec![
            ("fonte".to_owned(), false),
            ("agv_1".to_owned(), true),
            ("agv_2".to_owned(), true),
        ];
        assert_eq!(seguido("agv_2", &blocos), "agv_2");
        assert_eq!(seguido("sumiu", &blocos), "agv_1");
        assert_eq!(seguido("", &blocos[..1]), "");
        assert!(restaura_vista("seguir") && !restaura_vista("recurso"));
        assert_eq!(aviso("orbita", "x"), "Câmera de órbita.");
        assert_eq!(aviso("recurso", "agv_1"), "Câmera do recurso: agv_1.");
        assert!(aviso("seguir", "").starts_with("Escolha na lista"));
    }
}
