use crate::rng;

pub struct Flower {
    pub n: usize,
    pub map: Vec<Vec<char>>,
}

pub fn init_map(x: usize, y: usize) -> Flower {
	let mut map = vec![vec!['.'; x as usize]; y as usize];
	let n = 42;
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