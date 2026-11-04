use bevy::prelude::*;

use crate::{
    attack::AttackTimer,
    health::{ApplyDamage, Damage, DamageKind, Dead, Health},
    objective::Objective,
    state::{AppState, PauseState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<SpawnEnemy>()
        .add_message::<EnemyDied>()
        .add_systems(
            Update,
            (
                spawn_enemies,
                move_to_objective,
                attack_objective,
                despawn_dead_enemies,
            )
                .run_if(in_state(PauseState::Running)),
        );
}

#[derive(Component, Clone, Debug, Default)]
pub struct Enemy {
    speed: f32,
    damage: u32,
    range: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EnemyKind {
    Rat,
    Orc,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnemyDesc {
    kind: EnemyKind,
    health: u32,
    speed: f32,
    damage: u32,
    range: f32,
    attack_cooldown: f32,
}

// TODO: move enemy config to separate files
impl From<EnemyKind> for EnemyDesc {
    fn from(kind: EnemyKind) -> Self {
        match kind {
            EnemyKind::Rat => Self {
                kind,
                health: 5,
                speed: 128.0,
                damage: 1,
                range: 16.0,
                attack_cooldown: 1.0,
            },
            EnemyKind::Orc => Self {
                kind,
                health: 15,
                speed: 64.0,
                damage: 2,
                range: 32.0,
                attack_cooldown: 1.5,
            },
        }
    }
}

#[derive(Message)]
pub struct SpawnEnemy {
    pub kind: EnemyKind,
    pub pos: Vec2,
}

#[derive(Message)]
pub struct EnemyDied;

fn spawn_enemies(mut commands: Commands, mut spawn_enemies: MessageReader<SpawnEnemy>) {
    for spawn_enemy in spawn_enemies.read() {
        commands.spawn_scene(enemy_scene(spawn_enemy.kind.into(), spawn_enemy.pos));
    }
}

fn enemy_scene(desc: EnemyDesc, pos: Vec2) -> impl Scene {
    let color = match desc.kind {
        EnemyKind::Rat => Color::srgb(1.0, 0.0, 1.0),
        EnemyKind::Orc => Color::srgb(1.0, 0.0, 0.0),
    };

    bsn! {
        Enemy {
            speed: { desc.speed },
            damage: { desc.damage },
            range: { desc.range },
        }
        AttackTimer::new(desc.attack_cooldown)
        Health::new(desc.health)
        Mesh2d(asset_value(Circle::new(32.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(color))
        Transform::from_xyz(pos.x, pos.y, 0.0)
        DespawnOnExit<AppState>(AppState::Gameplay)
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
            damage: Damage::new(enemy.damage, DamageKind::Physical),
            target: obj.0,
        });

        attack_timer.reset();
    }
}

fn despawn_dead_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Enemy), Added<Dead>>,
    mut enemy_died: MessageWriter<EnemyDied>,
) {
    for (entity, _enemy) in enemies {
        enemy_died.write(EnemyDied);
        commands.entity(entity).despawn();
    }
}
