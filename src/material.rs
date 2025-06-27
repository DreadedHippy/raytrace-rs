use std::{sync::Arc};

use crate::{color::Color, hittable::HitRecord, rand::random_f64, ray::Ray, vec3::{random_unit_vector, reflect, refract, Vec3}};


pub trait Material: Send + Sync {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
pub struct Lambertian {
	albedo: Color,
}

pub fn blank_material() -> Arc<dyn Material> {
	Arc::new(Dielectric::new(1.0))
}

impl Lambertian {
	pub fn new(albedo: Color) -> Self{
		Self {albedo}
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
	albedo: Color,
	fuzz: f64
}

impl Metal {
	pub fn new(albedo: &Color, fuzz: f64) -> Self {
		Self { albedo: *albedo, fuzz }
	}
}


impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let reflected = reflect(&r_in.direction(), &rec.normal);
		let reflected = Vec3::unit_vector(&reflected) + (self.fuzz * random_unit_vector());
		*scattered = Ray::from_values(&rec.p, &reflected);
		*attenuation = self.albedo;
		return Vec3::dot(&scattered.direction(), &rec.normal) > 0.0;
	}
}

pub struct Dielectric {
	// Refractive index in vacuum or air, or the ratio of the material's refractive index over
	// the refractive index of the enclosing media
	refraction_index: f64
}

impl Dielectric {
	pub fn new(refraction_index: f64) -> Self {
		Self { refraction_index }
	}

	fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
		let mut r0 = (1.0 - refraction_index)/(1.0+refraction_index);
		r0 = r0*r0;
		return r0 + (1.0-r0)*(1.0-cosine).powi(5)
	}
}

impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		*attenuation = Color::from_xyz(1.0, 1.0, 1.0);
		let ri = if rec.front_face {1.0/self.refraction_index} else {self.refraction_index};

		let unit_direction =  Vec3::unit_vector(&r_in.direction());
		let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
		let sin_theta = 1.0 - (cos_theta * cos_theta);

		let cannot_refract = ri*sin_theta > 1.0;

		let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_f64(){
			reflect(&unit_direction, &rec.normal)
		} else {
			refract(&unit_direction, &rec.normal, ri)
		};

		*scattered = Ray::from_values(&rec.p, &direction);
		return true;
	}	
}