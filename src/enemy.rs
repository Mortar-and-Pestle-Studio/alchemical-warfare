use bevy::prelude::*;

use crate::{
    attack::AttackTimer,
    health::{ApplyDamage, Dead, Health},
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
        (move_to_objective, attack_objective, despawn_dead_enemies)
            .run_if(in_state(PauseState::Running)),
    );
}

#[derive(Component, Clone, Debug, Default)]
pub struct Enemy {
    speed: f32,
    damage: u32,
    range: f32,
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
        Health::new(10)
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

fn attack_objective(
    enemies: Query<(&Enemy, &mut AttackTimer, &Transform)>,
    mut apply_damage: MessageWriter<ApplyDamage>,
    obj: Single<(Entity, &Transform), (With<Objective>, Without<Enemy>)>,
) {
    for (enemy, mut attack_timer, enemy_tranform) in enemies {
        if !attack_timer.is_ready()
            || enemy_tranform.translation.distance(obj.1.translation) > enemy.range
        {
            continue;
        }

        apply_damage.write(ApplyDamage {
            amount: enemy.damage,
            target: obj.0,
        });

        attack_timer.reset();
    }
}

fn despawn_dead_enemies(mut commands: Commands, enemies: Query<(Entity, &Enemy), Added<Dead>>) {
    for (entity, enemy) in enemies {
        commands.entity(entity).despawn();
    }
}
