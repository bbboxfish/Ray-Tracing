use crate::hittable::{HitRecord,Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }

    pub fn hit(&self,r:&Ray,ray_tmin:f64,ray_tmax:f64,rec:&mut HitRecord)->bool {
        let mut tmp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;
        for object in self.objects.iter() {
            if object.hit(r,ray_tmin,closest_so_far,&mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }
        hit_anything
    }
}
