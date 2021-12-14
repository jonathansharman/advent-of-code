use itertools::{Itertools, MinMaxResult};

use crate::io::read_lines;

use std::collections::HashMap;

crate::test::test_part!(test1, part1, 2003);
crate::test::test_part!(test2, part2, 2276644000111);

pub fn part1() -> usize {
	let mut lines = read_lines("input/2021/14.txt");
	let mut polymer = lines.next().unwrap();
	lines.next();
	let mut rules: HashMap<(char, char), char> = HashMap::new();
	for rule in lines {
		rules.insert(
			(rule.chars().next().unwrap(), rule.chars().nth(1).unwrap()),
			rule.chars().nth(6).unwrap(),
		);
	}
	for _ in 0..10 {
		let mut next_polymer = String::new();
		for (a, b) in polymer.chars().tuple_windows() {
			next_polymer.push(a);
			if let Some(insertion) = rules.get(&(a, b)) {
				next_polymer.push(*insertion);
			}
		}
		next_polymer.push(polymer.chars().last().unwrap());
		polymer = next_polymer;
	}
	if let MinMaxResult::MinMax(min, max) = polymer
		.chars()
		.fold(HashMap::new(), |mut acc, c| {
			*acc.entry(c).or_insert(0) += 1;
			acc
		})
		.into_iter()
		.map(|(_, v)| v)
		.minmax()
	{
		max - min
	} else {
		0
	}
}

pub fn part2() -> usize {
	let mut lines = read_lines("input/2021/14.txt");
	// Insert starting polymer's k-mers.
	let mut element_counts = HashMap::new();
	let mut kmers = HashMap::new();
	let mut last = None;
	for (a, b) in lines.next().unwrap().chars().tuple_windows() {
		*element_counts.entry(a).or_insert(0) += 1;
		*kmers.entry((a, b)).or_insert(0) += 1;
		last = Some(b);
	}
	if let Some(last) = last {
		*element_counts.entry(last).or_insert(0) += 1;
	}
	// Read the rules.
	lines.next();
	let mut rules: HashMap<(char, char), char> = HashMap::new();
	for rule in lines {
		rules.insert(
			(rule.chars().next().unwrap(), rule.chars().nth(1).unwrap()),
			rule.chars().nth(6).unwrap(),
		);
	}
	// Apply the rules.
	for _ in 0..40 {
		let mut new_kmers = kmers.clone();
		for ((a, b), count) in kmers.iter() {
			if let Some(&insertion) = rules.get(&(*a, *b)) {
				*element_counts.entry(insertion).or_insert(0) += count;
				*new_kmers.entry((*a, insertion)).or_insert(0) += count;
				*new_kmers.entry((insertion, *b)).or_insert(0) += count;
				*new_kmers.entry((*a, *b)).or_insert(0) -= count;
			}
		}
		kmers = new_kmers;
	}
	// Find the min/max counts.
	if let MinMaxResult::MinMax(min, max) = element_counts.into_iter().map(|(_, v)| v).minmax() {
		max - min
	} else {
		0
	}
}
