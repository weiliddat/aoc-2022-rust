use std::collections::HashMap;
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
	println!("part02 {:?}", part02_result);
}

fn part01(input: &str) -> usize {
	let mut map = parse_map(input);

	let (_, (_, max_y)) = get_map_min_max(&map);
	let check_oob = |y: &usize| *y > max_y;

	let sand_source = (500_usize, 0_usize);
	let mut sand_went_oob = false;
	let mut sand_at_rest = 0_usize;

	while !sand_went_oob {
		let mut at_rest = false;
		let (mut x, mut y) = sand_source.clone();

		while !at_rest && !check_oob(&y) {
			let down = (x, y + 1);
			let down_left = (x - 1, y + 1);
			let down_right = (x + 1, y + 1);
			if map.get(&down) == None {
				(x, y) = down;
			} else if map.get(&down_left) == None {
				(x, y) = down_left;
			} else if map.get(&down_right) == None {
				(x, y) = down_right;
			} else {
				at_rest = true;
			}
		}

		if at_rest {
			map.insert((x, y), 'o');
			sand_at_rest += 1;
		} else {
			sand_went_oob = true;
		}

		// print_map_fullscreen(&map);
	}

	sand_at_rest
}

fn part02(input: &str) -> usize {
	let mut map = parse_map(input);

	let (_, (_, max_y)) = get_map_min_max(&map);
	let map_floor = max_y + 2;

	let sand_source = (500_usize, 0_usize);
	let mut sand_at_rest = 0_usize;

	loop {
		let mut at_rest = false;
		let (mut x, mut y) = sand_source.clone();

		while !at_rest {
			let down = (x, y + 1);
			let down_left = (x - 1, y + 1);
			let down_right = (x + 1, y + 1);
			if map.get(&down) == None && y != map_floor - 1 {
				(x, y) = down;
			} else if map.get(&down_left) == None && y != map_floor - 1 {
				(x, y) = down_left;
			} else if map.get(&down_right) == None && y != map_floor - 1 {
				(x, y) = down_right;
			} else {
				at_rest = true;
			}
		}

		if at_rest {
			map.insert((x, y), 'o');
			sand_at_rest += 1;
		}

		// print_map_fullscreen(&map);

		let sand_source_blocked = map.get(&sand_source) != None;
		if sand_source_blocked {
			break;
		}
	}

	sand_at_rest
}

fn parse_map(input: &str) -> HashMap<(usize, usize), char> {
	let mut map = HashMap::new();
	let scan_input = input
		.lines()
		.map(|l| {
			l.split(" -> ")
				.map(|c| {
					let (s1, s2) = c.split_once(',').expect("could not parse s1 and s2");

					(
						s1.parse::<usize>().expect("could not parse s1"),
						s2.parse::<usize>().expect("could not parse s2"),
					)
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	scan_input.iter().for_each(|path| {
		path.windows(2).for_each(|from_to| {
			let from = from_to.get(0).expect("could not get from");
			let to = from_to.get(1).expect("could not get to");

			let x_range = if from.0 <= to.0 {
				from.0..=to.0
			} else {
				to.0..=from.0
			};

			let y_range = if from.1 <= to.1 {
				from.1..=to.1
			} else {
				to.1..=from.1
			};

			for x in x_range {
				for y in y_range.clone() {
					map.insert((x, y), '#');
				}
			}
		});
	});

	map
}

fn pretty_print_map(map: &HashMap<(usize, usize), char>, c: usize) {
	let ((min_x, max_x), (_, max_y)) = get_map_min_max(map);

	for y in 0..=(max_y + c) {
		for x in (min_x - c)..=(max_x + c) {
			let c = map.get(&(x, y));
			if let Some(c) = c {
				print!("{}", c);
			} else {
				print!("{}", '.');
			}
		}
		println!();
	}
}

fn print_map_fullscreen(map: &HashMap<(usize, usize), char>) {
	print!("{esc}c", esc = 27 as char);
	stdout().flush().unwrap();
	sleep(Duration::from_secs_f64(1f64 / 60f64));
	pretty_print_map(map, 2);
	stdout().flush().unwrap();
	sleep(Duration::from_secs_f64(3f64 / 60f64));
}

fn get_map_min_max(map: &HashMap<(usize, usize), char>) -> ((usize, usize), (usize, usize)) {
	let (xs, ys): (Vec<_>, Vec<_>) = map.keys().copied().unzip();
	let min_x = xs.iter().min().expect("could not get min/max");
	let max_x = xs.iter().max().expect("could not get min/max");
	let min_y = ys.iter().min().expect("could not get min/max");
	let max_y = ys.iter().max().expect("could not get min/max");

	((*min_x, *max_x), (*min_y, *max_y))
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

	#[test]
	fn test_part01() {
		let result = part01(INPUT);
		assert_eq!(result, 24);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT);
		assert_eq!(result, 93);
	}
}
