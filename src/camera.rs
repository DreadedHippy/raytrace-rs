use std::{f64::consts::PI, io::{self, Write}, sync::{atomic::AtomicI32, Arc, Mutex}, thread};
use rayon::prelude::*;

use crate::{color::{write_color, Color}, hittable::{HitRecord, Hittable}, interval::Interval, rand::random_f64, ray::Ray, vec3::{random_in_unit_disk, Point3, Vec3}};

#[derive(Default)]
pub struct Camera {
	pub aspect_ratio: f64,
	pub image_width: i32, // rendered image width in pixel count
	pub samples_per_pixel: i32, // count of random samples for each pixel
	pub max_depth: i32,

	pub vfov: i32,
	pub lookfrom: Point3,
	pub lookat: Point3,
	pub vup: Vec3,

	pub defocus_angle: f64,
	pub focus_dist: f64,

	image_height: i32, // Rendered image height
	center: Point3, // Camera center
	pixel00_loc: Point3, // Location of pixel 0, 0
	pixel_delta_u: Vec3, // top-bottom pixel-pixel distance
	pixel_delta_v: Vec3, // left-right pixel-pixel distance
	pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
	u: Vec3, v: Vec3, w: Vec3, // Camera frame basis vectors
	defocus_disk_u: Vec3,
	defocus_disk_v: Vec3,
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
		let count = AtomicI32::new(0);
		let all = vec![vec![String::new(); image_width as usize]; image_height as usize];
		let arc = Arc::new(Mutex::new(all));
		let arc_clone = Arc::clone(&arc);

    (0..image_height).into_par_iter().for_each(|j| {
			let v = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

			eprintln!("\rScanlines remaining: {}", (image_height - (v + 1)));
			for i in 0..image_width {
				let mut pixel_color = Color::new();

				for _sample in 0..self.samples_per_pixel {
					let r = self.get_ray(i, j);
					pixel_color += Self::ray_color(&r, self.max_depth, world);
				}

				arc_clone.lock().unwrap()[j as usize][i as usize] = write_color(&(self.pixel_samples_scale * pixel_color));
			}
    });

		let e = arc_clone.lock().unwrap();

		let stdout = io::stdout();
		let lock = stdout.lock();
		let mut w = io::BufWriter::new(lock);

		for i in 0..image_height {
			for j in 0..image_width {
				writeln!(&mut w, "{}", e[i as usize][j as usize]).unwrap();
			}
		}

		io::stdout().flush();

		eprint!("\rDone         ");
	}

	fn initialize(&mut self) {
		self.image_height = ((self.image_width as f64/self.aspect_ratio) as i32).max(1);

		self.pixel_samples_scale = 1.0/self.samples_per_pixel as f64;

		self.center = self.lookfrom;
		// Determine viewport dimensions
		// let focal_length = (self.lookfrom - self.lookat).length();
		let theta = degrees_to_radians(self.vfov);
		let h = f64::tan(theta/2.0);
		let viewport_height = 2.0 * h * self.focus_dist;
		let viewport_width = viewport_height * (self.image_width as f64/self.image_height as f64);

		// Calculate the u,v,w unit basis vectors for the camera coordinate frame.
		self.w = Vec3::unit_vector(&(self.lookfrom - self.lookat));
		self.u = Vec3::unit_vector(&Vec3::cross(&self.vup, &self.w));
		self.v = Vec3::cross(&self.w, &self.u);
    
    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = viewport_width * self.u;
    let viewport_v = viewport_height * -self.v;

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    // Remember: Our pixel grid will be inset from the viewport edges by half the pixel-to-pixel distance
    self.pixel_delta_u = viewport_u/self.image_width as f64;
    self.pixel_delta_v = viewport_v/self.image_height as f64;

		let viewport_upper_left =
        self.center // start from camera center
        - (self.focus_dist * self.w) // Remember viewport-to-camera distance is focal_length in the negative-z direction, at this point, we are at viewport center
        - viewport_u/2.0 // The viewport center is halfway between left and right side so with this we go to the left
        - viewport_v/2.0; // The viewport center is also half-way between top and bottom so with this we go to top-left

    
    // Now to calculate position of upper-left pixel...
    // We know that the pixel grid is inset from the viewport by half the pixel-delta both top and left...
    // which gives us
    self.pixel00_loc = viewport_upper_left + (0.5 * (self.pixel_delta_u + self.pixel_delta_v));

		let defocus_radius = self.focus_dist * f64::tan(degrees_to_radians((self.defocus_angle/2.0) as i32));
		self.defocus_disk_u = self.u * defocus_radius;
		self.defocus_disk_v = self.v * defocus_radius;

	}

	fn get_ray(&self, i: i32, j: i32) -> Ray {
		// Construct a camera ray originating from the defocus disk and directed at a randomly
		// sampled point around the pixel location i, j.

		let offset = self.sample_square();
		let pixel_sample = self.pixel00_loc
			+ ((i as f64 + offset.x()) * self.pixel_delta_u)
			+ ((j as f64 + offset.y()) * self.pixel_delta_v);

		let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
		let ray_direction = pixel_sample - ray_origin;
		return Ray::from_values(&ray_origin, &ray_direction)

	}

	fn sample_square(&self) -> Vec3 {
		// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
		return Vec3::from_xyz(random_f64() - 0.5,random_f64() - 0.5, 0.0);
	}

	fn defocus_disk_sample(&self) -> Point3 {
		// Returns a random point in the defocus_disk
		let p = random_in_unit_disk();
		return self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v);
	}

	fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
		if depth <= 0 {
			return Color::new()
		}

		let mut rec = HitRecord::new();

		if world.hit(r, &Interval::from_values(0.001, f64::INFINITY), &mut rec) {
			let mut scattered = Ray::new();
			let mut attenuation = Color::new();

			if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
				return attenuation * Self::ray_color(&scattered, depth-1, world)
			}
			return Color::new();
		}

		// Scale ray direction to unit vector;
		let unit_direction = Vec3::unit_vector(&r.direction()); // now -1.0 <= y <= 1.0
		let a = 0.5 * (unit_direction.y() + 1.0); // a is the equivalent of y in the interval 0.0, 1.0, i,e 0.0 <= a <= 1.0
		((1.0-a)*Color::from_xyz(1.0, 1.0, 1.0)) + (a*Color::from_xyz(0.5, 0.7, 1.0))
	}
}

fn degrees_to_radians(deg: i32) -> f64 {
	deg as f64 * PI/180.0
}