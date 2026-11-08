use bevy::prelude::*;
use bevy_ecs_ldtk::LevelEvent::{self, Transformed};
use bevy_ecs_ldtk::assets::{LdtkProject, LevelMetadataAccessor};
use bevy_ecs_ldtk::ldtk::LayerInstance;
use bevy_ecs_ldtk::ldtk::Level as LdtkLevel;
use bevy_ecs_ldtk::utils::{grid_coords_to_ldtk_grid_coords, grid_coords_to_translation, int_grid_index_to_grid_coords, translation_to_grid_coords};

use crate::{
    asset::MapAssets,
    state::AppState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (on_level_load).run_if(in_state(AppState::Gameplay)))
        .add_message::<OnLevelLoadEvent>();
}

pub enum GroundKind {
    All,
    PlayerOnly,
    EnemyOnly,
}

pub struct GroundLayer {
    grid_cols: i32,
    grid_rows: i32,
    grid_size: i32,
    tiles: Vec<Option<GroundKind>>
}

pub enum WallKind {
    All,
    PlayerOnly,
    EnemyOnly,
}

pub struct WallLayer {
    grid_cols: i32,
    grid_rows: i32,
    grid_size: i32,
    tiles: Vec<Option<WallKind>>
}

pub enum SpawnsKind {
    Enemy,
}

pub struct SpawnsLayer {
    grid_cols: i32,
    grid_rows: i32,
    grid_size: i32,
    tiles: Vec<Option<SpawnsKind>>
}

pub enum BaseKind {
    Standard,
}

pub struct BasesLayer {
    grid_cols: i32,
    grid_rows: i32,
    grid_size: i32,
    tiles: Vec<Option<BaseKind>>
}

#[derive(PartialEq)]
pub enum CameraFocus {
    Min,
    Max,
}

pub struct CameraFocusLayer {
    pub min: Vec2,
    pub max: Vec2,
    pub center: Vec2,
}

pub struct Level {
    pub res_x: i32,
    pub res_y: i32,
    pub ground: GroundLayer,
    pub wall: WallLayer,
    pub spawns: SpawnsLayer,
    pub bases: BasesLayer,
    pub camera_focus: CameraFocusLayer,
}

#[derive(Default, Resource)]
pub struct LevelManager {
    pub level: Option<Level>
}

#[derive(Message)]
pub struct OnLevelLoadEvent {}

impl GroundLayer {
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
                    1 => Some(GroundKind::All),
                    2 => Some(GroundKind::PlayerOnly),
                    3 => Some(GroundKind::EnemyOnly),
                    _ => None,
                })
                .collect(),
        }
        
    }

    pub fn from_ldtk_level(ldtk_level: &LdtkLevel) -> Self {
        GroundLayer::new(
            ldtk_level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .find(|level| level.identifier == "Ground")
                .unwrap()
        )
    }
}

impl WallLayer {
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
                    1 => Some(WallKind::All),
                    2 => Some(WallKind::PlayerOnly),
                    3 => Some(WallKind::EnemyOnly),
                    _ => None,
                })
                .collect(),
        }
        
    }

    pub fn from_ldtk_level(ldtk_level: &LdtkLevel) -> Self {
        WallLayer::new(
            ldtk_level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .find(|level| level.identifier == "Walls")
                .unwrap()
        )
    }
}

impl SpawnsLayer {
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
                    1 => Some(SpawnsKind::Enemy),
                    _ => None,
                })
                .collect(),
        }
        
    }

    pub fn from_ldtk_level(ldtk_level: &LdtkLevel) -> Self {
        SpawnsLayer::new(
            ldtk_level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .find(|level| level.identifier == "Spawns")
                .unwrap()
        )
    }
}

impl BasesLayer {
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
                    1 => Some(BaseKind::Standard),
                    _ => None,
                })
                .collect(),
        }
        
    }

    pub fn from_ldtk_level(ldtk_level: &LdtkLevel) -> Self {
        BasesLayer::new(
            ldtk_level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .find(|level| level.identifier == "Bases")
                .unwrap()
        )
    }
}

