use clap::clap_app;


pub enum Config {
	File(String),
	Scramble(u16, u16)
}

pub fn retrieve() -> Result<Config, &'static str> {

	let matches = clap_app!(pocket =>
		(@setting SubcommandRequiredElseHelp)
		(version: "0.1.0")
		(author: "James Taylor <jamestaylor3@protonmail.com>")
		(about: "Solves pocket cubes from the command line")
		(@subcommand scramble =>
			(about: "Solve pocket cubes by scrambling")
			(@arg moves: -m +takes_value "number of times to randomly move in each scramble. default value is 6")
			(@arg cubes: -c +takes_value "number of cubes to run this on. default value is 1")
		)
		(@subcommand file =>
			(about: "Solve a pocket cube by reading cube from a file")
			(@arg path: -p +takes_value +required "path to file of pocket cube to be read")
		)
	).get_matches();

	if let Some(matches) = matches.subcommand_matches("file") {
		let path = matches.value_of("path").unwrap().to_string();
		return Ok(Config::File(path));
	}

	if let Some(matches) = matches.subcommand_matches("scramble") {
		let moves = matches.value_of("moves").unwrap_or("6");
		let cubes = matches.value_of("cubes").unwrap_or("1");

		let moves = moves.parse::<u16>().expect("Could not convert number of moves to integer!");
		let cubes = cubes.parse::<u16>().expect("Could not convert number of cubes to integer!");

		return Ok(Config::Scramble(moves, cubes))
	}

	Err("Somehow you reached an unreachable statement!")
}