use bevy::prelude::*;

#[derive(Component)]
pub struct Asteroid {
    pub direction: Vec2,
    pub rotation_speed: f32,
}
