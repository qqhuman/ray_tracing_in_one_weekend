use crate::rt::{
    random_i32_between, random_vec3_between,
    vec3::{self, Vec3},
    Point3,
};

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

const POINT_COUNT: i32 = 256;

impl Perlin {
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = (0..POINT_COUNT).collect();
        Perlin::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (0..n).rev() {
            let iu = i as usize;
            let target = random_i32_between(0, i) as usize;
            let tmp = p[iu];
            p[iu] = p[target];
            p[target] = tmp;
        }
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..c.len() {
            for j in 0..c[i].len() {
                for k in 0..c[i][j].len() {
                    let cijk = c[i][j][k];
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let weight_v = Vec3::new(u - i, v - j, w - k);
                    accum += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * Vec3::dot(cijk, weight_v);
                }
            }
        }
        accum
    }

    fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[vec3::ZERO; 2]; 2]; 2];
        for di in 0..c.len() {
            for dj in 0..c[di].len() {
                for dk in 0..c[di][dj].len() {
                    let i = ((i + di as i32) & 255) as usize;
                    let j = ((j + dj as i32) & 255) as usize;
                    let k = ((k + dk as i32) & 255) as usize;
                    let index = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[index];
                }
            }
        }
        Perlin::perlin_interp(c, u, v, w)
    }

    pub fn new() -> Perlin {
        Perlin {
            ranvec: (0..POINT_COUNT)
                .map(|_| random_vec3_between(-1.0, 1.0))
                .collect(),
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn turb(&self, p: Point3) -> f64 {
        self.turb_with_depth(p, 7)
    }

    pub fn turb_with_depth(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        let mut temp_p = p;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }

        accum.abs()
    }
}
