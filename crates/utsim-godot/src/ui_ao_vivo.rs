//! The live results panel of Simular, in egui (pure: no Godot types, unit tested).
//!
//! A click on an object in the 3D view during the playback opens a small panel in the
//! bottom-right corner of the free area (above the playback bar, left of the Fluxos window
//! when it is open) with the numbers of that object at the playback time
//! ([`crate::ao_vivo`]): what is there now, what went through, the times and states, a line
//! chart of the content (and of the queue in front of a process) up to now and a bar of the
//! states in the colours of the 3D view.
//!
//! One panel follows the clicks (a click on another object replaces it); "Fixar" keeps a
//! panel, and the next click opens a new one beside it (at most [`MAX_PANELS`]). The X
//! closes a panel; Esc closes the most recent one, the one not pinned first.
//!
//! Names: `PainelVivo` (the panel), `PainelVivo_Titulo`, `PainelVivo_Nota`,
//! `PainelVivo_Fechar`, `PainelVivo_Fixar`, `PainelVivo_<chave>` for each line
//! (`PainelVivo_Processados`, `PainelVivo_EmProcesso`, `PainelVivo_Fila`,
//! `PainelVivo_Refugo`, `PainelVivo_Retrabalho`, `PainelVivo_Ciclo`, `PainelVivo_Espera`,
//! `PainelVivo_Ocupado`...), `PainelVivo_Grafico` and `PainelVivo_Estados` (their text is
//! the number of curves / states). The second panel on screen is `PainelVivo2`, the third
//! `PainelVivo3`.

use egui::{
    Align, Align2, Color32, FontId, Label, Layout, Rect, RichText, ScrollArea, Sense, Stroke,
    StrokeKind, Ui, UiBuilder, Vec2, pos2, vec2,
};
use utsim_ui_tema::{colors, sizes};

use crate::ao_vivo::Leitura;
use crate::egui_widgets::{Registry, WidgetInfo};
use crate::texto::tx;
use crate::ui_props::{named, named_toggle};
use crate::ui_results::{Chart, Curve, draw_chart, num};

/// Width and largest height of a panel, in points.
pub const WIDTH: f32 = 340.0;
pub const MAX_HEIGHT: f32 = 620.0;
/// Gap between two panels.
pub const GAP: f32 = 8.0;
/// Panels on screen at most (the oldest pinned one goes).
pub const MAX_PANELS: usize = 3;
/// The curves: content (WIP) and queue.
const CORES: [Color32; 2] = [
    Color32::from_rgb(90, 150, 240),
    Color32::from_rgb(250, 204, 89),
];

/// One panel on screen: the object, whether it is pinned, and its numbers now.
#[derive(Clone, Debug, PartialEq)]
pub struct Painel {
    pub nome: String,
    pub fixo: bool,
    /// Shown in a window of its own (`UtsimPainelVivo`), not over the 3D view: it stays
    /// open (as if pinned) and keeps being updated.
    pub solta: bool,
    pub leitura: Option<Leitura>,
}

/// The names of the `k`-th panel on screen (`PainelVivo`, `PainelVivo2`...).
pub fn prefixo(k: usize) -> String {
    if k == 0 {
        "PainelVivo".into()
    } else {
        format!("PainelVivo{}", k + 1)
    }
}

/// Opens the panel of `nome` in front: an open one comes to the front, the one not pinned
/// is replaced. True when the list changed.
pub fn abrir(lista: &mut Vec<Painel>, nome: &str) -> bool {
    if let Some(i) = lista.iter().position(|p| p.nome == nome) {
        // In a window of its own: it is already shown.
        if i == 0 || lista[i].solta {
            return false;
        }
        let p = lista.remove(i);
        lista.insert(0, p);
        return true;
    }
    lista.retain(|p| p.fixo || p.solta);
    lista.insert(
        0,
        Painel {
            nome: nome.into(),
            fixo: false,
            solta: false,
            leitura: None,
        },
    );
    // At most MAX_PANELS over the view; the ones in windows of their own do not count.
    let mut n = 0;
    lista.retain(|p| {
        if p.solta {
            return true;
        }
        n += 1;
        n <= MAX_PANELS
    });
    true
}

