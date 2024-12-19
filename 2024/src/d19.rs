use std::collections::HashMap;

use aoc::io::read_lines;
use itertools::Itertools;
use regex::Regex;

aoc::test::test_part!(test1, part1, 340);
aoc::test::test_part!(test2, part2, 717561822679428);

pub fn part1() -> usize {
	let mut lines = read_lines("input/19.txt");
	let patterns = lines.next().unwrap().split(", ").join("|");
	let regex = Regex::new(&format!("^({patterns})+$",)).unwrap();
	lines.skip(1).filter(|towel| regex.is_match(towel)).count()
}

fn combos<'a>(
	patterns: &[String],
	towel: &'a str,
	cache: &mut HashMap<&'a str, usize>,
) -> usize {
	if let Some(&n) = cache.get(towel) {
		return n;
	}
	let n = patterns
		.iter()
		.filter_map(|pattern| {
			towel
				.strip_prefix(pattern)
				.map(|rest| combos(patterns, rest, cache))
		})
		.sum();
	cache.insert(towel, n);
	n
}

pub fn part2() -> usize {
	let mut lines = read_lines("input/19.txt");
	let patterns: Vec<String> = lines
		.next()
		.unwrap()
		.split(", ")
		.map(|pattern| pattern.to_string())
		.collect();
	lines
		.skip(1)
		.map(|towel| combos(&patterns, &towel, &mut HashMap::from([("", 1)])))
		.sum()
}
