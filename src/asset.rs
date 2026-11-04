use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};
use bevy_ecs_ldtk::assets::LdtkProject;

use crate::state::AppState;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(AppState::Loading)
            .continue_to_state(AppState::MainMenu)
            .load_collection::<MapAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct MapAssets {
    #[asset(path = "maps/test_0.ldtk")]
    pub test: Handle<LdtkProject>,
}
