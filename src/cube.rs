use crate::{hittable::Hittable, interval::Interval, vec3::{Point3, Vec3}};

pub struct Cube {
	pub center: Point3,
	pub side_length: f64
}

// Parallel ray epsilon
const E: f64 = 0.000001;

impl Cube {
	pub fn new(center: &Point3, side_length: f64) -> Self {
		Self { center: center.clone(), side_length: f64::max(0.0, side_length) }
	}
}

impl Hittable for Cube {
	fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval, rec: &mut crate::hittable::HitRecord) -> bool {
		// A cube in 3d geometric space at origin (0, 0, 0) with side length 2a
    // i.e, sides extend from -a to +a is defined by the inequalities
    // -a <= x <= a, -a <= y <= a, -a <= z <= a
    // starting from origin h, k, l gives us
    // h-a <= x <= h+a, k-a <= y <= k+a, l-a <= z <= l+a
    // let p be a point with coordinates (x, y, z) and C be the cube center (h, k, l),
    // and A be (a, a, a)  then:
    // C-A = (h-a, k-a, l-a),
    // C+A = (h+a, k+a, l+a)
    // by this, the point p must be between C-A and C+A
    // i.e C-A <= p <= C+A at any coordinate
    // because p is a function of t
    // C-A <= Q + t*d <= C+A;
    // taking each inequality:
    // left inequality
    // C-A <= Q + t*d;
    // C-A - Q <= t*d;
    // right inequality
    // Q + t*d <= C+A;
    // t*d <= C+A - Q;
    // hence:
    // (C-A-Q) <= t*d <= (C+A-Q);
    // let's code it up

		
    let c = self.center; // C
    let a = self.side_length/2.0;
    let a = Vec3::from_xyz(a, a, a); // A
    let q = r.origin(); // Q
    let v1 = c-a;
    let v2 = c+a;
    let d = Vec3::unit_vector(&r.direction());

		
    // if v1 <= t*d <= v2 for any given t
    // let set our t between 0, and infinity i.e f64::MAX
    let mut t_min = f64::MIN;
    let mut t_max = f64::MAX;

		// Now we iterate through all axes, maximizing t_max and minimizing t_min, which are the points of intersection between ray and cube.

		let mut hit_axis = 3; // X = 0, Y = 1, Z = 2;
		let mut hit_sign = 0;  // 1 for max_bound hit, -1 for min_bound hit

    for (i, (((v1, v2), q), d)) in v1.iter().zip(v2.iter()).zip(q.iter()).zip(d.iter()).enumerate() {
			let mut t1 = (v1 - q)/d;
			let mut t2 = (v2 - q)/d;
			
			if d.abs() < E {
				if q < v1 || q > v2 {
					return false
				}
			} else {
				let mut swapped = false;
				// check if max, min swapped
				if t1 > t2 {(t1, t2) = (t2, t1); swapped = true}
				
				// maximize t_min
				t_min = t_min.max(t1);
				// check if the face hit (i.e, the value t_min) came from the current axis
				if t1 == t_min {hit_axis = i}
				// adjust the hit sign to correspond with the hit point
				hit_sign = if swapped {1} else {-1};
				// minimize t_max;
				t_max = t_max.min(t2);
				
				if t_min > t_max {return false}
			}
			
			// eprintln!("{:?}", (t_min, t_max))
    }

		if !ray_t.surrounds(t_min) {
			t_min = t_max;

			if !ray_t.surrounds(t_min) {
				return false
			}
		}

		// if t_min < 0.0 {
		// 	// ray starts inside the cube, or at least behind the camera
		// 	if t_max < 0.0 {
		// 		// cube is entirely behind the camera
		// 		return false
		// 	}
		// }

		// t_min is lower than tmax and is the first point of intersection of the ray
		rec.t = t_min;
		rec.p = r.at(rec.t);
		let mut outward_normal = Vec3::new();
		if hit_axis != 3 {
			outward_normal =  match hit_axis {
			0 => Vec3::from_xyz(hit_sign as f64, 0.0, 0.0),
			1 => Vec3::from_xyz(0.0 , hit_sign as f64, 0.0),
			_ => Vec3::from_xyz(0.0 , 0.0, hit_sign as f64),
			};
		}
		rec.set_face_normal(r, &outward_normal);
		
    return true;


	}

	// My attempt at checking for ray collision with cube
	// fn hit(center: &Point3, side_length: f64, ray: &Ray) -> bool {
	// 	// A cube in 3d geometric space at origin (0, 0, 0) with side length 2a
  //   // i.e, sides extend from -a to +a is defined by the inequalities
  //   // -a <= x <= a, -a <= y <= a, -a <= z <= a
  //   // starting from origin h, k, l gives us
  //   // h-a <= x <= h+a, k-a <= y <= k+a, l-a <= z <= l+a
  //   // let p be a point with coordinates (x, y, z) and C be the cube center (h, k, l),
  //   // and A be (a, a, a)  then:
  //   // C-A = (h-a, k-a, l-a),
  //   // C+A = (h+a, k+a, l+a)
  //   // by this, the point p must be between C-A and C+A
  //   // i.e C-A <= p <= C+A at any coordinate
  //   // because p is a function of t
  //   // C-A <= Q + t*d <= C+A;
  //   // taking each inequality:
  //   // left inequality
  //   // C-A <= Q + t*d;
  //   // C-A - Q <= t*d;
  //   // right inequality
  //   // Q + t*d <= C+A;
  //   // t*d <= C+A - Q;
  //   // hence:
  //   // (C-A-Q) <= t*d <= (C+A-Q);
  //   // let's code it up
		
    // let c = *center; // C
    // let a = side_length/2.0;
    // let a = Vec3::from_xyz(a, a, a); // A
    // let q = ray.origin(); // Q
    // let v1 = c-a;
    // let v2 = c+a;
    // let d = Vec3::unit_vector(&ray.direction());
  //   // if v1 <= t*d <= v2 for any given t
  //   // let set our t between 0, and infinity i.e f64::MAX
		
  //   // check for x
		
  //   let mut t_min = 0.0_f64;
  //   let mut t_max = f64::MAX;
		
  //   for (((v1, v2), q), d) in v1.iter().zip(v2.iter()).zip(q.iter()).zip(d.iter()) {
	// 		let mut t1 = (v1 - q)/d;
	// 		let mut t2 = (v2 - q)/d;
			
	// 		if d.abs() < E {
	// 			if q < v1 || q > v2 {
	// 				return false
	// 			}
	// 		} else {
				
	// 			if t1 > t2 {(t1, t2) = (t2, t1)}
				
	// 			t_min = t_min.max(t1);
	// 			t_max = t_max.min(t2);
				
	// 			if t_min > t_max {return false}
	// 		}
			
	// 		// eprintln!("{:?}", (t_min, t_max))
  //   }
		
  //   return true;
	// }
}
	