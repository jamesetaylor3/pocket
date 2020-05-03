use std::error::Error;

use super::config::Config;
use super::cube::Cube;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	match config {
		Config::File(path) => solve_file(path),
		Config::Scramble(scrambles, count) => solve_scrambles(scrambles, count),
	}
}

fn solve_file(path: String) -> Result<(), Box<dyn Error>> {
	let cube = Cube::new_from_file(&path)?;

	Ok(())
}

fn solve_scrambles(scrambles: u16, count: u16) -> Result<(), Box<dyn Error>> {
	Ok(())
}