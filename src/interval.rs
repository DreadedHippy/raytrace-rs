use core::f64;

pub struct Interval {
	pub min: f64,
	pub max: f64
}

impl Interval {
	pub const EMPTY: Self = Self {min: f64::INFINITY, max: f64::NEG_INFINITY};
	pub const UNIVERSE: Self = Self {min: f64::NEG_INFINITY, max: f64::INFINITY};

	pub fn new() -> Self {
		Self {min: f64::INFINITY, max: f64::NEG_INFINITY}
	}

	pub fn from_values(min: f64, max: f64) -> Self {
		Self { min, max }
	}

	pub fn size(&self) -> f64 {
		return self.max - self.min;
	}

	pub fn contains(&self, x: f64) -> bool {
		self.min <= x && x <= self.max
	}

	pub fn surrounds(&self, x: f64) -> bool {
		self.min < x && x < self.max
	}

	pub fn clamp(&self, x: f64) -> f64 {
		if x < self.min {return self.min}
		if x > self.max {return self.max}
		return x
	}
}