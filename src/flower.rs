use crate::rng;

#[derive(Clone, Copy, Debug)]
pub struct Glyph {
	pub ch: char,
	pub color: termion::color::Rgb
}

#[derive(Clone, Debug)]
pub struct Flower {
    pub changes: Vec<(usize, usize)>,
    pub map: Vec<Vec<Glyph>>,
	pub width: usize,
	pub height: usize, 
	pub dot: Glyph,
}

pub fn init_map(x: usize, y: usize) -> Flower {
	let mut dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15)};
	let mut map = vec![vec![dot; x as usize]; y as usize];
	let changes = vec![(0, 0)];
	let width = x;
	let height = y;
	Flower {
		changes,
		map,
		width,
		height,
		dot
	}
}

fn test_animation(flower: &mut Flower) {
	//let mut numx = rng::RandGen::new(34545);
	//let mut numy = rng::RandGen::new(43530);
	let x = rng::rng(40);
	let y = rng::rng(40);
	flower.changes.push((x, y));
	flower.map[x][y].ch = 'X';
	flower.map[x][y].color = termion::color::Rgb(255, 38, 106);
}

pub fn update_map(flower: &mut Flower) -> &mut Flower {
	flower.changes = vec![(0,0)];
	let y = flower.height;
	let x = flower.width;
	test_animation(flower);
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