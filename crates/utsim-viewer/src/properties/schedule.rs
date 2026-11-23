//! Pure helpers of the "Chegadas por período" table (no egui): the time unit guessed
//! from the cycle, clock-time text (hh:mm) and the period modes.
//!
//! The model has no time unit setting: times are plain numbers. A cycle of 1440 (or
//! 10080) is read as a day (or week) in minutes, 24 (168) in hours and 86400 (604800)
//! in seconds; only then the table shows clock times and rates per hour.

use crate::texto::{fx, tx, vx};
use utsim_core::Dist;
use utsim_model::{ArrivalPeriod, ArrivalSchedule, PeriodArrivals};

/// Unit of the model time, guessed from a schedule cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Seconds,
    Minutes,
    Hours,
}

impl Unit {
    /// Time units in one hour.
    pub fn per_hour(self) -> f64 {
        match self {
            Unit::Seconds => 3600.0,
            Unit::Minutes => 60.0,
            Unit::Hours => 1.0,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Unit::Seconds => tx("prop-schedule-segundos"),
            Unit::Minutes => tx("prop-schedule-minutos"),
            Unit::Hours => tx("prop-schedule-horas"),
        }
    }
}

/// The unit a cycle suggests (a day or a week), if any.
pub fn unit_of_cycle(cycle: f64) -> Option<Unit> {
    let is = |v: f64| (cycle - v).abs() < 1e-9;
    if is(1440.0) || is(10080.0) {
        Some(Unit::Minutes)
    } else if is(24.0) || is(168.0) {
        Some(Unit::Hours)
    } else if is(86400.0) || is(604800.0) {
        Some(Unit::Seconds)
    } else {
        None
    }
}

/// Clock text of an offset in a daily or weekly cycle: "07:30", or "dia 2 07:30" in a
/// week. `None` when the cycle suggests no unit.
pub fn clock(t: f64, cycle: f64) -> Option<String> {
    let unit = unit_of_cycle(cycle)?;
    let minutes = (t / unit.per_hour() * 60.0).round() as i64;
    let (day, rest) = (minutes.div_euclid(1440), minutes.rem_euclid(1440));
    let hm = format!("{:02}:{:02}", rest / 60, rest % 60);
    let total = (cycle / unit.per_hour() * 60.0).round() as i64;
    let weekly = total > 1440;
    Some(match (weekly, minutes == total) {
        (false, true) => "24:00".into(),
        (false, false) => hm,
        (true, true) => format!("dia {} 24:00", total / 1440),
        (true, false) => format!("dia {} {hm}", day + 1),
    })
}

/// Reads "hh:mm" (or "dia N hh:mm" in a weekly cycle) as an offset in the model unit of
/// a cycle that suggests one; plain numbers ("420", "7,5") are model time. Errors are
/// user-facing.
pub fn parse_time(text: &str, cycle: f64) -> Result<f64, String> {
    let t = text.trim();
    if let Some(rest) = t.strip_prefix("dia ") {
        let (day, hm) = rest
            .trim()
            .split_once(' ')
            .ok_or_else(|| fx("prop-schedule-use_dia_n_hh_mm", &[("t", vx(&t))]))?;
        let day: u32 = day
            .parse()
            .ok()
            .filter(|d| *d >= 1)
            .ok_or_else(|| fx("prop-schedule-dia_invalido_em_a_partir", &[("t", vx(&t))]))?;
        let unit =
            unit_of_cycle(cycle).ok_or_else(|| tx("prop-schedule-use_um_numero").to_string())?;
        return Ok(parse_time(hm, cycle)? + f64::from(day - 1) * 24.0 * unit.per_hour());
    }
    if let Some((h, m)) = t.split_once(':') {
        let unit = unit_of_cycle(cycle)
            .ok_or_else(|| tx("prop-schedule-hh_mm_so_vale_com").to_string())?;
        let h: u32 = h
            .trim()
            .parse()
            .map_err(|_| fx("prop-schedule-hora_invalida_em", &[("t", vx(&t))]))?;
        let m: u32 = m
            .trim()
            .parse()
            .map_err(|_| fx("prop-schedule-minuto_invalido_em", &[("t", vx(&t))]))?;
        if m >= 60 {
            return Err(fx("prop-schedule-minuto_invalido_em_0_a", &[("t", vx(&t))]));
        }
        return Ok(f64::from(h * 60 + m) / 60.0 * unit.per_hour());
    }
    t.replace(',', ".")
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
        .ok_or_else(|| fx("prop-schedule-nao_e_um_horario_hh", &[("t", vx(&t))]))
}

/// Modes of a period, in menu order, as label keys (`editor.ftl`).
pub const MODES: [&str; 4] = [
    "prop-modo-igual-ao-padrao",
    "prop-modo-distribuicao",
    "prop-modo-taxa",
    "prop-modo-sem-chegadas",
];

pub fn mode_index(a: &PeriodArrivals) -> usize {
    match a {
        PeriodArrivals::Default => 0,
        PeriodArrivals::Interarrival(_) => 1,
        PeriodArrivals::Rate(_) => 2,
        PeriodArrivals::Closed => 3,
    }
}

