use itertools::{Itertools, MinMaxResult};

use crate::io::read_lines;

use std::collections::HashMap;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

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
	todo!();
}
