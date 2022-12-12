use std::collections::{HashMap, HashSet, VecDeque};
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
	let mut map = HashMap::new();
	let mut start = (0, 0);
	let mut end = (0, 0);
	let mut map_neighbors = HashMap::new();

	parse_map(input, &mut start, &mut end, &mut map, &mut map_neighbors);

	search(map_neighbors, start, end)
}

fn search(
	map_neighbors: HashMap<(usize, usize), Vec<(usize, usize)>>,
	start: (usize, usize),
	end: (usize, usize),
) -> usize {
	let mut found = None;
	let mut paths = VecDeque::from([start]);
	let mut visited = HashMap::new();
	visited.insert(start, 0_usize);

	while let Some(p) = paths.pop_front() {
		if p == end {
			found = Some(visited.get(&p).unwrap());
			break;
		}

		let all_possible = &map_neighbors.get(&p).unwrap();

		let unvisited = all_possible
			.iter()
			.filter(|&p| !visited.contains_key(p))
			.collect::<Vec<_>>();

		for new in unvisited {
			paths.push_back(*new);
			visited.insert(*new, *visited.get(&p).unwrap() + 1);
		}
	}

	*found.unwrap()
}

fn parse_map(
	input: &str,
	start: &mut (usize, usize),
	end: &mut (usize, usize),
	map: &mut HashMap<(usize, usize), char>,
	map_neighbors: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
) {
	input.lines().enumerate().for_each(|(y, l)| {
		l.chars().enumerate().for_each(|(x, c)| match c {
			'S' => {
				*start = (x, y);
				map.insert((x, y), 'a');
			}
			'E' => {
				*end = (x, y);
				map.insert((x, y), 'z');
			}
			_ => {
				map.insert((x, y), c);
			}
		});
	});

	// pretty_print_map(&map);

	let (map_max_x, map_max_y) = map.iter().max().unwrap().0;

	map.iter().for_each(|(curr, height)| {
		let neighbors = [
			(Some(curr.0), curr.1.checked_sub(1)),
			(Some(curr.0), curr.1.checked_add(1)),
			(curr.0.checked_sub(1), Some(curr.1)),
			(curr.0.checked_add(1), Some(curr.1)),
		]
		.iter()
		.filter_map(|p| {
			if let (Some(x), Some(y)) = (p.0, p.1) {
				if x <= *map_max_x && y <= *map_max_y {
					let p = (x, y);
					let p_h = map.get(&p).unwrap();

					if *p_h as u32 <= *height as u32 + 1 {
						return Some(p);
					}
				}
			}
			return None;
		})
		.collect::<Vec<_>>();

		map_neighbors.insert(*curr, neighbors);
	});
}

fn pretty_print_map(map: &HashMap<(usize, usize), char>) {
	let (size_x, size_y) = map.keys().max().unwrap();
	for y in 0..=*size_y {
		for x in 0..=*size_x {
			let c = map.get(&(x, y)).unwrap();
			print!("{}", c);
		}
		println!();
	}
}

fn part02(_input: &str) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

	#[test]
	fn test_part01() {
		let result = part01(INPUT);
		assert_eq!(result, 31);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT);
		assert_eq!(result, 29);
	}
}
