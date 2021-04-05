extern crate termion;
use termion::{async_stdin, clear, color, cursor, style};
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};
use std::time::{Duration, Instant};
use std::thread;
use nonsense; // That is the name of the library of this program
use nonsense::rng;
use nonsense::plants;
mod graphics {
	pub const PLAYER: char = '🦀';
}
mod colors {
	pub const CYAN: termion::color::Rgb = termion::color::Rgb(1, 221, 214); // This can't be right
}

use self::graphics::*;

// Player
struct Player {
	x: u16,
	y: u16
}

// The UI state.
pub struct UI<R, W> {
    width: usize,
    height: usize,
    /// Standard input.
    stdin: R,
    /// Standard output.
    stdout: W,
	player: Player,
	world: nonsense::world::World,
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
		write!(self.stdout, "{} ", cursor::Goto(self.player.x, self.player.y)).unwrap();
	}

	fn draw_player(&mut self) {
		self.draw_character(PLAYER as char, termion::color::Rgb(20,60,60), self.player.x, self.player.y);
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
		nonsense::world::update_map(&mut self.world);
		//let mut coords = &self.world.changes.iter();
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
		let width: u16 = self.width as u16;
        let height: u16 = self.height as u16;
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
		let width: u16 = self.width as u16;
        let height: u16 = self.height as u16;
        self.stdin.read(&mut key_bytes).unwrap();
		let mut rnd = rng::RandGen::new(self.player.x as usize * 4567);
        //self.rand.write_u8(key_bytes[0]);
		self.clear_player();
		//println!("{}", "Player");
        match key_bytes[0] {
            b'q' => return false,
            b'k' | b'w' => self.player.y -= 1,
            b'j' | b's' => self.player.y += 1,
            b'h' | b'a' => self.player.x -= 1,
            b'l' | b'd' => self.player.x += 1,
			b'f' => plants::plant_plant(self.player.x, self.player.y, 10, &mut self.world),
            _ => {},
        }
		self.draw_map();
		self.draw_player();
		self.draw_debug();
		let delay = std::time::Duration::from_millis(20);
		let now = std::time::Instant::now();
		thread::sleep(delay);
		true
	}
}

fn init_ui(width: usize, height: usize) {
	let stdout = stdout();
	let mut stdout = stdout.lock().into_raw_mode().unwrap();
	let stdin = async_stdin();
	//let stdin = stdin.lock();
	let mut ui = UI {
		width: width,
		height: height,
		stdin: stdin,
		stdout: stdout,
		player: Player {
			x: (width / 2) as u16,
			y: (height / 2) as u16
		},
		world: nonsense::world::init_map(width, height),
	};
	ui.reset();
	ui.start();
}

fn main() {
    // Initialize termion stuff.
	let size: (u16, u16) = termion::terminal_size().unwrap();
	init_ui(size.0 as usize, size.1 as usize);
}

