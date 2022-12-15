use std::collections::VecDeque;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, "TWSGQHNHL");
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> String {
	let mut lines = read_lines("input/2022/05.txt");
	let mut stacks: [_; 9] = std::array::from_fn(|_| VecDeque::new());
	loop {
		let line = lines.next().unwrap();
		if line.starts_with(" 1") {
			lines.next();
			break;
		}
		for (idx, c) in line.chars().skip(1).step_by(4).enumerate() {
			if c != ' ' {
				stacks[idx].push_front(c);
			}
		}
	}
	for line in lines {
		let (count, from, to) = line
			.split_whitespace()
			.skip(1)
			.step_by(2)
			.map(|n| n.parse::<usize>().unwrap())
			.collect_tuple()
			.unwrap();
		for _ in 0..count {
			let c = stacks[from - 1].pop_back().unwrap();
			stacks[to - 1].push_back(c);
		}
	}
	stacks.iter().map(|stack| stack.back().unwrap()).collect()
}

pub fn part2() -> usize {
	0
}
