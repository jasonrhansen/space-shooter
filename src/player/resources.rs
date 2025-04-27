use crate::ConvexShape;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "images/sprites/playerShip1_red.png")]
    pub ship_texture: Handle<Image>,
    #[asset(path = "images/sprites/fire13.png")]
    pub fire_texture: Handle<Image>,
    #[asset(path = "images/sprites/explosion.png")]
    pub explosion_texture: Handle<Image>,
    #[asset(path = "audio/explosionCrunch_000.ogg")]
    pub explosion_sound: Handle<AudioSource>,
    #[asset(path = "audio/laserLarge_000.ogg")]
    pub star_sound: Handle<AudioSource>,
    #[asset(path = "audio/thrusterFire_000.ogg")]
    pub thruster_sound: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct PlayerCollisionConvexShapes {
    pub player_shapes: Box<[ConvexShape]>,
}

impl Default for PlayerCollisionConvexShapes {
    fn default() -> Self {
        PlayerCollisionConvexShapes {
            // For playerShip1_red.png
            player_shapes: Box::new([
                vec![
                    Vec2::new(-8.0, 37.0),
                    Vec2::new(8.0, 37.0),
                    Vec2::new(15.0, -27.0),
                    Vec2::new(9.0, -37.0),
                    Vec2::new(-7.0, -37.0),
                    Vec2::new(-13.0, -28.0),
                ],
                vec![
                    Vec2::new(13.0, 11.0),
                    Vec2::new(36.0, -2.0),
                    Vec2::new(36.0, -22.0),
                    Vec2::new(15.0, -27.0),
                ],
                vec![
                    Vec2::new(-13.0, 11.0),
                    Vec2::new(-36.0, -2.0),
                    Vec2::new(-36.0, -22.0),
                    Vec2::new(-15.0, -27.0),
                ],
                vec![
                    Vec2::new(36.0, -2.0),
                    Vec2::new(49.0, 6.0),
                    Vec2::new(36.0, -22.0),
                    Vec2::new(46.0, -29.0),
                ],
                vec![
                    Vec2::new(-36.0, -2.0),
                    Vec2::new(-49.0, 6.0),
                    Vec2::new(-36.0, -22.0),
                    Vec2::new(-46.0, -29.0),
                ],
            ]),
        }
    }
}
