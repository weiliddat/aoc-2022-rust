use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let input = fs::read_to_string(path).expect("Could not read input.txt");

	let part01_result = part01(&input);
	println!("part01 {:?}", part01_result);
	assert_eq!(part01_result, 1763);

	let part02_result = part02(&input);
	println!("part02 {:?}", part02_result);
}

fn part01(input: &str) -> usize {
	let map = input
		.lines()
		.map(|l| {
			l.chars()
				.map(|c| c.to_digit(10).unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let size = map.len();
	let map = map.concat();
	let mut visible = HashSet::new();

	// from top
	let top_range = 0..size;
	fn top_iterate(c: usize, s: usize) -> Option<usize> {
		c.checked_add(s)
	}
	visible_from_outside(&map, &mut visible, size, top_range, top_iterate);

	// from left
	let left_range = (0..(size * size)).step_by(size);
	fn left_iterate(c: usize, _s: usize) -> Option<usize> {
		c.checked_add(1)
	}
	visible_from_outside(&map, &mut visible, size, left_range, left_iterate);

	// from right
	let right_range = ((size - 1)..(size * size)).step_by(size);
	fn right_iterate(c: usize, _s: usize) -> Option<usize> {
		c.checked_sub(1)
	}
	visible_from_outside(&map, &mut visible, size, right_range, right_iterate);

	// from bottom
	let bottom_range = (size * size - size)..(size * size);
	fn bottom_iterate(c: usize, s: usize) -> Option<usize> {
		c.checked_sub(s)
	}
	visible_from_outside(&map, &mut visible, size, bottom_range, bottom_iterate);

	visible.len()
}

fn visible_from_outside<I>(
	map: &Vec<u32>,
	visible: &mut HashSet<(usize, u32)>,
	size: usize,
	range: I,
	iterate: fn(usize, usize) -> Option<usize>,
) where
	I: Iterator<Item = usize>,
{
	for i in range {
		let mut curr_height = None;
		let mut cursor = i;

		while let Some(tree_compared) = map.get(cursor) {
			if let Some(height) = curr_height {
				if *tree_compared > height {
					curr_height = Some(*tree_compared);
					visible.insert((cursor, *tree_compared));
				}
			} else {
				curr_height = Some(*tree_compared);
				visible.insert((cursor, *tree_compared));
			}

			if let Some(new_cursor) = iterate(cursor, size) {
				cursor = new_cursor;
			} else {
				break;
			}
		}
	}
}

fn part02(input: &str) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let input = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390\n",);
		let result = part01(input);
		assert_eq!(result, 21);
	}

	#[test]
	fn test_part02() {
		let input = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390\n",);
		let result = part02(input);
		assert_eq!(result, 1);
	}
}
