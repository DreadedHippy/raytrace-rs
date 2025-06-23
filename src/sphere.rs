use crate::{hittable::{HitRecord, Hittable}, ray::Ray, vec3::{Point3, Vec3}};

pub struct Sphere {
  center: Point3,
  radius: f64
}

impl Sphere {
  	pub fn new(center: &Point3, radius: f64) -> Self {
    	Self {center: center.clone(), radius: f64::max(0.0, radius)}
  	}
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
		let center = self.center;
		let radius = self.radius;
      
		// get (C - Q) and store
		// Assume b in the quadratic eqn = -2h, you'd see how we got a c, h
		let oc = center - r.origin();
		let a = r.direction().length_squared();
		let h = Vec3::dot(&r.direction(), &oc);
		let c = oc.length_squared() - (radius * radius);
		let discriminant = h*h - a*c;

		// eprintln!("{:?}", discriminant);

		// return discriminant >= 0.0;

      	if discriminant < 0.0 {
        	return false
		}
		
		let sqrtd = discriminant.sqrt();

		// Find the nearest root that lies in the acceptable range
		let mut root = (h-sqrtd)/a;
		if (root <= ray_tmin || ray_tmax <= root) {
			root = (h + sqrtd)/a;

			if (root <= ray_tmin || ray_tmax <= root) {
				return false
			}
		}

		rec.t = root;
		rec.p = r.at(rec.t);
		let outward_normal = (rec.p - center)/radius;
		rec.set_face_normal(&r, &outward_normal);

		return true;

    }
}