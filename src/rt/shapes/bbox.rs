use std::sync::Arc;

use crate::rt::{materials::Material, ray::Ray, Point3};

use super::{
    aabb::Aabb, hit_record::HitRecord, hittable_list::HittableList, xy_rect::XyRect,
    xz_rect::XzRect, yz_rect::YzRect, Hittable,
};

pub struct Bbox {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl Bbox {
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Bbox {
        let mut sides = HittableList::default();

        sides.add(Arc::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            Arc::clone(&material),
        )));
        sides.add(Arc::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            Arc::clone(&material),
        )));

        sides.add(Arc::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            Arc::clone(&material),
        )));
        sides.add(Arc::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            Arc::clone(&material),
        )));

        sides.add(Arc::new(YzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            Arc::clone(&material),
        )));

        sides.add(Arc::new(YzRect::new(
            p0.y, p1.y, p0.z, p1.z, p0.x, material,
        )));

        Bbox {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for Bbox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}
