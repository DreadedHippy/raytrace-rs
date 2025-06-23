use crate::{ray::Ray, vec3::{Point3, Vec3}};

#[derive(Clone, Copy)]
pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool
}

impl HitRecord {
	pub fn new() -> Self {
		Self {p: Point3::new(), normal: Vec3::new(), t: 0.0, front_face: false}
	}

	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
		self.front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
		self.normal = if self.front_face {*outward_normal} else {-*outward_normal}
	}
}

/// A trait representing anything a ray can hit
pub trait Hittable {
	fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}