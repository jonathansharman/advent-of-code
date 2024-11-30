use std::collections::VecDeque;

use itertools::Itertools;

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, "TWSGQHNHL");
aoc::test::test_part!(test2, part2, "JNRSCDWPP");

pub fn part1() -> String {
	let mut lines = read_lines("input/05.txt");
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

pub fn part2() -> String {
	let mut lines = read_lines("input/05.txt");
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
		let mut stack = stacks[from - 1]
			.drain(stacks[from - 1].len() - count..)
			.collect();
		stacks[to - 1].append(&mut stack);
	}
	stacks.iter().map(|stack| stack.back().unwrap()).collect()
}
