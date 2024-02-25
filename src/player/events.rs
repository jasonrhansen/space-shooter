use bevy::prelude::*;

#[derive(Event, Eq, PartialEq, Clone, Copy, Debug)]
pub enum PlayerThrusterChanged {
    Forward,
    Backward,
    None,
}

#[derive(Event)]
pub struct PlayerDeath;
