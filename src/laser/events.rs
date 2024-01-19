use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnLaser {
    pub x: f32,
    pub y: f32,
    pub direction: Vec2,
}
