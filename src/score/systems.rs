use super::resources::*;
use bevy::prelude::*;

pub fn new_game_reset_score(mut score: ResMut<Score>) {
    score.value = 0;
}
