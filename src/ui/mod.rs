use bevy::prelude::*;
pub mod components;
pub mod systems;
use crate::AppState;
use systems::*;

pub const MENU_BUTTON_WIDTH: f32 = 300.0;
pub const MENU_BUTTON_HEIGHT: f32 = 80.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_health_text, update_score_text))
            .add_systems(
                Update,
                interact_with_resume_game_button.run_if(in_state(AppState::Paused)),
            )
            .add_systems(OnEnter(AppState::Paused), spawn_paused_screen)
            .add_systems(OnExit(AppState::Paused), despawn_paused_screen);
    }
}
