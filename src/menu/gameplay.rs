use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{health::Health, objective::Objective};

pub(super) fn gameplay_menu(
    mut contexts: EguiContexts,
    obj_health: Single<&Health, With<Objective>>,
) -> Result {
    egui::Window::new("Gameplay").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!("Objective Health: {}", obj_health.current()));
    });

    Ok(())
}
