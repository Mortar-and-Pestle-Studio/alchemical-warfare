use bevy::prelude::*;

use crate::{
    attack::AttackTimer,
    enemy::Enemy,
    health::ApplyDamage,
    projectile::{ProjectileDef, SpawnProjectile, Target},
    state::{AppState, PauseState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Gameplay), spawn_tower)
        .add_systems(Update, attack_enemies.run_if(in_state(PauseState::Running)));
}

#[derive(Component, Clone, Debug, Default)]
pub struct Tower {
    damage: u32,
    range: f32,
}

fn spawn_tower(mut commands: Commands) {
    commands.spawn_scene(tower_scene());
}

fn tower_scene() -> impl Scene {
    bsn! {
        Tower {
            damage: 1,
            range: 128.0,
        }
        AttackTimer::new(1.0)
        Mesh2d(asset_value(Circle::new(32.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.0, 1.0, 0.0)))
        Transform::from_xyz(0.0, 128.0, 0.0)
        DespawnOnExit<AppState>(AppState::Gameplay)
    }
}

fn attack_enemies(
    towers: Query<(&Tower, &mut AttackTimer, &Transform)>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Tower>)>,
    mut spawn_projectile: MessageWriter<SpawnProjectile>,
) {
    for (tower, mut attack_timer, tower_transform) in towers {
        if !attack_timer.is_ready() {
            continue;
        }

        let Some((enemy_entity, nearest_enemy_transform)) =
            enemies
                .iter()
                .min_by(|(_, e_transform_a), (_, e_transform_b)| {
                    let a_dist = tower_transform
                        .translation
                        .distance_squared(e_transform_a.translation);

                    let b_dist = tower_transform
                        .translation
                        .distance_squared(e_transform_b.translation);

                    a_dist.partial_cmp(&b_dist).unwrap()
                })
        else {
            continue;
        };

        // TODO: spawn potion instead of directly damaging
        spawn_projectile.write(SpawnProjectile {
            def: ProjectileDef::default(),
            pos: tower_transform.translation.truncate(),
            target: Target::Pos(nearest_enemy_transform.translation.truncate()),
        });

        attack_timer.reset();
    }
}
