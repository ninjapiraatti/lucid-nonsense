use crate::world;
use crate::rng;
use crate::utils;

#[derive(Clone, Debug)]
pub struct Building {
	pub map: Vec<Vec<world::Glyph>>,
	pub x: usize,
	pub y: usize,
	pub floors: usize,
	pub width: usize,
	pub height: usize,
	pub drawn: bool,
}

impl Building {
	pub fn new(x: usize, y: usize, floors: usize, width: usize, height: usize) -> Building {
		let brick = world::Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15), permissions: 0, z: 0};
		Building {
			map: vec![vec![brick; width]; floors + height],
			x,
			y,
			floors,
			width,
			height,
			drawn: false,
		}
	}

	pub fn update(&mut self, world: &world::World) {
		//world.debugstr = format!("Building drawn: {}", self.drawn);
		if self.drawn == true {
			return;
		}
		for y in 0..self.floors + self.height + 1 {
			for x in 0..self.width + 1 {
				if x < world.width && y < world.height {
					let b = get_glyph(x, y, self.floors, self.width, self.height);
					self.map[y][x] = world::Glyph {
						ch: b,
						color: termion::color::Rgb(255, 38, 106),
						permissions: 1,
						z: 1,
					};
				}
			}
		}
		self.drawn = true;
		//world.debugstr = format!("Building at ({}, {}). Drawn: {}", self.x, self.y, self.drawn);
	}
}

fn get_glyph(x: usize, y: usize, floors: usize, width: usize, height: usize) -> char {
	if y == 0 {
		if x == 0 {
			return crate::CHARS_TOPLEFT.chars().nth(rng::rng(crate::CHARS_TOPLEFT.chars().count())).unwrap();
		}
		if x == width {
			return crate::CHARS_TOPRIGHT.chars().nth(rng::rng(crate::CHARS_TOPRIGHT.chars().count())).unwrap();
		}
		return crate::CHARS_HRZ.chars().nth(rng::rng(crate::CHARS_HRZ.chars().count())).unwrap();
	}
	if y == floors {
		if x == 0 || x == width {
			return crate::CHARS_VRT.chars().nth(rng::rng(crate::CHARS_VRT.chars().count())).unwrap();
		}
		return crate::CHARS_HRZ.chars().nth(rng::rng(crate::CHARS_HRZ.chars().count())).unwrap();
	}
	if y == floors + height {
		if x == 0 {
			return crate::CHARS_BOTTOMLEFT.chars().nth(rng::rng(crate::CHARS_BOTTOMLEFT.chars().count())).unwrap();
		}
		if x == width {
			return crate::CHARS_BOTTOMRIGHT.chars().nth(rng::rng(crate::CHARS_BOTTOMRIGHT.chars().count())).unwrap();
		}
		return crate::CHARS_HRZ.chars().nth(rng::rng(crate::CHARS_HRZ.chars().count())).unwrap();
	}
	if x == 0 || x == width {
		return crate::CHARS_VRT.chars().nth(rng::rng(crate::CHARS_VRT.chars().count())).unwrap();
	}
	return crate::CHARS_FILL.chars().nth(rng::rng(crate::CHARS_FILL.chars().count())).unwrap();
}

pub fn init_buildings() -> Vec<Building> {
	let buildings = vec![];
	buildings
}