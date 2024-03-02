use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
    },
    sprite::Anchor,
};
use bevy_asset_loader::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Resource, Default, Clone)]
pub struct BackgroundAssets {
    pub dark_purple_background_image: Handle<Image>,
}

impl AssetCollection for BackgroundAssets {
    fn create(world: &mut World) -> Self {
        world.resource::<BackgroundAssets>().clone()
    }

    fn load(world: &mut World) -> Vec<UntypedHandle> {
        let sampler_desc = ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..Default::default()
        };

        let settings = move |s: &mut ImageLoaderSettings| {
            s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
        };

        let image_handle: Handle<Image> = {
            let asset_server = world.resource::<AssetServer>();
            asset_server.load_with_settings("images/backgrounds/darkPurple.png", settings)
        };

        let assets = BackgroundAssets {
            dark_purple_background_image: image_handle.clone(),
        };
        world.insert_resource(assets.clone());

        vec![image_handle.into()]
    }
}

pub fn spawn_background(mut commands: Commands, background_assets: Res<BackgroundAssets>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -1000.0),
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                rect: Some(Rect::new(0.0, 0.0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT)),
                ..default()
            },
            texture: background_assets.dark_purple_background_image.clone(),
            ..default()
        },
        Background {},
    ));
}
