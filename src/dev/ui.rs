use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{enemy::Enemy, health::ApplyDamage};

pub(super) fn gameplay_dev_menu(
    mut contexts: EguiContexts,
    enemies: Query<Entity, With<Enemy>>,
    mut apply_damage: MessageWriter<ApplyDamage>,
) -> Result {
    egui::Window::new("Dev Tools").show(contexts.ctx_mut()?, |ui| {
        if ui.button("Kill remaining enemies").clicked() {
            for enemy in enemies {
                apply_damage.write(ApplyDamage {
                    amount: u32::MAX,
                    target: enemy,
                });
            }
        }
    });

    Ok(())
}
