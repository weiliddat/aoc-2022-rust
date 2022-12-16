use pest::iterators::Pair;
use pest::Parser;
use std::collections::VecDeque;
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

#[derive(Parser)]
#[grammar = "day13/packet.pest"]
pub struct PacketParser;

#[derive(Debug, Clone)]
enum Packet {
	Int(usize),
	List(VecDeque<Packet>),
}

impl PartialEq for Packet {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Packet::Int(s), Packet::Int(o)) => s.cmp(o).is_eq(),
			(Packet::List(s), Packet::List(o)) => s.cmp(o).is_eq(),
			(Packet::Int(s), Packet::List(o)) => VecDeque::from([Packet::Int(*s)]).cmp(o).is_eq(),
			(Packet::List(s), Packet::Int(o)) => s.cmp(&VecDeque::from([Packet::Int(*o)])).is_eq(),
		}
	}
}
impl Eq for Packet {}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(Packet::Int(s), Packet::Int(o)) => s.cmp(o),
			(Packet::List(s), Packet::List(o)) => s.cmp(o),
			(Packet::Int(s), Packet::List(o)) => VecDeque::from([Packet::Int(*s)]).cmp(o),
			(Packet::List(s), Packet::Int(o)) => s.cmp(&VecDeque::from([Packet::Int(*o)])),
		}
	}
}

fn part01(input: &str) -> usize {
	let file = PacketParser::parse(Rule::file, input)
		.expect("could not parse file")
		.next()
		.unwrap();

	file.into_inner()
		.enumerate()
		.filter_map(|(i, p)| {
			let pair_index = i + 1;
			let parsed_lr = match p.as_rule() {
				Rule::pair => {
					let mut l_r = p.into_inner();

					// left and right are always present
					let left = l_r.next().expect("Missing left");
					let right = l_r.next().expect("Missing right");

					// first list is always present and always the only item
					let lpd = parse_item(left.into_inner().next().expect("Missing left items"));
					let rpd = parse_item(right.into_inner().next().expect("Missing right items"));

					Some((lpd, rpd))
				}
				Rule::EOI => None,
				_ => unreachable!(),
			};

			let mut in_right_order: Option<usize> = None;

			if let Some((lpd, rpd)) = parsed_lr {
				let result = lpd.cmp(&rpd);
				if result.is_lt() {
					in_right_order = Some(pair_index);
				}
			}

			in_right_order
		})
		.sum()
}

fn parse_item(item: Pair<Rule>) -> Packet {
	match item.as_rule() {
		Rule::int => {
			let int = item.as_str().parse::<usize>().unwrap();
			Packet::Int(int)
		}
		Rule::list => {
			let list = item;
			let pdl = list
				.into_inner()
				.flat_map(|items| items.into_inner().map(|item| parse_item(item)))
				.collect::<VecDeque<_>>();

			Packet::List(pdl)
		}
		_ => unreachable!(),
	}
}

fn parse_list(input: &str) -> Packet {
	let list = PacketParser::parse(Rule::list, input)
		.expect("could not parse list")
		.next()
		.unwrap();

	parse_item(list)
}

fn part02(input: &str) -> usize {
	let file = PacketParser::parse(Rule::file, input)
		.expect("could not parse file")
		.next()
		.unwrap();

	let mut list = file
		.into_inner()
		.enumerate()
		.filter_map(|(_i, p)| match p.as_rule() {
			Rule::pair => {
				let mut l_r = p.into_inner();

				// left and right are always present
				let left = l_r.next().expect("Missing left");
				let right = l_r.next().expect("Missing right");

				// first list is always present and always the only item
				let lpd = parse_item(left.into_inner().next().expect("Missing left items"));
				let rpd = parse_item(right.into_inner().next().expect("Missing right items"));

				Some([lpd, rpd])
			}
			Rule::EOI => None,
			_ => unreachable!(),
		})
		.flatten()
		.collect::<Vec<_>>();

	let divider_2 = parse_list("[[2]]");
	let divider_6 = parse_list("[[6]]");
	list.push(divider_2.clone());
	list.push(divider_6.clone());

	list.sort();

	let decoder_key: usize = list
		.iter()
		.enumerate()
		.filter_map(|(i, p)| {
			if p == &divider_2 || p == &divider_6 {
				Some(i + 1)
			} else {
				None
			}
		})
		.product();

	decoder_key
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::cmp::Ordering;

	const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

	#[test]
	fn test_parse() {
		let input = parse_list("[1,2,3,[4]]");
		let expected = Packet::List(VecDeque::from([
			Packet::Int(1),
			Packet::Int(2),
			Packet::Int(3),
			Packet::List(VecDeque::from([Packet::Int(4)])),
		]));
		assert_eq!(input, expected);
	}

	#[test]
	fn test_packet_data_comparisons() {
		let left = parse_list("[1,2,1]");
		let right = parse_list("[1,2,1]");
		let result = left.cmp(&right);
		assert_eq!(result, Ordering::Equal);

		let left = parse_list("[1,2,2]");
		let right = parse_list("[1,2,1]");
		let result = left.cmp(&right);
		assert_eq!(result, Ordering::Greater);

		let left = parse_list("[1,2]");
		let right = parse_list("[1,2,1]");
		let result = left.cmp(&right);
		assert_eq!(result, Ordering::Less);

		let left = parse_list("[1,2]");
		let right = parse_list("[1]");
		let result = left.cmp(&right);
		assert_eq!(result, Ordering::Greater);

		let left = parse_list("[2,1]");
		let right = parse_list("[1,2,2]");
		let result = left.cmp(&right);
		assert_eq!(result, Ordering::Greater);

		let left = parse_list("[[1]]");
		let right = parse_list("[1]");
		let result = left.cmp(&right);
		assert_eq!(result, Ordering::Equal);

		let left = parse_list("[[2]]");
		let right = parse_list("[1]");
		let result = left.cmp(&right);
		assert_eq!(result, Ordering::Greater);

		let left = parse_list("[[2]]");
		let right = parse_list("[[3]]");
		let result = left.cmp(&right);
		assert_eq!(result, Ordering::Less);
	}

	#[test]
	fn test_part01() {
		let result = part01(INPUT);
		assert_eq!(result, 13);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT);
		assert_eq!(result, 140);
	}
}
