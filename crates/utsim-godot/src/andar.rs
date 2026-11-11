//! Câmera livre em primeira pessoa do Simular ("Andar", tecla F): o usuário anda pela
//! fábrica na altura dos olhos ou voa por cima dela. WASD anda, o mouse olha (o cursor fica
//! preso), Shift corre, E e Q (ou Espaço e C) sobem e descem, V alterna entre andar e
//! voar, a roda muda a velocidade e Esc sai. Coordenadas do Godot (`y` para cima); o giro 0
//! olha para −Z, como a câmera de órbita.
//!
//! O estado ([`Andar`]) é puro: a cena principal só passa as teclas e o mouse e põe a
//! câmera no olho, olhando para [`Andar::alvo`]. O mesmo estado pode levar uma cabeça de
//! RV depois (o olho e o giro do corpo; a inclinação fica com o capacete).

use godot::prelude::*;

pub type V3 = [f64; 3];

/// Altura dos olhos andando, em metros.
pub const OLHO: f64 = 1.7;
/// Velocidade inicial, m/s (um passo rápido).
pub const VELOCIDADE: f64 = 2.0;
/// Faixa da velocidade que a roda escolhe.
pub const VELOCIDADE_MIN: f64 = 0.5;
pub const VELOCIDADE_MAX: f64 = 40.0;
/// Fator de um dente da roda.
pub const PASSO_RODA: f64 = 1.25;
/// Fator de Shift (correr).
pub const CORRER: f64 = 3.0;
/// Radianos por pixel do mouse.
pub const SENSIBILIDADE: f64 = 0.0025;
/// Inclinação máxima, para cima e para baixo (não vira de cabeça para baixo).
pub const INCLINACAO_MAX: f64 = 1.45;
/// Olho mais baixo voando.
pub const CHAO_VOO: f64 = 0.3;
/// Distância máxima do alvo da órbita ao começar a andar: de mais longe, o usuário começa
/// a essa distância dele, de frente para ele.
pub const DISTANCIA_INICIAL: f64 = 8.0;

/// O que o usuário aperta neste quadro: `frente` e `lado` de −1 a 1 (W/S e D/A), `subir` de
/// −1 a 1 (E/Q, Espaço/C) e Shift.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Comandos {
    pub frente: f64,
    pub lado: f64,
    pub subir: f64,
    pub correr: bool,
}

/// Onde está o olho e para onde ele olha.
#[derive(Clone, Debug, PartialEq)]
pub struct Andar {
    pub olho: V3,
    /// Giro em torno de `y` (0 olha para −Z, positivo vira para a esquerda).
    pub giro: f64,
    /// Positivo olha para cima.
    pub inclinacao: f64,
    /// Voando: anda para onde olha e sobe e desce; andando, fica na altura dos olhos.
    pub voo: bool,
    /// m/s sem Shift.
    pub velocidade: f64,
}

fn horizontal(v: V3) -> f64 {
    (v[0] * v[0] + v[2] * v[2]).sqrt()
}

impl Andar {
    /// Começa no lugar da câmera de órbita (`olho` olhando para `alvo`): de frente para o
    /// alvo, na altura dos olhos, no máximo a [`DISTANCIA_INICIAL`] dele. Vista de cima (o
    /// alvo logo abaixo do olho), mantém `giro` e começa 4 m antes do alvo.
    pub fn comecar(olho: V3, alvo: V3, giro: f64) -> Andar {
        let d = [alvo[0] - olho[0], 0.0, alvo[2] - olho[2]];
        let h = horizontal(d);
        let (giro, pos) = if h < 1e-6 {
            let f = [-giro.sin(), 0.0, -giro.cos()];
            (giro, [alvo[0] - f[0] * 4.0, alvo[2] - f[2] * 4.0])
        } else {
            let g = (-d[0]).atan2(-d[2]);
            let back = h.min(DISTANCIA_INICIAL);
            (g, [alvo[0] - d[0] / h * back, alvo[2] - d[2] / h * back])
        };
        Andar {
            olho: [pos[0], OLHO, pos[1]],
            giro,
            inclinacao: 0.0,
            voo: false,
            velocidade: VELOCIDADE,
        }
    }

    /// Para onde o corpo está virado, no chão.
    pub fn frente(&self) -> V3 {
        [-self.giro.sin(), 0.0, -self.giro.cos()]
    }

    /// A direita do corpo.
    pub fn direita(&self) -> V3 {
        [self.giro.cos(), 0.0, -self.giro.sin()]
    }

