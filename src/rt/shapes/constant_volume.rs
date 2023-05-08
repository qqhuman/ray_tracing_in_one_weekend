use std::sync::Arc;

use crate::rt::{
    color::Color,
    materials::isotropic::Isotropic,
    random_f64,
    ray::Ray,
    textures::{solid_color::SolidColor, Texture},
    vec3::Vec3,
};

use super::{aabb::Aabb, hit_record::HitRecord, Hittable};

pub struct ConstantVolume {
    boundary: Arc<dyn Hittable>,
    phase_function: Isotropic,
    neg_inv_density: f64,
}

impl ConstantVolume {
    pub fn from_color(b: Arc<dyn Hittable>, d: f64, a: Color) -> ConstantVolume {
        ConstantVolume::from_texture(b, d, Arc::new(SolidColor::new(a)))
    }

    pub fn from_texture(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> ConstantVolume {
        ConstantVolume {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::from_texture(a),
        }
    }
}

impl Hittable for ConstantVolume {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let enable_debug = false;
        let debugging = enable_debug && random_f64() < 0.00001;

        let mut rec1 = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY)?;
        let mut rec2 = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY)?;

        if debugging {
            println!("\nt_min={:?}, t_max={:?}\n", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }

        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        if debugging {
            println!(
                "hit_distance = {:?}\nrec.t = {:?}\nrec.p = {:?}\n",
                hit_distance, t, p
            );
        }

        Some(HitRecord {
            p,
            t,
            normal: Vec3::new(1.0, 0.0, 0.0), // arbitrary
            front_face: true,
            material: &self.phase_function, // also arbitrary
            u: 0.0,
            v: 0.0,
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
