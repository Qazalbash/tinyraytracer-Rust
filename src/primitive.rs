use crate::math;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Material {
    pub(crate) refractive_index: f64,
    pub(crate) albedo: [f64; 4],
    pub(crate) diffuse_color: math::Vec3,
    pub(crate) specular_exponent: f64,
}

impl Material {
    pub(crate) const fn new(
        refractive_index: f64,
        albedo: [f64; 4],
        diffuse_color: math::Vec3,
        specular_exponent: f64,
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
            diffuse_color: math::Vec3::void(),
            specular_exponent: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Sphere {
    pub(crate) center: math::Vec3,
    pub(crate) radius: f64,
    pub(crate) material: Material,
}

impl Sphere {
    pub(crate) const fn new(center: math::Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
