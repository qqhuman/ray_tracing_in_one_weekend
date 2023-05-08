use crate::rt::{materials::Material, ray::Ray, vec3::Vec3, Point3};

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        t: f64,
        u: f64,
        v: f64,
        r: &Ray,
        outward_normal: Vec3,
        material: &'a dyn Material,
    ) -> HitRecord<'a> {
        let (front_face, normal) = get_face_normal(r, outward_normal);
        HitRecord {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        (self.front_face, self.normal) = get_face_normal(r, outward_normal);
    }
}

fn get_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (front_face, normal)
}
