use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub direction: Vec2,
    pub velocity: Vec2,
}