    /// Para onde o olho olha (unitário).
    pub fn olhar(&self) -> V3 {
        let (c, s) = (self.inclinacao.cos(), self.inclinacao.sin());
        let f = self.frente();
        [f[0] * c, s, f[2] * c]
    }

    /// Um ponto 10 m à frente do olho, para a câmera olhar.
    pub fn alvo(&self) -> V3 {
        let o = self.olhar();
        [
            self.olho[0] + o[0] * 10.0,
            self.olho[1] + o[1] * 10.0,
            self.olho[2] + o[2] * 10.0,
        ]
    }

    /// O mouse andou `dx`, `dy` pixels: para a direita vira à direita, para baixo olha
    /// para baixo.
    pub fn mouse(&mut self, dx: f64, dy: f64) {
        self.giro = (self.giro - dx * SENSIBILIDADE).rem_euclid(std::f64::consts::TAU);
        self.inclinacao =
            (self.inclinacao - dy * SENSIBILIDADE).clamp(-INCLINACAO_MAX, INCLINACAO_MAX);
    }

    /// Vira o olho para o ponto `p` (sem sair do lugar), dentro dos limites da inclinação.
    pub fn mirar(&mut self, p: V3) {
        let d = [
            p[0] - self.olho[0],
            p[1] - self.olho[1],
            p[2] - self.olho[2],
        ];
        let h = horizontal(d);
        if h > 1e-9 {
            self.giro = (-d[0]).atan2(-d[2]).rem_euclid(std::f64::consts::TAU);
        }
        self.inclinacao = d[1].atan2(h).clamp(-INCLINACAO_MAX, INCLINACAO_MAX);
    }

    /// `dentes` da roda (positivo: para cima, mais rápido).
    pub fn roda(&mut self, dentes: i32) {
        self.velocidade =
            (self.velocidade * PASSO_RODA.powi(dentes)).clamp(VELOCIDADE_MIN, VELOCIDADE_MAX);
    }

    /// V: voar ou voltar ao chão (na altura dos olhos).
    pub fn alternar_voo(&mut self) {
        self.voo = !self.voo;
        if !self.voo {
            self.olho[1] = OLHO;
        }
    }

    /// Anda `dt` segundos com os `c` apertados. Andando, a frente é a do corpo (olhar para
    /// baixo não afunda no chão) e subir passa a voar; voando, vai para onde olha.
    pub fn passo(&mut self, c: &Comandos, dt: f64) {
        if c.subir.abs() > 1e-9 && !self.voo {
            self.voo = true;
        }
        let (mut f, mut l) = (c.frente.clamp(-1.0, 1.0), c.lado.clamp(-1.0, 1.0));
        let n = (f * f + l * l).sqrt();
        if n > 1.0 {
            f /= n;
            l /= n;
        }
        let ir = if self.voo {
            self.olhar()
        } else {
            self.frente()
        };
        let dir = self.direita();
        let up = if self.voo {
            c.subir.clamp(-1.0, 1.0)
        } else {
            0.0
        };
        let v = self.velocidade * if c.correr { CORRER } else { 1.0 } * dt.max(0.0);
        for k in 0..3 {
            self.olho[k] += (ir[k] * f + dir[k] * l) * v;
        }
        self.olho[1] += up * v;
        if self.voo {
            self.olho[1] = self.olho[1].max(CHAO_VOO);
        } else {
            self.olho[1] = OLHO;
        }
    }

    /// A dica que a tela mostra enquanto anda.
    pub fn dica(&self) -> String {
        format!(
            "{}: WASD anda · mouse olha · Shift corre · E/Q sobe e desce · V {} · roda: {} m/s · clique abre o painel · Esc sai",
            if self.voo { "Voando" } else { "Andando" },
            if self.voo { "volta ao chão" } else { "voa" },
            fmt_vel(self.velocidade)
        )
    }
}

fn fmt_vel(v: f64) -> String {
    let t = if v < 10.0 {
        format!("{v:.1}")
    } else {
        format!("{v:.0}")
    };
    t.replace('.', ",")
}

/// A câmera livre para o Godot: um `RefCounted` com o estado de [`Andar`].
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct UtsimAndar {
    estado: Option<Andar>,
    base: Base<RefCounted>,
}

fn v3(p: V3) -> Vector3 {
    Vector3::new(p[0] as f32, p[1] as f32, p[2] as f32)
}

fn of(p: Vector3) -> V3 {
    [f64::from(p.x), f64::from(p.y), f64::from(p.z)]
}