/// Esc: closes the panel not pinned, or else the most recent one. False when none is open.
pub fn fechar_topo(lista: &mut Vec<Painel>) -> bool {
    // The panels in windows of their own are closed by their windows.
    let Some(i) = lista
        .iter()
        .position(|p| !p.fixo && !p.solta)
        .or_else(|| lista.iter().position(|p| !p.solta))
    else {
        return false;
    };
    lista.remove(i);
    true
}

/// The panel of `nome` goes to a window of its own (`solta`) or back over the view. True
/// when it changed.
pub fn soltar(lista: &mut [Painel], nome: &str, solta: bool) -> bool {
    match lista.iter_mut().find(|p| p.nome == nome) {
        Some(p) if p.solta != solta => {
            p.solta = solta;
            true
        }
        _ => false,
    }
}

/// Runs a click of a panel (`vivo:fechar:<k>`, `vivo:fixar:<k>`); false for other actions.
pub fn acao(lista: &mut Vec<Painel>, a: &str) -> bool {
    let Some(rest) = a.strip_prefix("vivo:") else {
        return false;
    };
    let (what, k) = rest.split_once(':').unwrap_or((rest, ""));
    let Ok(k) = k.parse::<usize>() else {
        return true;
    };
    if k < lista.len() {
        match what {
            "fechar" => {
                lista.remove(k);
            }
            "fixar" => lista[k].fixo = !lista[k].fixo,
            "soltar" => lista[k].solta = !lista[k].solta,
            _ => {}
        }
    }
    true
}

/// Where the `n` panels go in `area` (points): side by side from the bottom-right corner,
/// the first one in the corner. Fewer when the area is narrow.
pub fn rects(area: Rect, n: usize) -> Vec<Rect> {
    let h = area.height().min(MAX_HEIGHT);
    if h < 160.0 {
        return Vec::new();
    }
    (0..n)
        .map(|k| {
            let right = area.max.x - k as f32 * (WIDTH + GAP);
            Rect::from_min_max(pos2(right - WIDTH, area.max.y - h), pos2(right, area.max.y))
        })
        .take_while(|r| r.min.x >= area.min.x)
        .collect()
}

