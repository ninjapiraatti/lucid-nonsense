extern crate termion;
use termion::{color, async_stdin, cursor, style};
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout}; // Add stdin if you need to switch away from async_stdin
use std::thread;
use nonsense; // That is the name of the library of this program
use nonsense::world;
use nonsense::plants;
use nonsense::utils;
mod graphics {
	pub const PLAYER: char = '🦀';
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
	world: world::World,
}

impl <R: Read, W: Write> UI<R, W> { // What does this declaration really do?
	fn start(&mut self) {
		write!(self.stdout, "{}", cursor::Hide).unwrap();
		self.reset();
		loop {
			if !self.update() {
                return;
            }
			write!(self.stdout, "{}", style::Reset).unwrap();
            self.stdout.flush().unwrap();
		}
	}
	fn clear_player(&mut self) {
		write!(self.stdout, "{} ", cursor::Goto(self.world.player.x, self.world.player.y)).unwrap();
	}

	fn draw_player(&mut self) {
		self.draw_character(PLAYER as char, termion::color::Rgb(20,60,60), self.world.player.x, self.world.player.y);
	}

	fn draw_debug(&mut self) {
		write!(self.stdout, "{}{}{}{:?}{}", 
		termion::color::Fg(color::Rgb(50,50,50)),
		termion::color::Bg(color::Rgb(1,5,5)), 
		cursor::Goto(2, 2 as u16),
		self.world.changes.len(),
		termion::color::Bg(color::Reset))
		.unwrap();
	}

	fn draw_map(&mut self) {
		self.world.update_world();
		for val in 0..self.world.changes.len() {
			let x = self.world.changes[val].0;
			let y = self.world.changes[val].1;
			self.draw_character(self.world.map[y][x].ch, self.world.map[y][x].color, (x + 1) as u16, (y + 1) as u16);	
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

	fn reset(&mut self) {
		write!(self.stdout,
			"{}{}{}",
			termion::clear::All,
			termion::cursor::Goto(1, 1),
			termion::color::Fg(color::Rgb(5,25,25)))
			.unwrap();
		self.stdout.flush().unwrap();
		for y in 0..self.height {
			for x in 0..self.width {
				self.draw_character(self.world.map[y][x].ch, self.world.map[y][x].color, (x + 1) as u16, (y + 1) as u16);
			}
		}
	}

	fn update(&mut self) -> bool{
		let mut key_bytes = [0];
        self.stdin.read(&mut key_bytes).unwrap();
		self.clear_player();
        match key_bytes[0] {
            b'q' => return false,
            b'k' | b'w' => {self.world.player.y -= 1; utils::check_bounds(&mut self.world);} // Any way to avoid this repetition?
            b'j' | b's' => {self.world.player.y += 1; utils::check_bounds(&mut self.world);}
            b'h' | b'a' => {self.world.player.x -= 1; utils::check_bounds(&mut self.world);}
            b'l' | b'd' => {self.world.player.x += 1; utils::check_bounds(&mut self.world);}
			b'f' => plants::plant_plant(&mut self.world, 10),
            _ => {},
        }
		self.draw_map();
		self.draw_player();
		self.draw_debug();
		let delay = std::time::Duration::from_millis(30); // The player should update fast and the rest of the world slow. How to do this?
		thread::sleep(delay);
		true
	}
}

fn init_ui(width: usize, height: usize) {
	let stdout = stdout();
	let stdout = stdout.lock().into_raw_mode().unwrap();
	let stdin = async_stdin();
	//let stdin = stdin.lock();
	let mut ui = UI {
		width,
		height,
		stdin,
		stdout,
		world: nonsense::world::init_world(width, height),
	};
	ui.reset();
	ui.start();
}

fn main() {
    // Initialize termion stuff.
	let size: (u16, u16) = termion::terminal_size().unwrap();
	init_ui(size.0 as usize, size.1 as usize);
}

