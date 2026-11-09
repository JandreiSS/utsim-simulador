//! The editors of a source's arrival table (`table`) and arrivals by period (`schedule`)
//! in the properties panel (pure: no Godot types, unit tested).
//!
//! Both fields stay RON text in the schema (`schema.rs`); here they are drawn as editors:
//!
//! - **Tabela de chegadas**: mode (horário or sequência) and repetition, a grid with one
//!   line per row (instant, only in the timetable, count, item type among the model's
//!   types, labels as `nome=valor; ...`), add, remove, move up and down, sort by instant,
//!   paste from a spreadsheet (tab or semicolon separated lines, decimal comma, optional
//!   header), the core's validation messages under their line and the totals per cycle and
//!   per type. "Ver as primeiras chegadas" draws a histogram per hour and per type.
//! - **Chegadas por período**: the cycle, a band of the cycle with the periods as blocks
//!   coloured by mode (padrão, distribuição, taxa, fechado), one line per period (start,
//!   end, mode, rate in items per hour or the distribution's RON), add and remove, and a bar
//!   chart of the expected arrivals per period (`ArrivalSchedule::expected_in_period`).
//!
//! Every edit leaves as one [`PropAction::Set`] on `table` / `schedule` with the RON of the
//! whole value (one undo step), which the bridge applies through `Editor::set_property`;
//! when it refuses, the panel shows its error as for any field. The `{ }` button shows the
//! plain RON line instead (the widget `Prop_table` / `Prop_schedule`, as before).
//!
//! As in the Bevy viewer's table (`utsim-viewer/src/properties/schedule.rs`, whose time
//! helpers are mirrored here: this crate does not depend on the viewer), the model has no
//! time unit: a cycle of 1440 (or 10080) reads as a day (or week) in minutes, 24 (168) in
//! hours and 86400 (604800) in seconds; only then times show as hh:mm and rates per hour.
//!
//! Widget names: `Prop_table_<part>` and `Prop_schedule_<part>` with `_modo`, `_repetir`,
//! `_ciclo`, `_linha_<k>_<coluna>` (`at`, `count`, `tipo`, `rotulos`, `subir`, `descer`,
//! `remover`, `erro`), `_mais`, `_ordenar`, `_colar` (with `_colar_texto`,
//! `_colar_substituir`, `_colar_acrescentar`, `_colar_erro`), `_total`, `_previa`
//! (`_previa_grafico`), `_texto`, `_erros`, and for the schedule `_faixa`,
//! `_periodo_<k>_<campo>` (`inicio`, `fim`, `modo`, `valor`, `remover`, `erro`), `_grafico`.
//! Rows and periods are numbered from 1, as in the core's messages.

use std::collections::{BTreeMap, HashMap, HashSet};

use egui::{
    Button, Color32, Label, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, TextEdit, Ui, Vec2,
};
use utsim_core::{Dist, TraceEvent};
use utsim_model::{
    ArrivalPeriod, ArrivalRow, ArrivalSchedule, ArrivalTable, Model, ObjectKind, PeriodArrivals,
    TableMode, TableRepeat,
};
use utsim_ui_tema::{colors, sizes};

use crate::egui_widgets::Registry;
use crate::ron_text as ron;
use crate::schema::{Field, Value};
use crate::ui_props::{
    self, PropAction, PropsState, PropsView, line_edit, named, named_toggle, num_text, set,
    widget_name,
};

/// Whether `path` is drawn by this module.
pub fn handles(path: &str) -> bool {
    path == "table" || path == "schedule"
}

/// What the editors keep between frames (dropped when another object is shown).
#[derive(Default, Debug)]
pub struct ArrivalsState {
    /// Fields shown as plain RON text (`{ }`).
    as_text: HashSet<String>,
    /// The paste area is open, with its text and the error of the last paste.
    paste_open: bool,
    paste: String,
    paste_error: String,
    /// The error of a cell that could not be read (by widget name).
    cell_errors: BTreeMap<String, String>,
    /// "Ver as primeiras chegadas" is open.
    preview: bool,
    /// The source run alone for the preview ([`run_preview`]), for the table or schedule it
    /// was run with (RON text); the bridge fills it when the panel asks.
    pub run: Option<(String, Result<Histogram, String>)>,
    /// A block of the band being dragged: the period, what was grabbed, the x of the press.
    drag: Option<(usize, Grab, f32)>,
}

impl ArrivalsState {
    /// The result of the source run alone for `key` (the RON it was run with).
    pub fn set_run(&mut self, key: String, r: Result<Histogram, String>) {
        self.run = Some((key, r));
    }
}

// ---------------------------------------------------------------------------------------
// Time unit and clock text (as the viewer's `properties/schedule.rs`).

/// Unit of the model time, guessed from a cycle.
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
            Unit::Seconds => "segundos",
            Unit::Minutes => "minutos",
            Unit::Hours => "horas",
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

/// A time as the editors show it: the clock when the cycle suggests a unit (and the time
/// falls on a whole minute), else the number.
pub fn time_text(t: f64, cycle: Option<f64>) -> String {
    let exact = cycle.and_then(unit_of_cycle).is_some_and(|u| {
        ((t / u.per_hour() * 60.0) - (t / u.per_hour() * 60.0).round()).abs() < 1e-6
    });
    match cycle.filter(|_| exact).and_then(|c| clock(t, c)) {
        Some(c) => c,
        None => num_text(t),
    }
}

/// A number typed by the user: decimal point or comma.
pub fn parse_number(text: &str) -> Result<f64, String> {
    let t = text.trim();
    t.replace(',', ".")
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
        .ok_or_else(|| format!("\"{t}\" não é um número"))
}

/// Reads "hh:mm" (or "dia N hh:mm" in a weekly cycle) as an offset in the model unit of
/// a cycle that suggests one; plain numbers ("420", "7,5") are model time.
pub fn parse_time(text: &str, cycle: Option<f64>) -> Result<f64, String> {
    let t = text.trim();
    let unit = cycle.and_then(unit_of_cycle);
    if let Some(rest) = t.strip_prefix("dia ") {
        let (day, hm) = rest
            .trim()
            .split_once(' ')
            .ok_or_else(|| format!("use \"dia N hh:mm\" (recebido \"{t}\")"))?;
        let day: u32 = day
            .parse()
            .ok()
            .filter(|d| *d >= 1)
            .ok_or_else(|| format!("dia inválido em \"{t}\" (a partir de 1)"))?;
        let unit = unit.ok_or_else(|| "use um número".to_string())?;
        return Ok(parse_time(hm, cycle)? + f64::from(day - 1) * 24.0 * unit.per_hour());
    }
    if let Some((h, m)) = t.split_once(':') {
        let unit = unit.ok_or_else(|| {
            "hh:mm só vale com ciclo de dia ou semana (1440 min, 24 h, 86400 s); use um número"
                .to_string()
        })?;
        let h: u32 = h
            .trim()
            .parse()
            .map_err(|_| format!("hora inválida em \"{t}\""))?;
        let m: u32 = m
            .trim()
            .parse()
            .map_err(|_| format!("minuto inválido em \"{t}\""))?;
        if m >= 60 {
            return Err(format!("minuto inválido em \"{t}\" (0 a 59)"));
        }
        return Ok(f64::from(h * 60 + m) / 60.0 * unit.per_hour());
    }
    parse_number(t).map_err(|_| format!("\"{t}\" não é um horário (hh:mm) nem um número"))
}

fn parse_count(text: &str) -> Result<u64, String> {
    let t = text.trim();
    if t.is_empty() {
        return Ok(1);
    }
    let v = parse_number(t)?;
    if v < 0.0 || v.fract() != 0.0 {
        return Err(format!("quantidade \"{t}\": um inteiro >= 0"));
    }
    Ok(v as u64)
}

// ---------------------------------------------------------------------------------------
// The arrival table.

/// The cycle of a table that repeats (`Cycle(d)`).
pub fn table_cycle(t: &ArrivalTable) -> Option<f64> {
    match t.repeat {
        TableRepeat::Cycle(d) => Some(d),
        _ => None,
    }
}

/// A row's labels as `nome=valor; ...`.
pub fn labels_text(labels: &BTreeMap<String, f64>) -> String {
    labels
        .iter()
        .map(|(k, v)| format!("{k}={}", num_text(*v)))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Reads `nome=valor; ...` (also `nome: valor`; decimal comma accepted).
pub fn parse_labels(text: &str) -> Result<BTreeMap<String, f64>, String> {
    let mut out = BTreeMap::new();
    for part in text.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (name, value) = part
            .split_once('=')
            .or_else(|| part.split_once(':'))
            .ok_or_else(|| format!("rótulo \"{part}\": use nome=valor"))?;
        let name = name.trim().trim_matches('"');
        if name.is_empty() {
            return Err(format!("rótulo \"{part}\" sem nome"));
        }
        let v = parse_number(value).map_err(|e| format!("rótulo {name}: {e}"))?;
        out.insert(name.to_string(), v);
    }
    Ok(out)
}

/// The mean time between arrivals of the source (1 when it has none that is usable).
fn usable_mean(default: &Dist) -> f64 {
    let m = default.mean();
    if m.is_finite() && m > 0.0 { m } else { 1.0 }
}

/// A cycle that holds every instant of `t`: a day of 1440 when it fits, else a week, else
/// one step after the last instant.
pub fn default_cycle(t: &ArrivalTable) -> f64 {
    let last = t.rows.iter().filter_map(|r| r.at).fold(0.0, f64::max);
    if last < 1440.0 {
        1440.0
    } else if last < 10080.0 {
        10080.0
    } else {
        (last + 1.0).ceil()
    }
}

/// The table in another mode: a sequence has no instants (and `Cycle` becomes `Loop`); a
/// timetable gets one instant per row, `mean` apart (and `Loop` becomes a cycle).
pub fn set_mode(t: &ArrivalTable, mode: TableMode, default: &Dist) -> ArrivalTable {
    let mut out = t.clone();
    if t.mode == mode {
        return out;
    }
    out.mode = mode;
    match mode {
        TableMode::Sequence => {
            for r in &mut out.rows {
                r.at = None;
            }
            if matches!(t.repeat, TableRepeat::Cycle(_)) {
                out.repeat = TableRepeat::Loop;
            }
        }
        TableMode::Times => {
            let m = usable_mean(default);
            for (k, r) in out.rows.iter_mut().enumerate() {
                r.at = Some(k as f64 * m);
            }
            if t.repeat == TableRepeat::Loop {
                out.repeat = TableRepeat::Cycle(default_cycle(&out));
            }
        }
    }
    out
}

/// The repetitions a mode accepts: `(value, label)`.
pub fn repeat_choices(mode: TableMode) -> Vec<(&'static str, &'static str)> {
    match mode {
        TableMode::Times => vec![("Once", "Uma vez"), ("Cycle", "A cada ciclo")],
        TableMode::Sequence => vec![("Once", "Uma vez"), ("Loop", "Volta ao início")],
    }
}

pub fn repeat_id(r: TableRepeat) -> &'static str {
    match r {
        TableRepeat::Once => "Once",
        TableRepeat::Cycle(_) => "Cycle",
        TableRepeat::Loop => "Loop",
    }
}

