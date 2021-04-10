use crate::world;
pub fn check_bounds(world: &mut world::World) {
	if world.player.x >= crate::SIZE.0 {
		world.player.x = crate::SIZE.0 - 1;
	}
	if world.player.x == 0 {
		world.player.x = 1;
	}
	if world.player.y >= crate::SIZE.1 {
		world.player.y = crate::SIZE.1;
	}
	if world.player.y == 0{
		world.player.y = 1;
	}
}