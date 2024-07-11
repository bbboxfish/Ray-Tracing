mod color;
mod vec3; 
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
use hittable_list::HittableList;
use hittable::{HitRecord, Hittable};
use sphere::Sphere;
use ray::Ray;
use vec3::{Color,Point3,Vec3};
use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::f64::INFINITY;
use std::fs::File;
use std::sync::Arc;
use std::rc::Rc;
const AUTHOR: &str = "box fish";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn ray_color(r:&Ray,world :&dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r,0.0,INFINITY,&mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = Vec3::unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
fn main() {
    let path = "output/test.jpg";
    let image_width = 800;
    let image_height = 450;
    let quality = 60;
    let bar: ProgressBar = if is_ci() {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let viewport_height = 2.0;
    let aspect_ratio = 16.0 / 9.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0),0.5,)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0),100.0,)));
    
    let camera_center = Point3::zero();
    //计算视窗边缘的矢量
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    //像素之间的增量
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
                             - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    for i in 0..image_width {
        for j in 0..image_height {
            let pixel_center = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let color_vec = ray_color(&r,&world);
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
