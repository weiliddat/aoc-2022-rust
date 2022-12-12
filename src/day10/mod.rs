use std::fs;
use std::io::{stdout, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let input = fs::read_to_string(path).expect("Could not read input.txt");

	let part01_result = part01(&input);
	println!("part01 {:?}", part01_result);

	let part02_result = part02(&input);
	println!("part02{:?}", part02_result);

	let short_pause = || sleep(Duration::from_millis(50));
	let long_pause = || sleep(Duration::from_millis(600));
	let flush = || stdout().flush().unwrap();
	for (i, c) in part02_result.chars().enumerate() {
		if i % 40 == 0 {
			long_pause();
			println!();
			flush();
		}
		short_pause();
		print!("{}", c);
		flush();
	}
	long_pause();
	println!();
	long_pause();
	println!();
	long_pause();
	println!("Done");
	long_pause();
	println!();
}

enum Instruction {
	Noop,
	Addx(isize),
}

fn part01(input: &str) -> isize {
	let mut instructions = vec![];

	input.lines().for_each(|l| match l {
		l if l.starts_with("noop") => {
			instructions.push(Instruction::Noop);
		}
		l if l.starts_with("addx") => {
			let (_i, s) = l.split_once(" ").unwrap();
			let size = s.parse::<isize>().unwrap();
			instructions.push(Instruction::Noop);
			instructions.push(Instruction::Addx(size));
		}
		_ => panic!("Unknown instruction {}", l),
	});

	let mut x = 1_isize;
	let to_check = (20..=220).step_by(40).collect::<Vec<_>>();
	let mut checked = vec![];

	instructions.iter().enumerate().for_each(|(c, i)| {
		let cycle = isize::try_from(c).unwrap() + 1;
		match i {
			Instruction::Noop => {
				if to_check.contains(&cycle) {
					checked.push(cycle * x);
				}
			}
			Instruction::Addx(v) => {
				if to_check.contains(&cycle) {
					checked.push(cycle * x);
				}
				x += v;
			}
		}
	});

	let signal_sum = checked.into_iter().sum();

	signal_sum
}

fn part02(input: &str) -> String {
	let mut instructions = vec![];

	input.lines().for_each(|l| match l {
		l if l.starts_with("noop") => {
			instructions.push(Instruction::Noop);
		}
		l if l.starts_with("addx") => {
			let (_i, s) = l.split_once(" ").unwrap();
			let size = s.parse::<isize>().unwrap();
			instructions.push(Instruction::Noop);
			instructions.push(Instruction::Addx(size));
		}
		_ => panic!("Unknown instruction {}", l),
	});

	let mut x = 1_isize;
	let mut display = vec![];

	instructions.iter().enumerate().for_each(|(c, i)| {
		let draw_pos = isize::try_from(c % 40).unwrap();
		let sprite_pos = get_sprite_pos(&x);
		match i {
			Instruction::Noop => {
				if sprite_pos.contains(&draw_pos) {
					display.push('#');
				} else {
					display.push('.');
				}
			}
			Instruction::Addx(v) => {
				if sprite_pos.contains(&draw_pos) {
					display.push('#');
				} else {
					display.push('.');
				}
				x += v;
			}
		}
	});

	display.iter().collect::<String>()
}

fn get_sprite_pos(x: &isize) -> [isize; 3] {
	[x - 1, x + 0, x + 1]
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
		let expected = concat!(
			"##..##..##..##..##..##..##..##..##..##..",
			"###...###...###...###...###...###...###.",
			"####....####....####....####....####....",
			"#####.....#####.....#####.....#####.....",
			"######......######......######......####",
			"#######.......#######.......#######.....",
		);
		let result = part02(INPUT);
		assert_eq!(&result, expected);
	}
}
