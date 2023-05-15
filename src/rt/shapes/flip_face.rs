use std::sync::Arc;

use crate::rt::ray::Ray;

use super::{aabb::Aabb, hit_record::HitRecord, Hittable};

pub struct FlipFace {
    hittable: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(hittable: Arc<dyn Hittable>) -> FlipFace {
        FlipFace { hittable }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = self.hittable.hit(r, t_min, t_max)?;
        hit.front_face = !hit.front_face;
        Some(hit)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.hittable.bounding_box(time0, time1)
    }
}
