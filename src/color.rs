use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: &Color) {
	let r = color.x();
	let g = color.y();
	let b = color.z();

	let rbyte = (255.999 * r) as i32;
	let gbyte = (255.999 * g) as i32;
	let bbyte = (255.999 * b) as i32;

	println!("{} {} {}", rbyte, gbyte, bbyte);

}