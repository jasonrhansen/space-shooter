pub mod components;
pub mod resources;
pub mod systems;

use self::resources::AsteroidAssets;
use crate::{state::GameState, AppState, UpdateSet};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
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
            .configure_loading_state(
                LoadingStateConfig::new(AppState::Loading).load_collection::<AsteroidAssets>(),
            )
            .add_systems(
                OnEnter(AppState::Running),
                new_game_spawn_asteroids.in_set(UpdateSet::Init),
            )
            .add_systems(
                Update,
                (wrap_asteroid_movement.in_set(UpdateSet::Movement),)
                    .run_if(in_state(AppState::Running).and_then(in_state(GameState::Playing))),
            );
    }
}
