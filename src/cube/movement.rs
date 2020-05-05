extern crate rand;

use rand::{
	distributions::{Distribution, Standard},
	Rng,
};

#[derive(Copy, Clone)]
pub enum Direction {
	CW,
	CCW,
}

pub use Direction::*;

impl Distribution<Direction> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
		match rng.gen_range(0, 2) {
			0 => Direction::CW,
			_ => Direction::CCW,
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
            0 => Move::A(rand::random()),
            1 => Move::B(rand::random()),
            2 => Move::C(rand::random()),
            3 => Move::D(rand::random()),
            4 => Move::E(rand::random()),
            _ => Move::F(rand::random()),
        }
    }
}

pub static ALLMOVES: [Move; 12] = [A(CW), A(CCW), B(CW), B(CCW), C(CW), C(CCW),
							       D(CW), D(CCW), E(CW), E(CCW), F(CW), F(CCW)];