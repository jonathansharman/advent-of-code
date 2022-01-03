use crate::io::read_lines;
use std::collections::HashSet;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut sum = 0;
	let mut group = HashSet::new();
	for line in read_lines("input/2020/06.txt") {
		if line.is_empty() {
			sum += group.len();
			group.clear();
		} else {
			for c in line.chars() {
				group.insert(c);
			}
		}
	}
	sum += group.len();
	sum
}

pub fn part2() -> usize {
	read_lines("input/2020/06.txt").count()
}
