use bevy::prelude::*;

use crate::{
    attack::AttackTimer,
    enemy::Enemy,
    input::CursorWorldPos,
    potion::{Potion, SpawnPotionProjectile},
    projectile::{ProjectileDef, Target},
    state::{AppState, PauseState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<TryPlaceTower>()
        .add_message::<SetPotion>()
        .init_resource::<SelectedTower>()
        .add_systems(OnEnter(AppState::Gameplay), spawn_tower)
        .add_systems(
            Update,
            (try_place_tower, set_tower_potion, attack_enemies)
                .run_if(in_state(PauseState::Running)),
        );
}

#[derive(Resource, Default)]
pub struct SelectedTower(Option<Entity>);

impl SelectedTower {
    pub fn get(&self) -> Option<Entity> {
        self.0
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct Tower {
    potion: Option<Potion>,
    range: f32,
}

impl Tower {
    pub fn range(&self) -> f32 {
        self.range
    }

    pub fn potion(&self) -> Option<&Potion> {
        self.potion.as_ref()
    }
}

#[derive(Message)]
pub struct TryPlaceTower;

#[derive(Message)]
pub struct SetPotion {
    pub tower: Entity,
    pub potion: Potion,
}

fn spawn_tower(mut commands: Commands) {
    commands.spawn_scene(tower_scene(Vec2::new(128.0, 128.0)));
}

fn tower_scene(pos: Vec2) -> impl Scene {
    bsn! {
        Tower {
            potion: Potion::fire(),
            range: 128.0,
        }
        on(select_tower)
        AttackTimer::new(1.0)
        Mesh2d(asset_value(Circle::new(32.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.0, 1.0, 0.0)))
        Transform::from_xyz(pos.x, pos.y, 0.0)
        DespawnOnExit<AppState>(AppState::Gameplay)
    }
}

fn select_tower(click: On<PointerClick>, mut selected: ResMut<SelectedTower>) {
    selected.0 = Some(click.entity);
}

fn attack_enemies(
    towers: Query<(&Tower, &mut AttackTimer, &Transform)>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Tower>)>,
    mut spawn_potion_projectile: MessageWriter<SpawnPotionProjectile>,
) {
    for (tower, mut attack_timer, tower_transform) in towers {
        if !attack_timer.is_ready() {
            continue;
        }

        let Some(potion) = &tower.potion else {
            continue;
        };

        let Some((_enemy_entity, nearest_enemy_transform)) =
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

        spawn_potion_projectile.write(SpawnPotionProjectile {
            def: ProjectileDef::default(),
            pos: tower_transform.translation.truncate(),
            target: Target::Pos(nearest_enemy_transform.translation.truncate()),
            potion: potion.clone(),
        });

        attack_timer.reset();
    }
}

fn try_place_tower(
    mut commands: Commands,
    mut try_place_tower: MessageReader<TryPlaceTower>,
    cursor_world_pos: Res<CursorWorldPos>,
) {
    for _try_place_tower in try_place_tower.read() {
        commands.spawn_scene(tower_scene(cursor_world_pos.get()));
    }
}

fn set_tower_potion(mut set_potion: MessageReader<SetPotion>, mut towers: Query<&mut Tower>) {
    for set_potion in set_potion.read() {
        if let Ok(mut tower) = towers.get_mut(set_potion.tower) {
            tower.potion = Some(set_potion.potion.clone());
        }
    }
}
