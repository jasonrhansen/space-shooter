use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;

use systems::*;

use self::events::SpawnLaser;

pub const LASER_SPEED: f32 = 800.0;
pub const LASER_SIZE: f32 = 64.0;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLaser>().add_systems(
            Update,
            (
                (laser_movement, despawn_offscreen_lasers).chain(),
                spawn_lasers,
            ),
        );
    }
}
