use crate::rt::{random_f64, vec3::Vec3};

use super::Pdf;

pub struct MixturePdf<'a, 'b> {
    p0: &'a dyn Pdf,
    p1: &'b dyn Pdf,
}

impl<'a, 'b> MixturePdf<'a, 'b> {
    pub fn new(p0: &'a dyn Pdf, p1: &'b dyn Pdf) -> MixturePdf<'a, 'b> {
        MixturePdf { p0, p1 }
    }
}

impl<'a, 'b> Pdf for MixturePdf<'a, 'b> {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_f64() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
