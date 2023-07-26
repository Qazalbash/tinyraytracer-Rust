use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write}; // For parallelization

mod constants;
mod math;
mod primitive;

/// Reflect an incident vector i around a normal n
#[inline]
fn reflect(i: &math::Vec3, n: &math::Vec3) -> math::Vec3 {
    *i - *n * 2.0 * (*i * *n)
}

/// Refract an incident vector i around a normal n
fn refract(i: &math::Vec3, n: &math::Vec3, eta_t: &f64, eta_i: &f64) -> math::Vec3 {
    let cosi: f64 = -((*i * *n).clamp(-1.0, 1.0));

    if cosi < 0.0 {
        return refract(i, &-*n, eta_i, eta_t);
    }

    let eta: f64 = *eta_i / *eta_t;
    let k: f64 = 1.0 + eta * eta * (cosi * cosi - 1.0);
    return if k < 0.0 {
        math::Vec3::new(1.0, 0.0, 0.0)
    } else {
        *i * eta + *n * (eta * cosi - k.sqrt())
    };
}

/// Intersection of a ray with a sphere
fn ray_sphere_intersect(
    orig: &math::Vec3,
    dir: &math::Vec3,
    sphere: &primitive::Sphere,
) -> (bool, f64) {
    let l: math::Vec3 = sphere.center - *orig;
    let tca: f64 = l * *dir;
    let d2: f64 = l * l - tca * tca;
    let radius_sq: f64 = sphere.radius * sphere.radius;

    if d2 > radius_sq {
        return (false, 0.0);
    }

    let thc: f64 = (radius_sq - d2).sqrt();

    let t0: f64 = tca - thc;
    if t0 > 0.001 {
        return (true, t0);
    }

    let t1: f64 = tca + thc;
    if t1 > 0.001 {
        return (true, t1);
    }
    return (false, 0.0);
}

/// A pattern function that returns a color based on the position of a point in space
#[inline]
fn pattern(a: f64, b: f64) -> math::Vec3 {
    match ((a + 1000.0) as i32 + b as i32) & 1 == 1 {
        true => constants::DARK_SQUARE,
        false => constants::LIGHT_SQUARE,
    }
}

/// Intersection of a ray with the scene
fn scene_intersect(
    orig: &math::Vec3,
    dir: &math::Vec3,
) -> (bool, math::Vec3, math::Vec3, primitive::Material) {
    let mut pt: math::Vec3 = math::Vec3::void();
    let mut n: math::Vec3 = math::Vec3::void();
    let mut material: primitive::Material = primitive::Material::void();
    let mut nearest_dist: f64 = std::f64::MAX;

    if dir.x.abs() > 0.001 {
        let mut d: f64 = -(orig.x + 12.0) / dir.x;
        let mut p: math::Vec3 = *orig + *dir * d;

        if d > 0.001 && d < nearest_dist && (p.z + 12.0).abs() < 10.0 && p.y.abs() < 4.0 {
            nearest_dist = d;
            pt = p;
            n = math::Vec3::new(1.0, 0.0, 0.0);
            material.diffuse_color = pattern(pt.y, pt.z);
            material.diffuse_color = material.diffuse_color * 0.3;
        }

        d = -(orig.x - 12.0) / dir.x;
        p = *orig + *dir * d;

        if d > 0.001 && d < nearest_dist && (p.z + 12.0).abs() < 10.0 && p.y.abs() < 4.0 {
            nearest_dist = d;
            pt = p;
            n = math::Vec3::new(-1.0, 0.0, 0.0);
            material.diffuse_color = pattern(pt.y, pt.z);
            material.diffuse_color = material.diffuse_color * 0.3;
        }
    }

    if dir.y.abs() > 0.001 {
        let d: f64 = -(orig.y + 4.0) / dir.y;
        let p: math::Vec3 = *orig + *dir * d;

        if d > 0.001 && d < nearest_dist && p.x.abs() < 12.0 && (p.z + 12.0).abs() < 10.0 {
            nearest_dist = d;
            pt = p;
            n = math::Vec3::new(0.0, 1.0, 0.0);
            material.diffuse_color = pattern(pt.x, pt.z);
            material.diffuse_color = material.diffuse_color * 0.3;
        }
    }

    if dir.z.abs() > 0.001 {
        let d: f64 = -(orig.z + 22.0) / dir.z;
        let p: math::Vec3 = *orig + *dir * d;

        if d > 0.001 && d < nearest_dist && p.x.abs() < 12.0 && p.y.abs() < 4.0 {
            nearest_dist = d;
            pt = p;
            n = math::Vec3::new(0.0, 0.0, 1.0);
            material.diffuse_color = pattern(pt.y, pt.x);
            material.diffuse_color = material.diffuse_color * 0.3;
        }
    }

    for sphere in &constants::SPHERES {
        let (intersection, d): (bool, f64) = ray_sphere_intersect(orig, dir, &sphere);
        if !intersection || d > nearest_dist {
            continue;
        }
        nearest_dist = d;
        pt = *orig + *dir * nearest_dist;
        n = (pt - sphere.center).normalized();
        material = sphere.material;
    }

    return (nearest_dist < 1000.0, pt, n, material);
}

