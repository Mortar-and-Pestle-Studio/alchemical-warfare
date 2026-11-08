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

pub enum CameraFocus {
    Min,
    Max,
}

pub struct CameraFocusLayer {
    grid_cols: i32,
    grid_rows: i32,
    grid_size: i32,
    tiles: Vec<Option<CameraFocus>>
}

pub struct Level {
    res_x: i32,
    res_y: i32,
    ground: GroundLayer,
    wall: WallLayer,
    spawns: SpawnsLayer,
    bases: BasesLayer,
    camera_focus: CameraFocusLayer,
}

#[derive(Default, Resource)]
pub struct LevelManager {
    level: Option<Level>
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
                    1 => Some(CameraFocus::Min),
                    2 => Some(CameraFocus::Max),
                    _ => None,
                })
                .collect(),
        }
        
    }

    pub fn from_ldtk_level(ldtk_level: &LdtkLevel) -> Self {
        CameraFocusLayer::new(
            ldtk_level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .find(|level| level.identifier == "CameraFocus")
                .unwrap()
        )
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
