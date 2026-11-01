use bevy::prelude::*;

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
