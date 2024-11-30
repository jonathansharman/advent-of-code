use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	read_lines("input/DD.txt").count()
}

pub fn part2() -> usize {
	0
}
