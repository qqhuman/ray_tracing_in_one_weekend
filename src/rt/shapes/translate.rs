use std::sync::Arc;

use crate::rt::{ray::Ray, vec3::Vec3};

use super::{aabb::Aabb, hit_record::HitRecord, Hittable};

pub struct Translate {
    hittable: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        Translate { hittable, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        let mut rec = self.hittable.hit(&moved_r, t_min, t_max)?;
        rec.p = rec.p + self.offset;
        rec.set_face_normal(&moved_r, rec.normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let output_box = self.hittable.bounding_box(time0, time1)?;
        Some(Aabb::new(
            output_box.min + self.offset,
            output_box.max + self.offset,
        ))
    }
}
