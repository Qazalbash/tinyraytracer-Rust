use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;

mod math;
mod primitive;

const IVORY: primitive::Material = primitive::Material::new(
    1.0,
    [0.9, 0.5, 0.1, 0.0],
    math::Vec3::new(0.4, 0.4, 0.3),
    50.0,
);

const GLASS: primitive::Material = primitive::Material::new(
    1.5,
    [0.0, 0.9, 0.1, 0.8],
    math::Vec3::new(0.6, 0.7, 0.8),
    125.0,
);

const RED_RUBBER: primitive::Material = primitive::Material::new(
    1.0,
    [1.4, 0.3, 0.0, 0.0],
    math::Vec3::new(0.3, 0.1, 0.1),
    10.0,
);

const MIRROR: primitive::Material = primitive::Material::new(
    1.0,
    [0.0, 16.0, 0.8, 0.0],
    math::Vec3::new(1.0, 1.0, 1.0),
    1425.0,
);

const SPHERES: [primitive::Sphere; 4] = [
    primitive::Sphere::new(math::Vec3::new(-3.0, 0.0, -16.0), 2.0, IVORY),
    primitive::Sphere::new(math::Vec3::new(-1.0, -1.5, -12.0), 2.0, GLASS),
    primitive::Sphere::new(math::Vec3::new(1.5, -0.5, -18.0), 3.0, RED_RUBBER),
    primitive::Sphere::new(math::Vec3::new(7.0, 5.0, -18.0), 4.0, MIRROR),
];

const LIGHTS: [math::Vec3; 3] = [
    math::Vec3::new(-20.0, 20.0, 20.0),
    math::Vec3::new(30.0, 50.0, -25.0),
    math::Vec3::new(30.0, 20.0, 30.0),
];

const BACKGROUND_COLOR: math::Vec3 = math::Vec3::new(0.1955, 0.9377, 0.6533);
const BOX_COLOR1: math::Vec3 = math::Vec3::new(0.9822, 0.6044, 0.1733);
const BOX_COLOR2: math::Vec3 = math::Vec3::new(0.9822, 0.2, 0.1733);

fn reflect(i: &math::Vec3, n: &math::Vec3) -> math::Vec3 {
    *i - *n * 2.0 * (*i * *n)
}

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
    let t1: f64 = tca + thc;

    if t0 > 0.001 {
        return (true, t0);
    }
    if t1 > 0.001 {
        return (true, t1);
    }
    return (false, 0.0);
}

fn scene_intersect(
    orig: &math::Vec3,
    dir: &math::Vec3,
) -> (bool, math::Vec3, math::Vec3, primitive::Material) {
    let mut pt: math::Vec3 = math::Vec3::void();
    let mut n: math::Vec3 = math::Vec3::void();
    let mut material: primitive::Material = primitive::Material::void();

    let mut nearest_dist: f64 = std::f64::MAX;

    if dir.y.abs() > 0.001 {
        let d = -(orig.y + 4.0) / dir.y;
        let p = *orig + *dir * d;

        if d > 0.001 && d < nearest_dist && p.x.abs() < 10.0 && p.z < -10.0 && p.z > -30.0 {
            nearest_dist = d;
            pt = p;
            n = math::Vec3::new(0.0, 1.0, 0.0);
            material.diffuse_color =
                if ((0.5 * pt.x + 1000.0) as i32 + (0.5 * pt.z) as i32) & 1 == 1 {
                    BOX_COLOR1
                    // math::Vec3::new(0.3, 0.3, 0.3)
                } else {
                    BOX_COLOR2
                    // math::Vec3::new(0.3, 0.2, 0.1)
                };
        }
    }

    for sphere in &SPHERES {
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

fn cast_ray(orig: &math::Vec3, dir: &math::Vec3, depth: u32) -> math::Vec3 {
    let (hit, point, n, material): (bool, math::Vec3, math::Vec3, primitive::Material) =
        scene_intersect(orig, dir);
    if depth > 4 || !hit {
        return BACKGROUND_COLOR;
    }

    let reflect_dir: math::Vec3 = reflect(dir, &n).normalized();
    let refract_dir: math::Vec3 = refract(dir, &n, &material.refractive_index, &1.0).normalized();
    let reflect_color: math::Vec3 = cast_ray(&point, &reflect_dir, depth + 1);
    let refract_color: math::Vec3 = cast_ray(&point, &refract_dir, depth + 1);

    let mut diffuse_light_intensity: f64 = 0.0;
    let mut specular_light_intensity: f64 = 0.0;

    for light in &LIGHTS {
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

fn main() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    const FOV: f64 = 1.05;

    let mut framebuffer: Vec<math::Vec3> = vec![math::Vec3::void(); WIDTH * HEIGHT];

    let dir_z: f64 = -(HEIGHT as f64) / (2.0 * (FOV / 2.0).tan());

    framebuffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(pix, pixel)| {
            let dir_x: f64 = ((pix % WIDTH) as f64 + 0.5) - (WIDTH as f64 / 2.0);
            let dir_y: f64 = -((pix / WIDTH) as f64 + 0.5) + (HEIGHT as f64 / 2.0);
            let dir: math::Vec3 = math::Vec3::new(dir_x, dir_y, dir_z).normalized();

            *pixel = cast_ray(&math::Vec3::void(), &dir, 0);
        });

    let mut file = File::create("out.ppm").unwrap();
    file.write_all(format!("P6\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())
        .unwrap();
    for color in &framebuffer {
        let scale = 255.0 / 1.0_f64.max(color.x.max(color.y.max(color.z)));
        for channel in &[color.x, color.y, color.z] {
            file.write_all(&[(channel * scale) as u8]).unwrap();
        }
    }

    println!("Done!");
}
