#![allow(clippy::type_complexity)]

use app_state::AppState;
use asteroid::AsteroidPlugin;
use bevy::{prelude::*, window::PresentMode};
use bevy_rapier2d::prelude::*;

pub mod app_state;
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

pub const VIEWPORT_WIDTH: f32 = 1280.0;
pub const VIEWPORT_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Shooter".into(),
                    present_mode: PresentMode::AutoVsync,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default().disabled(),
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
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
                update_paused_state,
                toggle_debug_render,
            ),
        )
        .add_systems(
            Update,
            handle_physics_active.run_if(state_changed::<AppState>()),
        )
        .run();
}
