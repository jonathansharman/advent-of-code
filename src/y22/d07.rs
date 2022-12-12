use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	read_lines("input/YYYY/DD.txt").count()
}

pub fn part2() -> usize {
	0
}
