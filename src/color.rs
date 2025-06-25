use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(color: &Color) {
	let r = color.x();
	let g = color.y();
	let b = color.z();

	
  // Translate the [0,1] component values to the byte range [0,255].
	let intensity = Interval::from_values(0.000, 0.999);

	let rbyte = (256.0 * intensity.clamp(r)) as i32;
	let gbyte = (256.0 * intensity.clamp(g)) as i32;
	let bbyte = (256.0 * intensity.clamp(b)) as i32;

	println!("{} {} {}", rbyte, gbyte, bbyte);

}