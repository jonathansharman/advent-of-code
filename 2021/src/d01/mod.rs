use aoc::input::parse_lines;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 1162);
aoc::test::test_part!(test2, part2, 1190);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	parse_lines(INPUT)
		.tuple_windows::<(u32, u32)>()
		.filter(|(prev, next)| prev < next)
		.count()
}

pub fn part2() -> usize {
	parse_lines(INPUT)
		.tuple_windows::<(u32, u32, u32)>()
		.map(|(first, second, third)| first + second + third)
		.tuple_windows()
		.filter(|(prev, next)| prev < next)
		.count()
}
