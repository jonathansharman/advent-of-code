use aoc::input::{input, ParseLines};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 731731);
aoc::test::test_part!(test2, part2, 116115990);

pub fn part1() -> i64 {
	solve(2)
}

pub fn part2() -> i64 {
	solve(3)
}

fn solve(k: usize) -> i64 {
	input!()
		.parse_lines()
		.combinations(k)
		.filter_map(|elems| {
			if elems.iter().sum::<i64>() == 2020 {
				Some(elems.iter().product())
			} else {
				None
			}
		})
		.next()
		.unwrap()
}
