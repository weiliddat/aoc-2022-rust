use std::fs;
use std::path::Path;

pub fn run() {
	let input = fs::read_to_string(Path::new("src/day00/input.txt"))
		.expect("Could not read input.txt")
		.lines()
		.collect::<Vec<_>>();
}
