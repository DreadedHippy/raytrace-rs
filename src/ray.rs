use crate::vec3::{Point3, Vec3};

pub struct Ray {
	orig: Point3,
	dir: Vec3
}

impl Ray {
	pub fn new() -> Self {
		Self { orig: Point3::new(), dir: Vec3::new() }
	}
	pub fn from_values(origin: &Point3, direction: &Vec3) -> Self {
		Self {
			orig: *origin,
			dir: *direction
		}
	}

	pub fn origin(&self) -> Point3 {
		return self.orig
	}
	
	pub fn direction(&self) -> Vec3 {
		return self.dir
	}

	pub fn at(&self, t: f64) -> Vec3{
		return self.orig + (t*self.dir)
	}
}