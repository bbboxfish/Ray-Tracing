use std::sync::Arc;
use crate::vec3::{Vec3,Point3};
use crate::material::Material;
use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::hittable::{HitRecord,Hittable};
use crate::hittable_list::HittableList;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = Vec3::cross(u, v);
        let normal = Vec3::unit_vector(n);
        Self {
            q,
            u,
            v,
            w: n / Vec3::dot(n, n),
            mat,
            bbox: Aabb::new_point(
                &q, &(q + u + v)
            ),
            normal,
            d: Vec3::dot(normal, q),
        }
    }

    pub fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
            return false;
        }

        rec.u = a;
        rec.v = b;

        true
    }
        
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(self.normal, r.dir);
        //射线与平面平行
        if denom.abs() < 1e-8 {
            return false;
        }
        //相交点射线区间之外
        let t = (self.d - Vec3::dot(self.normal, r.orig)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);

        let intersection = r.at(t);
       let planar_hitpt_vector = intersection - self.q;
       let alpha = Vec3::dot(self.w, Vec3::cross(planar_hitpt_vector, self.v));
       let beta = Vec3::dot(self.w, Vec3::cross(self.u, planar_hitpt_vector));

       if !self.is_interior(alpha, beta, rec) {
           return false;
       }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Arc::clone(&self.mat));
        rec.set_face_normal(r, self.normal);

        true
    }


    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
pub fn make_box(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    //两个对角顶点a和b的盒子

    let mut sides = HittableList::default();

    // 构造两个对角顶点，具有最小和最大的坐标。
    let min = Point3::new(
        a.x().min(b.x()),
        a.y().min(b.y()),
        a.z().min(b.z()),
    );
    let max = Point3::new(
        a.x().max(b.x()),
        a.y().max(b.y()),
        a.z().max(b.z()),
    );

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(
        Quad::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, Arc::clone(&mat))
    ));
    sides.add(Arc::new(
        Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz, dy, Arc::clone(&mat))
    ));
    sides.add(Arc::new(
        Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx, dy, Arc::clone(&mat))
    ));
    sides.add(Arc::new(
        Quad::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, Arc::clone(&mat))
    ));
    sides.add(Arc::new(
        Quad::new(Point3::new(min.x(), max.y(), max.z()), dx, -dz, Arc::clone(&mat))
    ));
    sides.add(Arc::new(
        Quad::new(Point3::new(min.x(), min.y(), min.z()), dx, dz, Arc::clone(&mat))
    ));

    Arc::new(sides)
}