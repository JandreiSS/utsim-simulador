//! Recording of every observation received, for a replay without network.
//!
//! `--gravar pasta` creates `pasta/sessao-AAAAMMDD-HHMMSS/` with:
//! - `sessao.json`: the `.gemeo.ron` text, the model path and hash, the start;
//! - `observacoes-AAAAMMDD.jsonl`: one JSON object per line, one file per day (UTC):
//!   `{"tipo":"obs","t":…,"ts":…,"fonte":…,"seq":…,"tag":…,"valor":…,"qualidade":…,"regra":…}`
//!   `{"tipo":"previsao","t":…,"pedido":…}` where a forecast was taken (with `--previsao`)
//!   and, on a clean end, `{"tipo":"fim","t":…}`.
//!
//! People: with `anonymize_people` (the default), the tags of rules marked `person` and
//! the tags containing one of `record.people` become stable nicknames (`pessoa-…`), and
//! the texts inside their values too (numbers stay: the coordinates of a position). The
//! nicknames come from a keyed hash whose key is drawn per recording and never written,
//! so they cannot be reversed from the files. The twin processes the anonymised
//! observation, live and in the replay alike.
//!
//! Retention: day files older than `keep_days` are deleted when a recording starts and
//! once a day while it runs; session folders left empty are removed.

use std::collections::hash_map::RandomState;
use std::fs::{self, File, OpenOptions};
use std::hash::{BuildHasher, Hasher};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::obs::{Observation, Value};

pub const HEADER: &str = "sessao.json";
const PREFIX: &str = "observacoes-";
const SESSION_PREFIX: &str = "sessao-";
const DAY_MS: u64 = 86_400_000;

/// Header of a recording.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionHeader {
    pub formato: u32,
    /// Wall clock at the start, milliseconds since 1970.
    pub inicio_ms: u64,
    /// Text of the `.gemeo.ron` file.
    pub config: String,
    /// Model file as given on the command line.
    pub modelo: String,
    pub hash_modelo: String,
    pub anonimizado: bool,
    pub retencao_dias: u32,
}

/// One line of a recording.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tipo")]
pub enum Line {
    #[serde(rename = "obs")]
    Obs(Observation),
    #[serde(rename = "fim")]
    End { t: f64 },
    /// A forecast was taken from the observed state at twin time `t` (scheduled, or on
    /// request): the replay takes it at the same point of the sequence.
    #[serde(rename = "previsao")]
    Forecast {
        t: f64,
        #[serde(default, rename = "pedido")]
        on_request: bool,
    },
}

/// Civil date `(year, month, day)` of a day count since 1970-01-01 (H. Hinnant).
fn civil(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

/// `AAAAMMDD` of a wall time.
pub fn day_stamp(ms: u64) -> String {
    let (y, m, d) = civil((ms / DAY_MS) as i64);
    format!("{y:04}{m:02}{d:02}")
}

fn time_stamp(ms: u64) -> String {
    let s = (ms / 1000) % 86_400;
    format!(
        "{}-{:02}{:02}{:02}",
        day_stamp(ms),
        s / 3600,
        (s / 60) % 60,
        s % 60
    )
}

/// Days since 1970 of an `AAAAMMDD` stamp.
fn days_of_stamp(stamp: &str) -> Option<i64> {
    if stamp.len() != 8 || !stamp.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let y: i64 = stamp[..4].parse().ok()?;
    let m: i64 = stamp[4..6].parse().ok()?;
    let d: i64 = stamp[6..].parse().ok()?;
    // Inverse of `civil`.
    let y2 = if m <= 2 { y - 1 } else { y };
    let era = y2.div_euclid(400);
    let yoe = y2 - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146_097 + doe - 719_468)
}

