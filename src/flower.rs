//use termion::{color};
use crate::rng;

#[derive(Clone)]
pub struct Glyph {
	pub ch: char,
	pub color: termion::color::Rgb
}

pub struct Flower {
    pub n: usize,
    pub map: Vec<Vec<Glyph>>,
}

pub fn init_map(x: usize, y: usize) -> Flower {
	let g = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15)};
	let mut map = vec![vec![g; x as usize]; y as usize];
	map[y / 2][x / 2].ch = 'O';
	map[y / 2][x / 2].color = termion::color::Rgb(255, 5, 5);
	let n = 5;
	Flower {
		n,
		map
	}
}
pub fn draw_flower(x: u16, y: u16, count: u16) -> (u16, u16, char, bool) {
	let xf = rng::rng(20) as u16;
	let yf = rng::rng(20) as u16;
	if count > 9 {
		return(1, 1, 'Ö', true);
	}
	return(xf, yf, 'ä', false)
}