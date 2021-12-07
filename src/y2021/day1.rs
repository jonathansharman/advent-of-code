use crate::{io::read_integer_lines, solution::Solution};

use itertools::Itertools;

pub struct Day1;

impl Solution for Day1 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		1
	}

	fn part1(&self) -> i64 {
		read_integer_lines("input/2021/1.txt")
			.tuple_windows()
			.filter(|(prev, next)| prev < next)
			.count() as i64
	}

	fn part2(&self) -> i64 {
		read_integer_lines("input/2021/1.txt")
			.tuple_windows()
			.map(|(first, second, third)| first + second + third)
			.tuple_windows()
			.filter(|(prev, next)| prev < next)
			.count() as i64
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(1162, Day1.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(1190, Day1.part2());
	}
}
