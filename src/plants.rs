use crate::world;
use crate::rng;

#[derive(Clone, Debug)]
pub struct Plant {
	pub x: u16,
	pub y: u16,
	pub height: u16,
	pub width: u16,
	pub family: u16, // Later enum or smth?
	pub totalstate: u16, // Also something else than magic number
	pub trunkstate: u16,
	pub branchnodes: Vec<(u16, u16)>,
	pub leafnodes: Vec<(u16, u16)>
}

pub fn grow_grass(world: &mut world::World) {
	let hue1 = rng::rng(15) as u8;
	let hue2 = rng::rng(15) as u8;
	let x = rng::rng(world.width);
	let y = rng::rng(world.height);
	let z = rng::rng(crate::CHARS_ALL.chars().count());
	if world.map[y][x].permissions == 0 {
		world.changes.push((x, y));
		world.map[y][x].ch = crate::CHARS_ALL.chars().nth(z).unwrap();
		world.map[y][x].color = termion::color::Rgb(0, 20 + hue1, 10 + hue2);
	}
}

pub fn plant_plant(world: &mut world::World, height: u16, width: u16) {
	let plant = Plant {
		x: world.player.x,
		y: world.player.y,
		height,
		width,
		family: 1,
		trunkstate: 0,
		branchnodes: vec![],
		leafnodes: vec![],
		totalstate: 0
	};
	world.plants.push(plant);
}

pub fn grow_plant(world: &mut world::World, p: usize){
	let trunk = crate::CHARS_VRT.chars().nth(rng::rng(crate::CHARS_VRT.chars().count())).unwrap();
	let branch = crate::CHARS_HRZ.chars().nth(rng::rng(crate::CHARS_HRZ.chars().count())).unwrap();
	let leaf = crate::CHARS_RND.chars().nth(rng::rng(crate::CHARS_RND.chars().count())).unwrap();
	let rnd = rng::rng(100);
	if world.plants[p].totalstate < world.plants[p].height { // This is hecking stupid. world.plants[p] -> something shorter
		let x: i32 = world.plants[p].x as i32;
		let y: i32 = world.plants[p].y as i32 - 2 - world.plants[p].trunkstate as i32;
		if x >= 0 && y >= 0 {
			let x = x as usize;
			let y = y as usize;
			world.changes.push((x, y));
			world.map[y][x].ch = trunk;
			world.map[y][x].z = world.plants[p].y;
			world.map[y][x].permissions = 1;
			world.map[y][x].color = termion::color::Rgb(255, 38, 106);
			world.plants[p].trunkstate += 1;
		}
		if world.plants[p].trunkstate > 3 && world.plants[p].trunkstate < world.plants[p].height{
			if rnd % 4 == 0 {
				if rnd < 51 {
					world.plants[p].branchnodes.push((x as u16 - 1, y as u16));
				} else {
					world.plants[p].branchnodes.push((x as u16 + 1, y as u16));
				}
			}
		}
		world.plants[p].totalstate += 1;
	}
}