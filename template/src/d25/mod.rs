aoc::test::test_part!(test1, part1, ?);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	INPUT.lines().count()
}
