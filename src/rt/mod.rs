use self::{color::Color, ray::Ray, shapes::Hittable, vec3::Vec3};

pub mod camera;
pub mod color;
pub mod materials;
pub mod noise;
pub mod ray;
pub mod shapes;
pub mod textures;
pub mod vec3;

pub type Point3 = Vec3;

pub fn ray_color(r: &Ray, background: Color, world: &dyn Hittable, depth: usize) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return color::BLACK;
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        None => background, // If the ray hits nothing, return the background color.
        Some(rec) => {
            let emitted = rec.material.emitted(rec.u, rec.v, rec.p);
            match rec.material.scatter(r, &rec) {
                None => emitted,
                Some((attenuation, scattered)) => {
                    emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
                }
            }
        }
    }
}

pub const PI: f64 = 3.1415926535897932385;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_i32_between(min: i32, max: i32) -> i32 {
    fastrand::i32(min..=max)
}

pub fn random_f64() -> f64 {
    fastrand::f64()
}
pub fn random_f64_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random_f64(), random_f64(), random_f64())
}

pub fn random_vec3_between(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_f64_between(min, max),
        random_f64_between(min, max),
        random_f64_between(min, max),
    )
}

pub fn random_vec2_between(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_f64_between(min, max),
        random_f64_between(min, max),
        0.0,
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_between(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = random_vec2_between(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    Vec3::unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if Vec3::dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
