use level_converter::load_level;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	let level = load_level(&args[1]);
	println!("{level}");
}