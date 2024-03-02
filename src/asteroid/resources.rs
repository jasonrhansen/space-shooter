use crate::ConvexShape;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct AsteroidAssets {
    #[asset(path = "images/sprites/meteorGrey_big1.png")]
    pub grey_1_texture: Handle<Image>,
    #[asset(path = "images/sprites/meteorGrey_big2.png")]
    pub grey_2_texture: Handle<Image>,
    #[asset(path = "images/sprites/meteorGrey_big3.png")]
    pub grey_3_texture: Handle<Image>,
    #[asset(path = "images/sprites/meteorGrey_big4.png")]
    pub grey_4_texture: Handle<Image>,
    #[asset(path = "images/sprites/meteorBrown_big1.png")]
    pub brown_1_texture: Handle<Image>,
    #[asset(path = "images/sprites/meteorBrown_big2.png")]
    pub brown_2_texture: Handle<Image>,
    #[asset(path = "images/sprites/meteorBrown_big3.png")]
    pub brown_3_texture: Handle<Image>,
    #[asset(path = "images/sprites/meteorBrown_big4.png")]
    pub brown_4_texture: Handle<Image>,
}

#[derive(Resource)]
pub struct AsteroidCollisionConvexShapes {
    pub asteroid_shapes: Box<[Box<[ConvexShape]>]>,
}

impl Default for AsteroidCollisionConvexShapes {
    fn default() -> Self {
        AsteroidCollisionConvexShapes {
            asteroid_shapes: Box::new([
                // For meteor*_big1.png
                Box::new([
                    Box::new([
                        Vec2::new(-49.89, -9.79),
                        Vec2::new(-21.92, -40.78),
                        Vec2::new(-33.16, 40.63),
                        Vec2::new(11.64, -28.27),
                        Vec2::new(49.72, 1.26),
                        Vec2::new(23.31, 40.81),
                    ]),
                    Box::new([
                        Vec2::new(33.75, -32.49),
                        Vec2::new(49.72, 1.26),
                        Vec2::new(11.64, -28.27),
                    ]),
                ]),
                // For meteor*_big2.png
                Box::new([
                    Box::new([
                        Vec2::new(-55.166, -26.606),
                        Vec2::new(-60.071, 5.822),
                        Vec2::new(-40.172, 41.042),
                        Vec2::new(-8.883, -33.307),
                        Vec2::new(45.602, -18.405),
                        Vec2::new(59.928, 29.770),
                        Vec2::new(6.505, 48.664),
                    ]),
                    Box::new([
                        Vec2::new(-8.883, -33.307),
                        Vec2::new(-55.166, -26.606),
                        Vec2::new(-26.479, -49.335),
                    ]),
                ]),
                // For meteor*_big3.png
                Box::new([Box::new([
                    Vec2::new(43.912, -0.812),
                    Vec2::new(28.949, 28.017),
                    Vec2::new(-43.451, 19.158),
                    Vec2::new(-10.015, 40.126),
                    Vec2::new(-42.228, -14.227),
                    Vec2::new(-27.900, -33.810),
                    Vec2::new(21.198, -40.288),
                ])]),
                // For meteor*_big4.png
                Box::new([Box::new([
                    Vec2::new(-49.000, -11.161),
                    Vec2::new(48.563, 11.990),
                    Vec2::new(-35.021, 34.021),
                    Vec2::new(16.966, 47.271),
                    Vec2::new(-19.294, -48.048),
                    Vec2::new(30.505, -42.564),
                ])]),
            ]),
        }
    }
}
