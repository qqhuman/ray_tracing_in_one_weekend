use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const ZERO: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const UNIT_X: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub const UNIT_Y: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub const UNIT_Z: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn get(&self, i: i32) -> f64 {
        match i {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }

    pub fn set(&mut self, i: i32, value: f64) {
        match i {
            0 => self.x = value,
            1 => self.y = value,
            _ => self.z = value,
        }
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - 2.0 * Vec3::dot(v, n) * n;
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(-uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        return r_out_parallel + r_out_perp;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self::Output {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
