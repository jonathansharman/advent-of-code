use aoc::input::{input, ParseLines};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 1162);
aoc::test::test_part!(test2, part2, 1190);

pub fn part1() -> usize {
	input!()
		.parse_lines()
		.tuple_windows::<(u32, u32)>()
		.filter(|(prev, next)| prev < next)
		.count()
}

pub fn part2() -> usize {
	input!()
		.parse_lines()
		.tuple_windows::<(u32, u32, u32)>()
		.map(|(first, second, third)| first + second + third)
		.tuple_windows()
		.filter(|(prev, next)| prev < next)
		.count()
}
