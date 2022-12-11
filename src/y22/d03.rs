use std::collections::HashSet;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 8088);
crate::test::test_part!(test2, part2, 2522);

fn priority(item: &u8) -> u32 {
	if (b'A'..=b'Z').contains(item) {
		(item - b'A' + 27) as u32
	} else {
		(item - b'a' + 1) as u32
	}
}

pub fn part1() -> u32 {
	read_lines("input/2022/03.txt")
		.map(|line| {
			let line = line.as_bytes();
			let (first, second) = line.split_at(line.len() / 2);
			let first = first.iter().collect::<HashSet<&u8>>();
			let second = second.iter().collect::<HashSet<&u8>>();
			priority(first.intersection(&second).next().unwrap())
		})
		.sum()
}

pub fn part2() -> u32 {
	read_lines("input/2022/03.txt")
		.chunks(3)
		.into_iter()
		.map(|chunk| {
			let item = chunk
				.map(|line| line.as_bytes().iter().cloned().collect::<HashSet<u8>>())
				.reduce(|acc, next| acc.intersection(&next).cloned().collect::<HashSet<u8>>())
				.unwrap()
				.into_iter()
				.next()
				.unwrap();
			priority(&item)
		})
		.sum()
}
