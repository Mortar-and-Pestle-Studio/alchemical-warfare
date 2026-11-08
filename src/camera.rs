use bevy::{camera::Projection::Orthographic, prelude::*};

use crate::level::{Level, LevelManager, OnLevelLoadEvent};
use crate::state::AppState;

const CAMERA_ZOOM: f32 = 1.0 / 4.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, (on_level_load).run_if(in_state(AppState::Gameplay)));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Orthographic (
            OrthographicProjection {
                scale: CAMERA_ZOOM,
                ..OrthographicProjection::default_2d()
            }
        )
    ));
}

#[allow(clippy::cast_precision_loss)]
fn on_level_load(
    mut on_level_load_event: MessageReader<OnLevelLoadEvent>,
    level_manager: Res<LevelManager>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    for _ in on_level_load_event.read() {
        let level: &Level = level_manager.level.as_ref().unwrap();
        camera.translation = level.camera_focus.center.extend(-10.0);
    }
}