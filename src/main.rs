mod asset;
mod attack;
mod audio;
mod camera;
#[cfg(debug_assertions)]
mod dev;
mod enemy;
mod game;
mod health;
mod input;
mod level;
mod map;
mod menu;
mod objective;
mod potion;
mod projectile;
mod spawner;
mod state;
mod tower;
mod wave;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_ldtk::prelude::*;

use crate::level::LevelManager;

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
                    title: "Alchemical Warfare".to_string(),
                    fit_canvas_to_parent: true,
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins((
        asset::plugin,
        attack::plugin,
        audio::plugin,
        camera::plugin,
        enemy::plugin,
        game::plugin,
        health::plugin,
        input::plugin,
        map::plugin,
        menu::plugin,
        objective::plugin,
        potion::plugin,
        projectile::plugin,
        spawner::plugin,
        state::plugin,
    ))
    .add_plugins((tower::plugin, level::plugin, MeshPickingPlugin, LdtkPlugin))
    .init_resource::<LevelManager>();

    #[cfg(debug_assertions)]
    app.add_plugins(dev::plugin);
}
