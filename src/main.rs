#![allow(clippy::type_complexity)]

use app_state::AppState;
use asteroid::AsteroidPlugin;
use bevy::prelude::*;
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

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
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
