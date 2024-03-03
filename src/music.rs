use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct Music;

#[derive(AssetCollection, Resource)]
pub struct MusicAssets {
    #[asset(path = "audio/sci-fi-dramatic-theme.ogg")]
    pub sci_fi_dramatic_theme_sound: Handle<AudioSource>,
}

pub fn spawn_music(music_assets: Res<MusicAssets>, audio: Res<Audio>) {
    audio.stop();
    audio
        .play(music_assets.sci_fi_dramatic_theme_sound.clone())
        .with_volume(0.2)
        .looped();
}
