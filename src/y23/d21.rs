use std::collections::{HashSet, VecDeque};

use crate::{io::read_lines, neighbors};

crate::test::test_part!(test1, part1, 3733);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut start = (0, 0);
	let open = read_lines("input/2023/21.txt")
		.enumerate()
		.map(|(i, line)| {
			line.chars()
				.enumerate()
				.map(|(j, c)| match c {
					'.' => true,
					'#' => false,
					'S' => {
						start = (i, j);
						true
					}
					_ => panic!("invalid character"),
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let mut count = 0;
	let mut queue = VecDeque::from([(start, 0)]);
	let mut visited = HashSet::new();
	while let Some(((i, j), d)) = queue.pop_front() {
		if !visited.insert((i, j)) {
			continue;
		}
		if d % 2 == 0 {
			count += 1;
		}
		for n in neighbors::four(open.len(), open[0].len(), i, j) {
			if open[n.0][n.1] && d < 64 {
				queue.push_back((n, d + 1));
			}
		}
	}
	count
}

pub fn part2() -> usize {
	0
}
