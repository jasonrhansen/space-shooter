use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource, Default)]
pub struct LaserAssets {
    pub laser_texture: Handle<Image>,
    pub laser_sound: Handle<AudioSource>,
}
