use bevy::prelude::*;

#[derive(Component)]
pub struct Laser {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct LaserSound;
