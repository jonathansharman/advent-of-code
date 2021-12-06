use crate::io::read_integers;

use itertools::Itertools;

pub fn part1() -> i64 {
	read_integers("input/2021/1.txt")
		.tuple_windows()
		.filter(|(prev, next)| prev < next)
		.count() as i64
}

pub fn part2() -> i64 {
	read_integers("input/2021/1.txt")
		.tuple_windows()
		.map(|(first, second, third)| first + second + third)
		.tuple_windows()
		.filter(|(prev, next)| prev < next)
		.count() as i64
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(1162, super::part1());
	}

	#[test]
	fn part2() {
		assert_eq!(1190, super::part2());
	}
}
