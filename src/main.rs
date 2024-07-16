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
mod qard;
use qard::Quad;
use crate::qard::make_box;
use texture::{SolidColor,Texture,CheckerTexture,ImageTexture,NoiseTexture};
use bvh::BvhNode;
use camera::Camera;
use util::{random_double, random_double_range};
use crate::material::{Lambertian,Metal,Material,DiffuseLight};
use interval::Interval;
use hittable_list::HittableList;
use hittable::{HitRecord, Hittable, RotateY,Translate};
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
    cam.background = Color::new(0.7, 0.8, 1.0);
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
    cam.background = Color::new(0.7, 0.8, 1.0);

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
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&HittableList::new(globe));
}

fn two_perlin_spheres() {
    let mut world = HittableList::default();

    let pertext: Arc::<dyn Texture + Send + Sync> = Arc::new(NoiseTexture::new(4.0));
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
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::default();

    // Material
    let left_red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quad
    world.add(
        Arc::new(Quad::new(
            Point3::new(-3.0, -2.0, 5.0),
            vec3::Vec3::new(0.0, 0.0, -4.0),
            vec3::Vec3::new(0.0, 4.0, 0.0),
            left_red
        ))
    );
    world.add(
        Arc::new(Quad::new(
            Point3::new(-2.0, -2.0, 0.0),
            vec3::Vec3::new(4.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 4.0, 0.0),
            back_green
        ))
    );
    world.add(
        Arc::new(Quad::new(
            Point3::new(3.0, -2.0, 1.0),
            vec3::Vec3::new(0.0, 0.0, 4.0),
            vec3::Vec3::new(0.0, 4.0, 0.0),
            right_blue
        ))
    );
    world.add(
        Arc::new(Quad::new(
            Point3::new(-2.0, 3.0, 1.0),
            vec3::Vec3::new(4.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 4.0),
            upper_orange
        ))
    );
    world.add(
        Arc::new(Quad::new(
            Point3::new(-2.0, -3.0, 5.0),
            vec3::Vec3::new(4.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, -4.0),
            lower_teal
        ))
    );

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::default();

    let pertext: Arc<dyn Texture + Send + Sync> = Arc::new(NoiseTexture::new(4.0));
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
            Arc::new(Lambertian::new_texture(pertext))
        )
    ));

    // let difflight = Arc::new(DiffuseLight::new_with_color(Color::new(4.0, 4.0, 4.0)));
    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::new_with_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(
        Sphere::new(
            Point3::new(0.0, 7.0, 0.0),
            2.0,
            Arc::clone(&difflight)
        )
    ));
    world.add(Arc::new(
        Quad::new(
            Point3::new(3.0, 1.0, -2.0),
            vec3::Vec3::new(2.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 2.0, 0.0),
            difflight
        )
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
    cam.background = Color::default();

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::default();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new_with_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(
        Quad::new(
            Point3::new(555.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 555.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 555.0),
            green
        )
    ));
    world.add(Arc::new(
        Quad::new(
            Point3::new(0.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 555.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 555.0),
            red
        )
    ));
    world.add(Arc::new(
        Quad::new(
            Point3::new(343.0, 554.0, 332.0),
            vec3::Vec3::new(-130.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, -105.0),
            light
        )
    ));
    world.add(Arc::new(
        Quad::new(
            Point3::new(0.0, 0.0, 0.0),
            vec3::Vec3::new(555.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 555.0),
            Arc::clone(&white)
        )
    ));
    world.add(Arc::new(
        Quad::new(
            Point3::new(555.0, 555.0, 555.0),
            vec3::Vec3::new(-555.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, -555.0),
            Arc::clone(&white)
        )
    ));
    world.add(Arc::new(
        Quad::new(
            Point3::new(0.0, 0.0, 555.0),
            vec3::Vec3::new(555.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 555.0, 0.0),
            Arc::clone(&white)
        )
    ));
    // let c_white = Arc::clone(&white);
    let box1 = make_box(Point3::new(0.0,0.0,0.0),
     Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1,Vec3::new(265.0, 0.0, 295.0)));   
    world.add(box1);

    let box2 = make_box(Point3::new(0.0,0.0,0.0),
     Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2,Vec3::new(130.0, 0.0, 65.0)));   
    world.add(box2);
    // world.add(make_box(
    //     Point3::new(130.0, 0.0, 65.0),
    //     Point3::new(295.0, 165.0, 230.0),
    //     Arc::clone(&white)
    // ));
    // world.add(make_box(
    //     Point3::new(265.0, 0.0, 295.0),
    //     Point3::new(430.0, 330.0, 460.0),
    //     Arc::clone(&white)
    // ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
    cam.background = Color::default();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}


fn main() {
    match 7 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => (),
    }
}