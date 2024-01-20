use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct OsdPlugin;

impl Plugin for OsdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_score_text);
    }
}
