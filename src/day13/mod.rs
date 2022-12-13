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

enum Packet {
	Integer(usize),
	List(Vec<usize>),
}

#[derive(Parser)]
#[grammar = "day13/packet.pest"]
pub struct PacketParser;

fn part01(input: &str) -> usize {
	let result = PacketParser::parse(Rule::file, input)
		.expect("could not parse file")
		.next()
		.unwrap();

	result.into_inner().for_each(|p| {
		p.into_inner().for_each(|l_r| {
			match l_r.as_rule() {
				Rule::left => println!("left {}", l_r.as_str()),
				Rule::right => println!("right {}", l_r.as_str()),
				_ => unreachable!()
			}
		});
	});

	0
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
		let result = part01(INPUT);
		assert_eq!(result, 1);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT);
		assert_eq!(result, 1);
	}
}
