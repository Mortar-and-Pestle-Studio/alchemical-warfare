mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::{dev::ui::gameplay_dev_menu, potion::Potion, projectile::Projectile, state::AppState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        EguiPrimaryContextPass,
        gameplay_dev_menu.run_if(in_state(AppState::Gameplay)),
    )
    .add_systems(Update, gizmos.run_if(in_state(AppState::Gameplay)));
}

fn gizmos(mut gizmos: Gizmos, potion_projectiles: Query<(&Transform, &Potion), With<Projectile>>) {
    for (transform, potion) in potion_projectiles {
        gizmos.circle_2d(
            Isometry2d::from_translation(transform.translation.truncate()),
            potion.aoe_radius(),
            Color::srgb(1.0, 0.5, 0.5),
        );
    }
}
