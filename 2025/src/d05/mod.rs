use std::ops::RangeInclusive;

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 643);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut lines = input!().lines();
	let fresh_ranges: Vec<RangeInclusive<usize>> = lines
		.by_ref()
		.map_while(|line| {
			(!line.is_empty()).then(|| {
				let (a, b) = line
					.split('-')
					.map(|n| n.parse().unwrap())
					.collect_tuple()
					.unwrap();
				a..=b
			})
		})
		.collect();
	lines
		.filter(|&ingredient| {
			fresh_ranges
				.iter()
				.any(|range| range.contains(&ingredient.parse().unwrap()))
		})
		.count()
}

pub fn part2() -> usize {
	0
}
