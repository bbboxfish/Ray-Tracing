mod color;
mod interval;
mod vec3; 
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod material;
mod aabb;
mod texture;
mod bvh;
mod rtw_stb_image;
pub mod util;
pub mod camera;
mod perlin;
use texture::{SolidColor,Texture,CheckerTexture,ImageTexture,NoiseTexture};
use bvh::BvhNode;
use camera::Camera;
use util::{random_double, random_double_range};
use crate::material::{Lambertian,Metal,Material};
use interval::Interval;
use hittable_list::HittableList;
use hittable::{HitRecord, Hittable};
use sphere::Sphere;
use ray::Ray;
use vec3::{Color,Point3,Vec3};
use color::write_color;
use image::{ColorType, ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
pub const INFINITY: f64 = std::f64::INFINITY;
use std::default;
use std::fs::File;
use std::sync::Arc;
const AUTHOR: &str = "box fish";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}


fn random_spheres(){ 
    let mut world = HittableList::default();
    let checker:Arc::<dyn Texture + Send + Sync> = Arc::new(
                CheckerTexture::new_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9))
            );
    // let material_ground = Arc::new(material::Lambertian::new(Color::new(0.757, 1.0, 0.756)));
    // world.add(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, -1.0),1000.0,material_ground)));
    let ground_material: Arc<dyn Material> = Arc::new(
        Lambertian::new_texture(Arc::clone(&checker))
        );
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material
    )));

    
    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9*random_double(),0.2,b as f64+0.9*random_double());
            
            if (center-Point3::new(4.0,0.2,0.0)).length() > 0.9 {
                let mut sphere_material: Arc<dyn Material> = 
                if choose_mat < 0.8 {
                    let albedo = Color::random()*Color::random();
                    Arc::new(material::Lambertian::new(albedo))
                } else if choose_mat < 0.95{
                    let albedo = Color::random_range(0.5,1.0);
                    let fuzz = random_double_range(0.0,0.5);
                    Arc::new(material::Metal::new(albedo,fuzz))
                } else {
                    Arc::new(material::Dielectric::new(1.5))
                };
                let center2 = center + vec3::Vec3::new(0.0, util::random_double_range(0.0, 0.5), 0.0);
                world.add(Arc::new(Sphere::new_center2(center,center2,0.2,sphere_material)));
            }
        }
    }

    let material1 = Arc::new(material::Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0,1.0,0.0),1.0,material1)));

    let material2 = Arc::new(material::Lambertian::new(Color::new(0.4,0.2,0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0,1.0,0.0),1.0,material2)));

    let material3 = Arc::new(material::Metal::new(Color::new(0.7,0.7,0.99),0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0,1.0,0.0),1.0,material3)));
    
    let bvh_node = Arc::new(BvhNode::new(&mut world));
    world = HittableList::new(bvh_node);

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 1;//500
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookat = Point3::new(0.0,0.0,0.0);
    cam.lookfrom = Point3::new(13.0,2.0,3.0);
    cam.vup = Vec3::new(0.0,1.0,0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&world);
}

fn two_spheres() {
    let mut world = HittableList::default();

    let checker1:Arc::<dyn Texture + Send + Sync> = Arc::new(
        CheckerTexture::new_color(0.32, Color::new(0.95, 0.78, 0.75), Color::new(0.9, 0.9, 0.9))
    );
    let checker2:Arc::<dyn Texture + Send + Sync> = Arc::new(
        CheckerTexture::new_color(0.32, Color::new(0.5, 0.8, 0.8), Color::new(0.9, 0.9, 0.9))
    );
    let material1: Arc<dyn Material> = Arc::new(
        Lambertian::new_texture(Arc::clone(&checker1))
        );
    let material2: Arc<dyn Material> = Arc::new(
        Lambertian::new_texture(Arc::clone(&checker2))
        );
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material1
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material2
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;
    print!("yyy");
    cam.render(&world);
}

fn earth() {
    let earth_texture:Arc::<dyn Texture + Send + Sync> = 
    Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Arc<dyn Material> = Arc::new(Lambertian::new_texture(Arc::clone(&earth_texture)));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&HittableList::new(globe));
}

fn two_perlin_spheres() {
    let mut world = HittableList::default();

    let pertext: Arc::<dyn Texture + Send + Sync> = Arc::new(NoiseTexture::default());
    world.add(Arc::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new_texture(Arc::clone(&pertext)))
        )
    ));
    world.add(Arc::new(
        Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::new(Lambertian::new_texture(Arc::clone(&pertext)))
        )
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 800;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 4 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        _ => (),
    }
}