use bevy::prelude::*;

use crate::state::{AppState, PauseState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::Gameplay),
        spawn_objective.run_if(in_state(PauseState::Running)),
    );
}

#[derive(Component, Clone, Debug, Default)]
pub struct Objective;

fn spawn_objective(mut commands: Commands) {
    commands.spawn_scene(enemy_scene());
}

fn enemy_scene() -> impl Scene {
    bsn! {
        Objective
        Mesh2d(asset_value(Circle::new(64.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.0, 0.0, 1.0)))
        Transform::from_xyz(256.0, 64.0, 0.0)
    }
}
