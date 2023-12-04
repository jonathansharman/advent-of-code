use std::collections::HashSet;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 21959);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	read_lines("input/2023/04.txt")
		.map(|line| {
			let line = &line[line.find(':').unwrap() + 1..];
			let (winners, have) = line.split_once('|').unwrap();
			let winners: HashSet<usize> = HashSet::from_iter(
				winners
					.split_whitespace()
					.map(|s| s.parse::<usize>().unwrap()),
			);
			let n_wins = have
				.split_whitespace()
				.filter(|s| winners.contains(&s.parse().unwrap()))
				.count();
			if n_wins == 0 {
				0
			} else {
				2u32.pow((n_wins - 1) as u32) as usize
			}
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
