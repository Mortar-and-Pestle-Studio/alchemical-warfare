use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{
    enemy::Enemy,
    health::{ApplyDamage, Damage, DamageKind},
    potion::{Effect, Potion},
    tower::{SelectedTower, SetPotion, Tower},
};

#[derive(Resource)]
pub(super) struct PotionConfig(Potion);

impl Default for PotionConfig {
    fn default() -> Self {
        Self(Potion::basic())
    }
}

pub(super) fn gameplay_dev_menu(
    mut contexts: EguiContexts,
    enemies: Query<Entity, With<Enemy>>,
    mut apply_damage: MessageWriter<ApplyDamage>,
    mut time: ResMut<Time<Virtual>>,
    selected: Res<SelectedTower>,
    mut towers: Query<&mut Tower>,
    mut potion: Local<PotionConfig>,
    mut set_potion: MessageWriter<SetPotion>,
) -> Result {
    egui::Window::new("Dev Tools").show(contexts.ctx_mut()?, |ui| {
        if ui.button("Kill remaining enemies").clicked() {
            for enemy in enemies {
                apply_damage.write(ApplyDamage {
                    damage: Damage::new(u32::MAX, DamageKind::Physical),
                    target: enemy,
                });
            }
        }

        let mut time_mult = time.relative_speed();

        ui.add(egui::Slider::new(&mut time_mult, 1.0..=10.0).text("Game Speed"));

        time.set_relative_speed(time_mult);

        // let potion =

        if let Some(selected_tower) = selected.get()
            && let Ok(mut tower) = towers.get_mut(selected_tower)
        {
            ui.label("Tower selected");

            ui.label("Potion effect");

            for potion_effect in potion.0.effects_mut() {
                let effect = potion_effect.effect_mut();

                ui.radio_value(
                    effect,
                    Effect::Damage(Damage::new(1, DamageKind::Physical)),
                    "Damage",
                );

                ui.radio_value(
                    effect,
                    Effect::Dot {
                        damage: Damage::new(1, DamageKind::Fire),
                        tick_count: 4,
                        interval: 1.0,
                    },
                    "Dot",
                );

                ui.radio_value(
                    effect,
                    Effect::Slow {
                        mult: 0.5,
                        duration: 2.0,
                    },
                    "Slow",
                );

                match effect {
                    Effect::Damage(damage) => {
                        damage_ui(ui, damage);
                    }
                    Effect::Dot {
                        damage,
                        tick_count,
                        interval,
                    } => {
                        damage_ui(ui, damage);

                        ui.add(egui::Slider::new(tick_count, 1..=10).text("tick count"));
                        ui.add(egui::Slider::new(interval, 0.5..=2.5).text("interval (s)"));
                    }
                    Effect::Slow { mult, duration } => {
                        ui.add(egui::Slider::new(mult, 0.1..=1.0).text("multiplier"));
                        ui.add(egui::Slider::new(duration, 0.5..=2.5).text("duration"));
                    }
                    Effect::Stun { duration } => todo!(),
                };
            }

            if ui.button("Set potion").clicked() {
                set_potion.write(SetPotion {
                    tower: selected_tower,
                    potion: potion.0.clone(),
                });
            }
        }
    });

    Ok(())
}

fn damage_ui(ui: &mut egui::Ui, damage: &mut Damage) {
    ui.label("Damage Settings");
    ui.add(egui::Slider::new(damage.amount_mut(), 1..=20).text("damage"));

    ui.label("damage kind");
    ui.radio_value(damage.kind_mut(), DamageKind::Physical, "Physical");
    ui.radio_value(damage.kind_mut(), DamageKind::Fire, "Fire");
}
