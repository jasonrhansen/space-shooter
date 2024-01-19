use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub const NUM_ASTEROIDS: usize = 4;
pub const ASTEROID_SPEED: f32 = 20.0;
pub const ASTEROID_SIZE: f32 = 64.0;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_asteroids).add_systems(
            Update,
            ((
                asteroid_movement,
                update_asteroid_direction,
                wrap_asteroid_movement,
            )
                .chain(),),
        );
    }
}
