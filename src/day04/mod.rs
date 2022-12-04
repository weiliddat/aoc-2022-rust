use std::fs;
use std::ops::RangeInclusive;
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

fn parse_input(raw: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
	let input = raw
		.lines()
		.map(|l| {
			let mut split = l.split(',');
			let first = split.next().unwrap();
			let second = split.next().unwrap();

			let mut first_split = first.split('-');
			let first_range_start = first_split
				.next()
				.and_then(|s| s.parse::<usize>().ok())
				.unwrap();
			let first_range_end = first_split
				.next()
				.and_then(|s| s.parse::<usize>().ok())
				.unwrap();

			let mut second_split = second.split('-');
			let second_range_start = second_split
				.next()
				.and_then(|s| s.parse::<usize>().ok())
				.unwrap();
			let second_range_end = second_split
				.next()
				.and_then(|s| s.parse::<usize>().ok())
				.unwrap();

			let first_range = first_range_start..=first_range_end;
			let second_range = second_range_start..=second_range_end;

			(first_range, second_range)
		})
		.collect::<Vec<_>>();

	input
}

fn part01(input: &Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>) -> usize {
	let contained_pairs = input
		.iter()
		.filter(|p| {
			(p.0.contains(&p.1.start()) && p.0.contains(&p.1.end()))
				|| (p.1.contains(&p.0.start()) && p.1.contains(&p.0.end()))
		})
		.collect::<Vec<_>>()
		.len();

	contained_pairs
}

fn part02(input: &Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>) -> usize {
	let contained_pairs = input
		.iter()
		.filter(|p| {
			p.0.contains(p.1.start())
				|| p.0.contains(p.1.end())
				|| p.1.contains(p.0.start())
				|| p.1.contains(p.0.end())
		})
		.collect::<Vec<_>>()
		.len();

	contained_pairs
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let raw = concat!(
			"2-4,6-8\n",
			"2-3,4-5\n",
			"5-7,7-9\n",
			"2-8,3-7\n",
			"6-6,4-6\n",
			"2-6,4-8\n",
		);

		let input = parse_input(raw);
		let result = part01(&input);

		assert_eq!(result, 2);
	}

	#[test]
	fn test_part02() {
		let raw = concat!(
			"2-4,6-8\n",
			"2-3,4-5\n",
			"5-7,7-9\n",
			"2-8,3-7\n",
			"6-6,4-6\n",
			"2-6,4-8\n",
		);

		let input = parse_input(raw);
		let result = part02(&input);

		assert_eq!(result, 4);
	}
}
