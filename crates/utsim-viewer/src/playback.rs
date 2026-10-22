//! Shared viewer state (replay data, clock, selection) and the clock systems.

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use utsim_model::replay::Replay;
use utsim_viewer::layout;
use utsim_viewer::timeline::{Timeline, TransitSetting};

/// Read-only replay data. Nothing here is ever fed back into a simulation.
#[derive(Resource)]
pub struct Data {
    pub replay: Replay,
    pub timeline: Timeline,
    /// Bevy-space base position of each block.
    pub positions: Vec<[f32; 3]>,
    /// End of the animation, in simulation time.
    pub end: f64,
}

impl Data {
    /// Indexes a replay and places its blocks (model positions or automatic layout).
    pub fn from_replay(replay: Replay, setting: TransitSetting) -> Self {
        let timeline = Timeline::build(&replay, setting);
        let raw: Vec<[f64; 3]> = replay.blocks.iter().map(|b| b.position).collect();
        let positions = layout::place(&raw, &replay.connections);
        let end = if replay.stop_time.is_finite() {
            replay.stop_time
        } else {
            timeline.last_event_time()
        }
        .max(timeline.last_event_time());
        Data {
            replay,
            timeline,
            positions,
            end,
        }
    }
}

/// Simulation-time units that pass per real second at speed x1.
pub const SPEEDS: [f64; 13] = [
    0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1000.0,
];

#[derive(Resource)]
pub struct Playback {
    pub t: f64,
    pub playing: bool,
    pub speed_idx: usize,
}

impl Playback {
    pub fn new(end: f64) -> Self {
        // Default: the whole recording lasts about a minute.
        let wanted = end / 60.0;
        let speed_idx = SPEEDS
            .iter()
            .position(|&s| s >= wanted)
            .unwrap_or(SPEEDS.len() - 1);
        Playback {
            t: 0.0,
            playing: false,
            speed_idx,
        }
    }

    pub fn speed(&self) -> f64 {
        SPEEDS[self.speed_idx]
    }

    pub fn faster(&mut self) {
        self.speed_idx = (self.speed_idx + 1).min(SPEEDS.len() - 1);
    }

    pub fn slower(&mut self) {
        self.speed_idx = self.speed_idx.saturating_sub(1);
    }
}

#[derive(Resource, Default)]
pub struct Selection(pub Option<usize>);

pub fn advance_clock(time: Res<Time>, data: Res<Data>, mut pb: ResMut<Playback>) {
    if !pb.playing {
        return;
    }
    pb.t += time.delta_secs_f64() * pb.speed();
    if pb.t >= data.end {
        pb.t = data.end;
        pb.playing = false;
    }
}

pub fn keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    data: Res<Data>,
    mut pb: ResMut<Playback>,
    mut contexts: EguiContexts,
) {
    if contexts
        .ctx_mut()
        .is_ok_and(|ctx| ctx.egui_wants_keyboard_input())
    {
        return;
    }
    if keys.just_pressed(KeyCode::Space) {
        if pb.t >= data.end {
            pb.t = 0.0;
        }
        pb.playing = !pb.playing;
    }
    if keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::NumpadAdd) {
        pb.faster();
    }
    if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::NumpadSubtract) {
        pb.slower();
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        pb.playing = false;
        let t = pb.t;
        pb.t = data
            .timeline
            .next_event_time(t)
            .unwrap_or(data.end)
            .min(data.end);
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        pb.playing = false;
        let t = pb.t;
        pb.t = data.timeline.prev_event_time(t).unwrap_or(0.0);
    }
    if keys.just_pressed(KeyCode::Home) {
        pb.t = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_speed_covers_the_recording_in_about_a_minute() {
        assert_eq!(Playback::new(60.0).speed(), 1.0);
        assert_eq!(Playback::new(100_000.0).speed(), 2000.0_f64.min(1000.0));
        assert_eq!(Playback::new(1.0).speed(), 0.1);
    }

    #[test]
    fn speed_stays_in_range() {
        let mut pb = Playback::new(60.0);
        for _ in 0..50 {
            pb.faster();
        }
        assert_eq!(pb.speed(), 1000.0);
        for _ in 0..50 {
            pb.slower();
        }
        assert_eq!(pb.speed(), 0.1);
    }
}