/// Deletes the day files older than `keep_days` (by the date in their names) in every
/// session folder of `dir`, and the session folders left without day files. Returns the
/// files deleted.
pub fn apply_retention(dir: &Path, keep_days: u32, now_ms: u64) -> io::Result<Vec<PathBuf>> {
    let today = (now_ms / DAY_MS) as i64;
    let mut removed = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(removed);
    };
    for e in entries.flatten() {
        let session = e.path();
        let is_session = session.is_dir()
            && session
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(SESSION_PREFIX))
            && session.join(HEADER).is_file();
        if !is_session {
            continue;
        }
        let (mut left, mut gone) = (0, 0);
        for f in fs::read_dir(&session)?.flatten() {
            let p = f.path();
            let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let Some(stamp) = name
                .strip_prefix(PREFIX)
                .and_then(|s| s.strip_suffix(".jsonl"))
            else {
                continue;
            };
            match days_of_stamp(stamp) {
                Some(day) if today - day >= i64::from(keep_days) => {
                    fs::remove_file(&p)?;
                    removed.push(p);
                    gone += 1;
                }
                _ => left += 1,
            }
        }
        if left == 0 && gone > 0 {
            fs::remove_file(session.join(HEADER))?;
            let _ = fs::remove_dir(&session);
        }
    }
    Ok(removed)
}

/// Replaces people's identifiers by nicknames (see the module docs).
pub struct Anonymizer {
    key: RandomState,
    patterns: Vec<String>,
}

impl Anonymizer {
    pub fn new(patterns: &[String]) -> Anonymizer {
        Anonymizer {
            key: RandomState::new(),
            patterns: patterns.iter().map(|p| p.to_lowercase()).collect(),
        }
    }

    pub fn nickname(&self, s: &str) -> String {
        let mut h = self.key.build_hasher();
        h.write(s.as_bytes());
        format!("pessoa-{:016x}", h.finish())
    }

    /// Whether a tag names a person by the configured patterns.
    pub fn matches(&self, tag: &str) -> bool {
        let t = tag.to_lowercase();
        self.patterns.iter().any(|p| !p.is_empty() && t.contains(p))
    }

    fn scrub_json(&self, v: &mut serde_json::Value) {
        match v {
            serde_json::Value::String(s) => *s = self.nickname(s),
            serde_json::Value::Array(a) => a.iter_mut().for_each(|x| self.scrub_json(x)),
            serde_json::Value::Object(o) => o.values_mut().for_each(|x| self.scrub_json(x)),
            _ => {}
        }
    }

    /// Anonymises an observation of a person: the tag, and the texts in the value.
    pub fn scrub(&self, obs: &mut Observation) {
        obs.tag = self.nickname(&obs.tag);
        if let Value::Text(t) = &obs.value {
            obs.value = match serde_json::from_str::<serde_json::Value>(t) {
                Ok(mut j @ (serde_json::Value::Object(_) | serde_json::Value::Array(_))) => {
                    self.scrub_json(&mut j);
                    Value::Text(j.to_string())
                }
                _ => match obs.value.as_f64() {
                    Some(_) => obs.value.clone(),
                    None => Value::Text(self.nickname(t)),
                },
            };
        }
    }
}

/// Writes a recording.
pub struct Recorder {
    session: PathBuf,
    root: PathBuf,
    keep_days: u32,
    day: String,
    out: BufWriter<File>,
    last_retention_day: u64,
}

impl Recorder {
    /// Starts a recording in a new session folder of `root` (after applying the
    /// retention to the older ones).
    pub fn start(root: &Path, header: &SessionHeader) -> io::Result<Recorder> {
        fs::create_dir_all(root)?;
        let _ = apply_retention(root, header.retencao_dias, header.inicio_ms)?;
        let mut session = root.join(format!("{SESSION_PREFIX}{}", time_stamp(header.inicio_ms)));
        let mut n = 2;
        while session.exists() {
            session = root.join(format!(
                "{SESSION_PREFIX}{}-{n}",
                time_stamp(header.inicio_ms)
            ));
            n += 1;
        }
        fs::create_dir_all(&session)?;
        fs::write(
            session.join(HEADER),
            serde_json::to_string_pretty(header).map_err(io::Error::other)?,
        )?;
        let day = day_stamp(header.inicio_ms);
        let out = Self::open(&session, &day)?;
        Ok(Recorder {
            session,
            root: root.to_path_buf(),
            keep_days: header.retencao_dias,
            day,
            out,
            last_retention_day: header.inicio_ms / DAY_MS,
        })
    }

    fn open(session: &Path, day: &str) -> io::Result<BufWriter<File>> {
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(session.join(format!("{PREFIX}{day}.jsonl")))?;
        Ok(BufWriter::new(f))
    }

