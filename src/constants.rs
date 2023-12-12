#![allow(dead_code)]

use crate::math;
use crate::primitive;

const BLACK: math::Vec3<f32> = math::Vec3::new(0.0, 0.0, 0.0);
const WHITE: math::Vec3<f32> = math::Vec3::new(1.0, 1.0, 1.0);
const RED: math::Vec3<f32> = math::Vec3::new(1.0, 0.0, 0.0);
const GREEN: math::Vec3<f32> = math::Vec3::new(0.0, 1.0, 0.0);
const BLUE: math::Vec3<f32> = math::Vec3::new(0.0, 0.0, 1.0);
const PURE_YELLOW: math::Vec3<f32> = math::Vec3::new(1.0, 1.0, 0.0);
const BROWN: math::Vec3<f32> = math::Vec3::new(0.71, 0.40, 0.16);
const DARK_GREEN: math::Vec3<f32> = math::Vec3::new(0.0, 0.41, 0.41);
const ORANGE: math::Vec3<f32> = math::Vec3::new(1.0, 0.75, 0.0);
const LIGHT_GREEN: math::Vec3<f32> = math::Vec3::new(0.65, 1.0, 0.30);
const DARK_YELLOW: math::Vec3<f32> = math::Vec3::new(0.61, 0.61, 0.0);
const LIGHT_PURPLE: math::Vec3<f32> = math::Vec3::new(0.65, 0.3, 1.0);
const DARK_PURPLE: math::Vec3<f32> = math::Vec3::new(0.5, 0.0, 1.0);
const GREY: math::Vec3<f32> = math::Vec3::new(0.25, 0.25, 0.25);
const PALE_BLUE: math::Vec3<f32> = math::Vec3::new(0.68, 0.85, 0.90);
const PALE_GREEN: math::Vec3<f32> = math::Vec3::new(0.63, 0.80, 0.63);
const PURPLE_BLUE: math::Vec3<f32> = math::Vec3::new(0.5, 0.3, 1.0);
const PINK: math::Vec3<f32> = math::Vec3::new(1.0, 0.0, 0.5);

// Materials
const IVORY: primitive::Material =
    primitive::Material::new(1.0, [0.9, 0.5, 0.1, 0.0], LIGHT_PURPLE, 50.0);

const GLASS: primitive::Material =
    primitive::Material::new(1.5, [0.0, 0.9, 0.1, 0.8], WHITE, 125.0);

const RED_RUBBER: primitive::Material =
    primitive::Material::new(1.0, [1.4, 0.3, 0.0, 0.0], ORANGE, 10.0);

const MIRROR: primitive::Material =
    primitive::Material::new(1.0, [0.0, 16.0, 0.8, 0.0], WHITE, 1425.0);

const STEEL: primitive::Material =
    primitive::Material::new(1.0, [1.4, 0.4, 0.25, 0.0], DARK_GREEN, 10.0);

// Spheres in the scene
pub const SPHERES: [primitive::Sphere<f32>; 5] = [
    primitive::Sphere::new(math::Vec3::new(1.0, 0.0, -6.0), 2.0, GLASS),
    primitive::Sphere::new(math::Vec3::new(2.0, 1.5, -12.0), 3.0, STEEL),
    primitive::Sphere::new(math::Vec3::new(-2.0, 3.5, -8.0), 2.0, IVORY),
    primitive::Sphere::new(math::Vec3::new(-2.0, 0.5, -6.0), 1.0, RED_RUBBER),
    primitive::Sphere::new(math::Vec3::new(5.0, 1.5, -7.0), 2.0, MIRROR),
];

// Lights in the scene
pub const LIGHTS: [math::Vec3<f32>; 2] = [
    math::Vec3::new(-20.0, 20.0, 20.0),
    math::Vec3::new(30.0, 20.0, 30.0),
];

pub const BACKGROUND_COLOR: math::Vec3<f32> = PINK;
pub const LIGHT_SQUARE: math::Vec3<f32> = BLACK;
pub const DARK_SQUARE: math::Vec3<f32> = WHITE;

pub const CAMERA_POSITION: math::Vec3<f32> = math::Vec3::new(10.0, 12.0, 2.0); // right side

pub const DEPTH: u32 = 5;
