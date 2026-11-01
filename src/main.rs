mod camera;
mod enemy;
mod input;
mod menu;
mod objective;
mod state;

use bevy::{asset::AssetMetaCheck, prelude::*};

fn main() -> AppExit {
    App::new().add_plugins(app_plugin).run()
}

fn app_plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Jam 8".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
    );

    app.add_plugins((
        camera::plugin,
        enemy::plugin,
        input::plugin,
        menu::plugin,
        objective::plugin,
        state::plugin,
    ));
}
