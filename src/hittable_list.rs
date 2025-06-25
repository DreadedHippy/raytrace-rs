use std::rc::Rc;

use crate::{hittable::{HitRecord, Hittable}, interval::Interval};

pub struct HittableList
{
	pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
	pub fn new() -> Self {Self { objects: Vec::new() }}

	pub fn from_hittable(object: Rc<dyn Hittable>) -> Self{
		Self {objects: vec![object]}
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}

	pub fn add(&mut self, object: Rc<dyn Hittable>) {
		self.objects.push(object);
	}
}

impl Hittable for HittableList {
	fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval, rec: &mut crate::hittable::HitRecord) -> bool {
		let mut temp_rec = HitRecord::new();
		let mut hit_anything = false;
		let mut closest_so_far = ray_t.max;

		for object in &self.objects {
			if object.hit(r, &Interval::from_values(ray_t.min, closest_so_far), &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.t;
				*rec = temp_rec;
			}
		}

		hit_anything
	}
}
