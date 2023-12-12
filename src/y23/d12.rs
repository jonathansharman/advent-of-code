use std::{cmp::Ordering, collections::HashMap};

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 7718);
crate::test::test_part!(test2, part2, 128741994134728);

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

pub fn part1() -> usize {
	read_lines("input/2023/12.txt")
		.map(|line| {
			let (springs, groups) = line.split_once(' ').unwrap();
			let springs = springs
				.chars()
				.map(|c| match c {
					'.' => Spring::Operational,
					'#' => Spring::Damaged,
					_ => Spring::Unknown,
				})
				.collect::<Vec<_>>();
			let groups = groups
				.split(',')
				.map(|n| n.parse::<usize>().unwrap())
				.collect::<Vec<_>>();
			arrangements(&mut HashMap::new(), 0, 0, &springs, &groups)
		})
		.sum()
}

fn arrangements(
	cache: &mut HashMap<(usize, usize), usize>,
	i: usize,
	j: usize,
	springs: &[Spring],
	groups: &[usize],
) -> usize {
	let s = &springs[i..];
	let g = &groups[j..];
	let key = (springs.len() - i, groups.len() - j);
	if let Some(result) = cache.get(&key) {
		return *result;
	}
	if g.is_empty() {
		// All of the remaining springs must be operational.
		let result = s.iter().all(Spring::operational) as usize;
		cache.insert(key, result);
		return result;
	}
	if s.len() < g.iter().sum::<usize>() + g.len() - 1 {
		// The groups definitely don't fit.
		cache.insert(key, 0);
		return 0;
	}
	match s.len().cmp(&g[0]) {
		// Can't fit the next group into the remaining springs.
		Ordering::Less => {
			cache.insert(key, 0);
			0
		}
		// All springs must be damaged, and there must be exactly one group.
		Ordering::Equal => {
			let result =
				(s.iter().all(Spring::damaged) && g.len() == 1) as usize;
			cache.insert(key, result);
			result
		}
		Ordering::Greater => {
			let mut result = 0;
			// Add arrangements with first group at the start.
			let group_fits = s[..g[0]].iter().all(Spring::damaged);
			let gap_after_group = s[g[0]].operational();
			if group_fits && gap_after_group {
				// Add the arrangements of the remaining springs and groups.
				let rest =
					arrangements(cache, i + g[0] + 1, j + 1, springs, groups);
				if rest > 0 {
					result += rest;
				}
			}
			// Add arrangements with first group after the start.
			if s[0].operational() {
				result += arrangements(cache, i + 1, j, springs, groups);
			}
			cache.insert(key, result);
			result
		}
	}
}

pub fn part2() -> usize {
	read_lines("input/2023/12.txt")
		.map(|line| {
			let (springs, groups) = line.split_once(' ').unwrap();
			let springs = springs
				.chars()
				.map(|c| match c {
					'.' => Spring::Operational,
					'#' => Spring::Damaged,
					_ => Spring::Unknown,
				})
				.collect::<Vec<_>>();
			let mut unfolded_springs = springs.clone();
			for _ in 0..4 {
				unfolded_springs.push(Spring::Unknown);
				unfolded_springs.extend(springs.iter());
			}
			let groups = groups
				.split(',')
				.map(|n| n.parse::<usize>().unwrap())
				.collect::<Vec<_>>();
			let mut unfolded_groups = groups.clone();
			for _ in 0..4 {
				unfolded_groups.extend(groups.iter());
			}
			arrangements(
				&mut HashMap::new(),
				0,
				0,
				&unfolded_springs,
				&unfolded_groups,
			)
		})
		.sum()
}
