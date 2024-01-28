use bevy::prelude::*;
use bevy_rapier2d::prelude::Vect;
use once_cell::sync::Lazy;

pub static COLLISION_VERTICES: Lazy<Box<[Box<[Box<[Vect]>]>]>> = Lazy::new(|| {
    Box::new([
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
        Box::new([Box::new([
            Vec2::new(43.912, -0.812),
            Vec2::new(28.949, 28.017),
            Vec2::new(-43.451, 19.158),
            Vec2::new(-10.015, 40.126),
            Vec2::new(-42.228, -14.227),
            Vec2::new(-27.900, -33.810),
            Vec2::new(21.198, -40.288),
        ])]),
        Box::new([Box::new([
            Vec2::new(-49.000, -11.161),
            Vec2::new(48.563, 11.990),
            Vec2::new(-35.021, 34.021),
            Vec2::new(16.966, 47.271),
            Vec2::new(-19.294, -48.048),
            Vec2::new(30.505, -42.564),
        ])]),
    ])
});
