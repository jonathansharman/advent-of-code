use std::collections::HashSet;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 8088);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> u32 {
	read_lines("input/2022/03.txt")
		.map(|line| {
			let line = line.as_bytes();
			let (first, second) = line.split_at(line.len() / 2);
			let first = first.iter().collect::<HashSet<&u8>>();
			let second = second.iter().collect::<HashSet<&u8>>();
			let dupe = *first.intersection(&second).next().unwrap();
			if (b'A'..=b'Z').contains(dupe) {
				(dupe - b'A' + 27) as u32
			} else {
				(dupe - b'a' + 1) as u32
			}
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
