#![allow(clippy::type_complexity)]

use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod asteroid;
pub mod events;
pub mod laser;
pub mod osd;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

use events::*;
use laser::LaserPlugin;
use osd::OsdPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_state::<AppState>()
        .add_event::<GameOver>()
        .add_systems(
            Startup,
            (setup_physics, spawn_camera, spawn_background, spawn_music),
        )
        .add_plugins((
            AsteroidPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
            LaserPlugin,
            OsdPlugin,
        ))
        .add_systems(Update, (exit_game, handle_game_over, update_paused_state))
        .add_systems(
            Update,
            handle_physics_active.run_if(state_changed::<AppState>()),
        )
        .run();
}

pub fn update_paused_state(
    app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        if app_state.as_ref() == &AppState::Paused {
            next_app_state.set(AppState::Playing);
        } else if app_state.as_ref() == &AppState::Playing {
            next_app_state.set(AppState::Paused);
        }
    }
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Playing,
    Paused,
    GameOver,
}

pub fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    // Disable gravity
    rapier_config.gravity = Vec2::ZERO;
}

pub fn handle_physics_active(
    app_state: Res<State<AppState>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.physics_pipeline_active = app_state.as_ref() == &AppState::Playing;
}
