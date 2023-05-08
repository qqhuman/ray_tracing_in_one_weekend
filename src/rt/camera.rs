use super::{
    degrees_to_radians, random_f64_between, random_in_unit_disk, ray::Ray, vec3::Vec3, Point3,
};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,

    pub aspect_ratio: f64,
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
        aspect_ratio: f64,
        width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time0,
            time1,
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as usize,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random_f64_between(self.time0, self.time1),
        )
    }
}
