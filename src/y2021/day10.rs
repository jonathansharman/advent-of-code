use crate::{io::read_lines, solution::Solution};

pub struct Day10;

impl Solution for Day10 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		10
	}

	fn part1(&self) -> i64 {
		read_lines("input/2021/10.txt")
			.map(|line| score(&line))
			.sum()
	}

	fn part2(&self) -> i64 {
		0
	}
}

fn score(input: &str) -> i64 {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(323613, Day10.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(0, Day10.part2());
	}
}
