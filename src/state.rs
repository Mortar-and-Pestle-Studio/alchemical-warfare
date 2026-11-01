use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<AppState>().add_sub_state::<PauseState>();
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Loading,
    Gameplay,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::Gameplay)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}
