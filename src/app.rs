use std::error::Error;
use std::time::SystemTime;

use super::config::Config;
use super::cube::Cube;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	match config {
		Config::File(path) => solve_file(path),
		Config::Scramble(scrambles, count) => solve_scrambles(scrambles, count),
	}
}

fn solve_file(path: String) -> Result<(), Box<dyn Error>> {
	let mut cube = Cube::new_from_file(&path)?;

	let start = SystemTime::now();

	let (path, steps) = cube.solve();

	let elapsed = start.elapsed()?.as_millis();

	println!("{}", path);
	println!("Steps to solve: {}", steps);
	println!("Time to solve: {}ms", elapsed);

	Ok(())
}

fn solve_scrambles(scrambles: u16, count: u16) -> Result<(), Box<dyn Error>> {
	Ok(())
}