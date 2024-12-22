aoc::test::test_part!(test1, part1, 32723);
aoc::test::test_part!(test2, part2, 34536);


pub fn part1() -> usize {
	let mut pattern = Vec::new();
	let mut result = 0;
	for line in input!().lines() {
		if line.is_empty() {
			result += summary(&pattern);
			pattern.clear();
		} else {
			pattern.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
		}
	}
	result += summary(&pattern);
	result
}

fn summary(pattern: &[Vec<bool>]) -> usize {
	let mut result = 0;
	'col: for col in 1..pattern[0].len() {
		let len = col.min(pattern[0].len() - col);
		for i in 0..len {
			for line in pattern {
				if line[col - i - 1] != line[col + i] {
					continue 'col;
				}
			}
		}
		result += col;
	}
	'row: for row in 1..pattern.len() {
		let len = row.min(pattern.len() - row);
		for i in 0..len {
			if pattern[row - i - 1] != pattern[row + i] {
				continue 'row;
			}
		}
		result += 100 * row;
	}
	result
}

pub fn part2() -> usize {
	let mut pattern = Vec::new();
	let mut result = 0;
	for line in input!().lines() {
		if line.is_empty() {
			result += summary2(&pattern);
			pattern.clear();
		} else {
			pattern.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
		}
	}
	result += summary2(&pattern);
	result
}

fn summary2(pattern: &[Vec<bool>]) -> usize {
	let mut result = 0;
	'col: for col in 1..pattern[0].len() {
		let len = col.min(pattern[0].len() - col);
		let mut unsmudged = false;
		for i in 0..len {
			for line in pattern {
				if line[col - i - 1] != line[col + i] {
					if unsmudged {
						continue 'col;
					} else {
						unsmudged = true;
					}
				}
			}
		}
		if unsmudged {
			result += col;
		}
	}
	'row: for row in 1..pattern.len() {
		let len = row.min(pattern.len() - row);
		let mut unsmudged = false;
		for i in 0..len {
			let (left, right) = (&pattern[row - i - 1], &pattern[row + i]);
			for j in 0..left.len() {
				if left[j] != right[j] {
					if unsmudged {
						continue 'row;
					} else {
						unsmudged = true;
					}
				}
			}
		}
		if unsmudged {
			result += 100 * row;
		}
	}
	result
}
