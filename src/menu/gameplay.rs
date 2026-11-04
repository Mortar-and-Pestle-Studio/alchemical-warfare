use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::{
    enemy::Enemy,
    game::{Game, GameState, StartNextWave},
    health::Health,
    objective::Objective,
    state::AppState,
};

pub(super) fn gameplay_menu(
    mut contexts: EguiContexts,
    obj_health: Single<&Health, With<Objective>>,
    enemy_healths: Query<&Health, With<Enemy>>,
    game: Res<Game>,
    mut start_next_wave: MessageWriter<StartNextWave>,
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
    });

    Ok(())
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
