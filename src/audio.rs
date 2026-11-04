use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};

use crate::{asset::AudioAssets, health::DamageApplied};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin).add_systems(
        Update,
        play_audio_on_messages.run_if(resource_exists::<AudioAssets>),
    );
}

#[derive(Debug, Clone, Copy)]
pub enum AudioKind {
    Hit,
}

fn play_audio_on_messages(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut damage_applied: MessageReader<DamageApplied>,
) {
    for _damage_applied in damage_applied.read() {
        audio.play(audio_assets.get_handle(AudioKind::Hit));
    }
}
