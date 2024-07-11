use crate::hittable::{HitRecord,Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
#[derive(Clone,Default)]
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
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
    
}
