use std::collections::{HashMap, HashSet};

use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 5747);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut lines = read_lines("input/05.txt");

	let mut rules: HashMap<String, HashSet<String>> = HashMap::new();
	for rule in lines.by_ref() {
		if rule.is_empty() {
			break;
		}
		let (a, b) = rule.split_once('|').unwrap();
		rules.entry(a.to_owned()).or_default().insert(b.to_owned());
	}

	lines
		.map(|update| {
			let pages: Vec<String> =
				update.split(',').map(|s| s.to_owned()).collect();
			for (i, a) in pages.iter().enumerate() {
				for b in pages.iter().skip(i) {
					if rules.get(b).map_or(false, |set| set.contains(a)) {
						return 0;
					}
				}
			}
			pages[pages.len() / 2].parse().unwrap()
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
