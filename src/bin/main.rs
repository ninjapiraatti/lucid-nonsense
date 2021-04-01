extern crate termion;
use termion::{async_stdin, clear, color, cursor, style};
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};
use futures::{join, task};
use futures::stream::{FuturesUnordered, StreamExt};
use std::time::{Duration, Instant};
use std::thread;
use nonsense; // That is the name of the library of this program
mod graphics {
	pub const HORIZONTAL_WALL: &'static str = "═"; // Public constant. The &'static (I think) tells the program that this will live until the end.
	pub const VERTICAL_WALL: &'static str = "║";
	pub const TOP_LEFT_CORNER: &'static str = "╔";
    pub const TOP_RIGHT_CORNER: &'static str = "╗";
    pub const BOTTOM_LEFT_CORNER: &'static str = "╚";
    pub const BOTTOM_RIGHT_CORNER: &'static str = "╝";
	pub const PLAYER: char = '@';
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
	random: usize,
	player: Player,
	flower: nonsense::flower::Flower,
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
		self.draw_character(PLAYER as char, self.player.x, self.player.y);
	}

	fn draw_map(&mut self) {
		for y in 0..self.height {
			for x in 0..self.width {
				self.draw_character(self.flower.map[y][x], (x + 1) as u16, (y + 1) as u16);
			}
		}
	}

	fn draw_character(&mut self, chr: char, x: u16, y: u16) {
		write!(self.stdout, "{}{}{}{}", 
		termion::color::Bg(color::Rgb(1,5,5)),
		cursor::Goto(x, y as u16),
		chr,
		termion::color::Bg(color::Reset))
		.unwrap();
	}

	fn draw_horizontal_line(&mut self, chr: &str, x: u16, y: u16, width: u16) {
		//for _ in 0..width { self.stdout.write(chr.as_bytes()).unwrap(); }
		for i in x..width {
			write!(self.stdout, "{}{}{}{}", 
				termion::color::Bg(color::Rgb(5,25,25)), 
				cursor::Goto(i, y as u16),
				chr,
				termion::color::Bg(color::Reset))
				.unwrap();
		}
	}

	fn draw_vertical_line(&mut self, chr: &str, x: u16, y: u16, height: u16) {
		for i in y..height {
			write!(self.stdout, "{}{}{}{}", 
				termion::color::Bg(color::Rgb(5,25,25)), 
				cursor::Goto(x, i as u16),
				chr,
				termion::color::Bg(color::Reset))
				.unwrap();
		}
	}

	/*
	fn draw_box(&mut self, x1: u16, y1:u16, x2: u16, y2: u16) {
		self.draw_character(TOP_LEFT_CORNER, x1, y1);
		self.draw_horizontal_line(HORIZONTAL_WALL, x1 + 1, y1, x2);
		self.draw_character(TOP_RIGHT_CORNER, x2, y1);
		self.draw_vertical_line(VERTICAL_WALL, x1, y1 + 1, y2);
		self.draw_vertical_line(VERTICAL_WALL, x2, y1 + 1, y2);
		self.draw_character(BOTTOM_LEFT_CORNER, x1, y2);
		self.draw_horizontal_line(HORIZONTAL_WALL, x1 + 1, y2, x2);
		self.draw_character(BOTTOM_RIGHT_CORNER, x2, y2);
	}
	*/

	fn reset(&mut self) {
		let width: u16 = self.width as u16;
        let height: u16 = self.height as u16;
		write!(self.stdout,
			"{}{}{}",
			termion::clear::All,
			termion::cursor::Goto(1, 1),
			termion::color::Fg(color::Rgb(5,25,25)))
			.unwrap();
		//self.draw_box(1, 1, width, height);
		self.stdout.flush().unwrap();
	}

	fn update(&mut self) -> bool{
		let mut key_bytes = [0];
		let width: u16 = self.width as u16;
        let height: u16 = self.height as u16;
		let random: u16 = self.random as u16;
        self.stdin.read(&mut key_bytes).unwrap();

        //self.rand.write_u8(key_bytes[0]);
		self.clear_player();
		println!("{}{}", self.width, self.height);
        match key_bytes[0] {
            b'q' => return false,
            b'k' | b'w' => self.player.y -= 1,
            b'j' | b's' => self.player.y += 1,
            b'h' | b'a' => self.player.x -= 1,
            b'l' | b'd' => self.player.x += 1,
            _ => {},
        }
		self.draw_player();
		self.draw_map();
		true
	}
}

fn init_ui(width: usize, height: usize, random: usize) {
	let stdout = stdout();
	let mut stdout = stdout.lock().into_raw_mode().unwrap();
	let stdin = stdin();
	let stdin = stdin.lock();
	let mut ui = UI {
		width: width,
		height: height,
		stdin: stdin,
		stdout: stdout,
		random: random,
		player: Player {
			x: (width / 2) as u16,
			y: (height / 2) as u16
		},
		flower: nonsense::flower::init_map(width, height),
	};
	ui.reset();
	ui.start();
}

fn main() {
    // Initialize termion stuff.
	let size: (u16, u16) = termion::terminal_size().unwrap();
	init_ui(size.0 as usize, size.1 as usize, 66);
}