impl CameraFocusLayer {
    pub fn from_ldtk_level(ldtk_level: &LdtkLevel) -> Self {
        let ldtk_layer: &LayerInstance = ldtk_level
            .layer_instances
            .as_ref()
            .unwrap()
            .iter()
            .find(|level| level.identifier == "CameraFocus")
            .unwrap();

        // true-positive: correct layer
        // false-positive: incorrect layer that also utilizes a IntMap
        // true-negative: incorrect layer
        debug_assert_eq!(ldtk_layer.c_hei * ldtk_layer.c_wid, ldtk_layer.int_grid_csv.len() as i32);

        let mut min_index: Option<usize> = None;
        let mut max_index: Option<usize> = None;
        for (index, value) in ldtk_layer.int_grid_csv.iter().enumerate() {
            let value_enum = match value {
                1 => Some(CameraFocus::Min),
                2 => Some(CameraFocus::Max),
                _ => None,
            };
            if value_enum == Some(CameraFocus::Min) {
                if min_index.is_some() {
                    panic!("Camera min has already been found");
                } else {
                    min_index = Some(index);
                }
            } else if value_enum == Some(CameraFocus::Max) {
                if max_index.is_some() {
                    panic!("Camera max has already been found");
                } else {
                    max_index = Some(index);
                }
            }
        }

        let min_index = min_index.unwrap();
        let max_index = max_index.unwrap();
        let grid_size = ldtk_layer.grid_size;
        let half_grid_size: f32 = (grid_size as f32) * 0.5;
        let cols = ldtk_layer.c_wid;
        let rows = ldtk_layer.c_hei;
        let min_adjustment = Vec2::new(-half_grid_size, -half_grid_size);
        let max_adjustment = Vec2::new(half_grid_size, half_grid_size);

        let min = index_to_position(min_index, grid_size, cols, rows) + min_adjustment;
        let max = index_to_position(max_index, grid_size, cols, rows) + max_adjustment;
        let center = min + ((max - min) * 0.5);

        Self {
            min,
            max,
            center,
        }
    }
}

impl Level {
    pub fn new(ldtk_level: &LdtkLevel) -> Self {
        Self {
            res_x: ldtk_level.px_wid,
            res_y: ldtk_level.px_hei,
            ground: GroundLayer::from_ldtk_level(ldtk_level),
            wall: WallLayer::from_ldtk_level(ldtk_level),
            spawns: SpawnsLayer::from_ldtk_level(ldtk_level),
            bases: BasesLayer::from_ldtk_level(ldtk_level),
            camera_focus: CameraFocusLayer::from_ldtk_level(ldtk_level),
        }
    }

    pub fn to_ground_index(self, position: &Vec3) -> Option<i32> {
        let grid_size: f32 = self.ground.grid_size as f32;
        let grid_y_offset: f32 = grid_size * (self.ground.grid_rows as f32);
        let col: i32 = f32::floor(position.x / (self.ground.grid_size as f32)) as i32;
        let row: i32 = f32::floor((grid_y_offset - position.y) / (self.ground.grid_size as f32)) as i32;
        if col < 0
            || col >= self.ground.grid_cols
            || row < 0
            || row >= self.ground.grid_rows {
            None    
        } else {
            Some(row * self.ground.grid_cols + col)
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn on_level_load(
    mut level_events: MessageReader<LevelEvent>,
    mut on_level_load_event: MessageWriter<OnLevelLoadEvent>,
    mut level_manager: ResMut<LevelManager>,
    map_assets: Res<MapAssets>,
    project_data: Res<Assets<LdtkProject>>
) {
    for level_event in level_events.read() {
        if let Transformed(level_iid) = level_event
            && let Some(project) = project_data.get(&map_assets.test)
            && let Some(ldtk_level) = project.get_raw_level_by_iid(level_iid.get()) {

            // Parse LDtk level data
            level_manager.level = Some(Level::new(ldtk_level));
            // Notify listeners level has been loaded
            on_level_load_event.write(OnLevelLoadEvent {});
        }
    }
}

pub fn index_to_position(index: usize, grid_size: i32, cols: i32, rows: i32) -> Vec2 {
    let grid_size = IVec2::new(grid_size, grid_size);
    let grid_coords = int_grid_index_to_grid_coords(index, cols as u32, rows as u32).unwrap();
    grid_coords_to_translation(grid_coords, grid_size)
}

// Assumes grid is positioned at Vec3::ZERO
pub fn position_to_index(position: Vec2, grid_size: i32, cols: i32, rows: i32) -> usize {
    let grid_size = IVec2::new(grid_size, grid_size);
    let grid_coords = translation_to_grid_coords(position, grid_size);
    let (col, row) = grid_coords_to_ldtk_grid_coords(grid_coords, rows).into();
    assert!(col >= 0 && col < cols);
    assert!(row >= 0 && row < rows);
    (row * cols + col) as usize
}
