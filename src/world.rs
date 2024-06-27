use crate::plants;
use std::cmp::Reverse;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

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
	pub z_index: i16,
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
	pub z_index: i16,
}

impl Entity {
    pub fn new(glyphmap: &Vec<Vec<Glyph>>, x: u16, y: u16) -> Rc<RefCell<Entity>> {
        Rc::new(RefCell::new(Entity {
            glyphmap: glyphmap.to_vec(),
            width: glyphmap[0].len() as u16,
            height: glyphmap.len() as u16,
            x,
            y,
            wants_update: true,
            draw: true,
            z_index: y as i16,
        }))
    }

    pub fn is_creature(&self) -> bool {
        self.glyphmap[0][0].ch == '#'
    }

    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    pub fn move_entity(&mut self, dx: i16, dy: i16, world: &mut World) {
			
        let new_x = (self.x as i16 + dx).max(0).min(world.width as i16 - 1) as u16;
        let new_y = (self.y as i16 + dy).max(0).min(world.height as i16 - 1) as u16;

        // Clear the old position
        world.map[self.y as usize][self.x as usize] = world.dot;
        world.changes.push((self.x as usize, self.y as usize));

        // Update the position
        self.x = new_x;
        self.y = new_y;

        // Update the world map with the new position
        //world.map[self.y as usize][self.x as usize] = self.glyphmap[0][0];
				self.wants_update = true;
				println!("Entity moved to position: ({}, {})", self.x, self.y);
    }
}

#[derive(Clone, Debug)]
pub struct World {
	pub changes: Vec<(usize, usize)>,
	pub map: Vec<Vec<Glyph>>,
	pub plants: Vec<plants::Plant>,
	pub entities: Vec<Rc<RefCell<Entity>>>,
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
		//self.check_graphics_overlap();
		self.draw_graphics();
		self.grow_plants();
		plants::grow_grass(self);
		//self.debugstr = format!("Changes len: {}", self.changes.len());
	}

	pub fn draw_graphics(&mut self) {
		self.entities.sort_by_key(|e| Reverse(e.borrow().z_index.clone()));
		for e in 0..self.entities.len() {
			let entity = &self.entities[e];
			if entity.borrow().wants_update == true {
				for y in 0..entity.borrow().glyphmap.len() {
					for x in 0..entity.borrow().glyphmap[y].len() {
						if entity.borrow().y as usize + y >= self.height || entity.borrow().x as usize + x >= self.width {
							break;
						}
						if entity.borrow().z_index <= self.map[entity.borrow().y as usize + y][entity.borrow().x as usize + x].z_index {
							break;
						}
						self.map[entity.borrow().y as usize + y][entity.borrow().x as usize + x] = entity.borrow().glyphmap[y][x];
						self.changes.push((entity.borrow().x as usize + x, entity.borrow().y as usize + y));
					}
				}
				self.entities[e].borrow_mut().wants_update = false;
			}
		}
	}

	pub fn update_entities(&mut self) {
		let mut entities_to_move: Vec<Rc<RefCell<Entity>>> = vec![];
		for entity in &self.entities {
			if entity.borrow().is_creature() {
				entities_to_move.push(Rc::clone(entity));
			}
		}
		for entity in entities_to_move {
			entity.borrow_mut().move_entity(10, 0, self); // Example: move entity to the right
		}
	}

	pub fn update_debugstr(&mut self, str: String) {
		self.debugstr = str;
	}
}

pub fn init_world(x: usize, y: usize) -> World {
	let dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15), permissions: 0, z_index: 0};
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
