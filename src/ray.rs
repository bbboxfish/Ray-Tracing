//acknowledgement : jj
use crate::vec3::Vec3;
#[derive(Default,Clone,Copy)]
pub struct Ray {
    pub ori:Vec3,
    pub dir:Vec3,
    // pub time:f64,
}
impl Ray{
    pub fn new(ori: Vec3, dir: Vec3) -> Self {
        Self { ori, dir }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + self.dir * t
    }
}