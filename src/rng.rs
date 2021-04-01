use std::sync::Mutex;
lazy_static::lazy_static! {
	static ref RNG :Mutex<RandGen> = Mutex::new(RandGen::new(82374));
}

pub fn rng(max: usize) -> usize {
	RNG.lock().unwrap().next_v(max)
}

pub struct RandGen {
	curr: usize,
	mul: usize,
	inc: usize,
	modulo: usize 
}

impl RandGen {
	pub fn new(curr: usize) -> Self {
		RandGen {
			curr,
			mul: 29837422,
			inc: 897290011,
			modulo: 292384298374
		}
	}

	pub fn next_v(&mut self, max:usize) -> usize {
		self.curr = (self.curr * self.mul + self.inc) % self.modulo;
		self.curr % max 
	}
}