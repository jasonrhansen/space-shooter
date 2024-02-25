use bevy::{audio::Volume, prelude::*};

#[derive(Component)]
pub struct Music;

pub fn spawn_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/sci-fi-dramatic-theme.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.2)),
        },
        Music,
    ));
}
