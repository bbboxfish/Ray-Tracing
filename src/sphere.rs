use crate::hittable::*;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;
use crate::util;
use crate::aabb::*;
use std::sync::Arc;
pub struct Sphere {
    // pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
    center1: Point3,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius,radius,radius);
        Self {
            center1:center,
            radius,
            mat: material,
            is_moving: false,
            center_vec: Vec3::default(),
            bbox: Aabb::new_point(&(center - rvec), &(center + rvec)),
        }
    }

    pub fn new_center2(center1: Point3,center2: Point3,radius: f64,material: Arc<dyn Material> ) -> Self {
        let rvec = Vec3::new(radius,radius,radius);
        let box1 = Aabb::new_point(&(center1 - rvec), &(center1 + rvec));
        let box2 = Aabb::new_point(&(center2 - rvec), &(center2 + rvec));

        Self {
            center1,
            radius,
            mat: material,
            is_moving: true,
            center_vec: center2 - center1,
            bbox: Aabb::new_box(&box1, &box2),
        }
    }

    fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + self.center_vec * time
    }

    fn get_sphere_uv(p :Point3) -> (f64,f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + util::PI;
         
        (phi / (2.0 * util::PI), theta / util::PI)
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.sphere_center(r.tm) 
        } else { 
            self.center1 
        };
        let oc = center - r.orig;
        let a = r.dir.squared_length();
        let h = Vec3::dot(r.dir, oc);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center1) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        (hit_record.u, hit_record.v) = Self::get_sphere_uv(outward_normal);
        hit_record.mat = Some(Arc::clone(&self.mat));
        return true;
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
