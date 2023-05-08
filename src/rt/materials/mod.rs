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

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        color::BLACK
    }
}
