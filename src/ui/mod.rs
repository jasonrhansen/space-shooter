pub mod components;
pub mod resources;
pub mod systems;

use crate::{AppState, UpdateSet};
use bevy::prelude::*;
use systems::*;

pub const MENU_BUTTON_WIDTH: f32 = 300.0;
pub const MENU_BUTTON_HEIGHT: f32 = 80.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::UiAssets>()
            .add_systems(Startup, (setup, load_ui_assets))
            .add_systems(
                Update,
                (
                    update_health_text,
                    update_score_text,
                    resume_game_button_action.run_if(in_state(AppState::Paused)),
                    (
                        button_interaction_color,
                        new_game_button_action,
                        quit_game_button_action,
                    )
                        .run_if(in_state(AppState::Paused).or_else(in_state(AppState::GameOver))),
                )
                    .in_set(UpdateSet::Ui)
                    .after(UpdateSet::Collision),
            )
            .add_systems(OnEnter(AppState::Paused), spawn_paused_screen)
            .add_systems(OnExit(AppState::Paused), despawn_paused_screen)
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_screen)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_screen);
    }
}
