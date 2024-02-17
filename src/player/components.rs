use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Player {
    pub direction: Vec2,
    pub velocity: Vec2,
    pub take_damage: bool,
}

#[derive(Component)]
pub struct DamageTime(pub Timer);

#[derive(Component)]
pub struct ForwardThruster;

#[derive(Component)]
pub struct BackwardThruster;
