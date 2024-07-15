use crate::aabb::{surrounding_box, Aabb};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::rtweekend::random_usize_range;
use std::sync::Arc;

#[derive(Clone)]
pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box_0: Aabb,
}

impl BvhNode {
    pub fn new_list(list: HittableList, time0: f64, time1: f64) -> Arc<dyn Hittable> {
        Self::new_vec(list.objects, time0, time1)
    }

    pub fn new_vec(
        mut objects: Vec<Arc<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Arc<dyn Hittable> {
        let axis = random_usize_range(0, 2);
        match objects.len() {
            0 => {
                panic!();
            }
            1 => Arc::new(Self {
                left: objects[0].clone(),
                right: objects[0].clone(),
                box_0: objects[0].bounding_box(time0, time1).unwrap(),
            }),
            2 => {
                objects.sort_by(|a, b| {
                    a.bounding_box(time0, time1).unwrap().minimum[axis]
                        .partial_cmp(&b.bounding_box(time0, time1).unwrap().minimum[axis])
                        .unwrap()
                });
                let l = objects[0].clone();
                let r = objects[1].clone();
                let b_0 = surrounding_box(
                    l.bounding_box(time0, time1).unwrap(),
                    r.bounding_box(time0, time1).unwrap(),
                );
                Arc::new(Self {
                    left: l,
                    right: r,
                    box_0: b_0,
                })
            }
            _ => {
                objects.sort_by(|a, b| {
                    a.bounding_box(time0, time1).unwrap().minimum[axis]
                        .partial_cmp(&b.bounding_box(time0, time1).unwrap().minimum[axis])
                        .unwrap()
                });
                let mut left_objects = objects;
                let right_objects = left_objects.split_off(left_objects.len() / 2);
                let l = Self::new_vec(left_objects, time0, time1);
                let r = Self::new_vec(right_objects, time0, time1);
                let b_0 = surrounding_box(
                    l.bounding_box(time0, time1).unwrap(),
                    r.bounding_box(time0, time1).unwrap(),
                );
                Arc::new(Self {
                    left: l,
                    right: r,
                    box_0: b_0,
                })
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.box_0.hit(ray, t_min, t_max) {
            if let Some(hit_left) = self.left.hit(ray, t_min, t_max) {
                return if let Some(hit_right) = self.right.hit(ray, t_min, hit_left.t) {
                    Some(hit_right)
                } else {
                    Some(hit_left)
                };
            } else if let Some(hit_right) = self.right.hit(ray, t_min, t_max) {
                return Some(hit_right);
            }
        }
        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.box_0)
    }
}