/// Cast a ray into the scene
fn cast_ray(orig: &math::Vec3, dir: &math::Vec3, depth: u32) -> math::Vec3 {
    let (hit, point, n, material): (bool, math::Vec3, math::Vec3, primitive::Material) =
        scene_intersect(orig, dir);
    if depth > constants::DEPTH || !hit {
        return constants::BACKGROUND_COLOR;
    }

    let reflect_dir: math::Vec3 = reflect(dir, &n).normalized();
    let refract_dir: math::Vec3 = refract(dir, &n, &material.refractive_index, &1.0).normalized();
    let reflect_color: math::Vec3 = cast_ray(&point, &reflect_dir, depth + 1);
    let refract_color: math::Vec3 = cast_ray(&point, &refract_dir, depth + 1);

    let mut diffuse_light_intensity: f64 = 0.0;
    let mut specular_light_intensity: f64 = 0.0;

    for light in &constants::LIGHTS {
        let light_dir: math::Vec3 = (*light - point).normalized();

        let (hit, shadow_pt, _, _): (bool, math::Vec3, math::Vec3, primitive::Material) =
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
        + (math::Vec3::new(1.0, 1.0, 1.0) * specular_light_intensity * material.albedo[1])
        + (reflect_color * material.albedo[2])
        + (refract_color * material.albedo[3])
}

/// Main function
fn main() {
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;
    const FOV: f64 = 1.05;

    let mut framebuffer: Vec<math::Vec3> = vec![math::Vec3::void(); WIDTH * HEIGHT];

    let dir_z: f64 = -(HEIGHT as f64) / (2.0 * (FOV / 2.0).tan());

    println!("Rendering...");

    framebuffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(pix, pixel)| {
            let dir_x: f64 = ((pix % WIDTH) as f64 + 0.5) - (WIDTH as f64 / 2.0);
            let dir_y: f64 = -((pix / WIDTH) as f64 + 0.5) + (HEIGHT as f64 / 2.0);
            let dir: math::Vec3 = math::Vec3::new(dir_x, dir_y, dir_z).normalized();

            *pixel = cast_ray(&constants::CAMERA_POSITION, &dir, 0);
        });

    println!("Writing to file...");

    let mut file = BufWriter::new(File::create("out.ppm").unwrap());
    file.write_all(format!("P6\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())
        .unwrap();
    for color in &framebuffer {
        let scale = 255.0 / 1.0_f64.max(color.x.max(color.y.max(color.z)));
        for channel in &[color.x, color.y, color.z] {
            file.write_all(&[(channel * scale) as u8]).unwrap();
        }
    }

    file.flush().unwrap();

    println!("Done!");
}
