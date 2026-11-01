use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{enemy::Enemy, health::Health, objective::Objective};

pub(super) fn gameplay_menu(
    mut contexts: EguiContexts,
    obj_health: Single<&Health, With<Objective>>,
    enemy_healths: Query<&Health, With<Enemy>>,
) -> Result {
    egui::Window::new("Gameplay").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!(
            "Objective Health: {} / {}",
            obj_health.current(),
            obj_health.max()
        ));

        for enemy_health in enemy_healths {
            ui.label(format!(
                "Enemy Health: {} / {}",
                enemy_health.current(),
                enemy_health.max()
            ));
        }
    });

    Ok(())
}
