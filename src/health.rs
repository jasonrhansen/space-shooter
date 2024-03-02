use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Reflect, Default, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Health {
    #[inspector(min = 0, max = 100)]
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
