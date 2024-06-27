use crate::world::{self, Glyph};

pub struct Creature {
  pub map: Vec<Vec<world::Glyph>>,
  pub x: u16,
  pub y: u16,
}

impl Creature {
  pub fn new(x: u16, y: u16) -> Creature {
    let creature_char = Glyph {ch: '#', color: termion::color::Rgb(255, 64, 64), permissions: 0, z_index: 0};
    let creature_map = vec![vec![creature_char; 1]; 1];
    Creature {
      map: creature_map,
      x,
      y
    }
  }
}

pub fn spawn_creature(world: &mut world::World) {
    let creature = Creature::new((world.width / 2) as u16, (world.height / 2) as u16);
    world.entities.push(world::Entity::new(&creature.map, creature.x, creature.y));
}