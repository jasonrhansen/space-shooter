pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use self::{
    events::{PlayerDeath, PlayerThrusterChanged},
    resources::PlayerCollisionConvexShapes,
};
use crate::{AppState, UpdateSet};
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
            .add_event::<PlayerDeath>()
            .add_systems(
                Update,
                (
                    new_game_spawn_player.in_set(UpdateSet::Init),
                    player_input.in_set(UpdateSet::Input).after(UpdateSet::Init),
                    (player_movement, wrap_player_movement)
                        .chain()
                        .in_set(UpdateSet::Movement)
                        .after(UpdateSet::Input),
                    (player_hit_asteroid, player_hit_star)
                        .in_set(UpdateSet::Collision)
                        .after(UpdateSet::Movement),
                    forward_thruster_visibility,
                    player_damage_timer,
                    player_death.after(UpdateSet::Ui),
                )
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
