use aoc::input::parse_lines;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 731731);
aoc::test::test_part!(test2, part2, 116115990);

const INPUT: &str = include_str!("input/01.txt");

pub fn part1() -> i64 {
	solve(2)
}

pub fn part2() -> i64 {
	solve(3)
}

fn solve(k: usize) -> i64 {
	parse_lines(INPUT)
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
