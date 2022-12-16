use std::collections::HashSet;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 6522);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut head = (0, 0);
	let mut tail = (0, 0);
	let mut trail = HashSet::from([(0, 0)]);
	for line in read_lines("input/2022/09.txt") {
		let (direction, count) = line.split_once(' ').unwrap();
		let count = count.parse::<u32>().unwrap();
		for _ in 0..count {
			match direction {
				"U" => head.1 -= 1,
				"D" => head.1 += 1,
				"L" => head.0 -= 1,
				"R" => head.0 += 1,
				_ => panic!(),
			}
			if head.0 - tail.0 == 2 {
				tail.0 += 1;
				tail.1 = head.1;
			} else if head.0 - tail.0 == -2 {
				tail.0 -= 1;
				tail.1 = head.1;
			} else if head.1 - tail.1 == 2 {
				tail.0 = head.0;
				tail.1 += 1;
			} else if head.1 - tail.1 == -2 {
				tail.0 = head.0;
				tail.1 -= 1;
			}
			trail.insert(tail);
		}
	}
	trail.len()
}

pub fn part2() -> usize {
	0
}
