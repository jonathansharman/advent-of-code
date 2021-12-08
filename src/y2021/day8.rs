use crate::{io::read_lines, solution::Solution};

pub struct Day8;

impl Solution for Day8 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		8
	}

	fn part1(&self) -> i64 {
		read_lines("input/2021/8.txt")
			.map(|line| -> i64 {
				line.split(" | ")
					.nth(1)
					.unwrap()
					.split_whitespace()
					.map(|word| match word.len() {
						2 | 3 | 4 | 7 => 1,
						_ => 0,
					})
					.sum()
			})
			.sum()
	}

	fn part2(&self) -> i64 {
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(0, Day8.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(0, Day8.part2());
	}
}
