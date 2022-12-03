use std::fs;
use std::path::Path;

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let raw = fs::read_to_string(path).expect("Could not read input.txt");

	let input = parse_input(&raw);

	let part01_result = part01(input);

	println!("part01 {:?}", part01_result);

	let part02_result = part02(input);

	println!("part02 {:?}", part02_result);
}

fn parse_input(raw: &String) -> () {
}

fn part01(input: ()) -> () {}

fn part02(input: ()) -> () {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let raw = String::from("");

		let input = parse_input(&raw);

		assert_eq!(input, ());

		let result = part01(input);

		assert_eq!(result, ());
	}

	#[test]
	fn test_part02() {
		let raw = String::from("");

		let input = parse_input(&raw);

		assert_eq!(input, ());

		let result = part02(input);

		assert_eq!(result, ());
	}
}
