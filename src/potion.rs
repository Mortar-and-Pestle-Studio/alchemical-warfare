use std::num::NonZeroU8;

use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    health::{ApplyDamage, Damage, DamageKind},
    projectile::{Projectile, ProjectileDef, ReachedTarget, Target},
    state::AppState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<SpawnPotionProjectile>().add_systems(
        Update,
        (spawn_projectiles, potion_projectile_reached_target).run_if(in_state(AppState::Gameplay)),
    );
}

#[derive(Debug, Component, Clone)]
pub struct Potion {
    effects: Vec<PotionEffect>,
    aoe_radius: f32,
    pierce: u32,
}

impl Potion {
    pub fn basic() -> Self {
        Self {
            effects: vec![PotionEffect {
                level: NonZeroU8::new(1).unwrap(),
                strength: 1.0,
                effect: Effect::Damage(Damage::new(1, DamageKind::Physical)),
            }],
            aoe_radius: 64.0,
            pierce: 4,
        }
    }
    fn fire() -> Self {
        Self {
            effects: vec![PotionEffect {
                level: NonZeroU8::new(1).unwrap(),
                strength: 1.0,
                effect: Effect::Dot {
                    damage: Damage::new(1, DamageKind::Fire),
                    tick_count: 4,
                    interval: 0.5,
                },
            }],
            aoe_radius: 32.0,
            pierce: 4,
        }
    }

    pub fn level(&self) -> NonZeroU8 {
        self.effects
            .iter()
            .map(|effect| effect.level)
            .max()
            .expect("potion should have at least one effect")
    }

    pub fn concentration(&self) -> f32 {
        assert!(self.effects.len() > 0);

        1.0 / self.effects.len() as f32
    }
}

#[derive(Debug, Clone)]
pub struct PotionEffect {
    level: NonZeroU8,
    strength: f32,
    effect: Effect,
}

#[derive(Debug, Clone)]
pub enum Effect {
    // TODO: how to handle multiple potentially overlapping/stacking effects applied to enemies
    Damage(Damage),
    Dot {
        damage: Damage,
        tick_count: u32,
        interval: f32,
    },
    Slow {
        mult: f32,
        duration: f32,
    },
    Stun {
        duration: f32,
    },
}

#[derive(Debug, Clone)]
pub enum MixPotionsError {}

fn mix_potions(potion_a: Potion, potion_b: Potion) -> Result<Potion, MixPotionsError> {
    todo!()
}

#[derive(Message)]
pub struct SpawnPotionProjectile {
    pub potion: Potion,
    pub def: ProjectileDef,
    pub pos: Vec2,
    pub target: Target,
}

fn spawn_projectiles(
    mut commands: Commands,
    mut spawn_potion_projectile: MessageReader<SpawnPotionProjectile>,
) {
    for spawn_potion_projectile in spawn_potion_projectile.read() {
        commands.spawn_scene(potion_projectile_scene(
            spawn_potion_projectile.potion.clone(),
            spawn_potion_projectile.pos.clone(),
            spawn_potion_projectile.target.clone(),
        ));
    }
}

fn potion_projectile_scene(potion: Potion, pos: Vec2, target: Target) -> impl Scene {
    bsn! {
        Projectile
        target
        potion
        Mesh2d(asset_value(Circle::new(8.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 1.0, 0.0)))
        Transform::from_xyz(pos.x, pos.y, 0.0)
        DespawnOnExit<AppState>(AppState::Gameplay)
    }
}

fn potion_projectile_reached_target(
    mut commands: Commands,
    potions: Query<(Entity, &Potion, &Transform), Added<ReachedTarget>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut apply_damage: MessageWriter<ApplyDamage>,
) {
    for (potion_entity, potion, potion_transform) in potions {
        let enemies_in_range = enemies
            .iter()
            .filter(|(_, enemy_transform)| {
                enemy_transform
                    .translation
                    .truncate()
                    .distance(potion_transform.translation.truncate())
                    <= potion.aoe_radius
            })
            .take(potion.pierce as usize);

        for (enemy_entity, _enemy_transform) in enemies_in_range {
            for potion_effect in &potion.effects {
                match &potion_effect.effect {
                    Effect::Damage(damage) => {
                        apply_damage.write(ApplyDamage {
                            damage: damage.clone(),
                            target: enemy_entity,
                        });
                    }
                    Effect::Dot {
                        damage,
                        tick_count,
                        interval,
                    } => todo!(),
                    Effect::Slow { mult, duration } => todo!(),
                    Effect::Stun { duration } => todo!(),
                }
            }
        }

        commands.entity(potion_entity).despawn();
    }
}
