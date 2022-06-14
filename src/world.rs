use crate::plants;
use crate::buildings;

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
pub struct Entity {
	pub glyphmap: Vec<Vec<Glyph>>,
	pub width: u16,
	pub height: u16,
	pub x: u16,
	pub y: u16,
	pub wants_update: bool,
	pub draw: bool,
}

#[derive(Clone, Debug)]
pub struct World {
	pub changes: Vec<(usize, usize)>,
	pub map: Vec<Vec<Glyph>>,
	pub plants: Vec<plants::Plant>,
	pub entities: Vec<Entity>,
	pub width: usize,
	pub height: usize, 
	pub dot: Glyph,
	pub player: Player,
	pub debugstr: String,
	pub debugint: i32
}

impl World {
	pub fn grow_plants(&mut self) {
		for p in 0..self.plants.len() {
			plants::grow_plant(self, p);
		}
	}

	pub fn update_world(&mut self) {
		self.changes = vec![(0,0)];
		self.draw_graphics();
		self.grow_plants();
		plants::grow_grass(self);
		//self.debugstr = format!("Changes len: {}", self.changes.len());
	}


	pub fn draw_graphics(&mut self) {
		for g in 0..self.entities.len() {
			if self.entities[g].wants_update == true {
				for y in 0..self.entities[g].glyphmap.len() {
					for x in 0..self.entities[g].glyphmap[y].len() {
						self.map[self.entities[g].y as usize + y][self.entities[g].x as usize + x] = self.entities[g].glyphmap[y][x];
						self.changes.push((self.entities[g].x as usize + x, self.entities[g].y as usize + y));
					}
				}
			}
		}
	}

	pub fn update_debugstr(&mut self, str: String) {
		self.debugstr = str;
	}
}

pub fn init_world(x: usize, y: usize) -> World {
	let dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15), permissions: 0, z: 0};
	World {
		dot,
		changes: vec![(0, 0)],
		map: vec![vec![dot; x as usize]; y as usize],
		plants: vec![],
		entities: vec![],
		width: x,
		height: y,
		debugstr: "Henlo".to_string(),
		debugint: 0,
		player: Player {
			x: (x / 2) as u16,
			y: (y / 2) as u16
		},
	}
}

impl Entity {
	pub fn new(glyphmap: &Vec<Vec<Glyph>>, x: u16, y: u16) -> Entity {
		Entity {
			glyphmap: glyphmap.to_vec(),
			width: glyphmap[0].len() as u16,
			height: glyphmap.len() as u16,
			x,
			y,
			wants_update: true,
			draw: true,
		}
	}
}