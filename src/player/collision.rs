use bevy::prelude::*;
use bevy_rapier2d::prelude::Vect;
use once_cell::sync::Lazy;

pub static COLLISION_VERTICES: Lazy<[Box<[Vect]>; 5]> = Lazy::new(|| {
    [
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
    ]
});
