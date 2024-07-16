use std::sync::Arc;
use crate::util;
use crate::hittable::{Hittable,HitRecord,};
use crate::material::{Material,Isotropic};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Color;
use crate::aabb::Aabb;
use crate::texture::Texture;
use crate::interval::{self,Interval};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(a)),
        }
    }
    pub fn new_with_color(b: Arc<dyn Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new_with_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && util::random_double() < 0.00001;

        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, &Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, &Interval::new(rec1.t + 0.0001, util::INFINITY), &mut rec2) {
            return false;
        }

        if debugging {
            eprintln!("\nray_tmin={} ray_tmax={}", rec1.t, rec2.t);
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * util::random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {:?}", rec.p);
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat = Some(Arc::clone(&self.phase_function));

        true
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}