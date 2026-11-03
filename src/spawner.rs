use bevy::prelude::*;

use crate::{
    enemy::SpawnEnemy,
    state::{AppState, PauseState},
    wave::SpawnGroup,
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<SpawnEnemies>()
        .add_message::<FinishedSpawningGroup>()
        .add_systems(OnEnter(AppState::Gameplay), spawn_spawner)
        .add_systems(
            Update,
            (on_spawn_enemies, tick_spawners).run_if(in_state(PauseState::Running)),
        );
}

#[derive(Message)]
pub struct SpawnEnemies(pub SpawnGroup);

#[derive(Message)]
pub struct FinishedSpawningGroup;

#[derive(Component, Clone, Default)]
struct Spawner {
    state: SpawnerState,
}

#[derive(Clone, Debug, Default)]
enum SpawnerState {
    #[default]
    Idle,
    Spawning {
        spawn_group: SpawnGroup,
        timer: Timer,
    },
}

fn spawn_spawner(mut commands: Commands) {
    commands.spawn_scene(spawner_scene());
}

fn spawner_scene() -> impl Scene {
    bsn! {
        Spawner
        Mesh2d(asset_value(Circle::new(32.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 1.0, 0.0)))
        Transform::from_xyz(-256.0, -256.0, -1.0)
    }
}

fn on_spawn_enemies(
    mut spawn_enemies: MessageReader<SpawnEnemies>,
    mut spawner: Single<&mut Spawner>,
) {
    for SpawnEnemies(spawn_group) in spawn_enemies.read() {
        if matches!(spawner.state, SpawnerState::Spawning { .. }) {
            warn!("Spawner currently spawning being overridden");
        }

        let mut timer = Timer::from_seconds(spawn_group.interval, TimerMode::Repeating);
        timer.almost_finish();

        spawner.state = SpawnerState::Spawning {
            spawn_group: spawn_group.clone(),
            timer,
        };
    }
}

fn tick_spawners(
    spawners: Query<(&mut Spawner, &Transform)>,
    time: Res<Time>,
    mut spawn_enemy: MessageWriter<SpawnEnemy>,
    mut finished_spawning_group: MessageWriter<FinishedSpawningGroup>,
) {
    for (mut spawner, transform) in spawners {
        match &mut spawner.state {
            SpawnerState::Idle => continue,
            SpawnerState::Spawning { spawn_group, timer } => {
                if timer.tick(time.delta()).just_finished() {
                    spawn_enemy.write(SpawnEnemy {
                        kind: spawn_group.enemy_kind,
                        pos: transform.translation.truncate(),
                    });

                    spawn_group.count -= 1;

                    if spawn_group.count == 0 {
                        finished_spawning_group.write(FinishedSpawningGroup);
                        spawner.state = SpawnerState::Idle;
                    }
                }
            }
        }
    }
}
