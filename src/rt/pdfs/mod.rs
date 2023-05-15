use super::vec3::Vec3;

pub mod cosine_pdf;
pub mod hittable_pdf;
pub mod mixture_pdf;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
