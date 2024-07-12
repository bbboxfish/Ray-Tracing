mod color;
mod interval;
mod vec3; 
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod material;
pub mod util;
pub mod camera;
use camera::Camera;
use crate::material::{Lambertian,Metal,Material};
use interval::Interval;
use hittable_list::HittableList;
use hittable::{HitRecord, Hittable};
use sphere::Sphere;
use ray::Ray;
use vec3::{Color,Point3,Vec3};
use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
pub const INFINITY: f64 = std::f64::INFINITY;
use std::fs::File;
use std::sync::Arc;
const AUTHOR: &str = "box fish";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}


fn main() { 
    let mut world = HittableList::default();
    // world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0),0.5,)));
    // world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0),100.0,)));

    let material_ground = Arc::new(material::Lambertian::new(Color::new(0.757, 1.0, 0.756)));
    let material_center = Arc::new(material::Lambertian::new(Color::new(0.8, 0.4, 0.3)));
    let material_left = Arc::new(material::Dielectric::new(1.50));
    let material_right = Arc::new(material::Metal::new(Color::new(0.4, 0.7, 0.99),1.0));

    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0),100.0,material_ground,)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2),0.5,material_center,)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, -0.0, -1.0),0.5,material_left,)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0),0.5,material_right,)));
    
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 800;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}
