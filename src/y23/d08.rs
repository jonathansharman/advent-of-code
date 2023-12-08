use std::collections::HashMap;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 19631);
crate::test::test_part!(test2, part2, ?);

#[derive(Clone, Copy)]
enum Direction {
	Left,
	Right,
}

pub fn part1() -> usize {
	let mut lines = read_lines("input/2023/08.txt");
	let directions = lines
		.next()
		.unwrap()
		.chars()
		.map(|c| {
			if c == 'L' {
				Direction::Left
			} else {
				Direction::Right
			}
		})
		.collect::<Vec<_>>();
	let network = HashMap::<String, (String, String)>::from_iter(
		lines.skip(1).map(|line| {
			let (lhs, rhs) = line.split_once(" = ").unwrap();
			let left = &rhs[1..4];
			let right = &rhs[6..9];
			(lhs.to_owned(), (left.to_owned(), right.to_owned()))
		}),
	);
	let mut symbol = "AAA".to_string();
	for i in 0.. {
		if symbol == "ZZZ" {
			return i;
		}
		let direction = directions[i % directions.len()];
		symbol = match direction {
			Direction::Left => network[&symbol].0.clone(),
			Direction::Right => network[&symbol].1.clone(),
		}
	}
	unreachable!()
}

pub fn part2() -> usize {
	0
}
