extern crate termion;
use nonsense::rng;
use termion::{color, async_stdin, cursor, style};
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout}; // Add stdin if you need to switch away from async_stdin
use std::thread;
use nonsense::{self, world::World}; // That is the name of the library of this program
use nonsense::world;
use nonsense::plants;
use nonsense::buildings;
mod graphics {
	pub const PLAYER: char = '♥';
}
use self::graphics::*;

// The UI state.
pub struct UI<R, W> {
	width: usize,
	height: usize,
	/// Standard input.
	stdin: R,
	/// Standard output.
	stdout: W,
}

impl <R: Read, W: Write> UI<R, W> {
	fn run(&mut self, world: &mut World, buildings: &mut Vec<buildings::Building>) {
		write!(self.stdout, "{}", cursor::Hide).unwrap();
		self.reset(world);
		loop {
			if !self.update(world, buildings) {
				return;
      }
			write!(self.stdout, "{}", style::Reset).unwrap();
      self.stdout.flush().unwrap();
		}
	}

	fn clear_player(&mut self, world: &mut World) {
		if world.map[(world.player.y - 1) as usize][(world.player.x - 1) as usize].z < world.player.y { // WTF
			write!(self.stdout, "{} ", cursor::Goto(world.player.x, world.player.y)).unwrap();
		}
	}

	fn draw_player(&mut self, world: &mut World) {
		if world.map[(world.player.y - 1) as usize][(world.player.x - 1) as usize].z < world.player.y {
			self.draw_character(PLAYER as char, termion::color::Rgb(240,160,0), world.player.x, world.player.y);
		}
	}

	fn move_player_and_plant(&mut self, world: &mut World) {
		let rnd = rng::rng(1000);
		match rnd {
			1..=100 => {world.player.y -= if world.player.y == 0 {0} else {1};}
			101..=200 => {world.player.y += if world.player.y == world.height as u16 {0} else {1};}
			201..=300 => {world.player.x -= if world.player.x == 0 {0} else {1};}
			301..=400 => {world.player.x += if world.player.y == world.width as u16 {0} else {1};}
			666 => plants::plant_plant(world, 10, 10),
			_ => {},
		}
	}

	fn draw_debug(&mut self, world: &mut World) {
		write!(self.stdout, "{}{}{}{:?} {:?}\n{:?}\n{}", 
		termion::color::Fg(color::Rgb(50,50,50)),
		termion::color::Bg(color::Rgb(1,5,5)), 
		cursor::Goto(2, 2 as u16),
		world.changes.len(),
		world.debugstr,
		world.debugint,
		termion::color::Bg(color::Reset))
		.unwrap();
	}

	fn draw_map(&mut self, world: &mut World) {
		world.update_world();
		for val in 0..world.changes.len() {
			let x = world.changes[val].0;
			let y = world.changes[val].1;
			self.draw_character(world.map[y][x].ch, world.map[y][x].color, (x + 1) as u16, (y + 1) as u16);
			//world.debugstr = format!("Drawing {}", world.map[y][x].ch);
		}
	}

	fn draw_character(&mut self, chr: char, color: termion::color::Rgb, x: u16, y: u16) {
		write!(self.stdout, "{}{}{}{}{}", 
		termion::color::Fg(color),
		termion::color::Bg(color::Rgb(1,5,5)), 
		cursor::Goto(x, y as u16),
		chr,
		termion::color::Bg(color::Reset))
		.unwrap();
	}

	fn reset(&mut self, world: &mut World) {
		write!(self.stdout,
			"{}{}{}",
			termion::clear::All,
			termion::cursor::Goto(1, 1),
			termion::color::Fg(color::Rgb(5,25,25)))
			.unwrap();
		self.stdout.flush().unwrap();
		for y in 0..self.height {
			for x in 0..self.width {
				self.draw_character(world.map[y][x].ch, world.map[y][x].color, (x + 1) as u16, (y + 1) as u16);
			}
		}
	}

	fn update(&mut self, world: &mut World, buildings: &mut Vec<buildings::Building>) -> bool{
		let mut key_bytes = [0];
    self.stdin.read(&mut key_bytes).unwrap();
		self.clear_player(world);
		match key_bytes[0] {
			b'q' => return false,
			b'k' | b'w' => {world.player.y -= if world.player.y == 0 {0} else {1};}// Any way to avoid this repetition?
			b'j' | b's' => {world.player.y += if world.player.y == world.height as u16 {0} else {1};}
			b'h' | b'a' => {world.player.x -= if world.player.x == 0 {0} else {1};}
			b'l' | b'd' => {world.player.x += if world.player.y == world.width as u16 {0} else {1};}
			b'f' => plants::plant_plant(world, 10, 10),
			b'g' => buildings.push(buildings::Building::new(world.player.x as usize, world.player.y as usize, 12, 8, 5)),
			_ => {},
		}
		self.draw_map(world);
		//self.move_player_and_plant(world);
		self.draw_player(world);
		self.draw_debug(world);
		let delay = std::time::Duration::from_millis(30); // The player should update fast and the rest of the world slow. How to do this?
		thread::sleep(delay);
		true
	}
}

fn run_game(width: usize, height: usize, world: &mut World, buildings: &mut Vec<buildings::Building>) {
	let stdout = stdout();
	let stdout = stdout.lock().into_raw_mode().unwrap();
	let stdin = async_stdin();
	//let stdin = stdin.lock(); // Bring this back in if you change back to synced stdin
	let mut ui = UI {
		width,
		height,
		stdin,
		stdout,
	};
	ui.reset(world);
	ui.run(world, buildings);
}

fn main() {
	let size: (u16, u16) = termion::terminal_size().unwrap();
	let mut world = world::init_world(size.0 as usize, size.1 as usize);
	let mut buildings = buildings::init_buildings();
	run_game(size.0 as usize, size.1 as usize, &mut world, &mut buildings);
}

