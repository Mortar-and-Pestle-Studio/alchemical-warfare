use bevy::prelude::*;

use crate::state::AppState;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<SpawnProjectile>().add_systems(
        Update,
        (spawn_projectiles, move_projectiles).run_if(in_state(AppState::Gameplay)),
    );
}

#[derive(Debug, Clone)]
pub struct ProjectileDef {
    damage: u32,
    speed: f32,
}

impl Default for ProjectileDef {
    fn default() -> Self {
        Self {
            damage: 1,
            speed: 64.0,
        }
    }
}

#[derive(Component, Clone)]
struct Projectile {
    damage: u32,
    speed: f32,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            damage: 1,
            speed: 64.0,
        }
    }
}

#[derive(Component, Clone)]
pub enum Target {
    Pos(Vec2),
}

impl Default for Target {
    fn default() -> Self {
        Self::Pos(Vec2::ZERO)
    }
}

#[derive(Message)]
pub struct SpawnProjectile {
    pub def: ProjectileDef,
    pub pos: Vec2,
    pub target: Target,
}

fn spawn_projectiles(mut commands: Commands, mut spawn_projectile: MessageReader<SpawnProjectile>) {
    for spawn_projectile in spawn_projectile.read() {
        commands.spawn_scene(projectile_scene(
            spawn_projectile.def.clone(),
            spawn_projectile.pos.clone(),
            spawn_projectile.target.clone(),
        ));
    }
}

fn projectile_scene(def: ProjectileDef, pos: Vec2, target: Target) -> impl Scene {
    bsn! {
        Projectile {
            damage: { def.damage },
            speed: { def.speed },
        }
        target
        Mesh2d(asset_value(Circle::new(8.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.0, 1.0, 1.0)))
        Transform::from_xyz(pos.x, pos.y, 0.0)
        DespawnOnExit<AppState>(AppState::Gameplay)
    }
}

fn move_projectiles(projectiles: Query<(&Projectile, &Target, &mut Transform)>, time: Res<Time>) {
    for (projectile, target, mut transform) in projectiles {
        let Target::Pos(target) = target;
        let dir = (target - transform.translation.truncate()).normalize_or_zero();

        transform.translation += (dir * projectile.speed * time.delta_secs()).extend(0.0);
    }
}
