use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Default, Resource)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}
