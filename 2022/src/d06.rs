use std::collections::HashMap;

use itertools::Itertools;

aoc::test::test_part!(test1, part1, 1538);
aoc::test::test_part!(test2, part2, 2315);

const INPUT: &str = include_str!("input/06.txt");

pub fn part1() -> usize {
	for (i, (a, b, c, d)) in INPUT.as_bytes().iter().tuple_windows().enumerate()
	{
		if a != b && a != c && a != d && b != c && b != d && c != d {
			return i + 4;
		}
	}
	panic!()
}

const START_LEN: usize = 14;

pub fn part2() -> usize {
	let mut counts: HashMap<char, usize> =
		INPUT
			.chars()
			.take(START_LEN)
			.fold(HashMap::new(), |mut counts, c| {
				*counts.entry(c).or_default() += 1;
				counts
			});
	for (i, (c_out, c_in)) in
		INPUT.chars().zip(INPUT.chars().skip(START_LEN)).enumerate()
	{
		if counts.len() == START_LEN {
			return i + START_LEN;
		}
		*counts.entry(c_in).or_default() += 1;
		let count_out = counts.entry(c_out).or_default();
		*count_out -= 1;
		if *count_out == 0 {
			counts.remove(&c_out);
		}
	}
	panic!()
}
