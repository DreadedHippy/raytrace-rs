use std::cell::{Cell, OnceCell};


const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;


thread_local! {
	static RANDOM: Cell<Rand> = const { Cell::new(Rand {x: KX, y: KY, z: KZ, w: KW}) };
}


fn rand() -> u32 {
	// get
	let mut s = RANDOM.get();

	// mutate
	let result = s.rand();

	// set
	RANDOM.set(s);

	return result;
}

pub fn shuffle<T>(a: &mut [T]) {
	let mut s = RANDOM.get();

	let result = s.shuffle(a);

	RANDOM.set(s);

	return result;
}



pub fn rand_range(a: i32, b: i32) -> i32 {
	// get
	let mut s = RANDOM.get();

	// mutate
	let result = s.rand_range(a, b);

	// set
	RANDOM.set(s);

	return result;
}

fn rand_float() -> f64 {
	let mut s = RANDOM.get();

	let result = s.rand_float();

	RANDOM.set(s);

	return result;
}

pub fn random_f64() -> f64 {
	let mut s = RANDOM.get();

	let result = s.random_f64();

	RANDOM.set(s);

	return result;
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
	let mut s = RANDOM.get();

	let result = s.random_f64_range(min, max);

	RANDOM.set(s);

	return result
}

// Random
#[derive(Clone, Copy)]
struct Rand {
  x: u32, y: u32, z: u32, w: u32
}

impl Default for Rand {
	fn default() -> Self {
		Self::new(0)
	}
}
 
impl Rand{
	pub fn new(seed: u32) -> Rand {
		Rand{
			x: KX^seed, y: KY^seed,
			z: KZ, w: KW
		}
	}

	// Xorshift 128, taken from German Wikipedia
	pub fn rand(&mut self) -> u32 {
		let t = self.x^self.x.wrapping_shl(11);
		self.x = self.y; self.y = self.z; self.z = self.w;
		self.w ^= self.w.wrapping_shr(19)^t^t.wrapping_shr(8);
		return self.w;
	}

	pub fn shuffle<T>(&mut self, a: &mut [T]) {
		if a.len()==0 {return;}
		let mut i = a.len()-1;
		while i>0 {
			let j = (self.rand() as usize)%(i+1);
			a.swap(i,j);
			i-=1;
		}
	}

	pub fn rand_range(&mut self, a: i32, b: i32) -> i32 {
		let m = (b-a+1) as u32;
		return a+(self.rand()%m) as i32;
	}

	pub fn rand_float(&mut self) -> f64 {
		(self.rand() as f64)/(<u32>::max_value() as f64)
	}

	pub fn random_f64(&mut self) -> f64 {
		let end = 1000000;
		let val = self.rand_range(0, end) as f64/ (end + 1) as f64;

		val
	}

	pub fn random_f64_range(&mut self, min: f64, max: f64) -> f64 {
		return min + (max - min) * self.random_f64();
	}


}
 
 