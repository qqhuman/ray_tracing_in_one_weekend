use std::sync::Arc;

use super::Material;
use crate::rt::{
    color::Color,
    random_unit_vector,
    ray::Ray,
    shapes::hit_record::HitRecord,
    textures::{solid_color::SolidColor, Texture},
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((
            self.albedo.value(rec.u, rec.v, rec.p),
            Ray::new(rec.p, scatter_direction, r_in.time),
        ))
    }
}
