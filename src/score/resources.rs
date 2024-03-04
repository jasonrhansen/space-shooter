use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Score {
    pub value: u32,
}
