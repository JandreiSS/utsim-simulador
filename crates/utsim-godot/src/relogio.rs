//! The playback clock of Simular (passo 7, parte 3): the time shown, the speed, playing or
//! paused, and the end of the replication loaded. The `UtsimUi` keeps it, the playback bar
//! reads it, and the bar's own actions (play, speed, time, events) change it without going
//! through the main scene; the main scene only applies the time to the 3D scene.

/// The clock of the playback.
#[derive(Clone, Debug, PartialEq)]
pub struct Relogio {
    /// Simulated time shown, in seconds.
    pub t: f64,
    /// Simulated seconds per real second.
    pub speed: f64,
    pub playing: bool,
    /// The end of the replication loaded, or `None` without one.
    pub end: Option<f64>,
}

impl Default for Relogio {
    fn default() -> Self {
        Relogio {
            t: 0.0,
            speed: 1.0,
            playing: false,
            end: None,
        }
    }
}

/// What a bar action did to the clock.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Efeito {
    /// Not a clock action: the main scene handles it.
    Nenhum,
    /// The clock changed but the time did not (play, pause, speed).
    Estado,
    /// The time jumped: the scene shows the new time without crossfade.
    Salto,
}

impl Relogio {
    /// A new replication to play (`None`: none), paused at 0.
    pub fn carregar(&mut self, end: Option<f64>) {
        self.end = end.map(|e| if e.is_finite() { e.max(0.0) } else { 0.0 });
        self.playing = false;
        self.t = 0.0;
    }

    /// Plays or pauses; it only plays with a replication loaded and in Simular.
    pub fn tocar(&mut self, on: bool, simulando: bool) {
        self.playing = on && simulando && self.end.is_some();
    }

    /// The speed; a value that is not a positive number is ignored.
    pub fn velocidade(&mut self, s: f64) {
        if s.is_finite() && s > 0.0 {
            self.speed = s;
        }
    }

    /// Advances a frame of `dt` real seconds while playing; stops at the end. True when
    /// the time moved (the scene must show it).
    pub fn avancar(&mut self, dt: f64) -> bool {
        let Some(end) = self.end else {
            return false;
        };
        if !self.playing || !dt.is_finite() || dt <= 0.0 {
            return false;
        }
        self.t += dt * self.speed;
        if self.t >= end {
            self.t = end;
            self.playing = false;
        }
        true
    }

    /// Jumps to `time` (clamped to the replication). False without a replication.
    pub fn ir(&mut self, time: f64) -> bool {
        let Some(end) = self.end else {
            return false;
        };
        self.t = if time.is_finite() {
            time.clamp(0.0, end)
        } else {
            0.0
        };
        true
    }

    /// Pauses and jumps to the next or previous event, given as `alvo` (negative: none).
    /// True when the time jumped.
    pub fn ao_evento(&mut self, alvo: f64) -> bool {
        if self.end.is_none() {
            return false;
        }
        self.playing = false;
        alvo >= 0.0 && self.ir(alvo)
    }

    /// Applies an action of the playback bar (`reproduzir`, `velocidade:<s>`,
    /// `tempo:<t>`; `evento:<±1>` is resolved by the caller, which knows the events).
    pub fn acao(&mut self, what: &str, simulando: bool) -> Efeito {
        let (verb, arg) = what.split_once(':').unwrap_or((what, ""));
        match verb {
            "reproduzir" => {
                self.tocar(!self.playing, simulando);
                Efeito::Estado
            }
            "velocidade" => {
                if let Ok(s) = arg.parse::<f64>() {
                    self.velocidade(s);
                }
                Efeito::Estado
            }
            "tempo" => match arg.parse::<f64>() {
                Ok(x) if self.ir(x) => Efeito::Salto,
                _ => Efeito::Estado,
            },
            _ => Efeito::Nenhum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays_only_with_a_replication_and_stops_at_its_end() {
        let mut r = Relogio::default();
        r.tocar(true, true);
        assert!(!r.playing, "nothing loaded");
        assert!(!r.avancar(0.5));
        r.carregar(Some(10.0));
        r.tocar(true, false);
        assert!(!r.playing, "not in Simular");
        r.tocar(true, true);
        r.velocidade(4.0);
        assert!(r.avancar(1.0));
        assert_eq!(r.t, 4.0);
        assert!(r.avancar(2.0));
        assert_eq!((r.t, r.playing), (10.0, false), "stops at the end");
        assert!(!r.avancar(1.0));
        // A new replication starts paused at 0.
        r.tocar(true, true);
        r.carregar(Some(5.0));
        assert_eq!((r.t, r.playing), (0.0, false));
    }

    #[test]
    fn the_bar_actions_change_the_clock() {
        let mut r = Relogio::default();
        assert_eq!(r.acao("tempo:3", true), Efeito::Estado, "no replication");
        r.carregar(Some(100.0));
        assert_eq!(r.acao("reproduzir", true), Efeito::Estado);
        assert!(r.playing);
        assert_eq!(r.acao("reproduzir", true), Efeito::Estado);
        assert!(!r.playing);
        assert_eq!(r.acao("velocidade:8", true), Efeito::Estado);
        assert_eq!(r.speed, 8.0);
        r.acao("velocidade:-1", true);
        r.acao("velocidade:x", true);
        assert_eq!(r.speed, 8.0, "bad speeds are ignored");
        assert_eq!(r.acao("tempo:250", true), Efeito::Salto);
        assert_eq!(r.t, 100.0, "clamped to the end");
        assert_eq!(r.acao("tempo:-3", true), Efeito::Salto);
        assert_eq!(r.t, 0.0);
        assert_eq!(r.acao("modo:editar", true), Efeito::Nenhum);
    }

    #[test]
    fn stepping_to_an_event_pauses_and_jumps() {
        let mut r = Relogio::default();
        assert!(!r.ao_evento(3.0));
        r.carregar(Some(50.0));
        r.tocar(true, true);
        assert!(r.ao_evento(12.5));
        assert_eq!((r.t, r.playing), (12.5, false));
        r.tocar(true, true);
        assert!(!r.ao_evento(-1.0), "no event left");
        assert_eq!((r.t, r.playing), (12.5, false));
    }
}
