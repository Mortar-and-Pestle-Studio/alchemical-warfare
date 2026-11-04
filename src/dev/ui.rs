use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{enemy::Enemy, health::ApplyDamage};

pub(super) fn gameplay_dev_menu(
    mut contexts: EguiContexts,
    enemies: Query<Entity, With<Enemy>>,
    mut apply_damage: MessageWriter<ApplyDamage>,
    mut time: ResMut<Time<Virtual>>,
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

        let mut time_mult = time.relative_speed();

        ui.add(egui::Slider::new(&mut time_mult, 1.0..=10.0).text("Game Speed"));

        time.set_relative_speed(time_mult);
    });

    Ok(())
}
