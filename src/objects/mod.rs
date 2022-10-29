use std::sync::Arc;

use crate::vec3::inverse;
use crate::{Ray, Vec3};
mod materials;
mod sphere;
pub use materials::Dielectrics;
pub use materials::Lambertian;
use materials::Material;
pub use materials::Metal;
pub use sphere::Sphere;

pub trait Objects: Send + Sync + 'static {
    fn pos(&self) -> &Vec3;
    fn be_hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_recod: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Default::default(),
            normal: Default::default(),
            t: Default::default(),
            material: Arc::new(materials::Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
            front_face: Default::default(),
        }
    }
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, material: Arc<dyn Material>) -> Self {
        HitRecord {
            point,
            normal,
            t,
            material,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = (inverse(&ray.dir) * &outward_normal)[0] < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
