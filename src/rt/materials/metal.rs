use super::{scatter_record::ScatterRecord, Material};
use crate::rt::{
    color::Color, random_in_unit_sphere, ray::Ray, shapes::hit_record::HitRecord, vec3::Vec3,
};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction), rec.normal);
        Some(ScatterRecord::Specular {
            attenuation: self.albedo,
            ray: Ray::new(
                rec.p,
                reflected + self.fuzz * random_in_unit_sphere(),
                r_in.time,
            ),
        })
    }
}
