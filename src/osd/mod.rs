use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

use crate::AppState;

pub struct OsdPlugin;

impl Plugin for OsdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_score_text)
            .add_systems(OnEnter(AppState::Paused), spawn_paused_screen)
            .add_systems(OnExit(AppState::Paused), despawn_paused_screen);
    }
}
