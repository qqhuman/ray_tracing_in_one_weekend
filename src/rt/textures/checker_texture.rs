use std::sync::Arc;

use crate::rt::{color::Color, Point3};

use super::{solid_color::SolidColor, Texture};

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_colors(even: Color, odd: Color) -> CheckerTexture {
        CheckerTexture::from_textures(
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
        )
    }

    pub fn from_textures(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        let texture = if sines < 0.0 { &self.odd } else { &self.even };
        texture.value(u, v, p)
    }
}
