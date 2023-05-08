use std::sync::Arc;

use crate::rt::{materials::Material, ray::Ray, vec3::Vec3, Point3};

use super::{aabb::Aabb, sphere::Sphere, HitRecord, Hittable};

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center(r.time)) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(outward_normal);
        Some(HitRecord::new(
            p,
            root,
            u,
            v,
            r,
            outward_normal,
            self.material.as_ref(),
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let center0 = self.center(time0);
        let center1 = self.center(time1);
        let box0 = Aabb::new(center0 - radius, center0 + radius);
        let box1 = Aabb::new(center1 - radius, center1 + radius);
        Some(Aabb::surrounding_box(&box0, &box1))
    }
}