/// The table with another repetition (`Once`, `Cycle` or `Loop`).
pub fn set_repeat(t: &ArrivalTable, id: &str) -> ArrivalTable {
    let mut out = t.clone();
    out.repeat = match id {
        "Cycle" => TableRepeat::Cycle(table_cycle(t).unwrap_or_else(|| default_cycle(t))),
        "Loop" => TableRepeat::Loop,
        _ => TableRepeat::Once,
    };
    out
}

/// A new row after the last: one item of the last row's type, one step later in a
/// timetable (the last gap, or the source's mean), kept inside the cycle.
pub fn add_row(t: &ArrivalTable, default: &Dist) -> ArrivalTable {
    let mut out = t.clone();
    let last = t.rows.last();
    let at = (t.mode == TableMode::Times).then(|| {
        let ats: Vec<f64> = t.rows.iter().filter_map(|r| r.at).collect();
        let gap = match ats.as_slice() {
            [.., a, b] if b > a => b - a,
            _ => usable_mean(default),
        };
        let next = ats.last().map_or(0.0, |a| a + gap);
        match table_cycle(t) {
            Some(d) if next >= d => ats.last().copied().unwrap_or(0.0),
            _ => next,
        }
    });
    out.rows.push(ArrivalRow {
        at,
        count: 1,
        item_type: last.and_then(|r| r.item_type.clone()),
        labels: BTreeMap::new(),
    });
    out
}

/// The rows in order of instant (stable: rows at the same instant keep their order).
pub fn sort_rows(t: &ArrivalTable) -> ArrivalTable {
    let mut out = t.clone();
    out.rows.sort_by(|a, b| {
        a.at.unwrap_or(f64::INFINITY)
            .total_cmp(&b.at.unwrap_or(f64::INFINITY))
    });
    out
}

/// Whether a timetable's rows are in order of instant.
pub fn is_sorted(t: &ArrivalTable) -> bool {
    t.rows
        .windows(2)
        .all(|w| w[0].at.unwrap_or(0.0) <= w[1].at.unwrap_or(0.0))
}

/// Items per pass over the table, in total and per type (`None` is the source's type), in
/// the order the types first appear.
#[derive(Clone, Debug, PartialEq)]
pub struct Totals {
    pub items: u64,
    pub rows: usize,
    pub by_type: Vec<(Option<String>, u64)>,
}

pub fn totals(t: &ArrivalTable) -> Totals {
    let mut by_type: Vec<(Option<String>, u64)> = Vec::new();
    for r in &t.rows {
        match by_type.iter_mut().find(|(k, _)| *k == r.item_type) {
            Some((_, n)) => *n += r.count,
            None => by_type.push((r.item_type.clone(), r.count)),
        }
    }
    Totals {
        items: t.rows.iter().map(|r| r.count).sum(),
        rows: t.rows.len(),
        by_type,
    }
}

/// The text of the totals line.
pub fn totals_text(t: &ArrivalTable) -> String {
    let s = totals(t);
    let per = match (t.mode, t.repeat) {
        (TableMode::Times, TableRepeat::Cycle(d)) => {
            format!("por ciclo de {}", time_span_text(d))
        }
        (_, TableRepeat::Loop) => "por volta".to_string(),
        _ => "no total".to_string(),
    };
    let mut text = format!(
        "{} {} {per} em {} linha(s)",
        s.items,
        if s.items == 1 { "item" } else { "itens" },
        s.rows
    );
    if s.by_type.len() > 1 || s.by_type.iter().any(|(k, _)| k.is_some()) {
        let parts: Vec<String> = s
            .by_type
            .iter()
            .map(|(k, n)| format!("{}: {n}", k.as_deref().unwrap_or("tipo da fonte")))
            .collect();
        text.push_str(&format!(" ({})", parts.join(", ")));
    }
    text
}

/// A length of time: the number, with the day or week it is when the unit is known.
fn time_span_text(d: f64) -> String {
    match unit_of_cycle(d) {
        Some(u) if (d / u.per_hour() - 24.0).abs() < 1e-9 => format!("{} (um dia)", num_text(d)),
        Some(_) => format!("{} (uma semana)", num_text(d)),
        None => num_text(d),
    }
}

/// The core's messages split by row (`table.rows[n]` or `schedule[n]`, from 1): the ones
/// of no row, and the ones of each row.
pub fn split_errors(
    errors: &[String],
    prefix: &str,
) -> (Vec<String>, BTreeMap<usize, Vec<String>>) {
    let mut general = Vec::new();
    let mut by_row: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let open = format!("{prefix}[");
    for e in errors {
        let row = e
            .strip_prefix(&open)
            .and_then(|r| r.split_once(']'))
            .and_then(|(n, _)| n.parse::<usize>().ok());
        match row {
            Some(n) => by_row.entry(n).or_default().push(e.clone()),
            None => general.push(e.clone()),
        }
    }
    (general, by_row)
}

/// A column of a pasted line.
#[derive(Clone, Debug, PartialEq)]
enum Column {
    At,
    Count,
    Type,
    Labels,
    /// A column whose header is a label's name.
    Label(String),
}

fn column_of(header: &str) -> Column {
    let h = header.trim().trim_matches('"').to_lowercase();
    match h.as_str() {
        "at" | "instante" | "hora" | "horario" | "horário" | "tempo" | "chegada" => Column::At,
        "count" | "quantidade" | "qtd" | "qtde" | "quant" | "itens" | "n" => Column::Count,
        "item_type" | "tipo" | "type" | "tipo de item" => Column::Type,
        "labels" | "rótulos" | "rotulos" | "rótulo" | "rotulo" => Column::Labels,
        _ => Column::Label(header.trim().trim_matches('"').to_string()),
    }
}

fn cells(line: &str) -> Vec<String> {
    let sep = if line.contains('\t') { '\t' } else { ';' };
    line.split(sep).map(|c| c.trim().to_string()).collect()
}

/// Rows read from lines copied from a spreadsheet: tab separated (as a spreadsheet copies)
/// or semicolon separated, decimal comma accepted, empty lines skipped. The first line is
/// a header when its first cell is not a number nor a time: its names pick the columns
/// (`hora`/`instante`/`at`, `quantidade`/`qtd`/`count`, `tipo`/`item_type`, `rótulos`) and
/// any other name is a label column. Without a header the columns are the grid's: instant
/// (timetable only), count, type, then `nome=valor` labels. Errors give the line, from 1.
pub fn parse_paste(
    text: &str,
    mode: TableMode,
    cycle: Option<f64>,
) -> Result<Vec<ArrivalRow>, String> {
    let lines: Vec<(usize, &str)> = text
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty())
        .map(|(k, l)| (k + 1, l))
        .collect();
    let Some(&(_, first)) = lines.first() else {
        return Err("nada para colar: copie as linhas da planilha".into());
    };
    let head = cells(first);
    let is_value =
        |c: &str| c.is_empty() || parse_time(c, cycle).is_ok() || parse_number(c).is_ok();
    let header: Option<Vec<Column>> = (!is_value(&head[0]) && !head[0].contains('='))
        .then(|| head.iter().map(|h| column_of(h)).collect());
    let body = if header.is_some() {
        &lines[1..]
    } else {
        &lines[..]
    };
    let positional: Vec<Column> = match mode {
        TableMode::Times => vec![Column::At, Column::Count, Column::Type],
        TableMode::Sequence => vec![Column::Count, Column::Type],
    };
    let mut rows = Vec::new();
    for &(n, line) in body {
        let fail = |e: String| format!("linha {n}: {e}");
        let mut row = ArrivalRow {
            at: None,
            count: 1,
            item_type: None,
            labels: BTreeMap::new(),
        };
        let mut slot = 0;
        for (k, cell) in cells(line).into_iter().enumerate() {
            let col = match &header {
                Some(h) => h.get(k).cloned().unwrap_or(Column::Labels),
                // Without a header, `nome=valor` cells are labels wherever they are.
                None if cell.contains('=') => Column::Labels,
                None => {
                    slot += 1;
                    positional.get(slot - 1).cloned().unwrap_or(Column::Labels)
                }
            };
            match col {
                Column::At if mode == TableMode::Times => {
                    if !cell.is_empty() {
                        row.at = Some(parse_time(&cell, cycle).map_err(fail)?);
                    }
                }
                // A sequence has no instants: the column is left out.
                Column::At => {}
                Column::Count => row.count = parse_count(&cell).map_err(fail)?,
                Column::Type => {
                    row.item_type = (!cell.is_empty()).then(|| cell.trim_matches('"').to_string());
                }
                Column::Labels => row.labels.extend(parse_labels(&cell).map_err(fail)?),
                Column::Label(name) => {
                    if !cell.is_empty() {
                        let v = parse_number(&cell).map_err(|e| fail(format!("{name}: {e}")))?;
                        row.labels.insert(name, v);
                    }
                }
            }
        }
        if mode == TableMode::Times && row.at.is_none() {
            return Err(fail("falta o instante (modo horário)".into()));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("só o cabeçalho: nenhuma linha para colar".into());
    }
    Ok(rows)
}

/// A preview of the arrivals: items per bin and per type over one cycle (or one pass).
#[derive(Clone, Debug, PartialEq)]
pub struct Histogram {
    /// Width of a bin in model time.
    pub bin: f64,
    /// Types in the order of the legend (`None` is the source's type).
    pub types: Vec<Option<String>>,
    /// Items per bin, one value per type.
    pub bins: Vec<Vec<f64>>,
    /// The bins are hours.
    pub hours: bool,
}

/// What the source creates in one cycle (timetable) or one pass (sequence), per hour when
/// the cycle suggests a unit (else in 24 bins). The instants of a timetable are exact; a
/// sequence is drawn at its expected instants (one row every mean interval, from the mean,
/// or from 0 with `first_at_zero`). This reads the table, it does not run the source: the
/// spacing of `release` and blocking are left out.
pub fn preview(t: &ArrivalTable, default: &Dist, first_at_zero: bool) -> Histogram {
    let mean = usable_mean(default);
    let instants: Vec<f64> = match t.mode {
        TableMode::Times => t.rows.iter().map(|r| r.at.unwrap_or(0.0)).collect(),
        TableMode::Sequence => (0..t.rows.len())
            .map(|k| (k as f64 + if first_at_zero { 0.0 } else { 1.0 }) * mean)
            .collect(),
    };
    let last = instants.iter().copied().fold(0.0, f64::max);
    let span = table_cycle(t).unwrap_or_else(|| default_cycle(t).max(last));
    let span = if t.mode == TableMode::Sequence && table_cycle(t).is_none() {
        // A pass of the sequence, in whole hours of a minutes model when it fits a day.
        if last < 1440.0 { 1440.0 } else { last + mean }
    } else {
        span
    };
    let unit = unit_of_cycle(span).or_else(|| unit_of_cycle(default_cycle(t)));
    let (bin, hours) = match unit {
        Some(u) => (u.per_hour(), true),
        None => (span / 24.0, false),
    };
    let n = ((span / bin).ceil() as usize).clamp(1, 24 * 7);
    let mut types: Vec<Option<String>> = Vec::new();
    for r in &t.rows {
        if !types.contains(&r.item_type) {
            types.push(r.item_type.clone());
        }
    }
    let mut bins = vec![vec![0.0; types.len()]; n];
    for (r, at) in t.rows.iter().zip(&instants) {
        let k = ((at / bin).floor().max(0.0) as usize).min(n - 1);
        let j = types.iter().position(|x| *x == r.item_type).unwrap_or(0);
        bins[k][j] += r.count as f64;
    }
    Histogram {
        bin,
        types,
        bins,
        hours,
    }
}

