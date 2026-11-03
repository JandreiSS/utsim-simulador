//! Observations: what a connector reads from the plant, before and after it gets its
//! instant on the twin's clock.

use std::time::Instant;

use serde::{Deserialize, Serialize};

/// A value read from a sensor: a number, a flag or a text (a state code, a JSON payload).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    Num(f64),
    Text(String),
}

impl Value {
    /// The value as a number: numbers as they are, flags as 1 and 0, texts parsed (with a
    /// decimal point or a decimal comma).
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Num(x) if x.is_finite() => Some(*x),
            Value::Num(_) => None,
            Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::Text(s) => {
                let s = s.trim();
                s.parse::<f64>()
                    .ok()
                    .or_else(|| s.replace(',', ".").parse::<f64>().ok())
                    .filter(|x| x.is_finite())
            }
        }
    }

    /// The value as the key of a state table: integers without decimals (`3`, not `3.0`),
    /// flags as `true`/`false`, texts trimmed.
    pub fn as_key(&self) -> String {
        match self {
            Value::Num(x) if x.fract() == 0.0 && x.abs() < 1e15 => format!("{}", *x as i64),
            Value::Num(x) => format!("{x}"),
            Value::Bool(b) => b.to_string(),
            Value::Text(s) => s.trim().to_string(),
        }
    }
}

/// Quality of a reading, as the source reports it (OPC UA status, Sparkplug death).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quality {
    #[default]
    Good,
    Uncertain,
    Bad,
}

/// A reading as a connector delivers it: no twin time yet, only the local instant it
/// arrived.
#[derive(Clone, Debug)]
pub struct RawObservation {
    pub source: String,
    pub tag: String,
    pub value: Value,
    pub quality: Quality,
    /// Order of arrival within the connector.
    pub seq: u64,
    /// Local instant of arrival.
    pub at: Instant,
    /// Timestamp given by the source (OPC UA SourceTimestamp, Sparkplug `timestamp`),
    /// microseconds since 1970.
    pub source_us: Option<u64>,
}

/// A reading with its instant on the twin's clock: what is recorded and what the shadow
/// processes. Replaying the same observations in the same order gives the same results.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    /// Twin time (model time units since the twin started).
    pub t: f64,
    /// Wall clock, milliseconds since 1970 (information only; never used in calculations).
    #[serde(rename = "ts")]
    pub wall_ms: u64,
    #[serde(rename = "fonte")]
    pub source: String,
    pub seq: u64,
    pub tag: String,
    #[serde(rename = "valor")]
    pub value: Value,
    #[serde(rename = "qualidade", default)]
    pub quality: Quality,
    /// Index of the mapping rule that matched the tag (`None`: not mapped). Stored so
    /// that a replay of an anonymised recording finds the rule without the original tag.
    #[serde(rename = "regra", default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<usize>,
}

/// Wall clock in milliseconds since 1970.
pub fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_convert() {
        assert_eq!(Value::Text(" 2,5 ".into()).as_f64(), Some(2.5));
        assert_eq!(Value::Bool(true).as_f64(), Some(1.0));
        assert_eq!(Value::Num(f64::NAN).as_f64(), None);
        assert_eq!(Value::Num(3.0).as_key(), "3");
        assert_eq!(Value::Num(3.5).as_key(), "3.5");
        assert_eq!(Value::Text(" down ".into()).as_key(), "down");
    }

    #[test]
    fn observation_round_trips_through_json() {
        let o = Observation {
            t: 0.1 + 0.2,
            wall_ms: 5,
            source: "clp".into(),
            seq: 7,
            tag: "a/b".into(),
            value: Value::Num(1.0 / 3.0),
            quality: Quality::Uncertain,
            rule: Some(2),
        };
        let text = serde_json::to_string(&o).unwrap();
        assert_eq!(serde_json::from_str::<Observation>(&text).unwrap(), o);
    }
}
