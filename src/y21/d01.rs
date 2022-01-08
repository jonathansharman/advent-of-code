use crate::io::parse_lines;
use itertools::Itertools;

crate::test::test_part!(test1, part1, 1162);
crate::test::test_part!(test2, part2, 1190);

pub fn part1() -> usize {
	parse_lines("input/2021/01.txt")
		.tuple_windows::<(u32, u32)>()
		.filter(|(prev, next)| prev < next)
		.count()
}

pub fn part2() -> usize {
	parse_lines("input/2021/01.txt")
		.tuple_windows::<(u32, u32, u32)>()
		.map(|(first, second, third)| first + second + third)
		.tuple_windows()
		.filter(|(prev, next)| prev < next)
		.count()
}