// ---------------------------------------------------------------------------------------
// Arrivals by period.

/// Modes of a period, in menu order, with their labels.
pub const MODES: [&str; 4] = ["padrão", "distribuição", "taxa", "fechado"];
const MODE_IDS: [&str; 4] = ["Default", "Interarrival", "Rate", "Closed"];

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
        (Some(p), _) if p.end > p.start => p.end - p.start,
        (_, Some(u)) => u.per_hour(),
        _ => s.cycle,
    };
    Some(ArrivalPeriod {
        start,
        end: (start + len).min(s.cycle),
        arrivals: PeriodArrivals::Default,
        labels: Default::default(),
    })
}

/// A rate as typed and shown: per hour when the cycle suggests a unit, else per unit of
/// model time.
pub fn rate_shown(rate: f64, cycle: f64) -> f64 {
    rate * unit_of_cycle(cycle).map_or(1.0, Unit::per_hour)
}

pub fn rate_of_shown(shown: f64, cycle: f64) -> f64 {
    shown / unit_of_cycle(cycle).map_or(1.0, Unit::per_hour)
}

/// One block of the band of the cycle, in points from the band's left edge.
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub x0: f32,
    pub x1: f32,
    /// The period (from 0) drawn here; `None` for the time outside every period.
    pub period: Option<usize>,
}

/// The band of a schedule `width` points wide: a block per period (clipped to the cycle;
/// empty or reversed periods are left out) and the gaps between them, left to right.
pub fn band(s: &ArrivalSchedule, width: f32) -> Vec<Block> {
    if !(s.cycle.is_finite() && s.cycle > 0.0) || width <= 0.0 {
        return Vec::new();
    }
    let x = |t: f64| ((t.clamp(0.0, s.cycle) / s.cycle) as f32) * width;
    let mut spans: Vec<(f32, f32, usize)> = s
        .periods
        .iter()
        .enumerate()
        .filter(|(_, p)| p.start.is_finite() && p.end.is_finite())
        .map(|(k, p)| (x(p.start), x(p.end), k))
        .filter(|(a, b, _)| b > a)
        .collect();
    spans.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out = Vec::new();
    let mut at = 0.0f32;
    for (a, b, k) in spans {
        if a > at + 0.01 {
            out.push(Block {
                x0: at,
                x1: a,
                period: None,
            });
        }
        out.push(Block {
            x0: a,
            x1: b,
            period: Some(k),
        });
        at = at.max(b);
    }
    if at < width - 0.01 {
        out.push(Block {
            x0: at,
            x1: width,
            period: None,
        });
    }
    out
}

/// Expected arrivals in each period (the core's `expected_in_period`), with the source's
/// interarrival as the default; 0 for a default with no usable mean.
pub fn expected(s: &ArrivalSchedule, default: &Dist) -> Vec<f64> {
    (0..s.periods.len())
        .map(|k| s.expected_in_period(k, default))
        .map(|v| if v.is_finite() { v.max(0.0) } else { 0.0 })
        .collect()
}

// ---------------------------------------------------------------------------------------
// Drawing.

/// What a one-line text field takes beyond its desired width (its margins).
const EDIT_SLACK: f32 = 8.0;
/// What a list takes beyond its width (the arrow and its margins).
const COMBO_SLACK: f32 = 24.0;
/// Room left at the end of a grid line (the buttons' own margins).
const ROW_SLACK: f32 = 16.0;

/// Colour of a period mode in the band and the chart.
pub fn mode_color(index: usize) -> Color32 {
    match index {
        0 => Color32::from_rgb(96, 146, 230),
        1 => Color32::from_rgb(168, 120, 222),
        2 => Color32::from_rgb(86, 186, 120),
        _ => Color32::from_rgb(92, 98, 114),
    }
}

/// Colour of the k-th item type of a chart.
fn type_color(k: usize, ty: &Option<String>) -> Color32 {
    const PALETTE: [Color32; 8] = [
        Color32::from_rgb(59, 130, 246),
        Color32::from_rgb(245, 158, 11),
        Color32::from_rgb(16, 185, 129),
        Color32::from_rgb(239, 68, 68),
        Color32::from_rgb(139, 92, 246),
        Color32::from_rgb(236, 72, 153),
        Color32::from_rgb(20, 184, 166),
        Color32::from_rgb(234, 179, 8),
    ];
    if ty.is_none() {
        Color32::from_rgb(140, 150, 170)
    } else {
        PALETTE[k % PALETTE.len()]
    }
}

/// The source's interarrival time, read from its field.
fn default_dist(v: &PropsView) -> Dist {
    v.props
        .fields
        .iter()
        .find(|f| f.path == "interarrival")
        .and_then(|f| f.value.as_text())
        .and_then(|t| ron::parse::<Dist>(t).ok())
        .unwrap_or(Dist::Exp { mean: 1.0 })
}

fn first_at_zero(v: &PropsView) -> bool {
    v.props
        .fields
        .iter()
        .find(|f| f.path == "first_at_zero")
        .and_then(|f| f.value.as_bool())
        .unwrap_or(false)
}

fn small(text: &str) -> RichText {
    RichText::new(text)
        .size(sizes::SMALL)
        .color(colors::TEXT_WEAK)
}

fn error_lines(ui: &mut Ui, reg: &mut Registry, name: &str, errors: &[String]) {
    if errors.is_empty() {
        return;
    }
    let text = errors.join("\n");
    let r = ui.add(Label::new(RichText::new(&text).size(sizes::SMALL).color(colors::ERROR)).wrap());
    named(ui, reg, name, &r, &text);
}

/// A small button of a grid line.
fn tiny(ui: &mut Ui, reg: &mut Registry, name: &str, text: &str, tip: &str, enabled: bool) -> bool {
    let r = ui
        .add_enabled(enabled, Button::new(text).min_size(Vec2::new(18.0, 22.0)))
        .on_hover_text(tip);
    named(ui, reg, name, &r, text);
    r.clicked()
}

/// A fixed-width label of a header line.
fn head(ui: &mut Ui, text: &str, w: f32) {
    ui.add_sized([w, 16.0], Label::new(small(text)));
}

/// A choice list over `(value, label)`.
fn choose(
    ui: &mut Ui,
    reg: &mut Registry,
    name: &str,
    selected: &str,
    choices: &[(&str, &str)],
    width: f32,
    help: &str,
) -> Option<String> {
    let choices: Vec<(String, String, String)> = choices
        .iter()
        .map(|(v, l)| (v.to_string(), l.to_string(), String::new()))
        .collect();
    ui_props::combo(ui, reg, name, selected, &choices, width, help)
}

/// The `table` and `schedule` fields: the row with the switch and `{ }`, then the editor
/// (or the RON line).
pub fn field(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    f: &Field,
) {
    let name = widget_name(&f.path);
    let path = f.path.as_str();
    let text_on = st.arrivals.as_text.contains(path);
    let on = !matches!(f.value, Value::Null);
    ui_props::row_label(ui, &f.label, &f.help, f.depth, |ui, _| {
        if f.optional {
            ui_props::toggle(ui, reg, out, f);
        }
        if !on {
            return;
        }
        let r = ui
            .add(Button::new("{ }").selected(text_on))
            .on_hover_text("Editar como texto (RON)");
        named_toggle(ui, reg, &format!("{name}_texto"), &r, "{ }", text_on);
        if r.clicked() {
            if text_on {
                st.arrivals.as_text.remove(path);
            } else {
                st.arrivals.as_text.insert(path.to_owned());
            }
        }
    });
    let Some(text) = f.value.as_text().map(str::to_owned) else {
        return;
    };
    let as_text = st.arrivals.as_text.contains(path);
    let editor = (!as_text).then(|| match path {
        "table" => ron::parse::<ArrivalTable>(&text).map(|t| {
            table_editor(ui, v, st, reg, out, f, &t);
        }),
        _ => ron::parse::<ArrivalSchedule>(&text).map(|s| {
            schedule_editor(ui, v, st, reg, out, &s);
        }),
    });
    if let Some(Err(e)) = &editor {
        error_lines(ui, reg, &format!("{name}_erros"), std::slice::from_ref(e));
    }
    if !matches!(editor, Some(Ok(()))) {
        let w = ui.available_width();
        if let Some(t) = line_edit(ui, st, reg, &name, &text, w, true, "") {
            out.push(set(path, Value::Text(t)));
        }
    }
}

/// What a grid line asked, applied after the grid is drawn.
enum RowOp {
    Remove(usize),
    Move(usize, isize),
}

