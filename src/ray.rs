//acknowledgement : jj
use crate::vec3::{Point3, Vec3};
#[derive(Default,Clone,Copy)]
pub struct Ray {
    pub orig:Point3,
    pub dir:Vec3,
    pub tm:f64,
}
impl Ray{
    pub fn new(orig:Point3, dir: Vec3,) -> Self {
        Self { orig, dir, tm:0.0 }
    }

    pub fn new_time(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Self {
            orig,
            dir,
            tm,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}