#![feature(iter_array_chunks)]
#![feature(map_many_mut)]
#![feature(get_many_mut)]
#![feature(test)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

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
		"day07" => day07::run(),
		"day08" => day08::run(),
		"day09" => day09::run(),
		"day10" => day10::run(),
		"day11" => day11::run(),
		"day12" => day12::run(),
		"day13" => day13::run(),
		"day14" => day14::run(),
		"day15" => day15::run(),
		_ => println!("{} was not done yet!", day),
	}
}