/// Draws the `k`-th panel in `rect` (points).
pub fn show(
    ui: &mut Ui,
    rect: Rect,
    k: usize,
    p: &Painel,
    reg: &mut Registry,
    out: &mut Vec<String>,
) {
    let pre = prefixo(k);
    let painter = ui.painter().clone();
    painter.rect_filled(rect, sizes::RADIUS, colors::PANEL);
    painter.rect_stroke(
        rect,
        sizes::RADIUS,
        Stroke::new(
            if p.fixo { 1.5 } else { 1.0 },
            if p.fixo {
                colors::ACCENT
            } else {
                colors::BORDER
            },
        ),
        StrokeKind::Inside,
    );
    let panel = ui.allocate_rect(rect, Sense::hover());
    named(ui, reg, &pre, &panel, &p.nome);
    let inner = rect.shrink2(Vec2::new(12.0, 10.0));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(inner)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(inner.expand(2.0));
    let ui = &mut child;
    ui.spacing_mut().item_spacing = vec2(6.0, 4.0);
    let titulo = match &p.leitura {
        Some(l) => format!("{} ({})", l.nome, l.tipo),
        None => p.nome.clone(),
    };
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 26.0),
        Layout::right_to_left(Align::Center),
        |ui| {
            let r = ui
                .add(egui::Button::new("×").min_size(vec2(24.0, 22.0)))
                .on_hover_text(tx("vivo-fechar-dica"));
            named(ui, reg, &format!("{pre}_Fechar"), &r, tx("vivo-fechar"));
            if r.clicked() {
                out.push(format!("vivo:fechar:{k}"));
            }
            // A window of its own (that can go to another monitor), or back over the view.
            let (label, tip) = if p.solta {
                (tx("vivo-prender"), tx("vivo-prender-dica"))
            } else {
                (tx("vivo-soltar"), tx("vivo-soltar-dica"))
            };
            let r = ui.button(label).on_hover_text(tip);
            named(ui, reg, &format!("{pre}_Janela"), &r, label);
            if r.clicked() {
                out.push(format!("vivo:soltar:{k}"));
            }
            let r = ui
                .selectable_label(
                    p.fixo,
                    if p.fixo {
                        tx("vivo-fixado")
                    } else {
                        tx("vivo-fixar")
                    },
                )
                .on_hover_text(tx("vivo-fixar-dica"));
            named_toggle(
                ui,
                reg,
                &format!("{pre}_Fixar"),
                &r,
                tx("vivo-fixar"),
                p.fixo,
            );
            if r.clicked() {
                out.push(format!("vivo:fixar:{k}"));
            }
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let r = ui.add(
                    Label::new(
                        RichText::new(&titulo)
                            .size(sizes::HEADING)
                            .strong()
                            .color(colors::HEADING),
                    )
                    .truncate(),
                );
                named(ui, reg, &format!("{pre}_Titulo"), &r, &titulo);
            });
        },
    );
    let Some(l) = &p.leitura else {
        ui.label(RichText::new(tx("vivo-sem-numeros")).color(colors::TEXT_WEAK));
        return;
    };
    let r = ui.add(
        Label::new(
            RichText::new(&l.nota)
                .size(sizes::SMALL)
                .color(colors::TEXT_WEAK),
        )
        .wrap(),
    );
    named(ui, reg, &format!("{pre}_Nota"), &r, &l.nota);
    ScrollArea::vertical()
        .id_salt(("utsim_painel_vivo", k))
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for s in &l.secoes {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(s.titulo)
                        .size(sizes::SMALL)
                        .strong()
                        .color(colors::ACCENT),
                );
                for x in &s.linhas {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&x.rotulo).color(colors::TEXT_WEAK))
                            .on_hover_text(&x.ajuda);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let r = ui.label(RichText::new(&x.valor).color(colors::TEXT_STRONG));
                            named(ui, reg, &format!("{pre}_{}", x.chave), &r, &x.valor);
                        });
                    });
                }
            }
            ui.add_space(6.0);
            let chart = Chart::Lines {
                curves: l
                    .curvas
                    .iter()
                    .map(|(name, points, cor)| Curve {
                        name: name.clone(),
                        points: points.clone(),
                        color: CORES[*cor % CORES.len()],
                        width: 1.6,
                    })
                    .collect(),
                markers: Vec::new(),
                x: tx("vivo-eixo-tempo").into(),
                y: tx("vivo-eixo-itens").into(),
            };
            draw_chart(
                ui,
                reg,
                &format!("{pre}_Grafico"),
                &chart,
                tx("vivo-sem-dados"),
                120.0,
            );
            ui.add_space(6.0);
            estados(ui, reg, &format!("{pre}_Estados"), l);
        });
}

