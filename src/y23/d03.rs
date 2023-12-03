use crate::io::read_lines;

crate::test::test_part!(test1, part1, 543867);
crate::test::test_part!(test2, part2, 79613331);

fn neighbor_is_symbol(lines: &[Vec<char>], i: usize, j: usize) -> bool {
	for (ii, line) in lines.iter().skip(i - 1).take(3).enumerate() {
		for (jj, c) in line.iter().skip(j - 1).take(3).enumerate() {
			if ii == i && jj == j {
				continue;
			}
			if !c.is_ascii_digit() && *c != '.' {
				return true;
			}
		}
	}
	false
}

pub fn part1() -> usize {
	let mut lines = read_lines("input/2023/03.txt")
		.map(|line| {
			let mut line = line.chars().collect::<Vec<_>>();
			line.insert(0, '.');
			line.push('.');
			line
		})
		.collect::<Vec<_>>();
	lines.insert(0, vec!['0'; lines[0].len()]);
	lines.push(vec!['0'; lines[0].len()]);
	let mut sum = 0;
	for i in 1..lines.len() - 1 {
		let mut n = 0;
		let mut adj = false;
		for j in 1..lines[i].len() - 1 {
			match lines[i][j] {
				d @ '0'..='9' => {
					adj = adj || neighbor_is_symbol(&lines, i, j);
					n = 10 * n + (d as usize - '0' as usize);
				}
				_ => {
					if adj {
						sum += n;
					}
					adj = false;
					n = 0;
				}
			};
		}
		if adj {
			sum += n;
		}
	}
	sum
}

fn get_n(line: &[char], mut i: usize) -> Option<u32> {
	if !line[i].is_ascii_digit() {
		return None;
	}
	while line[i - 1].is_ascii_digit() {
		i -= 1;
	}
	line.iter()
		.skip(i)
		.map_while(|b| b.to_digit(10))
		.reduce(|acc, next| 10 * acc + next)
}

pub fn part2() -> u32 {
	let mut lines = read_lines("input/2023/03.txt")
		.map(|line| {
			let mut line = line.chars().collect::<Vec<_>>();
			line.insert(0, '.');
			line.push('.');
			line
		})
		.collect::<Vec<_>>();
	lines.insert(0, vec!['0'; lines[0].len()]);
	lines.push(vec!['0'; lines[0].len()]);
	let mut sum = 0;
	for (i, line) in lines.iter().enumerate() {
		for (j, c) in line.iter().enumerate() {
			if *c != '*' {
				continue;
			}
			let mut neighbors = Vec::new();
			if let Some(top) = get_n(&lines[i - 1], j) {
				neighbors.push(top);
			} else {
				if let Some(top_left) = get_n(&lines[i - 1], j - 1) {
					neighbors.push(top_left);
				}
				if let Some(top_right) = get_n(&lines[i - 1], j + 1) {
					neighbors.push(top_right);
				}
			}
			if let Some(left) = get_n(line, j - 1) {
				neighbors.push(left);
			}
			if let Some(right) = get_n(line, j + 1) {
				neighbors.push(right);
			}
			if let Some(bottom) = get_n(&lines[i + 1], j) {
				neighbors.push(bottom);
			} else {
				if let Some(bottom_left) = get_n(&lines[i + 1], j - 1) {
					neighbors.push(bottom_left);
				}
				if let Some(bottom_right) = get_n(&lines[i + 1], j + 1) {
					neighbors.push(bottom_right);
				}
			}
			if neighbors.len() == 2 {
				sum += neighbors.iter().product::<u32>();
			}
		}
	}
	sum
}
