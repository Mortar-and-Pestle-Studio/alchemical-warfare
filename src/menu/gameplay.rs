use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{
    enemy::Enemy,
    game::{Game, GameState, StartNextWave},
    health::{Damage, Health},
    objective::Objective,
    potion::Effect,
    state::AppState,
    tower::{SelectedTower, Tower},
};

pub(super) fn gameplay_menu(
    mut contexts: EguiContexts,
    obj_health: Single<&Health, With<Objective>>,
    enemy_healths: Query<&Health, With<Enemy>>,
    game: Res<Game>,
    mut start_next_wave: MessageWriter<StartNextWave>,
    selected: Res<SelectedTower>,
    towers: Query<&Tower>,
) -> Result {
    egui::Window::new("Gameplay").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!(
            "Wave {} / {}",
            game.waves().current(),
            game.waves().count(),
        ));

        if ui
            .add_enabled(
                game.state() == &GameState::Idle,
                egui::Button::new("Start Next Wave"),
            )
            .clicked()
        {
            start_next_wave.write(StartNextWave);
        }

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

        if let Some(selected_tower) = selected.get()
            && let Ok(tower) = towers.get(selected_tower)
        {
            ui.add_space(16.0);

            ui.label("Tower selected");

            ui.add_space(8.0);

            if let Some(potion) = tower.potion() {
                ui.label("Potion Effects");

                for potion_effect in potion.effects() {
                    ui.label(format!("Level: {}", potion_effect.level()));
                    ui.label(format!("Strength: {}", potion_effect.strength()));

                    match potion_effect.effect() {
                        Effect::Damage(damage) => {
                            ui.label("Damage");

                            damage_ui(ui, damage);
                        }
                        Effect::Dot {
                            damage,
                            tick_count,
                            interval,
                        } => {
                            ui.label("Damage Over Time");

                            damage_ui(ui, damage);

                            ui.label(format!("Tick Count: {tick_count}"));
                            ui.label(format!("Interval: {interval}"));
                        }
                        Effect::Slow { mult, duration } => {
                            ui.label("Slow");

                            ui.label(format!("Multiplier: {mult}"));
                            ui.label(format!("Duration: {duration}"));
                        }
                        Effect::Stun { duration } => todo!(),
                    }
                }
            }
        }
    });

    Ok(())
}

fn damage_ui(ui: &mut egui::Ui, damage: &Damage) {
    ui.label(format!("Damage: {}", damage.amount()));
    ui.label(format!("Damage Kind: {:?}", damage.kind()));
}

pub(super) fn game_won(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    egui::Window::new("Game Won").show(contexts.ctx_mut()?, |ui| {
        ui.label("You win");

        if ui.button("Main Menu").clicked() {
            next_state.set(AppState::MainMenu);
        }
    });

    Ok(())
}

pub(super) fn game_lost(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    egui::Window::new("Game Lost").show(contexts.ctx_mut()?, |ui| {
        ui.label("You lose");

        if ui.button("Main Menu").clicked() {
            next_state.set(AppState::MainMenu);
        }
    });

    Ok(())
}
