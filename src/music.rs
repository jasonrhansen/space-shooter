use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct Music;

pub fn spawn_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("audio/sci-fi-dramatic-theme.ogg"))
        .with_volume(0.2)
        .looped();
}
