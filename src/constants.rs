use crate::math;
use crate::primitive;

// Materials
pub(crate) const IVORY: primitive::Material = primitive::Material::new(
    1.0,
    [0.9, 0.5, 0.1, 0.0],
    math::Vec3::new(0.4, 0.4, 0.3),
    50.0,
);

pub(crate) const GLASS: primitive::Material = primitive::Material::new(
    1.5,
    [0.0, 0.9, 0.1, 0.8],
    math::Vec3::new(0.6, 0.7, 0.8),
    125.0,
);

pub(crate) const RED_RUBBER: primitive::Material = primitive::Material::new(
    1.0,
    [1.4, 0.3, 0.0, 0.0],
    math::Vec3::new(0.3, 0.1, 0.1),
    10.0,
);

pub(crate) const MIRROR: primitive::Material = primitive::Material::new(
    1.0,
    [0.0, 16.0, 0.8, 0.0],
    math::Vec3::new(1.0, 1.0, 1.0),
    1425.0,
);

// Spheres in the scene
pub(crate) const SPHERES: [primitive::Sphere; 4] = [
    primitive::Sphere::new(math::Vec3::new(-3.0, 0.0, -16.0), 2.0, IVORY),
    primitive::Sphere::new(math::Vec3::new(-1.0, -1.5, -12.0), 2.0, GLASS),
    primitive::Sphere::new(math::Vec3::new(1.5, -0.5, -18.0), 3.0, RED_RUBBER),
    primitive::Sphere::new(math::Vec3::new(7.0, 5.0, -18.0), 4.0, MIRROR),
];

// Lights in the scene
pub(crate) const LIGHTS: [math::Vec3; 3] = [
    math::Vec3::new(-20.0, 20.0, 20.0),
    math::Vec3::new(30.0, 50.0, -25.0),
    math::Vec3::new(30.0, 20.0, 30.0),
];

pub(crate) const BACKGROUND_COLOR: math::Vec3 = math::Vec3::new(0.1955, 0.9377, 0.6533);
pub(crate) const BOX_COLOR1: math::Vec3 = math::Vec3::new(0.9822, 0.6044, 0.1733);
pub(crate) const BOX_COLOR2: math::Vec3 = math::Vec3::new(0.9822, 0.2, 0.1733);