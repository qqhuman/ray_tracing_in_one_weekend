use crate::rt::color::Color;

use super::Texture;

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> SolidColor {
        SolidColor { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: crate::rt::Point3) -> Color {
        self.color_value
    }
}
