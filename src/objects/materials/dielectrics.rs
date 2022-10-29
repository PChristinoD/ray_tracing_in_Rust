use crate::{
    rand,
    vec3::{inverse, length},
    Ray, Vec3,
};

use super::Material;

pub struct Dielectrics {
    ir: f64,
}

impl Dielectrics {
    pub fn new(ir: f64) -> Self {
        Dielectrics { ir }
    }
}
impl Material for Dielectrics {
    fn scatter(
        &self,
        ray_in: &crate::Ray,
        hit_record: &crate::objects::HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut crate::Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let eta_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let cos_alpha = (-inverse(&ray_in.dir) * hit_record.normal)[0].min(1.0);
        let sin_alpha = (1.0 - cos_alpha * cos_alpha).sqrt();

        if eta_ratio * sin_alpha > 1.0 || reflectance(cos_alpha, eta_ratio) > rand() {
            *scattered = Ray::new(hit_record.point, reflect(&ray_in.dir, &hit_record.normal));
        } else {
            *scattered = Ray::new(
                hit_record.point,
                refracted(&ray_in.dir, &hit_record.normal, eta_ratio),
            );
        }

        true
    }
}

fn refracted(ray_in: &Vec3, normal: &Vec3, eta_ratio: f64) -> Vec3 {
    let cos_alpha = (-inverse(ray_in) * normal)[0].min(1.0);
    let v_para = (ray_in + normal * cos_alpha) * eta_ratio;
    let v_per = -normal * (1.0 - length(&v_para) * length(&v_para)).abs().sqrt();
    v_para + v_per
}
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - n * (inverse(v) * n)[0] * 2.0
}
fn reflectance(cos: f64, ir: f64) -> f64 {
    let mut r0 = (1.0 - ir) / (1.0 + ir);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
