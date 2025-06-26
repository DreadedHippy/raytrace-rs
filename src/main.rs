use core::f64;
use std::rc::Rc;
use crate::{camera::Camera, color::{write_color, Color}, cube::Cube, hittable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, material::{Lambertian, Metal}, ray::Ray, sphere::Sphere, vec3::{Point3, Vec3}};


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
    let mut material_ground = Rc::new(Lambertian::new(&Color::from_xyz(0.8, 0.8, 0.0)));
    let mut material_center = Rc::new(Lambertian::new(&Color::from_xyz(0.1, 0.2, 0.5)));
    let mut material_left = Rc::new(Metal::new(&Color::from_xyz(0.8, 0.8, 0.8)));
    let mut material_right = Rc::new(Metal::new(&Color::from_xyz(0.8, 0.6, 0.2)));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(&Point3::from_xyz(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(&Point3::from_xyz(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(&Point3::from_xyz(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(&Point3::from_xyz(1.0, 0.0, -1.0), 0.5, material_right)));
    // world.add(Rc::new(Cube::new(&Point3::from_xyz(-0.8, 0.5, -1.0), 0.5)));

    let mut cam = Camera::new();
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}
