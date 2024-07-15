use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::interval::Interval;
use crate::material::Material;
use std::sync::Arc;
use crate::aabb::Aabb;
#[derive(Default,Clone)]
pub struct HitRecord {
    pub p : Point3,
    pub normal : Vec3,
    pub t : f64,//交点处光线的t值
    pub front_face : bool,
    pub mat: Option<Arc<dyn Material>>,
    pub u: f64,
    pub v: f64,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub trait Hittable {
    // fn hit(&self ,r:&Ray,ray_tmin:f64,ray_tmax:f64,rec:&mut HitRecord)->bool;
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> &Aabb;
}