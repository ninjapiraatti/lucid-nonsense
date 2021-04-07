use crate::world;
use crate::rng;

#[derive(Clone, Copy, Debug)]
pub struct Plant {
	pub x: u16,
	pub y: u16,
	pub height: u16,
	pub family: u16, // Later enum or smth?
	pub state: u16, // Also something else than magic number
}

pub fn animate_world(world: &mut world::World) {
	let hue1 = rng::rng(15) as u8;
	let hue2 = rng::rng(15) as u8;
	let x = rng::rng(world.width);
	let y = rng::rng(world.height);
	let z = rng::rng(world::VGA.chars().count());
	if world.map[y][x].permissions == 0 {
		world.changes.push((x, y));
		world.map[y][x].ch = world::VGA.chars().nth(z).unwrap();
		world.map[y][x].color = termion::color::Rgb(0, 20 + hue1, 10 + hue2);
	}
}

pub fn grow_plant(world: &mut world::World, p: usize){
	let z = rng::rng(world::VGA.chars().count());
	let glyph = world::VGA.chars().nth(z).unwrap();
	if world.plants[p].state < world.plants[p].height { // This is hecking stupid
		let x = world.plants[p].x as usize;
		let y = (world.plants[p].y - 2 - world.plants[p].state) as usize;
		world.changes.push((x, y));
		world.map[y][x].ch = glyph;
		world.map[y][x].permissions = 1;
		world.map[y][x].color = termion::color::Rgb(255, 38, 106);
		world.plants[p].state += 1;
	}
}