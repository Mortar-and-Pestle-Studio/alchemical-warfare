mod gameplay;
mod main;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::{
    game::{Game, GameState},
    menu::{
        gameplay::{game_lost, game_won, gameplay_menu},
        main::main_menu,
    },
    state::AppState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin::default())
        .add_systems(
            EguiPrimaryContextPass,
            main_menu.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            EguiPrimaryContextPass,
            (
                gameplay_menu,
                game_won.run_if(resource_exists_and(|game: &Game| {
                    *game.state() == GameState::Won
                })),
                game_lost.run_if(resource_exists_and(|game: &Game| {
                    *game.state() == GameState::Lost
                })),
            )
                .run_if(in_state(AppState::Gameplay)),
        );
}