fn table_editor(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    f: &Field,
    t: &ArrivalTable,
) {
    let p = "Prop_table";
    let default = default_dist(v);
    let mut work = t.clone();
    let mut changed = false;
    let cycle = table_cycle(t);
    let times = t.mode == TableMode::Times;

    // Mode and repetition.
    ui.horizontal(|ui| {
        ui.label(small("Modo"));
        if let Some(m) = choose(
            ui,
            reg,
            &format!("{p}_modo"),
            if times { "Times" } else { "Sequence" },
            &[("Times", "Horário"), ("Sequence", "Sequência")],
            110.0,
            "Horário: cada linha no seu instante. Sequência: uma linha por chegada, com o \
             tempo entre chegadas da fonte entre elas.",
        ) {
            let mode = if m == "Sequence" {
                TableMode::Sequence
            } else {
                TableMode::Times
            };
            work = set_mode(&work, mode, &default);
            changed = true;
        }
        ui.label(small("Repetir"));
        let choices = repeat_choices(t.mode);
        if let Some(r) = choose(
            ui,
            reg,
            &format!("{p}_repetir"),
            repeat_id(t.repeat),
            &choices,
            110.0,
            "",
        ) {
            work = set_repeat(&work, &r);
            changed = true;
        }
    });
    if let Some(d) = cycle {
        let unit = unit_of_cycle(d).map_or(String::new(), |u| format!("({})", u.name()));
        ui.horizontal(|ui| {
            ui.label(small("Ciclo"));
            let name = format!("{p}_ciclo");
            if let Some(typed) = line_edit(ui, st, reg, &name, &num_text(d), 80.0, false, "") {
                match parse_number(&typed) {
                    Ok(x) => {
                        st.arrivals.cell_errors.remove(&name);
                        work.repeat = TableRepeat::Cycle(x);
                        changed = true;
                    }
                    Err(e) => {
                        st.arrivals.cell_errors.insert(name.clone(), e);
                    }
                }
            }
            ui.label(small(&unit));
        });
        let e: Vec<String> = st
            .arrivals
            .cell_errors
            .get(&format!("{p}_ciclo"))
            .cloned()
            .into_iter()
            .collect();
        error_lines(ui, reg, &format!("{p}_ciclo_erro"), &e);
    }

    // The grid.
    let types: Vec<(String, String, String)> = std::iter::once((
        String::new(),
        "(da fonte)".to_string(),
        "O tipo da fonte".to_string(),
    ))
    .chain(
        f.choices
            .iter()
            .map(|c| (c.value.clone(), c.label.clone(), String::new())),
    )
    .collect();
    let (general, by_row) = split_errors(&t.validate(), "table.rows");
    let full = ui.available_width();
    let gap = 4.0;
    let (wn, wat, wc, wt, wb) = (18.0, 50.0, 32.0, 72.0, 18.0);
    // Text fields are wider than their desired width (margins); the list's width given
    // to `combo` leaves room for its arrow.
    let at_w = if times { wat + EDIT_SLACK + gap } else { 0.0 };
    let fixed = wn + at_w + wc + wt + 3.0 * wb + 6.0 * gap + 2.0 * EDIT_SLACK;
    let wl = (full - fixed - ROW_SLACK).max(60.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        head(ui, "#", wn);
        if times {
            head(ui, "instante", wat + EDIT_SLACK);
        }
        head(ui, "qtd", wc + EDIT_SLACK);
        head(ui, "tipo", wt);
        head(ui, "rótulos", wl + EDIT_SLACK);
    });
    let mut op = None;
    let n = t.rows.len();
    for (k, r) in t.rows.iter().enumerate() {
        let line = format!("{p}_linha_{}", k + 1);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            ui.add_sized([wn, 22.0], Label::new(small(&(k + 1).to_string())));
            if times {
                let name = format!("{line}_at");
                let shown = r.at.map(|a| time_text(a, cycle)).unwrap_or_default();
                if let Some(typed) = line_edit(ui, st, reg, &name, &shown, wat, false, "") {
                    match parse_time(&typed, cycle) {
                        Ok(x) => {
                            st.arrivals.cell_errors.remove(&name);
                            work.rows[k].at = Some(x);
                            changed = true;
                        }
                        Err(e) => {
                            st.arrivals.cell_errors.insert(name, e);
                        }
                    }
                }
            }
            let name = format!("{line}_count");
            if let Some(typed) = line_edit(ui, st, reg, &name, &r.count.to_string(), wc, false, "")
            {
                match parse_count(&typed) {
                    Ok(c) => {
                        st.arrivals.cell_errors.remove(&name);
                        work.rows[k].count = c;
                        changed = true;
                    }
                    Err(e) => {
                        st.arrivals.cell_errors.insert(name, e);
                    }
                }
            }
            let selected = r.item_type.clone().unwrap_or_default();
            let mut choices = types.clone();
            if !selected.is_empty() && !choices.iter().any(|c| c.0 == selected) {
                // A type the model does not have (the core says so in the problems).
                choices.push((selected.clone(), format!("{selected} (?)"), String::new()));
            }
            if let Some(ty) = ui_props::combo(
                ui,
                reg,
                &format!("{line}_tipo"),
                &selected,
                &choices,
                wt - COMBO_SLACK,
                "Tipo de item da linha",
            ) {
                work.rows[k].item_type = (!ty.is_empty()).then_some(ty);
                changed = true;
            }
            let name = format!("{line}_rotulos");
            if let Some(typed) = line_edit(
                ui,
                st,
                reg,
                &name,
                &labels_text(&r.labels),
                wl,
                false,
                "nome=valor",
            ) {
                match parse_labels(&typed) {
                    Ok(l) => {
                        st.arrivals.cell_errors.remove(&name);
                        work.rows[k].labels = l;
                        changed = true;
                    }
                    Err(e) => {
                        st.arrivals.cell_errors.insert(name, e);
                    }
                }
            }
            if tiny(
                ui,
                reg,
                &format!("{line}_subir"),
                "↑",
                "Subir a linha",
                k > 0,
            ) {
                op = Some(RowOp::Move(k, -1));
            }
            if tiny(
                ui,
                reg,
                &format!("{line}_descer"),
                "↓",
                "Descer a linha",
                k + 1 < n,
            ) {
                op = Some(RowOp::Move(k, 1));
            }
            if tiny(
                ui,
                reg,
                &format!("{line}_remover"),
                "×",
                "Remover a linha",
                true,
            ) {
                op = Some(RowOp::Remove(k));
            }
        });
        let mut errors: Vec<String> = st
            .arrivals
            .cell_errors
            .iter()
            .filter(|(c, _)| c.starts_with(&format!("{line}_")))
            .map(|(_, e)| e.clone())
            .collect();
        errors.extend(by_row.get(&(k + 1)).cloned().unwrap_or_default());
        error_lines(ui, reg, &format!("{line}_erro"), &errors);
    }
    match op {
        Some(RowOp::Remove(k)) => {
            work.rows.remove(k);
            st.arrivals.cell_errors.clear();
            changed = true;
        }
        Some(RowOp::Move(k, d)) => {
            let j = (k as isize + d) as usize;
            work.rows.swap(k, j);
            st.arrivals.cell_errors.clear();
            changed = true;
        }
        None => {}
    }

    // Buttons of the table.
    ui.horizontal_wrapped(|ui| {
        let r = ui
            .button("+ Linha")
            .on_hover_text("Acrescenta uma linha no fim");
        named(ui, reg, &format!("{p}_mais"), &r, "+ Linha");
        if r.clicked() {
            work = add_row(&work, &default);
            changed = true;
        }
        let r = ui
            .add_enabled(times && !is_sorted(t), Button::new("Ordenar por instante"))
            .on_hover_text("Põe as linhas em ordem de instante")
            .on_disabled_hover_text("As linhas já estão em ordem (ou é uma sequência)");
        named(ui, reg, &format!("{p}_ordenar"), &r, "Ordenar por instante");
        if r.clicked() {
            work = sort_rows(&work);
            st.arrivals.cell_errors.clear();
            changed = true;
        }
        let open = st.arrivals.paste_open;
        let r = ui
            .add(Button::new("Colar da planilha").selected(open))
            .on_hover_text("Linhas copiadas de uma planilha (separadas por tabulação ou ;)");
        named_toggle(
            ui,
            reg,
            &format!("{p}_colar"),
            &r,
            "Colar da planilha",
            open,
        );
        if r.clicked() {
            st.arrivals.paste_open = !open;
        }
    });
    if st.arrivals.paste_open {
        let hint = if times {
            "hora; qtd; tipo; rótulo...\n08:00; 12; caixa; prioridade=2"
        } else {
            "qtd; tipo; rótulo...\n2; caixa; prioridade=2"
        };
        let mut buf = std::mem::take(&mut st.arrivals.paste);
        let r = ui.add(
            TextEdit::multiline(&mut buf)
                .id_salt("utsim_table_colar")
                .desired_width(ui.available_width())
                .desired_rows(4)
                .font(egui::TextStyle::Monospace)
                .hint_text(hint),
        );
        named(ui, reg, &format!("{p}_colar_texto"), &r, &buf);
        st.arrivals.paste = buf;
        let mut pasted = None;
        ui.horizontal(|ui| {
            for (what, label, replace) in [
                ("substituir", "Substituir as linhas", true),
                ("acrescentar", "Acrescentar", false),
            ] {
                let r = ui.add_enabled(!st.arrivals.paste.trim().is_empty(), Button::new(label));
                named(ui, reg, &format!("{p}_colar_{what}"), &r, label);
                if r.clicked() {
                    pasted = Some(replace);
                }
            }
        });
        if let Some(replace) = pasted {
            match parse_paste(&st.arrivals.paste, t.mode, cycle) {
                Ok(rows) => {
                    if replace {
                        work.rows = rows;
                    } else {
                        work.rows.extend(rows);
                    }
                    st.arrivals.paste.clear();
                    st.arrivals.paste_error.clear();
                    st.arrivals.paste_open = false;
                    st.arrivals.cell_errors.clear();
                    changed = true;
                }
                Err(e) => st.arrivals.paste_error = e,
            }
        }
        let e: Vec<String> = Some(st.arrivals.paste_error.clone())
            .filter(|e| !e.is_empty())
            .into_iter()
            .collect();
        error_lines(ui, reg, &format!("{p}_colar_erro"), &e);
    }
    error_lines(ui, reg, &format!("{p}_erros"), &general);
    let total = totals_text(t);
    let r = ui.add(Label::new(RichText::new(&total).color(colors::TEXT_WEAK)).wrap());
    named(ui, reg, &format!("{p}_total"), &r, &total);

    // Preview.
    let open = st.arrivals.preview;
    let r = ui
        .add(Button::new("Ver as primeiras chegadas").selected(open))
        .on_hover_text("Histograma das chegadas de um ciclo (ou uma volta), por hora e por tipo");
    named_toggle(
        ui,
        reg,
        &format!("{p}_previa"),
        &r,
        "Ver as primeiras chegadas",
        open,
    );
    if r.clicked() {
        st.arrivals.preview = !open;
    }
    if st.arrivals.preview && !t.rows.is_empty() {
        let key = ron::table(t);
        run_or_estimate(ui, st, reg, out, p, &key, || {
            preview(t, &default, first_at_zero(v))
        });
    }

    if changed && work != *t {
        out.push(set("table", Value::Text(ron::table(&work))));
    }
}

/// The preview: the source run alone (asked of the bridge once per table or schedule), or,
/// until it comes or when it cannot run, the estimate read from the table.
#[allow(clippy::too_many_arguments)]
fn run_or_estimate(
    ui: &mut Ui,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    p: &str,
    key: &str,
    estimate: impl FnOnce() -> Histogram,
) {
    let ran = st
        .arrivals
        .run
        .as_ref()
        .filter(|r| r.0 == key)
        .map(|r| r.1.clone());
    let (h, note) = match ran {
        Some(Ok(h)) => (
            h,
            "Simulado: só a fonte, ligada a uma saída, por um ciclo.".to_owned(),
        ),
        Some(Err(e)) => (
            estimate(),
            format!("Estimado pela tabela (a fonte sozinha não rodou: {e})."),
        ),
        None => {
            if st.arrivals.run.as_ref().is_none_or(|r| r.0 != key) {
                // Ask once for this table; the mark keeps it from asking every frame.
                st.arrivals.run = Some((key.to_owned(), Err("rodando".into())));
                out.push(PropAction::ArrivalsPreview);
            }
            (
                estimate(),
                "Estimado pela tabela; rodando a fonte...".to_owned(),
            )
        }
    };
    let r = ui.add(Label::new(small(&note)).wrap());
    named(ui, reg, &format!("{p}_previa_nota"), &r, &note);
    histogram(ui, reg, &format!("{p}_previa_grafico"), &h);
}

/// The preview's stacked bars, with the legend of the types.
fn histogram(ui: &mut Ui, reg: &mut Registry, name: &str, h: &Histogram) {
    let w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 90.0), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 3.0, colors::FIELD);
    let max = h
        .bins
        .iter()
        .map(|b| b.iter().sum::<f64>())
        .fold(0.0, f64::max)
        .max(1.0);
    let plot = rect.shrink2(Vec2::new(4.0, 6.0));
    let bw = plot.width() / h.bins.len() as f32;
    for (k, b) in h.bins.iter().enumerate() {
        let mut y = plot.bottom();
        for (j, v) in b.iter().enumerate() {
            if *v <= 0.0 {
                continue;
            }
            let hh = (*v / max) as f32 * plot.height();
            let x0 = plot.left() + k as f32 * bw;
            let r = Rect::from_min_max(Pos2::new(x0 + 0.5, y - hh), Pos2::new(x0 + bw - 0.5, y));
            painter.rect_filled(r, 0.0, type_color(j, &h.types[j]));
            y -= hh;
        }
    }
    let bin_label = if h.hours { "hora" } else { "faixa" };
    let tip: Vec<String> = h
        .bins
        .iter()
        .enumerate()
        .filter(|(_, b)| b.iter().sum::<f64>() > 0.0)
        .map(|(k, b)| format!("{bin_label} {}: {}", k, num_text(b.iter().sum())))
        .collect();
    let resp = resp.on_hover_text(tip.join("\n"));
    named(
        ui,
        reg,
        name,
        &resp,
        &format!("máximo {} por {bin_label}", num_text(max)),
    );
    if let Some(w) = reg.read(name).cloned() {
        reg.put_info(
            name,
            crate::egui_widgets::WidgetInfo {
                count: Some(h.bins.len()),
                ..w
            },
        );
    }
    ui.horizontal_wrapped(|ui| {
        for (j, ty) in h.types.iter().enumerate() {
            let (r, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
            ui.painter().rect_filled(r, 2.0, type_color(j, ty));
            ui.label(small(ty.as_deref().unwrap_or("tipo da fonte")));
        }
        ui.label(small(&format!(
            "por {bin_label} ({}); instantes da tabela, sem rodar o modelo",
            if h.hours {
                "eixo de 0 ao fim do ciclo".to_string()
            } else {
                format!("{} por faixa", num_text(h.bin))
            }
        )));
    });
}

