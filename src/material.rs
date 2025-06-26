use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::{random_unit_vector, reflect}};


pub trait Material {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		false
	}
}

pub struct Mat;

impl Material for Mat {
}

pub struct Lambertian {
	albedo: Color
}

impl Lambertian {
	pub fn new(albedo: &Color) -> Self{
		Self {albedo: *albedo}
	}
}

impl Material for Lambertian {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let mut scatter_direction = rec.normal + random_unit_vector();

		// Catch degenerate scatter direction i.e, random vector is opposite direction of normal
		if scatter_direction.near_zero() {scatter_direction = rec.normal}
		*scattered = Ray::from_values(&rec.p, &scatter_direction);
		*attenuation = self.albedo;
		return true;
	}
}

pub struct Metal {
	albedo: Color
}

impl Metal {
	pub fn new(albedo: &Color) -> Self {
		Self { albedo: *albedo }
	}
}


impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let reflected = reflect(&r_in.direction(), &rec.normal);
		*scattered = Ray::from_values(&rec.p, &reflected);
		*attenuation = self.albedo;
		return true;
	}
}