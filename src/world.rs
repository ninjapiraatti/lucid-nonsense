use crate::rng;

#[derive(Clone, Copy, Debug)]
pub struct Glyph {
	pub ch: char,
	pub color: termion::color::Rgb
}

#[derive(Clone, Debug)]
pub struct World {
    pub changes: Vec<(usize, usize)>,
    pub map: Vec<Vec<Glyph>>,
	pub width: usize,
	pub height: usize, 
	pub dot: Glyph,
}

pub fn init_map(x: usize, y: usize) -> World {
	let mut dot = Glyph {ch: '.', color: termion::color::Rgb(15, 15, 15)};
	let mut map = vec![vec![dot; x as usize]; y as usize];
	let changes = vec![(0, 0)];
	let width = x;
	let height = y;
	World {
		changes,
		map,
		width,
		height,
		dot
	}
}

fn test_animation(world: &mut World) {
	//let mut numx = rng::RandGen::new(34545);
	//let mut numy = rng::RandGen::new(43530);
	let x = rng::rng(40);
	let y = rng::rng(40);
	world.changes.push((x, y));
	world.map[x][y].ch = 'X';
	world.map[x][y].color = termion::color::Rgb(255, 38, 106);
}

pub fn update_map(world: &mut World) -> &mut World {
	world.changes = vec![(0,0)];
	let y = world.height;
	let x = world.width;
	test_animation(world);
	world.map[y / 2][x / 2].ch = 'O';
	world.map[y / 2][x / 2].color = termion::color::Rgb(255, 38, 106);
	world
}

pub fn draw_world(x: u16, y: u16, count: u16) -> (u16, u16, char, bool) {
	let xf = rng::rng(20) as u16;
	let yf = rng::rng(20) as u16;
	if count > 9 {
		return(1, 1, 'Ö', true);
	}
	return(xf, yf, 'ä', false)
}