/// A bar of the states of the window, in the colours of the 3D view, and its legend.
fn estados(ui: &mut Ui, reg: &mut Registry, name: &str, l: &Leitura) {
    ui.label(
        RichText::new(tx("vivo-estados"))
            .size(sizes::SMALL)
            .strong()
            .color(colors::ACCENT),
    );
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 16.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, sizes::RADIUS, colors::FIELD);
    let total: f64 = l.estados.iter().map(|e| e.1).sum();
    let mut x = rect.min.x;
    let mut tip = Vec::new();
    for (s, f) in &l.estados {
        let w = if total > 0.0 {
            (f / total) as f32 * rect.width()
        } else {
            0.0
        };
        let c = utsim_scene::style::state_color(Some(*s));
        let cor = Color32::from_rgb(
            (c[0] * 255.0) as u8,
            (c[1] * 255.0) as u8,
            (c[2] * 255.0) as u8,
        );
        p.rect_filled(
            Rect::from_min_max(pos2(x, rect.min.y), pos2(x + w, rect.max.y)),
            0,
            cor,
        );
        x += w;
        tip.push((
            utsim_scene::style::state_label(Some(*s)).to_string(),
            *f,
            cor,
        ));
    }
    let resp = resp.on_hover_text(
        tip.iter()
            .map(|(n, f, _)| format!("{n}: {} %", num(f * 100.0)))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    named(ui, reg, name, &resp, &l.estados.len().to_string());
    if let Some(w) = reg.read(name).cloned() {
        reg.put_info(
            name,
            WidgetInfo {
                count: Some(l.estados.len()),
                ..w
            },
        );
    }
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for (n, f, cor) in tip {
            let (r, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
            ui.painter().circle_filled(r.center(), 4.5, cor);
            ui.label(
                RichText::new(format!("{n} {} %", num(f * 100.0)))
                    .size(sizes::SMALL)
                    .color(colors::TEXT),
            );
        }
    });
    if l.estados.is_empty() {
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            tx("vivo-sem-estados"),
            FontId::proportional(sizes::SMALL),
            colors::TEXT_WEAK,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_panel_in_its_own_window_stays_open_and_out_of_the_view() {
        let mut l = Vec::new();
        abrir(&mut l, "a");
        assert!(soltar(&mut l, "a", true));
        assert!(!soltar(&mut l, "a", true));
        // Another object: "a" stays (as if pinned) and does not come back over the view.
        abrir(&mut l, "b");
        assert_eq!(
            l.iter().map(|p| p.nome.as_str()).collect::<Vec<_>>(),
            ["b", "a"]
        );
        assert!(!abrir(&mut l, "a"));
        // Esc closes the one over the view, never the one in a window.
        assert!(fechar_topo(&mut l));
        assert!(!fechar_topo(&mut l));
        assert_eq!(l.len(), 1);
        // The panels in windows do not count in the limit over the view.
        for n in ["c", "d", "e", "f", "g"] {
            abrir(&mut l, n);
            acao(&mut l, "vivo:fixar:0");
        }
        assert_eq!(l.iter().filter(|p| !p.solta).count(), MAX_PANELS);
        assert!(l.iter().any(|p| p.nome == "a" && p.solta));
        // The button: to a window and back.
        let k = l.iter().position(|p| p.nome == "a").unwrap();
        assert!(acao(&mut l, &format!("vivo:soltar:{k}")));
        assert!(!l[k].solta);
    }

    fn nomes(l: &[Painel]) -> Vec<(&str, bool)> {
        l.iter().map(|p| (p.nome.as_str(), p.fixo)).collect()
    }

    #[test]
    fn a_click_replaces_the_panel_not_pinned_and_pinning_keeps_it() {
        let mut l = Vec::new();
        assert!(abrir(&mut l, "torno"));
        assert!(abrir(&mut l, "inspecao"));
        assert_eq!(nomes(&l), [("inspecao", false)]);
        assert!(acao(&mut l, "vivo:fixar:0"));
        abrir(&mut l, "torno");
        assert_eq!(nomes(&l), [("torno", false), ("inspecao", true)]);
        // The same object again: nothing changes.
        assert!(!abrir(&mut l, "torno"));
        // A pinned one clicked again comes to the front.
        assert!(abrir(&mut l, "inspecao"));
        assert_eq!(nomes(&l), [("inspecao", true), ("torno", false)]);
        // Esc: the one not pinned first, then the most recent.
        assert!(fechar_topo(&mut l));
        assert_eq!(nomes(&l), [("inspecao", true)]);
        assert!(fechar_topo(&mut l));
        assert!(!fechar_topo(&mut l));
        // Never more than MAX_PANELS.
        for n in ["a", "b", "c", "d"] {
            abrir(&mut l, n);
            acao(&mut l, "vivo:fixar:0");
        }
        assert_eq!(l.len(), MAX_PANELS);
        assert!(acao(&mut l, "vivo:fechar:1"));
        assert_eq!(l.len(), MAX_PANELS - 1);
        assert!(!acao(&mut l, "fluxos:fechar"));
    }

    #[test]
    fn the_panels_go_from_the_bottom_right_corner() {
        let area = Rect::from_min_max(pos2(300.0, 60.0), pos2(1500.0, 800.0));
        let r = rects(area, 2);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].max, area.max);
        assert!(r[1].max.x <= r[0].min.x - GAP + 0.01);
        assert!(r[0].height() <= MAX_HEIGHT);
        // A narrow area takes fewer.
        let narrow = Rect::from_min_max(pos2(0.0, 0.0), pos2(WIDTH + 10.0, 700.0));
        assert_eq!(rects(narrow, 3).len(), 1);
        assert!(rects(Rect::from_min_max(pos2(0.0, 0.0), pos2(900.0, 100.0)), 1).is_empty());
    }
}
