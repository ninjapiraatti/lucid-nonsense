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

pub fn init_map(x: usize, y: usize) -> World {
	let mut dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15), permissions: 0};
	let mut map = vec![vec![dot; x as usize]; y as usize];
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

pub fn update_map(world: &mut World) -> &mut World {
	world.changes = vec![(0,0)];
	let y = world.height;
	let x = world.width;
	//plants::test_animation(world);
	plants::grow_plants(world);
	plants::animate_world(world);
	//world.map[y / 2][x / 2].ch = 'O';
	//world.map[y / 2][x / 2].color = termion::color::Rgb(255, 38, 106);
	world
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