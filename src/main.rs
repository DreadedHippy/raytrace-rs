use core::f64;
use std::sync::Arc;
use crate::{camera::Camera, color::Color, cube::Cube, hittable_list::HittableList, material::{Dielectric, Lambertian, Metal}, rand::{random_f64, random_f64_range}, sphere::Sphere, vec3::{Point3, Vec3}};


pub mod vec3;
pub mod color;
pub mod ray;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod sphere;
pub mod cube;
pub mod interval;
pub mod rand;
pub mod material;
fn main() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_xyz(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(&Point3::from_xyz(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));


    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_mat = random_f64();
    //         let center = Point3::from_xyz(a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9 * random_f64());


    //         if (center - Point3::from_xyz(4.0, 0.2, 0.0)).length() > 0.9 {

    //             if choose_mat < 0.8 {
    //                 // diffuse
    //                 let albedo = Color::random() * Color::random();
    //                 let sphere_material = Arc::new(Lambertian::new(albedo));
    //                 world.add(Arc::new(Sphere::new(&center, 0.2, sphere_material)));
    //             } else if choose_mat < 0.95 {
    //                 // metal
    //                 let albedo = Color::random_range(0.5, 1.0);
    //                 let fuzz = random_f64_range(0.0, 0.5);
    //                 let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
    //                 world.add(Arc::new(Sphere::new(&center, 0.2, sphere_material)));
    //             } else {
    //                 // glass
    //                 let sphere_material = Arc::new(Dielectric::new(1.5));
    //                 world.add(Arc::new(Sphere::new(&center, 0.2, sphere_material)));
    //             }
    //         }
    //     }
    // }


    // let material1 = Arc::new(Dielectric::new(1.5));
    // world.add(Arc::new(Sphere::new(&Point3::from_xyz(0.0, 1.0, 0.0), 1.0, material1)));
    
    // let material1 = Arc::new(Dielectric::new(1.5));
    // let material2 = Arc::new(Lambertian::new(Color::from_xyz(0.1, 0.1, 0.1)));
    let material3 = Arc::new(Metal::new(&Color::from_xyz(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Cube::new(&Point3::from_xyz(-4.0, 1.0, 0.0), 1.0, material3)));
    
    // let material3 = Arc::new(Metal::new(&Color::from_xyz(0.7, 0.6, 0.5), 0.0));
    // world.add(Arc::new(Sphere::new(&Point3::from_xyz(4.0, 1.0, 0.0), 1.0,material3)));


    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1200;
    cam.samples_per_pixel = 50; // reduce for faster render times
    cam.max_depth         = 50;

    cam.vfov     = 20;
    cam.lookfrom = Point3::from_xyz(13.0,2.0,3.0);
    cam.lookat   = Point3::from_xyz(0.0,0.0,0.0);
    cam.vup      = Vec3::from_xyz(0.0,1.0,0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.render(&world);
}
