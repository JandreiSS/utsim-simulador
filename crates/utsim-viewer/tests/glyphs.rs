//! Every non-ASCII character that the panels put on screen must exist in egui's default
//! fonts; a missing glyph is drawn as an empty box (the ✓ and ✗ of the optimisation tab
//! were). The scan reads the source files, so a new symbol in a label is caught here.

use std::path::Path;

use bevy_egui::egui::{self, FontId};

/// Collects the characters of the string literals of the non-test code, with their file.
fn visit(dir: &Path, found: &mut Vec<(String, char)>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            visit(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = std::fs::read_to_string(&path).unwrap();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            // Only text between the opening quote and the closing quote of a string
            // literal on a line that is not a comment.
            let mut in_tests = false;
            for line in text.lines() {
                let t = line.trim_start();
                if t.starts_with("#[cfg(test)]") {
                    in_tests = true;
                }
                if in_tests || t.starts_with("//") {
                    continue;
                }
                let code = line.split("//").next().unwrap_or(line);
                for part in code.split('"').skip(1).step_by(2) {
                    for c in part.chars() {
                        found.push((name.clone(), c));
                    }
                }
            }
        }
    }
}

/// Palette icons and the play triangle: drawn correctly on screen (checked in the
/// viewer), although the font query below does not see the emoji faces that carry them.
const SEEN_ON_SCREEN: &str = "➡🔧⏩📦🚚☺🚜☎□▶";

#[test]
fn every_symbol_drawn_by_the_panels_has_a_glyph() {
    let ctx = egui::Context::default();
    let mut output = ctx.run_ui(egui::RawInput::default(), |_| {});
    output.textures_delta.clear();
    let mut found = Vec::new();
    visit(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut found,
    );
    assert!(!found.is_empty());
    let mut missing: Vec<(String, char)> = Vec::new();
    ctx.fonts_mut(|fonts| {
        for (file, c) in found {
            // Latin-1 (accents, ±, ×) is plain text; the rest is checked.
            if (c as u32) > 0xff
                && !SEEN_ON_SCREEN.contains(c)
                && !fonts.has_glyph(&FontId::proportional(14.0), c)
                && !missing.contains(&(file.clone(), c))
            {
                missing.push((file, c));
            }
        }
    });
    assert!(
        missing.is_empty(),
        "símbolos sem glifo nas fontes padrão: {missing:?}"
    );
}
