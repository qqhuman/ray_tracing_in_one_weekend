use super::{scatter_record::ScatterRecord, Material};
use crate::rt::{color, random_f64, ray::Ray, shapes::hit_record::HitRecord, vec3::Vec3};

pub struct Dielectric {
    ior: f64,
}

impl Dielectric {
    pub fn new(ior: f64) -> Dielectric {
        Dielectric { ior }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = Vec3::unit_vector(r_in.direction);
        let cos_theata = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theata = (1.0 - cos_theata * cos_theata).sqrt();

        let cannot_refract = refraction_ratio * sin_theata > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theata, refraction_ratio) > random_f64() {
                Vec3::reflect(unit_direction, rec.normal)
            } else {
                Vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };

        Some(ScatterRecord::Specular {
            attenuation: color::WHITE,
            ray: Ray::new(rec.p, direction, r_in.time),
        })
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
