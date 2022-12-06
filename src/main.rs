#![feature(iter_array_chunks)]
#![feature(map_many_mut)]
#![feature(test)]

use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
	let args: Vec<String> = env::args().collect();

	let day = if args.len() > 1 {
		&args[args.len() - 1]
	} else {
		panic!("Please provide a day to run!")
	};

	let day = day.as_str();

	println!("Running {}", day);

	match day {
		"day01" => day01::run(),
		"day02" => day02::run(),
		"day03" => day03::run(),
		"day04" => day04::run(),
		"day05" => day05::run(),
		"day06" => day06::run(),
		_ => println!("{} was not done yet!", day),
	}
}
