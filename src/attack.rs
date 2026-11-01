use bevy::prelude::*;

use crate::state::PauseState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        tick_attack_timer.run_if(in_state(PauseState::Running)),
    );
}

#[derive(Component, Clone, Debug, Default)]
pub struct AttackTimer(Timer);

impl AttackTimer {
    pub fn new(attack_cooldown: f32) -> Self {
        Self(Timer::from_seconds(attack_cooldown, TimerMode::Once))
    }

    pub fn is_ready(&self) -> bool {
        self.0.is_finished()
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }
}

fn tick_attack_timer(attack_timers: Query<&mut AttackTimer>, time: Res<Time>) {
    for mut attack_timer in attack_timers {
        attack_timer.0.tick(time.delta());
    }
}
