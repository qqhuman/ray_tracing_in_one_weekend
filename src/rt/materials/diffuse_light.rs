use std::sync::Arc;

use crate::rt::{
    color::{self, Color},
    ray::Ray,
    shapes::hit_record::HitRecord,
    textures::{solid_color::SolidColor, Texture},
    Point3,
};

use super::{scatter_record::ScatterRecord, Material};

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
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            color::BLACK
        }
    }
}
