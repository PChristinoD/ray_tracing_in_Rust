use crate::degrees_to_radians;
use crate::rand;
use crate::rand_in_unit_disk;
use crate::vec3::{cross, unit};
use crate::{Ray, Vec3};

pub struct Camera {
    pub orig: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        //cross product
        let w = unit(&(look_from - look_at));
        let u = unit(&cross(&vup, &w));
        let v = cross(&w, &u);

        let orig = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = -v * viewport_height * focus_dist;
        let lower_left_corner = orig - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;
        Camera {
            orig,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = rand_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.orig + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.orig - offset,
        )
    }
}
