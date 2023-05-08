use std::{cmp::Ordering, sync::Arc};

use crate::rt::{random_i32_between, ray::Ray};

use super::{aabb::Aabb, hit_record::HitRecord, hittable_list::HittableList, Hittable};

pub struct BvhNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    bbox: Option<Aabb>,
}

impl BvhNode {
    pub fn from_slice(objects: &mut [Arc<dyn Hittable>], time0: f64, time1: f64) -> BvhNode {
        let axis = random_i32_between(0, 2);
        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            _ => BvhNode::box_z_compare,
        };

        let comparator_ordering = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
            if comparator(a, b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        };

        let (left, right) = match objects.split_first() {
            None => (None, None),
            Some((head, [])) => (Some(head.clone()), None),
            Some((head, [tail])) if comparator(head, tail) => {
                (Some(head.clone()), Some(tail.clone()))
            }
            Some((head, [tail])) => (Some(tail.clone()), Some(head.clone())),
            _ => {
                objects.sort_by(comparator_ordering);
                let mid = objects.len() / 2;
                let left: Arc<dyn Hittable> =
                    Arc::new(BvhNode::from_slice(&mut objects[0..mid], time0, time1));
                let right: Arc<dyn Hittable> =
                    Arc::new(BvhNode::from_slice(&mut objects[mid..], time0, time1));
                (Some(left), Some(right))
            }
        };

        let box_left = left
            .as_ref()
            .map(|h| h.bounding_box(time0, time1))
            .unwrap_or_default();
        let box_right = right
            .as_ref()
            .map(|h| h.bounding_box(time0, time1))
            .unwrap_or_default();
        let bbox = match (box_left, box_right) {
            (Some(box_left), Some(box_right)) => Some(Aabb::surrounding_box(&box_left, &box_right)),
            (Some(box_left), None) => Some(box_left),
            (None, Some(box_right)) => Some(box_right),
            _ => None,
        };

        Self { left, right, bbox }
    }

    pub fn from_list(list: &mut HittableList, time0: f64, time1: f64) -> BvhNode {
        BvhNode::from_slice(&mut list.objects[..], time0, time1)
    }

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: i32) -> bool {
        let box_a = a
            .bounding_box(0.0, 0.0)
            .expect("box_compare couldn't get box_a");
        let box_b = b
            .bounding_box(0.0, 0.0)
            .expect("box_compare couldn't get box_b");
        box_a.min.get(axis) < box_b.min.get(axis)
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a.as_ref(), b.as_ref(), 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a.as_ref(), b.as_ref(), 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a.as_ref(), b.as_ref(), 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.map(|b| b.hit(r, t_min, t_max)).unwrap_or(false) {
            return None;
        }

        let left = self.left.as_ref();
        let right = self.right.as_ref();

        let hit_left = left.map(|h| h.hit(r, t_min, t_max)).unwrap_or_default();
        let t_max = hit_left.as_ref().map(|h| h.t).unwrap_or(t_max);
        let hit_right = right.map(|h| h.hit(r, t_min, t_max)).unwrap_or_default();
        hit_right.or(hit_left)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
