use crate::{
    vec3::{inverse, unit},
    Ray, Vec3,
};

use super::Material;

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &crate::Ray,
        hit_record: &crate::objects::HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut crate::Ray,
    ) -> bool {
        let reflected = reflect(&unit(&ray_in.dir), &hit_record.normal);
        *scattered = Ray::new(hit_record.point, reflected);
        *attenuation = self.albedo;
        (inverse(&scattered.dir) * hit_record.normal)[0] > 0.0
    }
}
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - n * (inverse(v) * n)[0] * 2.0
}
