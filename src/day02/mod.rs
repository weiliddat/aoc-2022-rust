use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, PartialEq)]
enum Outcome {
	Win,
	Draw,
	Loss,
}

#[derive(Debug, PartialEq)]
enum Hand {
	Rock,
	Paper,
	Scissors,
}

impl FromStr for Hand {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" => Ok(Hand::Rock),
			"X" => Ok(Hand::Rock),

			"B" => Ok(Hand::Paper),
			"Y" => Ok(Hand::Paper),

			"C" => Ok(Hand::Scissors),
			"Z" => Ok(Hand::Scissors),

			_ => panic!("Could not parse {}", s),
		}
	}
}

impl FromStr for Outcome {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" => Ok(Outcome::Loss),
			"Y" => Ok(Outcome::Draw),
			"Z" => Ok(Outcome::Win),
			_ => panic!("Could not parse {}", s),
		}
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self {
			Hand::Rock => match other {
				Hand::Rock => Some(Ordering::Equal),
				Hand::Paper => Some(Ordering::Less),
				Hand::Scissors => Some(Ordering::Greater),
			},
			Hand::Paper => match other {
				Hand::Rock => Some(Ordering::Greater),
				Hand::Paper => Some(Ordering::Equal),
				Hand::Scissors => Some(Ordering::Less),
			},
			Hand::Scissors => match other {
				Hand::Rock => Some(Ordering::Less),
				Hand::Paper => Some(Ordering::Greater),
				Hand::Scissors => Some(Ordering::Equal),
			},
		}
	}
}

#[derive(Debug, PartialEq)]
struct Round {
	opponent: Hand,
	player: Option<Hand>,
	outcome: Option<Outcome>,
}

pub fn run() {
	let module_name = module_path!().split("::").last().unwrap();
	let input_path = format!("src/{module_name}/input.txt");
	let path = Path::new(&input_path);
	let raw = fs::read_to_string(path).expect("Could not read input.txt");
	let input = raw;

	let part01_result = part01(&input);

	println!("part01 {:?}", part01_result);

	let part02_result = part02(&input);

	println!("part02 {:?}", part02_result);
}

fn part01(input: &str) -> usize {
	let rounds: Vec<Round> = input
		.lines()
		.map(|l| {
			let mut s = l.split(" ");
			let round = Round {
				opponent: Hand::from_str(s.next().unwrap()).unwrap(),
				player: Some(Hand::from_str(s.next().unwrap()).unwrap()),
				outcome: None,
			};
			round
		})
		.collect::<Vec<_>>();

	rounds
		.iter()
		.map(|r| {
			let player_hand = match &r.player {
				Some(hand) => hand,
				None => panic!(),
			};

			let shape_score = match player_hand {
				Hand::Rock => 1,
				Hand::Paper => 2,
				Hand::Scissors => 3,
			};

			let round_score: usize;

			if player_hand > &r.opponent {
				round_score = 6;
			} else if player_hand == &r.opponent {
				round_score = 3;
			} else {
				round_score = 0;
			}

			shape_score + round_score
		})
		.sum()
}

fn part02(input: &str) -> usize {
	let rounds: Vec<Round> = input
		.lines()
		.map(|l| {
			let mut s = l.split(" ");
			let round = Round {
				opponent: Hand::from_str(s.next().unwrap()).unwrap(),
				player: None,
				outcome: Some(Outcome::from_str(s.next().unwrap()).unwrap()),
			};
			round
		})
		.collect::<Vec<_>>();

	rounds
		.iter()
		.map(|r| {
			let outcome = match &r.outcome {
				Some(outcome) => outcome,
				None => panic!(),
			};

			let round_score = match outcome {
				Outcome::Loss => 0,
				Outcome::Draw => 3,
				Outcome::Win => 6,
			};

			let player = match &r.opponent {
				Hand::Rock => match outcome {
					Outcome::Win => Hand::Paper,
					Outcome::Draw => Hand::Rock,
					Outcome::Loss => Hand::Scissors,
				},
				Hand::Paper => match outcome {
					Outcome::Win => Hand::Scissors,
					Outcome::Draw => Hand::Paper,
					Outcome::Loss => Hand::Rock,
				},
				Hand::Scissors => match outcome {
					Outcome::Win => Hand::Rock,
					Outcome::Draw => Hand::Scissors,
					Outcome::Loss => Hand::Paper,
				},
			};

			let shape_score = match player {
				Hand::Rock => 1,
				Hand::Paper => 2,
				Hand::Scissors => 3,
			};

			shape_score + round_score
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let input = concat!("A Y\n", "B X\n", "C Z\n");
		let result = part01(input);
		assert_eq!(result, 15);
	}

	#[test]
	fn test_part02() {
		let input = concat!("A Y\n", "B X\n", "C Z\n");
		let result = part02(input);
		assert_eq!(result, 12);
	}
}