fn schedule_editor(
    ui: &mut Ui,
    v: &PropsView,
    st: &mut PropsState,
    reg: &mut Registry,
    out: &mut Vec<PropAction>,
    s: &ArrivalSchedule,
) {
    let p = "Prop_schedule";
    let default = default_dist(v);
    let mut work = s.clone();
    let mut changed = false;
    let cycle = Some(s.cycle).filter(|c| c.is_finite() && *c > 0.0);
    let unit = unit_of_cycle(s.cycle);

    // Cycle.
    ui.horizontal(|ui| {
        ui.label(small("Ciclo"));
        let name = format!("{p}_ciclo");
        if let Some(typed) = line_edit(ui, st, reg, &name, &num_text(s.cycle), 80.0, false, "") {
            match parse_number(&typed) {
                Ok(x) => {
                    st.arrivals.cell_errors.remove(&name);
                    work.cycle = x;
                    changed = true;
                }
                Err(e) => {
                    st.arrivals.cell_errors.insert(name, e);
                }
            }
        }
        let note = match unit {
            Some(u) => format!("{} (horários em hh:mm, taxas por hora)", u.name()),
            None => "unidade do modelo".to_string(),
        };
        ui.label(small(&note));
    });

    // The band of the cycle.
    let w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 26.0), Sense::click_and_drag());
    // Dragging a block (an edge, or the whole period) shows the schedule being dragged and
    // sends it when the button is released (one undo step).
    if resp.drag_started()
        && let Some(q) = resp.interact_pointer_pos()
        && let Some((k, g)) = grab_at(&band(s, w), q.x - rect.left())
    {
        st.arrivals.drag = Some((k, g, q.x));
    }
    let dragged = match (st.arrivals.drag, resp.interact_pointer_pos()) {
        (Some((k, g, x0)), Some(q)) if w > 0.0 => {
            Some(drag_period(s, k, g, ((q.x - x0) / w) as f64 * s.cycle))
        }
        _ => None,
    };
    if resp.drag_stopped() {
        if let Some(d) = &dragged {
            work = d.clone();
            changed = true;
        }
        st.arrivals.drag = None;
    }
    let shown = dragged.clone().unwrap_or_else(|| s.clone());
    if let Some(q) = resp.hover_pos() {
        let cursor = match grab_at(&band(s, w), q.x - rect.left()) {
            Some((_, Grab::Whole)) => egui::CursorIcon::Grab,
            Some(_) => egui::CursorIcon::ResizeHorizontal,
            None => egui::CursorIcon::Default,
        };
        ui.ctx().set_cursor_icon(cursor);
    }
    let s = &shown;
    let blocks = band(s, w);
    let painter = ui.painter_at(rect.expand(1.0));
    let mut hovered = None;
    let pointer = resp.hover_pos();
    for b in &blocks {
        let r = Rect::from_min_max(
            Pos2::new(rect.left() + b.x0, rect.top()),
            Pos2::new(rect.left() + b.x1, rect.bottom()),
        );
        match b.period {
            Some(k) => {
                let fill = mode_color(mode_index(&s.periods[k].arrivals));
                painter.rect_filled(r.shrink2(Vec2::new(0.5, 0.0)), 2.0, fill);
                if r.width() > 14.0 {
                    painter.text(
                        r.center(),
                        egui::Align2::CENTER_CENTER,
                        (k + 1).to_string(),
                        egui::FontId::proportional(sizes::SMALL),
                        Color32::WHITE,
                    );
                }
                if pointer.is_some_and(|q| r.contains(q)) {
                    hovered = Some(k);
                }
            }
            None => {
                // Outside every period: no arrivals ("fora do horário"), hatched.
                painter.rect_filled(r, 0.0, colors::FIELD);
                let mut x = r.left() - r.height();
                while x < r.right() {
                    let a = Pos2::new(x.max(r.left()), r.bottom() - (x.max(r.left()) - x));
                    let end = (x + r.height()).min(r.right());
                    let bpt = Pos2::new(end, r.bottom() - (end - x));
                    painter.line_segment([a, bpt], Stroke::new(1.0, colors::BORDER));
                    x += 6.0;
                }
            }
        }
    }
    painter.rect_stroke(
        rect,
        2.0,
        Stroke::new(1.0, colors::BORDER),
        StrokeKind::Inside,
    );
    let band_tip = match hovered {
        Some(k) => period_text(s, k, &default),
        None => "Fora dos períodos a fonte não cria itens".to_string(),
    };
    let resp = resp.on_hover_text(band_tip);
    named(
        ui,
        reg,
        &format!("{p}_faixa"),
        &resp,
        &format!("{} período(s)", s.periods.len()),
    );
    // Ticks: start, quarters and end of the cycle.
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let tw = w / 5.0;
        for q in 0..5 {
            let t = s.cycle * q as f64 / 4.0;
            let text = time_text(t, cycle);
            let align = match q {
                0 => egui::Align::Min,
                4 => egui::Align::Max,
                _ => egui::Align::Center,
            };
            ui.allocate_ui_with_layout(
                Vec2::new(tw, 14.0),
                egui::Layout::left_to_right(egui::Align::Center).with_main_align(align),
                |ui| {
                    ui.label(small(&text));
                },
            );
        }
    });

    // One line per period.
    let (general, by_period) = split_errors(&s.validate(), "schedule");
    let full = ui.available_width();
    let gap = 4.0;
    let (wn, wt, wm, wb) = (18.0, 50.0, 92.0, 18.0);
    let wv = (full - (wn + 2.0 * (wt + EDIT_SLACK) + wm + wb + 5.0 * gap + EDIT_SLACK) - ROW_SLACK)
        .max(50.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        head(ui, "#", wn);
        head(ui, "início", wt + EDIT_SLACK);
        head(ui, "fim", wt + EDIT_SLACK);
        head(ui, "modo", wm);
        head(
            ui,
            if unit.is_some() {
                "itens/h ou distribuição"
            } else {
                "taxa ou distribuição"
            },
            wv + EDIT_SLACK,
        );
    });
    let mut remove = None;
    for (k, per) in s.periods.iter().enumerate() {
        let line = format!("{p}_periodo_{}", k + 1);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            let (r, _) = ui.allocate_exact_size(Vec2::new(wn, 22.0), Sense::hover());
            let mode = mode_index(&per.arrivals);
            ui.painter()
                .rect_filled(r.shrink(3.0), 2.0, mode_color(mode));
            ui.painter().text(
                r.center(),
                egui::Align2::CENTER_CENTER,
                (k + 1).to_string(),
                egui::FontId::proportional(sizes::SMALL),
                Color32::WHITE,
            );
            for (part, value) in [("inicio", per.start), ("fim", per.end)] {
                let name = format!("{line}_{part}");
                let shown = time_text(value, cycle);
                if let Some(typed) = line_edit(ui, st, reg, &name, &shown, wt, false, "") {
                    match parse_time(&typed, cycle) {
                        Ok(x) => {
                            st.arrivals.cell_errors.remove(&name);
                            if part == "inicio" {
                                work.periods[k].start = x;
                            } else {
                                work.periods[k].end = x;
                            }
                            changed = true;
                        }
                        Err(e) => {
                            st.arrivals.cell_errors.insert(name, e);
                        }
                    }
                }
            }
            let choices: Vec<(&str, &str)> = MODE_IDS.iter().copied().zip(MODES).collect();
            if let Some(m) = choose(
                ui,
                reg,
                &format!("{line}_modo"),
                MODE_IDS[mode],
                &choices,
                wm - COMBO_SLACK,
                "Padrão: o tempo entre chegadas da fonte. Distribuição: um tempo entre chegadas \
                 próprio. Taxa: chegadas de Poisson. Fechado: sem chegadas.",
            ) {
                let index = MODE_IDS.iter().position(|x| *x == m).unwrap_or(0);
                work.periods[k].arrivals = switch_mode(&per.arrivals, index, &default);
                changed = true;
            }
            let name = format!("{line}_valor");
            match &per.arrivals {
                PeriodArrivals::Rate(r) => {
                    let shown = num_text(rate_shown(*r, s.cycle));
                    if let Some(typed) = line_edit(ui, st, reg, &name, &shown, wv, false, "") {
                        match parse_number(&typed) {
                            Ok(x) => {
                                st.arrivals.cell_errors.remove(&name);
                                work.periods[k].arrivals =
                                    PeriodArrivals::Rate(rate_of_shown(x, s.cycle));
                                changed = true;
                            }
                            Err(e) => {
                                st.arrivals.cell_errors.insert(name, e);
                            }
                        }
                    }
                }
                PeriodArrivals::Interarrival(d) => {
                    if let Some(typed) = line_edit(ui, st, reg, &name, &ron::dist(d), wv, true, "")
                    {
                        match ron::parse::<Dist>(&typed) {
                            Ok(d) => {
                                st.arrivals.cell_errors.remove(&name);
                                work.periods[k].arrivals = PeriodArrivals::Interarrival(d);
                                changed = true;
                            }
                            Err(e) => {
                                st.arrivals
                                    .cell_errors
                                    .insert(name, format!("distribuição: {e}"));
                            }
                        }
                    }
                }
                PeriodArrivals::Default => {
                    let text = format!("média {}", num_text(default.mean()));
                    let r = ui.add_sized([wv, 22.0], Label::new(small(&text)));
                    named(ui, reg, &name, &r, &text);
                }
                PeriodArrivals::Closed => {
                    let r = ui.add_sized([wv, 22.0], Label::new(small("sem chegadas")));
                    named(ui, reg, &name, &r, "sem chegadas");
                }
            }
            if tiny(
                ui,
                reg,
                &format!("{line}_remover"),
                "×",
                "Remover o período",
                true,
            ) {
                remove = Some(k);
            }
        });
        let mut errors: Vec<String> = st
            .arrivals
            .cell_errors
            .iter()
            .filter(|(c, _)| c.starts_with(&format!("{line}_")))
            .map(|(_, e)| e.clone())
            .collect();
        errors.extend(by_period.get(&(k + 1)).cloned().unwrap_or_default());
        error_lines(ui, reg, &format!("{line}_erro"), &errors);
    }
    if let Some(k) = remove {
        work.periods.remove(k);
        st.arrivals.cell_errors.clear();
        changed = true;
    }
    let next = next_period(s);
    let r = ui
        .add_enabled(next.is_some(), Button::new("+ Período"))
        .on_hover_text("Acrescenta um período depois do último")
        .on_disabled_hover_text("O último período já vai até o fim do ciclo");
    named(ui, reg, &format!("{p}_mais"), &r, "+ Período");
    if r.clicked()
        && let Some(per) = next
    {
        work.periods.push(per);
        changed = true;
    }
    let mut errors = general;
    if let Some(e) = st.arrivals.cell_errors.get(&format!("{p}_ciclo")) {
        errors.push(e.clone());
    }
    error_lines(ui, reg, &format!("{p}_erros"), &errors);

    // Expected arrivals per period.
    let exp = expected(s, &default);
    if !exp.is_empty() {
        expected_chart(ui, reg, &format!("{p}_grafico"), s, &exp);
    }
    let total: f64 = exp.iter().sum();
    let text = format!(
        "Esperado por ciclo: {} chegadas",
        num_text((total * 10.0).round() / 10.0)
    );
    let r = ui.add(Label::new(RichText::new(&text).color(colors::TEXT_WEAK)).wrap());
    named(ui, reg, &format!("{p}_total"), &r, &text);

    if changed && work != *s {
        out.push(set("schedule", Value::Text(ron::schedule(&work))));
    }
}

