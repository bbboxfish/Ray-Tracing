use crate::util::{INFINITY};
use crate::ray::Ray;
use crate::hittable::{HitRecord,Hittable};
use crate::vec3::{Point3, Vec3};
#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min : INFINITY,max: -INFINITY//EMPTY
        }
    }
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_interval(a: &Interval, b: &Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    // pub fn contains(&self, x: f64) -> bool {
    //     self.min <= x && x <= self.max
    // }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn merge(&mut self, other: &Interval) {
        if other.min < self.min {
            self.min = other.min;
        }
        if other.max > self.max {
            self.max = other.max;
        }
    }

    pub fn expand(&self,delta: f64) -> Interval {
        let padding = delta/2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
    pub const EMPTY: Interval = Interval {
        min: INFINITY,
        max: -INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        min: -INFINITY,
        max: INFINITY,
    };
}
