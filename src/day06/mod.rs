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

fn part01(input: &str) -> usize {
	find_consec_uniq(input, 4)
}

fn part02(input: &str) -> usize {
	find_consec_uniq(input, 14)
}

fn find_consec_uniq(input: &str, uniq_len: usize) -> usize {
	let chars = input.chars().collect::<Vec<_>>();
	let consec_uniq = chars
		.windows(uniq_len)
		.enumerate()
		.find(|(_i, cs)| {
			let mut csc = cs.to_vec();
			csc.sort_unstable();
			csc.dedup();
			csc.len() == cs.len()
		})
		.unwrap();
	consec_uniq.0 + uniq_len
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
		let result = part01(input);
		assert_eq!(result, 7);

		let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
		let result = part01(input);
		assert_eq!(result, 5);

		let input = "nppdvjthqldpwncqszvftbrmjlhg";
		let result = part01(input);
		assert_eq!(result, 6);

		let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
		let result = part01(input);
		assert_eq!(result, 10);

		let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
		let result = part01(input);
		assert_eq!(result, 11);
	}

	#[test]
	fn test_part02() {
		let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
		let result = part02(input);
		assert_eq!(result, 19);

		let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
		let result = part02(input);
		assert_eq!(result, 23);

		let input = "nppdvjthqldpwncqszvftbrmjlhg";
		let result = part02(input);
		assert_eq!(result, 23);

		let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
		let result = part02(input);
		assert_eq!(result, 29);

		let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
		let result = part02(input);
		assert_eq!(result, 26);
	}
}
