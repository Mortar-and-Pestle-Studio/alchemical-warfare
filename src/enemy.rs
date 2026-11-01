use bevy::prelude::*;

use crate::{
    health::Health,
    objective::Objective,
    state::{AppState, PauseState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::Gameplay),
        spawn_enemy.run_if(in_state(PauseState::Running)),
    )
    .add_systems(
        Update,
        (move_to_objective, tick_attack_timer, attack_objective)
            .run_if(in_state(PauseState::Running)),
    );
}

#[derive(Component, Clone, Debug, Default)]
pub struct Enemy {
    speed: f32,
    damage: u32,
    range: f32,
}

#[derive(Component, Clone, Debug, Default)]
struct AttackTimer(Timer);

impl AttackTimer {
    fn new(attack_cooldown: f32) -> Self {
        Self(Timer::from_seconds(attack_cooldown, TimerMode::Once))
    }

    fn is_ready(&self) -> bool {
        self.0.is_finished()
    }

    fn reset(&mut self) {
        self.0.reset();
    }
}

fn spawn_enemy(mut commands: Commands) {
    commands.spawn_scene(enemy_scene());
}

fn enemy_scene() -> impl Scene {
    bsn! {
        Enemy {
            speed: 64.0,
            damage: 1,
            range: 32.0,
        }
        AttackTimer::new(1.0)
        Mesh2d(asset_value(Circle::new(32.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 0.0, 0.0)))
        Transform::from_xyz(0.0, 0.0, 0.0)
    }
}

fn move_to_objective(
    enemies: Query<(&Enemy, &mut Transform)>,
    obj_transform: Single<&Transform, (With<Objective>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for (enemy, mut enemy_transform) in enemies {
        let direction =
            (obj_transform.translation - enemy_transform.translation).normalize_or_zero();

        enemy_transform.translation += direction * enemy.speed * time.delta_secs();
    }
}

fn tick_attack_timer(attack_timers: Query<&mut AttackTimer, With<Enemy>>, time: Res<Time>) {
    for mut attack_timer in attack_timers {
        attack_timer.0.tick(time.delta());
    }
}

fn attack_objective(
    enemies: Query<(&Enemy, &mut AttackTimer, &Transform)>,
    mut obj: Single<(&Transform, &mut Health), (With<Objective>, Without<Enemy>)>,
) {
    for (enemy, mut attack_timer, enemy_tranform) in enemies {
        if !attack_timer.is_ready()
            || enemy_tranform.translation.distance(obj.0.translation) > enemy.range
        {
            continue;
        }

        obj.1.damage(enemy.damage);
        info!("Obj health: {}", obj.1.current());

        attack_timer.reset();
    }
}
