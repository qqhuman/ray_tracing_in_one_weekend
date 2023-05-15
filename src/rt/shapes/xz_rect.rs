use std::sync::Arc;

use crate::rt::{materials::Material, random_f64_between, ray::Ray, vec3::Vec3, Point3};

use super::{aabb::Aabb, hit_record::HitRecord, Hittable};

pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> XzRect {
        XzRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let p = r.at(t);
        Some(HitRecord::new(
            p,
            t,
            u,
            v,
            r,
            outward_normal,
            self.material.as_ref(),
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn pdf_value(&self, origin: Point3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(origin, v, 0.0), 0.001, f64::INFINITY) {
            None => 0.0,
            Some(rec) => {
                let area = (self.x1 - self.x0) * (self.z1 - self.z0);
                let distance_squared = rec.t * rec.t * v.length_squared();
                let cosine = f64::abs(Vec3::dot(v, rec.normal) / v.length());
                distance_squared / (cosine * area)
            }
        }
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let random_point = Point3::new(
            random_f64_between(self.x0, self.x1),
            self.k,
            random_f64_between(self.z0, self.z1),
        );
        random_point - origin
    }
}
