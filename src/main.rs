mod color;
mod vec3; 
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::{Color,Point3,Vec3};
use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::f64::INFINITY;
use std::fs::File;
use std::rc::Rc;
const AUTHOR: &str = "box fish";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

//image 3,4
fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.ori;
    let a = Vec3::dot(r.dir, r.dir);
    let b = -2.0 * Vec3::dot(r.dir,oc);
    let c = Vec3::dot(oc,oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
// fn ray_color(r: &Ray) -> Color {
//     if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
//         return Vec3::new(171.0/255.0, 132.0/255.0, 1.0); // Red color
//     }
//     let unit_direction = Vec3::unit_vector(r.dir);
//     let t = 0.5 * (unit_direction.y() + 1.0);
//     Color::new(
//         (1.0 - t) * 1.0 + t * 0.5,
//         (1.0 - t) * 1.0 + t * 0.7,
//         (1.0 - t) * 1.0 + t * 1.0,
//     )
// }
fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(r.dir);
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
// fn ray_color(r: &Ray) -> Color {
//     let unit_direction = Vec3::unit_vector(r.dir);
//     let t = 0.5 * (unit_direction.y() + 1.0);
//     let white = Color::new(1.0, 1.0, 1.0);
//     let blue = Color::new(0.5, 0.7, 1.0);
//     Color::new(
//         (1.0 - t) * white.x + t * blue.x,
//         (1.0 - t) * white.y + t * blue.y,
//         (1.0 - t) * white.z + t * blue.z,
//     )
// }

// fn ray_color(r:&Ray,world :&dyn Hittable) -> Color {
//     let mut rec = HitRecord::default();
//     if world.hit(r,0.0,INFINITY,&mut rec) {
//         return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
//     }
//     let unit_direction = Vec3::unit_vector(r.dir);
//     let t = 0.5 * (unit_direction.y() + 1.0);
//     (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
// }
fn main() {
    let path = "output/test.jpg";
    let width = 800;
    let height = 450;
    let quality = 60;
    let bar: ProgressBar = if is_ci() {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let viewport_height = 2.0;
    let aspect_ratio = 16.0 / 9.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    for i in 0..width {
        for j in 0..height {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color_vec = ray_color(&r);
            let pixel_color = [
                (color_vec.x * 255.999) as u8,
                (color_vec.y * 255.999) as u8,
                (color_vec.z * 255.999) as u8,
            ];
            write_color(pixel_color, &mut img, i as usize, j as usize);
            bar.inc(1);
        }
    }
    bar.finish();
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