#[godot_api]
impl UtsimAndar {
    /// Começa a andar a partir da câmera de órbita (o olho, o alvo e o giro dela).
    #[func]
    fn begin(&mut self, eye: Vector3, target: Vector3, yaw: f64) {
        self.estado = Some(Andar::comecar(of(eye), of(target), yaw));
    }

    #[func]
    fn active(&self) -> bool {
        self.estado.is_some()
    }

    #[func]
    fn end(&mut self) {
        self.estado = None;
    }

    #[func]
    fn look(&mut self, dx: f64, dy: f64) {
        if let Some(a) = &mut self.estado {
            a.mouse(dx, dy);
        }
    }

    /// Olha para `point` (os roteiros miram um objeto antes do clique na mira).
    #[func]
    fn aim_at(&mut self, point: Vector3) {
        if let Some(a) = &mut self.estado {
            a.mirar(of(point));
        }
    }

    #[func]
    fn wheel(&mut self, notches: i32) {
        if let Some(a) = &mut self.estado {
            a.roda(notches);
        }
    }

    #[func]
    fn toggle_flight(&mut self) {
        if let Some(a) = &mut self.estado {
            a.alternar_voo();
        }
    }

    /// Anda `dt` segundos: `forward` e `side` de −1 a 1, `up` de −1 a 1, `run` Shift.
    #[func]
    fn step(&mut self, forward: f64, side: f64, up: f64, run: bool, dt: f64) {
        if let Some(a) = &mut self.estado {
            let c = Comandos {
                frente: forward,
                lado: side,
                subir: up,
                correr: run,
            };
            a.passo(&c, dt);
        }
    }

    #[func]
    fn eye(&self) -> Vector3 {
        self.estado.as_ref().map_or(Vector3::ZERO, |a| v3(a.olho))
    }

    #[func]
    fn aim(&self) -> Vector3 {
        self.estado
            .as_ref()
            .map_or(Vector3::new(0.0, 0.0, -1.0), |a| v3(a.alvo()))
    }

    #[func]
    fn yaw(&self) -> f64 {
        self.estado.as_ref().map_or(0.0, |a| a.giro)
    }

    #[func]
    fn flying(&self) -> bool {
        self.estado.as_ref().is_some_and(|a| a.voo)
    }

    #[func]
    fn speed(&self) -> f64 {
        self.estado.as_ref().map_or(0.0, |a| a.velocidade)
    }

