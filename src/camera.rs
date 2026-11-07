use bevy::{camera::Projection::Orthographic, prelude::*};
use bevy_ecs_ldtk::LevelEvent::{self, Transformed};
use bevy_ecs_ldtk::assets::{LdtkProject, LevelMetadataAccessor};

use crate::{
    asset::MapAssets,
    state::AppState,
};

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
    mut level_events: MessageReader<LevelEvent>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
    map_assets: Res<MapAssets>,
    project_data: Res<Assets<LdtkProject>>
) {
    for level_event in level_events.read() {
        if let Transformed(level_iid) = level_event
            && let Some(project) = project_data.get(&map_assets.test)
            && let Some(level) = project.get_raw_level_by_iid(level_iid.get()) {

            let x: f32 = (level.px_wid as f32) * 0.5;
            let y: f32 = (level.px_hei as f32) * 0.5;

            camera.translation = Vec3::new(x, y, 0.0);
        }
    }
}