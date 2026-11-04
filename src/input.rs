use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{state::AppState, tower::TryPlaceTower};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .add_input_context::<InputController>()
        .init_resource::<CursorWorldPos>()
        .add_systems(Update, cursor_world_pos)
        .add_systems(OnEnter(AppState::Gameplay), spawn_input_controller)
        .add_observer(on_place_tower);
}

#[derive(Resource, Default)]
pub struct CursorWorldPos(Vec2);

impl CursorWorldPos {
    pub fn get(&self) -> Vec2 {
        self.0
    }
}

#[derive(Component, Clone, Default)]
struct InputController;

#[derive(InputAction)]
#[action_output(bool)]
struct PlaceTower;

fn spawn_input_controller(mut commands: Commands) {
    commands.spawn_scene(input_controller_scene());
}

fn input_controller_scene() -> impl Scene {
    bsn! {
        InputController
        Actions<InputController> [
            Action::<PlaceTower>::new()
            Bindings [
                Binding(KeyCode::KeyT)
            ]
        ]
        DespawnOnExit<AppState>(AppState::Gameplay)
    }
}

fn on_place_tower(
    _place_tower: On<Start<PlaceTower>>,
    mut try_place_tower: MessageWriter<TryPlaceTower>,
) {
    try_place_tower.write(TryPlaceTower);
}

fn cursor_world_pos(
    mut cursor_world_pos: ResMut<CursorWorldPos>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = *camera;

    if let Some(window_pos) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, window_pos)
    {
        cursor_world_pos.0 = world_pos;
    }
}
