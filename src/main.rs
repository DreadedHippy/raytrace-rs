use core::f64;
use std::rc::Rc;
use crate::{camera::Camera, color::{write_color, Color}, cube::Cube, hittable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, ray::Ray, sphere::Sphere, vec3::{Point3, Vec3}};


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
fn main() {
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(&Point3::from_xyz(0.0, 0.0, -1.0), 0.5)));
    // world.add(Rc::new(Cube::new(&Point3::from_xyz(-0.8, 0.5, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(&Point3::from_xyz(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new();
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}
