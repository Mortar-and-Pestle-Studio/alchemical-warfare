use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

use crate::{asset::MapAssets, state::AppState};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(LevelSelection::index(0))
        .add_systems(OnEnter(AppState::Gameplay), spawn_map);
}

fn spawn_map(mut commands: Commands, map_assets: Res<MapAssets>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: map_assets.test.clone().into(),
            ..Default::default()
        },
        DespawnOnExit(AppState::Gameplay),
    ));
}
