pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use self::{events::PlayerThrusterChanged, resources::PlayerCollisionConvexShapes};
use crate::AppState;
use bevy::prelude::*;
use systems::*;

pub const PLAYER_SIZE: f32 = 75.0;
pub const PLAYER_MAX_SPEED: f32 = 800.0;
pub const PLAYER_ACCELERATION: f32 = 400.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerCollisionConvexShapes>()
            .add_event::<PlayerThrusterChanged>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    (player_input, player_movement, wrap_player_movement).chain(),
                    // player_hit_asteroid,
                    player_hit_star,
                    forward_thruster_visibility,
                )
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
