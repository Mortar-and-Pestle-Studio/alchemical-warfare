use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub(super) fn main_menu(
    mut contexts: EguiContexts,
    mut app_exit: MessageWriter<AppExit>,
) -> Result {
    egui::Window::new("Main Menu").show(contexts.ctx_mut()?, |ui| {
        if ui.button("Quit").clicked() {
            app_exit.write(AppExit::Success);
        }
    });

    Ok(())
}
