use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct LaserAssets {
    #[asset(path = "images/sprites/laserRed01.png")]
    pub laser_texture: Handle<Image>,
    #[asset(path = "audio/sfx_laser1.ogg")]
    pub laser_sound: Handle<AudioSource>,
}
