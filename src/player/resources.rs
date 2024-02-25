use crate::ConvexShape;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerAssets {
    pub ship_texture: Handle<Image>,
    pub fire_texture: Handle<Image>,
    pub explosion_texture: Handle<Image>,
    pub explosion_sound: Handle<AudioSource>,
    pub star_sound: Handle<AudioSource>,
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
                Box::new([
                    Vec2::new(-8.0, 37.0),
                    Vec2::new(8.0, 37.0),
                    Vec2::new(15.0, -27.0),
                    Vec2::new(9.0, -37.0),
                    Vec2::new(-7.0, -37.0),
                    Vec2::new(-13.0, -28.0),
                ]),
                Box::new([
                    Vec2::new(13.0, 11.0),
                    Vec2::new(36.0, -2.0),
                    Vec2::new(36.0, -22.0),
                    Vec2::new(15.0, -27.0),
                ]),
                Box::new([
                    Vec2::new(-13.0, 11.0),
                    Vec2::new(-36.0, -2.0),
                    Vec2::new(-36.0, -22.0),
                    Vec2::new(-15.0, -27.0),
                ]),
                Box::new([
                    Vec2::new(36.0, -2.0),
                    Vec2::new(49.0, 6.0),
                    Vec2::new(36.0, -22.0),
                    Vec2::new(46.0, -29.0),
                ]),
                Box::new([
                    Vec2::new(-36.0, -2.0),
                    Vec2::new(-49.0, 6.0),
                    Vec2::new(-36.0, -22.0),
                    Vec2::new(-46.0, -29.0),
                ]),
            ]),
        }
    }
}
