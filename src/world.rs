use crate::rng;
use crate::plants;
lazy_static::lazy_static! {
	pub static ref VGA: String = String::from("ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■ ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂");
}

#[derive(Clone, Copy, Debug)]
pub struct Glyph {
	pub ch: char,
	pub color: termion::color::Rgb,
	pub permissions: usize
}

#[derive(Clone, Debug)]
pub struct World {
    pub changes: Vec<(usize, usize)>,
    pub map: Vec<Vec<Glyph>>,
	pub plants: Vec<plants::Plant>,
	pub width: usize,
	pub height: usize, 
	pub dot: Glyph,
}

impl World {
	pub fn plant_plant(&mut self, x: u16, y: u16, height: u16) {
		let plant = plants::Plant {
			x: x,
			y: y,
			height: height,
			family: 1,
			state: 0,
		};
		self.plants.push(plant);
	}
	pub fn grow_plants(&mut self) {
		for p in 0..self.plants.len() {
			//self.plants[p].grow_plant(self);
			//self.plants[p].grow_plant(self);
			//println!("Grow plants");
			plants::grow_plant2(self, p);
		}
	}
	pub fn update_map(&mut self) {
		self.changes = vec![(0,0)];
		let y = self.height;
		let x = self.width;
		//plants::test_animation(self);
		//plants::grow_plants(self);
		self.grow_plants();
		plants::animate_world(self);
		//self.map[y / 2][x / 2].ch = 'O';
		//self.map[y / 2][x / 2].color = termion::color::Rgb(255, 38, 106);
	}
}

pub fn init_map(x: usize, y: usize) -> World {
	let mut dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15), permissions: 0};
	let mut map = vec![vec![dot; x as usize]; y as usize];
	let changes = vec![(0, 0)];
	let mut plants = vec![];
	let width = x;
	let height = y;
	World {
		changes,
		map,
		plants,
		width,
		height,
		dot
	}
}
/*
pub fn draw_world(x: u16, y: u16, count: u16) -> (u16, u16, char, bool) {
	let xf = rng::rng(20) as u16;
	let yf = rng::rng(20) as u16;
	if count > 9 {
		return(1, 1, 'Ö', true);
	}
	return(xf, yf, 'ä', false)
}
*/