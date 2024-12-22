use std::{cmp::Ordering, collections::HashMap};

aoc::test::test_part!(test1, part1, 7718);
aoc::test::test_part!(test2, part2, 128741994134728);


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
	Operational,
	Damaged,
	Unknown,
}

impl Spring {
	fn operational(&self) -> bool {
		!matches!(self, Spring::Damaged)
	}

	fn damaged(&self) -> bool {
		!matches!(self, Spring::Operational)
	}
}

fn parse_input() -> Vec<(Vec<Spring>, Vec<usize>)> {
	input!()
		.lines()
		.map(|line| {
			let (springs, groups) = line.split_once(' ').unwrap();
			let springs = springs
				.chars()
				.map(|c| match c {
					'.' => Spring::Operational,
					'#' => Spring::Damaged,
					_ => Spring::Unknown,
				})
				.collect();
			let groups = groups
				.split(',')
				.map(|n| n.parse::<usize>().unwrap())
				.collect();
			(springs, groups)
		})
		.collect()
}

pub fn part1() -> usize {
	parse_input()
		.into_iter()
		.map(|(springs, groups)| {
			arrangements(&mut HashMap::new(), &springs, &groups)
		})
		.sum()
}

pub fn part2() -> usize {
	parse_input()
		.into_iter()
		.map(|(springs, groups)| {
			let mut unfolded_springs = springs.clone();
			for _ in 0..4 {
				unfolded_springs.push(Spring::Unknown);
				unfolded_springs.extend(springs.iter());
			}
			let mut unfolded_groups = groups.clone();
			for _ in 0..4 {
				unfolded_groups.extend(groups.iter());
			}
			arrangements(
				&mut HashMap::new(),
				&unfolded_springs,
				&unfolded_groups,
			)
		})
		.sum()
}

fn arrangements(
	cache: &mut HashMap<(usize, usize), usize>,
	springs: &[Spring],
	groups: &[usize],
) -> usize {
	let key = (springs.len(), groups.len());
	if let Some(result) = cache.get(&key) {
		return *result;
	}
	if groups.is_empty() {
		// All of the remaining springs must be operational.
		let result = springs.iter().all(Spring::operational) as usize;
		cache.insert(key, result);
		return result;
	}
	if springs.len() < groups.iter().sum::<usize>() + groups.len() - 1 {
		// The groups definitely don't fit.
		cache.insert(key, 0);
		return 0;
	}
	match springs.len().cmp(&groups[0]) {
		// Can't fit the next group into the remaining springs.
		Ordering::Less => {
			cache.insert(key, 0);
			0
		}
		// All springs must be damaged, and there must be exactly one group.
		Ordering::Equal => {
			let result = (springs.iter().all(Spring::damaged)
				&& groups.len() == 1) as usize;
			cache.insert(key, result);
			result
		}
		Ordering::Greater => {
			let mut result = 0;
			// Add arrangements with first group at the start.
			let group_fits = springs[..groups[0]].iter().all(Spring::damaged);
			let gap_after_group = springs[groups[0]].operational();
			if group_fits && gap_after_group {
				// Add the arrangements of the remaining springs and groups.
				let rest = arrangements(
					cache,
					&springs[groups[0] + 1..],
					&groups[1..],
				);
				if rest > 0 {
					result += rest;
				}
			}
			// Add arrangements with first group after the start.
			if springs[0].operational() {
				result += arrangements(cache, &springs[1..], groups);
			}
			cache.insert(key, result);
			result
		}
	}
}
