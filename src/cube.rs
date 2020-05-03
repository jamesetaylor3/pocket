extern crate rand;

use std::fs::File;
use std::io::prelude::*;

use rand::{
	distributions::{Distribution, Standard},
	Rng,
};

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

enum Direction {
	CW,
	CCW,
}

impl Distribution<Direction> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
		match rng.gen_range(0, 2) {
			0 => Direction::CW,
			_ => Direction::CCW,
		}
	}
}

enum Move {
	A(Direction),
	B(Direction),
	C(Direction),
	D(Direction),
	E(Direction),
	F(Direction),
}

impl Distribution<Move> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        match rng.gen_range(0, 6) {
            0 => Move::A(rand::random()),
            1 => Move::B(rand::random()),
            2 => Move::C(rand::random()),
            3 => Move::D(rand::random()),
            4 => Move::E(rand::random()),
            _ => Move::F(rand::random()),
        }
    }
}

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
		if let Err(e) = file.read_to_string(&mut contents) {
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

		Ok(Cube{state})
	}

	pub fn new_from_scramble(scrambles: u16) -> Cube {
		let state = [BLUE, BLUE, BLUE, BLUE, ORANGE, ORANGE, ORANGE, ORANGE, WHITE, WHITE, WHITE, WHITE,
					 RED, RED, RED, RED, YELLOW, YELLOW, YELLOW, YELLOW, GREEN, GREEN, GREEN, GREEN];

		let mut cube = Cube{state};

		for _ in 0..scrambles {
			let m: Move = rand::random();
			cube.execute_move(m);
		}

		return cube
	}

	// maybe be able to change this to custom algorithms like a*
	// pub fn solve

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

	fn execute_move(&mut self, m: Move) {

	}
}