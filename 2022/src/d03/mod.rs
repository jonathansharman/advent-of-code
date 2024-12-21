use std::collections::HashSet;

use itertools::Itertools;

aoc::test::test_part!(test1, part1, 8088);
aoc::test::test_part!(test2, part2, 2522);

const INPUT: &str = include_str!("input.txt");

fn priority(item: &u8) -> u32 {
	if item.is_ascii_uppercase() {
		(item - b'A' + 27) as u32
	} else {
		(item - b'a' + 1) as u32
	}
}

pub fn part1() -> u32 {
	INPUT
		.lines()
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
	INPUT
		.lines()
		.chunks(3)
		.into_iter()
		.map(|chunk| {
			let item = chunk
				.map(|line| {
					line.as_bytes().iter().cloned().collect::<HashSet<u8>>()
				})
				.reduce(|acc, next| {
					acc.intersection(&next).cloned().collect::<HashSet<u8>>()
				})
				.unwrap()
				.into_iter()
				.next()
				.unwrap();
			priority(&item)
		})
		.sum()
}
