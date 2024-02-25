use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UiAssets {
    pub pause_sound: Handle<AudioSource>,
    pub resume_game_sound: Handle<AudioSource>,
}
