use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

extern crate test;

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let input = fs::read_to_string(path).expect("Could not read input.txt");

	let part01_result = part01(&input, 2000000);
	println!("part01 {:?}", part01_result);

	let part02_result = part02(&input, 4000000);
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
		let c = (i, row);

		for (s, d) in sensor_ranges.iter() {
			if taxi_dist(&c, s) <= *d && !beacons.contains(&c) {
				covered.insert(c);
				break;
			}
		}

		i += 1;
	}

	println!("row {}", row);
	let mut sorted = covered.iter().collect::<Vec<_>>();
	sorted.sort();
	sorted.iter().for_each(|c| {
		println!("{:?}", c);
	});

	covered.len()
}

fn part02(input: &str, max_xy: isize) -> isize {
	let mut sensor_coverage = input
		.lines()
		.map(parse_line)
		.map(|o| o.unwrap())
		.map(|(s, b)| (s, taxi_dist(&s, &b)))
		.flat_map(|(s, d)| {
			(0..max_xy)
				.filter_map(move |y| {
					let x_diff = d - (s.1 - y).abs();
					if x_diff > 0 {
						Some((y, s.0 - x_diff, s.0 + x_diff))
					} else if x_diff == 0 {
						Some((y, s.0, s.0))
					} else {
						None
					}
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	sensor_coverage.sort();

	let sensor_coverage: HashMap<&isize, Vec<(&isize, &isize)>> =
		sensor_coverage
			.iter()
			.fold(HashMap::new(), |mut acc, (y, min, max)| {
				println!("y {}", y);
				let min = min.clamp(&0, &max_xy);
				let max = max.clamp(&0, &max_xy);

				acc.entry(y)
					.and_modify(|ranges| {
						println!("ranges {:?}", ranges.clone());

						let mut n = (min, max);
						let mut new_ranges = vec![];
						println!("new {:?}", n);

						while let Some(e) = ranges.pop() {
							println!("existing {:?}", e);
							// n |----|
							// e |---------|
							// if new is within existing range
							// update new range to existing range
							// remove existing range
							if n.0 >= e.0 && n.1 <= e.1 {
								println!("n within e");
								n = e;
							} else
							// n |---------|
							// e   |----|
							// if existing range is within new range
							// do nothing
							// remove existing range
							if n.0 <= e.0 && n.1 >= e.1 {
								println!("e within n");
							} else
							// n |---------|
							// e        |----|
							// n |---------|
							// e            |----|
							// if new range max is gte/adjacent existing range min
							// update new range max to existing range max
							if n.0 <= e.0 && &(n.1 + 1) >= e.0 {
								println!("n extend e min");
								n.1 = e.1;
							} else
							// n       |---------|
							// e   |----|
							// n         |---------|
							// e   |----|
							// if new range min is lte/adjacent existing range max
							// update new range min to existing range min
							if n.1 >= e.1 && n.0 <= &(e.1 + 1) {
								println!("n extend e max");
								n.0 = e.0;
							} else {
								// e |----|
								// e                    |----|
								// n        |---------|
								// add existing range
								println!("outside");
								new_ranges.push(e);
							}
						}

						new_ranges.push(n);

						*ranges = new_ranges;
					})
					.or_insert(vec![(min, max)]);
				acc
			});

	{
		let mut a = sensor_coverage.iter().collect::<Vec<_>>();
		a.sort();
		a.iter().for_each(|a| println!("{:?}", a));
	}

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
	use test::Bencher;

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
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

	#[test]
	fn test_part01() {
		let result = part01(INPUT, 10);
		assert_eq!(result, 26);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT, 20);
		assert_eq!(result, 56000011);
	}

	#[bench]
	fn bench_part01(b: &mut Bencher) {
		b.iter(|| part01(INPUT, 10));
	}

	#[bench]
	fn bench_part02(b: &mut Bencher) {
		b.iter(|| part02(INPUT, 20));
	}
}
