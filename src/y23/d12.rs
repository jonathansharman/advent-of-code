use std::cmp::Ordering;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 7718);
crate::test::test_part!(test2, part2, ?);

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
			println!("Sum: {}", arrangements(&springs, &groups));
			arrangements(&springs, &groups)
		})
		.sum()
}

fn arrangements(springs: &[Spring], groups: &[usize]) -> usize {
	if groups.is_empty() {
		// All of the remaining springs must be operational.
		return springs.iter().all(Spring::operational) as usize;
	}
	match springs.len().cmp(&groups[0]) {
		// Can't fit the next group into the remaining springs.
		Ordering::Less => 0,
		// All springs must be damaged, and there must be exactly one group.
		Ordering::Equal => {
			(springs.iter().all(Spring::damaged) && groups.len() == 1) as usize
		}
		Ordering::Greater => {
			let mut sum = 0;
			// Add arrangements with first group at the start.
			let group_fits = springs[..groups[0]].iter().all(Spring::damaged);
			let gap_after_group = springs[groups[0]].operational();
			if group_fits && gap_after_group {
				// Add the arrangements of the remaining springs and groups.
				let rest =
					arrangements(&springs[groups[0] + 1..], &groups[1..]);
				if rest > 0 {
					sum += rest;
				}
			}
			// Add arrangements with first group after the start.
			if springs[0].operational() {
				sum += arrangements(&springs[1..], groups);
			}
			sum
		}
	}
}

pub fn part2() -> usize {
	0
}
