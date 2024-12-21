use std::{cmp::Ordering, collections::HashSet};

aoc::test::test_part!(test1, part1, 5747);
aoc::test::test_part!(test2, part2, 5502);

const INPUT: &str = include_str!("input.txt");

fn get_less(
	rules: &mut impl Iterator<Item = &'static str>,
) -> HashSet<(u32, u32)> {
	rules
		.map_while(|rule| {
			if rule.is_empty() {
				None
			} else {
				let (a, b) = rule.split_once('|').unwrap();
				Some((a.parse().unwrap(), b.parse().unwrap()))
			}
		})
		.collect()
}

pub fn part1() -> u32 {
	let mut lines = INPUT.lines();
	let less = get_less(lines.by_ref());
	lines
		.map(|update| {
			let pages: Vec<u32> =
				update.split(',').map(|s| s.parse().unwrap()).collect();
			for (i, a) in pages.iter().enumerate() {
				for b in pages.iter().skip(i) {
					if less.contains(&(*b, *a)) {
						return 0;
					}
				}
			}
			pages[pages.len() / 2]
		})
		.sum()
}

pub fn part2() -> u32 {
	let mut lines = INPUT.lines();
	let less = get_less(lines.by_ref());
	lines
		.map(|update| {
			let mut pages: Vec<u32> =
				update.split(',').map(|s| s.parse().unwrap()).collect();
			for (i, a) in pages.iter().copied().enumerate() {
				for b in pages.iter().skip(i).copied() {
					if less.contains(&(b, a)) {
						pages.sort_by(|a, b| {
							if less.contains(&(*a, *b)) {
								Ordering::Less
							} else {
								Ordering::Greater
							}
						});
						return pages[pages.len() / 2];
					}
				}
			}
			0
		})
		.sum()
}
