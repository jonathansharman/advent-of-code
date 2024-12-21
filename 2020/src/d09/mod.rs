use aoc::input::parse_lines;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 1309761972);
aoc::test::test_part!(test2, part2, 177989832);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> u64 {
	find_invalid(&parse_lines(INPUT).collect_vec())
}

pub fn part2() -> u64 {
	let numbers: Vec<u64> = parse_lines(INPUT).collect_vec();
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
