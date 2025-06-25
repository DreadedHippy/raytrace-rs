use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(color: &Color) {
	let r = color.x();
	let g = color.y();
	let b = color.z();

	// Apply a linear transform for gamma 2
	let r = linear_to_gamma(r);
	let g = linear_to_gamma(g);
	let b = linear_to_gamma(b);
	
  // Translate the [0,1] component values to the byte range [0,255].
	let intensity = Interval::from_values(0.000, 0.999);

	let rbyte = (256.0 * intensity.clamp(r)) as i32;
	let gbyte = (256.0 * intensity.clamp(g)) as i32;
	let bbyte = (256.0 * intensity.clamp(b)) as i32;

	println!("{} {} {}", rbyte, gbyte, bbyte);
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
	if linear_component > 0.0 {
		return linear_component.sqrt()
	}

	return 0.0
}