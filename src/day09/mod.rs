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
	assert_eq!(part01_result, 6464);

	let part02_result = part02(&input);
	println!("part02 {:?}", part02_result);
}

#[derive(Debug, PartialEq, Hash, Eq, Default, Clone)]
struct Coord {
	x: isize,
	y: isize,
}

#[derive(Debug)]
enum Move {
	U(isize),
	D(isize),
	L(isize),
	R(isize),
}

fn part01(input: &str) -> usize {
	let start = Coord::default();
	let mut head = start.clone();
	let mut tail = start.clone();
	let mut tail_visited: HashSet<Coord> = HashSet::new();
	tail_visited.insert(tail.clone());

	let instructions = input.lines().map(|l| {
		let (i, s) = l.split_once(' ').unwrap();
		let size = s.parse().unwrap();
		match i {
			"U" => Move::U(size),
			"D" => Move::D(size),
			"L" => Move::L(size),
			"R" => Move::R(size),
			_ => panic!("Unexpected char"),
		}
	});

	instructions.for_each(|i| {
		match i {
			Move::U(s) => head.y += s,
			Move::D(s) => head.y -= s,
			Move::L(s) => head.x -= s,
			Move::R(s) => head.x += s,
		}

		while is_far(&head, &tail) {
			if tail.x != head.x && tail.y != head.y {
				// diagonal
				if tail.x < head.x {
					tail.x += 1;
				} else {
					tail.x -= 1;
				}

				if tail.y < head.y {
					tail.y += 1;
				} else {
					tail.y -= 1;
				}
			} else if tail.x != head.x {
				// x-axis
				if tail.x < head.x {
					tail.x += 1;
				} else {
					tail.x -= 1;
				}
			} else if tail.y != head.y {
				// y-axis
				if tail.y < head.y {
					tail.y += 1;
				} else {
					tail.y -= 1;
				}
			}

			tail_visited.insert(tail.clone());
		}
	});

	tail_visited.len()
}

fn is_far(head: &Coord, tail: &Coord) -> bool {
	let x_diff = head.x - tail.x;
	let y_diff = head.y - tail.y;
	x_diff.abs() > 1 || y_diff.abs() > 1
}

fn part02(input: &str) -> usize {
	let start = Coord::default();
	let rope_size = 10_usize;
	let mut rope = (0..rope_size).map(|_| start.clone()).collect::<Vec<_>>();
	let mut tail_visited: HashSet<Coord> = HashSet::new();
	tail_visited.insert(start.clone());

	let instructions = input.lines().map(|l| {
		let (i, s) = l.split_once(' ').unwrap();
		let size = s.parse().unwrap();
		match i {
			"U" => Move::U(size),
			"D" => Move::D(size),
			"L" => Move::L(size),
			"R" => Move::R(size),
			_ => panic!("Unexpected char"),
		}
	});

	instructions.for_each(|i| {
		let head = rope.get_mut(0).unwrap();

		match i {
			Move::U(s) => head.y += s,
			Move::D(s) => head.y -= s,
			Move::L(s) => head.x -= s,
			Move::R(s) => head.x += s,
		}

		println!("head {:?}", head);

		for hi in 0..(rope_size - 1) {
			let ti = hi + 1;
			let [head, tail] = rope.get_many_mut([hi, ti]).unwrap();

			while is_far(&head, &tail) {
				if tail.x != head.x && tail.y != head.y {
					// diagonal
					if tail.x < head.x {
						tail.x += 1;
					} else {
						tail.x -= 1;
					}

					if tail.y < head.y {
						tail.y += 1;
					} else {
						tail.y -= 1;
					}
				} else if tail.x != head.x {
					// x-axis
					if tail.x < head.x {
						tail.x += 1;
					} else {
						tail.x -= 1;
					}
				} else if tail.y != head.y {
					// y-axis
					if tail.y < head.y {
						tail.y += 1;
					} else {
						tail.y -= 1;
					}
				}

				// add visited for only tail tail
				if ti == rope_size - 1 {
					tail_visited.insert(tail.clone());
				}
			}
		}

		println!("rope {:#?}", rope);
	});

	println!("visited {:#?}", tail_visited);

	tail_visited.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_far() {
		let head = Coord { x: 3, y: 3 };
		let tail = Coord { x: 0, y: 1 };
		assert_eq!(true, is_far(&head, &tail));

		let head = Coord { x: 1, y: 2 };
		let tail = Coord { x: 1, y: 1 };
		assert_eq!(false, is_far(&head, &tail));

		let head = Coord { x: 2, y: 2 };
		let tail = Coord { x: 1, y: 1 };
		assert_eq!(false, is_far(&head, &tail));

		let head = Coord { x: 0, y: 0 };
		let tail = Coord { x: 1, y: -1 };
		assert_eq!(false, is_far(&head, &tail));

		let head = Coord { x: -1, y: 0 };
		let tail = Coord { x: -1, y: -1 };
		assert_eq!(false, is_far(&head, &tail));

		let head = Coord { x: 2, y: 2 };
		let tail = Coord { x: 2, y: 2 };
		assert_eq!(false, is_far(&head, &tail));

		let head = Coord { x: 0, y: 4 };
		let tail = Coord { x: 0, y: 0 };
		assert_eq!(true, is_far(&head, &tail));
	}

	#[test]
	fn test_part01() {
		let input =
			concat!("R 4\n", "U 4\n", "L 3\n", "D 1\n", "R 4\n", "D 1\n", "L 5\n", "R 2\n",);
		let result = part01(input);
		assert_eq!(result, 13);
	}

	#[test]
	fn test_part02() {
		let input =
			concat!("R 4\n", "U 4\n", "L 3\n", "D 1\n", "R 4\n", "D 1\n", "L 5\n", "R 2\n",);
		let result = part02(input);
		assert_eq!(result, 1);

		let input =
			concat!("R 5\n", "U 8\n", "L 8\n", "D 3\n", "R 17\n", "D 10\n", "L 25\n", "U 20\n",);
		let result = part02(input);
		assert_eq!(result, 36);
	}
}
