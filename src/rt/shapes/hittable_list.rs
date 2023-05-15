use std::sync::Arc;

use crate::rt::{random_i32_between, ray::Ray, vec3::Vec3, Point3};

use super::{aabb::Aabb, HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new_from_object(object: Arc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn new_from_objects(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        return closest_hit;
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let (head, tail) = self.objects.split_first()?;
        let mut output_box = head.bounding_box(time0, time1)?;
        for object in tail {
            let tail_box = object.bounding_box(time0, time1)?;
            output_box = Aabb::surrounding_box(&output_box, &tail_box);
        }
        return Some(output_box);
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        self.objects
            .iter()
            .map(|obj| weight * obj.pdf_value(o, v))
            .sum()
    }

    fn random(&self, o: Point3) -> Vec3 {
        let index = random_i32_between(0, self.objects.len() as i32 - 1) as usize;
        self.objects[index].random(o)
    }
}
