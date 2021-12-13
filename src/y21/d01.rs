use crate::io::read_integer_lines;

use itertools::Itertools;

crate::test::test_part!(test1, part1, 1162);
crate::test::test_part!(test2, part2, 1190);

pub fn part1() -> usize {
	read_integer_lines("input/2021/01.txt")
		.tuple_windows()
		.filter(|(prev, next)| prev < next)
		.count()
}

pub fn part2() -> usize {
	read_integer_lines("input/2021/01.txt")
		.tuple_windows()
		.map(|(first, second, third)| first + second + third)
		.tuple_windows()
		.filter(|(prev, next)| prev < next)
		.count()
}
