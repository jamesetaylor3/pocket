use std::fs::File;
use std::io::prelude::*;

use super::movement::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Color {
	WHITE,
	BLUE,
	RED,
	GREEN,
	ORANGE,
	YELLOW,
}

use Color::*;

pub struct Cube {
	state: [Color; 24],
}

impl Cube {
	pub fn new_from_file(path: &str) -> Result<Cube, &'static str> {
		let mut file = match File::open(path) {
			Ok(file) => file,
			Err(_) => return Err("Error reading file at specified path"),
		};

		let mut contents = String::new();
		if let Err(_) = file.read_to_string(&mut contents) {
			return Err("Error reading contents of file at path")
		}

		// remove whitespace
		contents.retain(|c| !c.is_whitespace());

		let contents = contents.to_ascii_uppercase();

		let mut state = [WHITE; 24];

		for (i, face) in contents.char_indices() {
			state[i] = match face {
				'W' => WHITE,
				'B' => BLUE,
				'R' => RED,
				'G' => GREEN,
				'O' => ORANGE,
				'Y' => YELLOW,
				_ => return Err("Read unidentifiable face color"),
			}
		}

		// will need to ensure that user submitted valid cube: every corner exists

		Ok(Cube{ state })
	}

	pub fn new() -> Cube {
		let state = [BLUE, BLUE, BLUE, BLUE, ORANGE, ORANGE, ORANGE, ORANGE, WHITE, WHITE, WHITE, WHITE,
					 RED, RED, RED, RED, YELLOW, YELLOW, YELLOW, YELLOW, GREEN, GREEN, GREEN, GREEN];

		return Cube{ state }
	}

	pub fn new_from_scramble(scrambles: u16) -> Cube {
		let mut cube = Cube::new();

		for _ in 0..scrambles {
			let m: Move = rand::random();
			cube.execute_move(m);
		}

		return cube
	}

	// maybe be able to change this to custom algorithms like a*
	pub fn solve(&self) {

	}

	// this will return what to do to solve it. U' -> R etc.
	pub fn solution(&self) -> String {
		String::from("no solution yet!")
	}

	// we do want to minimize the number of times we call this
	// here i really went for ease of programming understanding over performance
	// maybe change that
	fn is_solved(&self) -> bool {
		for face in 0..6 {
			let color = self.state[4 * face];
			for piece in 1..4 {
				if !(color == self.state[4 * face + piece]) {
					return false
				}
			}
		}
		true
	}

	// this is a monster of a function. would be cool to make it more modular but the swap functions are so specific
	// find something that is easy to read
	fn execute_move(&mut self, m: Move) {
		match m {
			A(dir) => {
				match dir {
					CW => {
						self.roll_main_right(0);
						self.roll_corners(16, 17, 12, 13, 8, 9, 4, 5);
					},
					CCW => {
						self.roll_main_left(0);
						self.roll_corners(4, 5, 8, 9, 12, 13, 16, 17);
					},
				}
			},
			B(dir) => {
				match dir {
					CW => {
						self.roll_main_right(4);
						self.roll_corners(0, 3, 8, 11, 20, 23, 18, 17);
					},
					CCW => {
						self.roll_main_left(4);
						self.roll_corners(18, 17, 20, 23, 8, 11, 0, 3);
					},
				}
			},
			C(dir) => {
				match dir {
					CW => {
						self.roll_main_right(8);
						self.roll_corners(3, 2, 6, 5, 21, 20, 15, 12);
					},
					CCW => {
						self.roll_main_left(8);
						self.roll_corners(15, 12, 21, 20, 6, 5, 3, 2);
					},
				}
			},
			D(dir) => {
				match dir {
					CW => {
						self.roll_main_right(12);
						self.roll_corners(1, 2, 9, 10, 21, 22, 19, 16);
					},
					CCW => {
						self.roll_main_left(12);
						self.roll_corners(19, 16, 21, 22, 9, 10, 1, 2);
					},
				}
			},
			E(dir) => {
				match dir {
					CW => {
						self.roll_main_left(16);
						self.roll_corners(0, 1, 13, 14, 22, 23, 7, 4);
					},
					CCW => {
						self.roll_main_right(16);
						self.roll_corners(7, 4, 22, 23, 13, 14, 0, 1);
					},
				}
			},
			F(dir) => {
				match dir {
					CW => {
						self.roll_main_right(20);
						self.roll_corners(19, 18, 15, 14, 11, 10, 7, 6);
					},
					CCW => {
						self.roll_main_left(20);
						self.roll_corners(7, 6, 11, 10, 15, 14, 19, 18);
					},
				}
			},
		}
	}

	#[inline]
	fn roll_main_left(&mut self, i: usize) {
		self.state.swap(i, i + 1);
		self.state.swap(i + 1, i + 2);
		self.state.swap(i + 2, i + 3);
	}

	#[inline]
	fn roll_main_right(&mut self, i: usize) {
		self.state.swap(i, i + 3);
		self.state.swap(i + 3, i + 2);
		self.state.swap(i + 2, i + 1);
	}

	#[inline]
	fn roll_corners(&mut self, i: usize, j: usize, k: usize, l: usize, m: usize, n: usize, o: usize, p: usize) {
		self.state.swap(m, o);
		self.state.swap(n, p);
		self.state.swap(k, m);
		self.state.swap(l, n);
		self.state.swap(i, k);
		self.state.swap(j, l);
	}
}

// need to create unit tests for all of the rotating and is_solved