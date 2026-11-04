use bevy::prelude::*;

use crate::enemy::EnemyKind;

#[derive(Debug, Clone)]
pub struct Waves {
    current: usize,
    waves: Vec<Wave>,
}

impl Waves {
    /// Starting wave returns one, not zero
    pub fn current(&self) -> usize {
        self.current
    }

    pub fn count(&self) -> usize {
        self.waves.len()
    }

    pub fn waves_remaining(&self) -> usize {
        self.waves.len() - self.current
    }

    pub fn next_wave(&mut self) -> &Wave {
        assert!(self.current != self.waves.len());

        let next_wave = &self.waves[self.current];
        self.current += 1;

        next_wave
    }
}

impl From<WavesDef> for Waves {
    fn from(waves_def: WavesDef) -> Self {
        Self {
            current: 0,
            waves: waves_def
                .wave_defs
                .into_iter()
                .map(|wave_def| wave_def.into())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wave {
    current: usize,
    spawn_groups: Vec<SpawnGroup>,
}

impl Wave {
    /// Starting spawn group returns one, not zero
    pub fn current(&self) -> usize {
        self.current
    }

    pub fn count(&self) -> usize {
        self.spawn_groups.len()
    }

    pub fn next_spawn_group(&mut self) -> &SpawnGroup {
        assert!(self.current != self.spawn_groups.len());

        let next_spawn_group = &self.spawn_groups[self.current];
        self.current += 1;

        next_spawn_group
    }

    pub fn is_finished(&self) -> bool {
        self.current == self.spawn_groups.len()
    }

    pub fn enemy_count(&self) -> u32 {
        self.spawn_groups
            .iter()
            .map(|spawn_group| spawn_group.count)
            .sum()
    }
}

impl From<WaveDef> for Wave {
    fn from(wave_def: WaveDef) -> Self {
        Self {
            current: 0,
            spawn_groups: wave_def.spawn_groups,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WavesDef {
    wave_defs: Vec<WaveDef>,
}

impl Default for WavesDef {
    fn default() -> Self {
        // TODO: Use ron files to configure waves
        Self {
            wave_defs: vec![
                WaveDef {
                    spawn_groups: vec![
                        SpawnGroup {
                            enemy_kind: EnemyKind::Orc,
                            count: 5,
                            interval: 1.5,
                        },
                        SpawnGroup {
                            enemy_kind: EnemyKind::Rat,
                            count: 4,
                            interval: 1.0,
                        },
                    ],
                },
                WaveDef {
                    spawn_groups: vec![
                        SpawnGroup {
                            enemy_kind: EnemyKind::Orc,
                            count: 10,
                            interval: 1.0,
                        },
                        SpawnGroup {
                            enemy_kind: EnemyKind::Rat,
                            count: 8,
                            interval: 0.75,
                        },
                    ],
                },
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaveDef {
    spawn_groups: Vec<SpawnGroup>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpawnGroup {
    pub enemy_kind: EnemyKind,
    pub count: u32,
    /// Time between individual enemy spawns in this group
    pub interval: f32,
}
