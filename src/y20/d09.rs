use crate::io::parse_lines;
use itertools::Itertools;

crate::test::test_part!(test1, part1, 1309761972);
crate::test::test_part!(test2, part2, 177989832);

pub fn part1() -> u64 {
	find_invalid(&parse_lines("input/2020/09.txt").collect_vec())
}

pub fn part2() -> u64 {
	let numbers: Vec<u64> = parse_lines("input/2020/09.txt").collect_vec();
	let target = find_invalid(&numbers);
	for window_size in 2..numbers.len() {
		for window in numbers.windows(window_size) {
			if window.iter().sum::<u64>() == target {
				return window.iter().min().unwrap()
					+ window.iter().max().unwrap();
			}
		}
	}
	unreachable!()
}

fn find_invalid(numbers: &[u64]) -> u64 {
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
