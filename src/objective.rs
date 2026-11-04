use bevy::prelude::*;

use crate::{
    health::{Dead, Health},
    state::{AppState, PauseState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<ObjectiveDied>()
        .add_systems(
            OnEnter(AppState::Gameplay),
            spawn_objective.run_if(in_state(PauseState::Running)),
        )
        .add_systems(
            Update,
            detect_objective_death.run_if(in_state(PauseState::Running)),
        );
}

#[derive(Component, Clone, Debug, Default)]
pub struct Objective;

#[derive(Message)]
pub struct ObjectiveDied;

fn spawn_objective(mut commands: Commands) {
    commands.spawn_scene(enemy_scene());
}

fn enemy_scene() -> impl Scene {
    bsn! {
        Objective
        Health::new(10)
        Mesh2d(asset_value(Circle::new(64.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.0, 0.0, 1.0)))
        Transform::from_xyz(256.0, 64.0, 0.0)
        DespawnOnExit<AppState>(AppState::Gameplay)
    }
}

fn detect_objective_death(
    dead_objective: Query<Entity, (Added<Dead>, With<Objective>)>,
    mut objective_died: MessageWriter<ObjectiveDied>,
) {
    for _dead_objective in dead_objective {
        objective_died.write(ObjectiveDied);
    }
}
