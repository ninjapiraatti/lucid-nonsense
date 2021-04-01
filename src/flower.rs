use crate::rng;
pub fn draw_flower(x: u16, y: u16, count: u16) -> (u16, u16, char, bool) {
	let xf = rng::rng(20) as u16;
	let yf = rng::rng(20) as u16;
	if count > 9 {
		return(1, 1, 'Ö', true);
	}
	return(xf, yf, 'ä', false)
}