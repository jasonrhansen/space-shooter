use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource, Default)]
pub struct UiAssets {
    pub pause_sound: Handle<AudioSource>,
    pub resume_game_sound: Handle<AudioSource>,
}
