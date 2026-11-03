use bevy::prelude::*;

use crate::{
    enemy::EnemyDied,
    spawner::{FinishedSpawningGroup, SpawnEnemies},
    state::{AppState, PauseState},
    wave::{Wave, Waves, WavesDef},
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<StartNextWave>()
        .add_message::<WaveStarted>()
        .add_message::<WaveFinished>()
        .add_systems(OnEnter(AppState::Gameplay), create_game)
        .add_systems(
            Update,
            (
                on_start_next_wave,
                on_finished_spawning_group,
                on_enemy_died,
            )
                .run_if(in_state(PauseState::Running)),
        );
}

#[derive(Resource)]
pub struct Game {
    state: GameState,
    waves: Waves,
}

impl Game {
    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn waves(&self) -> &Waves {
        &self.waves
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum GameState {
    #[default]
    Idle,
    WaveInProgress {
        wave: Wave,
        enemies_remaining: u32,
    },
}

#[derive(Message)]
pub struct StartNextWave;

#[derive(Message)]
pub struct WaveStarted;

#[derive(Message)]
pub struct WaveFinished;

fn on_start_next_wave(
    mut game: ResMut<Game>,
    mut start_next_wave: MessageReader<StartNextWave>,
    mut wave_started: MessageWriter<WaveStarted>,
    mut spawn_enemies: MessageWriter<SpawnEnemies>,
) {
    if game.state != GameState::Idle {
        return;
    }

    for _start_next_wave in start_next_wave.read() {
        let mut next_wave = game.waves.next_wave().clone();
        let next_spawn_group = next_wave.next_spawn_group();

        wave_started.write(WaveStarted);
        spawn_enemies.write(SpawnEnemies(next_spawn_group.clone()));

        game.state = GameState::WaveInProgress {
            enemies_remaining: next_wave.enemy_count(),
            wave: next_wave,
        };
    }
}

fn on_finished_spawning_group(
    mut game: ResMut<Game>,
    mut finished_spawning_group: MessageReader<FinishedSpawningGroup>,
    mut spawn_enemies: MessageWriter<SpawnEnemies>,
) {
    if let GameState::WaveInProgress {
        wave,
        enemies_remaining: _,
    } = &mut game.state
    {
        for _finished_spawning_group in finished_spawning_group.read() {
            if !wave.is_finished() {
                spawn_enemies.write(SpawnEnemies(wave.next_spawn_group().clone()));
            }
        }
    }
}

fn on_enemy_died(
    mut game: ResMut<Game>,
    mut enemy_died: MessageReader<EnemyDied>,
    mut wave_finished: MessageWriter<WaveFinished>,
) {
    let mut is_wave_finished = false;

    if let GameState::WaveInProgress {
        wave: _,
        enemies_remaining,
    } = &mut game.state
    {
        for _enemy_died in enemy_died.read() {
            *enemies_remaining -= 1;

            if *enemies_remaining == 0 {
                is_wave_finished = true;
            }
        }
    }

    if is_wave_finished {
        wave_finished.write(WaveFinished);
        info!("wave finished");

        game.state = GameState::Idle;
    }
}

fn create_game(mut commands: Commands) {
    commands.insert_resource(Game {
        state: GameState::Idle,
        waves: Waves::from(WavesDef::default()),
    });
}
