use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_kira_audio::AudioSource;

use crate::{audio::AudioKind, state::AppState};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(AppState::Loading)
            .continue_to_state(AppState::MainMenu)
            .load_collection::<MapAssets>()
            .load_collection::<AudioAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct MapAssets {
    #[asset(path = "maps/test_0.ldtk")]
    pub test: Handle<LdtkProject>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/hit_0.ogg")]
    pub hit: Handle<AudioSource>,
}

impl AudioAssets {
    pub fn get_handle(&self, audio_kind: AudioKind) -> Handle<AudioSource> {
        match audio_kind {
            AudioKind::Hit => self.hit.clone(),
        }
    }
}
