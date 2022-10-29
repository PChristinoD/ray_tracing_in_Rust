use std::sync::Arc;

use log::info;

use crate::{vec3::inverse, Ray, Vec3};

use super::{materials::Material, HitRecord, Objects};

pub struct Sphere {
    pos: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            pos,
            radius,
            material,
        }
    }
}

impl Objects for Sphere {
    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    //let P(t)=A*t+B, A is origin of Ray, B is direction of Ray
    //let C = (c_x,c_y,c_z) which is center of sphere(position of sphere)
    // (P(t)-C)*(P(t)-C) = r*r
    // (A+t*B-C)*(A+t*B-C) = r*r
    // t^2*B^2+2t*B*(A-C)+(A-C)^2=r^2
    fn be_hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let a = inverse(&ray.dir) * &ray.dir;
        let half_b = inverse(&ray.dir) * (ray.orig - self.pos);
        let c = inverse(&(ray.orig - self.pos)) * (ray.orig - self.pos) - self.radius * self.radius;
        let discriminat = half_b[0] * half_b[0] - a[0] * c[0];
        if discriminat < 0.0 {
            return false;
        }
        let mut root = (-half_b[0] - discriminat.sqrt()) / a[0];
        if root < t_min || t_max < root {
            root = (-half_b[0] + discriminat.sqrt()) / a[0];
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_normal = (hit_record.point - self.pos()) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = self.material.clone();

        true
    }
}
