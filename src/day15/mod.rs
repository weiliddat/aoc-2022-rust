use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let input = fs::read_to_string(path).expect("Could not read input.txt");

	let part01_result = part01(&input, 2000000);
	println!("part01 {:?}", part01_result);

	let part02_result = part02(&input);
	println!("part02 {:?}", part02_result);
}

fn part01(input: &str, row: isize) -> usize {
	let sensor_beacons = input
		.lines()
		.map(parse_line)
		.map(|o| o.unwrap())
		.collect::<Vec<_>>();

	let beacons = sensor_beacons
		.iter()
		.map(|(_, b)| b)
		.collect::<HashSet<_>>();

	let sensor_ranges = sensor_beacons
		.iter()
		.map(|(s, b)| (s, taxi_dist(&s, &b)))
		.collect::<HashMap<_, _>>();

	let (xs, ys): (Vec<_>, Vec<_>) = sensor_ranges
		.iter()
		.flat_map(|(s, d)| [(s.0 + d, s.1 + d), (s.0 - d, s.1 - d)])
		.unzip();

	let min_x = xs.iter().min().expect("could not get min/max");
	let max_x = xs.iter().max().expect("could not get min/max");
	let _min_y = ys.iter().min().expect("could not get min/max");
	let _max_y = ys.iter().max().expect("could not get min/max");

	let mut i = *min_x;
	let mut covered = HashSet::new();

	while i <= *max_x {
		i += 1;
		let c = (i, row);

		for (s, d) in sensor_ranges.iter() {
			if taxi_dist(&c, s) <= *d && !beacons.contains(&c) {
				covered.insert(c);
			}
		}
	}

	covered.len()
}

fn part02(input: &str) -> usize {
	0
}

fn parse_line(line: &str) -> Option<((isize, isize), (isize, isize))> {
	lazy_static! {
		static ref RE: Regex = Regex::new(
			r"(?i)sensor at x=([\d\-]+), y=([\d\-]+): closest beacon is at x=([\d\-]+), y=([\d\-]+)"
		)
		.unwrap();
	}
	let matches = RE.captures(line)?;
	let sensor_x = matches
		.get(1)
		.and_then(|s| s.as_str().parse::<isize>().ok())?;
	let sensor_y = matches
		.get(2)
		.and_then(|s| s.as_str().parse::<isize>().ok())?;
	let beacon_x = matches
		.get(3)
		.and_then(|s| s.as_str().parse::<isize>().ok())?;
	let beacon_y = matches
		.get(4)
		.and_then(|s| s.as_str().parse::<isize>().ok())?;

	Some(((sensor_x, sensor_y), (beacon_x, beacon_y)))
}

fn taxi_dist(a: &(isize, isize), b: &(isize, isize)) -> isize {
	(a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

	#[test]
	fn test_part01() {
		let result = part01(INPUT, 10);
		assert_eq!(result, 26);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT);
		assert_eq!(result, 1);
	}
}
