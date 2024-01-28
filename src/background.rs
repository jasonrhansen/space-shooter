use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
    sprite::Anchor,
};

#[derive(Component)]
pub struct Background;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..Default::default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -1000.0),
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                rect: Some(Rect::new(0.0, 0.0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT)),
                ..default()
            },
            texture: asset_server.load_with_settings("images/backgrounds/darkPurple.png", settings),
            ..default()
        },
        Background {},
    ));
}
