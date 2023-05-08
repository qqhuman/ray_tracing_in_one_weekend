use super::vec3::Vec3;

pub type Color = Vec3;
pub const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

pub const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
