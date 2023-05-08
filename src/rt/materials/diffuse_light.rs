use std::sync::Arc;

use crate::rt::{
    color::Color,
    ray::Ray,
    shapes::hit_record::HitRecord,
    textures::{solid_color::SolidColor, Texture},
    Point3,
};

use super::Material;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_color(emit: Color) -> DiffuseLight {
        DiffuseLight::from_texture(Arc::new(SolidColor::new(emit)))
    }

    pub fn from_texture(emit: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
