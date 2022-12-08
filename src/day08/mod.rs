use std::collections::{HashMap, HashSet};
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
	assert_eq!(part02_result, 671160);
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

type Map = HashMap<(usize, usize), u32>;

fn part02(input: &str) -> usize {
	let mut map: Map = HashMap::new();

	let size = input.lines().next().unwrap().len();

	input.lines().enumerate().for_each(|(y, cs)| {
		cs.chars().enumerate().for_each(|(x, c)| {
			let h = c.to_digit(10).unwrap();
			map.insert((x, y), h);
		});
	});

	let mut scenic_scores = map
		.iter()
		.map(|(coord, _height)| {
			// to top
			let top_list = (0..=coord.1)
				.rev()
				.map(|y| map.get_key_value(&(coord.0, y)).unwrap())
				.collect::<Vec<_>>();

			let visible_top = visible_in_list(&top_list);

			// to right
			let right_list = (coord.0..size)
				.map(|x| map.get_key_value(&(x, coord.1)).unwrap())
				.collect::<Vec<_>>();

			let visible_right = visible_in_list(&right_list);

			// to left
			let left_list = (0..=coord.0)
				.rev()
				.map(|x| map.get_key_value(&(x, coord.1)).unwrap())
				.collect::<Vec<_>>();

			let visible_left = visible_in_list(&left_list);

			// to bottom
			let bottom_list = (coord.1..size)
				.map(|y| map.get_key_value(&(coord.0, y)).unwrap())
				.collect::<Vec<_>>();

			let visible_bottom = visible_in_list(&bottom_list);

			visible_top.len() * visible_left.len() * visible_right.len() * visible_bottom.len()
		})
		.collect::<Vec<_>>();

	scenic_scores.sort();
	let best_scenic_score = scenic_scores.last().unwrap();
	*best_scenic_score
}

fn visible_in_list(list: &Vec<(&(usize, usize), &u32)>) -> Vec<(usize, usize)> {
	let mut visible = vec![];
	let mut iterator = list.iter();
	let (_, house_height) = iterator.next().unwrap();

	for (coord, tree_height) in iterator {
		if tree_height >= house_height {
			visible.push(**coord);
			break;
		} else {
			visible.push(**coord);
		}
	}

	visible
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
		assert_eq!(result, 8);
	}
}
