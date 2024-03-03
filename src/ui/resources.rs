use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "audio/confirmation_001.ogg")]
    pub pause_sound: Handle<AudioSource>,
    #[asset(path = "audio/confirmation_002.ogg")]
    pub resume_game_sound: Handle<AudioSource>,
}
