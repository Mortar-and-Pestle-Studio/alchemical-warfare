use crate::{
    health::{ApplyDamage, Damage},
    state::AppState,
};

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_ignited.run_if(in_state(AppState::Gameplay)));
}

#[derive(Component)]
pub(super) struct Ignited {
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
