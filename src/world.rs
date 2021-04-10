use crate::plants;

// Player
#[derive(Clone, Debug)]
pub struct Player {
	pub x: u16,
	pub y: u16
}

#[derive(Clone, Copy, Debug)]
pub struct Glyph {
	pub ch: char,
	pub color: termion::color::Rgb,
	pub permissions: usize,
	pub z: u16,
}

#[derive(Clone, Debug)]
pub struct World {
    pub changes: Vec<(usize, usize)>,
    pub map: Vec<Vec<Glyph>>,
	pub plants: Vec<plants::Plant>,
	pub width: usize,
	pub height: usize, 
	pub dot: Glyph,
	pub player: Player,
}

impl World {
	pub fn grow_plants(&mut self) {
		for p in 0..self.plants.len() {
			plants::grow_plant(self, p);
		}
	}
	pub fn update_world(&mut self) {
		self.changes = vec![(0,0)];
		self.grow_plants();
		plants::grow_grass(self);
	}
}

pub fn init_world(x: usize, y: usize) -> World {
	let dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15), permissions: 0, z: 0};
	World {
		dot,
		changes: vec![(0, 0)],
		map: vec![vec![dot; x as usize]; y as usize],
		plants: vec![],
		width: x,
		height: y,
		player: Player {
			x: (x / 2) as u16,
			y: (y / 2) as u16
		},
	}
}