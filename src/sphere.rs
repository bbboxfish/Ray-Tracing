use crate::vec3::*;
use crate::ray::Ray;
use crate::hittable::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center:Point3,radius:f64)->Self {
        Self{center , radius}
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - r.ori;
        let a = r.dir.squared_length();
        let h = Vec3::dot(r.dir,oc);
        let c = oc.squared_length() - self.radius*self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd)/a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
        hit_record.normal = (hit_record.p - self.center)/self.radius;
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        return true;
    }
}