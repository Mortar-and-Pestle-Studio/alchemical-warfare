use bevy::prelude::*;
use bevy_ecs_ldtk::LevelEvent::{self, Transformed};
use bevy_ecs_ldtk::assets::{LdtkProject, LevelMetadataAccessor};
use bevy_ecs_ldtk::ldtk::LayerInstance;
use bevy_ecs_ldtk::ldtk::Level as LdtkLevel;

use crate::{
    asset::MapAssets,
    state::AppState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (on_level_load).run_if(in_state(AppState::Gameplay)));
}

pub enum CollisionKind {
    Wall,
    Ground,
}

pub struct CollisionLayer {
    grid_cols: i32,
    grid_rows: i32,
    grid_size: i32,
    tiles: Vec<Option<CollisionKind>>
}

pub struct Level {
    res_x: i32,
    res_y: i32,
    collision: CollisionLayer
}

#[derive(Default, Resource)]
pub struct LevelManager {
    level: Option<Level>
}

impl CollisionLayer {
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    pub fn new(ldtk_layer: &LayerInstance) -> Self {
        // true-positive: correct layer
        // false-positive: incorrect layer that also utilizes a IntMap
        // true-negative: incorrect layer
        debug_assert_eq!(ldtk_layer.c_hei * ldtk_layer.c_wid, ldtk_layer.int_grid_csv.len() as i32);
        Self {
            grid_cols: ldtk_layer.c_wid,
            grid_rows: ldtk_layer.c_hei,
            grid_size: ldtk_layer.grid_size,
            tiles: ldtk_layer
                .int_grid_csv
                .iter()
                .map(|t| match t {
                    // 0 => None,
                    1 => Some(CollisionKind::Wall),
                    2 => Some(CollisionKind::Ground),
                    _ => None,
                })
                .collect(),
        }
        
    }

    pub fn from_ldtk_level(ldtk_level: &LdtkLevel) -> Self {
        CollisionLayer::new(
            ldtk_level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .find(|level| level.identifier == "Collision")
                .unwrap()
        )
    }
}

impl Level {
    pub fn new(ldtk_level: &LdtkLevel) -> Self {
        Self {
            res_x: ldtk_level.px_wid,
            res_y: ldtk_level.px_hei,
            collision: CollisionLayer::from_ldtk_level(ldtk_level),
        }
    }

    pub fn to_collision_index(self, position: &Vec3) -> Option<i32> {
        let grid_size: f32 = self.collision.grid_size as f32;
        let grid_y_offset: f32 = grid_size * (self.collision.grid_rows as f32);
        let col: i32 = f32::floor(position.x / (self.collision.grid_size as f32)) as i32;
        let row: i32 = f32::floor((grid_y_offset - position.y) / (self.collision.grid_size as f32)) as i32;
        if col < 0
            || col >= self.collision.grid_cols
            || row < 0
            || row >= self.collision.grid_rows {
            None    
        } else {
            Some(row * self.collision.grid_cols + col)
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn on_level_load(
    mut level_events: MessageReader<LevelEvent>,
    mut level_manager: ResMut<LevelManager>,
    map_assets: Res<MapAssets>,
    project_data: Res<Assets<LdtkProject>>
) {
    for level_event in level_events.read() {
        if let Transformed(level_iid) = level_event
            && let Some(project) = project_data.get(&map_assets.test)
            && let Some(ldtk_level) = project.get_raw_level_by_iid(level_iid.get()) {

            level_manager.level = Some(Level::new(ldtk_level));
        }
    }
}
