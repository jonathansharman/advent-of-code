use itertools::Itertools;

use crate::io::parse_lines;

crate::test::test_part!(test1, part1, 1309761972);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> i64 {
	let numbers: Vec<i64> = parse_lines("input/2020/09.txt").collect();
	let window_size = 25;
	for (k, &n) in numbers.iter().enumerate().skip(window_size) {
		if numbers[k - window_size..k]
			.iter()
			.tuple_combinations()
			.all(|(a, b)| a + b != n)
		{
			return n;
		}
	}
	unreachable!()
}

pub fn part2() -> usize {
	0
}
