use crate::rt::{onb::Onb, random_cosine_direction, vec3::Vec3, PI};

use super::Pdf;

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> CosinePdf {
        CosinePdf {
            uvw: Onb::build_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = Vec3::dot(Vec3::unit_vector(direction), self.uvw.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(random_cosine_direction())
    }
}
