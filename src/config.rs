pub enum Config {
	File(String),
	Scramble(u16, u16)
}

/* this could certainly use some cleaning up */
pub fn parse(mut args: std::env::Args) -> Result<Config, &'static str> {
	args.next();

	match args.next().as_ref().map(String::as_str) {
		Some("file") => {
			match args.next() {
				Some(path) => Ok(Config::File(path)),
				None => return Err("Did not receive a file path"),
			}
		},
		Some("scramble") => {
			let scrambles = match args.next() {
				Some(s) => s.parse::<u16>().unwrap(),
				None => return Err("Did not receive a number of scrambles per cube"),
			};
			let count = match args.next() {
				Some(s) => s.parse::<u16>().unwrap(),
				None => return Err("Did not receive a count of cubes to scrable")
			};
			Ok(Config::Scramble(scrambles, count))
		},
		Some(_) => return Err("Did not receive a valid first argument"),
		None => return Err("Did not receive a first argument"),
	}
}