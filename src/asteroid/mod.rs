use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

use crate::AppState;

pub const NUM_ASTEROIDS: usize = 4;
pub const ASTEROID_SPEED: f32 = 50.0;
pub const ASTEROID_SIZE: f32 = 100.0;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_asteroids).add_systems(
            Update,
            (
                asteroid_movement,
                update_asteroid_direction,
                wrap_asteroid_movement,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}
