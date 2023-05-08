use std::sync::Arc;

use crate::rt::{degrees_to_radians, ray::Ray, vec3::Vec3, Point3};

use super::{aabb::Aabb, hit_record::HitRecord, Hittable};

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hittable.bounding_box(0.0, 1.0).map(|bbox| {
            let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i = i as f64;
                        let j = j as f64;
                        let k = k as f64;
                        let x = i * bbox.max.x + (1.0 - i) * bbox.min.x;
                        let y = j * bbox.max.y + (1.0 - j) * bbox.min.y;
                        let z = k * bbox.max.z + (1.0 - k) * bbox.min.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;
                        let tester = Vec3::new(newx, y, newz);
                        for c in 0..3 {
                            min.set(c, min.get(c).min(tester.get(c)));
                            max.set(c, max.get(c).max(tester.get(c)));
                        }
                    }
                }
            }

            return Aabb::new(min, max);
        });

        RotateY {
            hittable,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.set(
            0,
            self.cos_theta * r.origin.get(0) - self.sin_theta * r.origin.get(2),
        );
        origin.set(
            2,
            self.sin_theta * r.origin.get(0) + self.cos_theta * r.origin.get(2),
        );

        direction.set(
            0,
            self.cos_theta * r.direction.get(0) - self.sin_theta * r.direction.get(2),
        );
        direction.set(
            2,
            self.sin_theta * r.direction.get(0) + self.cos_theta * r.direction.get(2),
        );

        let rotated_r = Ray::new(origin, direction, r.time);
        let mut rec = self.hittable.hit(&rotated_r, t_min, t_max)?;

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.set(
            0,
            self.cos_theta * rec.p.get(0) + self.sin_theta * rec.p.get(2),
        );
        p.set(
            2,
            -self.sin_theta * rec.p.get(0) + self.cos_theta * rec.p.get(2),
        );

        normal.set(
            0,
            self.cos_theta * rec.normal.get(0) + self.sin_theta * rec.normal.get(2),
        );
        normal.set(
            2,
            -self.sin_theta * rec.normal.get(0) + self.cos_theta * rec.normal.get(2),
        );

        rec.p = p;
        rec.set_face_normal(&rotated_r, normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
