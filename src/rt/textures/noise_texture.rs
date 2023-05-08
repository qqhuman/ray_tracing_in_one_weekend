use crate::rt::{
    color::{self, Color},
    noise::perlin::Perlin,
    Point3,
};

use super::Texture;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        color::WHITE * 0.5 * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(p)))
    }
}
