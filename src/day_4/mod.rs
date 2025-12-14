use std::fs;

fn read_input() -> Vec<Vec<char>> {
	fs::read_to_string("src/day_4/input.txt")
		.unwrap()
		.lines()
		.map(|x| x.chars().collect::<Vec<char>>())
		.collect::<Vec<Vec<char>>>()
}

pub fn solve_1() -> i32 {
	let diagram = read_input();

	let mut result = 0;

	for i in 0..diagram.len() {
		for j in 0..diagram[0].len() {
			if diagram[i][j] == '.' {
				continue;
			}

			let mut neighbor_count = 0;

			if i > 0 && diagram[i - 1][j] == '@' {
				neighbor_count += 1;
			}

			if i < diagram.len() - 1 && diagram[i + 1][j] =='@' {
				neighbor_count += 1;
			}

			if j > 0 && diagram[i][j - 1] == '@' {
				neighbor_count += 1;
			}

			if j < diagram[0].len() - 1 && diagram[i][j + 1] == '@' {
				neighbor_count += 1;
			}

			if i > 0 && j > 0 && diagram[i - 1][j - 1] == '@' {
				neighbor_count += 1;
			}

			if i < diagram.len() - 1 && j < diagram[0].len() - 1 && diagram[i + 1][j + 1] == '@' {
				neighbor_count += 1;
			}

			if i > 0 && j < diagram[0].len() - 1 && diagram[i - 1][j + 1] == '@' {
				neighbor_count += 1;
			}

			if i < diagram.len() - 1 && j > 0 && diagram[i + 1][j - 1] == '@' {
				neighbor_count += 1;
			}

			if neighbor_count < 4 {
				result += 1;
			}
		}
	}

	result
}

pub fn solve_2() -> i32 {
	let mut diagram = read_input();

	let mut result = 0;

	loop {
		let before = result;

		for i in 0..diagram.len() {
			for j in 0..diagram[0].len() {
				if diagram[i][j] == '.' {
					continue;
				}
	
				let mut neighbor_count = 0;
	
				if i > 0 && diagram[i - 1][j] == '@' {
					neighbor_count += 1;
				}
	
				if i < diagram.len() - 1 && diagram[i + 1][j] =='@' {
					neighbor_count += 1;
				}
	
				if j > 0 && diagram[i][j - 1] == '@' {
					neighbor_count += 1;
				}
	
				if j < diagram[0].len() - 1 && diagram[i][j + 1] == '@' {
					neighbor_count += 1;
				}
	
				if i > 0 && j > 0 && diagram[i - 1][j - 1] == '@' {
					neighbor_count += 1;
				}
	
				if i < diagram.len() - 1 && j < diagram[0].len() - 1 && diagram[i + 1][j + 1] == '@' {
					neighbor_count += 1;
				}
	
				if i > 0 && j < diagram[0].len() - 1 && diagram[i - 1][j + 1] == '@' {
					neighbor_count += 1;
				}
	
				if i < diagram.len() - 1 && j > 0 && diagram[i + 1][j - 1] == '@' {
					neighbor_count += 1;
				}
	
				if neighbor_count < 4 {
					result += 1;
					diagram[i][j] = '.';
				}
			}
		}

		if before == result {
			break;
		}
	}

	result
}