use std::fs;
use std::path::Path;

pub fn run() {
	let raw =
		fs::read_to_string(Path::new("src/day01/input.txt")).expect("Could not read input.txt");

	let input = parse_input(&raw);

	let part01_result = part01(&input);

	println!("part01 {}", part01_result);

	let part02_result = part02(&input);

	println!("part02 {}", part02_result);
}

fn parse_input(raw: &String) -> Vec<Vec<usize>> {
	raw.split("\n\n")
		.map(|s| {
			s.lines()
				.map(|l| l.parse::<usize>().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn part01(elves_inventory: &Vec<Vec<usize>>) -> usize {
	elves_inventory
		.iter()
		.map(|elf| elf.iter().fold(0, |acc, x| acc + x))
		.max()
		.unwrap()
}

fn part02(elves_inventory: &Vec<Vec<usize>>) -> usize {
	let mut calories = elves_inventory
		.iter()
		.map(|elf| elf.iter().fold(0, |acc, x| acc + x))
		.collect::<Vec<_>>();

	calories.sort();

	let top_3 = calories.get((calories.len() - 3)..calories.len()).unwrap();

	top_3.iter().sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let raw = String::from(concat!(
			"1000\n", "2000\n", "3000\n", "\n", "4000\n", "\n", "5000\n", "6000\n", "\n", "7000\n",
			"8000\n", "9000\n", "\n", "10000\n",
		));

		let input = parse_input(&raw);

		assert_eq!(
			input,
			vec![
				vec![1000, 2000, 3000],
				vec![4000],
				vec![5000, 6000],
				vec![7000, 8000, 9000],
				vec![10000],
			]
		);

		let result = part01(&input);

		assert_eq!(result, 24000);
	}

	#[test]
	fn test_part02() {
		let raw = String::from(concat!(
			"1000\n", "2000\n", "3000\n", "\n", "4000\n", "\n", "5000\n", "6000\n", "\n", "7000\n",
			"8000\n", "9000\n", "\n", "10000\n",
		));

		let input = parse_input(&raw);

		assert_eq!(
			input,
			vec![
				vec![1000, 2000, 3000],
				vec![4000],
				vec![5000, 6000],
				vec![7000, 8000, 9000],
				vec![10000],
			]
		);

		let result = part02(&input);

		assert_eq!(result, 45000);
	}
}
