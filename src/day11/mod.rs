use std::collections::HashMap;
use std::fs;
use std::path::Path;

use pest::Parser;

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

#[derive(Debug)]
enum Operand {
	Old,
	Num(usize),
}

#[derive(Debug)]
enum Operator {
	Add,
	Mult,
}

#[derive(Debug)]
struct Monkey {
	id: usize,
	items: Vec<usize>,
	operands: Vec<Operand>,
	operator: Operator,
	divisible_by: usize,
	true_throw_to_id: usize,
	false_throw_to_id: usize,
}

#[derive(Parser)]
#[grammar = "day11/monkey.pest"]
pub struct MonkeyParser;

fn part01(input: &str) -> usize {
	let result = MonkeyParser::parse(Rule::file, input)
		.expect("could not parse file")
		.next()
		.unwrap();

	let monkeys = result
		.into_inner()
		.map(|monkey| {
			let mut monkey_id = 0;
			let mut items = vec![];
			let mut operands = vec![];
			let mut operator = Operator::Add;
			let mut divisible_by = 1;
			let mut true_throw_to_id = 0;
			let mut false_throw_to_id = 0;

			match monkey.as_rule() {
				Rule::monkey => {
					for terms in monkey.into_inner() {
						match terms.as_rule() {
							Rule::monkey_id => {
								monkey_id = terms.as_str().parse::<usize>().unwrap();
							}
							Rule::item_list => {
								for item in terms.into_inner() {
									match item.as_rule() {
										Rule::item => {
											let item = item.as_str().parse::<usize>().unwrap();
											items.push(item);
										}
										_ => unreachable!(),
									}
								}
							}
							Rule::op_old => {
								operands.push(Operand::Old);
							}
							Rule::op_number => {
								let num = terms.as_str().parse::<usize>().unwrap();
								operands.push(Operand::Num(num));
							}
							Rule::op => match terms.as_str() {
								"+" => operator = Operator::Add,
								"*" => operator = Operator::Mult,
								_ => unreachable!(),
							},
							Rule::div_number => {
								divisible_by = terms.as_str().parse::<usize>().unwrap();
							}
							Rule::true_throw_to_id => {
								true_throw_to_id = terms.as_str().parse::<usize>().unwrap();
							}
							Rule::false_throw_to_id => {
								false_throw_to_id = terms.as_str().parse::<usize>().unwrap();
							}
							_ => {}
						}
					}
				}
				_ => {}
			}

			let monkey = Monkey {
				id: monkey_id,
				items,
				operands,
				operator,
				divisible_by,
				true_throw_to_id,
				false_throw_to_id,
			};

			monkey
		})
		.collect::<Vec<_>>();

	println!("{:?}", monkeys);

	0
}

fn part02(input: &str) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

	#[test]
	fn test_part01() {
		let result = part01(INPUT);
		assert_eq!(result, 1);
	}

	#[test]
	fn test_part02() {
		let input = concat!();
		let result = part02(input);
		assert_eq!(result, 1);
	}
}
