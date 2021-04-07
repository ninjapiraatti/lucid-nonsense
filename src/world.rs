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
	let dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15), permissions: 0};
	let map = vec![vec![dot; x as usize]; y as usize];
	let changes = vec![(0, 0)];
	let plants = vec![];
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