/// What a period is, for the band's tooltip.
fn period_text(s: &ArrivalSchedule, k: usize, default: &Dist) -> String {
    let per = &s.periods[k];
    let cycle = Some(s.cycle);
    let how = match &per.arrivals {
        PeriodArrivals::Default => format!("padrão (média {})", num_text(default.mean())),
        PeriodArrivals::Interarrival(d) => format!("distribuição {}", ron::dist(d)),
        PeriodArrivals::Rate(r) => match unit_of_cycle(s.cycle) {
            Some(_) => format!("taxa {} itens/h", num_text(rate_shown(*r, s.cycle))),
            None => format!("taxa {}", num_text(*r)),
        },
        PeriodArrivals::Closed => "fechado".into(),
    };
    format!(
        "Período {}: {} a {}, {how}; esperadas {}",
        k + 1,
        time_text(per.start, cycle),
        time_text(per.end, cycle),
        num_text((s.expected_in_period(k, default) * 10.0).round() / 10.0)
    )
}

/// One bar per period with the expected arrivals, coloured by mode.
fn expected_chart(ui: &mut Ui, reg: &mut Registry, name: &str, s: &ArrivalSchedule, exp: &[f64]) {
    let w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 80.0), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 3.0, colors::FIELD);
    let max = exp.iter().copied().fold(0.0, f64::max).max(1e-9);
    let plot = Rect::from_min_max(
        Pos2::new(rect.left() + 4.0, rect.top() + 16.0),
        Pos2::new(rect.right() - 4.0, rect.bottom() - 14.0),
    );
    let bw = plot.width() / exp.len() as f32;
    for (k, v) in exp.iter().enumerate() {
        let x0 = plot.left() + k as f32 * bw;
        let hh = (*v / max) as f32 * plot.height();
        let bar = Rect::from_min_max(
            Pos2::new(x0 + bw * 0.15, plot.bottom() - hh),
            Pos2::new(x0 + bw * 0.85, plot.bottom()),
        );
        painter.rect_filled(bar, 2.0, mode_color(mode_index(&s.periods[k].arrivals)));
        painter.text(
            Pos2::new(bar.center().x, bar.top() - 2.0),
            egui::Align2::CENTER_BOTTOM,
            num_text((*v * 10.0).round() / 10.0),
            egui::FontId::proportional(sizes::SMALL),
            colors::TEXT,
        );
        painter.text(
            Pos2::new(bar.center().x, rect.bottom() - 1.0),
            egui::Align2::CENTER_BOTTOM,
            (k + 1).to_string(),
            egui::FontId::proportional(sizes::SMALL),
            colors::TEXT_WEAK,
        );
    }
    let resp = resp
        .on_hover_text("Chegadas esperadas em cada período (taxa × duração, ou duração ÷ média)");
    named(ui, reg, name, &resp, &format!("{} barra(s)", exp.len()));
    if let Some(w) = reg.read(name).cloned() {
        reg.put_info(
            name,
            crate::egui_widgets::WidgetInfo {
                count: Some(exp.len()),
                ..w
            },
        );
    }
}

// ---------------------------------------------------------------------------------------
// Dragging the blocks of the band.

/// What a press on the band grabbed: one edge of a period, or the whole block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grab {
    Start,
    End,
    Whole,
}

/// Points from an edge that grab it instead of the whole block.
const EDGE_POINTS: f32 = 6.0;

/// The period and the part of it under `x` (points from the band's left edge).
pub fn grab_at(blocks: &[Block], x: f32) -> Option<(usize, Grab)> {
    let b = blocks
        .iter()
        .filter(|b| b.period.is_some())
        .find(|b| x >= b.x0 - EDGE_POINTS && x <= b.x1 + EDGE_POINTS)?;
    let k = b.period?;
    let edge = EDGE_POINTS.min((b.x1 - b.x0) / 3.0);
    Some(if x <= b.x0 + edge {
        (k, Grab::Start)
    } else if x >= b.x1 - edge {
        (k, Grab::End)
    } else {
        (k, Grab::Whole)
    })
}

/// The step the band snaps to: a quarter of an hour when the cycle is a day or a week in a
/// known unit, else 1/96 of the cycle.
pub fn snap_step(cycle: f64) -> f64 {
    match unit_of_cycle(cycle) {
        Some(u) => u.per_hour() / 4.0,
        None => cycle / 96.0,
    }
}

fn snap(t: f64, step: f64) -> f64 {
    if step > 0.0 {
        (t / step).round() * step
    } else {
        t
    }
}

/// `s` with period `k` dragged by `dt` (model time): one edge moves (never past the other,
/// at least one step long), or the whole period moves keeping its length; inside the cycle,
/// snapped to [`snap_step`].
pub fn drag_period(s: &ArrivalSchedule, k: usize, grab: Grab, dt: f64) -> ArrivalSchedule {
    let mut out = s.clone();
    let Some(p) = out.periods.get_mut(k) else {
        return out;
    };
    let step = snap_step(s.cycle);
    let min_len = step.max(1e-9);
    let (a, b) = (p.start, p.end);
    match grab {
        Grab::Start => p.start = snap(a + dt, step).clamp(0.0, b - min_len),
        Grab::End => p.end = snap(b + dt, step).clamp(a + min_len, s.cycle),
        Grab::Whole => {
            let len = b - a;
            let start = snap(a + dt, step).clamp(0.0, (s.cycle - len).max(0.0));
            p.start = start;
            p.end = start + len;
        }
    }
    out
}

// ---------------------------------------------------------------------------------------
// "Ver as primeiras chegadas": running the source alone.

/// What a preview run of `source` is for: the RON of its table, else of its schedule (the
/// panel shows a run only for the table or schedule it was made with).
pub fn preview_key(model: &Model, source: &str) -> String {
    match model
        .objects
        .iter()
        .find(|o| o.name == source)
        .map(|o| &o.kind)
    {
        Some(ObjectKind::Source { table: Some(t), .. }) => ron::table(t),
        Some(ObjectKind::Source {
            schedule: Some(s), ..
        }) => ron::schedule(s),
        _ => String::new(),
    }
}

