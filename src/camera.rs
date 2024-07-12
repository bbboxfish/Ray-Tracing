use crate::vec3::{Vec3,Point3,Color};
use crate::hittable::{Hittable,HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::color::{self, linear_to_gamma, write_color};
use crate::util::{random_double};
use image::{ImageBuffer, RgbImage,Rgb}; //接收render传回来的图片，在main中文件输出
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{rngs::ThreadRng, Rng};
use std::sync::{Arc, Mutex};
use std::thread;
pub const INFINITY: f64 = std::f64::INFINITY;
use std::fs::File;
const INTENSITY: Interval = Interval{ min: 0.0, max: 0.999 };
const AUTHOR: &str = "box fish";
pub struct Camera {
    pub aspect_ratio: f64,  
    pub image_width: u32,   
    pub image_height: u32,      
    center: Point3,       
    pixel00_loc: Point3,    // (0,0)位置
    pixel_delta_u: Vec3,    // 向右增量
    pixel_delta_v: Vec3,    // 向下增量
    pub samples_per_pixel: usize, //每个像素的随机样本计数
    pub max_depth: i32,//反射次数上限
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }
}
impl Camera {
    fn ray_color(r:&Ray,depth: i32,world :&dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if depth <= 0 {
            return Color::default();
        }
        if world.hit(r, &Interval::new(0.001,INFINITY),&mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if let Some(mat) = rec.mat.clone() {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * Self::ray_color(&scattered, depth - 1, world);
                }
            }
            return Color::default();
        }
        let unit_direction = Vec3::unit_vector(r.dir);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    fn initialize(&mut self) {
        self.image_height = 450;

        self.center = Point3::default();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        //计算视窗边缘的矢量
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        //像素之间的增量
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        //计算左上角像素
        let viewport_upper_left = self.center
            - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn is_ci() -> bool {
        option_env!("CI").unwrap_or_default() == "true"
    }
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    pub fn sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.sample_square();
    
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
    
        Ray::new(ray_origin, ray_direction)
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let path = "output/test.jpg";
        let quality = 60;
        let bar: ProgressBar = if Self::is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        
        for i in 0..self.image_width {
            for j in 0..self.image_height {
                let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                // let color_vec = Self::ray_color(&r,world);
                let mut color_vec = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color_vec += Self::ray_color(&r,self.max_depth, world)/self.samples_per_pixel as f64;
                }
                color_vec.x = linear_to_gamma(color_vec.x);
                color_vec.y = linear_to_gamma(color_vec.y);
                color_vec.z = linear_to_gamma(color_vec.z);
                let pixel_color = [
                    (color_vec.x * 255.999) as u8,
                    (color_vec.y * 255.999) as u8,
                    (color_vec.z * 255.999) as u8,
                ];
                // let pixel_color = [
                //     255.999 * INTENSITY.clamp(color_vec.x) as u8,
                //     INTENSITY.clamp(color_vec.y) as u8 * 256.0 as u8,
                //     INTENSITY.clamp(color_vec.z) as u8 * 256.0 as u8,
                // ];
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
    // pub fn multi_render(&mut self, world: &dyn Hittable,thread_num:usize) {
    //     self.initialize();

    //     let path = "output/fast.jpg";
    //     let quality = 60;
        // let bar: ProgressBar = if Self::is_ci() {
        //     ProgressBar::hidden()
        // } else {
        //     ProgressBar::new((self.image_height * self.image_width) as u64)
        // };
        //多线程：
        // let multipb = MultiProgress::new();
        // let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        // let image = Arc::new(Mutex::new(img));
        // let cam = Arc::new(self);
        // let mut handles = vec![];
        // for index in 0..thread_num {
        //     let c = Arc::clone(&cam);
        //     let image = Arc::clone(&image);
            
        // }
        // for i in 0..self.image_width {
        //     for j in 0..self.image_height {
        //         let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        //         let ray_direction = pixel_center - self.center;
        //         let r = Ray::new(self.center, ray_direction);
        //         // let color_vec = Self::ray_color(&r,world);
        //         let mut color_vec = Vec3::zero();
        //         for _ in 0..self.samples_per_pixel {
        //             let r = self.get_ray(i, j);
        //             color_vec += Self::ray_color(&r, world)/self.samples_per_pixel as f64;
        //         }
        //         let pixel_color = [
        //             (color_vec.x * 255.999) as u8,
        //             (color_vec.y * 255.999) as u8,
        //             (color_vec.z * 255.999) as u8,
        //         ];
        //         write_color(pixel_color, &mut img, i as usize, j as usize);
        //         bar.inc(1);
        //     }
        // }
        // bar.finish();
        // println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
        // let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
        // let mut output_file: File = File::create(path).unwrap();
        // match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        //     Ok(_) => {}
        //     Err(_) => println!("Outputting image fails."),
        // }
    // }
}