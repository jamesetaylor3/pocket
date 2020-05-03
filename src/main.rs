use std::env;
use std::process;

use pocket::app;
use pocket::config;

fn main() {
	let config = config::parse(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	if let Err(e) = app::run(config) {
		eprintln!("Application error: {}", e);

		process::exit(1);
	}
}