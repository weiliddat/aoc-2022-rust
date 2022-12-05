use std::fs;
use std::path::Path;

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let raw = fs::read_to_string(path).expect("Could not read input.txt");
	let input = parse_input(&raw);

	let part01_result = part01(&input);
	println!("part01 {:?}", part01_result);

	let part02_result = part02(&input);
	println!("part02 {:?}", part02_result);
}

fn parse_input(raw: &str) -> Vec<()> {
	let input = raw
		.lines()
		.map(|l| {
			
		})
		.collect::<Vec<_>>();

	input
}

fn part01(input: &Vec<()>) -> usize {
	0
}

fn part02(input: &Vec<()>) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let raw = concat!(
			""
		);

		let input = parse_input(raw);
		let result = part01(&input);

		assert_eq!(result, 1);
	}

	#[test]
	fn test_part02() {
		let raw = concat!(
			""
		);

		let input = parse_input(raw);
		let result = part02(&input);

		assert_eq!(result, 1);
	}
}
