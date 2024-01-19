use asteroid::AsteroidPlugin;
use bevy::prelude::*;

pub mod asteroid;
pub mod events;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugins((AsteroidPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
