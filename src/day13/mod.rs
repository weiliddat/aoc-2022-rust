use pest::iterators::Pair;
use pest::Parser;
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

#[derive(Debug)]
enum PacketData {
	Int(usize),
	List(Vec<PacketData>),
}

fn part01(input: &str) -> usize {
	let file = PacketParser::parse(Rule::file, input)
		.expect("could not parse file")
		.next()
		.unwrap();

	file.into_inner()
		.enumerate()
		.filter_map(|(i, p)| {
			let parsed_lr = match p.as_rule() {
				Rule::pair => {
					let mut l_r = p.into_inner();

					// left and right are always present
					let left = l_r.next().unwrap();
					let right = l_r.next().unwrap();

					// first list is always present and always the only item
					let lpd = parse_item(left.into_inner().next().unwrap());
					let rpd = parse_item(right.into_inner().next().unwrap());

					Some((lpd, rpd))
				}
				Rule::EOI => None,
				_ => unreachable!(),
			};

			if let Some((lpd, rpd)) = parsed_lr {
			}

			Some(i)
		})
		.sum()
}

fn parse_item(item: Pair<Rule>) -> PacketData {
	match item.as_rule() {
		Rule::int => {
			let int = item.as_str().parse::<usize>().unwrap();
			PacketData::Int(int)
		}
		Rule::list => {
			let list = item;
			let pdl = list
				.into_inner()
				.flat_map(|items| items.into_inner().map(|item| parse_item(item)))
				.collect::<Vec<_>>();

			PacketData::List(pdl)
		}
		_ => unreachable!(),
	}
}

fn part02(input: &str) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

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
	fn test_part01() {
		let left = PacketData::List(vec![
			PacketData::List(vec![PacketData::Int(1)]),
			PacketData::List(vec![
				PacketData::Int(2),
				PacketData::Int(3),
				PacketData::Int(4),
			]),
		]);
		let result = part01(INPUT);
		assert_eq!(result, 13);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT);
		assert_eq!(result, 1);
	}
}
