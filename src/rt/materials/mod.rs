use self::scatter_record::ScatterRecord;

use super::{
    color::{self, Color},
    ray::Ray,
    shapes::hit_record::HitRecord,
    Point3,
};

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;
pub mod scatter_record;

pub trait Material: Sync + Send {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Color {
        color::BLACK
    }
}
