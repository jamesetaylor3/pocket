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
	let mut cube = Cube::new_from_file(&path)?;

	let (path, steps, elapsed) = cube.solve();

	println!("{}", path);
	println!("Steps to solve: {}", steps);
	println!("Time to solve: {}ms", elapsed.as_millis());

	Ok(())
}

fn solve_scrambles(scrambles: u16, count: u16) -> Result<(), Box<dyn Error>> {
	println!("NOTE: We are not taking into account your count value of {}. Not there yet!", count);

	let mut cube = Cube::new_from_scramble(scrambles);

	let (path, steps, elapsed) = cube.solve();

	println!("Path: {}", path);
	println!("It took {} steps to solve.", steps);
	println!("Time to solve: {}ms", elapsed.as_millis());

	Ok(())
}