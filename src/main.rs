#![allow(dead_code)]
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

mod constants;
mod math;
mod primitive;

/// Reflect an incident vector i around a normal n
#[inline]
fn reflect(i: &math::Vec3<f32>, n: &math::Vec3<f32>) -> math::Vec3<f32> {
    *i - *n * 2.0 * (*i * *n)
}

#[inline]
fn rotate_at_axis_x(v: &math::Vec3<f32>, angle: f32) -> math::Vec3<f32> {
    let angle_in_radians: f32 = angle * std::f32::consts::PI / 180.0;
    let sine: f32 = f32::sin(angle_in_radians);
    let cosine: f32 = f32::cos(angle_in_radians);

    math::Vec3::new(v.x, v.y * cosine - v.z * sine, v.y * sine + v.z * cosine)
}

#[inline]
fn rotate_at_axis_y(v: &math::Vec3<f32>, angle: f32) -> math::Vec3<f32> {
    let angle_in_radians: f32 = angle * std::f32::consts::PI / 180.0;
    let sine: f32 = f32::sin(angle_in_radians);
    let cosine: f32 = f32::cos(angle_in_radians);

    math::Vec3::new(v.x * cosine - v.z * sine, v.y, v.x * sine + v.z * cosine)
}

#[inline]
fn rotate_at_axis_z(v: &math::Vec3<f32>, angle: f32) -> math::Vec3<f32> {
    let angle_in_radians: f32 = angle * std::f32::consts::PI / 180.0;
    let sine: f32 = f32::sin(angle_in_radians);
    let cosine: f32 = f32::cos(angle_in_radians);

    math::Vec3::new(v.x * cosine - v.y * sine, v.x * sine + v.y * cosine, v.z)
}

/// Refract an incident vector i around a normal n
fn refract(i: &math::Vec3<f32>, n: &math::Vec3<f32>, eta_t: &f32, eta_i: &f32) -> math::Vec3<f32> {
    let cosi: f32 = -((*i * *n).clamp(-1.0, 1.0));

    if cosi < 0.0 {
        return refract(i, &-*n, eta_i, eta_t);
    }

    let eta: f32 = *eta_i / *eta_t;
    let k: f32 = 1.0 + eta * eta * (cosi * cosi - 1.0);
    if k < 0.0 {
        math::Vec3::new(1.0, 0.0, 0.0)
    } else {
        *i * eta + *n * (eta * cosi - k.sqrt())
    }
}

/// Intersection of a ray with a sphere
fn ray_sphere_intersect(
    orig: &math::Vec3<f32>,
    dir: &math::Vec3<f32>,
    sphere: &primitive::Sphere<f32>,
) -> (bool, f32) {
    let l: math::Vec3<f32> = sphere.center - *orig;
    let tca: f32 = l * *dir;
    let d2: f32 = l * l - tca * tca;
    let radius_sq: f32 = sphere.radius * sphere.radius;

    if d2 > radius_sq {
        return (false, 0.0);
    }

    let thc: f32 = (radius_sq - d2).sqrt();

    let t0: f32 = tca - thc;
    if t0 > 0.001 {
        return (true, t0);
    }

    let t1: f32 = tca + thc;
    if t1 > 0.001 {
        return (true, t1);
    }

    (false, 0.0)
}

/// A pattern function that returns a color based on the position of a point in space
#[inline]
fn pattern(a: f32, b: f32) -> math::Vec3<f32> {
    match ((a + 1000.0) as i32 + b as i32) & 1 == 1 {
        true => constants::DARK_SQUARE,
        false => constants::LIGHT_SQUARE,
    }
}

