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
	let rounds = 20;
	let monkeys = parse_input(input);
	let manage_worry = |r: usize| -> usize { r / 3 };
	let monkey_inspections = get_inspections_after_rounds(rounds, monkeys, manage_worry);

	let mut inspections_sorted = monkey_inspections.values().collect::<Vec<_>>();
	inspections_sorted.sort();
	inspections_sorted.reverse();

	let monkey_business = inspections_sorted[0] * inspections_sorted[1];
	monkey_business
}

fn part02(input: &str) -> usize {
	let rounds = 10_000;
	let monkeys = parse_input(input);
	let common_divisible = monkeys
		.iter()
		.map(|m| m.divisible_by)
		.reduce(|acc, d| acc * d)
		.unwrap();
	let manage_worry = |r: usize| -> usize { r % common_divisible };
	let monkey_inspections = get_inspections_after_rounds(rounds, monkeys, manage_worry);

	let mut inspections_sorted = monkey_inspections.values().collect::<Vec<_>>();
	inspections_sorted.sort();
	inspections_sorted.reverse();

	let monkey_business = inspections_sorted[0] * inspections_sorted[1];
	monkey_business
}

fn get_inspections_after_rounds<F: Fn(usize) -> usize>(
	rounds: i32,
	mut monkeys: Vec<Monkey>,
	manage_worry: F,
) -> HashMap<usize, usize> {
	let mut monkey_inspections: HashMap<usize, usize> = HashMap::new();
	for _ in 1..=rounds {
		for mi in 0..monkeys.len() {
			let mut item_changes = HashMap::new();
			let m = monkeys.get(mi).unwrap();
			item_changes.insert(m.id, vec![]);

			m.items.iter().for_each(|i| {
				monkey_inspections
					.entry(mi)
					.and_modify(|i| *i += 1)
					.or_insert(1);

				let first = match m.operands[0] {
					Operand::Old => i.clone(),
					Operand::Num(n) => n.clone(),
				};
				let second = match m.operands[1] {
					Operand::Old => i.clone(),
					Operand::Num(n) => n.clone(),
				};
				let result = match m.operator {
					Operator::Add => first + second,
					Operator::Mult => first * second,
				};
				let result = manage_worry(result);
				if result % m.divisible_by == 0 {
					item_changes
						.entry(m.true_throw_to_id)
						.and_modify(|i| i.push(result))
						.or_insert_with(|| {
							let mut old_items = monkeys[m.true_throw_to_id].items.clone();
							old_items.push(result);
							old_items
						});
				} else {
					item_changes
						.entry(m.false_throw_to_id)
						.and_modify(|i| i.push(result))
						.or_insert_with(|| {
							let mut old_items = monkeys[m.false_throw_to_id].items.clone();
							old_items.push(result);
							old_items
						});
				}
			});

			for (id, items) in item_changes {
				monkeys.get_mut(id).unwrap().items = items;
			}
		}
	}
	monkey_inspections
}

fn parse_input(input: &str) -> Vec<Monkey> {
	let result = MonkeyParser::parse(Rule::file, input)
		.expect("could not parse file")
		.next()
		.unwrap();

	result
		.into_inner()
		.filter_map(|monkey| match monkey.as_rule() {
			Rule::monkey => {
				let mut monkey_id = 0;
				let mut items = vec![];
				let mut operands = vec![];
				let mut operator = Operator::Add;
				let mut divisible_by = 1;
				let mut true_throw_to_id = 0;
				let mut false_throw_to_id = 0;

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

				let monkey = Monkey {
					id: monkey_id,
					items,
					operands,
					operator,
					divisible_by,
					true_throw_to_id,
					false_throw_to_id,
				};

				Some(monkey)
			}
			_ => None,
		})
		.collect::<Vec<_>>()
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
		assert_eq!(result, 10605);
	}

	#[test]
	fn test_part02() {
		let result = part02(INPUT);
		assert_eq!(result, 2713310158);
	}
}
