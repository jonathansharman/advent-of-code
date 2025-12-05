use std::ops::RangeInclusive;

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 643);
aoc::test::test_part!(test2, part2, 342018167474526);

type Range = RangeInclusive<usize>;

fn parse_ranges(lines: &mut impl Iterator<Item = &'static str>) -> Vec<Range> {
	lines
		.by_ref()
		.map_while(|line| {
			(!line.is_empty()).then(|| {
				let (a, b) = line
					.split('-')
					.map(|n| n.parse().unwrap())
					.collect_tuple()
					.unwrap();
				a..=b
			})
		})
		.collect()
}

/// The overlapping range of `r1` and `r2` or else `None`.
fn merge_ranges(r1: &Range, r2: &Range) -> Option<Range> {
	// Ensure the lower bound of r1 is not greater than that of r2.
	let (r1, r2) = if r1.start() > r2.start() {
		(r2, r1)
	} else {
		(r1, r2)
	};

	(*r2.start() <= r1.end() + 1)
		.then_some(*r1.start()..=*(r1.end().max(r2.end())))
}

fn merge_all_ranges(ranges: Vec<Range>) -> Vec<Range> {
	let mut merged = Vec::new();
	let mut queue = ranges;
	'queue: while let Some(next) = queue.pop() {
		for i in 0..merged.len() {
			if let Some(new) = merge_ranges(&merged[i], &next) {
				merged.swap_remove(i);
				queue.push(new);
				continue 'queue;
			}
		}
		merged.push(next);
	}
	merged
}

pub fn part1() -> usize {
	let mut lines = input!().lines();
	let ranges = merge_all_ranges(parse_ranges(lines.by_ref()));
	lines
		.filter(|&ingredient| {
			ranges
				.iter()
				.any(|range| range.contains(&ingredient.parse().unwrap()))
		})
		.count()
}

pub fn part2() -> usize {
	merge_all_ranges(parse_ranges(&mut input!().lines()))
		.into_iter()
		.map(|range| range.end() - range.start() + 1)
		.sum()
}
