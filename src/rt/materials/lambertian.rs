use std::sync::Arc;

use super::{scatter_record::ScatterRecord, Material};
use crate::rt::{
    color::Color,
    pdfs::cosine_pdf::CosinePdf,
    ray::Ray,
    shapes::hit_record::HitRecord,
    textures::{solid_color::SolidColor, Texture},
    vec3::Vec3,
    PI,
};

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(albedo: Color) -> Lambertian {
        Lambertian::from_texture(Arc::new(SolidColor::new(albedo)))
    }

    pub fn from_texture(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::Diffuse {
            attenuation: self.albedo.value(rec.u, rec.v, rec.p),
            pdf: Box::new(CosinePdf::new(rec.normal)),
        })
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, Vec3::unit_vector(scattered.direction));
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
