use crate::math;
use num_traits::Float;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Material {
    pub(crate) refractive_index: f32,
    pub(crate) albedo: [f32; 4],
    pub(crate) diffuse_color: math::Vec3<f32>,
    pub(crate) specular_exponent: f32,
}

impl Material {
    pub(crate) const fn new(
        refractive_index: f32,
        albedo: [f32; 4],
        diffuse_color: math::Vec3<f32>,
        specular_exponent: f32,
    ) -> Self {
        Self {
            refractive_index,
            albedo,
            diffuse_color,
            specular_exponent,
        }
    }

    pub(crate) fn void() -> Self {
        Self {
            refractive_index: 1.0,
            albedo: [2.0, 0.0, 0.0, 0.0],
            diffuse_color: math::Vec3::default(),
            specular_exponent: 0.0,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Sphere<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    pub(crate) center: math::Vec3<T>,
    pub(crate) radius: T,
    pub(crate) material: Material,
}

impl<T> Sphere<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    pub(crate) const fn new(center: math::Vec3<T>, radius: T, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
