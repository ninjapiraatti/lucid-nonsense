//use termion::{color};
use crate::rng;

#[derive(Clone)]
pub struct Glyph {
	pub ch: char,
	pub color: termion::color::Rgb
}

#[derive(Clone)]
pub struct Flower {
    pub n: usize,
    pub map: Vec<Vec<Glyph>>,
	pub width: usize,
	pub height: usize, 
}

pub fn init_map(x: usize, y: usize) -> Flower {
	let g = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15)};
	let mut map = vec![vec![g; x as usize]; y as usize];
	let n = 5;
	let width = x;
	let height = y;
	Flower {
		n,
		map,
		width,
		height,
	}
}

pub fn update_map(flower: &mut Flower) -> &mut Flower {
	let y = flower.height;
	let x = flower.width;
	flower.map[y / 2][x / 2].ch = 'O';
	flower.map[y / 2][x / 2].color = termion::color::Rgb(255, 38, 106);
	flower
}

pub fn draw_flower(x: u16, y: u16, count: u16) -> (u16, u16, char, bool) {
	let xf = rng::rng(20) as u16;
	let yf = rng::rng(20) as u16;
	if count > 9 {
		return(1, 1, 'Ö', true);
	}
	return(xf, yf, 'ä', false)
}