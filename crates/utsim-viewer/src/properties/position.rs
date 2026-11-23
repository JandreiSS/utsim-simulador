//! The "Posição" section of the property panel: x, y, z of the object (metres) and its
//! rotation about the vertical axis. Position is the model's own `position`; rotation is
//! the look's `yaw` (degrees, counter-clockwise seen from above), so it only turns what
//! is drawn, not how the object behaves in the simulation.

use crate::texto::tx;
use bevy_egui::egui::{Id, Ui};
use utsim_edit::Command;
use utsim_model::{ObjectDef, VisualDef};

use super::panel::num;

/// Draws the section and pushes the commands for what the user committed: one `Move`
/// for the position, one `SetVisual` for the rotation.
pub(crate) fn position_field(ui: &mut Ui, obj: &ObjectDef, out: &mut Vec<Command>) {
    let id = |axis: &str| Id::new(("utsim-props", &obj.name, "#pos", axis.to_string()));
    ui.strong(tx("prop-position-posicao_m"));
    let mut next = obj.position;
    let mut moved = false;
    ui.horizontal(|ui| {
        for (k, axis) in ["X", "Y", "Z"].into_iter().enumerate() {
            ui.label(axis);
            moved |= num(ui, id(axis), &mut next[k], 0.1, f64::MIN);
        }
    });
    if moved {
        out.push(Command::Move {
            name: obj.name.clone(),
            position: next,
        });
    }
    let current = obj.visual.clone().unwrap_or_default();
    ui.horizontal(|ui| {
        ui.label(tx("prop-position-giro_graus"));
        let mut yaw = current.yaw.unwrap_or(0.0);
        if num(ui, id("giro"), &mut yaw, 1.0, f64::MIN) {
            let visual = VisualDef {
                yaw: (yaw != 0.0).then_some(yaw),
                ..current.clone()
            };
            out.push(Command::SetVisual {
                name: obj.name.clone(),
                visual: Some(visual),
            });
        }
    });
}
