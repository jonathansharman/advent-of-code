use std::collections::HashMap;

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 2066446);
aoc::test::test_part!(test2, part2, 24931009);

fn parse_line(line: &str) -> (u32, u32) {
	line.split_whitespace()
		.map(|n| n.parse().unwrap())
		.next_tuple()
		.unwrap()
}

pub fn part1() -> u32 {
	let (mut a, mut b): (Vec<u32>, Vec<u32>) =
		input!().lines().map(parse_line).unzip();
	a.sort();
	b.sort();
	a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part2() -> u32 {
	let (values, counts): (Vec<u32>, HashMap<u32, u32>) =
		input!().lines().map(parse_line).fold(
			(Vec::new(), HashMap::new()),
			|(mut values, mut counts), (a, b)| {
				values.push(a);
				*counts.entry(b).or_default() += 1;
				(values, counts)
			},
		);
	values
		.into_iter()
		.map(|v| v * *counts.get(&v).unwrap_or(&0))
		.sum()
}
