use crate::io::read_lines;

crate::test::test_part!(test1, part1, 543867);
crate::test::test_part!(test2, part2, ?);

fn neighbor_is_symbol(lines: &[Vec<char>], i: usize, j: usize) -> bool {
	for ii in i - 1..=i + 1 {
		for jj in j - 1..=j + 1 {
			if ii == i && jj == j {
				continue;
			}
			let Some(c) = lines.get(ii).and_then(|line| line.get(jj)) else {
				continue;
			};
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

pub fn part2() -> usize {
	0
}
