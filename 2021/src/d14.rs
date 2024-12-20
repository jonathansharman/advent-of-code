use itertools::{Itertools, MinMaxResult};

use std::collections::HashMap;

aoc::test::test_part!(test1, part1, 2003);
aoc::test::test_part!(test2, part2, 2276644000111);

const INPUT: &str = include_str!("input/14.txt");

pub fn part1() -> usize {
	polymerize(10)
}

pub fn part2() -> usize {
	polymerize(40)
}

fn polymerize(steps: usize) -> usize {
	let mut lines = INPUT.lines().map(|line| line.as_bytes().to_owned());
	// Insert starting polymer's k-mers.
	let mut element_counts = HashMap::new();
	let mut kmers = HashMap::new();
	let mut last = None;
	for (&a, &b) in lines.next().unwrap().iter().tuple_windows() {
		*element_counts.entry(a).or_insert(0) += 1;
		*kmers.entry((a, b)).or_insert(0) += 1;
		last = Some(b);
	}
	if let Some(last) = last {
		*element_counts.entry(last).or_insert(0) += 1;
	}
	// Read the rules.
	lines.next();
	let rules: HashMap<(u8, u8), u8> =
		lines.map(|line| ((line[0], line[1]), line[6])).collect();
	// Apply the rules.
	for _ in 0..steps {
		let mut new_kmers = HashMap::new();
		for ((a, b), count) in kmers.into_iter() {
			if let Some(&insertion) = rules.get(&(a, b)) {
				*element_counts.entry(insertion).or_insert(0) += count;
				*new_kmers.entry((a, insertion)).or_insert(0) += count;
				*new_kmers.entry((insertion, b)).or_insert(0) += count;
			}
		}
		kmers = new_kmers;
	}
	// Find the min/max counts.
	if let MinMaxResult::MinMax(min, max) = element_counts.values().minmax() {
		max - min
	} else {
		0
	}
}
