pub mod components;
pub mod resources;
pub mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::*;

pub const NUM_ASTEROIDS: usize = 4;
pub const ASTEROID_SPEED: f32 = 50.0;
pub const ASTEROID_SIZE: f32 = 100.0;

// Don't let asteroids spawn too close to the player.
pub const PLAYER_SAFE_RADIUS: f32 = 120.0;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::AsteroidCollisionConvexShapes>()
            .add_systems(
                Update,
                (new_game_spawn_asteroids, wrap_asteroid_movement)
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
