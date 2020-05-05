use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashSet;

use super::movement::*;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
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
			cube.execute_move(rand::random());
		}

		return cube
	}

	// need smart pointers to back track the path
	pub fn solve(&mut self) -> (String, u8) {
		let mut queue: VecDeque<[Color; 24]> = VecDeque::new();
		let mut discovered: HashSet<[Color; 24]> = HashSet::new();

		discovered.insert(self.state);
		queue.push_back(self.state);

		while !queue.is_empty() {
			// this is guarenteed to not panic
			self.state = queue.pop_front().unwrap();

			if self.is_solved() {
				break;
			}

			let curr_state = self.state.clone();  // don't know if this clone is needed!

			for &m in ALLMOVES.iter() {
				self.execute_move(m);

				if discovered.len() % 1000000 == 0 {
					println!("{:?}", discovered.len());
				}

				if !discovered.contains(&self.state) {
					discovered.insert(self.state);
					queue.push_back(self.state);
				}
				self.state = curr_state;
			}

		}

		(String::from("got a solution"), 0)
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

	fn is_valid_cube(&self) -> bool {
		true
	}

	fn execute_move(&mut self, m: Move) {
		let (dir, face, mut corners) = match m {
			A(d) => (d, 0, [16, 17, 12, 13, 8, 9, 4, 5]),
			B(d) => (d, 4, [0, 3, 8, 11, 20, 23, 18, 17]),
			C(d) => (d, 8, [15, 12, 21, 20, 6, 5, 3, 2]),
			D(d) => (d, 12, [19, 16, 21, 22, 9, 10, 1, 2]),
			E(d) => (d, 16, [7, 4, 22, 23, 13, 14, 0, 1]),
			F(d) => (d, 20, [7, 6, 11, 10, 15, 14, 19, 18]),
		};

		let mut face = [face, face + 1, face + 2, face + 3];

		if let CCW = dir {
			face.reverse();
			corners.reverse();
		}

		let mut roll_main = |f: [usize; 4]| {
			self.state.swap(f[3], f[2]);
			self.state.swap(f[2], f[1]);
			self.state.swap(f[1], f[0]);
		};

		roll_main(face);

		let mut roll_corners = |c: [usize; 8]| {
			self.state.swap(c[4], c[6]);
			self.state.swap(c[5], c[7]);
			self.state.swap(c[2], c[4]);
			self.state.swap(c[3], c[5]);
			self.state.swap(c[0], c[2]);
			self.state.swap(c[1], c[3]);
		};

		roll_corners(corners);
	}
}

// UNIT TESTING

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_solved_true() {
		let c = Cube::new();

		assert!(c.is_solved());
	}

	#[test]
	fn is_solved_false() {
		let mut c = Cube::new();
		let m = A(CW);
		c.execute_move(m);

		assert!(!c.is_solved());
	}

	#[test]
	fn execute_moves_1() {
		let mut c = Cube::new();
		c.execute_move(A(CW));
		c.execute_move(B(CCW));
		c.execute_move(E(CW));

		let state = [YELLOW, RED, BLUE, WHITE, BLUE, ORANGE, ORANGE, RED, GREEN, RED, WHITE, GREEN,
					 YELLOW, GREEN, ORANGE, RED, YELLOW, ORANGE, BLUE, BLUE, YELLOW, GREEN, WHITE, WHITE];

		assert_eq!(c.state, state);
	}

	#[test]
	fn execute_moves_2() {
		let mut c = Cube::new();
		c.execute_move(C(CCW));
		c.execute_move(D(CW));
		c.execute_move(F(CCW));

		let state = [BLUE, WHITE, WHITE, RED, ORANGE, BLUE, GREEN, WHITE, WHITE, ORANGE, RED, RED,
					 GREEN, GREEN, YELLOW, BLUE, RED, YELLOW, BLUE, ORANGE, YELLOW, YELLOW, GREEN, ORANGE];
		
		assert_eq!(c.state, state);
	}
}