use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::from_values(center, &Vec3::new(), &0.0),
            radius: f64::max(0.0, radius),
            mat,
        }
    }

	
    pub fn new_moving(center1: &Point3, center2: &Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::from_values(center1, &(*center2 - *center1), &0.0),
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
		let current_center = self.center.at(r.time());
        let radius = self.radius;

        // get (C - Q) and store
        // Assume b in the quadratic eqn = -2h, you'd see how we got a c, h
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(&r.direction(), &oc);
        let c = oc.length_squared() - (radius * radius);
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;

            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / radius;
        rec.set_face_normal(&r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