    pub fn session(&self) -> &Path {
        &self.session
    }

    pub fn write(&mut self, line: &Line, wall_ms: u64) -> io::Result<()> {
        let day = day_stamp(wall_ms);
        if day != self.day {
            self.out.flush()?;
            self.out = Self::open(&self.session, &day)?;
            self.day = day;
        }
        if wall_ms / DAY_MS > self.last_retention_day {
            self.last_retention_day = wall_ms / DAY_MS;
            self.out.flush()?;
            let _ = apply_retention(&self.root, self.keep_days, wall_ms);
        }
        serde_json::to_writer(&mut self.out, line).map_err(io::Error::other)?;
        self.out.write_all(b"\n")
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

/// A recording read back.
pub struct Recording {
    pub session: PathBuf,
    pub header: SessionHeader,
    pub lines: Vec<Line>,
}

impl Recording {
    /// Opens a session folder, or the most recent session of a recording folder.
    pub fn open(path: &Path) -> Result<Recording, String> {
        let session = if path.join(HEADER).is_file() {
            path.to_path_buf()
        } else {
            let mut sessions: Vec<PathBuf> = fs::read_dir(path)
                .map_err(|e| format!("{}: {e}", path.display()))?
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.join(HEADER).is_file())
                .collect();
            sessions.sort();
            sessions.pop().ok_or_else(|| {
                format!(
                    "{}: nenhuma gravação do gêmeo (sessao.json) encontrada",
                    path.display()
                )
            })?
        };
        let text = fs::read_to_string(session.join(HEADER))
            .map_err(|e| format!("{}: {e}", session.display()))?;
        let header: SessionHeader =
            serde_json::from_str(&text).map_err(|e| format!("{HEADER}: {e}"))?;
        let mut files: Vec<PathBuf> = fs::read_dir(&session)
            .map_err(|e| e.to_string())?
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with(PREFIX) && n.ends_with(".jsonl"))
            })
            .collect();
        files.sort();
        let mut lines = Vec::new();
        for f in files {
            let r = BufReader::new(File::open(&f).map_err(|e| format!("{}: {e}", f.display()))?);
            for (i, l) in r.lines().enumerate() {
                let l = l.map_err(|e| format!("{}: {e}", f.display()))?;
                if l.trim().is_empty() {
                    continue;
                }
                let line: Line = serde_json::from_str(&l)
                    .map_err(|e| format!("{}, linha {}: {e}", f.display(), i + 1))?;
                lines.push(line);
            }
        }
        Ok(Recording {
            session,
            header,
            lines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates() {
        assert_eq!(day_stamp(0), "19700101");
        // 2026-11-03 00:00 UTC = 20395 days.
        assert_eq!(day_stamp(20_760 * DAY_MS), "20261103");
        assert_eq!(days_of_stamp("20261103"), Some(20_760));
        assert_eq!(days_of_stamp("20240229"), Some(19_782));
        assert_eq!(day_stamp(19_782 * DAY_MS + 5), "20240229");
        assert_eq!(days_of_stamp("2024022"), None);
        assert_eq!(time_stamp(20_760 * DAY_MS + 3_723_000), "20261103-010203");
    }

    #[test]
    fn nicknames_are_stable_per_recording_and_hide_texts() {
        let a = Anonymizer::new(&["rtls".into()]);
        assert_eq!(a.nickname("João"), a.nickname("João"));
        assert_ne!(a.nickname("João"), a.nickname("Maria"));
        assert!(a.matches("fabrica/RTLS/crachá-17"));
        let b = Anonymizer::new(&[]);
        assert_ne!(
            a.nickname("João"),
            b.nickname("João"),
            "outra gravação, outro apelido"
        );
        let mut o = Observation {
            t: 0.0,
            wall_ms: 0,
            source: "s".into(),
            seq: 0,
            tag: "rtls/joao".into(),
            value: Value::Text(r#"{"nome":"João","x":1.5,"y":2}"#.into()),
            quality: Default::default(),
            rule: Some(0),
        };
        a.scrub(&mut o);
        let Value::Text(t) = &o.value else { panic!() };
        assert!(!t.contains("João") && t.contains("1.5"), "{t}");
        assert!(o.tag.starts_with("pessoa-"));
    }
}
