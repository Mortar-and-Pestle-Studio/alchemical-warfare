use bevy::prelude::*;

use crate::state::AppState;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<ApplyDamage>()
        .add_systems(Update, apply_damage.run_if(in_state(AppState::Gameplay)));
}

#[derive(Component, Clone, Debug, Default)]
pub struct Health {
    current: u32,
    max: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self { current: max, max }
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn damage(&mut self, damage: u32) -> &mut Self {
        self.current = self.current.saturating_sub(damage);

        self
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct Dead;

#[derive(Message)]
pub struct ApplyDamage {
    pub amount: u32,
    pub target: Entity,
}

fn apply_damage(
    mut commands: Commands,
    mut apply_damages: MessageReader<ApplyDamage>,
    mut healths: Query<&mut Health>,
) {
    for apply_damage in apply_damages.read() {
        if let Ok(mut health) = healths.get_mut(apply_damage.target) {
            if health.damage(apply_damage.amount).is_dead() {
                commands.entity(apply_damage.target).insert(Dead);
            }
        }
    }
}
