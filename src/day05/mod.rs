use std::collections::HashMap;
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

#[derive(Debug, PartialEq)]
struct Step {
	source: usize,
	target: usize,
	amount: usize,
}

type Stacks = HashMap<usize, Vec<char>>;
type Steps = Vec<Step>;

fn parse_input(raw: &str) -> (Stacks, Steps) {
	let (raw_stacks, raw_steps) = raw.split_once("\n\n").unwrap();

	let mut raw_stacks_lines = raw_stacks.lines().rev();
	let raw_stacks_length = raw_stacks_lines.next().unwrap();
	let stacks_length = raw_stacks_length
		.split_whitespace()
		.collect::<Vec<_>>()
		.len();

	let mut stacks = HashMap::new();

	raw_stacks_lines.for_each(|l| {
		let mut chunks = l.chars().array_chunks::<4>();
		let mut i = 1;
		while let Some(cs) = chunks.next() {
			let c = cs.get(1).unwrap();
			if *c != ' ' {
				let exist = stacks.get_mut(&i);
				match exist {
					None => {
						stacks.insert(i, vec![*c]);
					}
					Some(s) => {
						s.push(*c);
					}
				};
			}
			i += 1;
		}

		let last = chunks.into_remainder().unwrap().nth(1).unwrap();
		if last != ' ' {
			let exist = stacks.get_mut(&stacks_length);
			match exist {
				None => {
					stacks.insert(stacks_length, vec![last]);
				}
				Some(s) => {
					s.push(last);
				}
			};
		}
	});

	let steps = raw_steps
		.lines()
		.map(|l| {
			let mut step_split = l.split_whitespace();
			let amount = step_split.nth(1).unwrap().parse::<usize>().unwrap();
			let source = step_split.nth(1).unwrap().parse::<usize>().unwrap();
			let target = step_split.nth(1).unwrap().parse::<usize>().unwrap();
			Step {
				amount,
				source,
				target,
			}
		})
		.collect::<Vec<_>>();

	(stacks, steps)
}

fn part01(input: &(Stacks, Steps)) -> String {
	let mut stacks = input.0.clone();
	let steps = &input.1;

	steps.iter().for_each(|step| {
		for _ in 0..step.amount {
			let [source, target] = stacks.get_many_mut([&step.source, &step.target]).unwrap();
			let char = source.pop().unwrap();
			target.push(char);
		}
	});

	let top_stacks = (1..(stacks.len() + 1)).map(|i| stacks.get(&i).unwrap().last().unwrap());

	top_stacks.collect::<String>()
}

fn part02(input: &(Stacks, Steps)) -> String {
	let mut stacks = input.0.clone();
	let steps = &input.1;

	steps.iter().for_each(|step| {
		let [source, target] = stacks.get_many_mut([&step.source, &step.target]).unwrap();
		let mut chars = (0..step.amount)
			.map(|_| source.pop().unwrap())
			.collect::<Vec<_>>();
		chars.reverse();
		target.append(&mut chars);
	});

	let top_stacks = (1..(stacks.len() + 1)).map(|i| stacks.get(&i).unwrap().last().unwrap());

	top_stacks.collect::<String>()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_input() {
		let raw = concat!(
			"    [D]    \n",
			"[N] [C]    \n",
			"[Z] [M] [P]\n",
			" 1   2   3 \n",
			"\n",
			"move 1 from 2 to 1\n",
			"move 3 from 1 to 3\n",
			"move 2 from 2 to 1\n",
			"move 1 from 1 to 2\n",
		);

		let (stacks, steps) = parse_input(raw);

		let expected_stacks = HashMap::from([
			(1, vec!['Z', 'N']),
			(2, vec!['M', 'C', 'D']),
			(3, vec!['P']),
		]);

		assert_eq!(stacks, expected_stacks);

		let expected_steps = vec![
			Step {
				amount: 1,
				source: 2,
				target: 1,
			},
			Step {
				amount: 3,
				source: 1,
				target: 3,
			},
			Step {
				amount: 2,
				source: 2,
				target: 1,
			},
			Step {
				amount: 1,
				source: 1,
				target: 2,
			},
		];

		assert_eq!(steps, expected_steps);
	}

	#[test]
	fn test_part01() {
		let raw = concat!(
			"    [D]    \n",
			"[N] [C]    \n",
			"[Z] [M] [P]\n",
			" 1   2   3 \n",
			"\n",
			"move 1 from 2 to 1\n",
			"move 3 from 1 to 3\n",
			"move 2 from 2 to 1\n",
			"move 1 from 1 to 2\n",
		);

		let input = parse_input(raw);
		let result = part01(&input);

		assert_eq!(result, "CMZ");
	}

	#[test]
	fn test_part02() {
		let raw = concat!(
			"    [D]    \n",
			"[N] [C]    \n",
			"[Z] [M] [P]\n",
			" 1   2   3 \n",
			"\n",
			"move 1 from 2 to 1\n",
			"move 3 from 1 to 3\n",
			"move 2 from 2 to 1\n",
			"move 1 from 1 to 2\n",
		);

		let input = parse_input(raw);
		let result = part02(&input);

		assert_eq!(result, "MCD");
	}
}
