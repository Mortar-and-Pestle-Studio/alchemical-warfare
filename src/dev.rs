mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::{dev::ui::gameplay_dev_menu, state::AppState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        EguiPrimaryContextPass,
        gameplay_dev_menu.run_if(in_state(AppState::Gameplay)),
    );
}
