//! Pure logic of the object list and of the selection of overlapping objects (no window,
//! no GPU, unit-tested): the list filter, and the state that makes repeated clicks on the
//! same spot walk through the objects under the cursor.

use utsim_model::Model;

use crate::properties::spec;

/// A click within this many pixels of the previous one counts as "the same spot".
pub const SAME_SPOT_PIXELS: f32 = 6.0;

/// Objects under the cursor (nearest first) and which one a click selected last.
#[derive(Clone, Debug, PartialEq)]
pub struct Cycle {
    /// Window position of the last click.
    pub at: [f32; 2],
    /// Names of the objects under the cursor, nearest first.
    pub stack: Vec<String>,
    /// Index into `stack` of the object that click selected.
    pub index: usize,
}

impl Cycle {
    /// The state after a click at `at` over `stack`: when it is the same spot over the
    /// same objects as the previous click, the next object of the stack (wrapping
    /// around); otherwise the nearest one. `stack` must not be empty.
    pub fn advance(previous: Option<&Cycle>, at: [f32; 2], stack: Vec<String>) -> Cycle {
        let index = match previous {
            Some(p) if p.stack == stack && same_spot(p.at, at) => (p.index + 1) % stack.len(),
            _ => 0,
        };
        Cycle { at, stack, index }
    }

    /// The object this click selects.
    pub fn current(&self) -> &str {
        &self.stack[self.index]
    }

    /// Whether the cursor at `at` over `stack` still belongs to this cycle (so the
    /// selection can be kept, e.g. to drag the object picked by the last click).
    pub fn holds(&self, at: [f32; 2], stack: &[String]) -> bool {
        self.stack == stack && same_spot(self.at, at)
    }
}

fn same_spot(a: [f32; 2], b: [f32; 2]) -> bool {
    (a[0] - b[0]).hypot(a[1] - b[1]) <= SAME_SPOT_PIXELS
}

/// Text of the tooltip over a stack of objects: the one a click selects now is marked.
pub fn stack_hint(stack: &[String], selected: Option<&str>) -> String {
    let list: Vec<String> = stack
        .iter()
        .map(|n| {
            if Some(n.as_str()) == selected {
                format!("[{n}]")
            } else {
                n.clone()
            }
        })
        .collect();
    format!(
        "{} objetos neste ponto: {}. Clique de novo para alternar.",
        stack.len(),
        list.join(", ")
    )
}

/// Indices of the objects whose name contains `text` (ignoring case) and, when `kind` is
/// given, whose kind is that one (Portuguese label of [`spec::kind_name`]).
pub fn filter_objects(model: &Model, text: &str, kind: Option<&str>) -> Vec<usize> {
    let wanted = text.trim().to_lowercase();
    model
        .objects
        .iter()
        .enumerate()
        .filter(|(_, o)| {
            (wanted.is_empty() || o.name.to_lowercase().contains(&wanted))
                && kind.is_none_or(|k| spec::kind_name(&o.kind) == k)
        })
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn repeated_clicks_walk_through_the_stack_and_wrap() {
        let stack = names(&["a", "b", "c"]);
        let first = Cycle::advance(None, [100.0, 100.0], stack.clone());
        assert_eq!(first.current(), "a");
        let second = Cycle::advance(Some(&first), [102.0, 99.0], stack.clone());
        assert_eq!(second.current(), "b");
        let third = Cycle::advance(Some(&second), [100.0, 100.0], stack.clone());
        assert_eq!(third.current(), "c");
        let again = Cycle::advance(Some(&third), [100.0, 100.0], stack);
        assert_eq!(again.current(), "a");
    }

    #[test]
    fn another_spot_or_another_stack_starts_over() {
        let stack = names(&["a", "b"]);
        let first = Cycle::advance(None, [100.0, 100.0], stack.clone());
        let moved = Cycle::advance(Some(&first), [140.0, 100.0], stack.clone());
        assert_eq!(moved.current(), "a");
        let other = Cycle::advance(Some(&first), [100.0, 100.0], names(&["a", "z"]));
        assert_eq!(other.current(), "a");
        assert!(first.holds([101.0, 100.0], &stack));
        assert!(!first.holds([160.0, 100.0], &stack));
    }

    #[test]
    fn the_hint_marks_the_selected_object() {
        let hint = stack_hint(&names(&["a", "b"]), Some("b"));
        assert!(hint.contains("a, [b]"), "{hint}");
        assert!(hint.starts_with("2 objetos"));
    }

    #[test]
    fn the_list_filters_by_name_and_kind() {
        let model = utsim_model::Model::parse(
            r#"Model(name: "t", run: (stop_time: 10.0), objects: [
                (name: "Fila_A", kind: Queue()),
                (name: "fila_b", kind: Queue()),
                (name: "Maq", kind: Processor(process: Const(1.0))),
                (name: "fim", kind: Sink),
            ], connections: [])"#,
        )
        .unwrap();
        assert_eq!(filter_objects(&model, "", None), vec![0, 1, 2, 3]);
        assert_eq!(filter_objects(&model, " FILA", None), vec![0, 1]);
        assert_eq!(filter_objects(&model, "", Some("Processo")), vec![2]);
        assert_eq!(
            filter_objects(&model, "fila", Some("Saída")),
            Vec::<usize>::new()
        );
    }
}
