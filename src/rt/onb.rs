use super::vec3::{self, Vec3};

pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Onb {
    pub fn build_from_w(n: Vec3) -> Onb {
        let w = Vec3::unit_vector(n);
        let a = if w.x.abs() > 0.9 {
            vec3::UNIT_Y
        } else {
            vec3::UNIT_X
        };
        let v = Vec3::unit_vector(Vec3::cross(w, a));
        let u = Vec3::cross(w, v);
        Onb { u, v, w }
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
