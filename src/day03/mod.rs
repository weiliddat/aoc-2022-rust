use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let raw = fs::read_to_string(path).expect("Could not read input.txt");

	let input = parse_input_01(&raw);
	let part01_result = part01(&input);
	println!("part01 {:?}", part01_result);

	let input = parse_input_02(&raw);
	let part02_result = part02(&input);
	println!("part02 {:?}", part02_result);
}

type Bag = HashSet<char>;

fn parse_input_01(raw: &str) -> Vec<(Bag, Bag)> {
	let input = raw
		.lines()
		.map(|l| {
			let mut bag_part_1 = Bag::new();
			let mut bag_part_2 = Bag::new();
			let length = l.len();
			let (first_string, second_string) = l.split_at(length / 2);
			first_string.chars().for_each(|c| {
				bag_part_1.insert(c);
			});
			second_string.chars().for_each(|c| {
				bag_part_2.insert(c);
			});

			(bag_part_1, bag_part_2)
		})
		.collect::<Vec<_>>();

	input
}

fn part01(input: &Vec<(Bag, Bag)>) -> usize {
	let priority_map = ('a'..='z')
		.chain('A'..='Z')
		.enumerate()
		.map(|(a, b)| (b, a + 1))
		.collect::<HashMap<_, _>>();

	let priority_scores: usize = input
		.iter()
		.map(|(one, two)| {
			let common = one.intersection(two).collect::<Vec<_>>();
			let common = common.get(0).unwrap();
			let score = priority_map.get(common).unwrap();
			score
		})
		.sum();

	priority_scores
}

fn parse_input_02(raw: &str) -> Vec<(Bag, Bag, Bag)> {
	let line_chunks = raw.lines().array_chunks::<3>();
	line_chunks
		.map(|lines| {
			let mut bag0 = Bag::new();
			let mut bag1 = Bag::new();
			let mut bag2 = Bag::new();

			lines.get(0).unwrap().chars().for_each(|c| {
				bag0.insert(c);
			});
			lines.get(1).unwrap().chars().for_each(|c| {
				bag1.insert(c);
			});
			lines.get(2).unwrap().chars().for_each(|c| {
				bag2.insert(c);
			});

			(bag0, bag1, bag2)
		})
		.collect::<Vec<_>>()
}

fn part02(input: &Vec<(Bag, Bag, Bag)>) -> usize {
	let priority_map = ('a'..='z')
		.chain('A'..='Z')
		.enumerate()
		.map(|(a, b)| (b, a + 1))
		.collect::<HashMap<_, _>>();

	let priority_scores: usize = input
		.iter()
		.map(|(one, two, three)| {
			let first_inter = one
				.intersection(two)
				.map(|a| a.clone())
				.collect::<HashSet<_>>();
			let second_inter = first_inter.intersection(three).collect::<Vec<_>>();
			let common = second_inter.get(0).unwrap();
			let score = priority_map.get(common).unwrap();
			score
		})
		.sum();

	priority_scores
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let raw = concat!(
			"vJrwpWtwJgWrhcsFMMfFFhFp\n",
			"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
			"PmmdzqPrVvPwwTWBwg\n",
			"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
			"ttgJtRGJQctTZtZT\n",
			"CrZsJsPPZsGzwwsLwLmpwMDw\n",
		);

		let input = parse_input_01(raw);
		let result = part01(&input);
		assert_eq!(result, 157);
	}

	#[test]
	fn test_part02() {
		let raw = concat!(
			"vJrwpWtwJgWrhcsFMMfFFhFp\n",
			"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
			"PmmdzqPrVvPwwTWBwg\n",
			"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
			"ttgJtRGJQctTZtZT\n",
			"CrZsJsPPZsGzwwsLwLmpwMDw\n",
		);

		let input = parse_input_02(&raw);
		let result = part02(&input);
		assert_eq!(result, 70);
	}
}
