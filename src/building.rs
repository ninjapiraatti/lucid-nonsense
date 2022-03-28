use crate::world;
use crate::rng;

#[derive(Clone, Debug)]
pub struct Building {
	pub x: u16,
	pub y: u16,
	pub floors: u16,
	pub width: u16,
	pub height: u16,
}

impl Building {
	pub fn new(x: u16, y: u16, floors: u16, width: u16, height: u16) -> Building {
		Building {
			x,
			y,
			floors,
			width,
			height,
		}
	}
}