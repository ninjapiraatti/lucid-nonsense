use crate::world;
use crate::rng;

#[derive(Clone, Copy, Debug)]
pub struct Plant {
	x: u16,
	y: u16,
	height: u16,
	family: u16, // Later enum or smth?
	state: u16, // Also something else than magic number
}

pub fn animate_world(world: &mut world::World) {
	//let mut numx = rng::RandGen::new(34545);
	//let mut numy = rng::RandGen::new(43530);
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

pub fn write_glyph(x: usize, y: usize, glyph: char, perms: usize, world: &mut world::World) {
	world.changes.push((x, y));
	world.map[y][x].ch = glyph;
	world.map[y][x].permissions = perms;
	world.map[y][x].color = termion::color::Rgb(255, 38, 106);
}

pub fn plant_plant(x: u16, y: u16, height: u16, world: &mut world::World) {
	let mut plant = Plant {
		x: x,
		y: y,
		height: height,
		family: 1,
		state: 0,
	};
	world.plants.push(plant);
}

pub fn grow_plants(world: &mut world::World) {
	for p in 0..world.plants.len() {
		let z = rng::rng(world::VGA.chars().count());
		let glyph = world::VGA.chars().nth(z).unwrap();
		if world.plants[p].state < world.plants[p].height { // This is hecking stupid
			write_glyph(world.plants[p].x as usize, (world.plants[p].y - 2 - world.plants[p].state) as usize, glyph, 1, world);
			world.plants[p].state += 1;
			println!("{:?}", world.plants[p].state);
		}
	}
}