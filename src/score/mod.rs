use bevy::prelude::*;
use resources::*;
use systems::*;

use crate::state::AppState;

pub mod resources;
pub mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(AppState::Running), new_game_reset_score);
    }
}
