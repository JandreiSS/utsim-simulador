//! The specialised lines (`utsim_linhas`, `docs/plano_linhas_especializadas.md`, step 12) in
//! the editor: the presets of every built-in line as library entries (one category per
//! line), placing a preset as one undo step, the ghost shown while placing it, and the
//! line's vocabulary in the object list of a model of that line.
//!
//! A preset's entry id is `"<line>/<preset>"` (`"lean_office/posto"`), the value the
//! preset writes in the objects' `preset` field; the core library ids never have a `/`.

use utsim_edit::{Command, Document};
use utsim_linhas::{Pack, Placement, builtin, find};
use utsim_model::Model;
use utsim_model::replay::kind_name;

/// One preset in the editor's library.
#[derive(Clone, Debug, PartialEq)]
pub struct LineEntry {
    /// `"<line>/<preset>"`.
    pub id: String,
    /// The preset's name ("Posto de trabalho").
    pub label: String,
    /// The line's title ("Lean Office (processos administrativos)").
    pub category: String,
    /// Kind of the preset's main object ("Processor"): its icon is the core entry's.
    pub base_kind: &'static str,
}

/// The line and the preset of an entry id, when it is one.
fn split(id: &str) -> Option<(Pack, String)> {
    let (line, preset) = id.split_once('/')?;
    let pack = find(line)?;
    pack.preset(preset)?;
    Some((pack, preset.to_string()))
}

/// Every preset of every built-in line, in the order of each line's library.
pub fn entries() -> Vec<LineEntry> {
    let mut out = Vec::new();
    for pack in builtin() {
        let category = pack.library.title();
        for p in &pack.library.presets {
            let id = format!("{}/{}", pack.id(), p.id);
            let base_kind = preset_model(&id)
                .and_then(|m| m.objects.first().map(|o| kind_name(&o.kind)))
                .unwrap_or("Processor");
            out.push(LineEntry {
                id,
                label: p.name.clone(),
                category: category.clone(),
                base_kind,
            });
        }
    }
    out
}

/// The library categories of the lines (their titles), after the core ones.
pub fn categories() -> Vec<String> {
    builtin().iter().map(|p| p.library.title()).collect()
}

/// For a preset entry id: the name of its main object and the command that places the
/// preset at `position` with the default field values (`None`: not a preset id).
pub fn add_command(
    doc: &Document,
    id: &str,
    position: [f64; 3],
) -> Option<Result<(String, Command), String>> {
    let (pack, preset) = split(id)?;
    let at = Placement {
        preset: &preset,
        name: None,
        values: Default::default(),
        position,
        department: None,
    };
    Some(
        pack.placement_command(doc, &at)
            .map(|(names, command)| (names.into_iter().next().unwrap_or_default(), command)),
    )
}

/// The objects of a preset at the origin, in a model of its own (the placing ghost).
pub fn preset_model(id: &str) -> Option<Model> {
    let (line, _) = id.split_once('/')?;
    let mut doc = Document::new("fantasma");
    let (_, command) = add_command(&doc, id, [0.0; 3])?.ok()?;
    doc.apply(command).ok()?;
    // A line sets `line`; a cross-cutting pack (quality, safety) goes into `packs`.
    debug_assert!(
        doc.model().line.as_deref() == Some(line) || doc.model().packs.iter().any(|p| p == line)
    );
    Some(doc.model().clone())
}

/// The line's word for a core kind ("Processor" → "Posto de trabalho") in a model of a
/// built-in line; `None` outside a line or for a kind the vocabulary does not rename.
pub fn kind_label(model: &Model, kind: &str) -> Option<String> {
    let pack = find(model.line.as_deref()?)?;
    let term = pack.vocabulary.terms.get(kind)?;
    let mut chars = term.chars();
    let first = chars.next()?;
    Some(first.to_uppercase().chain(chars).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_is_an_entry_with_a_core_icon() {
        let list = entries();
        let total: usize = builtin().iter().map(|p| p.library.presets.len()).sum();
        assert_eq!(list.len(), total);
        let posto = list.iter().find(|e| e.id == "lean_office/posto").unwrap();
        assert_eq!(posto.label, "Posto de trabalho");
        assert_eq!(posto.base_kind, "Processor");
        assert_eq!(posto.category, "Lean Office (processos administrativos)");
        let equipe = list.iter().find(|e| e.id == "lean_office/equipe").unwrap();
        assert_eq!(equipe.base_kind, "Dispatcher");
        for e in &list {
            assert!(
                utsim_edit::PALETTE.iter().any(|p| p.id == e.base_kind),
                "{}: {}",
                e.id,
                e.base_kind
            );
        }
        // The lines and the cross-cutting packs (quality, safety), one category each.
        assert_eq!(categories().len(), builtin().len());
        assert_eq!(categories()[0], posto.category);
    }

    #[test]
    fn placing_a_preset_is_one_undo_step_and_sets_the_line() {
        let mut doc = Document::new("novo");
        let (name, command) = add_command(&doc, "lean_office/posto", [3.0, 1.0, 0.0])
            .unwrap()
            .unwrap();
        doc.apply(command).unwrap();
        assert_eq!(name, "posto");
        let m = doc.model();
        assert_eq!(m.line.as_deref(), Some("lean_office"));
        let o = &m.objects[0];
        assert_eq!(o.preset.as_deref(), Some("lean_office/posto"));
        assert_eq!(o.position, [3.0, 1.0, 0.0]);
        assert_eq!(
            o.visual.as_ref().and_then(|v| v.asset.as_deref()),
            Some("mesa")
        );
        // A second one gets a free name; one undo removes it whole.
        let (second, command) = add_command(&doc, "lean_office/equipe", [0.0; 3])
            .unwrap()
            .unwrap();
        doc.apply(command).unwrap();
        assert_eq!(second, "equipe");
        assert_eq!(doc.model().objects.len(), 4);
        doc.undo();
        assert_eq!(doc.model().objects.len(), 1);
        assert!(add_command(&doc, "Processor", [0.0; 3]).is_none());
        assert!(add_command(&doc, "lean_office/nada", [0.0; 3]).is_none());
    }

    #[test]
    fn ghost_and_vocabulary() {
        let ghost = preset_model("lean_office/reuniao").unwrap();
        assert_eq!(ghost.objects.len(), 2);
        let mut m = ghost.clone();
        assert_eq!(
            kind_label(&m, "Processor").as_deref(),
            Some("Posto de trabalho")
        );
        assert_eq!(kind_label(&m, "Queue").as_deref(), Some("Caixa de entrada"));
        assert_eq!(kind_label(&m, "Conveyor"), None);
        m.line = None;
        assert_eq!(kind_label(&m, "Processor"), None);
    }
}
