use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 323613);
crate::test::test_part!(test2, part2, 3103006161);

pub fn part1() -> i64 {
	read_lines("input/2021/10.txt")
		.map(|line| score1(&line))
		.sum()
}

pub fn part2() -> i64 {
	let scores: Vec<i64> = read_lines("input/2021/10.txt")
		.map(|line| score2(&line))
		.filter(|&score| score != 0)
		.sorted()
		.collect();
	scores[scores.len() / 2]
}

fn score1(input: &str) -> i64 {
	let mut stack = Vec::new();
	for c in input.chars() {
		match c {
			'(' | '[' | '{' | '<' => stack.push(c),
			')' => match stack.pop() {
				Some('(') => (),
				_ => return 3,
			},
			']' => match stack.pop() {
				Some('[') => (),
				_ => return 57,
			},
			'}' => match stack.pop() {
				Some('{') => (),
				_ => return 1197,
			},
			'>' => match stack.pop() {
				Some('<') => (),
				_ => return 25137,
			},
			_ => return 0,
		}
	}
	0
}

fn score2(input: &str) -> i64 {
	let mut stack = Vec::new();
	for c in input.chars() {
		match c {
			'(' | '[' | '{' | '<' => stack.push(c),
			')' => match stack.pop() {
				Some('(') => (),
				_ => return 0,
			},
			']' => match stack.pop() {
				Some('[') => (),
				_ => return 0,
			},
			'}' => match stack.pop() {
				Some('{') => (),
				_ => return 0,
			},
			'>' => match stack.pop() {
				Some('<') => (),
				_ => return 0,
			},
			_ => return 0,
		}
	}
	stack.iter().rev().fold(0, |acc, c| {
		5 * acc
			+ match c {
				'(' => 1,
				'[' => 2,
				'{' => 3,
				'<' => 4,
				_ => panic!("I didn't put that there"),
			}
	})
}
