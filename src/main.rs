mod attack;
mod camera;
mod enemy;
mod game;
mod health;
mod input;
mod menu;
mod objective;
mod spawner;
mod state;
mod tower;
mod wave;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_ldtk::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(app_plugin)
        .add_systems(Startup, startup)
        .insert_resource(LevelSelection::index(0))
        .run()
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
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins((
        attack::plugin,
        camera::plugin,
        enemy::plugin,
        game::plugin,
        health::plugin,
        input::plugin,
        menu::plugin,
        objective::plugin,
        spawner::plugin,
        state::plugin,
        tower::plugin,
    ))
    .add_plugins(LdtkPlugin);
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((
    //     Camera2d,
    //     Projection::Orthographic(OrthographicProjection {
    //         scale: 0.5,
    //         ..OrthographicProjection::default_2d()
    //     }),
    //     Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    // ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/test_0.ldtk").into(),
        ..Default::default()
    });
}
