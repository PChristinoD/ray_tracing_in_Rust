use std::sync::Arc;

use log::info;

use crate::{
    objects::{HitRecord, Objects},
    Ray,
};

#[derive(Default)]
pub struct ObjectList {
    objects: Vec<Arc<dyn Objects>>,
}

impl ObjectList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: Arc<dyn Objects>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn be_hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closet_so_far = t_max;

        // reference of Rc??????? WTF
        for object in &self.objects {
            if object.be_hit(ray, t_min, closet_so_far, &mut tmp_rec) {
                hit_anything = true;
                closet_so_far = tmp_rec.t;
                *hit_record = tmp_rec.clone();
            }
        }

        hit_anything
    }
}
