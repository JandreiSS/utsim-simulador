//! Wall-clock pacing: keeps simulated time close to wall time x speed factor.

use std::cell::Cell;
use std::time::{Duration, Instant};

/// Source of wall time. A fake one makes the pacing logic testable.
pub trait Clock {
    /// Time elapsed since the clock started.
    fn now(&self) -> Duration;
    fn sleep(&self, d: Duration);
}

/// The real clock.
pub struct SystemClock {
    start: Instant,
}

impl SystemClock {
    pub fn new() -> Self {
        SystemClock {
            start: Instant::now(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for SystemClock {
    fn now(&self) -> Duration {
        self.start.elapsed()
    }

    fn sleep(&self, d: Duration) {
        std::thread::sleep(d);
    }
}

/// A clock that only moves when somebody sleeps (or `advance` is called).
#[derive(Default)]
pub struct FakeClock {
    t: Cell<Duration>,
}

impl FakeClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance(&self, d: Duration) {
        self.t.set(self.t.get() + d);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Duration {
        self.t.get()
    }

    fn sleep(&self, d: Duration) {
        self.advance(d);
    }
}

/// Maps wall time to the simulated time that should have been reached.
///
/// One simulated time unit corresponds to one second of wall time at speed 1.0.
#[derive(Clone, Debug)]
pub struct Pacer {
    speed: f64,
    wall_anchor: Duration,
    sim_anchor: f64,
    /// Longest lag (in wall time) tolerated before the pacer gives up catching up.
    max_lag: Duration,
}

impl Pacer {
    pub fn new(speed: f64, wall_now: Duration, sim_now: f64) -> Self {
        Pacer {
            speed: sanitize(speed),
            wall_anchor: wall_now,
            sim_anchor: sim_now,
            max_lag: Duration::from_secs(1),
        }
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    /// Simulated time that should have been reached at wall time `wall_now`.
    pub fn target(&self, wall_now: Duration) -> f64 {
        let elapsed = wall_now.saturating_sub(self.wall_anchor);
        self.sim_anchor + elapsed.as_secs_f64() * self.speed
    }

    /// Changes the speed without a jump in the target.
    pub fn set_speed(&mut self, speed: f64, wall_now: Duration, sim_now: f64) {
        self.wall_anchor = wall_now;
        self.sim_anchor = sim_now;
        self.speed = sanitize(speed);
    }

    /// Called after each tick with the time the simulation actually reached. When the
    /// simulation is too far behind the wall clock (heavy model), the lag is dropped
    /// instead of being caught up in one long burst.
    pub fn resync_if_late(&mut self, wall_now: Duration, sim_now: f64) {
        let lag_sim = self.target(wall_now) - sim_now;
        if lag_sim > self.max_lag.as_secs_f64() * self.speed {
            self.wall_anchor = wall_now;
            self.sim_anchor = sim_now;
        }
    }
}

fn sanitize(speed: f64) -> f64 {
    if speed.is_finite() && speed > 0.0 {
        speed
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    #[test]
    fn target_follows_wall_time_times_speed() {
        let p = Pacer::new(10.0, ms(0), 0.0);
        assert_eq!(p.target(ms(0)), 0.0);
        assert!((p.target(ms(500)) - 5.0).abs() < 1e-12);
        let p = Pacer::new(1.0, ms(200), 3.0);
        assert!((p.target(ms(1200)) - 4.0).abs() < 1e-12);
    }

    #[test]
    fn speed_change_does_not_jump() {
        let mut p = Pacer::new(1.0, ms(0), 0.0);
        let before = p.target(ms(1000));
        p.set_speed(4.0, ms(1000), before);
        assert!((p.target(ms(1000)) - before).abs() < 1e-12);
        assert!((p.target(ms(1500)) - (before + 2.0)).abs() < 1e-12);
    }

    #[test]
    fn invalid_speed_falls_back_to_real_time() {
        assert_eq!(Pacer::new(0.0, ms(0), 0.0).speed(), 1.0);
        assert_eq!(Pacer::new(f64::NAN, ms(0), 0.0).speed(), 1.0);
        assert_eq!(Pacer::new(-3.0, ms(0), 0.0).speed(), 1.0);
    }

    #[test]
    fn large_lag_is_dropped() {
        let mut p = Pacer::new(2.0, ms(0), 0.0);
        // 10 s of wall time passed but the simulation only reached 1.0.
        p.resync_if_late(ms(10_000), 1.0);
        assert!((p.target(ms(10_000)) - 1.0).abs() < 1e-12);
        // A small lag is kept.
        let mut p = Pacer::new(2.0, ms(0), 0.0);
        p.resync_if_late(ms(500), 0.5);
        assert!((p.target(ms(500)) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn fake_clock_advances_on_sleep() {
        let c = FakeClock::new();
        c.sleep(ms(50));
        c.sleep(ms(50));
        assert_eq!(c.now(), ms(100));
    }
}
