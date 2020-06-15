extern crate rand;

use std::fmt::{self, Write};
use rand::{
	distributions::{Distribution, Standard},
	Rng,
};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
	CW,
	CCW,
}

pub use Direction::*;

impl Distribution<Direction> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
		match rng.gen_range(0, 2) {
			0 => CW,
			_ => CCW,
		}
	}
}

#[derive(Copy, Clone)]
pub enum Move {
	A(Direction),
	B(Direction),
	C(Direction),
	D(Direction),
	E(Direction),
	F(Direction),
}

pub use Move::*;

impl Distribution<Move> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        match rng.gen_range(0, 6) {
            0 => A(rand::random()),
            1 => B(rand::random()),
            2 => C(rand::random()),
            3 => D(rand::random()),
            4 => E(rand::random()),
            _ => F(rand::random()),
        }
    }
}

impl fmt::Display for Move {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {

		let (face_char, dir) = match self {
			A(dir) => ('U', dir),
			B(dir) => ('L', dir),
			C(dir) => ('F', dir),
			D(dir) => ('R', dir),
			E(dir) => ('B', dir),
			F(dir) => ('D', dir),
		};


		fmt.write_char(face_char)?;

		if *dir == CCW {
			fmt.write_char('\'')?;
		}

		Ok(())
	}
}

pub static ALLMOVES: [Move; 12] = [A(CW), A(CCW), B(CW), B(CCW), C(CW), C(CCW),
							       D(CW), D(CCW), E(CW), E(CCW), F(CW), F(CCW)];