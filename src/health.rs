use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Health {
    pub percent: u8,
}

impl Health {
    pub fn new(percent: u8) -> Self {
        Health { percent }
    }

    pub fn full() -> Self {
        Health { percent: 100 }
    }
}
