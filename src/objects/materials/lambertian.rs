use super::Material;
use crate::objects::HitRecord;
use crate::rand_unit;
use crate::vec3;
use crate::vec3::unit;
use crate::Ray;
use crate::Vec3;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + unit(&Vec3::from(rand_unit()));
        if vec3::near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }
        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
