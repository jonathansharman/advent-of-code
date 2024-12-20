aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

const INPUT: &str = include_str!("input/23.txt");

pub fn part1() -> usize {
	INPUT.lines().count()
}

pub fn part2() -> usize {
	0
}
