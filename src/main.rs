use asteroid::AsteroidPlugin;
use bevy::prelude::*;

pub mod asteroid;
pub mod events;
pub mod laser;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

use events::*;
use laser::LaserPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_systems(Startup, (spawn_camera, spawn_background))
        .add_plugins((
            AsteroidPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
            LaserPlugin,
        ))
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
