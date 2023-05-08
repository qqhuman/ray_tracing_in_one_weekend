use self::{aabb::Aabb, hit_record::HitRecord};

use super::ray::Ray;

pub mod aabb;
pub mod bbox;
pub mod bvh_node;
pub mod constant_volume;
pub mod hit_record;
pub mod hittable_list;
pub mod mooving_sphere;
pub mod rotate_y;
pub mod sphere;
pub mod translate;
pub mod xy_rect;
pub mod xz_rect;
pub mod yz_rect;

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
