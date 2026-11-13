//! The "Configurações" window of UTSim 3D: the preferences of this computer
//! ([`crate::preferencias`]). For now, whether the distribution fields open as the form or
//! as code.

use egui::{Align, Label, Layout, RichText};
use utsim_ui_tema::{colors, sizes};

use crate::egui_widgets::Registry;
use crate::texto::tx;
use crate::ui_props::named;

/// Shows the window; `codigo` is the default mode of the distribution fields. Returns
/// whether it changed, and pushes `configuracoes:fechar` when it is closed.
pub fn show(
    ctx: &egui::Context,
    codigo: &mut bool,
    reg: &mut Registry,
    out: &mut Vec<String>,
) -> bool {
    let mut changed = false;
    let mut close = false;
    let modal = egui::Modal::new(egui::Id::new("utsim_configuracoes"))
        .frame(
            egui::Frame::popup(&ctx.global_style())
                .fill(colors::PANEL)
                .inner_margin(egui::Margin::same(18)),
        )
        .show(ctx, |ui| {
            ui.set_min_width(360.0);
            ui.set_max_width(460.0);
            ui.spacing_mut().item_spacing.y = 10.0;
            ui.label(
                RichText::new(tx("configuracoes-titulo"))
                    .size(sizes::HEADING)
                    .strong()
                    .color(colors::HEADING),
            );
            ui.label(RichText::new(tx("configuracoes-distribuicoes")).color(colors::TEXT));
            ui.horizontal(|ui| {
                for (name, value, label) in [
                    (
                        "Config_Dist_Formulario",
                        false,
                        tx("configuracoes-formulario"),
                    ),
                    ("Config_Dist_Codigo", true, tx("configuracoes-codigo")),
                ] {
                    let r = ui.radio(*codigo == value, label);
                    named(ui, reg, name, &r, label);
                    if r.clicked() && *codigo != value {
                        *codigo = value;
                        changed = true;
                    }
                }
            });
            ui.add(
                Label::new(
                    RichText::new(tx("configuracoes-distribuicoes-ajuda"))
                        .small()
                        .color(colors::TEXT_WEAK),
                )
                .wrap(),
            );
            ui.add_space(4.0);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let r = ui.add(utsim_ui_tema::widgets::primary(tx("configuracoes-fechar")));
                named(ui, reg, "Config_Fechar", &r, tx("configuracoes-fechar"));
                close |= r.clicked();
            });
        });
    reg.put("Configuracoes", &modal.response, tx("configuracoes-titulo"));
    if close || modal.should_close() {
        out.push("configuracoes:fechar".into());
    }
    changed
}
