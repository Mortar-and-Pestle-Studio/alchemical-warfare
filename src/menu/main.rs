use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::state::AppState;

pub(super) fn main_menu(
    mut contexts: EguiContexts,
    mut app_exit: MessageWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    egui::Window::new("Main Menu").show(contexts.ctx_mut()?, |ui| {
        if ui.button("Play").clicked() {
            next_state.set(AppState::Gameplay);
        }

        if ui.button("Quit").clicked() {
            app_exit.write(AppExit::Success);
        }
    });

    Ok(())
}
