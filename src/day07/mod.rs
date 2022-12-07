use std::collections::HashMap;
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
	let mut lines = input.lines();

	let mut file_list = HashMap::new();
	let mut current_path = String::from("");
	let mut dir_list = vec![String::from("/")];
	let mut du = HashMap::new();

	while let Some(line) = lines.next() {
		if line.starts_with("$ cd /") {
			current_path = String::from("/");
		} else if line.starts_with("$ cd ..") {
			let last_dirname = current_path
				.trim_end_matches('/')
				.split('/')
				.last()
				.unwrap();
			current_path = String::from(
				current_path
					.trim_end_matches("/")
					.trim_end_matches(last_dirname),
			);
		} else if line.starts_with("$ cd") {
			current_path = current_path.to_owned() + &line[5..] + "/";
		} else if line.starts_with("$ ls") {
		} else {
			if line.starts_with("dir") {
				let (_, dir_name) = line.split_once(' ').unwrap();
				let dir_path = current_path.clone() + dir_name;
				dir_list.push(dir_path);
			} else {
				let (file_size, file_name) = line.split_once(' ').unwrap();
				let file_size = file_size.parse::<usize>().unwrap();
				let file_path = current_path.clone() + file_name;
				file_list.insert(file_path, file_size);
			}
		}
	}

	for (file_path, file_size) in file_list {
		dir_list
			.iter()
			.filter(|d| file_path.starts_with(*d))
			.for_each(|d| {
				du.entry(d.as_str())
					.and_modify(|s| *s += file_size)
					.or_insert(file_size);
			});
	}

	du.iter()
		.filter_map(|(_, size)| if *size <= 100000 { Some(size) } else { None })
		.sum()
}

fn part02(input: &str) -> usize {
	let mut lines = input.lines();

	let mut file_list = HashMap::new();
	let mut current_path = String::from("");
	let mut dir_list = vec![String::from("/")];
	let mut du = HashMap::new();

	while let Some(line) = lines.next() {
		if line.starts_with("$ cd /") {
			current_path = String::from("/");
		} else if line.starts_with("$ cd ..") {
			let last_dirname = current_path
				.trim_end_matches('/')
				.split('/')
				.last()
				.unwrap();
			current_path = String::from(
				current_path
					.trim_end_matches("/")
					.trim_end_matches(last_dirname),
			);
		} else if line.starts_with("$ cd") {
			current_path = current_path.to_owned() + &line[5..] + "/";
		} else if line.starts_with("$ ls") {
		} else {
			if line.starts_with("dir") {
				let (_, dir_name) = line.split_once(' ').unwrap();
				let dir_path = current_path.clone() + dir_name;
				dir_list.push(dir_path);
			} else {
				let (file_size, file_name) = line.split_once(' ').unwrap();
				let file_size = file_size.parse::<usize>().unwrap();
				let file_path = current_path.clone() + file_name;
				file_list.insert(file_path, file_size);
			}
		}
	}

	for (file_path, file_size) in file_list {
		dir_list
			.iter()
			.filter(|d| file_path.starts_with(*d))
			.for_each(|d| {
				du.entry(d.as_str())
					.and_modify(|s| *s += file_size)
					.or_insert(file_size);
			});
	}

	let total_size = du.get("/").unwrap();
	let space_left = 70000000 - total_size;
	let space_needed = 30000000 - space_left;

	let mut dirs_to_delete = du
		.iter()
		.filter_map(|(_, size)| {
			if *size >= space_needed {
				Some(size)
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	dirs_to_delete.sort();
	*dirs_to_delete[0]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part01() {
		let input = concat!(
			"$ cd /\n",
			"$ ls\n",
			"dir a\n",
			"14848514 b.txt\n",
			"8504156 c.dat\n",
			"dir d\n",
			"$ cd a\n",
			"$ ls\n",
			"dir e\n",
			"29116 f\n",
			"2557 g\n",
			"62596 h.lst\n",
			"$ cd e\n",
			"$ ls\n",
			"584 i\n",
			"$ cd ..\n",
			"$ cd ..\n",
			"$ cd d\n",
			"$ ls\n",
			"4060174 j\n",
			"8033020 d.log\n",
			"5626152 d.ext\n",
			"7214296 k\n",
		);

		let result = part01(input);

		assert_eq!(result, 95437);
	}

	#[test]
	fn test_part02() {
		let input = concat!(
			"$ cd /\n",
			"$ ls\n",
			"dir a\n",
			"14848514 b.txt\n",
			"8504156 c.dat\n",
			"dir d\n",
			"$ cd a\n",
			"$ ls\n",
			"dir e\n",
			"29116 f\n",
			"2557 g\n",
			"62596 h.lst\n",
			"$ cd e\n",
			"$ ls\n",
			"584 i\n",
			"$ cd ..\n",
			"$ cd ..\n",
			"$ cd d\n",
			"$ ls\n",
			"4060174 j\n",
			"8033020 d.log\n",
			"5626152 d.ext\n",
			"7214296 k\n",
		);

		let result = part02(input);

		assert_eq!(result, 24933642);
	}
}
