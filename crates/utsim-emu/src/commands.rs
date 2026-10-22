//! External commands and the command log that makes an emulation run reproducible.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Name of the global speed variable.
pub const VAR_SPEED: &str = "Velocidade";
/// Suffix of the per-block stop variable (`<block>.Parar`).
pub const VAR_STOP_SUFFIX: &str = ".Parar";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    Num(f64),
}

/// A write coming from the outside world (an OPC UA client, a replay file).
#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    pub variable: String,
    pub value: Value,
}

/// One command applied at a simulated time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoggedCommand {
    pub time: f64,
    pub variable: String,
    pub value: Value,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CommandLog {
    pub commands: Vec<LoggedCommand>,
}

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("{0}: {1}")]
    Io(String, std::io::Error),
    #[error("{0}: {1}")]
    Parse(String, String),
}

impl CommandLog {
    pub fn record(&mut self, time: f64, input: &Input) {
        self.commands.push(LoggedCommand {
            time,
            variable: input.variable.clone(),
            value: input.value.clone(),
        });
    }

    pub fn save(&self, path: &Path) -> Result<(), LogError> {
        let text = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e| LogError::Parse(path.display().to_string(), e.to_string()))?;
        std::fs::write(path, text).map_err(|e| LogError::Io(path.display().to_string(), e))
    }

    pub fn load(path: &Path) -> Result<CommandLog, LogError> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| LogError::Io(path.display().to_string(), e))?;
        Self::parse(&text).map_err(|e| LogError::Parse(path.display().to_string(), e))
    }

    pub fn parse(text: &str) -> Result<CommandLog, String> {
        let mut log: CommandLog = ron::from_str(text).map_err(|e| e.to_string())?;
        // Replay applies commands in time order; the sort is stable so commands with the
        // same time keep their order.
        log.commands.sort_by(|a, b| a.time.total_cmp(&b.time));
        Ok(log)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_round_trips_through_ron() {
        let mut log = CommandLog::default();
        log.record(
            1.5,
            &Input {
                variable: "maquina.Parar".into(),
                value: Value::Bool(true),
            },
        );
        log.record(
            3.0,
            &Input {
                variable: VAR_SPEED.into(),
                value: Value::Num(2.0),
            },
        );
        let text = ron::ser::to_string(&log).unwrap();
        assert_eq!(CommandLog::parse(&text).unwrap(), log);
    }
}
