use std::sync::Arc;

use crate::rt::{
    color::Color,
    random_in_unit_sphere,
    ray::Ray,
    shapes::hit_record::HitRecord,
    textures::{solid_color::SolidColor, Texture},
};

use super::Material;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(albedo: Color) -> Isotropic {
        Isotropic::from_texture(Arc::new(SolidColor::new(albedo)))
    }

    pub fn from_texture(albedo: Arc<dyn Texture>) -> Isotropic {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, rec.p),
            Ray::new(rec.p, random_in_unit_sphere(), r_in.time),
        ))
    }
}
