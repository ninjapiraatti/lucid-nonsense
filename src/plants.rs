use crate::world;
use crate::rng;
use crate as lib;

#[derive(Clone, Copy, Debug)]
pub struct Plant {
	pub x: u16,
	pub y: u16,
	pub height: u16,
	pub family: u16, // Later enum or smth?
	pub state: u16, // Also something else than magic number
}

pub fn grow_grass(world: &mut world::World) {
	let hue1 = rng::rng(15) as u8;
	let hue2 = rng::rng(15) as u8;
	let x = rng::rng(world.width);
	let y = rng::rng(world.height);
	let z = rng::rng(lib::VGA.chars().count());
	if world.map[y][x].permissions == 0 {
		world.changes.push((x, y));
		world.map[y][x].ch = lib::VGA.chars().nth(z).unwrap();
		world.map[y][x].color = termion::color::Rgb(0, 20 + hue1, 10 + hue2);
	}
}

pub fn plant_plant(world: &mut world::World, height: u16) {
	let plant = Plant {
		x: world.player.x,
		y: world.player.y,
		height: height,
		family: 1,
		state: 0,
	};
	world.plants.push(plant);
}

pub fn grow_plant(world: &mut world::World, p: usize){
	let z = rng::rng(lib::VGA.chars().count());
	let glyph = lib::VGA.chars().nth(z).unwrap();
	if world.plants[p].state < world.plants[p].height { // This is hecking stupid. world.plants[p]
		let x = world.plants[p].x as usize;
		let y = (world.plants[p].y - 2 - world.plants[p].state) as usize;
		world.changes.push((x, y));
		world.map[y][x].ch = glyph;
		world.map[y][x].permissions = 1;
		world.map[y][x].color = termion::color::Rgb(255, 38, 106);
		world.plants[p].state += 1;
	}
}