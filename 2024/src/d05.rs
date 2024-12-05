use std::{cmp::Ordering, collections::HashSet};

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 5747);
aoc::test::test_part!(test2, part2, 5502);

fn get_less(
	rules: &mut impl Iterator<Item = String>,
) -> HashSet<(String, String)> {
	let mut less: HashSet<(String, String)> = HashSet::new();
	for rule in rules {
		if rule.is_empty() {
			break;
		}
		let (a, b) = rule.split_once('|').unwrap();
		less.insert((a.to_owned(), b.to_owned()));
	}
	less
}

pub fn part1() -> usize {
	let mut lines = read_lines("input/05.txt");
	let less = get_less(lines.by_ref());
	lines
		.map(|update| {
			let pages: Vec<String> =
				update.split(',').map(|s| s.to_owned()).collect();
			for (i, a) in pages.iter().enumerate() {
				for b in pages.iter().skip(i) {
					if less.contains(&(b.clone(), a.clone())) {
						return 0;
					}
				}
			}
			pages[pages.len() / 2].parse().unwrap()
		})
		.sum()
}

pub fn part2() -> usize {
	let mut lines = read_lines("input/05.txt");
	let less = get_less(lines.by_ref());
	lines
		.map(|update| {
			let mut pages: Vec<String> =
				update.split(',').map(|s| s.to_owned()).collect();
			for i in 0..pages.len() - 1 {
				let a = &pages[i];
				for j in i + 1..pages.len() {
					let b = pages[j].clone();
					if less.contains(&(b, a.clone())) {
						pages.sort_by(|a, b| {
							if less.contains(&(a.clone(), b.clone())) {
								Ordering::Less
							} else {
								Ordering::Greater
							}
						});
						return pages[pages.len() / 2].parse().unwrap();
					}
				}
			}
			0
		})
		.sum()
}
