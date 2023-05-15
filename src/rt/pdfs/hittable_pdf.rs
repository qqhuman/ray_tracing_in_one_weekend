use crate::rt::{shapes::Hittable, vec3::Vec3, Point3};

use super::Pdf;

pub struct HittablePdf<'a> {
    hittable: &'a dyn Hittable,
    o: Point3,
}

impl<'a> HittablePdf<'a> {
    pub fn new(hittable: &'a dyn Hittable, o: Point3) -> HittablePdf<'a> {
        HittablePdf { hittable, o }
    }
}

impl<'a> Pdf for HittablePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        self.hittable.pdf_value(self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.hittable.random(self.o)
    }
}
