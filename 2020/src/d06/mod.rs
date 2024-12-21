use std::collections::HashSet;

aoc::test::test_part!(test1, part1, 6782);
aoc::test::test_part!(test2, part2, 3596);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	let mut sum = 0;
	let mut group = HashSet::new();
	for line in INPUT.lines() {
		if line.is_empty() {
			sum += group.len();
			group.clear();
		} else {
			for c in line.chars() {
				group.insert(c);
			}
		}
	}
	sum += group.len();
	sum
}

pub fn part2() -> usize {
	let mut sum = 0;
	let mut group = Vec::new();
	let mut process_group = |group: &mut Vec<&str>| {
		sum += group
			.iter()
			.map(|line| HashSet::from_iter(line.chars()))
			.reduce(|acc: HashSet<_>, chars| {
				acc.intersection(&chars).cloned().collect()
			})
			.unwrap_or_default()
			.len();
		group.clear();
	};
	for line in INPUT.lines() {
		if line.is_empty() {
			process_group(&mut group);
		} else {
			group.push(line);
		}
	}
	process_group(&mut group);
	sum
}
