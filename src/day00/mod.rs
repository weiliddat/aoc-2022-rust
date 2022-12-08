use std::fs;
use std::path::Path;

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let input = fs::read_to_string(path).expect("Could not read input.txt");

	let part01_result = part01(&input);
	println!("part01 {:?}", part01_result);

	let part02_result = part02(&input);
	println!("part02 {:?}", part02_result);
}

fn part01(input: &str) -> usize {
	0
}

fn part02(input: &str) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let input = concat!(
		);
		let result = part01(input);
		assert_eq!(result, 1);
	}

	#[test]
	fn test_part02() {
		let input = concat!(
		);
		let result = part02(input);
		assert_eq!(result, 1);
	}
}
