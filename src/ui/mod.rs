use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

use crate::AppState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                update_score_text.run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                Update,
                interact_with_resume_game_button.run_if(in_state(AppState::Paused)),
            )
            .add_systems(OnEnter(AppState::Paused), spawn_paused_screen)
            .add_systems(OnExit(AppState::Paused), despawn_paused_screen);
    }
}
