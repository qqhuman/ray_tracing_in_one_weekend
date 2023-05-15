use self::{
    camera::Camera,
    color::Color,
    materials::scatter_record::ScatterRecord,
    pdfs::{hittable_pdf::HittablePdf, mixture_pdf::MixturePdf, Pdf},
    ray::Ray,
    shapes::Hittable,
    vec3::Vec3,
};

pub mod camera;
pub mod color;
pub mod materials;
pub mod noise;
mod onb;
mod pdfs;
mod ray;
pub mod shapes;
pub mod textures;
pub mod vec3;

pub type Point3 = Vec3;

pub fn ray_color(
    r: &Ray,
    background: Color,
    world: &dyn Hittable,
    lights: &dyn Hittable,
    depth: usize,
) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return color::BLACK;
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        None => background, // If the ray hits nothing, return the background color.
        Some(rec) => {
            let emitted = rec.material.emitted(r, &rec, rec.u, rec.v, rec.p);
            match rec.material.scatter(r, &rec) {
                None => emitted,
                Some(srec) => match srec {
                    ScatterRecord::Specular { attenuation, ray } => {
                        attenuation * ray_color(&ray, background, world, lights, depth - 1)
                    }
                    ScatterRecord::Diffuse { attenuation, pdf } => {
                        let lights_pdf = HittablePdf::new(lights, rec.p);
                        let p = MixturePdf::new(&lights_pdf, pdf.as_ref());
                        let scattered = Ray::new(rec.p, p.generate(), r.time);
                        let pdf_value = p.value(scattered.direction);
                        emitted
                            + attenuation
                                * rec.material.scattering_pdf(r, &rec, &scattered)
                                * ray_color(&scattered, background, world, lights, depth - 1)
                                / pdf_value
                    }
                },
            }
        }
    }
}

pub fn render_pixel(
    x: u32,
    y: u32,
    camera: &Camera,
    background: Color,
    world: &dyn Hittable,
    lights: &dyn Hittable,
) -> Color {
    let mut color = color::BLACK;
    for _ in 0..camera.samples_per_pixel {
        let u = (x as f64 + random_f64()) / (camera.width - 1) as f64;
        let v = (y as f64 + random_f64()) / (camera.height - 1) as f64;
        let r = camera.get_ray(u, v);
        color = color + ray_color(&r, background, world, lights, camera.max_depth);
    }
    color
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

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_f64();
    let r2 = random_f64();
    let z = f64::sqrt(1.0 - r2);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_f64();
    let r2 = random_f64();
    let z = 1.0 + r2 * (f64::sqrt(1.0 - radius * radius / distance_squared) - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * f64::sqrt(1.0 - z * z);
    let y = phi.sin() * f64::sqrt(1.0 - z * z);

    Vec3::new(x, y, z)
}
