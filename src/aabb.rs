use crate::vec3::Point3;
use crate::interval::*;
use crate::ray::Ray;
#[derive(Clone,Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}
impl Aabb {
    pub fn new(x: &Interval,y: &Interval,z: &Interval) -> Self {
        Self {
            x:(*x).clone(),
            y:(*y).clone(),
            z:(*z).clone()
        }
    }

    pub fn new_point(a: &Point3, b: &Point3) -> Self {
        let mut _self = Self {
            x: Interval::new((a.x()).min(b.x()), (a.x()).max(b.x())),
            y: Interval::new((a.y()).min(b.y()), (a.y()).max(b.y())),
            z: Interval::new((a.z()).min(b.z()), (a.z()).max(b.z())),
        };
        _self.pad_to_minimums();
        _self
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        for a in 0..3 {
            let inv0 = 1.0 / r.dir[a];
            let orig = r.orig[a];

            let mut t0 = (self.axis(a).min - orig) * inv0;
            let mut t1 = (self.axis(a).max - orig) * inv0;

            if inv0 < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    fn pad_to_minimums(&mut self) {
        //每个维度的大小至少为delta
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z.expand(delta);
        }
    }

    pub fn new_box(box1: &Aabb,box2: &Aabb) -> Self {
        Self {
            x: Interval::new_interval(&box1.x, &box2.x),
            y: Interval::new_interval(&box1.y, &box2.y),
            z: Interval::new_interval(&box1.z, &box2.z),
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }
    
    pub const EMPTY: Aabb = Aabb {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };
    pub const UNIVERSE: Aabb = Aabb {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };
}