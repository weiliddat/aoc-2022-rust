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

enum Instruction {
	Noop,
	Addx(isize),
}

fn part01(input: &str) -> isize {
	let instructions = input
		.lines()
		.map(|l| match l {
			l if l.starts_with("noop") => Instruction::Noop,
			l if l.starts_with("addx") => {
				let (_i, s) = l.split_once(" ").unwrap();
				let size = s.parse::<isize>().unwrap();
				Instruction::Addx(size)
			}
			_ => panic!("Unknown instruction {}", l),
		})
		.collect::<Vec<_>>();

	let mut curr_cycle = 0;
	let mut x = 1;

	let to_check = (20..=220).step_by(40).collect::<Vec<_>>();
	let mut checked = vec![];

	instructions.iter().for_each(|i| {
		let mut cycle = || {
			curr_cycle += 1;

			if to_check.contains(&curr_cycle) {
				checked.push(curr_cycle * x);
			}
		};

		match i {
			Instruction::Noop => {
				cycle();
			}
			Instruction::Addx(value) => {
				cycle();
				cycle();
				x += value;
			}
		};
	});

	let signal_sum = checked.into_iter().sum();

	signal_sum
}

fn part02(input: &str) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = concat!(
		"addx 15\n",
		"addx -11\n",
		"addx 6\n",
		"addx -3\n",
		"addx 5\n",
		"addx -1\n",
		"addx -8\n",
		"addx 13\n",
		"addx 4\n",
		"noop\n",
		"addx -1\n",
		"addx 5\n",
		"addx -1\n",
		"addx 5\n",
		"addx -1\n",
		"addx 5\n",
		"addx -1\n",
		"addx 5\n",
		"addx -1\n",
		"addx -35\n",
		"addx 1\n",
		"addx 24\n",
		"addx -19\n",
		"addx 1\n",
		"addx 16\n",
		"addx -11\n",
		"noop\n",
		"noop\n",
		"addx 21\n",
		"addx -15\n",
		"noop\n",
		"noop\n",
		"addx -3\n",
		"addx 9\n",
		"addx 1\n",
		"addx -3\n",
		"addx 8\n",
		"addx 1\n",
		"addx 5\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"addx -36\n",
		"noop\n",
		"addx 1\n",
		"addx 7\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"addx 2\n",
		"addx 6\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"addx 1\n",
		"noop\n",
		"noop\n",
		"addx 7\n",
		"addx 1\n",
		"noop\n",
		"addx -13\n",
		"addx 13\n",
		"addx 7\n",
		"noop\n",
		"addx 1\n",
		"addx -33\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"addx 2\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"addx 8\n",
		"noop\n",
		"addx -1\n",
		"addx 2\n",
		"addx 1\n",
		"noop\n",
		"addx 17\n",
		"addx -9\n",
		"addx 1\n",
		"addx 1\n",
		"addx -3\n",
		"addx 11\n",
		"noop\n",
		"noop\n",
		"addx 1\n",
		"noop\n",
		"addx 1\n",
		"noop\n",
		"noop\n",
		"addx -13\n",
		"addx -19\n",
		"addx 1\n",
		"addx 3\n",
		"addx 26\n",
		"addx -30\n",
		"addx 12\n",
		"addx -1\n",
		"addx 3\n",
		"addx 1\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"addx -9\n",
		"addx 18\n",
		"addx 1\n",
		"addx 2\n",
		"noop\n",
		"noop\n",
		"addx 9\n",
		"noop\n",
		"noop\n",
		"noop\n",
		"addx -1\n",
		"addx 2\n",
		"addx -37\n",
		"addx 1\n",
		"addx 3\n",
		"noop\n",
		"addx 15\n",
		"addx -21\n",
		"addx 22\n",
		"addx -6\n",
		"addx 1\n",
		"noop\n",
		"addx 2\n",
		"addx 1\n",
		"noop\n",
		"addx -10\n",
		"noop\n",
		"noop\n",
		"addx 20\n",
		"addx 1\n",
		"addx 2\n",
		"addx 2\n",
		"addx -6\n",
		"addx -11\n",
		"noop\n",
		"noop\n",
		"noop\n"
	);

	#[test]
	fn test_part01() {
		let result = part01(INPUT);
		assert_eq!(result, 13140);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT);
		assert_eq!(result, 1);
	}
}
