use std::mem::swap;

use crate::rt::{ray::Ray, Point3};

#[derive(Copy, Clone)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Aabb {
        Aabb { min, max }
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let small = Point3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Point3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        Aabb::new(small, big)
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction.get(a);
            let mut t0 = (self.min.get(a) - r.origin.get(a)) * inv_d;
            let mut t1 = (self.max.get(a) - r.origin.get(a)) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }

    pub fn _hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let t0 = f64::min(
                (self.min.get(a) - r.origin.get(a)) / r.direction.get(a),
                (self.max.get(a) - r.origin.get(a)) / r.direction.get(a),
            );
            let t1 = f64::max(
                (self.min.get(a) - r.origin.get(a)) / r.direction.get(a),
                (self.max.get(a) - r.origin.get(a)) / r.direction.get(a),
            );

            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }
}
