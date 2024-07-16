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
mod constant_medium;
use constant_medium::ConstantMedium;
use qard::Quad;
use crate::qard::make_box;
use texture::{SolidColor,Texture,CheckerTexture,ImageTexture,NoiseTexture};
use bvh::BvhNode;
use camera::Camera;
use util::{random_double, random_double_range};
use crate::material::{Lambertian,Metal,Material,DiffuseLight,Dielectric};
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

fn cornell_smoke() {
    let mut world = HittableList::default();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0)));

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
            Point3::new(113.0, 554.0, 127.0),
            vec3::Vec3::new(330.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 305.0),
            light
        )
    ));
    world.add(Arc::new(
        Quad::new(
            Point3::new(0.0, 555.0, 0.0),
            vec3::Vec3::new(555.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 555.0),
            Arc::clone(&white)
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
            Point3::new(0.0, 0.0, 555.0),
            vec3::Vec3::new(555.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 555.0),
            Arc::clone(&white)
        )
    ));

    let box1 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white)
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, vec3::Vec3::new(265.0, 0.0, 295.0)));

    let box2 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white)
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, vec3::Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(
        ConstantMedium::new_with_color(box1, 0.01, Color::new(0.0, 0.0, 0.0))
    ));
    world.add(Arc::new(
        ConstantMedium::new_with_color(box2, 0.01, Color::new(1.0, 1.0, 1.0))
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 10;
    cam.background = Color::default();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn final_scene(image_width: u32, samples_per_pixel: usize, max_depth: i32) {
    let mut boxes1 = HittableList::default();
    let ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    (0..boxes_per_side).for_each(|i| {
        (0..boxes_per_side).for_each(|j| {
        let w = 100.0;
        let x0 = -1000.0 + i as f64 * w;
        let z0 = -1000.0 + j as f64 * w;
        let y0 = 0.0;
        let x1 = x0 + w;
        let y1 = util::random_double_range(1.0, 101.0);
        let z1 = z0 + w;

        boxes1.add(
            make_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground)
            )
        );
        });
    });

    let mut world = HittableList::default();

    world.add(Arc::new(BvhNode::new(&mut boxes1)));

    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(
        Quad::new(
            Point3::new(123.0, 554.0, 147.0),
            vec3::Vec3::new(412.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 412.0),
            light
        )
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + vec3::Vec3::new(30.0, 0.0, 0.0);
    let sphere_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(
        Sphere::new_center2(center1, center2, 50.0, sphere_material)
    ));

    world.add(Arc::new(
        Sphere::new(
            Point3::new(260.0, 150.0, 45.0),
            50.0,
            Arc::new(Dielectric::new(1.5))
        )
    ));
    world.add(Arc::new(
        Sphere::new(
            Point3::new(0.0, 150.0, 145.0),
            50.0,
            Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0))
        )
    ));

    let boundary: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));
    world.add(Arc::clone(&boundary));
    world.add(Arc::new(ConstantMedium::new_with_color(
        Arc::clone(&boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9)
    )));
    let boundary: Arc<dyn Hittable> = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
    world.add(Arc::new(ConstantMedium::new_with_color(
        Arc::clone(&boundary),
        0.0001,
        Color::new(1.0, 1.0, 1.0)
    )));

    let emat: Arc<dyn Material> = Arc::new(Lambertian::new_texture(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Arc::new(
        Sphere::new(
            Point3::new(400.0, 200.0, 400.0),
            100.0,
            emat
        )
    ));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    world.add(Arc::new(
        Sphere::new(
            Point3::new(220.0, 280.0, 300.0),
            80.0,
            Arc::new(Lambertian::new_texture(pertext))
        )
    ));

    let mut boxes2 = HittableList::default();
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    (0..ns).for_each(|_| {
        boxes2.add(
            Arc::new(Sphere::new(
                Point3::random_range(0.0, 165.0),
                10.0,
                Arc::clone(&white)
            ))
        );
    });

    world.add(Arc::new(
        Translate::new(
            Arc::new(RotateY::new(
                Arc::new(BvhNode::new(&mut boxes2)),
                15.0
            )),
            vec3::Vec3::new(-100.0, 270.0, 395.0)
        )
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::default();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}
fn main() {
    match 9 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(400, 200, 10),
        _ => (),
    }
}