/// Intersection of a ray with the scene
fn scene_intersect(
    orig: &math::Vec3<f32>,
    dir: &math::Vec3<f32>,
) -> (bool, math::Vec3<f32>, math::Vec3<f32>, primitive::Material) {
    let mut pt: math::Vec3<f32> = math::Vec3::default();
    let mut n: math::Vec3<f32> = math::Vec3::default();
    let mut material: primitive::Material = primitive::Material::void();
    let mut nearest_dist: f32 = std::f32::MAX;

    if dir.y.abs() > 0.001 {
        let d: f32 = -(orig.y + 4.0) / dir.y;
        let p: math::Vec3<f32> = *orig + *dir * d;

        if d > 0.0001 && d < nearest_dist && p.x.abs() < 10.0 && (p.z + 10.0).abs() < 8.0 {
            nearest_dist = d;
            pt = p;
            n = math::Vec3::new(0.0, 1.0, 0.0).normalized();
            material.diffuse_color = pattern(pt.x, pt.z) * 0.3;
        }
    }

    for sphere in &constants::SPHERES {
        let (intersection, d): (bool, f32) = ray_sphere_intersect(orig, dir, sphere);
        if !intersection || d > nearest_dist {
            continue;
        }
        nearest_dist = d;
        pt = *orig + *dir * nearest_dist;
        n = (pt - sphere.center).normalized();
        material = sphere.material;
    }

    (nearest_dist < 1000.0, pt, n, material)
}

/// Cast a ray into the scene
fn cast_ray(orig: &math::Vec3<f32>, dir: &math::Vec3<f32>, depth: u32) -> math::Vec3<f32> {
    let (hit, point, n, material): (bool, math::Vec3<f32>, math::Vec3<f32>, primitive::Material) =
        scene_intersect(orig, dir);
    if depth > constants::DEPTH || !hit {
        return constants::BACKGROUND_COLOR;
    }

    let reflect_dir: math::Vec3<f32> = reflect(dir, &n).normalized();
    let refract_dir: math::Vec3<f32> =
        refract(dir, &n, &material.refractive_index, &1.0).normalized();
    let reflect_color: math::Vec3<f32> = cast_ray(&point, &reflect_dir, depth + 1);
    let refract_color: math::Vec3<f32> = cast_ray(&point, &refract_dir, depth + 1);

    let mut diffuse_light_intensity: f32 = 0.0;
    let mut specular_light_intensity: f32 = 0.0;

    for light in &constants::LIGHTS {
        let light_dir: math::Vec3<f32> = (*light - point).normalized();

        let (hit, shadow_pt, _, _): (bool, math::Vec3<f32>, math::Vec3<f32>, primitive::Material) =
            scene_intersect(&point, &light_dir);

        if hit && (shadow_pt - point).norm() < (*light - point).norm() {
            continue;
        }

        diffuse_light_intensity += (light_dir * n).max(0.0);
        specular_light_intensity += (-reflect(&-light_dir, &n) * *dir)
            .max(0.0)
            .powf(material.specular_exponent);
    }

    (material.diffuse_color * diffuse_light_intensity * material.albedo[0])
        + (math::Vec3::new(
            specular_light_intensity,
            specular_light_intensity,
            specular_light_intensity,
        ) * material.albedo[1])
        + (reflect_color * material.albedo[2])
        + (refract_color * material.albedo[3])
}

/// Main function
fn main() {
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;
    const FOV: f32 = 1.05;

    let mut framebuffer: Vec<math::Vec3<f32>> = vec![math::Vec3::default(); WIDTH * HEIGHT];

    let dir_z: f32 = -(HEIGHT as f32) / (2.0 * (FOV / 2.0).tan());

    println!("Rendering...");

    framebuffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(pix, pixel)| {
            let dir_x: f32 = ((pix % WIDTH) as f32 + 0.5) - (WIDTH as f32 / 2.0);
            let dir_y: f32 = -((pix / WIDTH) as f32 + 0.5) + (HEIGHT as f32 / 2.0);
            // let dir: math::Vec3<f32> = math::Vec3::new(dir_x, dir_y, dir_z).normalized();

            let mut dir: math::Vec3<f32> = math::Vec3::new(dir_x, dir_y, dir_z);

            dir = rotate_at_axis_x(&dir, -45.0);
            dir = rotate_at_axis_y(&dir, -45.0);
            // dir = rotate_at_axis_z(&dir, 15.0);
            dir = dir.normalized();

            *pixel = cast_ray(&constants::CAMERA_POSITION, &dir, 0);
        });

    println!("Writing to file...");

    let mut file: BufWriter<File> = BufWriter::new(File::create("out.ppm").unwrap());
    file.write_all(format!("P6\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())
        .unwrap();
    for color in &framebuffer {
        let scale = 255.0 / 1.0_f32.max(color.x.max(color.y.max(color.z)));
        for channel in &[color.x, color.y, color.z] {
            file.write_all(&[(channel * scale) as u8]).unwrap();
        }
    }

    file.flush().unwrap();

    println!("Done!");
}