/// Switches a period to mode `index`, keeping the expected count when it can: a rate
/// starts at `1 / mean`, a distribution at `Exp` with the current mean.
pub fn switch_mode(current: &PeriodArrivals, index: usize, default: &Dist) -> PeriodArrivals {
    let mean = match current {
        PeriodArrivals::Default => default.mean(),
        PeriodArrivals::Interarrival(d) => d.mean(),
        PeriodArrivals::Rate(r) => 1.0 / r,
        PeriodArrivals::Closed => default.mean(),
    };
    let mean = if mean.is_finite() && mean > 0.0 {
        mean
    } else {
        1.0
    };
    match index {
        0 => PeriodArrivals::Default,
        1 => match current {
            PeriodArrivals::Interarrival(d) => PeriodArrivals::Interarrival(d.clone()),
            _ => PeriodArrivals::Interarrival(Dist::Exp { mean }),
        },
        2 => PeriodArrivals::Rate(1.0 / mean),
        _ => PeriodArrivals::Closed,
    }
}

/// A new period after the last one: same length as it (or an hour, or the whole cycle),
/// clipped to the cycle; `None` when the cycle is already full.
pub fn next_period(s: &ArrivalSchedule) -> Option<ArrivalPeriod> {
    let start = s.periods.last().map_or(0.0, |p| p.end);
    if start >= s.cycle {
        return None;
    }
    let len = match (s.periods.last(), unit_of_cycle(s.cycle)) {
        (Some(p), _) => p.end - p.start,
        (None, Some(u)) => u.per_hour(),
        (None, None) => s.cycle,
    };
    Some(ArrivalPeriod {
        start,
        end: (start + len).min(s.cycle),
        arrivals: PeriodArrivals::Default,
        labels: Default::default(),
    })
}

/// A starting table: one period over the whole cycle (a day of 1440 when the source's
/// mean suggests nothing else).
pub fn default_schedule() -> ArrivalSchedule {
    ArrivalSchedule {
        cycle: 1440.0,
        periods: vec![ArrivalPeriod {
            start: 0.0,
            end: 1440.0,
            arrivals: PeriodArrivals::Default,
            labels: Default::default(),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_times_follow_the_cycle() {
        assert_eq!(clock(420.0, 1440.0).as_deref(), Some("07:00"));
        assert_eq!(clock(1440.0, 1440.0).as_deref(), Some("24:00"));
        assert_eq!(clock(7.5, 24.0).as_deref(), Some("07:30"));
        assert_eq!(clock(3600.0 * 13.25, 86400.0).as_deref(), Some("13:15"));
        assert_eq!(
            clock(1440.0 + 60.0, 10080.0).as_deref(),
            Some("dia 2 01:00")
        );
        assert_eq!(clock(5.0, 100.0), None);
    }

    #[test]
    fn times_parse_as_clock_or_number() {
        assert_eq!(parse_time("07:30", 1440.0), Ok(450.0));
        assert_eq!(parse_time("24:00", 1440.0), Ok(1440.0));
        assert_eq!(parse_time("7:30", 24.0), Ok(7.5));
        assert_eq!(parse_time("420", 1440.0), Ok(420.0));
        assert_eq!(parse_time("7,5", 100.0), Ok(7.5));
        assert!(parse_time("07:30", 100.0).is_err());
        assert!(parse_time("07:75", 1440.0).is_err());
        assert!(parse_time("abc", 1440.0).is_err());
        for t in [0.0, 60.0, 450.0, 1439.0, 1440.0] {
            assert_eq!(parse_time(&clock(t, 1440.0).unwrap(), 1440.0), Ok(t));
        }
        assert_eq!(parse_time("dia 2 01:00", 10080.0), Ok(1500.0));
        assert_eq!(clock(10080.0, 10080.0).as_deref(), Some("dia 7 24:00"));
        for t in [0.0, 1500.0, 10079.0, 10080.0] {
            assert_eq!(parse_time(&clock(t, 10080.0).unwrap(), 10080.0), Ok(t));
        }
        assert!(parse_time("dia 0 01:00", 10080.0).is_err());
    }

    #[test]
    fn switching_modes_keeps_the_expected_count() {
        let default = Dist::Exp { mean: 4.0 };
        let rate = switch_mode(&PeriodArrivals::Default, 2, &default);
        assert_eq!(rate, PeriodArrivals::Rate(0.25));
        assert_eq!(
            switch_mode(&rate, 1, &default),
            PeriodArrivals::Interarrival(Dist::Exp { mean: 4.0 })
        );
        assert_eq!(switch_mode(&rate, 3, &default), PeriodArrivals::Closed);
        assert_eq!(switch_mode(&rate, 0, &default), PeriodArrivals::Default);
        for (i, _) in MODES.iter().enumerate() {
            assert_eq!(mode_index(&switch_mode(&rate, i, &default)), i);
        }
    }

    #[test]
    fn new_periods_follow_the_last_one_and_stop_at_the_cycle() {
        let mut s = ArrivalSchedule {
            cycle: 1440.0,
            periods: Vec::new(),
        };
        let p = next_period(&s).unwrap();
        assert_eq!((p.start, p.end), (0.0, 60.0));
        s.periods.push(ArrivalPeriod {
            start: 0.0,
            end: 1000.0,
            ..p
        });
        let p = next_period(&s).unwrap();
        assert_eq!((p.start, p.end), (1000.0, 1440.0));
        s.periods.push(p);
        assert!(next_period(&s).is_none());
        assert!(default_schedule().validate().is_empty());
    }
}
