pub mod components;
pub mod resources;
pub mod systems;

use crate::{state::GameState, AppState, UpdateSet};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use systems::*;

use self::resources::UiAssets;

pub const MENU_BUTTON_WIDTH: f32 = 300.0;
pub const MENU_BUTTON_HEIGHT: f32 = 80.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppState::Loading).load_collection::<UiAssets>(),
        )
        .add_systems(
            Update,
            (
                update_health_text,
                update_score_text,
                resume_game_button_action
                    .run_if(in_state(AppState::Running).and_then(in_state(GameState::Paused))),
                (
                    button_interaction_color,
                    new_game_button_action,
                    quit_game_button_action,
                )
                    .run_if(
                        in_state(AppState::Running)
                            .and_then(in_state(GameState::Paused))
                            .or_else(in_state(AppState::GameOver)),
                    ),
            )
                .in_set(UpdateSet::Ui)
                .after(UpdateSet::Collision),
        )
        .add_systems(
            OnEnter(GameState::Paused),
            spawn_paused_screen.run_if(in_state(AppState::Running)),
        )
        .add_systems(
            OnExit(GameState::Paused),
            despawn_paused_screen.run_if(in_state(AppState::Running)),
        )
        .add_systems(OnEnter(AppState::GameOver), spawn_game_over_screen)
        .add_systems(OnExit(AppState::GameOver), despawn_game_over_screen);
    }
}