/// The arrivals of `source` run alone (connected to a sink, nothing else of the model) for
/// one cycle of its table or schedule (a day of the model's unit otherwise), binned per
/// hour and per item type. Unlike [`preview`], this runs the source: `release`, batches,
/// shifts and the random interarrivals are in it.
pub fn run_preview(model: &Model, source: &str) -> Result<Histogram, String> {
    let obj = model
        .objects
        .iter()
        .find(|o| o.name == source)
        .ok_or_else(|| format!("'{source}' não existe"))?;
    let ObjectKind::Source {
        interarrival,
        schedule,
        table,
        first_at_zero,
        ..
    } = &obj.kind
    else {
        return Err(format!("'{source}' não é uma chegada"));
    };
    // The span and the bins: those of the table's preview, else the schedule's cycle.
    let (bin, n, hours) = match (table, schedule) {
        (Some(t), _) => {
            let h = preview(t, interarrival, *first_at_zero);
            (h.bin, h.bins.len(), h.hours)
        }
        (None, Some(s)) if s.cycle.is_finite() && s.cycle > 0.0 => match unit_of_cycle(s.cycle) {
            Some(u) => (
                u.per_hour(),
                ((s.cycle / u.per_hour()).ceil() as usize).clamp(1, 24 * 7),
                true,
            ),
            None => (s.cycle / 24.0, 24, false),
        },
        _ => (60.0, 24, true),
    };
    let span = bin * n as f64;
    let sink_name = "previa_saida";
    let mut sink = Model::parse(&format!(
        "Model(name: \"p\", run: (stop_time: 1.0), objects: [(name: \"{sink_name}\", kind: Sink)], connections: [])"
    ))
    .map_err(|e| e.to_string())?
    .objects
    .remove(0);
    sink.position = obj.position;
    let mut alone = obj.clone();
    alone.transport = None;
    alone.downtime = None;
    alone.transfer = None;
    alone.on_entry = None;
    // One output port: a rule that picks among several (by label, by share) has no say.
    alone.output = Default::default();
    let mut m = model.clone();
    m.objects = vec![alone, sink];
    m.connections = vec![(source.to_owned(), sink_name.to_owned())];
    m.network = None;
    m.flows.clear();
    m.fluids = None;
    m.spans.clear();
    m.data
        .retain(|d| d.field.starts_with(&format!("{source}.")));
    m.floor_plan = None;
    m.obstacles.clear();
    m.aisles.clear();
    m.floor_marking = None;
    m.run.stop_time = span;
    m.run.warmup = 0.0;
    m.run.replications = 1;
    m.run.sample_interval = None;
    m.run.histogram_bins = None;
    let replay = m.replay(0).map_err(|e| e.to_string())?;
    // Created items, then their type (the trace says it right after).
    let mut types: Vec<Option<String>> = Vec::new();
    let mut bins = vec![Vec::<f64>::new(); n];
    let mut born: HashMap<u64, usize> = HashMap::new();
    let mut add = |k: usize, ty: Option<String>, sign: f64, bins: &mut Vec<Vec<f64>>| {
        let j = match types.iter().position(|x| *x == ty) {
            Some(j) => j,
            None => {
                types.push(ty);
                for b in bins.iter_mut() {
                    b.push(0.0);
                }
                types.len() - 1
            }
        };
        bins[k][j] += sign;
    };
    for r in &replay.records {
        match &r.event {
            TraceEvent::Created { item, .. } if r.t <= span => {
                let k = ((r.t / bin).floor().max(0.0) as usize).min(n - 1);
                born.insert(*item, k);
                add(k, None, 1.0, &mut bins);
            }
            TraceEvent::ItemType { item, item_type } => {
                if let Some(&k) = born.get(item) {
                    let name = model
                        .item_types
                        .get((*item_type as usize).saturating_sub(1))
                        .map(|d| d.name.clone());
                    // Counted as untyped at creation: move it to its type.
                    add(k, None, -1.0, &mut bins);
                    add(k, name, 1.0, &mut bins);
                }
            }
            _ => {}
        }
    }
    // Types left empty by the moves (the untyped column when every item has a type).
    let keep: Vec<usize> = (0..types.len())
        .filter(|&j| bins.iter().any(|b| b[j].abs() > 1e-9))
        .collect();
    Ok(Histogram {
        bin,
        types: keep.iter().map(|&j| types[j].clone()).collect(),
        bins: bins
            .iter()
            .map(|b| keep.iter().map(|&j| b[j]).collect())
            .collect(),
        hours,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(at: Option<f64>, count: u64, ty: Option<&str>) -> ArrivalRow {
        ArrivalRow {
            at,
            count,
            item_type: ty.map(str::to_string),
            labels: BTreeMap::new(),
        }
    }

    fn timetable() -> ArrivalTable {
        ArrivalTable {
            mode: TableMode::Times,
            repeat: TableRepeat::Cycle(1440.0),
            rows: vec![
                row(Some(480.0), 12, Some("caixa")),
                row(Some(600.0), 5, Some("palete")),
                row(Some(840.0), 8, Some("caixa")),
            ],
        }
    }

    #[test]
    fn times_show_as_clock_in_a_day() {
        assert_eq!(time_text(480.0, Some(1440.0)), "08:00");
        assert_eq!(time_text(480.5, Some(1440.0)), "480.5");
        assert_eq!(time_text(480.0, None), "480");
        assert_eq!(time_text(1500.0, Some(10080.0)), "dia 2 01:00");
        assert_eq!(parse_time("08:30", Some(1440.0)).unwrap(), 510.0);
        assert_eq!(parse_time("7,5", None).unwrap(), 7.5);
        assert!(parse_time("08:30", None).is_err());
        assert_eq!(parse_time("dia 2 01:00", Some(10080.0)).unwrap(), 1500.0);
    }

    #[test]
    fn labels_are_name_equals_value() {
        let l = parse_labels("prioridade=2; prazo = 1080,5").unwrap();
        assert_eq!(l["prioridade"], 2.0);
        assert_eq!(l["prazo"], 1080.5);
        assert_eq!(labels_text(&l), "prazo=1080.5; prioridade=2");
        assert_eq!(parse_labels(&labels_text(&l)).unwrap(), l);
        assert!(parse_labels("prioridade").is_err());
        assert!(parse_labels("=2").is_err());
        assert!(parse_labels("").unwrap().is_empty());
    }

    #[test]
    fn the_grid_goes_back_to_the_same_ron() {
        let mut t = timetable();
        t.rows[0].labels = parse_labels("prioridade=2").unwrap();
        let text = ron::table(&t);
        let back: ArrivalTable = ron::parse(&text).unwrap();
        assert_eq!(back, t);
        // Each cell read back from what the grid shows gives the same row.
        for r in &t.rows {
            let at = parse_time(&time_text(r.at.unwrap(), table_cycle(&t)), table_cycle(&t));
            assert_eq!(at.unwrap(), r.at.unwrap());
            assert_eq!(parse_count(&r.count.to_string()).unwrap(), r.count);
            assert_eq!(parse_labels(&labels_text(&r.labels)).unwrap(), r.labels);
        }
    }

    #[test]
    fn pasted_lines_become_rows() {
        // Semicolons, decimal comma, a header with a label column.
        let text = "hora;qtd;tipo;prioridade\n08:00;12;caixa;2\n\n10:00;5;palete;1,5\n";
        let rows = parse_paste(text, TableMode::Times, Some(1440.0)).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].at, Some(480.0));
        assert_eq!(rows[0].count, 12);
        assert_eq!(rows[0].item_type.as_deref(), Some("caixa"));
        assert_eq!(rows[1].labels["prioridade"], 1.5);
        // Tabs as a spreadsheet copies, no header, labels as nome=valor.
        let rows = parse_paste("480\t3\t\tprazo=900\n600,5\t1", TableMode::Times, None).unwrap();
        assert_eq!(rows[0].at, Some(480.0));
        assert_eq!(rows[0].item_type, None);
        assert_eq!(rows[0].labels["prazo"], 900.0);
        assert_eq!(rows[1].at, Some(600.5));
        // A sequence has no instants: count first.
        let rows =
            parse_paste("2;caixa\n1;palete;prioridade=3", TableMode::Sequence, None).unwrap();
        assert_eq!((rows[0].count, rows[0].at), (2, None));
        assert_eq!(rows[1].labels["prioridade"], 3.0);
        // A sequence ignores the instant column of a header.
        let rows = parse_paste("hora;qtd\n08:00;4", TableMode::Sequence, Some(1440.0)).unwrap();
        assert_eq!((rows[0].at, rows[0].count), (None, 4));
        // Errors give the line.
        let e = parse_paste("hora;qtd\n08:00;x", TableMode::Times, Some(1440.0)).unwrap_err();
        assert!(e.starts_with("linha 2:"), "{e}");
        let e = parse_paste("480;2,5", TableMode::Times, None).unwrap_err();
        assert!(e.contains("inteiro"), "{e}");
        assert!(parse_paste("", TableMode::Times, None).is_err());
        assert!(parse_paste("hora;qtd", TableMode::Times, None).is_err());
    }

    #[test]
    fn totals_per_cycle_and_per_type() {
        let t = timetable();
        let s = totals(&t);
        assert_eq!(s.items, 25);
        assert_eq!(
            s.by_type,
            vec![
                (Some("caixa".to_string()), 20),
                (Some("palete".to_string()), 5)
            ]
        );
        assert_eq!(
            totals_text(&t),
            "25 itens por ciclo de 1440 (um dia) em 3 linha(s) (caixa: 20, palete: 5)"
        );
        let mut seq = set_mode(&t, TableMode::Sequence, &Dist::Const(30.0));
        seq.rows[1].item_type = None;
        assert!(totals_text(&seq).contains("por volta"));
        assert!(totals_text(&seq).contains("tipo da fonte: 5"));
    }

    #[test]
    fn rows_are_added_sorted_and_the_mode_changes() {
        let mean = Dist::Exp { mean: 45.0 };
        let t = timetable();
        let t2 = add_row(&t, &mean);
        // The last gap (240) after the last instant.
        assert_eq!(t2.rows[3].at, Some(1080.0));
        assert_eq!(t2.rows[3].item_type.as_deref(), Some("caixa"));
        let mut t3 = t.clone();
        t3.rows.swap(0, 2);
        assert!(!is_sorted(&t3));
        assert_eq!(sort_rows(&t3), t);
        let seq = set_mode(&t, TableMode::Sequence, &mean);
        assert_eq!(seq.repeat, TableRepeat::Loop);
        assert!(seq.rows.iter().all(|r| r.at.is_none()));
        assert!(seq.validate().is_empty());
        let back = set_mode(&seq, TableMode::Times, &mean);
        assert_eq!(back.rows[1].at, Some(45.0));
        assert_eq!(back.repeat, TableRepeat::Cycle(1440.0));
        assert!(back.validate().is_empty());
        assert_eq!(set_repeat(&t, "Once").repeat, TableRepeat::Once);
        assert_eq!(
            set_repeat(&set_repeat(&t, "Once"), "Cycle").repeat,
            TableRepeat::Cycle(1440.0)
        );
        // The core's messages land on their row.
        let mut bad = t.clone();
        bad.rows[1].at = Some(100.0);
        let (general, by_row) = split_errors(&bad.validate(), "table.rows");
        assert!(general.is_empty());
        assert!(
            by_row[&2][0].contains("antes da linha anterior"),
            "{by_row:?}"
        );
    }

    #[test]
    fn the_preview_counts_per_hour_and_type() {
        let h = preview(&timetable(), &Dist::Const(60.0), false);
        assert!(h.hours);
        assert_eq!(h.bins.len(), 24);
        assert_eq!(h.types.len(), 2);
        assert_eq!(h.bins[8], vec![12.0, 0.0]);
        assert_eq!(h.bins[10], vec![0.0, 5.0]);
        assert_eq!(h.bins[14], vec![8.0, 0.0]);
        let total: f64 = h.bins.iter().flatten().sum();
        assert_eq!(total, 25.0);
        let seq = set_mode(&timetable(), TableMode::Sequence, &Dist::Const(60.0));
        let h = preview(&seq, &Dist::Const(60.0), true);
        assert_eq!(h.bins[0], vec![12.0, 0.0]);
        assert_eq!(h.bins[1], vec![0.0, 5.0]);
    }

    fn schedule() -> ArrivalSchedule {
        ron::parse(
            "(cycle: 1440.0, periods: [(start: 480.0, end: 720.0, arrivals: Rate(0.5)), \
             (start: 720.0, end: 780.0, arrivals: Closed), (start: 780.0, end: 1080.0)])",
        )
        .unwrap()
    }

    #[test]
    fn blocks_of_the_band_drag_by_edge_or_whole() {
        let s = schedule();
        let w = 1440.0;
        let blocks = band(&s, w);
        // A minute is a point here: edges within a few points, the middle is the block.
        assert_eq!(grab_at(&blocks, 481.0), Some((0, Grab::Start)));
        assert_eq!(grab_at(&blocks, 719.0), Some((0, Grab::End)));
        assert_eq!(grab_at(&blocks, 600.0), Some((0, Grab::Whole)));
        assert_eq!(grab_at(&blocks, 200.0), None);
        // A quarter of an hour in a day of minutes.
        assert_eq!(snap_step(1440.0), 15.0);
        let d = drag_period(&s, 0, Grab::Start, 40.0);
        assert_eq!((d.periods[0].start, d.periods[0].end), (525.0, 720.0));
        // An edge never passes the other one.
        let d = drag_period(&s, 0, Grab::End, -1000.0);
        assert_eq!(d.periods[0].end, 495.0);
        // The whole block keeps its length and stays in the cycle.
        let d = drag_period(&s, 2, Grab::Whole, 10_000.0);
        assert_eq!((d.periods[2].start, d.periods[2].end), (1140.0, 1440.0));
        assert_eq!(d.periods[1], s.periods[1]);
    }

    #[test]
    fn the_preview_runs_the_source_alone() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/chegadas_por_tabela.ron");
        let model = Model::load(&path).unwrap();
        // A timetable: the run creates exactly what the table says in a cycle.
        let ObjectKind::Source { table: Some(t), .. } = &model
            .objects
            .iter()
            .find(|o| o.name == "caminhoes")
            .unwrap()
            .kind
        else {
            panic!("a source with a table");
        };
        let h = run_preview(&model, "caminhoes").unwrap();
        let ran: f64 = h.bins.iter().flatten().sum();
        assert_eq!(ran, totals(t).items as f64, "{h:?}");
        assert!(
            h.types.iter().all(|t| t.is_some()),
            "typed items: {:?}",
            h.types
        );
        // The key is the table's text; another object is refused.
        assert_eq!(preview_key(&model, "caminhoes"), ron::table(t));
        assert!(run_preview(&model, "fila_caixas").is_err());
    }

    #[test]
    fn the_band_has_the_periods_and_the_gaps() {
        let b = band(&schedule(), 144.0);
        let got: Vec<(f32, f32, Option<usize>)> =
            b.iter().map(|b| (b.x0, b.x1, b.period)).collect();
        assert_eq!(
            got,
            vec![
                (0.0, 48.0, None),
                (48.0, 72.0, Some(0)),
                (72.0, 78.0, Some(1)),
                (78.0, 108.0, Some(2)),
                (108.0, 144.0, None),
            ]
        );
        let mut s = schedule();
        s.periods[1].end = s.periods[1].start;
        assert_eq!(
            band(&s, 144.0)
                .iter()
                .filter(|b| b.period == Some(1))
                .count(),
            0
        );
        s.cycle = 0.0;
        assert!(band(&s, 144.0).is_empty());
    }

    #[test]
    fn expected_arrivals_follow_the_core() {
        let s = schedule();
        let e = expected(&s, &Dist::Exp { mean: 10.0 });
        assert_eq!(e, vec![120.0, 0.0, 30.0]);
        // Rates show per hour in a day of minutes.
        assert_eq!(rate_shown(0.5, 1440.0), 30.0);
        assert_eq!(rate_of_shown(30.0, 1440.0), 0.5);
        assert_eq!(rate_shown(0.5, 100.0), 0.5);
        let next = next_period(&s).unwrap();
        assert_eq!((next.start, next.end), (1080.0, 1380.0));
        assert_eq!(
            switch_mode(&PeriodArrivals::Default, 2, &Dist::Exp { mean: 10.0 }),
            PeriodArrivals::Rate(0.1)
        );
    }

    /// The editors drawn in a panel: their names exist and an edit is one `Set` with the
    /// whole RON.
    mod panel {
        use super::super::*;
        use crate::document::Editor;
        use crate::ui_props::{PropsState, PropsView, WIDTH, show};
        use egui::{Context, Event, Modifiers, PointerButton, Pos2, RawInput};
        use std::collections::HashMap;

        fn source() -> (Editor, String) {
            let mut e = Editor::new("teste");
            let name = e.add("Source", [0.0, 0.0, 0.0]).unwrap();
            e.set_property(&name, "table.on", &Value::Bool(true), &[])
                .unwrap();
            (e, name)
        }

        fn view(e: &Editor, name: &str) -> PropsView {
            PropsView {
                props: e.properties(name),
                families: crate::schema::FAMILIES
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                data_paths: Vec::new(),
                extra: String::new(),
                driver: -1,
                miniature: false,
                data: HashMap::new(),
                data_families: Vec::new(),
                base_dir: None,
                abas: Vec::new(),
                somente_leitura: None,
                simulado: None,
            }
        }

        struct P {
            ctx: Context,
            st: PropsState,
            reg: Registry,
            time: f64,
        }

        impl P {
            fn new() -> P {
                let ctx = Context::default();
                utsim_ui_tema::install(&ctx);
                P {
                    ctx,
                    st: PropsState::default(),
                    reg: Registry::default(),
                    time: 0.0,
                }
            }

            fn frame(&mut self, v: &PropsView, events: Vec<Event>) -> Vec<PropAction> {
                self.time += 0.1;
                let input = RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1600.0, 3000.0))),
                    time: Some(self.time),
                    events,
                    ..RawInput::default()
                };
                let mut out = Vec::new();
                let (st, reg) = (&mut self.st, &mut self.reg);
                let rect =
                    Rect::from_min_max(Pos2::new(1600.0 - WIDTH, 46.0), Pos2::new(1600.0, 2960.0));
                self.ctx
                    .run_ui(input, |ui| {
                        reg.begin_frame();
                        out.clear();
                        show(ui, rect, v, st, reg, &mut out);
                    })
                    .drop_without_applying_deltas();
                out
            }

            fn at(&self, name: &str) -> Pos2 {
                let w = self
                    .reg
                    .get(name)
                    .unwrap_or_else(|| panic!("{name} ausente: {:?}", self.reg.names()));
                assert!(w.rect.is_positive(), "{name} fora da tela");
                w.rect.center()
            }

            fn click(&mut self, v: &PropsView, name: &str) -> Vec<PropAction> {
                let at = self.at(name);
                let mut out = self.frame(v, vec![Event::PointerMoved(at)]);
                out.extend(self.frame(v, vec![press(at, true)]));
                out.extend(self.frame(v, vec![press(at, false)]));
                out
            }

            fn type_in(
                &mut self,
                v: &PropsView,
                name: &str,
                text: &str,
                enter: bool,
            ) -> Vec<PropAction> {
                let mut out = self.click(v, name);
                out.extend(self.frame(v, vec![Event::Text(text.to_owned())]));
                if enter {
                    out.extend(self.frame(v, vec![key(egui::Key::Enter)]));
                }
                out.extend(self.frame(v, vec![]));
                out
            }
        }

        fn press(at: Pos2, pressed: bool) -> Event {
            Event::PointerButton {
                pos: at,
                button: PointerButton::Primary,
                pressed,
                modifiers: Modifiers::NONE,
            }
        }

        fn key(k: egui::Key) -> Event {
            Event::Key {
                key: k,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::NONE,
            }
        }

        fn table_of(out: &[PropAction]) -> ArrivalTable {
            let texts: Vec<&str> = out
                .iter()
                .filter_map(|a| match a {
                    PropAction::Set {
                        path,
                        value,
                        end_gesture: true,
                    } if path == "table" => value.as_text(),
                    _ => None,
                })
                .collect();
            assert_eq!(texts.len(), 1, "{out:?}");
            ron::parse(texts[0]).unwrap()
        }

        #[test]
        fn a_cell_edit_is_one_set_of_the_whole_table() {
            let (mut e, name) = source();
            let mut p = P::new();
            let v = view(&e, &name);
            p.frame(&v, vec![]);
            p.frame(&v, vec![]);
            for w in [
                "Prop_table_modo",
                "Prop_table_repetir",
                "Prop_table_ciclo",
                "Prop_table_linha_1_at",
                "Prop_table_linha_1_count",
                "Prop_table_linha_1_tipo",
                "Prop_table_linha_1_rotulos",
                "Prop_table_mais",
                "Prop_table_ordenar",
                "Prop_table_colar",
                "Prop_table_total",
                "Prop_table_texto",
            ] {
                p.at(w);
            }
            assert_eq!(p.reg.get("Prop_table_linha_1_at").unwrap().text, "08:00");
            let out = p.type_in(&v, "Prop_table_linha_1_count", "15", true);
            let t = table_of(&out);
            assert_eq!(t.rows[0].count, 15);
            // A cell that cannot be read sends nothing and says why on its line.
            let out = p.type_in(&v, "Prop_table_linha_1_at", "8h", true);
            assert!(
                out.iter().all(|a| !matches!(a, PropAction::Set { .. })),
                "{out:?}"
            );
            assert!(
                p.reg
                    .get("Prop_table_linha_1_erro")
                    .unwrap()
                    .text
                    .contains("8h")
            );
            // + Linha, then the bridge applies it.
            let out = p.click(&v, "Prop_table_mais");
            let t = table_of(&out);
            assert_eq!(t.rows.len(), 2);
            e.set_property(&name, "table", &Value::Text(ron::table(&t)), &[])
                .unwrap();
            let v = view(&e, &name);
            p.frame(&v, vec![]);
            // The whole line fits in the panel.
            p.at("Prop_table_linha_2_remover");
            let r = p.reg.get("Prop_table_linha_2_remover").unwrap().rect;
            assert!(r.max.x <= 1600.0 - 10.0 && r.width() >= 17.0, "{r:?}");
            // Pasting replaces the rows.
            p.click(&v, "Prop_table_colar");
            let out = p.type_in(
                &v,
                "Prop_table_colar_texto",
                "hora;qtd;prioridade\n09:00;3;1\n07:00;2;2",
                false,
            );
            assert!(out.is_empty(), "{out:?}");
            let out = p.click(&v, "Prop_table_colar_substituir");
            let t = table_of(&out);
            assert_eq!(t.rows.len(), 2);
            assert_eq!(t.rows[1].labels["prioridade"], 2.0);
            e.set_property(&name, "table", &Value::Text(ron::table(&t)), &[])
                .unwrap();
            // Out of order: the core's message on line 2, and Ordenar fixes it.
            let v = view(&e, &name);
            p.frame(&v, vec![]);
            assert!(
                p.reg
                    .get("Prop_table_linha_2_erro")
                    .unwrap()
                    .text
                    .contains("antes da linha anterior")
            );
            let out = p.click(&v, "Prop_table_ordenar");
            let t = table_of(&out);
            assert_eq!(t.rows[0].at, Some(420.0));
            // { } shows the RON line.
            p.click(&v, "Prop_table_texto");
            p.frame(&v, vec![]);
            assert!(
                p.reg
                    .get("Prop_table")
                    .unwrap()
                    .text
                    .starts_with("(mode: Times")
            );
            assert!(p.reg.get("Prop_table_linha_1_at").is_none());
        }

        #[test]
        fn the_schedule_editor_sends_the_whole_schedule() {
            let mut e = Editor::new("teste");
            let name = e.add("Source", [0.0, 0.0, 0.0]).unwrap();
            e.set_property(&name, "schedule.on", &Value::Bool(true), &[])
                .unwrap();
            let v = view(&e, &name);
            let mut p = P::new();
            p.frame(&v, vec![]);
            p.frame(&v, vec![]);
            for w in [
                "Prop_schedule_ciclo",
                "Prop_schedule_faixa",
                "Prop_schedule_periodo_1_inicio",
                "Prop_schedule_periodo_1_fim",
                "Prop_schedule_periodo_2_modo",
                "Prop_schedule_periodo_1_valor",
                "Prop_schedule_grafico",
                "Prop_schedule_total",
                "Prop_schedule_mais",
            ] {
                p.at(w);
            }
            let r = p.reg.get("Prop_schedule_periodo_2_remover").unwrap().rect;
            assert!(r.max.x <= 1600.0 - 10.0 && r.width() >= 17.0, "{r:?}");
            assert_eq!(p.reg.get("Prop_schedule_grafico").unwrap().count, Some(2));
            assert_eq!(
                p.reg.get("Prop_schedule_periodo_1_fim").unwrap().text,
                "08:00"
            );
            let out = p.type_in(&v, "Prop_schedule_periodo_1_fim", "09:00", true);
            let texts: Vec<&PropAction> = out
                .iter()
                .filter(|a| matches!(a, PropAction::Set { .. }))
                .collect();
            assert_eq!(texts.len(), 1);
            let PropAction::Set { path, value, .. } = texts[0] else {
                panic!()
            };
            assert_eq!(path, "schedule");
            let s: ArrivalSchedule = ron::parse(value.as_text().unwrap()).unwrap();
            assert_eq!(s.periods[0].end, 540.0);
            // A rate typed per hour is stored per minute.
            assert!(
                e.set_property(
                    &name,
                    "schedule",
                    &Value::Text(
                        "(cycle: 1440.0, periods: [(start: 0.0, end: 480.0, arrivals: Rate(0.5))])"
                            .into()
                    ),
                    &[]
                )
                .is_ok()
            );
            let v = view(&e, &name);
            p.frame(&v, vec![]);
            assert_eq!(
                p.reg.get("Prop_schedule_periodo_1_valor").unwrap().text,
                "30"
            );
            let out = p.type_in(&v, "Prop_schedule_periodo_1_valor", "6", true);
            let PropAction::Set { value, .. } = &out
                .iter()
                .find(|a| matches!(a, PropAction::Set { .. }))
                .unwrap()
            else {
                panic!()
            };
            let s: ArrivalSchedule = ron::parse(value.as_text().unwrap()).unwrap();
            assert_eq!(s.periods[0].arrivals, PeriodArrivals::Rate(0.1));
        }
    }
}