    #[func]
    fn hint(&self) -> GString {
        GString::from(
            self.estado
                .as_ref()
                .map_or(String::new(), Andar::dica)
                .as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: V3, b: V3) -> bool {
        (0..3).all(|k| (a[k] - b[k]).abs() < 1e-9)
    }

    #[test]
    fn it_starts_at_eye_height_facing_the_orbit_target() {
        // The orbit eye 30 m back along +Z and 20 m up: the walk starts 8 m in front of the
        // target, looking at it (−Z), level.
        let a = Andar::comecar([0.0, 20.0, 30.0], [0.0, 0.0, 0.0], 0.3);
        assert!(close(a.olho, [0.0, OLHO, 8.0]), "{:?}", a.olho);
        assert!(a.giro.abs() < 1e-9 && a.inclinacao == 0.0 && !a.voo);
        assert!(close(a.frente(), [0.0, 0.0, -1.0]));
        // Closer than 8 m: right where the camera is, on the floor, facing −X (its right
        // is −Z).
        let a = Andar::comecar([3.0, 5.0, 0.0], [0.0, 0.0, 0.0], 0.0);
        assert!(close(a.olho, [3.0, OLHO, 0.0]), "{:?}", a.olho);
        assert!(close(a.frente(), [-1.0, 0.0, 0.0]), "{:?}", a.frente());
        assert!(close(a.direita(), [0.0, 0.0, -1.0]), "{:?}", a.direita());
        // From straight above: the orbit's yaw, 4 m before the target.
        let a = Andar::comecar([2.0, 30.0, 2.0], [2.0, 0.0, 2.0], 0.5);
        assert!((a.giro - 0.5).abs() < 1e-9);
        assert!((horizontal([a.olho[0] - 2.0, 0.0, a.olho[2] - 2.0]) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn wasd_walks_on_the_floor_whatever_the_eye_looks_at() {
        let mut a = Andar::comecar([0.0, 20.0, 30.0], [0.0, 0.0, 0.0], 0.0);
        // Looking down does not sink: forward is the body's.
        a.mouse(0.0, 200.0);
        assert!(a.inclinacao < -0.4);
        let w = Comandos {
            frente: 1.0,
            ..Default::default()
        };
        a.passo(&w, 1.0);
        assert!(close(a.olho, [0.0, OLHO, 6.0]), "{:?}", a.olho);
        // Shift runs three times as fast; D goes right (+X here).
        let d = Comandos {
            lado: 1.0,
            correr: true,
            ..Default::default()
        };
        a.passo(&d, 0.5);
        assert!(close(a.olho, [3.0, OLHO, 6.0]), "{:?}", a.olho);
        // W and D together are not faster than one of them.
        let before = a.olho;
        let wd = Comandos {
            frente: 1.0,
            lado: 1.0,
            ..Default::default()
        };
        a.passo(&wd, 1.0);
        let moved = horizontal([a.olho[0] - before[0], 0.0, a.olho[2] - before[2]]);
        assert!((moved - VELOCIDADE).abs() < 1e-9, "{moved}");
    }

    #[test]
    fn the_mouse_turns_and_tilts_within_limits() {
        let mut a = Andar::comecar([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], 0.0);
        // Right turns right: from −Z to +X after a quarter turn.
        a.mouse(std::f64::consts::FRAC_PI_2 / SENSIBILIDADE, 0.0);
        assert!(close(a.frente(), [1.0, 0.0, 0.0]), "{:?}", a.frente());
        // Up looks up, never past the limit.
        a.mouse(0.0, -1e6);
        assert_eq!(a.inclinacao, INCLINACAO_MAX);
        assert!(a.olhar()[1] > 0.99);
        a.mouse(0.0, 1e6);
        assert_eq!(a.inclinacao, -INCLINACAO_MAX);
        // Aiming at a point turns the eye to it, wherever it was looking.
        let mut b = a.clone();
        let p = [b.olho[0] + 3.0, b.olho[1] - 1.0, b.olho[2] + 4.0];
        b.mirar(p);
        let o = b.olhar();
        let d = [3.0 / 26f64.sqrt(), -1.0 / 26f64.sqrt(), 4.0 / 26f64.sqrt()];
        assert!(close(o, d), "{o:?} {d:?}");
        assert_eq!(b.olho, a.olho);
        // The aim is 10 m along the sight line.
        let o = a.olhar();
        let t = a.alvo();
        let want = [
            a.olho[0] + o[0] * 10.0,
            a.olho[1] + o[1] * 10.0,
            a.olho[2] + o[2] * 10.0,
        ];
        assert!(close(t, want));
    }

    #[test]
    fn going_up_flies_and_v_comes_back_to_the_floor() {
        let mut a = Andar::comecar([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], 0.0);
        let up = Comandos {
            subir: 1.0,
            ..Default::default()
        };
        a.passo(&up, 2.0);
        assert!(a.voo);
        assert!((a.olho[1] - (OLHO + 4.0)).abs() < 1e-9, "{:?}", a.olho);
        // Flying, forward follows the eye: looking up climbs.
        a.mouse(0.0, -0.5 / SENSIBILIDADE);
        let y = a.olho[1];
        let w = Comandos {
            frente: 1.0,
            ..Default::default()
        };
        a.passo(&w, 1.0);
        assert!((a.olho[1] - y - VELOCIDADE * 0.5f64.sin()).abs() < 1e-9);
        // Never under the floor.
        let down = Comandos {
            subir: -1.0,
            ..Default::default()
        };
        a.passo(&down, 100.0);
        assert_eq!(a.olho[1], CHAO_VOO);
        // V: back on the floor at eye height; again: flying from there.
        a.alternar_voo();
        assert!(!a.voo && a.olho[1] == OLHO);
        a.alternar_voo();
        assert!(a.voo && a.olho[1] == OLHO);
    }

    #[test]
    fn the_wheel_sets_the_speed_within_its_range() {
        let mut a = Andar::comecar([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], 0.0);
        a.roda(2);
        assert!((a.velocidade - VELOCIDADE * 1.5625).abs() < 1e-9);
        a.roda(100);
        assert_eq!(a.velocidade, VELOCIDADE_MAX);
        a.roda(-100);
        assert_eq!(a.velocidade, VELOCIDADE_MIN);
        assert!(a.dica().contains("0,5 m/s"), "{}", a.dica());
        assert!(a.dica().starts_with("Andando") && a.dica().contains("Esc sai"));
        a.alternar_voo();
        assert!(a.dica().starts_with("Voando") && a.dica().contains("V volta ao chão"));
    }
}
