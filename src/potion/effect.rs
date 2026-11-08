use crate::{
    health::{ApplyDamage, Damage},
    state::AppState,
};

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_ignited, update_chilled).run_if(in_state(AppState::Gameplay)),
    );
}

#[derive(Component)]
pub struct Ignited {
    damage: Damage,
    ticks_remaining: u32,
    timer: Timer,
}

impl Ignited {
    pub(super) fn new(damage: Damage, tick_count: u32, interval: f32) -> Self {
        Self {
            damage,
            ticks_remaining: tick_count,
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Chilled {
    mult: f32,
    timer: Timer,
}

impl Chilled {
    pub(super) fn new(mult: f32, duration: f32) -> Self {
        Self {
            mult,
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }

    pub fn mult(&self) -> f32 {
        self.mult
    }
}

fn update_ignited(
    mut commands: Commands,
    ignited: Query<(Entity, &mut Ignited)>,
    time: Res<Time>,
    mut apply_damage: MessageWriter<ApplyDamage>,
) {
    for (entity, mut ignited) in ignited {
        if ignited.timer.tick(time.delta()).just_finished() {
            apply_damage.write(ApplyDamage {
                damage: ignited.damage.clone(),
                target: entity,
            });

            ignited.timer.reset();
            ignited.ticks_remaining -= 1;

            if ignited.ticks_remaining == 0 {
                commands.entity(entity).remove::<Ignited>();
            }
        }
    }
}

fn update_chilled(mut commands: Commands, chilled: Query<(Entity, &mut Chilled)>, time: Res<Time>) {
    for (entity, mut chilled) in chilled {
        if chilled.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).remove::<Chilled>();
        }
    }
}
