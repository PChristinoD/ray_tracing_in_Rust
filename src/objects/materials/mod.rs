use crate::{Ray, Vec3};

use super::HitRecord;
mod dielectrics;
mod lambertian;
mod metal;
pub use dielectrics::Dielectrics;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Send + Sync + 'static {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
