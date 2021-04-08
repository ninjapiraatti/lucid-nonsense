use crate as lib;
use crate::world;
pub fn check_bounds(world: &mut world::World) {
	if world.player.x >= lib::SIZE.0 {
		world.player.x = lib::SIZE.0 - 1;
	}
	if world.player.x == 0 {
		world.player.x = 1;
	}
	if world.player.y >= lib::SIZE.1 {
		world.player.y = lib::SIZE.1;
	}
	if world.player.y == 0{
		world.player.y = 1;
	}
}