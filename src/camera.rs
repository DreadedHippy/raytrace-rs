use crate::{color::{write_color, Color}, hittable::{HitRecord, Hittable}, interval::Interval, rand::random_f64, ray::Ray, vec3::{random_on_hemisphere, random_unit_vector, Point3, Vec3}};

#[derive(Default)]
pub struct Camera {
	pub aspect_ratio: f64,
	pub image_width: i32, // rendered image width in pixel count
	pub samples_per_pixel: i32, // count of random samples for each pixel
	pub max_depth: i32,
	image_height: i32, // Rendered image height
	center: Point3, // Camera center
	pixel00_loc: Point3, // Location of pixel 0, 0
	pixel_delta_u: Vec3, // top-bottom pixel-pixel distance
	pixel_delta_v: Vec3, // left-right pixel-pixel distance
	pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
}

impl Camera {
	pub fn new() -> Self {
		let mut c = Self::default();
		c.aspect_ratio = 1.0;
		c.image_width = 100;
		c.samples_per_pixel = 10;
		c.max_depth = 10;

		c
	}
	
	pub fn render(&mut self, world: &dyn Hittable) {
		self.initialize();
		let Self {
			image_height,
			image_width,
			// pixel00_loc,
			// pixel_delta_u,
			// pixel_delta_v,
			// center,
		..} = *self;

		
    // Render
		
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
			eprintln!("\rScanlines remaining: {}", (image_height - j));
			for i in 0..image_width {
				let mut pixel_color = Color::new();

				for _sample in 0..self.samples_per_pixel {
					let r = self.get_ray(i, j);
					pixel_color += Self::ray_color(&r, self.max_depth, world);
				}
				// let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
				// // let ray_center = Vec3::from_xyz(pixel_center.x(), pixel_center.y(), center.z());
				// let ray_center = center;
				// let ray_direction = pixel_center - ray_center;
				// let r = Ray::from_values(&ray_center, &ray_direction);

				// let pixel_color = Self::ray_color(&r, world);
				write_color(&(self.pixel_samples_scale * pixel_color));
			}
    }

		eprint!("\rDone         ");
	}

	fn initialize(&mut self) {
		self.image_height = ((self.image_width as f64/self.aspect_ratio) as i32).max(1);

		self.pixel_samples_scale = 1.0/self.samples_per_pixel as f64;

		self.center = Point3::from_xyz(0.0, 0.0, 0.0);

		let focal_length = 1.0;
		let viewport_height = 2.0;
		let viewport_width = viewport_height * (self.image_width as f64/self.image_height as f64);
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::from_xyz(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from_xyz(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    // Remember: Our pixel grid will be inset from the viewport edges by half the pixel-to-pixel distance
    self.pixel_delta_u = viewport_u/self.image_width as f64;
    self.pixel_delta_v = viewport_v/self.image_height as f64;

		let viewport_upper_left =
        self.center // start from camera center
        - Vec3::from_xyz(0.0, 0.0, focal_length) // Remember viewport-to-camera distance is focal_length in the negative-z direction, at this point, we are at viewport center
        - viewport_u/2.0 // The viewport center is halfway between left and right side so with this we go to the left
        - viewport_v/2.0; // The viewport center is also half-way between top and bottom so with this we go to top-left

    
    // Now to calculate position of upper-left pixel...
    // We know that the pixel grid is inset from the viewport by half the pixel-delta both top and left...
    // which gives us
    self.pixel00_loc = viewport_upper_left + (0.5 * (self.pixel_delta_u + self.pixel_delta_v));

	}

	fn get_ray(&mut self, i: i32, j: i32) -> Ray {
		// Construct a camera ray originating from the origin and directed at randomly sampled
		// point around the pixel location i, j.

		let offset = self.sample_square();
		let pixel_sample = self.pixel00_loc
			+ ((i as f64 + offset.x()) * self.pixel_delta_u)
			+ ((j as f64 + offset.y()) * self.pixel_delta_v);

		let ray_origin = self.center;
		let ray_direction = pixel_sample - ray_origin;
		return Ray::from_values(&ray_origin, &ray_direction)

	}

	fn sample_square(&mut self) -> Vec3 {
		// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
		return Vec3::from_xyz(random_f64() - 0.5,random_f64() - 0.5, 0.0);
	}

	fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
		if depth <= 0 {
			return Color::new()
		}

		let mut rec = HitRecord::new();

		if (world.hit(r, &Interval::from_values(0.001, f64::INFINITY), &mut rec)) {
			let direction = rec.normal + random_unit_vector();
			return 0.3 * Self::ray_color(&Ray::from_values(&rec.p, &direction), depth - 1, world);
		}

		// Scale ray direction to unit vector;
		let unit_direction = Vec3::unit_vector(&r.direction()); // now -1.0 <= y <= 1.0
		let a = 0.5 * (unit_direction.y() + 1.0); // a is the equivalent of y in the interval 0.0, 1.0, i,e 0.0 <= a <= 1.0
		((1.0-a)*Color::from_xyz(1.0, 1.0, 1.0)) + (a*Color::from_xyz(0.5, 0.7, 1.0))
	}
}